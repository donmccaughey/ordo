#include "errores.h"

#include <assert.h>
#include <stddef.h>


static char const *const fila_errorum[] = {
    "nullus error (no error)",
    "data vitiosa (invalid input)",
    "numerus immodicus (number out of range)",
};
static size_t const numerus_filorum_errorum = sizeof fila_errorum / sizeof fila_errorum[0];


char const *
erroris_filum(enum errores error)
{
    assert(error >= 0);
    assert(error < numerus_filorum_errorum);
    return fila_errorum[error];
}
