# Ship clear-data review sheets with class-selective cleanup

Class-selective clear-data review for the heavy artifact families the M5 depth
lanes add. The full contract — invariants, the matrix-backed composer, and the
support/export projection — lives in
[`/docs/storage/m5_clear_data_review_contract.md`](../storage/m5_clear_data_review_contract.md).

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  support-export projection: `crates/aureline-support/src/m5_clear_data_review/`.
- The boundary schema:
  [`/schemas/storage/m5_clear_data_review.schema.json`](../../schemas/storage/m5_clear_data_review.schema.json).
- A scenario corpus exercising user-driven cleanup, admin-driven cleanup,
  offboarding/reset, low-disk pressure, and a blocked managed-quota case:
  [`/fixtures/storage/m5_clear_data_review_cases/`](../../fixtures/storage/m5_clear_data_review_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_clear_data_review/support_export.golden.json`](../../fixtures/storage/m5_clear_data_review/support_export.golden.json).
- The human-readable summary:
  [`/artifacts/storage/m5_clear_data_review.md`](../../artifacts/storage/m5_clear_data_review.md).

## Why

M5 lands notebook outputs, profiler traces, replay bundles, docs/model/template
packs, generated previews, prebuild layers, support artifacts, and
review/incident evidence — and it touches user-owned recovery state. A generic
clear-cache or reset button cannot be allowed to erase authoritative recovery or
referenced evidence state by accident. The review sheet replaces that button
with a class-selective review that names what is rebuilt, what may be lost, what
is pinned or protected, and which export/checkpoint path exists first.

## Guarantees

- Protected evidence and user-owned recovery state are **excluded unless
  explicitly selected**, and never offered a generic clear.
- Every selected class names its **rebuild cost, offline impact, and
  reversibility**; irreversible removals spell out the consequence.
- Protected classes always carry an **export-before-delete** path first.
- **Low-disk ordering is disclosed in full** on pressure-triggered sheets and is
  never hidden in logs-only diagnostics.
- **Managed quota or disk pressure never silently deletes local user-owned
  state**; a sheet with only protected classes left is blocked with a guardrail
  notice instead.
- **Offboarding/reset accounts for every protected family** as selected (with
  export) or retained.

## Proof

Automated proof lives in
`crates/aureline-support/src/m5_clear_data_review/tests.rs`:

- the scenario corpus parses and validates with zero violations;
- protected rows never admit a generic clear and always require
  export-before-delete;
- pressure sheets disclose the full eviction order and never auto-select
  user-owned recovery state;
- offboarding surfaces every protected family;
- the matrix-backed composer excludes protected families unless explicit,
  refuses user-owned state under pressure, and stays within the matrix's allowed
  clear-data actions;
- failure drills reject a protected row mutated to a generic clear, a pressure
  sheet mutated to select user-owned state, an offboarding sheet that drops a
  protected family, a hidden rebuild disclosure, and a tampered reclaim total;
- the metadata-safe support export matches its checked-in golden.
