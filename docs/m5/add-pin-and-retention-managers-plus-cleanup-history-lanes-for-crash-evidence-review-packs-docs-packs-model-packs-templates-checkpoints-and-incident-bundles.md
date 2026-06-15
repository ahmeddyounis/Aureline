# Add pin / retention managers plus cleanup-history lanes

Visible pin / retention management and attributable cleanup history for the heavy
artifact families the M5 depth lanes add. The full contract — invariants, the
matrix-backed composer, and the support / export projection — lives in
[`/docs/storage/m5_pin_retention_contract.md`](../storage/m5_pin_retention_contract.md).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  support-export projection: `crates/aureline-support/src/m5_pin_retention/`.
- The boundary schema:
  [`/schemas/storage/m5_pin_retention.schema.json`](../../schemas/storage/m5_pin_retention.schema.json).
- A scenario corpus exercising evidence and checkpoint pins, offline packs and
  certified templates, pin-blocked cleanup history, and a managed-quota case that
  refuses to delete user-owned recovery state:
  [`/fixtures/storage/m5_pin_retention_cases/`](../../fixtures/storage/m5_pin_retention_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_pin_retention/support_export.golden.json`](../../fixtures/storage/m5_pin_retention/support_export.golden.json).
- The human-readable summary:
  [`/artifacts/storage/m5_pin_retention.md`](../../artifacts/storage/m5_pin_retention.md).

## Why

M5 lands notebook outputs, profiler traces, replay bundles, docs / model /
template packs, certified templates, support artifacts, and review / incident
evidence — and it touches user-owned checkpoints. Those lanes are not complete
until the shell can explain *why* an artifact remains on disk and keep eviction
attributable after the fact. The pin / retention manager answers the first: pin
source, who pinned it, expiry / policy window, referenced object, unpin path, and
export-before-delete path. The cleanup-history lane answers the second: actor,
trigger, class and family, reclaimed bytes, blocked pins, and resulting stale /
reindex-needed state — without ever capturing a raw payload.

## Guarantees

- The **pin actor, unpin path, and export path are derived** from the pin source
  and the frozen artifact-family matrix; no surface invents a private mapping.
- A **finite retention window is the only state that carries an expiry**, and a
  protected entry always requires export before delete.
- **Storage pressure never reclaims user-owned recovery bytes**; the only cleanup
  that may delete recovery state is an explicit, exported-then-deleted user
  action.
- **Pinned and in-window evidence is retained**; only unpinned evidence past
  retention may expire.
- **Blocked pins are always recorded**, never hidden in logs-only diagnostics,
  and no cleanup event ever touches authoritative state or captures a raw
  payload.
- Every manager **binds both the pin-manager and cleanup-history surfaces** and
  offers the open-inspector and class-selective clear-data review actions.

## Proof

Automated proof lives in
`crates/aureline-support/src/m5_pin_retention/tests.rs`:

- the scenario corpus parses and validates with zero violations;
- every pin derives its actor, unpin path, and export path from its source;
- the matrix-backed composer reproduces every seeded manager exactly;
- storage pressure never reclaims user-owned recovery bytes, and a recovery
  delete requires an explicit, exported user action;
- blocked cleanups reclaim zero bytes and record their blocking pins;
- evidence expiry only targets the evidence class;
- negative tests reject a silent recovery delete under pressure and a
  derived-field mismatch;
- the metadata-safe support export matches its checked-in golden.
