"""Part 61 aircraft classification across the Python boundary."""

import pytest

import au_casa as c


def test_catalogued_designator_resolves() -> None:
    result = c.resolve_classification("C172")
    assert result.category == c.AircraftCategory.Aeroplane
    assert result.class_rating == c.AircraftClassRating.SingleEngineAeroplane
    assert result.design_features == []
    assert result.confidence == c.Confidence.Confirmed
    assert result.source is not None


def test_confidence_crosses_the_boundary_for_both_values() -> None:
    # Confidence is per-row upstream, so both variants must be reachable
    # from Python — a binding that only ever saw one value would hide a
    # consumer's ability to tell a checked row from an unchecked one.
    assert c.resolve_classification("CH7B").confidence == c.Confidence.Confirmed
    assert c.resolve_classification("R44").confidence == c.Confidence.Provisional


def test_citabria_resolves_to_single_engine_aeroplane() -> None:
    # pilot-logbook maps its "Citabria" rows to CH7B (both airframes are
    # American Champion 7GCBC), so this is the path that import exercises.
    result = c.resolve_classification("CH7B")
    assert result.category == c.AircraftCategory.Aeroplane
    assert result.class_rating == c.AircraftClassRating.SingleEngineAeroplane


def test_twin_resolves_to_multi_engine() -> None:
    result = c.resolve_classification("PA44")
    assert result.class_rating == c.AircraftClassRating.MultiEngineAeroplane


def test_gas_turbine_feature_is_derived() -> None:
    result = c.resolve_classification("C208")
    assert result.design_features == [c.DesignFeature.GasTurbineEngine]


def test_uncatalogued_designator_raises() -> None:
    with pytest.raises(c.UnknownDesignatorError):
        c.resolve_classification("ZZZZ")


def test_uncatalogued_designator_falls_back_to_override() -> None:
    override = c.ClassificationOverride(
        category=c.AircraftCategory.Aeroplane,
        class_rating=c.AircraftClassRating.SingleEngineAeroplane,
    )
    result = c.resolve_classification("ZZZZ", override)
    assert result.category == c.AircraftCategory.Aeroplane
    assert result.confidence == c.Confidence.Overridden
    assert result.source is None


def test_design_feature_override_accepted() -> None:
    override = c.ClassificationOverride(
        design_features=[c.DesignFeature.TailwheelUndercarriage]
    )
    result = c.resolve_classification("PA25", override)
    assert result.design_features == [c.DesignFeature.TailwheelUndercarriage]


def test_design_feature_override_invalid_for_category_raises() -> None:
    # R44 is a helicopter; floatplane is an aeroplane-only reg 61.755 feature.
    override = c.ClassificationOverride(design_features=[c.DesignFeature.Floatplane])
    with pytest.raises(c.FeatureNotValidForCategoryError):
        c.resolve_classification("R44", override)


def test_errors_share_a_catchable_base() -> None:
    with pytest.raises(c.ClassificationError):
        c.resolve_classification("ZZZZ")


@pytest.mark.parametrize(
    ("category", "count"),
    [
        (c.AircraftCategory.Aeroplane, 9),
        (c.AircraftCategory.Helicopter, 3),
        (c.AircraftCategory.PoweredLift, 3),
        (c.AircraftCategory.Gyroplane, 3),
        (c.AircraftCategory.Airship, 2),
        (c.AircraftCategory.RegisteredSailplane, 0),
    ],
)
def test_design_feature_table_sizes(category: c.AircraftCategory, count: int) -> None:
    assert len(category.design_features()) == count


def test_category_and_feature_tables_agree() -> None:
    assert c.AircraftCategory.Helicopter.allows(c.DesignFeature.FloatAlightingGear)
    assert not c.AircraftCategory.Helicopter.allows(c.DesignFeature.Floatplane)
    assert (
        c.AircraftCategory.Helicopter
        in c.DesignFeature.FloatAlightingGear.applicable_categories()
    )
