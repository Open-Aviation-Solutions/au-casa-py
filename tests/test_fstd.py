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
