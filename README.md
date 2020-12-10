# `vcell`

Just like [`Cell`] but with [volatile] read / write operations. This version (0.1.2) of `vcell` has a `klee-analysis` feature which allows embedded applications to be analysed with the following semantics:

* `fn get(&self) -> T` will return a new (fresh) symbolic value
* `fn set(&self, _value: T)` will have no side effect

[`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
[volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
[`klee-sys`]: https://gitlab.henriktjader.com/pln/klee-sys.git

## License

All changes to the original files, copyright Per Lindgen <per.lindgren@ltu.se>
All rights reserved.
