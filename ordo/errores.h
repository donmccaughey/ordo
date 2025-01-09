#ifndef ORDO_ERRORES_H_INSERTUS_EST
#define ORDO_ERRORES_H_INSERTUS_EST


enum errores {
    /// No error.
    nullus_error = 0,

    /// Invalid ([vitiosus](https://logeion.uchicago.edu/vitiosus)) input.
    data_vitiosa,

    /// Number out of range ([immodicus](https://logeion.uchicago.edu/immodicus)).
    numerus_immodicus,
};


char const *
erroris_filum(enum errores error);


#endif
