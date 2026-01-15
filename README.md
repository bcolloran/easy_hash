# easy_hash

[![CI](https://github.com/bcolloran/easy_hash/actions/workflows/rust.yml/badge.svg)](https://github.com/bcolloran/easy_hash/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/bcolloran/easy_hash/branch/master/graph/badge.svg)](https://codecov.io/gh/bcolloran/easy_hash)

`easy_hash` provides a deterministic 64‑bit hash based on the
structure of a value.  It is split into two crates:

- **easy_hash** &ndash; the trait implementation and built in hashers
- **easy_hash_derive** &ndash; a proc–macro that derives `EasyHash` for your types

The hashing algorithm uses `Fletcher64` with a compile time salt derived
from the type’s name.  Implementations are available for most primitive
Rust types along with optional support for common crates.

## Optional features

The `easy_hash` crate exposes several features.  The default feature set
enables `nalgebra`, `ordered_float` and `rapier` support.  Additional
support for the Bevy ECS can be enabled with the `bevy` feature.

```toml
[dependencies]
easy_hash = { path = "easy_hash", features = ["bevy"] }
```

## Deriving `EasyHash`

```rust
use easy_hash::EasyHash;

#[derive(EasyHash)]
struct Point {
    x: f32,
    y: f32,
    #[easy_hash_ignore] // ignored when computing the hash
    cached: Option<u64>,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0, cached: None };
    println!("hash = {}", p.ehash());
}
```

The attribute `easy_hash_ignore` skips a field when computing the hash.

## Building

This workspace targets the **Rust 2024 edition** and currently requires a
nightly toolchain (1.78 or newer) to build the crates. Use Cargo to run the
tests:

```bash
cargo test
```

The tests demonstrate correctness across a variety of types including
structures, enums and tuples.
