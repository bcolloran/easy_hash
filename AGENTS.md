# Tests

**Before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user.**


## Running tests

The agent should always run `cargo test` in the workspace root to run all tests in the workspace, not just the tests in the current crate. Do not pass additional flags to `cargo test` unless specifically requested by the user.


## File layout
Try to ensure that the file organization for tests matches that of the implementation code, e.g., if `EasyHash` for `OrderedFloat` is implemented in the file `ordered_float.rs`, 
put tests in `tests/test_ordered_float.rs`.

## Multiple cases in one test
When it makes sense to repeatedly test a single function on multiple *input* cases, use a `#[test_case(data; "data case description")]` attribute on a test to specify the data cases. This allows the test to be run multiple times with different inputs, and will report each case separately in the test results.

This is "DRY"er than writing a separate test function for each case, and cleaner than putting multiple assertion statements in a single test function that loops over the data cases.

For example, we have this in the file `easy_hash/tests/test_utils.rs`:
```rust
#[test_case(0 ; "0u64")]
#[test_case(1 ; "1u64")]
#[test_case(u32::MAX as u64 ; "u32::MAX as u64")]
#[test_case(u64::MAX ; "u64::MAX")]
#[test_case(u64::MAX - 1 ; "u64::MAX - 1")]
#[test_case(0x1234_5678_9abc_def0 ; "0x1234_5678_9abc_def0")]
fn test_split_u64_roundtrip(val: u64) {
    let parts = split_u64(val);
    assert_eq!(join_u32s(parts[0], parts[1]), val);
}
```

## Do not use doctests
Tests should always be in dedicated test functions, doctests don't work well with rust-analyzer and other tools, and are harder to maintain.


# Implementing macros
## Test-driven development for macros
Before implementing or updating a macro the agent should write write test cases that show the expected input and output of the macro. Before writing the tests, the agent should enumerate the different scenarios that the macro should handle, and write test example input/output pairs for each scenario.

The agent should also make note of any invariants that should cause macro expansion to fail, and write test cases that ensure that the macro fails to compile when those invariants are violated.

The agent should then implement the macro to make the tests pass.

## Use fully qualified paths in generated code
When generating code in a macro, always use fully qualified paths to refer to types and functions. This avoids issues with name resolution and ensures that the generated code will compile correctly regardless of the context in which it is used. For example, instead of generating code that refers to `Vec`, generate code that refers to `::std::vec::Vec`.


## Macro Implementation practices
Be sure to break up the macro implementation into small, manageable functions that handle specific parts/cases of the macro expansion. This will make the code easier to read and maintain, and, most importantly, will make the tests easier to write and understand (it is essential for reviews that the macros give clear examples of input and output; breaking up the macro implementation into small functions helps with this).

This is an example of a test for a macro from a crate that generates Godot resource wrapper structs for nested structs. You can see how the crate has a helper function `expand_as_gd_res` that allows the comparison of generated code with expected output, and how the test uses this helper to verify that the macro generates the expected code for a complex nested struct. 

```rust
#[test]
fn test_complex_nested_struct() {
    let input: syn::DeriveInput = parse_quote! {
      pub struct EnemyParams {
          pub brain_params_required: OnEditorInit<BrainParams>,
          pub brain_params_optional: Option<BrainParams>,
          pub brains_vec: Vec<BrainParams>,
          pub drop_params: Option<DropParams2>,
          pub damage_team: DamageTeam,
      }
    };

    let actual = expand_as_gd_res(input);
    let expected = quote! {

        impl ::as_gd_res::AsGdRes for EnemyParams {
            type ResType = ::godot::prelude::OnEditor<::godot::obj::Gd<EnemyParamsResource>>;
        }

        impl ::as_gd_res::AsGdResOpt for EnemyParams {
            type GdOption = Option<::godot::obj::Gd<EnemyParamsResource>>;
        }

        impl ::as_gd_res::AsGdResArray for EnemyParams {
            type GdArray = ::godot::prelude::Array<::godot::obj::Gd<EnemyParamsResource>>;
        }


        #[derive(::godot::prelude::GodotClass)]
        #[class(tool,init,base=Resource)]

        pub struct EnemyParamsResource {
            #[base]
            base: ::godot::obj::Base<::godot::classes::Resource>,

            #[export]
            pub brain_params_required: <OnEditorInit<BrainParams> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub brain_params_optional: <Option<BrainParams> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub brains_vec: <Vec<BrainParams> as ::as_gd_res::AsGdRes>::ResType,

            #[export]
            pub drop_params: <Option<DropParams2> as ::as_gd_res::AsGdRes>::ResType,
            #[export]
            pub damage_team: <DamageTeam as ::as_gd_res::AsGdRes>::ResType,
        }

        impl ::as_gd_res::ExtractGd for EnemyParamsResource {
            type Extracted = EnemyParams;
            fn extract(&self) -> Self::Extracted {
                use ::as_gd_res::ExtractGd;
                Self::Extracted {
                    brain_params_required: self.brain_params_required.extract(),
                    brain_params_optional: self.brain_params_optional.extract(),
                    brains_vec: self.brains_vec.extract(),
                    drop_params: self.drop_params.extract(),
                    damage_team: self.damage_team.extract(),
                }
            }
        }

    };

    assert_eq!(actual.to_string(), expected.to_string());
}
```