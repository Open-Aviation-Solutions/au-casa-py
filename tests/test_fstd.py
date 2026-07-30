"""FSTD recognition under Part 61, across the Python boundary."""

import pytest

import au_casa as c

ALL_SUB_TYPES = [
    c.FstdRecognition.qualified_flight_simulator(),  # 61.010(a)
    c.FstdRecognition.qualified_flight_training_device(),  # 61.010(b)
    c.FstdRecognition.synthetic_trainer_cao_45(),  # 61.010(c)
    c.FstdRecognition.prescribed_under_reg_61_045(),  # 61.010(d)
    c.FstdRecognition.foreign_state_qualified(  # 61.010(e)
        c.RecognisedForeignState.NewZealand
    ),
]


@pytest.mark.parametrize("recognition", ALL_SUB_TYPES)
def test_every_sub_type_counts(recognition: c.FstdRecognition) -> None:
    assert c.counts_for_part61(recognition) is True


def test_unrecognised_device_does_not_count() -> None:
    # A personal simulator: loggable, but no credit.
    assert c.counts_for_part61(None) is False
    assert c.counts_for_part61() is False


def test_foreign_state_is_readable_back() -> None:
    recognition = c.FstdRecognition.foreign_state_qualified(
        c.RecognisedForeignState.UnitedKingdom
    )
    assert recognition.foreign_state == c.RecognisedForeignState.UnitedKingdom


def test_foreign_state_is_none_for_other_sub_types() -> None:
    assert c.FstdRecognition.qualified_flight_simulator().foreign_state is None


# --- reading a recognition back --------------------------------------------

KIND_FOR = [
    (c.FstdRecognition.qualified_flight_simulator(), c.FstdRecognitionKind.QualifiedFlightSimulator),
    (c.FstdRecognition.qualified_flight_training_device(), c.FstdRecognitionKind.QualifiedFlightTrainingDevice),
    (c.FstdRecognition.synthetic_trainer_cao_45(), c.FstdRecognitionKind.SyntheticTrainerCao45),
    (c.FstdRecognition.prescribed_under_reg_61_045(), c.FstdRecognitionKind.PrescribedUnderReg61045),
    (
        c.FstdRecognition.foreign_state_qualified(c.RecognisedForeignState.NewZealand),
        c.FstdRecognitionKind.ForeignStateQualified,
    ),
]


@pytest.mark.parametrize(("recognition", "kind"), KIND_FOR)
def test_kind_identifies_every_sub_type(
    recognition: c.FstdRecognition, kind: c.FstdRecognitionKind
) -> None:
    assert recognition.kind == kind


def test_the_four_non_foreign_sub_types_are_distinguishable() -> None:
    # The gap this closes: foreign_state was the only readable field, so the
    # other four were indistinguishable and a stored recognition could not be
    # restored.
    kinds = {r.kind for r, _ in KIND_FOR if r.foreign_state is None}
    assert len(kinds) == 4


@pytest.mark.parametrize(("recognition", "_kind"), KIND_FOR)
def test_recognition_round_trips_through_its_parts(
    recognition: c.FstdRecognition, _kind: c.FstdRecognitionKind
) -> None:
    restored = c.FstdRecognition.from_parts(
        recognition.kind, recognition.foreign_state
    )
    assert restored == recognition


def test_from_parts_requires_a_state_for_the_foreign_sub_type() -> None:
    with pytest.raises(ValueError):
        c.FstdRecognition.from_parts(c.FstdRecognitionKind.ForeignStateQualified)


def test_from_parts_rejects_a_state_on_the_other_sub_types() -> None:
    # Silently ignoring it would produce a recognition that does not match
    # what the caller asked for.
    with pytest.raises(ValueError):
        c.FstdRecognition.from_parts(
            c.FstdRecognitionKind.QualifiedFlightSimulator,
            c.RecognisedForeignState.NewZealand,
        )
