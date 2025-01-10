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
    const char filum[5];
    int longitudo_fili;
    int saltus;
    unsigned short vis;
};


static struct segmentum_numeri const millecupla[] = {
    {"MMM", 3, 2, 3000},
    {"MM", 2, 2,2000},
    {"M", 1, 1, 1000}
};
static size_t const numerus_millecuplorum = sizeof millecupla / sizeof millecupla[0];


static struct segmentum_numeri const centupla[] = {
    {"CM", 2, 9, 900},
    {"DCCC", 4, 8, 800},
    {"DCC", 3, 7, 700},
    {"DC", 2, 6, 600},
    {"D", 1, 5, 500},
    {"CD", 2, 4, 400},
    {"CCC", 3, 2, 300},
    {"CC", 2, 2, 200},
    {"C", 1, 1, 100}
};
static size_t const numerus_centuplorum = sizeof centupla / sizeof centupla[0];


static struct segmentum_numeri const decupla[] = {
    {"XC", 2, 9, 90},
    {"LXXX", 4, 8, 80},
    {"LXX", 3, 7, 70},
    {"LX", 2, 6, 60},
    {"L", 1, 5, 50},
    {"XL", 2, 4, 40},
    {"XXX", 3, 2, 30},
    {"XX", 2, 2, 20},
    {"X", 1, 1, 10}
};
static size_t const numerus_decuplorum = sizeof decupla / sizeof decupla[0];


static struct segmentum_numeri const simpla[] = {
    {"IX", 2, 9, 9},
    {"VIII", 4, 8, 8},
    {"VII", 3, 7, 7},
    {"VI", 2, 6, 6},
    {"V", 1, 5, 5},
    {"IV", 2, 4, 4},
    {"III", 3, 2, 3},
    {"II", 2, 2, 2},
    {"I", 1, 1, 1}
};
static size_t const numerus_simplorum = sizeof simpla / sizeof simpla[0];


static void
adde_segmenta(
        unsigned short *summa, char const **f, char const *f_max,
        struct segmentum_numeri const *segmenta, size_t numerus_segmentorum
) {
    for (int i = 0; i < numerus_segmentorum; /* nihil */) {
        if (*summa > NUMERUS_MAX.vis) return;
        if (*f > f_max) return;
        if (0 == strncmp(*f, segmenta[i].filum, segmenta[i].longitudo_fili)) {
            *summa += segmenta[i].vis;
            (*f) += segmenta[i].longitudo_fili;
            i += segmenta[i].saltus;
        } else {
            ++i;
        }
    }
}


struct numerus
numerum_fac_e_filio(char const *filum)
{
    assert(filum);
    assert(filum[0]);

    if ( ! filum || ! filum[0]) return numerum_fac(0);
    if (0 == strcmp("nihil", filum)) return numerum_fac(0);

    unsigned short summa = 0;
    char const *f = filum;
    char const *f_max = f + 15;

    if ('M' == *f) {
        adde_segmenta(&summa, &f, f_max, millecupla, numerus_millecuplorum);
    }

    if ('D' == *f || 'C' == *f) {
        adde_segmenta(&summa, &f, f_max, centupla, numerus_centuplorum);
    }

    if ('L' == *f || 'X' == *f) {
        adde_segmenta(&summa, &f, f_max, decupla, numerus_decuplorum);
    }

    if ('V' == *f || 'I' == *f) {
        adde_segmenta(&summa, &f, f_max, simpla, numerus_simplorum);
    }

    if ('\0' != *f) return numerum_fac(0);
    if (summa > NUMERUS_MAX.vis) return numerum_fac(0);
    return numerum_fac(summa);
}
