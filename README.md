# ordo

Latin language utility library for Rust.

[![Tests](https://github.com/donmccaughey/ordo/actions/workflows/tests.yml/badge.svg)](https://github.com/donmccaughey/ordo/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/ordo)](https://crates.io/crates/ordo)
[![docs.rs](https://img.shields.io/docsrs/ordo)](https://docs.rs/ordo/*/ordo/)
[![Crates.io](https://img.shields.io/crates/l/ordo)](https://github.com/donmccaughey/ordo/blob/main/LICENSE)

## Contents

### `Numerus` struct

A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form) Roman numeral in
the range `1..=3999` (__I__ to __MMMCMXCIX__).

A [`Numerus`](https://docs.rs/ordo/*/ordo/struct.Numerus.html) is an integer value type that 
represents a Roman numeral like __XVII__ or __IX__.  _Numeri_ are unsigned integers with a 
limited range that display as Roman numerals.

```rust
use ordo::Numerus;
use std::convert::TryFrom;
use std::convert::TryInto;

let xxvi = Numerus::try_from(26).unwrap();
let cxi: Numerus = 111.try_into().unwrap();

let lxxxv = cxi - xxvi;
println!("The answer is {}", lxxxv);
// prints "The answer is LXXXV"
```
    

## License

`ordo` is made available under a BSD-style license; see the `LICENSE` file for
details.

