<!-- SPDX-License-Identifier: Apache-2.0 -->
# Clean-room rebuild lane

This document defines the first clean-room rebuild lane for the Aureline
repository. It is intentionally narrower than a release-grade
reproducibility program: the lane proves that the seeded workspace can be
rebuilt from a clean checkout with explicit inputs, explicit trust
assumptions, and explicit provenance-capture outputs. It does not yet claim
release signing, notarization, or byte-identical binaries across every host.

Companion artifacts:

- [`/ci/cleanroom_rebuild.sh`](../../ci/cleanroom_rebuild.sh) — documented
  command entry point.
- [`/.github/workflows/cleanroom_rebuild.yml`](../../.github/workflows/cleanroom_rebuild.yml)
  — GitHub Actions wrapper that runs the lane on an ephemeral runner.
- [`/docs/build/reproducible_build_baseline.md`](./reproducible_build_baseline.md)
  — pinned build inputs and the baseline build-identity contract this lane
  composes with.
- [`/docs/build/exact_build_identity_model.md`](./exact_build_identity_model.md)
  — exact-build identity model the lane links into, even though full
  `exact_build_identity_record` emission is still deferred.
- [`/fixtures/build/cleanroom_input_manifest.json`](../../fixtures/build/cleanroom_input_manifest.json)
  — checked-in seed of the emitted input-manifest shape.
- [`/artifacts/release/provenance_capture_seed.json`](../../artifacts/release/provenance_capture_seed.json)
  — checked-in seed of the emitted provenance-capture shape.
- [`/docs/release/build_farm_and_remote_cache_policy.md`](../release/build_farm_and_remote_cache_policy.md),
  [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml),
  and
  [`/artifacts/release/cache_trust_classes.yaml`](../../artifacts/release/cache_trust_classes.yaml)
  — build-farm trust-domain rules and remote-cache non-dependence
  policy. This lane is the protected reproducibility floor those rules
  fall back to whenever release-bearing cache fast paths are bypassed
  or invalidated.

## Command

Primary command:

```sh
./ci/cleanroom_rebuild.sh --out-dir target/cleanroom-rebuild
```

Offline variant:

```sh
./ci/cleanroom_rebuild.sh --offline --out-dir target/cleanroom-rebuild
```

The command rejects dirty checkouts. That is deliberate: a clean-room lane
must be explainable from committed files and the documented invocation
alone.

## CI wrapper

The repository-level CI wrapper lives at
[`/.github/workflows/cleanroom_rebuild.yml`](../../.github/workflows/cleanroom_rebuild.yml).
It adapts the suggested single-file CI output to the repository's existing
GitHub Actions layout and makes two stricter choices than a normal local
invocation:

1. It runs on an ephemeral `ubuntu-latest` runner.
2. It points `CARGO_HOME` and `RUSTUP_HOME` at runner-temp directories so
   the lane does not inherit pre-baked toolchain state from the default
   runner account.

## Outputs

The lane writes one self-contained artifact directory. The expected files
are:

| File | Purpose |
|---|---|
| `build_identity.json` | Baseline deterministic build identity emitted by the pinned build command. |
| `sbom_workspace.json` | Placeholder workspace SBOM stub emitted by the existing provenance lane. |
| `provenance_summary.json` | Placeholder provenance summary anchored on the build identity. |
| `cleanroom_input_manifest.json` | Machine-readable list of source refs, pinned files, mirror settings, commands, and trust assumptions. |
| `artifact_digests.json` | Digest manifest for the top-level release-profile outputs built in this lane. |
| `provenance_capture.json` | Capture summary linking producer lane, exact-build linkage, artifact-family refs, publishability classes, and known limitations. |

The current workspace is still pre-release. As a result, the top-level
binary rows in `artifact_digests.json` and `provenance_capture.json` are
truthfully marked `development_prototype_non_publishable` rather than being
presented as releasable artifacts.

## Required inputs and trust assumptions

The lane is defined by committed inputs plus declared environment inputs.
The emitted `cleanroom_input_manifest.json` records them explicitly.

