# M5 clear-data review sheet (human-readable)

Companion to the boundary schema at
[`/schemas/storage/m5_clear_data_review.schema.json`](../../schemas/storage/m5_clear_data_review.schema.json),
the contract at
[`/docs/storage/m5_clear_data_review_contract.md`](../../docs/storage/m5_clear_data_review_contract.md),
and the scenario corpus under
[`/fixtures/storage/m5_clear_data_review_cases/`](../../fixtures/storage/m5_clear_data_review_cases/).

A clear-data review sheet is the operator-facing object shown **before** any
cleanup commits. Every storage-class, authority, rebuild-cost, clear-protection,
low-disk-ladder, pin-source, and clear-data-action value re-exports verbatim
from [`m5_artifact_family_storage_matrix.yaml`](./m5_artifact_family_storage_matrix.yaml);
the cleanup-flow, trigger, selection-state, retention-reason,
export-before-delete, reversibility, and consent columns are the only sets this
sheet introduces.

## Per-family posture in a sheet

| Family | Storage class | Clear-data action | Export before delete | Reversibility |
| --- | --- | --- | --- | --- |
| Generated previews | interactive_hot_cache | generic_clear_in_bulk | not applicable (disposable) | reversible (rebuildable) |
| Notebook outputs | artifact_cache | class_selective_clear | not applicable (disposable) | reversible (rebuildable) |
| Docs / model / template packs | artifact_cache | class_selective_clear | offered (optional) | reversible from pinned / offline source |
| Extension downloads | artifact_cache | class_selective_clear | offered (optional) | reversible from pinned / offline source |
| Prebuild layers | prebuild_environment_cache | class_selective_clear | offered (optional) | reversible from pinned / offline source |
| Profiler traces | evidence_support_cache | class_specific_review_required | **required** | **irreversible (evidence loss)** |
| Replay bundles | evidence_support_cache | class_specific_review_required | **required** | **irreversible (evidence loss)** |
| Support artifacts | evidence_support_cache | class_specific_review_required | **required** | **irreversible (evidence loss)** |
| Review / incident evidence | evidence_support_cache | class_specific_review_required | **required** | **irreversible (evidence loss)** |
| User-owned recovery state | user_owned_recovery_state | explicit_per_item_review_required | **required** | **irreversible (authoritative loss)** |

Evidence families and user-owned recovery state are **excluded by default** and
appear in a sheet only as retained / export-before-delete rows unless the
operator explicitly selects them.

## Flows and triggers covered

| Flow | Trigger | Sheet behavior |
| --- | --- | --- |
| User-driven cleanup | manual_user_request | Clears selected rebuildable caches; pinned bytes and protected classes preserved. |
| Admin-driven cleanup | admin_policy_cleanup | Reclaims unpinned packs across managed workspaces; pinned downloads and user-owned state preserved. |
| Offboarding / reset | offboarding_or_device_reset | Disposes rebuildable state; routes every protected family through export-before-delete. |
| User-driven cleanup | low_disk_pressure | Discloses the full eviction order; trims disposable classes first; never auto-deletes user-owned state. |
| Admin-driven cleanup | managed_quota_pressure | When only protected classes remain over quota, the sheet is blocked with a guardrail notice rather than purging local state. |

## Guardrails enforced by the sheet

- No generic clear can erase a protected class.
- Protected classes are excluded unless explicitly selected.
- Rebuild cost, offline impact, and irreversible consequences are always
  disclosed — never hidden in logs.
- Low-disk ordering is disclosed in full on pressure-triggered sheets.
- Managed quota or disk pressure never silently deletes local user-owned state.
