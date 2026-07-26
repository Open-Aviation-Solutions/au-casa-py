# Python bindings for the au-casa crate

**Status:** in progress — implemented and passing; the Cargo dependency is
pinned to an unmerged `au-casa` branch (see Before merging).

## Purpose

`au-casa` task `0001`'s Sequencing deferred this crate: *"land the Rust domain
logic first, same order as `icao-shared-kernel-rs` before
`icao-shared-kernel-py`"*. That was the right call when `au-casa` had no code.
Both of its modules now exist, and the deferral has become the blocker:
`pilot-logbook` is Python and cannot reach either of them.

This crate is the missing link. It exposes:

- **Aircraft classification** (`au-casa` task `0001`) — the 13 call sites in
  `pilot-logbook`'s `metrics/` and `services/currency.py` that still import
  `AircraftCategory` / `AircraftClassRating` from the removed `aviation_core`.
- **FSTD recognition** (`au-casa` task `0002`) — the six call sites that still
  use `SimType` / `is_regulatory_sim`.

## Decisions

1. **Separate repo, mirroring `icao-shared-kernel-py`.** Keeps `au-casa` free
   of PyO3/CPython linkage for other consumers, exactly as the kernel split
   does. Layout, `Makefile`, CI workflow and stub approach are all copied from
   that repo deliberately — two bindings crates that behave differently would
   be a maintenance tax for no gain.
2. **No domain logic here.** If a rule needs changing it changes in `au-casa`
   and is re-exposed. There is nothing to test in this crate beyond the
   boundary itself.
3. **Boundary naming.** Two Rust shapes have no direct Python equivalent:
   - `class` is a Python keyword, so
     `CasaAircraftClassification.class` is exposed as `class_rating`.
   - `FstdRecognition::ForeignStateQualified(state)` carries data, which a
     PyO3 plain enum cannot. It becomes five static constructors
     (`FstdRecognition.qualified_flight_simulator()` etc.) plus a
     `foreign_state` getter that is `None` for the other four.
4. **Exception mapping follows the kernel's shape**: one Python exception per
   Rust error variant, all subclassing a common `ClassificationError` base so
   callers can catch broadly or narrowly.
5. **`from_py_object` opt-in.** Types passed *into* functions by value (the
   plain enums) need it; types only passed by reference or returned take
   `skip_from_py_object`. PyO3 0.29 warns if neither is chosen and the derive
   goes away in a later release.

## Before merging

`Cargo.toml` pins `au-casa` to `branch = "add-aircraft-classification"`, which
is unmerged (`au-casa` PR #2, stacked on PR #1). Once both land, **repoint to
`branch = "main"`** and re-run `make check`. That pin is the only thing
blocking this.

Local verification uses a temporary `path` dependency — see `INSTRUCTIONS.md`.

## Acceptance criteria

- [x] `aircraft` module: the four enums, `ClassificationOverride`,
      `CasaAircraftClassification`, `resolve_classification`, plus the
      `design_features()` / `applicable_categories()` table accessors.
- [x] `fstd` module: `RecognisedForeignState`, `FstdRecognition`,
      `counts_for_part61`.
- [x] `error` module: `ClassificationError` base with two subclasses.
- [x] Hand-written `au_casa.pyi` stubs.
- [x] Tests across the boundary, including that the unrecognised-device case
      is `None` rather than a sentinel, and that an invalid design-feature
      override raises.
- [x] `make check` passes (clippy `-D warnings`, fmt, 23 pytest tests).
- [ ] Dependency repointed to `main` after `au-casa` PRs #1 and #2 merge.
- [ ] CI verified green — the workflow has never run, since the repo is new
      and its dependency branch is unmerged.

## Related

- `au-casa` tasks `0001` (aircraft classification) and `0002` (FSTD
  recognition) — the domain logic exposed here.
- `icao-shared-kernel-py` task `0002` — the parallel FSTD bindings on the
  kernel side.
- `pilot-logbook` tasks `0007` and `0008` — the consumers waiting on both.