Pinned repository inputs:

- `rust-toolchain.toml`
- `.cargo/config.toml`
- `Cargo.toml`
- `Cargo.lock`
- the checked-out Git commit and clean tree state

Declared environment inputs:

- `SOURCE_DATE_EPOCH` (defaults to the commit timestamp if the caller does
  not provide one)
- `RUSTUP_DIST_SERVER` and `RUSTUP_UPDATE_ROOT`
- Cargo registry protocol and local Cargo/Rustup home locations

Declared trust assumptions:

- mirrors serving Rust toolchains and Cargo content resolve to the pinned
  bytes for this commit;
- release signing and final attestation material are intentionally absent;
- meaningful sameness currently means matching build-identity axes plus the
  emitted artifact digest manifest, not universal byte-identical binaries;
- dirty checkouts, hidden local cache edits, and unpublished signing
  shortcuts are outside the supported lane.

## Exact-build linkage

This lane already exercises exact-build concepts in code, but it stops one
step short of emitting a full `exact_build_identity_record`:

- `build_identity.json` is the deterministic builder-side anchor.
- `provenance_capture.json` records the exact-build linkage state as
  `baseline_build_identity_only`.
- each built artifact row carries an `artifact_graph_seed_ref`,
  a publishability class, and, where applicable, an
  `exact_build_artifact_family_class`.

For the current codebase, the primary `shell_spike` executable is the only
row that maps directly onto the exact-build `ide_binary` family. All other
workspace binaries remain prototype/support outputs and are labeled as such.

## Comparison rules

Two clean-room runs are meaningfully the same today when all of the
conditions below hold:

1. `build_identity.json` matches on `commit`, `toolchain_channel`,
   `target_triple`, `profile`, and `source_date_epoch`.
2. `cleanroom_input_manifest.json` matches on the digests of the four pinned
   build-input files and records equivalent mirror settings.
3. `artifact_digests.json` matches for the rows being compared.
4. any difference that remains is confined to producer-lane context or other
   fields explicitly documented as provisional.

A mismatch is not hidden or hand-waved:

- if the input-manifest digests differ, the runs are different builds;
- if the build identity differs, the runs are different builds;
- if the build identity matches but artifact digests differ, the lane has
  surfaced a reproducibility gap that must be treated as open;
- if the tree is dirty, the lane is invalid and should be rerun from a clean
  checkout.

## Current gaps

These limitations are intentionally named here and repeated in the emitted
`provenance_capture.json`. They are not allowed to hide in shell comments or
ad hoc CI knowledge.

| Row id | Category | Current posture | Closure condition |
|---|---|---|---|
| `lim.signing_dependencies_absent` | signing/secrets | Release signing, notarization, and final attestation keys are absent from this lane. | Replace the placeholder provenance summary with the real signing/attestation pipeline. |
| `lim.mirror_assumptions_declared_not_verified` | mirrors/offline | The lane records which Rustup/Cargo endpoints it used, but it does not yet prove the mirror served identical bytes beyond the pinned inputs. | Add mirror-equivalence verification and mirror-pack attestations. |
| `lim.binary_byte_identity_not_claimed` | nondeterministic inputs | The repository guarantees deterministic build-identity records today, not universal byte-identical binaries. | Land the later reproducibility work that makes binary byte identity a release claim. |
| `lim.developer_shortcuts_unsupported` | local shortcuts | Dirty trees, hidden cache tweaks, and unpublished signing shortcuts are outside the lane contract. | Keep failing closed; only documented, replayable inputs may widen the lane. |

## Publishability classes

The lane uses a small explicit publishability vocabulary in
`artifact_digests.json` and `provenance_capture.json`:

- `development_prototype_non_publishable` — built workspace binaries that
  are useful for development/rebuild comparison but are not release-ready.
- `non_publishable_placeholder` — structural SBOM/provenance outputs that
  reserve their future home without claiming release-grade conformance.
- `internal_control_artifact` — lane metadata such as manifests and capture
  summaries.

Anything not explicitly marked publishable is non-publishable by default.
