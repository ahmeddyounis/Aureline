# Ship clear-data review sheets with class-selective cleanup

Evidence record for the class-selective clear-data review sheet that governs how
the heavy artifacts the M5 depth lanes add are cleared, retained, and protected.

## What shipped

- The canonical product object plus its validator, matrix-backed composer, and
  metadata-safe support-export projection:
  `crates/aureline-support/src/m5_clear_data_review/`.
- The boundary schema:
  [`/schemas/storage/m5_clear_data_review.schema.json`](../../schemas/storage/m5_clear_data_review.schema.json).
- The contract and human-readable summary:
  [`/docs/storage/m5_clear_data_review_contract.md`](../../docs/storage/m5_clear_data_review_contract.md)
  and [`/artifacts/storage/m5_clear_data_review.md`](../storage/m5_clear_data_review.md).
- A scenario corpus across all three cleanup flows and all three triggers:
  [`/fixtures/storage/m5_clear_data_review_cases/`](../../fixtures/storage/m5_clear_data_review_cases/).
- The golden support-export projection and replay gate:
  [`/fixtures/storage/m5_clear_data_review/support_export.golden.json`](../../fixtures/storage/m5_clear_data_review/support_export.golden.json).

## Scenarios covered

`user_cleanup_rebuildable_caches`, `admin_cleanup_artifact_packs_pin_excluded`,
`offboarding_reset_full_export_first`, `low_disk_pressure_disposable_first`, and
`blocked_quota_pressure_refuses_user_owned`.

## Acceptance

- Sheets enumerate the selected classes, affected workspaces, rebuild cost,
  retained / protected classes, export-before-delete options, and irreversible
  consequences.
- Local history, rollback checkpoints, support evidence, policy bundles, offline
  entitlement bundles, and pinned review artifacts are excluded unless
  explicitly selected.

## Proof

The sheet folds the frozen artifact-family matrix at
`/artifacts/storage/m5_artifact_family_storage_matrix.yaml`; each row's storage
class, authority, rebuild cost, clear protection, low-disk step, and clear-data
action re-export from that matrix. Automated proof lives in
`crates/aureline-support/src/m5_clear_data_review/tests.rs`:

- the scenario corpus parses and validates with zero violations;
- protected evidence and user-owned recovery state never admit a generic clear
  and always require export-before-delete;
- pressure sheets disclose the full low-disk eviction order and never
  auto-select user-owned recovery state;
- offboarding/reset surfaces every protected family;
- the matrix-backed composer excludes protected families unless explicitly
  selected, refuses user-owned state under disk/quota pressure, and stays within
  the matrix's allowed clear-data actions;
- failure drills reject a protected row mutated to a generic clear, a pressure
  sheet mutated to select user-owned state, an offboarding sheet that drops a
  protected family, a hidden rebuild disclosure, and a tampered reclaim total;
- the metadata-safe support export matches its checked-in golden.

## Guardrails honored

- No generic clear-cache button can erase authoritative recovery or referenced
  evidence state without a class-selective review.
- Low-disk ordering and pin reasons are surfaced on the sheet, not buried in
  logs-only diagnostics.
- Managed quota or storage pressure is never satisfied by silently deleting
  local user-owned state.
- Rebuild cost and offline/mirror consequences are never hidden from the sheet.
