# ordo

Latin language utility library.


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


## Glossarium

Here are the Latin terms I use in the `ordo` library and their Engish software
jargon equivalents.  For the full meaning and connotations of these words, I
recommend looking them up in [λογεῖον](https://logeion.uchicago.edu).

- _cella_, _-ae_ (n): a buffer
- _datum_, _-i_ (n): a value
- _error_, _-oris_ (n): an error
- _filum_, _-i_ (n): a string (of characters)
- _immodicus_, _-a_, _-um_ (adj): excessive
- _insero_ (v): to include (i.e. to insert)
- _loco_ (v): to allocate (memory)
- _maximus_, _-a_, _-um_ (adj): largest
- _minimus_, _-a_, _-um_ (adj): smallest
- _nota_, _-ae_ (n): a symbol or character
- _nullus_, _-a_, _-um_ (adj): none
- _numerus_, _-i_ (n): a number
- _probo_ (v): to test
- _probatio_, _-onis_ (n): a test
- _reliquum_, _-i_ (n): a remainder
- _summa_, _-ae_ (n): an amount or total
- _tabula_, _-ae_ (n): a table
- _vis_, _vis_ (n): an amount
- _vitiosus_, _-a_, _-um_ (adj): invalid, corrupt


## Contents

### struct `numerus`

A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form) 
Roman numeral in the range `1..=3999` (__I__ to __MMMCMXCIX__).

A `numerus` struct holds an unsigned short value `vis`
represents a Roman numeral like __XVII__ or __IX__.  _Numeri_ are unsigned integers with a
limited range that display as Roman numerals.

```c
#include <ordo/ordo.h>

struct numerus n = numerum_fac(17);

// for a number (numero) allocate (loca) a string (filum)
char *filum = numero_loca_filum(n);

printf("%s\n", filum);  
// prints "XVII"

free(filum);
```


## License

`ordo` is made available under a BSD-style license; see the `LICENSE` file for
details.
