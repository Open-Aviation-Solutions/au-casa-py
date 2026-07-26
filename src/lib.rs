//! Python bindings (PyO3) for the `au-casa` CASA regulatory crate.
//!
//! Exposes the national (CASA Part 61) derivations that the ICAO-universal
//! kernel deliberately does not carry: aircraft classification, and
//! recognition of a flight simulation training device.
//!
//! Stateless, like the Rust crate — pure functions and value objects, no
//! repository protocols and no persistence. Storage of anything with a
//! lifecycle (a per-airframe classification override, a recognition snapshot)
//! belongs to the consuming application.

mod aircraft;
mod error;
mod fstd;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn au_casa(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<aircraft::AircraftCategory>()?;
    m.add_class::<aircraft::AircraftClassRating>()?;
    m.add_class::<aircraft::DesignFeature>()?;
    m.add_class::<aircraft::Confidence>()?;
    m.add_class::<aircraft::ClassificationOverride>()?;
    m.add_class::<aircraft::CasaAircraftClassification>()?;
    m.add_class::<fstd::RecognisedForeignState>()?;
    m.add_class::<fstd::FstdRecognition>()?;

    m.add_function(wrap_pyfunction!(aircraft::resolve_classification, m)?)?;
    m.add_function(wrap_pyfunction!(fstd::counts_for_part61, m)?)?;

    m.add(
        "ClassificationError",
        py.get_type::<error::ClassificationError>(),
    )?;
    m.add(
        "UnknownDesignatorError",
        py.get_type::<error::UnknownDesignatorError>(),
    )?;
    m.add(
        "FeatureNotValidForCategoryError",
        py.get_type::<error::FeatureNotValidForCategoryError>(),
    )?;

    Ok(())
}
