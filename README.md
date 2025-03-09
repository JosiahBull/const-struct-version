# Const-struct-version

[![CI](https://github.com/JosiahBull/const-struct-version/actions/workflows/ci.yaml/badge.svg)](https://github.com/JosiahBull/constr-struct-version/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/v/const_struct_version)](https://crates.io/crates/const_struct_version)
[![Docs.rs](https://docs.rs/const-struct-version/badge.svg)](https://docs.rs/const_struct_version)

This crate has a trait + proc-macro to generate a hash based on the fields of a struct. Useful for
intelligently expiring a cache when the stored object changes.

Note that this crate is NOT perfect and may not work for all use cases, especially given our
limitations with proc-macros.

Known limitations:

1. Custom derivations that have special logic we can't see.
2. Objects that are inherently unstable/mutable.
3. Struct/Enum names that are significant to the versioning.

Feel free to open an issue if you have a use case that doesn't work with this crate.

## Features

- `serde-attributes` - add #[serde(XXX)] attributes into the generated hash.
- `derive` - derive the `StructVersion` trait for an object automatically.
- `uuid` - Support for `uuid` crate.
- `chrono` - Support for `chrono` crate.
- `indexmap` - Support for `indexmap` crate.
- `url` - Support for `url` crate.

## Usage

```toml
[dependencies]
const_struct_version = "0.1"
```

```rust
use const_struct_version::StructVersion;

#[derive(StructVersion)]
pub struct Cart {
    items: Vec<String>,
    owner: String,
}

fn main() {
    println!("Cart version: {}", Cart::version_cached());
}
```

## Contribution

Contribution are welcome! Please feel free to open an issue or a pull request.

## License

Licensed under either of

- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.
