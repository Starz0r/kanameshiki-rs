# 領式(Kaname Shiki)
[![GitHub](https://img.shields.io/github/license/Starz0r/kanameshiki-rs?style=flat-square)](https://github.com/Starz0r/kanameshiki-rs) [![crates.io badge](https://shields.io/crates/v/kanameshiki?style=flat-square)](https://crates.io/kanameshiki) [![Docs.rs](https://img.shields.io/docsrs/kanameshiki/latest?style=flat-square)](https://docs.rs/kanameshiki/latest) ![rustc requirements](https://img.shields.io/badge/rust-1.49+-brightgreen.svg?logo=rust&style=flat-square)

Bindings of KanameShiki that also make it usable as the global allocator for use in Rust.

## Usage

```rust
use kanameshiki::KanameShiki;

#[global_allocator]
static GLOBAL: KanameShiki = KanameShiki;
```