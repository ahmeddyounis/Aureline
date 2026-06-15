# Offboarding-safe cleanup, offline / mirror continuity warnings, certified-workspace / policy-bundle pin protection

Evidence record for the offboarding continuity plan that makes account
offboarding, device reset, workspace wipe, and sign-out cleanup honest before
anything is removed across the M5 heavy-artifact storage classes (generated
previews, notebook outputs, docs / model / template packs, extension downloads,
prebuild layers, profiler / replay / support / review evidence, and user-owned
recovery state).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  metadata-safe support-export projection:
  `crates/aureline-support/src/m5_offboarding_continuity/`.
- The boundary schema:
  [`/schemas/storage/m5_offboarding_continuity.schema.json`](../../schemas/storage/m5_offboarding_continuity.schema.json).
- The contract and human-readable summary:
  [`/docs/storage/m5_offboarding_continuity_contract.md`](../../docs/storage/m5_offboarding_continuity_contract.md)
  and
  [`/artifacts/storage/m5_offboarding_continuity.md`](../storage/m5_offboarding_continuity.md).
- A scenario corpus across an account offboarding that retains durable state, a
  device reset that clears only caches, a sign-out cleanup that keeps offline /
  certified / policy pins, an offboarding that reviews offline packs away with
  continuity warnings, and a workspace wipe that exports evidence and recovery
  state before removing them:
  [`/fixtures/storage/m5_offboarding_continuity_cases/`](../../fixtures/storage/m5_offboarding_continuity_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_offboarding_continuity/support_export.golden.json`](../../fixtures/storage/m5_offboarding_continuity/support_export.golden.json).

## Scenarios covered

`account_offboarding_durable_retained`, `device_reset_caches_only`,
`offline_certified_policy_pins_retained`,
`offline_bundle_reviewed_away_continuity_warned`, and
`workspace_wipe_reviewed_away_export_first`.

## Acceptance

- Offboarding / reset / cleanup surfaces distinguish exportable durable state
  (user-owned recovery state, captured evidence) from non-portable derived data
  (caches, packs), name the offline / mirror / certified-workspace implications of
  removal, and protect certified-workspace evidence and policy / offline bundles
  unless explicitly reviewed away.
- The portability headline never implies full data portability when only caches
  were removed.
- The metadata-safe support export exposes the storage-class and pin-state
  summary for these flows to Help / About / diagnostics / support packets.

## Proof

The composer `compose_offboarding_plan` folds the frozen artifact-family matrix at
`/artifacts/storage/m5_artifact_family_storage_matrix.yaml` (which itself projects
the canonical runtime storage-class contract), so the plan's protection posture,
export-before-delete requirement, portability class, continuity warnings, and
disposition come from the same contract the storage-governance matrix and the
clear-data review validate rather than a local synonym set. Pins a family's matrix
row does not admit carry no continuity. The validator, the schema, and the fixture
corpus all enforce the invariants in
`crates/aureline-support/src/m5_offboarding_continuity/tests.rs`, including
negative tests that reject a protected row moved into the disposed bucket, a
disposed protected row with its review dropped, a mutated portability headline, a
hidden continuity note, a tampered byte total, and a mis-derived portability
class. The metadata-safe support export carries the same storage-class and
pin-state truth for Help / About, diagnostics, and support-bundle surfaces, and
matches its checked-in golden.
