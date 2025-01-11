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
lineae_mutabili_adjunge(
        struct linea_mutabilis *lmut,
        char const *linea, int lineae_longitudo
) {
    assert(lmut);
    assert(linea);
    assert(lineae_longitudo >= 0);
    assert(lineae_longitudo == strlen(linea));

    if ( ! linea && ! linea[0] && ! lineae_longitudo) return;

    lmut->mensura += lineae_longitudo;
    lmut->linea = xrealloc(lmut->linea, lmut->mensura);
    strncpy(&lmut->linea[lmut->proximus], linea, lineae_longitudo);
    lmut->proximus += lineae_longitudo;

    assert(lmut->proximus <= lmut->mensura - 1);
    lmut->linea[lmut->proximus] = '\0';
}
