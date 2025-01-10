#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "numerus.h"


#define FIL_AEQ(f1, f2) assert(0 == strcmp((f1), (f2)))
#define NUM_AEQ(n1, n2) assert((n1) == (n2))


static void
proba_fac_e_filo(void)
{
    struct numerus numerus;
    enum error error;

    struct {
        char const *filum;
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
        numerus = numerum_fac_e_filo(casus_recti[i].filum, &error);
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
        numerus = numerum_fac_e_filo(casus_errorum[i], &error);
        NUM_AEQ(0, numerus.vis);
        NUM_AEQ(error_datis_vitiosis, error);
    }
}


static void
proba_loca_filum(void)
{
    char *filum;

    filum = numero_loca_filum(numerum_fac(0));
    FIL_AEQ("nihil", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(1));
    FIL_AEQ("I", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(3));
    FIL_AEQ("III", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(837));
    FIL_AEQ("DCCCXXXVII", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(3899));
    FIL_AEQ("MMMDCCCXCIX", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(3999));
    FIL_AEQ("MMMCMXCIX", filum);
    free(filum);
}


static void
proba_max(void)
{
    NUM_AEQ(3999, NUMERUS_MAX.vis);
}


int
main(int argc, char *argv[])
{
    proba_fac_e_filo();
    proba_loca_filum();
    proba_max();
    return EXIT_SUCCESS;
}
