#ifndef ORDO_NUMERUS_H_INSERTUS_EST
#define ORDO_NUMERUS_H_INSERTUS_EST


/// A [standard form](https://en.wikipedia.org/wiki/Roman_numerals#Standard_form)
/// Roman numeral in the range `1..=3999` (__I__ to __MMMCMXCIX__).
///
/// _Numerus_ represents a Roman numeral like __XVII__ or __IX__.
struct numerus {
    unsigned short vis;
};


extern struct numerus const NUMERUS_MAX;
extern struct numerus const NUMERUS_MIN;


char *
numero_loca_filum(struct numerus numerus);

struct numerus
numerum_fac(unsigned short vis);


#endif
