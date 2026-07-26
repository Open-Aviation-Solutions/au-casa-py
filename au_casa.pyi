"""Type stubs for the au_casa native extension.

Hand-written, not generated — same approach as icao-shared-kernel-py. Keep in
sync with src/lib.rs and the per-module src/*.rs by hand.
"""

class ClassificationError(ValueError):
    """Base class for every au_casa classification error."""

class UnknownDesignatorError(ClassificationError): ...
class FeatureNotValidForCategoryError(ClassificationError): ...

class AircraftCategory:
    """A category of aircraft for Part 61 purposes (reg 61.015).

    RegisteredSailplane is not a 61.015 category — it reaches Part 61 through
    reg 61.007(2) instead.
    """

    Aeroplane: AircraftCategory
    Helicopter: AircraftCategory
    PoweredLift: AircraftCategory
    Gyroplane: AircraftCategory
    Airship: AircraftCategory
    RegisteredSailplane: AircraftCategory

    def design_features(self) -> list[DesignFeature]:
        """The design features requiring an endorsement for this category
        (reg 61.755)."""

    def allows(self, feature: DesignFeature) -> bool: ...

class AircraftClassRating:
    """A class of aircraft for Part 61 purposes (reg 61.020)."""

    SingleEngineAeroplane: AircraftClassRating
    MultiEngineAeroplane: AircraftClassRating
    SingleEngineHelicopter: AircraftClassRating
    PoweredLiftAircraft: AircraftClassRating
    SingleEngineGyroplane: AircraftClassRating
    Airship: AircraftClassRating

class DesignFeature:
    """A design feature requiring an endorsement under reg 61.755."""

    TailwheelUndercarriage: DesignFeature
    RetractableUndercarriage: DesignFeature
    ManualPropellerPitchControl: DesignFeature
    GasTurbineEngine: DesignFeature
    MultiEngineCentreLineThrust: DesignFeature
    PressurisationSystem: DesignFeature
    Floatplane: DesignFeature
    FloatingHull: DesignFeature
    SkiLandingGear: DesignFeature
    FloatAlightingGear: DesignFeature

    def applicable_categories(self) -> list[AircraftCategory]: ...

class Confidence:
    """How far to trust a classification."""

    Confirmed: Confidence
    Provisional: Confidence
    Overridden: Confidence

class ClassificationOverride:
    def __init__(
        self,
        category: AircraftCategory | None = None,
        class_rating: AircraftClassRating | None = None,
        design_features: list[DesignFeature] | None = None,
    ) -> None: ...
    @property
    def category(self) -> AircraftCategory | None: ...
    @property
    def class_rating(self) -> AircraftClassRating | None: ...
    @property
    def design_features(self) -> list[DesignFeature] | None: ...

class CasaAircraftClassification:
    @property
    def category(self) -> AircraftCategory: ...
    @property
    def class_rating(self) -> AircraftClassRating | None: ...
    @property
    def design_features(self) -> list[DesignFeature]: ...
    @property
    def confidence(self) -> Confidence: ...
    @property
    def source(self) -> str | None: ...

def resolve_classification(
    designator: str,
    classification_override: ClassificationOverride | None = None,
) -> CasaAircraftClassification:
    """Resolve the effective Part 61 classification of an aircraft type.

    Raises UnknownDesignatorError for an uncatalogued designator with no
    category override, and FeatureNotValidForCategoryError if the override
    names a design feature that does not apply to the resolved category.
    """

class RecognisedForeignState:
    """A State whose device qualifications CASA recognises (reg 61.010)."""

    Canada: RecognisedForeignState
    HongKong: RecognisedForeignState
    NewZealand: RecognisedForeignState
    UnitedStatesOfAmerica: RecognisedForeignState
    Belgium: RecognisedForeignState
    CzechRepublic: RecognisedForeignState
    Denmark: RecognisedForeignState
    Finland: RecognisedForeignState
    France: RecognisedForeignState
    Germany: RecognisedForeignState
    Ireland: RecognisedForeignState
    Italy: RecognisedForeignState
    Netherlands: RecognisedForeignState
    Norway: RecognisedForeignState
    Portugal: RecognisedForeignState
    Spain: RecognisedForeignState
    Sweden: RecognisedForeignState
    Switzerland: RecognisedForeignState
    UnitedKingdom: RecognisedForeignState

class FstdRecognition:
    """The basis on which a device is an FSTD for Part 61 purposes — the five
    sub-types of the reg 61.010 definition.

    There is deliberately no value for an unrecognised device: that is the
    absence of recognition, i.e. None.
    """

    @staticmethod
    def qualified_flight_simulator() -> FstdRecognition:
        """Reg 61.010(a)."""

    @staticmethod
    def qualified_flight_training_device() -> FstdRecognition:
        """Reg 61.010(b)."""

    @staticmethod
    def synthetic_trainer_cao_45() -> FstdRecognition:
        """Reg 61.010(c) — historical only; CAO 45.0 is no longer in force."""

    @staticmethod
    def prescribed_under_reg_61_045() -> FstdRecognition:
        """Reg 61.010(d) — the live catch-all."""

    @staticmethod
    def foreign_state_qualified(
        state: RecognisedForeignState,
    ) -> FstdRecognition:
        """Reg 61.010(e)."""

    @property
    def foreign_state(self) -> RecognisedForeignState | None: ...

def counts_for_part61(recognition: FstdRecognition | None = None) -> bool:
    """Whether time on a device with this recognition counts for Part 61.

    Takes no date: the consumer records the recognition that applied at the
    time of the session, so the temporal question is already answered.
    """
