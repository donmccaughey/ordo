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
proba_max(void)
{
    NUM_AEQ(3999, NUMERUS_MAX.vis);
}


int
main(int argc, char *argv[])
{
    proba_fac_e_linea();
    proba_loca_linea();
    proba_max();
    return EXIT_SUCCESS;
}
