#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "errores.h"


#define FIL_EQ(f1, f2) assert(0 == strcmp((f1), (f2)))


static void
proba_erroris_filum(void)
{
    FIL_EQ("nullus error (no error)", erroris_filum(nullus_error));
    FIL_EQ("data vitiosa (invalid input)", erroris_filum(data_vitiosa));
    FIL_EQ("numerus immodicus (number out of range)", erroris_filum(numerus_immodicus));
}


int
main(int argc, char *argv[])
{
    proba_erroris_filum();
    return EXIT_SUCCESS;
}
