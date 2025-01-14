#include "litterae.h"

#include <ctype.h>


static bool
estne_ascii(unichar_t ch)
{
    return ch < 128;
}


bool
estne_majuscula(unichar_t ch)
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
