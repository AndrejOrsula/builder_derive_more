# builder_derive_more

<p align="left">
  <a href="https://crates.io/crates/builder_derive_more">                                   <img alt="crates.io"  src="https://img.shields.io/crates/v/builder_derive_more.svg"></a>
  <a href="https://docs.rs/builder_derive_more">                                            <img alt="docs.rs"    src="https://docs.rs/builder_derive_more/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/builder_derive_more/actions/workflows/rust.yml"> <img alt="Rust"       src="https://github.com/AndrejOrsula/builder_derive_more/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://deps.rs/repo/github/AndrejOrsula/builder_derive_more">                   <img alt="deps.rs"    src="https://deps.rs/repo/github/AndrejOrsula/builder_derive_more/status.svg"></a>
  <a href="https://codecov.io/gh/AndrejOrsula/builder_derive_more">                         <img alt="codecov.io" src="https://codecov.io/gh/AndrejOrsula/builder_derive_more/branch/main/graph/badge.svg"></a>
</p>

Additional Rust procedural macros for the builder pattern.

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

Add `builder_derive_more` as a Rust dependency to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest.

```toml
[dependencies]
builder_derive_more = "0.1"
```

### Usage

Hereafter, you can `#[derive(IntoBuilder)]` alongside deriving the builder pattern (e.g. via [derive_builder](https://docs.rs/derive_builder/latest/derive_builder)).

```rs
use builder_derive_more::IntoBuilder;
use derive_builder::Builder;

#[derive(Builder, IntoBuilder)]
pub struct Foo {
    bar: String,
}
```

This allows you to convert from the struct with named fields back into the builder for configuration.

```rs
// `IntoBuilder` derive provides the `Foo::builder()` and `Foo::configure(self)` methods.
let foo: Foo = Foo::builder().bar("bar").build().unwrap();
let foo_builder: FooBuilder = foo.configure();
```

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
