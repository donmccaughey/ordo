# ordo

Latin language utility library.

[![GitHub Actions][11]][12]

[11]: https://github.com/donmccaughey/ordo/actions/workflows/tests.yml/badge.svg?branch=main
[12]: https://github.com/donmccaughey/ordo/actions/workflows/tests.yml


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


## English, Latin, C and Software Jargon

Since I am a _discipulum linguae Latinae_, I endeavor to use Latin for names in
the code for `ordo`.  I'm far from a fluent Latin speaker, so commentary on my
choices is welcome.  You can find terms used in `ordo` in [the glossary][40].

[40]: ./docs/glossarium.md


## Contents

### struct `numerus`

A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form) 
Roman numeral in the range of 1 to 3999 inclusive (__I__ to __MMMCMXCIX__).
It's single field `vis` holds the numeric value as an unsigned short.

```c
#include <ordo/ordo.h>

// make a number
struct numerus n = numerum_fac(17);

// allocate a Roman numeral string for a number
char *notae_romanae = numero_loca_notam_romanam(n);

printf("%s = %i\n", notae_romanae, n.vis);  
// prints "XVII = 17"

free(notae_romanae);

// allocate a cardinal number string for a number
char *cardinalis = numero_loca_cardinalem(n, genus_m);

printf("%s = %i\n", cardinalis, n.vis);
// prints "septendecim = 17"

free(cardinalis);

// make a number from a Roman numeral string
enum error error;
struct numerus m = numerum_fac_e_notis_romanis("MMXVIII", &error);
assert(error_nullus == error);

printf("m = %i\n", m.vis);
// prints "m = 2018"
```

### enum `error`

Error codes for the `ordo` library.

```c
#include <ordo/ordo.h>

// get the message for an error code
enum error e = error_datis_vitiosis;
printf("Erroris nota %i: %s\n", e, erroris_nuntium(e));
// prints "Erroris nota 1: data vitiosa (invalid input)"
```

### struct 'linea_mutabilis'

A mutable string of characters.  Used internally in `ordo` for string building.
The struct contains a heap allocated string `linea`.  Call `free()` on `linea`
when done.

```c
#include <ordo/ordo.h>

// make an empty mutable string
struct linea_mutabilis lmut;
lineam_mutabilem_fac(&lmut);

printf("Linea = '%s'\n", lmut.linea);
// prints "Linea = ''"

// append a string to the mutable string
char const salve[] = "Salve, Munde!";
lineae_mutabili_adjunge(&lmut, salve, strlen(salve));

printf("Linea = '%s'\n", lmut.linea);
// prints "Linea = 'Salve, Munde!'"

free(lmut.linea);
```

## License

`ordo` is made available under a BSD-style license; see the `LICENSE` file for
details.
