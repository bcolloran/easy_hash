# Running in Codex environment

The Codex environment runs the steps commands below to clone the `rapier` repository and checks out a specific commit, and create a `.cargo/config.toml` file including
```
[patch.crates-io]
rapier2d = { path = "./rapier/crates/rapier2d" }
```
in order to point the Codex env at the required version of the `rapier` crate.


```
git clone https://github.com/bcolloran/rapier.git ./rapier
cd ./rapier
git checkout bc9c06fbe523d7f3c5a0458e0899c9bf7fb2c7a3
cd ..
mkdir -p .cargo
cat > .cargo/config.toml << EOF
[patch.crates-io]
rapier2d = { path = "./rapier/crates/rapier2d" }
EOF
```

This is necessary because the `rapier` crate is not published on crates.io, and the Codex environment has no network access to download it from GitHub. **The files that have been cloned into the `./rapier` folder are totally irrelevant to our project, and should always be ignored. We just need to vendor them to get the Codex environment working. In all other circumstances, we use the files directly from git**


# Tests

## File layout
Try to ensure that the file organization for tests matches that of the implementation code, e.g., if `EasyHash` for `OrderedFloat` is implemented in the file `ordered_float.rs`, 
put tests in `tests/test_ordered_float.rs`.
