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

- _addo_ (v): add
- _aequus, -a, -um_ (adj): equal
- _casus, -us_ (n): a case
- _cella, -ae_ (n): a buffer; "a cell, a storeroom"
- _centuplus, -a, -um_ (adj): a hundred times; "a hundredfold"
- _datus, -i_ (n): a data item, a value; "a given"
- _decuplus, -a, -um_ (adj): ten times; "tenfold"
- _error, -oris_ (n): an error; "a wandering"
- _facio_ (v): make
- _filum, -i_ (n): a string (of characters)
- _finis, -is_ (n): an end
- _immodicus, -a, -um_ (adj): out of range; "excessive"
- _insero_ (v): include; "insert"
- _localis, -e_ (adj): local
- _loco_ (v): allocate (memory); "place, arrange"
- _longitudo, -inis_ (n): a length
- _maximus, -a, -um_ (adj): largest
- _millecuplus, -a, -um_ (adj): a thousand times; "a thousandfold"
- _minimus, -a, -um_ (adj): smallest
- _nihil_ (n): nothing
- _nimius, -a, -um_ (adj): too large; "excessive"
- _nota, -ae_ (n): a symbol or character
- _nullus, -a, -um_ (adj): none
- _numerus, -i_ (n): a number
- _passus, -us_ (n): a step; "a pace"
- _pravus, -a, -um_ (adj): incorrect
- _probo_ (v): test
- _probatio, -onis_ (n): a test
- _rectus, -a, -um_ (adj): correct
- _reliquum, -i_ (n): a remainder
- _saltus, -us_ (n): a jump; "a leap"
- _segmentum, -i_ (n): a segment; "a piece"
- _simplus, -a, -um_ (adj): one times; "a number taken once"
- _summa, -ae_ (n): an amount or total
- _tabula, -ae_ (n): a table
- _video_ (v): see
- _vis, vis_ (n): a magnitude; "a strength or force"
- _vitiosus, -a, -um_ (adj): invalid; "corrupt, faulty"


## Contents

### struct `numerus`

A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form) 
Roman numeral in the range of 1 to 3999 inclusive (__I__ to __MMMCMXCIX__).
It's single field `vis` holds the numeric value as an unsigned short.

```c
#include <ordo/ordo.h>

// make a number
struct numerus n = numerum_fac(17);

// allocate a string for a number
char *filum = numero_loca_filum(n);

printf("%s = %i\n", filum, n.vis);  
// prints "XVII = 17"

free(filum);

// make a number from a string
struct numerus m = numerum_fac_e_filo("MMXVIII");

printf("m = %i\n", m.vis);
// prints "m = 2018"
```

### enum `errores`

Error codes for the `ordo` library.

```c
#include <ordo/ordo.h>

// get the message for an error code
enum errores e = data_vitiosa;
printf("Error %i: %s\n", e, erroris_filum(e));
// prints "Error 1: data vitiosa (invalid input)"
```


## License

`ordo` is made available under a BSD-style license; see the `LICENSE` file for
details.
