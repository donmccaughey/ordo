# ordo

Latin language utility library for Rust.

[![Tests](https://github.com/donmccaughey/ordo/actions/workflows/tests.yml/badge.svg)](https://github.com/donmccaughey/ordo/actions/workflows/tests.yml)

## Usage

### `Numerus` struct

_Numeri_ are integer Roman numbers in the inclusive range `1..=3999`.  Like other Rust integers,
math operations on _Numeri_ will panic on overflow in debug builds.  

```rust
use ordo::Numerus;
let xlii = Numerus::try_from(42).unwrap();
let ix = Numerus::try_from(9).unwrap();
let xxxiii = xlii - ix;
println!("The answer is {}", xxxiii);
// prints "XXXIII"
```
    

## License

`ordo` is made available under a BSD-style license; see the `LICENSE` file for
details.

