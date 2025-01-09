#ifndef ORDO_NUMERUS_H_INSERTUS_EST
#define ORDO_NUMERUS_H_INSERTUS_EST


/// A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form)
/// Roman numeral in the range of 1 to 3999 inclusive (__I__ to __MMMCMXCIX__).
struct numerus {
    unsigned short vis;
};


extern struct numerus const NUMERUS_MAX;


/// Allocate a Roman numeral string for a number.  "For a number allocate a
/// string."  Call `free()` to deallocate the string.
char *
numero_loca_filum(struct numerus numerus);

/// Make a numerus struct.  "Make a number."
struct numerus
numerum_fac(unsigned short vis);


#endif
