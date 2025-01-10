#ifndef ORDO_ERRORES_H_INSERTUS_EST
#define ORDO_ERRORES_H_INSERTUS_EST


/// Error codes for the `ordo` library.
enum error {
    /// "No error."
    error_nullus = 0,

    /// Invalid input.  "Data corrupt."
    error_datis_vitiosis,

    /// Number out of range.  "Number excessive."
    error_numero_immodico,
};


/// Get the error message for an error.  "An error's string."
char const *
erroris_filum(enum error error);


#endif
