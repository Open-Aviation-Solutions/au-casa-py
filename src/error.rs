//! Mapping from the domain's `ClassificationError` to Python exceptions.
//!
//! One Python exception per Rust variant, all subclassing a common base so
//! callers can catch broadly or narrowly — the same shape
//! `icao-shared-kernel-py` uses for `ValidationError`.

use au_casa::ClassificationError as DomainClassificationError;
use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::PyErr;

create_exception!(
    au_casa,
    ClassificationError,
    PyValueError,
    "Base class for every au_casa classification error."
);
create_exception!(
    au_casa,
    UnknownDesignatorError,
    ClassificationError,
    "The designator is not in the compiled table and no category override was supplied."
);
create_exception!(
    au_casa,
    FeatureNotValidForCategoryError,
    ClassificationError,
    "A design feature that does not apply to the resolved category (reg 61.755)."
);

// `PyErr` and `DomainClassificationError` are both foreign types here, so the
// orphan rule rules out a `From` impl; call sites use `.map_err(to_py_err)`.
pub(crate) fn to_py_err(err: DomainClassificationError) -> PyErr {
    let message = err.to_string();
    match err {
        DomainClassificationError::UnknownDesignator(_) => UnknownDesignatorError::new_err(message),
        DomainClassificationError::FeatureNotValidForCategory { .. } => {
            FeatureNotValidForCategoryError::new_err(message)
        }
    }
}
