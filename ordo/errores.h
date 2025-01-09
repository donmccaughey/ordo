#ifndef ORDO_ERRORES_H_INSERTUS_EST
#define ORDO_ERRORES_H_INSERTUS_EST


/// Error codes for the `ordo` library.
enum errores {
    /// "No error."
    nullus_error = 0,

    /// Invalid input.  "Corrupt data."
    data_vitiosa,

    /// Number out of range.  "Excessive number."
    numerus_immodicus,
};


/// Get the error message for an error.  "The string of an error."
char const *
erroris_filum(enum errores error);


#endif
