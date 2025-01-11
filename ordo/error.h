#ifndef ORDO_ERRORES_H_INSERTUS_EST
#define ORDO_ERRORES_H_INSERTUS_EST


/// Error codes for the `ordo` library.
enum error {
    /// "No error."
    error_nullus = 0,

    /// Invalid input.  "Error due to data corrupt."
    error_datis_vitiosis,

    /// Number out of range.  "Error due to number excessive."
    error_numero_immodico,
};


/// Get the error message for an error.  "An error's message."
char const *
erroris_nuntium(enum error error);


#endif
