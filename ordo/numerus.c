#include "numerus.h"

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "linea_mutabilis.h"


struct numerus const NUMERUS_MAX = { .vis=3999 };


static void *
xstrdup(char const *linea)
{
    assert(linea);
    char *duplicata = strdup(linea);
    if ( ! duplicata) abort();
    return duplicata;
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


char *
numero_loca_linea(struct numerus numerus)
{
    assert(numerus.vis <= NUMERUS_MAX.vis);

    if ( ! numerus.vis) return xstrdup("nihil");
    if (numerus.vis > NUMERUS_MAX.vis) return xstrdup("nimium");

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

    return xstrdup(cella);
}


#define ADJUNGE(lmut, linea_fixa) \
        linea_mutabilis_adjunge((lmut), (linea_fixa), LIN_LON((linea_fixa)))


static void
adjunge_denos(
        struct linea_mutabilis *lmut, int *reliquum,
        int deni, char const *verbum, int verbi_longitudo
) {
    assert(*reliquum < deni + 10 - 2);
    if (*reliquum >= deni - 2) {
        if (deni - 2 == *reliquum) {
            ADJUNGE(lmut, "duode");
            *reliquum += 2;
        } else if (deni - 1 == *reliquum) {
            ADJUNGE(lmut, "unde");
            *reliquum += 1;
        }
        linea_mutabilis_adjunge(lmut, verbum, verbi_longitudo);
        *reliquum -= deni;
        if (*reliquum) ADJUNGE(lmut, " ");
    }
}


#define ADJUNGE_DENOS(lmut, reliquum, deni, linea_fixa) \
        adjunge_denos((lmut), (reliquum), (deni), (linea_fixa), LIN_LON((linea_fixa)))


char *
numero_loca_cardinalem(struct numerus numerus, enum genus genus)
{
    assert(numerus.vis <= NUMERUS_MAX.vis);
    assert(genus > genus_nullus);
    assert(genus <= genus_n);

    if (0 == numerus.vis) return xstrdup("nihil");

    struct linea_mutabilis lmut;
    linea_mutabilis_fac(&lmut);
    int reliquum = numerus.vis;

    ADJUNGE_DENOS(&lmut, &reliquum, 90, "nonaginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 80, "octoginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 70, "septuaginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 60, "sexaginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 50, "quinquaginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 40, "quadraginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 30, "triginta");
    ADJUNGE_DENOS(&lmut, &reliquum, 20, "viginti");

    assert(reliquum < 18);
    switch (reliquum) {
        case 17: ADJUNGE(&lmut, "septendecim"); reliquum -= 17; break;
        case 16: ADJUNGE(&lmut, "sedecim"); reliquum -= 16; break;
        case 15: ADJUNGE(&lmut, "quindecim"); reliquum -= 15; break;
        case 14: ADJUNGE(&lmut, "quattuordecim"); reliquum -= 14; break;
        case 13: ADJUNGE(&lmut, "tredecim"); reliquum -= 13; break;
        case 12: ADJUNGE(&lmut, "duodecim"); reliquum -= 12; break;
        case 11: ADJUNGE(&lmut, "undecim"); reliquum -= 11; break;

        case 10: ADJUNGE(&lmut, "decem"); reliquum -= 10; break;
        case 9: ADJUNGE(&lmut, "novem"); reliquum -= 9; break;
        case 8: ADJUNGE(&lmut, "octo"); reliquum -= 8; break;
        case 7: ADJUNGE(&lmut, "septem"); reliquum -= 7; break;
        case 6: ADJUNGE(&lmut, "sex"); reliquum -= 6; break;
        case 5: ADJUNGE(&lmut, "quinque"); reliquum -= 5; break;
        case 4: ADJUNGE(&lmut, "quattuor"); reliquum -= 4; break;
        case 3:
            ADJUNGE(&lmut, (genus_m == genus || genus_f == genus) ? "tres" : "tria");
            reliquum -= 3;
            break;
        case 2:
            ADJUNGE(&lmut, (genus_m == genus || genus_n == genus) ? "duo" : "duae");
            reliquum -= 2;
            break;
        case 1:
            ADJUNGE(&lmut, (genus_m == genus) ? "unus" : ((genus_f == genus) ? "una" : "unum"));
            reliquum -= 1;
            break;
        default:
            assert("Non deberet attingere");
            break;
    }

    assert( ! reliquum);
    return lmut.linea;
}
