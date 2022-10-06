# kanameshiki
[![GitHub](https://img.shields.io/github/license/Starz0r/kanameshiki-rs?style=flat-square)](https://github.com/Starz0r/kanameshiki-rs) [![crates.io badge](https://shields.io/crates/v/kanameshiki?style=flat-square)](https://crates.io/kanameshiki) [![Docs.rs](https://img.shields.io/docsrs/kanameshiki/latest?style=flat-square)](https://docs.rs/kanameshiki/latest) ![rustc requirements](https://img.shields.io/badge/rust-1.49+-brightgreen.svg?logo=rust&style=flat-square)

This crate provides the high-level interface. It implements the `GlobalAlloc` trait, allowing it to be used for global allocations in Rust programs.

## Usage

```rust
use kanameshiki::KanameShiki;

#[global_allocator]
static GLOBAL: KanameShiki = KanameShiki;
```