#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "numerus.h"


#define LIN_AEQ(f1, f2) assert(0 == strcmp((f1), (f2)))
#define NUM_AEQ(n1, n2) assert((n1) == (n2))


static void
proba_fac_e_linea(void)
{
    struct numerus numerus;
    enum error error;

    struct {
        char const *linea;
        unsigned short vis;
    } const casus_recti[] = {
        {"nihil", 0},
        {"I", 1},
        {"III", 3},
        {"IV", 4},
        {"IIII", 4},
        {"XXXIV", 34},
        {"XXXX", 40},
        {"XL", 40},
        {"XLII", 42},
        {"XXXXII", 42},
        {"CCCC", 400},
        {"CD", 400},
        {"DCCCXXXVII", 837},
        {"MMMCDXLIV", 3444},
        {"MMMCCCCXXXXIIII", 3444},
        {"MMMDCCCXCIX", 3899},
        {"MMMCMXCIX", 3999},
    };
    int casuum_rectorum_numerus = sizeof casus_recti / sizeof casus_recti[0];
    for (int i = 0; i < casuum_rectorum_numerus; ++i) {
        numerus = numerum_fac_e_linea(casus_recti[i].linea, &error);
        NUM_AEQ(casus_recti[i].vis, numerus.vis);
        NUM_AEQ(error_nullus, error);
    }

    char const *casus_errorum[] = {
        "MMMM",
        "CCCCC",
        "XXXXX",
        "IIIII",
        "IIIIIIIIIIIIIIIIIIII",  // length = 20
        "foobar",
    };
    int casuum_errorum_numerus = sizeof casus_errorum / sizeof casus_errorum[0];

    for (int i = 0; i < casuum_errorum_numerus; ++i) {
        error = error_nullus;
        numerus = numerum_fac_e_linea(casus_errorum[i], &error);
        NUM_AEQ(0, numerus.vis);
        NUM_AEQ(error_datis_vitiosis, error);
    }
}


static void
proba_loca_linea(void)
{
    char *linea;

    linea = numero_loca_linea(numerum_fac(0));
    LIN_AEQ("nihil", linea);
    free(linea);

    linea = numero_loca_linea(numerum_fac(1));
    LIN_AEQ("I", linea);
    free(linea);

    linea = numero_loca_linea(numerum_fac(3));
    LIN_AEQ("III", linea);
    free(linea);

    linea = numero_loca_linea(numerum_fac(837));
    LIN_AEQ("DCCCXXXVII", linea);
    free(linea);

    linea = numero_loca_linea(numerum_fac(3899));
    LIN_AEQ("MMMDCCCXCIX", linea);
    free(linea);

    linea = numero_loca_linea(numerum_fac(3999));
    LIN_AEQ("MMMCMXCIX", linea);
    free(linea);
}


static void
proba_loca_cardinalem(void)
{
    struct {
        unsigned short vis;
        enum genus genus;
        char const *cardinalis;
    } const casus_recti[] = {
        {0,    genus_m, "nihil"},

        {1,    genus_m, "unus"},
        {1,    genus_f, "una"},
        {1,    genus_n, "unum"},

        {2,    genus_m, "duo"},
        {2,    genus_f, "duae"},
        {2,    genus_n, "duo"},

        {3,    genus_m, "tres"},
        {3,    genus_f, "tres"},
        {3,    genus_n, "tria"},

        {4,    genus_m, "quattuor"},
        {5,    genus_m, "quinque"},
        {6,    genus_m, "sex"},
        {7,    genus_m, "septem"},
        {8,    genus_m, "octo"},
        {9,    genus_m, "novem"},

        {10,    genus_m, "decem"},

        {11,   genus_m, "undecim"},
        {11,   genus_f, "undecim"},
        {11,   genus_n, "undecim"},

        {12,   genus_m, "duodecim"},
        {12,   genus_f, "duodecim"},
        {12,   genus_n, "duodecim"},

        {13,   genus_m, "tredecim"},
        {13,   genus_f, "tredecim"},
        {13,   genus_n, "tredecim"},

        {14,   genus_n, "quattuordecim"},
        {15,   genus_n, "quindecim"},
        {16,   genus_n, "sedecim"},
        {17,   genus_n, "septendecim"},
        {18,   genus_n, "duodeviginti"},
        {19,   genus_n, "undeviginti"},

        {20,   genus_n, "viginti"},

        {21,   genus_m, "viginti unus"},
        {21,   genus_f, "viginti una"},
        {21,   genus_n, "viginti unum"},

        {22,   genus_m, "viginti duo"},
        {22,   genus_f, "viginti duae"},
        {22,   genus_n, "viginti duo"},

        {23,   genus_m, "viginti tres"},
        {23,   genus_f, "viginti tres"},
        {23,   genus_n, "viginti tria"},

        {24,   genus_m, "viginti quattuor"},
        {25,   genus_m, "viginti quinque"},
        {26,   genus_m, "viginti sex"},
        {27,   genus_m, "viginti septem"},
        {28,   genus_m, "duodetriginta"},
        {29,   genus_m, "undetriginta"},

        {30,   genus_m, "triginta"},

        {31,   genus_m, "triginta unus"},
        {31,   genus_f, "triginta una"},
        {31,   genus_n, "triginta unum"},

        {32,   genus_m, "triginta duo"},
        {32,   genus_f, "triginta duae"},
        {32,   genus_n, "triginta duo"},

        {33,   genus_m, "triginta tres"},
        {33,   genus_f, "triginta tres"},
        {33,   genus_n, "triginta tria"},

        {34,   genus_m, "triginta quattuor"},
        {35,   genus_m, "triginta quinque"},
        {36,   genus_m, "triginta sex"},
        {37,   genus_m, "triginta septem"},
        {38,   genus_m, "duodequadraginta"},
        {39,   genus_m, "undequadraginta"},

        {40,   genus_m, "quadraginta"},
        {41,   genus_m, "quadraginta unus"},
        {42,   genus_f, "quadraginta duae"},
        {43,   genus_n, "quadraginta tria"},
        {47,   genus_m, "quadraginta septem"},
        {48,   genus_f, "duodequinquaginta"},
        {49,   genus_n, "undequinquaginta"},

        {50,   genus_m, "quinquaginta"},
        {60,   genus_m, "sexaginta"},
        {70,   genus_m, "septuaginta"},
        {80,   genus_m, "octoginta"},
        {90,   genus_m, "nonaginta"},

        {97,   genus_m, "nonaginta septem"},

        /*

        {837,  genus_m, "octingenti triginta septem"},

        {3899, genus_m, "trecenti octoginta novem"},
        {3999, genus_m, "trecenti nonaginta novem"},
        */
    };
    int casuum_rectorum_numerus = sizeof casus_recti / sizeof casus_recti[0];

    for (int i = 0; i < casuum_rectorum_numerus; ++i) {
        struct numerus n = numerum_fac(casus_recti[i].vis);
        char *cardinalis = numero_loca_cardinalem(n, casus_recti[i].genus);
        LIN_AEQ(casus_recti[i].cardinalis, cardinalis);
        free(cardinalis);
    }
}


static void
proba_max(void)
{
    NUM_AEQ(3999, NUMERUS_MAX.vis);
}


int
main(int argc, char *argv[])
{
    proba_fac_e_linea();
    proba_loca_linea();
    proba_loca_cardinalem();
    proba_max();
    return EXIT_SUCCESS;
}
