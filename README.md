# au-casa-py

Python bindings (PyO3) for [`au-casa`](https://github.com/Open-Aviation-Solutions/au-casa),
the CASA (Australia) national aviation regulatory crate.

Sibling to [`icao-shared-kernel-py`](https://github.com/Open-Aviation-Solutions/icao-shared-kernel-py):
that package exposes the ICAO-universal domain, this one exposes the national
Part 61 layer built on top of it.

## What it exposes

**Aircraft classification** — CASA category, class rating and design features,
derived from an ICAO Doc 8643 type designator:

```python
import au_casa

result = au_casa.resolve_classification("C172")
result.category        # AircraftCategory.Aeroplane          (reg 61.015)
result.class_rating    # SingleEngineAeroplane               (reg 61.020)
result.design_features # []                                  (reg 61.755)
result.confidence      # Confidence.Provisional
```

Per-airframe facts that vary between two aircraft of the same type — tailwheel
versus tricycle undercarriage, floats versus wheels — are supplied by the
consumer as an override:

```python
override = au_casa.ClassificationOverride(
    design_features=[au_casa.DesignFeature.TailwheelUndercarriage]
)
au_casa.resolve_classification("PA25", override)
```

**FSTD recognition** — whether a simulator is a flight simulation training
device for Part 61 purposes (reg 61.010), and whether time on it counts:

```python
au_casa.counts_for_part61(au_casa.FstdRecognition.qualified_flight_simulator())
# True

au_casa.counts_for_part61(None)   # an unrecognised personal device
# False
```

An unrecognised device is `None`, not a sentinel value — "not approved" is not
one of reg 61.010's sub-types, it is the absence of recognition. Such a
session is still perfectly loggable; it just earns no regulatory credit.

## Scope

Stateless: pure functions and value objects. No repository protocols, no
persistence, no identity-keyed storage — storage of anything with a lifecycle
belongs to the consuming application. See `INSTRUCTIONS.md`.

## Development

```sh
make dev    # build the extension into the local venv
make check  # lint, format check, tests
```
