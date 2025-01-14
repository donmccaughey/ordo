#include <assert.h>
#include <stdlib.h>

#include "ordo.h"


static void
proba_estne_majuscula(void)
{
    assert(estne_majuscula('A'));
    assert(estne_majuscula(A_LONGA_MAJUSCULA));
    assert(estne_majuscula('E'));
    assert(estne_majuscula(E_LONGA_MAJUSCULA));
    assert(estne_majuscula('I'));
    assert(estne_majuscula(I_LONGA_MAJUSCULA));
    assert(estne_majuscula('O'));
    assert(estne_majuscula(O_LONGA_MAJUSCULA));
    assert(estne_majuscula('U'));
    assert(estne_majuscula(U_LONGA_MAJUSCULA));
    assert(estne_majuscula('Y'));
    assert(estne_majuscula(Y_LONGA_MAJUSCULA));

    assert( ! estne_majuscula('a'));
    assert( ! estne_majuscula(A_LONGA_MINUSCULA));
    assert( ! estne_majuscula('e'));
    assert( ! estne_majuscula(E_LONGA_MINUSCULA));
    assert( ! estne_majuscula('i'));
    assert( ! estne_majuscula(I_LONGA_MINUSCULA));
    assert( ! estne_majuscula('o'));
    assert( ! estne_majuscula(O_LONGA_MINUSCULA));
    assert( ! estne_majuscula('u'));
    assert( ! estne_majuscula(U_LONGA_MINUSCULA));
    assert( ! estne_majuscula('y'));
    assert( ! estne_majuscula(Y_LONGA_MINUSCULA));
}


static void
proba_estne_minuscula(void)
{
    assert(estne_minuscula('a'));
    assert(estne_minuscula(A_LONGA_MINUSCULA));
    assert(estne_minuscula('e'));
    assert(estne_minuscula(E_LONGA_MINUSCULA));
    assert(estne_minuscula('i'));
    assert(estne_minuscula(I_LONGA_MINUSCULA));
    assert(estne_minuscula('o'));
    assert(estne_minuscula(O_LONGA_MINUSCULA));
    assert(estne_minuscula('u'));
    assert(estne_minuscula(U_LONGA_MINUSCULA));
    assert(estne_minuscula('y'));
    assert(estne_minuscula(Y_LONGA_MINUSCULA));

    assert( ! estne_minuscula('A'));
    assert( ! estne_minuscula(A_LONGA_MAJUSCULA));
    assert( ! estne_minuscula('E'));
    assert( ! estne_minuscula(E_LONGA_MAJUSCULA));
    assert( ! estne_minuscula('I'));
    assert( ! estne_minuscula(I_LONGA_MAJUSCULA));
    assert( ! estne_minuscula('O'));
    assert( ! estne_minuscula(O_LONGA_MAJUSCULA));
    assert( ! estne_minuscula('U'));
    assert( ! estne_minuscula(U_LONGA_MAJUSCULA));
    assert( ! estne_minuscula('Y'));
    assert( ! estne_minuscula(Y_LONGA_MAJUSCULA));
}


int
main(int argc, char *argv[])
{
    proba_estne_majuscula();
    proba_estne_minuscula();
    return EXIT_SUCCESS;
}
