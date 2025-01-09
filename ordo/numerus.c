#include "numerus.h"

#include <assert.h>
#include <string.h>


struct numerus const NUMERUS_MAX = { .vis=3999 };


char *
numero_loca_filum(struct numerus numerus)
{
    assert(numerus.vis <= NUMERUS_MAX.vis);

    if ( ! numerus.vis) return strdup("nihil");
    if (numerus.vis > NUMERUS_MAX.vis) return strdup("nimium");

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
    assert(vis <= NUMERUS_MAX.vis);

    return (struct numerus){ .vis=vis };
}


struct segmentum_numeri {
    const char* filum;
    size_t longitudo_fili;
    unsigned short vis;
};


static struct segmentum_numeri const millecupla[] = {
    {"MMM", 3, 3000},
    {"MM", 2, 2000},
    {"M", 1, 1000}
};
static size_t const numerus_millecuplorum = sizeof millecupla / sizeof millecupla[0];


static struct segmentum_numeri const centupla[] = {
    {"CM", 2, 900},
    {"DCCC", 4, 800},
    {"DCC", 3, 700},
    {"DC", 2, 600},
    {"D", 1, 500},
    {"CD", 2, 400},
    {"CCC", 3, 300},
    {"CC", 2, 200},
    {"C", 1, 100}
};
static size_t const numerus_centuplorum = sizeof centupla / sizeof centupla[0];


static struct segmentum_numeri const decupla[] = {
    {"XC", 2, 90},
    {"LXXX", 4, 80},
    {"LXX", 3, 70},
    {"LX", 2, 60},
    {"L", 1, 50},
    {"XL", 2, 40},
    {"XXX", 3, 30},
    {"XX", 2, 20},
    {"X", 1, 10}
};
static size_t const numerus_decuplorum = sizeof decupla / sizeof decupla[0];


static struct segmentum_numeri const simpla[] = {
    {"IX", 2, 9},
    {"VIII", 4, 8},
    {"VII", 3, 7},
    {"VI", 2, 6},
    {"V", 1, 5},
    {"IV", 2, 4},
    {"III", 3, 3},
    {"II", 2, 2},
    {"I", 1, 1}
};
static size_t const numerus_simplorum = sizeof simpla / sizeof simpla[0];


struct numerus
numerum_fac_e_filio(char const *filum)
{
    assert(filum);
    assert(filum[0]);

    if ( ! filum || ! filum[0]) return numerum_fac(0);
    if (0 == strcmp("nihil", filum)) return numerum_fac(0);

    unsigned short summa = 0;
    char const *f = filum;

    // TODO: check if we exceed NUMERUS_MAX or overrun the string

    for (size_t i = 0; i < numerus_millecuplorum; i++) {
        if (0 == strncmp(f, millecupla[i].filum, millecupla[i].longitudo_fili)) {
            summa += millecupla[i].vis;
            f += millecupla[i].longitudo_fili;
            break;
        }
    }

    for (size_t i = 0; i < numerus_centuplorum; i++) {
        if (0 == strncmp(f, centupla[i].filum, centupla[i].longitudo_fili)) {
            summa += centupla[i].vis;
            f += centupla[i].longitudo_fili;
            break;
        }
    }

    for (size_t i = 0; i < numerus_decuplorum; i++) {
        if (0 == strncmp(f, decupla[i].filum, decupla[i].longitudo_fili)) {
            summa += decupla[i].vis;
            f += decupla[i].longitudo_fili;
            break;
        }
    }

    for (size_t i = 0; i < numerus_simplorum; i++) {
        if (0 == strncmp(f, simpla[i].filum, simpla[i].longitudo_fili)) {
            summa += simpla[i].vis;
            f += simpla[i].longitudo_fili;
            break;
        }
    }

    if ('\0' != *f) return numerum_fac(0);

    return numerum_fac(summa);
}
