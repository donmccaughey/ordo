#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "linea_mutabilis.h"


#define LIN_AEQ(f1, f2) assert(0 == strcmp((f1), (f2)))
#define NUM_AEQ(n1, n2) assert((n1) == (n2))


static void
proba_fac(void)
{
    struct linea_mutabilis lmut;

    linea_mutabilis_fac(&lmut);
    assert(lmut.linea);
    assert(lmut.mensura == 1);
    assert(lmut.proximus == 0);

    free(lmut.linea);
}


static void
proba_adjunge(void)
{
    struct linea_mutabilis lmut;

    linea_mutabilis_fac(&lmut);

    LIN_AEQ("", lmut.linea);

    linea_mutabilis_adjunge(&lmut, "mus", LIN_LON("mus"));

    LIN_AEQ("mus", lmut.linea);
    NUM_AEQ(4, lmut.mensura);
    NUM_AEQ(3, lmut.proximus);

    linea_mutabilis_adjunge(&lmut, " parvus est.", LIN_LON(" parvus est."));

    LIN_AEQ("mus parvus est.", lmut.linea);
    NUM_AEQ(16, lmut.mensura);
    NUM_AEQ(15, lmut.proximus);

    free(lmut.linea);
}


int
main(int argc, char *argv[])
{
    proba_fac();
    proba_adjunge();
    return EXIT_SUCCESS;
}
