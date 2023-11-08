use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

// use std::num::std::num::Wrapping;

#[proc_macro_derive(EasyHash)]
pub fn derive_easy_hash(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: EasyHash` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let sum = hash_sum(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics easy_hash::EasyHash for #name #ty_generics #where_clause {
            const TYPE_SALT: u32 = easy_hash::type_salt::<#name>();

            fn ehash(&self) -> u64 {
                // println!("type salt, {} for {}",)
                #sum
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: EasyHash` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(easy_hash::HeapSize));
        }
    }
    generics
}

// Generate an expression to sum up the heap size of each field.
fn hash_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Enum(ref data_enum) => {
            let match_arms = data_enum.variants.iter().enumerate().map(|v| {
                let enum_variant_index = Index::from(v.0);
                let variant = &v.1.ident;
                // let recurse = hash_sum(&v.1.fields);

                match &v.1.fields {
                    Fields::Unit => quote! {
                        Self::#variant => {
                            checksum.update(&[Self::TYPE_SALT, #enum_variant_index]);
                            checksum.value()
                        }
                    },
                    Fields::Unnamed(fields) => {
                        let field_names = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let name = Ident::new(&format!("f{}", i), f.span());
                            quote_spanned! {f.span()=>
                                #name
                            }
                        });

                        let field_hash_exprs = fields.unnamed.iter().enumerate().map(|(i, f)| {
                            let name = Ident::new(&format!("f{}", i), f.span());
                            quote_spanned! {f.span()=>
                                easy_hash::EasyHash::ehash(#name)
                            }
                        });

                        let sum_expr = quote! {
                            (std::num::Wrapping(0) #(+ std::num::Wrapping(#field_hash_exprs))* ).0
                        };

                        quote! {
                            Self::#variant(#(#field_names,)*) => {
                                checksum.update(&[Self::TYPE_SALT, #enum_variant_index]);
                                checksum.update(&easy_hash::split_u64(#sum_expr));
                                checksum.value()
                            }
                        }
                    }
                    Fields::Named(_fields) => {
                        unimplemented!()
                    }
                }
            });

            quote! {
                // 0 #(+ #recurse)*
                // variant.enum_token.to_tokens(tokens);

                // (std::num::Wrapping(0) #(+ std::num::Wrapping(#recurse))* ).0
                let mut checksum = fletcher::Fletcher64::new();
                // checksum.update(&[Self::TYPE_SALT]);
                // #(checksum.update(#recurse);)*
                match self {
                    #(#match_arms)*
                }
                // checksum.value()
            }
        }
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    // ```
                    // checksum.update(&[Self::TYPE_SALT]);
                    // checksum.update(easy_hash::split_u64(easy_hash::EasyHash::ehash(&self.x)))
                    // checksum.update(easy_hash::split_u64(easy_hash::EasyHash::ehash(&self.y)))
                    // checksum.update(easy_hash::split_u64(easy_hash::EasyHash::ehash(&self.z)))
                    // checksum.value()
                    // ```
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `easy_hash_of_children`
                    // call. This way if one of the field types does not
                    // implement `EasyHash` then the compiler's error message
                    // underlines which field it is. An example is shown in the
                    // readme of the parent directory.
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            &easy_hash::split_u64(easy_hash::EasyHash::ehash(&self.#name))
                        }
                    });
                    quote! {
                        // 0 #(+ #recurse)*

                        // (std::num::Wrapping(0) #(+ std::num::Wrapping(#recurse))* ).0
                        let mut checksum = fletcher::Fletcher64::new();
                        checksum.update(&[Self::TYPE_SALT]);
                        #(checksum.update(#recurse);)*
                        checksum.value()
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    //  ```
                    // 0 + self.0.easy_hash() + self.1.easy_hash() + self.2.easy_hash()
                    // ```
                    // using wrapping arithemtic to avoid overflow panics.
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            easy_hash::EasyHash::ehash(&self.#index)
                        }
                    });
                    quote! {
                        (std::num::Wrapping(0) #(+ std::num::Wrapping(#recurse))* ).0
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }

        Data::Union(_) => unimplemented!(),
    }
}
