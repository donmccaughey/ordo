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


static struct segmentum_numeri const milleni[] = {
    {"MMM", 3, 3, 3000},
    {"MM", 2, 2,2000},
    {"M", 1, 1, 1000}
};
static size_t const millenorum_numerus = sizeof milleni / sizeof milleni[0];


static struct segmentum_numeri const centeni[] = {
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
static size_t const centenorum_numerus = sizeof centeni / sizeof centeni[0];


static struct segmentum_numeri const deni[] = {
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
static size_t const denorum_numerus = sizeof deni / sizeof deni[0];


static struct segmentum_numeri const singuli[] = {
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
static size_t const singulorum_numerus = sizeof singuli / sizeof singuli[0];


static void
adde_segmenta(
        unsigned short *summa, char const **notae_romanae,
        struct segmentum_numeri const *segmenta, size_t numerus_segmentorum
) {
    for (int i = 0; i < numerus_segmentorum; /* vide saltum */) {
        if (0 == strncmp(*notae_romanae, segmenta[i].linea, segmenta[i].longitudo_lineae)) {
            *summa += segmenta[i].vis;
            (*notae_romanae) += segmenta[i].longitudo_lineae;
            i += segmenta[i].saltus;
        } else {
            ++i;
        }
    }
}


struct numerus
numerum_fac_e_notae_romanae(char const *notae_romanae, enum error *error)
{
    assert(notae_romanae);
    assert(notae_romanae[0]);

    if ( ! notae_romanae || ! notae_romanae[0]) {
        if (error) *error = error_datis_vitiosis;
        return numerum_fac(0);
    }

    if (error) *error = error_nullus;

    if (0 == strcmp("nihil", notae_romanae)) return numerum_fac(0);

    unsigned short summa = 0;
    char const *ch = notae_romanae;

    if ('M' == *ch) {
        adde_segmenta(&summa, &ch, milleni, millenorum_numerus);
    }

    if ('D' == *ch || 'C' == *ch) {
        adde_segmenta(&summa, &ch, centeni, centenorum_numerus);
    }

    if ('L' == *ch || 'X' == *ch) {
        adde_segmenta(&summa, &ch, deni, denorum_numerus);
    }

    if ('V' == *ch || 'I' == *ch) {
        adde_segmenta(&summa, &ch, singuli, singulorum_numerus);
    }

    assert(ch - notae_romanae <= 15);  // longest accepted numeral is "MMMCCCCXXXXIIII"
    assert(summa <= NUMERUS_MAX.vis);

    if ('\0' != *ch) {
        if (error) *error = error_datis_vitiosis;
        return numerum_fac(0);
    }

    return numerum_fac(summa);
}


char *
numero_loca_notae_romanae(struct numerus numerus)
{
    assert(numerus.vis <= NUMERUS_MAX.vis);

    if ( ! numerus.vis) return xstrdup("nihil");
    if (numerus.vis > NUMERUS_MAX.vis) return xstrdup("nimius");

    char cella[12];
    char *notae_romanae = cella;
    unsigned short reliquum = numerus.vis;

    while (reliquum >= 1000) {
        *notae_romanae++ = 'M';
        reliquum -= 1000;
    }

    if (reliquum >= 900) {
        *notae_romanae++ = 'C';
        *notae_romanae++ = 'M';
        reliquum -= 900;
    } else if (reliquum >= 500) {
        *notae_romanae++ = 'D';
        reliquum -= 500;
    } else if (reliquum >= 400) {
        *notae_romanae++ = 'C';
        *notae_romanae++ = 'D';
        reliquum -= 400;
    }

    while (reliquum >= 100) {
        *notae_romanae++ = 'C';
        reliquum -= 100;
    }

    if (reliquum >= 90) {
        *notae_romanae++ = 'X';
        *notae_romanae++ = 'C';
        reliquum -= 90;
    } else if (reliquum >= 50) {
        *notae_romanae++ = 'L';
        reliquum -= 50;
    } else if (reliquum >= 40) {
        *notae_romanae++ = 'X';
        *notae_romanae++ = 'L';
        reliquum -= 40;
    }

    while (reliquum >= 10) {
        *notae_romanae++ = 'X';
        reliquum -= 10;
    }

    if (reliquum >= 9) {
        *notae_romanae++ = 'I';
        *notae_romanae++ = 'X';
        reliquum -= 9;
    } else if (reliquum >= 5) {
        *notae_romanae++ = 'V';
        reliquum -= 5;
    } else if (reliquum >= 4) {
        *notae_romanae++ = 'I';
        *notae_romanae++ = 'V';
        reliquum -= 4;
    }

    while (reliquum >= 1) {
        *notae_romanae++ = 'I';
        reliquum -= 1;
    }

    *notae_romanae = '\0';
    assert(strlen(cella) < sizeof cella);

    return xstrdup(cella);
}


#define ADJUNGE(lmut, linea_fixa) \
        lineae_mutabili_adjunge((lmut), (linea_fixa), LIN_LON((linea_fixa)))


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
        lineae_mutabili_adjunge(lmut, verbum, verbi_longitudo);
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
    lineam_mutabilem_fac(&lmut);
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
            if (genus_n == genus) {
                ADJUNGE(&lmut, "tria");
            } else {
                ADJUNGE(&lmut, "tres");
            }
            reliquum -= 3;
            break;
        case 2:
            if (genus_f == genus) {
                ADJUNGE(&lmut, "duae");
            } else {
                ADJUNGE(&lmut, "duo");
            }
            reliquum -= 2;
            break;
        case 1:
            if (genus_m == genus) {
                ADJUNGE(&lmut, "unus");
            } else if (genus_f == genus) {
                ADJUNGE(&lmut, "una");
            } else {
                ADJUNGE(&lmut, "unum");
            }
            reliquum -= 1;
            break;
        case 0:
            break;
        default:
            // non deberet attingere hunc casum
            assert(0);
            break;
    }

    assert( ! reliquum);
    return lmut.linea;
}
