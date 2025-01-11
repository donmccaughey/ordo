#ifndef ORDO_LINEA_MUTABILIS_H_INSERTUS_EST
#define ORDO_LINEA_MUTABILIS_H_INSERTUS_EST


#define LIN_LON(linea_fixa) (sizeof linea_fixa - 1)


/// A mutable string.  "A string mutable."
struct linea_mutabilis {
    char *linea;
    int mensura;
    int proximus;
};


/// Make a mutable string.  "A string mutable make."
void
linea_mutabilis_fac(struct linea_mutabilis *lmut);

/// Append to a mutable string.  "For a string mutable append."
void
lineae_mutabili_adjunge(
        struct linea_mutabilis *lmut,
        char const *linea, int lineae_longitudo
);


#endif
