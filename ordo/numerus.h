#ifndef ORDO_NUMERUS_H_INSERTUS_EST
#define ORDO_NUMERUS_H_INSERTUS_EST


#include <ordo/error.h>


/// A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form)
/// Roman numeral in the range of 1 to 3999 inclusive (__I__ to __MMMCMXCIX__).
struct numerus {
    unsigned short vis;
};


extern struct numerus const NUMERUS_MAX;


/// Allocate a Roman numeral string for a number.  "For a number allocate a
/// string."  Call `free()` to deallocate the string.
char *
numero_loca_linea(struct numerus numerus);

/// Make a numerus struct.  "A number make."
struct numerus
numerum_fac(unsigned short vis);

/// Make a numerus struct from a string.  "A number make from a string."
struct numerus
numerum_fac_e_linea(char const *linea, enum error *error);


#endif
