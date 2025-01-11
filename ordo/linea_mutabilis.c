#include "linea_mutabilis.h"

#include <assert.h>
#include <stdlib.h>
#include <string.h>


static void *
xcalloc(size_t n, size_t mensura)
{
    void *memoria = calloc(n, mensura);
    if ( ! memoria) abort();
    return memoria;
}


static void *
xrealloc(void *memoria, size_t mensura)
{
    void *memoria_nova = realloc(memoria, mensura);
    if ( ! memoria_nova) abort();
    return memoria_nova;
}


void
linea_mutabilis_fac(struct linea_mutabilis *lmut)
{
    assert(lmut);
    lmut->linea = xcalloc(1, 1);
    lmut->mensura = 1;
    lmut->proximus = 0;
}


void
linea_mutabilis_adjunge(
        struct linea_mutabilis *lmut,
        char const *linea, int lineae_longitudo
) {
    lmut->mensura += lineae_longitudo;
    lmut->linea = xrealloc(lmut->linea, lmut->mensura);
    strcpy(&lmut->linea[lmut->proximus], linea);
    lmut->proximus += lineae_longitudo;
}
