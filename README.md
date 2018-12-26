[![crates.io](https://img.shields.io/crates/v/vcell.svg)](https://crates.io/crates/vcell)
[![crates.io](https://img.shields.io/crates/d/vcell.svg)](https://crates.io/crates/vcell)

# `vcell`

Just like [`Cell`] but with [volatile] read / write operations. From version 0.2.0 `vcell` has a `klee-analysis` feature which allows embedde applications to be analysed using [`cargo-klee`], with the following semantics:

* `fn get(&self) -> T` will return a new (fresh) symbolic value
* `fn set(&self, _value: T)` will have no side effect

[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
[`cargo-klee`]: https://gitlab.henriktjader.com/pln/cargo-klee.git

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
