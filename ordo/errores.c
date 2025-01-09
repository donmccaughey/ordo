#include "errores.h"

#include <assert.h>
#include <stddef.h>


static char const *const fila_errorum[] = {
    "Nullus error (no error)",
    "Data vitiosa (invalid input)",
    "Numerus immodicus (number out of range)",
};
size_t const numerus_filorum_errorum = sizeof(fila_errorum) / sizeof(fila_errorum[0]);


char const *
erroris_filum(enum errores error)
{
    assert(error >= 0);
    assert(error < numerus_filorum_errorum);
    return fila_errorum[error];
}
