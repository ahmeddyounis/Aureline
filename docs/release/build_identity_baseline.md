# Build identity baseline

This page is the reviewer-facing entry point for Aureline's baseline build identity.
The intent is that every runtime, diagnostic export, and release-facing stub can
reference one build identity object instead of minting surface-local version
strings.

## Canonical artifacts

- Machine-readable build identity record: `artifacts/build/build_identity.json`
- Schema: `schemas/build/build_identity.schema.json`
- Exact-build identity model (cross-artifact ref vocabulary): `docs/build/exact_build_identity_model.md`
- Reproducibility packet seed (future release/support workflow): `docs/release/reproducibility_packet_seed.md`

`artifacts/build/build_identity.json` is intentionally checked in so downstream
proof indices, dashboards, and review packets can link to a stable, versioned
example without requiring a local build step.

## How build identity is produced

The builder-side identity record is emitted by:

- `tools/build/print_build_identity.sh`
- `tools/build/build.sh` (writes `target/**/build_identity.json`)

The record is deterministic for a given `(commit, toolchain, target, profile)`
when `SOURCE_DATE_EPOCH` is pinned, per `docs/build/reproducible_build_baseline.md`.

## Validation

- Validate the checked-in record shape: `python3 tools/validate_build_identity_artifact.py`
- Validate that proof indices reference a seeded identity artifact:
  - `python3 ci/check_m1_checkpoint.py --repo-root .`
  - `python3 ci/check_m1_dogfood_matrix.py --repo-root .`

## Consumer rule

Any About/help/provenance surface, support export manifest, or release-facing stub
that needs to answer “what build is this?” should:

1. Read the runtime build identity object (baseline record fields), and
2. Quote exactly one exact-build identity ref (see `docs/build/exact_build_identity_model.md`)
   for cross-artifact joins.

Downstream surfaces must not invent separate “version strings” outside these
objects.
