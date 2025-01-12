#ifndef ORDO_LITTERAE_H_INSERTUS_EST
#define ORDO_LITTERAE_H_INSERTUS_EST


#include <stdbool.h>
#include <stdint.h>


typedef uint32_t char_uni;


// Precomposed long vowels

#define A_LONGA_MAJUSCULA 0x0100  // \u0100
#define A_LONGA_MINUSCULA 0x0101  // \u0101

#define E_LONGA_MAJUSCULA 0x0112  // \u0112
#define E_LONGA_MINUSCULA 0x0113  // \u0113

#define I_LONGA_MAJUSCULA 0x012a  // \u012a
#define I_LONGA_MINUSCULA 0x012b  // \u012b

#define O_LONGA_MAJUSCULA 0x014c  // \u014c
#define O_LONGA_MINUSCULA 0x014d  // \u014d

#define U_LONGA_MAJUSCULA 0x016a  // \u016a
#define U_LONGA_MINUSCULA 0x016b  // \u016b

#define Y_LONGA_MAJUSCULA 0x0232  // \u0232
#define Y_LONGA_MINUSCULA 0x0233  // \u0233

// Combining macron
#define MACRON 0x0304  // \u0304


bool
estne_majuscula(char_uni ch);


#endif
