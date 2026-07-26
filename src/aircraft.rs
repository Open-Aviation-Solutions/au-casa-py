//! Python bindings for CASA Part 61 aircraft classification.

use au_casa::{
    resolve_classification as domain_resolve, AircraftCategory as DomainCategory,
    AircraftClassRating as DomainClass, CasaAircraftClassification as DomainClassification,
    ClassificationOverride as DomainOverride, Confidence as DomainConfidence,
    DesignFeature as DomainFeature,
};
use pyo3::prelude::*;

use crate::error::to_py_err;

macro_rules! plain_enum {
    ($name:ident, $domain:ty, $doc:literal, $( $variant:ident ),+ $(,)?) => {
        #[doc = $doc]
        #[pyclass(eq, hash, frozen, from_py_object, module = "au_casa")]
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $variant, )+
        }

        impl From<$name> for $domain {
            fn from(value: $name) -> Self {
                match value {
                    $( $name::$variant => Self::$variant, )+
                }
            }
        }

        impl From<$domain> for $name {
            fn from(value: $domain) -> Self {
                match value {
                    $( <$domain>::$variant => Self::$variant, )+
                }
            }
        }
    };
}

plain_enum!(
    AircraftCategory,
    DomainCategory,
    "A category of aircraft for Part 61 purposes (reg 61.015). \
     `RegisteredSailplane` is not a 61.015 category — it reaches Part 61 \
     through reg 61.007(2) instead.",
    Aeroplane,
    Helicopter,
    PoweredLift,
    Gyroplane,
    Airship,
    RegisteredSailplane,
);

plain_enum!(
    AircraftClassRating,
    DomainClass,
    "A class of aircraft for Part 61 purposes (reg 61.020).",
    SingleEngineAeroplane,
    MultiEngineAeroplane,
    SingleEngineHelicopter,
    PoweredLiftAircraft,
    SingleEngineGyroplane,
    Airship,
);

plain_enum!(
    DesignFeature,
    DomainFeature,
    "A design feature requiring an endorsement under reg 61.755.",
    TailwheelUndercarriage,
    RetractableUndercarriage,
    ManualPropellerPitchControl,
    GasTurbineEngine,
    MultiEngineCentreLineThrust,
    PressurisationSystem,
    Floatplane,
    FloatingHull,
    SkiLandingGear,
    FloatAlightingGear,
);

plain_enum!(
    Confidence,
    DomainConfidence,
    "How far to trust a classification — determined by the compiled \
     designator row it came from, or `Overridden` when it came wholly from a \
     consumer override.",
    Confirmed,
    Provisional,
    Overridden,
);

#[pymethods]
impl AircraftCategory {
    /// The design features requiring an endorsement for this category
    /// (reg 61.755).
    fn design_features(&self) -> Vec<DesignFeature> {
        DomainCategory::from(*self)
            .design_features()
            .iter()
            .map(|f| DesignFeature::from(*f))
            .collect()
    }

    /// Whether the given feature requires an endorsement for this category.
    fn allows(&self, feature: DesignFeature) -> bool {
        DomainCategory::from(*self).allows(feature.into())
    }
}

#[pymethods]
impl DesignFeature {
    /// The categories for which this feature requires an endorsement.
    fn applicable_categories(&self) -> Vec<AircraftCategory> {
        DomainFeature::from(*self)
            .applicable_categories()
            .iter()
            .map(|c| AircraftCategory::from(*c))
            .collect()
    }
}

/// A consumer-supplied correction to the derived classification.
///
/// Needed routinely, not exceptionally: two airframes of the same type
/// designator can differ on undercarriage, floats or propeller controls.
/// Storage of an override belongs to the consuming application.
#[pyclass(eq, skip_from_py_object, module = "au_casa")]
#[derive(Clone, PartialEq, Default)]
pub struct ClassificationOverride(pub(crate) DomainOverride);

#[pymethods]
impl ClassificationOverride {
    #[new]
    #[pyo3(signature = (category=None, class_rating=None, design_features=None))]
    fn new(
        category: Option<AircraftCategory>,
        class_rating: Option<AircraftClassRating>,
        design_features: Option<Vec<DesignFeature>>,
    ) -> Self {
        Self(DomainOverride {
            category: category.map(Into::into),
            class: class_rating.map(Into::into),
            design_features: design_features
                .map(|features| features.into_iter().map(Into::into).collect()),
        })
    }

    #[getter]
    fn category(&self) -> Option<AircraftCategory> {
        self.0.category.map(Into::into)
    }

    #[getter]
    fn class_rating(&self) -> Option<AircraftClassRating> {
        self.0.class.map(Into::into)
    }

    #[getter]
    fn design_features(&self) -> Option<Vec<DesignFeature>> {
        self.0
            .design_features
            .as_ref()
            .map(|features| features.iter().map(|f| DesignFeature::from(*f)).collect())
    }
}

/// The Part 61 classification of an aircraft type — a value object with no
/// identity and nothing persisted.
#[pyclass(eq, skip_from_py_object, module = "au_casa")]
#[derive(Clone, PartialEq)]
pub struct CasaAircraftClassification(pub(crate) DomainClassification);

#[pymethods]
impl CasaAircraftClassification {
    #[getter]
    fn category(&self) -> AircraftCategory {
        self.0.category.into()
    }

    /// `None` where reg 61.020 defines no class — a registered sailplane, or
    /// a multi-engine helicopter or gyroplane (type-rated, not class-rated).
    #[getter]
    fn class_rating(&self) -> Option<AircraftClassRating> {
        self.0.class.map(Into::into)
    }

    /// Always a *default* pending the consumer's own override: undercarriage,
    /// float fitment and propeller controls vary between airframes of the
    /// same type.
    #[getter]
    fn design_features(&self) -> Vec<DesignFeature> {
        self.0
            .design_features
            .iter()
            .map(|f| DesignFeature::from(*f))
            .collect()
    }

    #[getter]
    fn confidence(&self) -> Confidence {
        self.0.confidence.into()
    }

    /// `None` when the classification came wholly from an override.
    #[getter]
    fn source(&self) -> Option<&'static str> {
        self.0.source
    }

    fn __repr__(&self) -> String {
        format!(
            "CasaAircraftClassification({:?}, {:?})",
            self.0.category, self.0.class
        )
    }
}

/// Resolve the effective Part 61 classification of an aircraft type.
///
/// Raises `UnknownDesignatorError` for an uncatalogued designator with no
/// category override — the table never guesses. Raises
/// `FeatureNotValidForCategoryError` if the override names a design feature
/// that does not apply to the resolved category (reg 61.755).
#[pyfunction]
#[pyo3(signature = (designator, classification_override=None))]
pub fn resolve_classification(
    designator: &str,
    classification_override: Option<&ClassificationOverride>,
) -> PyResult<CasaAircraftClassification> {
    domain_resolve(designator, classification_override.map(|o| &o.0))
        .map(CasaAircraftClassification)
        .map_err(to_py_err)
}
