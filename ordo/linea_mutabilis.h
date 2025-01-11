#ifndef ORDO_LINEA_MUTABILIS_H_INSERTUS_EST
#define ORDO_LINEA_MUTABILIS_H_INSERTUS_EST


#define LIN_LON(linea_fixa) (sizeof linea_fixa - 1)


struct linea_mutabilis {
    char *linea;
    int mensura;
    int proximus;
};


void
linea_mutabilis_fac(struct linea_mutabilis *lmut);

void
linea_mutabilis_adjunge(
        struct linea_mutabilis *lmut,
        char const *linea, int lineae_longitudo
);


#endif
