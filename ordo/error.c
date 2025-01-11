#include "error.h"

#include <assert.h>
#include <stddef.h>


static char const *const nuntia_errorum[] = {
    "error nullus (no error)",
    "data vitiosa (invalid input)",
    "numerus immodicus (number out of range)",
};
static size_t const numerus_nuntiorum_errorum = sizeof nuntia_errorum / sizeof nuntia_errorum[0];


char const *
erroris_nuntium(enum error error)
{
    assert(error >= 0);
    assert(error < numerus_nuntiorum_errorum);
    return nuntia_errorum[error];
}
