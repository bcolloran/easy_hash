# Running in Codex environment

The Codex environment runs the commands below:
```
git clone https://github.com/bcolloran/rapier.git ./rapier
cd ./rapier
git checkout bc9c06fbe523d7f3c5a0458e0899c9bf7fb2c7a3
cd ..
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[patch."https://github.com/bcolloran/rapier"]
rapier2d = { path = "./rapier/crates/rapier2d" }
EOF
```

This clones the `rapier` repository and checks out a specific commit, and create a `.cargo/config.toml` file including
```
[patch."https://github.com/bcolloran/rapier"]
rapier2d = { path = "./rapier/crates/rapier2d" }
```
in order to point the Codex env at the required version of the `rapier` crate (which is normally specified in the `Cargo.toml` as pointing at a specific commit of a specific fork on github. ).

This is necessary because the `rapier` crate is not published on crates.io, and the Codex environment has no network access to download it from GitHub. **The files that have been cloned into the `./rapier` folder are totally irrelevant to our project, and should always be ignored. We just need to vendor them to get the Codex environment working. In all other circumstances, we use the files directly from git. The agent must not change the version of `rapier` specified in `Cargo.toml`.**

The agent should not change the contents of the `.cargo/config.toml` file, or the contents of the `rapier` folder. The agent should confirm that the `.cargo/config.toml` file is created correctly and has the contents described, and that the `rapier` folder exists and is not empty. **If these conditions are not met, the agent must immediately halt all work and inform the user.**

# Tests

**Before making any changes to the code, the agent must run all tests to ensure that they pass. If any tests fail, the agent must immediately halt all work and inform the user.**


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
