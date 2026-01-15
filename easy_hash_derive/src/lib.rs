use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam,
    Generics, Index, Variant, parse_macro_input, parse_quote,
};

#[proc_macro_derive(EasyHash, attributes(easy_hash_ignore))]
pub fn derive_easy_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: EasyHash` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the hash of the input
    let ehash_fn_inner = hash_sum(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics easy_hash::EasyHash for #name #ty_generics #where_clause {
            const TYPE_SALT: u32 = easy_hash::type_salt::<#name #ty_generics>();

            fn ehash(&self) -> u64 {
                #ehash_fn_inner
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: EasyHash` to every type parameter T.
// Note: PhantomData<T> doesn't require T: EasyHash, so we keep the simple approach
// of adding the bound to all type parameters. The PhantomData implementation
// doesn't depend on T implementing EasyHash.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(easy_hash::EasyHash));
        }
    }
    generics
}

// Generate an expression to sum up the hashes of each field.
fn hash_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Enum(ref data_enum) => expand_enum(data_enum),
        Data::Struct(ref data_struct) => expand_struct(data_struct),
        Data::Union(_) => unimplemented!(),
    }
}

/// Generate the hash implementation for an enum.
/// Creates a match expression that hashes the type salt, variant index, and variant fields.
fn expand_enum(data_enum: &DataEnum) -> TokenStream {
    let match_arms = data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(variant_index, variant)| expand_enum_variant(variant_index, variant));

    quote! {
        let mut checksum = easy_hash::fletcher::Fletcher64::new();
        match self {
            #(#match_arms)*
        }
        checksum.value()
    }
}

/// Generate a match arm for a single enum variant.
fn expand_enum_variant(variant_index: usize, variant: &Variant) -> TokenStream {
    let enum_variant_index = Index::from(variant_index);
    let variant_ident = &variant.ident;

    match &variant.fields {
        Fields::Unit => expand_enum_variant_unit(variant_ident, enum_variant_index),
        Fields::Unnamed(fields) => {
            expand_enum_variant_unnamed(variant_ident, enum_variant_index, fields)
        }
        Fields::Named(fields) => {
            expand_enum_variant_named(variant_ident, enum_variant_index, fields)
        }
    }
}

