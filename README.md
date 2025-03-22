# collect_exact

Allows zero-cost collection into exact-size arrays and tuples.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/collect_exact.svg
[crates-url]: https://crates.io/crates/collect_exact
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/Benjamin-Davies/collect_exact/blob/main/LICENSE
[actions-badge]: https://github.com/Benjamin-Davies/collect_exact/workflows/Rust/badge.svg
[actions-url]: https://github.com/Benjamin-Davies/collect_exact/actions?query=workflow%3ARust+branch%3Amain

## Usage

```rust
use collect_exact::CollectExact;

let iter = [1, 2, 3].into_iter();
let result = iter.collect_exact::<[u32; 3]>();

assert_eq!(result, Ok([1, 2, 3]));
```
