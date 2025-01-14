#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "characteres.h"
#include "litterae.h"


#define LIN_AEQ(lin1, lin2) assert(0 == strcmp((lin1), (lin2)))


static void
proba_unicode_in_utf8(void)
{
    char utf8[5];

    // UTF-8 boundaries

    assert(1 == unicode_in_utf8(0x0000, utf8, 2));
    LIN_AEQ("\0", utf8);

    assert(1 == unicode_in_utf8(0x007f, utf8, 2));
    LIN_AEQ("\x7f", utf8);

    assert(0 == unicode_in_utf8(0x0080, utf8, 2));
    assert(2 == unicode_in_utf8(0x0080, utf8, 3));
    LIN_AEQ("\xc2\x80", utf8);

    assert(2 == unicode_in_utf8(0x07ff, utf8, 3));
    LIN_AEQ("\xdf\xbf", utf8);

    assert(0 == unicode_in_utf8(0x0800, utf8, 3));
    assert(3 == unicode_in_utf8(0x0800, utf8, 4));
    LIN_AEQ("\xe0\xa0\x80", utf8);

    assert(3 == unicode_in_utf8(0xffff, utf8, 4));
    LIN_AEQ("\xef\xbf\xbf", utf8);

    assert(0 == unicode_in_utf8(0x10000, utf8, 4));
    assert(4 == unicode_in_utf8(0x10000, utf8, 5));
    LIN_AEQ("\xf0\x90\x80\x80", utf8);

    // code points we care about

    assert(2 == unicode_in_utf8(A_LONGA_MAJUSCULA, utf8, 3));
    LIN_AEQ("\xc4\x80", utf8);

    assert(2 == unicode_in_utf8(A_LONGA_MINUSCULA, utf8, 3));
    LIN_AEQ("\xc4\x81", utf8);

    assert(2 == unicode_in_utf8(E_LONGA_MAJUSCULA, utf8, 3));
    LIN_AEQ("\xc4\x92", utf8);

    assert(2 == unicode_in_utf8(E_LONGA_MINUSCULA, utf8, 3));
    LIN_AEQ("\xc4\x93", utf8);

    assert(2 == unicode_in_utf8(I_LONGA_MAJUSCULA, utf8, 3));
    LIN_AEQ("\xc4\xaa", utf8);

    assert(2 == unicode_in_utf8(I_LONGA_MINUSCULA, utf8, 3));
    LIN_AEQ("\xc4\xab", utf8);

    assert(2 == unicode_in_utf8(O_LONGA_MAJUSCULA, utf8, 3));
    LIN_AEQ("\xc5\x8c", utf8);

    assert(2 == unicode_in_utf8(O_LONGA_MINUSCULA, utf8, 3));
    LIN_AEQ("\xc5\x8d", utf8);

    assert(2 == unicode_in_utf8(U_LONGA_MAJUSCULA, utf8, 3));
    LIN_AEQ("\xc5\xaa", utf8);

    assert(2 == unicode_in_utf8(U_LONGA_MINUSCULA, utf8, 3));
    LIN_AEQ("\xc5\xab", utf8);

    assert(2 == unicode_in_utf8(Y_LONGA_MAJUSCULA, utf8, 3));
    LIN_AEQ("\xc8\xb2", utf8);

    assert(2 == unicode_in_utf8(Y_LONGA_MINUSCULA, utf8, 3));
    LIN_AEQ("\xc8\xb3", utf8);

    assert(2 == unicode_in_utf8(MACRON, utf8, 3));
    LIN_AEQ("\xcc\x84", utf8);
}


int
main(int argc, char *argv[])
{
    proba_unicode_in_utf8();
    return EXIT_SUCCESS;
}
