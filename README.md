# ordo

Latin language utility library.

[![Tests](https://github.com/donmccaughey/ordo/actions/workflows/tests.yml/badge.svg)](https://github.com/donmccaughey/ordo/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/ordo)](https://crates.io/crates/ordo)
[![docs.rs](https://img.shields.io/docsrs/ordo)](https://docs.rs/ordo/*/ordo/)
[![Crates.io](https://img.shields.io/crates/l/ordo)](https://github.com/donmccaughey/ordo/blob/main/LICENSE)


## Build

Ordo uses [CMake][61] 3.15 or later.  To build from source:

    git clone https://github.com/donmccaughey/ordo.git
    cd mook
    cmake -S . -B tmp
    cmake --build tmp --target all test
    cmake --install tmp

### CMake Build Options

To build with the [AddressSanitizer][62] enabled, set the `ADDRESS_SANITIZER`
option to `ON`.

    cmake -S . -B tmp -DADDRESS_SANITIZER=ON

Setting the `WALL` option to `ON` turns on [additional warnings][63] using the
`-Wall` compiler flag and also treats warnings as errors.  `WALL` is off by
default but is recommended for development and integration builds.

    cmake -S . -B tmp -DWALL=ON

To disable test targets, set the `BUILD_TESTING` option to `OFF`.

    cmake -S . -B tmp -DBUILD_TESTING=OFF

[61]: https://cmake.org
[62]: https://clang.llvm.org/docs/LeakSanitizer.html
[63]: https://gcc.gnu.org/onlinedocs/gcc/Warning-Options.html#Warning-Options


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
