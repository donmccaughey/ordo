#ifndef ORDO_NUMERUS_H_INSERTUS_EST
#define ORDO_NUMERUS_H_INSERTUS_EST


#include <ordo/genus.h>
#include <ordo/error.h>


/// A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form)
/// Roman numeral in the range of 1 to 3999 inclusive (__I__ to __MMMCMXCIX__).
struct numerus {
    unsigned short vis;
};


extern struct numerus const NUMERUS_MAX;


/// Make a numerus struct.  "A number make."
struct numerus
numerum_fac(unsigned short vis);

/// Make a numerus struct from a Roman numeral string.  "A number make from
/// numerals Roman."
struct numerus
numerum_fac_e_notae_romanae(char const *notae_romanae, enum error *error);

/// Allocate a cardinal number string for a number.  "For a number allocate a
/// cardinal [number]."  Call `free()` to deallocate the string.
char *
numero_loca_cardinalem(struct numerus numerus, enum genus genus);

/// Allocate a Roman numeral string for a number.  "For a number allocate
/// numerals Roman."  Call `free()` to deallocate the string.
char *
numero_loca_notae_romanae(struct numerus numerus);


#endif
