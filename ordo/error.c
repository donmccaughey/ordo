#include "error.h"

#include <assert.h>
#include <stddef.h>


static char const *const lineae_errorum[] = {
    "error nullus (no error)",
    "data vitiosa (invalid input)",
    "numerus immodicus (number out of range)",
};
static size_t const numerus_filorum_errorum = sizeof lineae_errorum / sizeof lineae_errorum[0];


char const *
erroris_nuntium(enum error error)
{
    assert(error >= 0);
    assert(error < numerus_filorum_errorum);
    return lineae_errorum[error];
}
