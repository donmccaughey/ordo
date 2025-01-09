#include "numerus.h"

#include <assert.h>
#include <string.h>


struct numerus const NUMERUS_MAX = { .vis=3999 };
struct numerus const NUMERUS_MIN = { .vis=1 };


char *
numero_loca_filum(struct numerus numerus)
{
    assert(numerus.vis >= NUMERUS_MIN.vis);
    assert(numerus.vis <= NUMERUS_MAX.vis);

    char cella[12];
    char *filum = cella;
    unsigned short reliquum = numerus.vis;

    while (reliquum >= 1000) {
        *filum++ = 'M';
        reliquum -= 1000;
    }

    if (reliquum >= 900) {
        *filum++ = 'C';
        *filum++ = 'M';
        reliquum -= 900;
    }

    if (reliquum >= 500) {
        *filum++ = 'D';
        reliquum -= 500;
    }

    if (reliquum >= 400) {
        *filum++ = 'C';
        *filum++ = 'D';
        reliquum -= 400;
    }

    while (reliquum >= 100) {
        *filum++ = 'C';
        reliquum -= 100;
    }

    if (reliquum >= 90) {
        *filum++ = 'X';
        *filum++ = 'C';
        reliquum -= 90;
    }

    if (reliquum >= 50) {
        *filum++ = 'L';
        reliquum -= 50;
    }

    if (reliquum >= 40) {
        *filum++ = 'X';
        *filum++ = 'L';
        reliquum -= 40;
    }

    while (reliquum >= 10) {
        *filum++ = 'X';
        reliquum -= 10;
    }

    if (reliquum >= 9) {
        *filum++ = 'I';
        *filum++ = 'X';
        reliquum -= 9;
    }

    if (reliquum >= 5) {
        *filum++ = 'V';
        reliquum -= 5;
    }

    if (reliquum >= 4) {
        *filum++ = 'I';
        *filum++ = 'V';
        reliquum -= 4;
    }

    while (reliquum >= 1) {
        *filum++ = 'I';
        reliquum -= 1;
    }

    *filum = '\0';
    assert(strlen(cella) < sizeof cella);

    return strdup(cella);
}


struct numerus
numerum_fac(unsigned short vis)
{
    return (struct numerus){ .vis=vis };
}
