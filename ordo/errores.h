#ifndef ORDO_ERRORES_H_INSERTUS_EST
#define ORDO_ERRORES_H_INSERTUS_EST


/// Error codes for the `ordo` library.
enum errores {
    /// "No error."
    nullus_error = 0,

    /// Invalid input.  "Data corrupt."
    data_vitiosa,

    /// Number out of range.  "Number excessive."
    numerus_immodicus,
};


/// Get the error message for an error.  "An error's string."
char const *
erroris_filum(enum errores error);


#endif
