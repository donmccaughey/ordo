#include "numerus.h"

#include <assert.h>
#include <string.h>


struct numerus const NUMERUS_MAX = { .vis=3999 };


char *
numero_loca_linea(struct numerus numerus)
{
    assert(numerus.vis <= NUMERUS_MAX.vis);

    if ( ! numerus.vis) return strdup("nihil");
    if (numerus.vis > NUMERUS_MAX.vis) return strdup("nimium");

    char cella[12];
    char *linea = cella;
    unsigned short reliquum = numerus.vis;

    while (reliquum >= 1000) {
        *linea++ = 'M';
        reliquum -= 1000;
    }

    if (reliquum >= 900) {
        *linea++ = 'C';
        *linea++ = 'M';
        reliquum -= 900;
    }

    if (reliquum >= 500) {
        *linea++ = 'D';
        reliquum -= 500;
    }

    if (reliquum >= 400) {
        *linea++ = 'C';
        *linea++ = 'D';
        reliquum -= 400;
    }

    while (reliquum >= 100) {
        *linea++ = 'C';
        reliquum -= 100;
    }

    if (reliquum >= 90) {
        *linea++ = 'X';
        *linea++ = 'C';
        reliquum -= 90;
    }

    if (reliquum >= 50) {
        *linea++ = 'L';
        reliquum -= 50;
    }

    if (reliquum >= 40) {
        *linea++ = 'X';
        *linea++ = 'L';
        reliquum -= 40;
    }

    while (reliquum >= 10) {
        *linea++ = 'X';
        reliquum -= 10;
    }

    if (reliquum >= 9) {
        *linea++ = 'I';
        *linea++ = 'X';
        reliquum -= 9;
    }

    if (reliquum >= 5) {
        *linea++ = 'V';
        reliquum -= 5;
    }

    if (reliquum >= 4) {
        *linea++ = 'I';
        *linea++ = 'V';
        reliquum -= 4;
    }

    while (reliquum >= 1) {
        *linea++ = 'I';
        reliquum -= 1;
    }

    *linea = '\0';
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
    const char linea[5];
    int longitudo_lineae;
    int saltus;
    unsigned short vis;
};


static struct segmentum_numeri const millecupla[] = {
    {"MMM", 3, 3, 3000},
    {"MM", 2, 2,2000},
    {"M", 1, 1, 1000}
};
static size_t const millecuplorum_numerus = sizeof millecupla / sizeof millecupla[0];


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
static size_t const centuplorum_numerus = sizeof centupla / sizeof centupla[0];


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
static size_t const decuplorum_numerus = sizeof decupla / sizeof decupla[0];


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
static size_t const simplorum_numerus = sizeof simpla / sizeof simpla[0];


static void
adde_segmenta(
        unsigned short *summa, char const **linea,
        struct segmentum_numeri const *segmenta, size_t numerus_segmentorum
) {
    for (int i = 0; i < numerus_segmentorum; /* vide saltum */) {
        if (0 == strncmp(*linea, segmenta[i].linea, segmenta[i].longitudo_lineae)) {
            *summa += segmenta[i].vis;
            (*linea) += segmenta[i].longitudo_lineae;
            i += segmenta[i].saltus;
        } else {
            ++i;
        }
    }
}


struct numerus
numerum_fac_e_linea(char const *linea, enum error *error)
{
    assert(linea);
    assert(linea[0]);

    if ( ! linea || ! linea[0]) {
        if (error) *error = error_datis_vitiosis;
        return numerum_fac(0);
    }

    if (error) *error = error_nullus;

    if (0 == strcmp("nihil", linea)) return numerum_fac(0);

    unsigned short summa = 0;
    char const *ch = linea;

    if ('M' == *ch) {
        adde_segmenta(&summa, &ch, millecupla, millecuplorum_numerus);
    }

    if ('D' == *ch || 'C' == *ch) {
        adde_segmenta(&summa, &ch, centupla, centuplorum_numerus);
    }

    if ('L' == *ch || 'X' == *ch) {
        adde_segmenta(&summa, &ch, decupla, decuplorum_numerus);
    }

    if ('V' == *ch || 'I' == *ch) {
        adde_segmenta(&summa, &ch, simpla, simplorum_numerus);
    }

    assert(ch - linea <= 15);  // longest accepted numeral is "MMMCCCCXXXXIIII"
    assert(summa <= NUMERUS_MAX.vis);

    if ('\0' != *ch) {
        if (error) *error = error_datis_vitiosis;
        return numerum_fac(0);
    }

    return numerum_fac(summa);
}
