//! Python bindings for FSTD recognition under CASA Part 61.

use au_casa::{
    counts_for_part61 as domain_counts, FstdRecognition as DomainRecognition,
    RecognisedForeignState as DomainState,
};
use pyo3::prelude::*;

/// A State whose national aviation authority's device qualifications CASA
/// recognises (reg 61.010, *recognised foreign State*).
///
/// A closed list. Reg 61.047 lets CASA prescribe further countries by
/// legislative instrument; none is known to be in force.
#[pyclass(eq, hash, frozen, from_py_object, module = "au_casa")]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecognisedForeignState {
    Canada,
    HongKong,
    NewZealand,
    UnitedStatesOfAmerica,
    Belgium,
    CzechRepublic,
    Denmark,
    Finland,
    France,
    Germany,
    Ireland,
    Italy,
    Netherlands,
    Norway,
    Portugal,
    Spain,
    Sweden,
    Switzerland,
    UnitedKingdom,
}

macro_rules! state_map {
    ($( $variant:ident ),+ $(,)?) => {
        impl From<RecognisedForeignState> for DomainState {
            fn from(value: RecognisedForeignState) -> Self {
                match value {
                    $( RecognisedForeignState::$variant => Self::$variant, )+
                }
            }
        }
        impl From<DomainState> for RecognisedForeignState {
            fn from(value: DomainState) -> Self {
                match value {
                    $( DomainState::$variant => Self::$variant, )+
                }
            }
        }
    };
}

state_map!(
    Canada,
    HongKong,
    NewZealand,
    UnitedStatesOfAmerica,
    Belgium,
    CzechRepublic,
    Denmark,
    Finland,
    France,
    Germany,
    Ireland,
    Italy,
    Netherlands,
    Norway,
    Portugal,
    Spain,
    Sweden,
    Switzerland,
    UnitedKingdom,
);

/// The basis on which a device is a flight simulation training device for
/// Part 61 purposes — the five sub-types of the reg 61.010 definition.
///
/// There is deliberately no variant for an unrecognised device: "not
/// approved" is not one of the regulation's sub-types, it is the *absence* of
/// recognition, so it is `None`.
#[pyclass(eq, skip_from_py_object, module = "au_casa")]
#[derive(Clone, PartialEq)]
pub struct FstdRecognition(pub(crate) DomainRecognition);

#[pymethods]
impl FstdRecognition {
    /// Reg 61.010(a) — a flight simulator qualified under CASR Part 60.
    #[staticmethod]
    fn qualified_flight_simulator() -> Self {
        Self(DomainRecognition::QualifiedFlightSimulator)
    }

    /// Reg 61.010(b) — a flight training device qualified under CASR Part 60.
    #[staticmethod]
    fn qualified_flight_training_device() -> Self {
        Self(DomainRecognition::QualifiedFlightTrainingDevice)
    }

    /// Reg 61.010(c) — **historical only**. CAO 45.0 is no longer in force;
    /// devices formerly approved under it are now recognised through a reg
    /// 61.045 instrument (CASA AC 60-01 v2.0, July 2026). Kept for records
    /// predating the change.
    #[staticmethod]
    fn synthetic_trainer_cao_45() -> Self {
        Self(DomainRecognition::SyntheticTrainerCao45)
    }

    /// Reg 61.010(d) — a device meeting qualification standards prescribed by
    /// a legislative instrument under reg 61.045. The live catch-all.
    #[staticmethod]
    fn prescribed_under_reg_61_045() -> Self {
        Self(DomainRecognition::PrescribedUnderReg61045)
    }

    /// Reg 61.010(e) — a device qualified (however described) by the national
    /// aviation authority of a recognised foreign State.
    #[staticmethod]
    fn foreign_state_qualified(state: RecognisedForeignState) -> Self {
        Self(DomainRecognition::ForeignStateQualified(state.into()))
    }

    /// The recognised foreign State, for a reg 61.010(e) recognition only.
    #[getter]
    fn foreign_state(&self) -> Option<RecognisedForeignState> {
        match self.0 {
            DomainRecognition::ForeignStateQualified(state) => Some(state.into()),
            _ => None,
        }
    }

    fn __repr__(&self) -> String {
        format!("FstdRecognition({:?})", self.0)
    }
}

/// Whether time logged on a device with this recognition counts toward Part
/// 61 aeronautical experience and currency requirements.
///
/// All five sub-types count — the regulation draws no distinction — so the
/// question is only whether the device was recognised at all. `None` does not
/// count.
///
/// **Takes no date, by design.** Recognition is time-bounded, so the intended
/// usage is that the consumer records the recognition that applied *at the
/// time of the session* — checking
/// `DeviceQualification.is_in_force_on(session_date)` when it does so — and
/// stores that snapshot. By the time this function sees a recognition, the
/// temporal question has already been answered.
#[pyfunction]
#[pyo3(signature = (recognition=None))]
pub fn counts_for_part61(recognition: Option<&FstdRecognition>) -> bool {
    domain_counts(recognition.map(|r| &r.0))
}
