#include "litterae.h"

#include <ctype.h>


static bool
estne_ascii(char_uni ch)
{
    return ch < 128;
}


bool
estne_majuscula(char_uni ch)
{
    if (estne_ascii(ch)) {
        return isupper((int)ch);
    } else {
        return ch == A_LONGA_MAJUSCULA
            || ch == E_LONGA_MAJUSCULA
            || ch == I_LONGA_MAJUSCULA
            || ch == O_LONGA_MAJUSCULA
            || ch == U_LONGA_MAJUSCULA
            || ch == Y_LONGA_MAJUSCULA;
    }
}
