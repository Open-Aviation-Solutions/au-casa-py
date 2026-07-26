# au-casa-py — Working Instructions

Python bindings (PyO3) for the [`au-casa`](https://github.com/Open-Aviation-Solutions/au-casa)
CASA regulatory crate. Sibling to
[`icao-shared-kernel-py`](https://github.com/Open-Aviation-Solutions/icao-shared-kernel-py),
which does the same job for the ICAO-universal kernel, and follows its layout
and conventions closely — read that repo's `INSTRUCTIONS.md` too.

## Purpose and scope

`au-casa` resolves national (CASA Part 61) facts that the ICAO-universal
kernel deliberately does not carry. This crate exposes those to Python for
consumers such as `pilot-logbook`.

In scope: thin wrappers over the Rust API, the exception mapping, and the
hand-written `au_casa.pyi` stubs.

Out of scope (do not add here): any domain logic. If a rule needs changing,
change it in `au-casa` and re-expose it. This crate must stay a binding layer
with nothing to test beyond the boundary itself.

Also out of scope, inherited from `au-casa`: repository protocols,
persistence, and identity-keyed storage. Storage of anything with a lifecycle
— a per-airframe classification override, a recognition snapshot — belongs to
the consuming application.

## Conventions

- **One module per concern**, mirroring the Rust crate's modules
  (`src/aircraft.rs`, `src/fstd.rs`), plus `src/error.rs` for the exception
  mapping. `lib.rs` registers everything so the Python API stays flat.
- **One Python exception per Rust error variant**, all subclassing a common
  base (`ClassificationError`) so callers can catch broadly or narrowly.
- **Stubs are hand-written**, not generated. Keep `au_casa.pyi` in sync with
  `src/lib.rs` by hand when adding or changing anything.
- **`#[pyclass]` `from_py_object` opt-in**: types passed *into* functions by
  value (the plain enums) need `from_py_object`; types only passed by
  reference or returned (`ClassificationOverride`, `FstdRecognition`,
  `CasaAircraftClassification`) take `skip_from_py_object`. PyO3 0.29 warns if
  neither is chosen.
- **Naming at the boundary**: `class` is a Python keyword, so
  `CasaAircraftClassification.class` is exposed as `class_rating`. Data-carrying
  Rust enum variants (`FstdRecognition::ForeignStateQualified`) become static
  constructors plus a getter, since PyO3 plain enums cannot carry data.

## Verifying in a sandboxed agent environment

`Cargo.toml` depends on `au-casa` over git. Nothing is wrong with that
dependency — but this developer's global gitconfig rewrites
`https://github.com/` to `git@github.com:` (`url.insteadOf`), so cargo's fetch
becomes an SSH one, and an agent sandbox without the SSH key cannot
authenticate.

Work around it by bypassing the global rewrite and supplying a token, rather
than by switching to a path dependency (which changes what is actually being
verified):

```sh
GIT_CONFIG_GLOBAL=/dev/null \
GIT_CONFIG_COUNT=1 \
GIT_CONFIG_KEY_0="url.https://x-access-token:$(gh auth token)@github.com/.insteadOf" \
GIT_CONFIG_VALUE_0="https://github.com/" \
CARGO_NET_GIT_FETCH_WITH_CLI=true make check
```

Ordinary developer machines with working SSH need none of this.

## Commands

```sh
make dev      # build the extension into the local venv
make test     # pytest against the built extension
make lint     # cargo clippy -- -D warnings
make fmt      # cargo fmt
make check    # lint + fmt check + test
```
