#ifndef ORDO_LITTERAE_H_INSERTUS_EST
#define ORDO_LITTERAE_H_INSERTUS_EST


#include <stdbool.h>

#include <ordo/characteres.h>


// Precomposed long vowels

#define A_LONGA_MAJUSCULA 0x0100  // \u0100 "Ā" "\xc4\x80"
#define A_LONGA_MINUSCULA 0x0101  // \u0101 "ā" "\xc4\x81"

#define E_LONGA_MAJUSCULA 0x0112  // \u0112 "Ē" "\xc4\x92"
#define E_LONGA_MINUSCULA 0x0113  // \u0113 "ē" "\xc4\x93"

#define I_LONGA_MAJUSCULA 0x012a  // \u012a "Ī" "\xc4\xaa"
#define I_LONGA_MINUSCULA 0x012b  // \u012b "ī" "\xc4\xab"

#define O_LONGA_MAJUSCULA 0x014c  // \u014c "Ō" "\xc5\x8c"
#define O_LONGA_MINUSCULA 0x014d  // \u014d "ō" "\xc5\x8d"

#define U_LONGA_MAJUSCULA 0x016a  // \u016a "Ū" "\xc5\xaa"
#define U_LONGA_MINUSCULA 0x016b  // \u016b "ū" "\xc5\xab"

#define Y_LONGA_MAJUSCULA 0x0232  // \u0232 "Ȳ" "\xc8\xb2"
#define Y_LONGA_MINUSCULA 0x0233  // \u0233 "ȳ" "\xc8\xb3"

// Combining macron
#define MACRON 0x0304  // \u0304 "\xcc\x84"


/// Is the character uppercase?  "Is [it] uppercase?"
/// Handles ASCII characters and precomposed long vowels.
bool
estne_majuscula(unichar_t ch);

/// Is the character lowercase?  "Is [it] lowercase?"
/// Handles ASCII characters and precomposed long vowels.
bool
estne_minuscula(unichar_t ch);


#endif
