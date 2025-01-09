#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "numerus.h"


#define FIL_EQ(f1, f2) assert(0 == strcmp((f1), (f2)))
#define NUM_EQ(n1, n2) assert((n1) == (n2))


static void
proba_min_maxque(void)
{
    NUM_EQ(1, NUMERUS_MIN.vis);
    NUM_EQ(3999, NUMERUS_MAX.vis);
}


static void
proba_loca_filum(void)
{
    char *filum;

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


int
main(int argc, char *argv[])
{
    proba_min_maxque();
    proba_loca_filum();
    return EXIT_SUCCESS;
}
