#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"


#define FIL_AEQ(f1, f2) assert(0 == strcmp((f1), (f2)))


static void
proba_erroris_filum(void)
{
    FIL_AEQ("nullus error (no error)", erroris_filum(error_nullus));
    FIL_AEQ("data vitiosa (invalid input)", erroris_filum(error_datis_vitiosis));
    FIL_AEQ("numerus immodicus (number out of range)", erroris_filum(error_numero_immodico));
}


int
main(int argc, char *argv[])
{
    proba_erroris_filum();
    return EXIT_SUCCESS;
}
