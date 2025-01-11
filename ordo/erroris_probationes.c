#include <assert.h>
#include <stdlib.h>
#include <string.h>

#include "error.h"


#define LIN_AEQ(f1, f2) assert(0 == strcmp((f1), (f2)))


static void
proba_erroris_nuntium(void)
{
    LIN_AEQ("error nullus (no error)", erroris_nuntium(error_nullus));
    LIN_AEQ("data vitiosa (invalid input)", erroris_nuntium(error_datis_vitiosis));
    LIN_AEQ("numerus immodicus (number out of range)", erroris_nuntium(error_numero_immodico));
}


int
main(int argc, char *argv[])
{
    proba_erroris_nuntium();
    return EXIT_SUCCESS;
}