/// Generate a match arm for a unit enum variant.
/// Hashes only the type salt and variant index.
fn expand_enum_variant_unit(variant_ident: &Ident, variant_index: Index) -> TokenStream {
    quote! {
        Self::#variant_ident => {
            checksum.update(&[Self::TYPE_SALT, #variant_index]);
        }
    }
}

/// Generate a match arm for an unnamed fields enum variant (tuple variant).
/// Hashes the type salt, variant index, and all tuple fields.
fn expand_enum_variant_unnamed(
    variant_ident: &Ident,
    variant_index: Index,
    fields: &FieldsUnnamed,
) -> TokenStream {
    let field_names = fields.unnamed.iter().enumerate().map(|(i, f)| {
        let name = Ident::new(&format!("f{}", i), f.span());
        quote_spanned! {f.span()=> #name }
    });

    let field_hash_exprs = fields.unnamed.iter().enumerate().map(|(i, f)| {
        let name = Ident::new(&format!("f{}", i), f.span());
        quote_spanned! {f.span()=>
            easy_hash::EasyHash::ehash(#name)
        }
    });

    quote! {
        Self::#variant_ident(#(#field_names,)*) => {
            checksum.update(&[Self::TYPE_SALT, #variant_index]);
            checksum.update(&easy_hash::u64_to_u32_slice(&[#(#field_hash_exprs,)*]));
        }
    }
}

/// Generate a match arm for a named fields enum variant.
/// Hashes the type salt, variant index, and all named fields (respecting `#[easy_hash_ignore]`).
fn expand_enum_variant_named(
    variant_ident: &Ident,
    variant_index: Index,
    fields: &FieldsNamed,
) -> TokenStream {
    let field_names = fields
        .named
        .iter()
        .filter(|f| !has_easy_hash_ignore_attr(f))
        .map(|f| {
            let name = &f.ident;
            quote_spanned! {f.span()=> #name }
        });

    let field_hash_exprs = fields
        .named
        .iter()
        .filter(|f| !has_easy_hash_ignore_attr(f))
        .map(|f| {
            let name = &f.ident;
            quote_spanned! {f.span()=> #name.ehash() }
        });

    quote! {
        Self::#variant_ident { #(#field_names,)* } => {
            checksum.update(&[Self::TYPE_SALT, #variant_index]);
            checksum.update(&easy_hash::u64_to_u32_slice(&[#(#field_hash_exprs,)*]));
        }
    }
}

/// Generate the hash implementation for a struct.
fn expand_struct(data_struct: &DataStruct) -> TokenStream {
    match &data_struct.fields {
        Fields::Named(fields) => expand_struct_named(fields),
        Fields::Unnamed(fields) => expand_struct_unnamed(fields),
        Fields::Unit => expand_struct_unit(),
    }
}

/// Generate the hash implementation for a struct with named fields.
/// Hashes the type salt and all fields (respecting `#[easy_hash_ignore]`).
fn expand_struct_named(fields: &FieldsNamed) -> TokenStream {
    let field_hash_exprs = fields
        .named
        .iter()
        .filter(|f| !has_easy_hash_ignore_attr(f))
        .map(|f| {
            let name = &f.ident;
            quote_spanned! {f.span()=>
                easy_hash::EasyHash::ehash(&self.#name)
            }
        });

    quote! {
        let mut checksum = easy_hash::fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&easy_hash::u64_to_u32_slice(&[ #(#field_hash_exprs,)* ]));
        checksum.value()
    }
}

/// Generate the hash implementation for a struct with unnamed fields (tuple struct).
/// Hashes the type salt and all tuple fields.
fn expand_struct_unnamed(fields: &FieldsUnnamed) -> TokenStream {
    let field_hash_exprs = fields.unnamed.iter().enumerate().map(|(i, f)| {
        let index = Index::from(i);
        quote_spanned! {f.span()=>
            easy_hash::EasyHash::ehash(&self.#index)
        }
    });

    quote! {
        let mut checksum = easy_hash::fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.update(&easy_hash::u64_to_u32_slice(&[ #(#field_hash_exprs,)* ]));
        checksum.value()
    }
}

/// Generate the hash implementation for a unit struct.
/// Hashes only the type salt.
fn expand_struct_unit() -> TokenStream {
    quote! {
        let mut checksum = easy_hash::fletcher::Fletcher64::new();
        checksum.update(&[Self::TYPE_SALT]);
        checksum.value()
    }
}

/// Check if a field has the `#[easy_hash_ignore]` attribute.
fn has_easy_hash_ignore_attr(field: &syn::Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("easy_hash_ignore"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    fn expand_as_string(input: DeriveInput) -> String {
        let name = input.ident;
        let generics = add_trait_bounds(input.generics);
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let ehash_fn_inner = hash_sum(&input.data);

        quote! {
            impl #impl_generics easy_hash::EasyHash for #name #ty_generics #where_clause {
                const TYPE_SALT: u32 = easy_hash::type_salt::<#name #ty_generics>();

                fn ehash(&self) -> u64 {
                    #ehash_fn_inner
                }
            }
        }
        .to_string()
    }

    #[test]
    fn test_struct_named_ignores_marked_fields() {
        let input: DeriveInput = parse_quote! {
            struct Example<T> {
                a: u32,
                #[easy_hash_ignore]
                b: T,
            }
        };

        let actual = expand_as_string(input);
        let expected = quote! {
            impl<T: easy_hash::EasyHash> easy_hash::EasyHash for Example<T> {
                const TYPE_SALT: u32 = easy_hash::type_salt::<Example<T> >();

                fn ehash(&self) -> u64 {
                    let mut checksum = easy_hash::fletcher::Fletcher64::new();
                    checksum.update(&[Self::TYPE_SALT]);
                    checksum.update(&easy_hash::u64_to_u32_slice(&[
                        easy_hash::EasyHash::ehash(&self.a),
                    ]));
                    checksum.value()
                }
            }
        }
        .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_enum_variants_mixed_fields() {
        let input: DeriveInput = parse_quote! {
            enum Example {
                Unit,
                Tuple(u32, u64),
                Named {
                    x: u8,
                    #[easy_hash_ignore]
                    y: u16,
                },
            }
        };

        let actual = expand_as_string(input);
        let expected = quote! {
            impl easy_hash::EasyHash for Example {
                const TYPE_SALT: u32 = easy_hash::type_salt::<Example>();

                fn ehash(&self) -> u64 {
                    let mut checksum = easy_hash::fletcher::Fletcher64::new();
                    match self {
                        Self::Unit => {
                            checksum.update(&[Self::TYPE_SALT, 0]);
                        }
                        Self::Tuple(f0, f1,) => {
                            checksum.update(&[Self::TYPE_SALT, 1]);
                            checksum.update(&easy_hash::u64_to_u32_slice(&[
                                easy_hash::EasyHash::ehash(f0),
                                easy_hash::EasyHash::ehash(f1),
                            ]));
                        }
                        Self::Named { x, } => {
                            checksum.update(&[Self::TYPE_SALT, 2]);
                            checksum.update(&easy_hash::u64_to_u32_slice(&[
                                x.ehash(),
                            ]));
                        }
                    }
                    checksum.value()
                }
            }
        }
        .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tuple_struct_hashing() {
        let input: DeriveInput = parse_quote! {
            struct Wrapper(u8, u16);
        };

        let actual = expand_as_string(input);
        let expected = quote! {
            impl easy_hash::EasyHash for Wrapper {
                const TYPE_SALT: u32 = easy_hash::type_salt::<Wrapper>();

                fn ehash(&self) -> u64 {
                    let mut checksum = easy_hash::fletcher::Fletcher64::new();
                    checksum.update(&[Self::TYPE_SALT]);
                    checksum.update(&easy_hash::u64_to_u32_slice(&[
                        easy_hash::EasyHash::ehash(&self.0),
                        easy_hash::EasyHash::ehash(&self.1),
                    ]));
                    checksum.value()
                }
            }
        }
        .to_string();

        assert_eq!(actual, expected);
    }
}
