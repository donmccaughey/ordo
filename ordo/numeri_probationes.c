#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "numerus.h"


#define FIL_EQ(f1, f2) assert(0 == strcmp((f1), (f2)))
#define NUM_EQ(n1, n2) assert((n1) == (n2))


static void
proba_fac_e_filio(void)
{
    struct numerus numerus;

    numerus = numerum_fac_e_filio("nihil");
    NUM_EQ(0, numerus.vis);

    numerus = numerum_fac_e_filio("I");
    NUM_EQ(1, numerus.vis);

    numerus = numerum_fac_e_filio("III");
    NUM_EQ(3, numerus.vis);

    numerus = numerum_fac_e_filio("DCCCXXXVII");
    NUM_EQ(837, numerus.vis);

    numerus = numerum_fac_e_filio("MMMDCCCXCIX");
    NUM_EQ(3899, numerus.vis);

    numerus = numerum_fac_e_filio("MMMCMXCIX");
    NUM_EQ(3999, numerus.vis);
}


static void
proba_loca_filum(void)
{
    char *filum;

    filum = numero_loca_filum(numerum_fac(0));
    FIL_EQ("nihil", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(1));
    FIL_EQ("I", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(3));
    FIL_EQ("III", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(837));
    FIL_EQ("DCCCXXXVII", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(3899));
    FIL_EQ("MMMDCCCXCIX", filum);
    free(filum);

    filum = numero_loca_filum(numerum_fac(3999));
    FIL_EQ("MMMCMXCIX", filum);
    free(filum);
}


static void
proba_max(void)
{
    NUM_EQ(3999, NUMERUS_MAX.vis);
}


int
main(int argc, char *argv[])
{
    proba_fac_e_filio();
    proba_loca_filum();
    proba_max();
    return EXIT_SUCCESS;
}
