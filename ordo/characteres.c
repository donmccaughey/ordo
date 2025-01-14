#include "characteres.h"

#include <assert.h>


int
unicode_in_utf8(unichar_t codepoint, char cella[], int cellae_mensura)
{
    assert(codepoint <= 0x10ffff);
    assert(codepoint < 0xd800 || codepoint > 0xdfff);
    assert(cella);
    assert(cellae_mensura >= 2);

    if (codepoint > 0x10ffff) return 0;
    if (codepoint >= 0xd800 && codepoint <= 0xdfff) return 0;
    if ( ! cella) return 0;
    if (cellae_mensura < 2) return 0;

    if (codepoint <= 0x7f) {
        cella[0] = (char)codepoint;
        cella[1] = '\0';
        return 1;
    }

    if (cellae_mensura < 3) return 0;
    if (codepoint <= 0x7ff) {
        cella[0] = (char)(0xc0 | ((codepoint >> 6) & 0x1f));  // top 5 bits
        cella[1] = (char)(0x80 | (codepoint & 0x3f));         // bottom 6 bits
        cella[2] = '\0';
        return 2;
    }

    if (cellae_mensura < 4) return 0;
    if (codepoint <= 0xffff) {
        cella[0] = (char)(0xe0 | ((codepoint >> 12) & 0x0f));  // top 4 bits
        cella[1] = (char)(0x80 | ((codepoint >> 6) & 0x3f));   // middle 6 bits
        cella[2] = (char)(0x80 | (codepoint & 0x3f));
        cella[3] = '\0';
        return 3;
    }

    if (cellae_mensura < 5) return 0;
    cella[0] = (char)(0xf0 | ((codepoint >> 18) & 0x07));  // top 3 bits
    cella[1] = (char)(0x80 | ((codepoint >> 12) & 0x3f));  // next 6 bits
    cella[2] = (char)(0x80 | ((codepoint >> 6) & 0x3f));   // next 6 bits
    cella[3] = (char)(0x80 | (codepoint & 0x3f));
    cella[4] = '\0';
    return 4;
}
