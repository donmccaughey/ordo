#ifndef ORDO_CHARACTERES_H_INSERTUS_EST
#define ORDO_CHARACTERES_H_INSERTUS_EST


#include <stdint.h>


/// A Unicode code point.
typedef uint32_t unichar_t;


/// Convert a Unicode code point to UTF-8.  "unicode into utf-8."
int
unicode_in_utf8(unichar_t codepoint, char cella[], int cellae_mensura);


#endif
