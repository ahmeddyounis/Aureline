# M5 cache-repair plan (human-readable)

Companion to the boundary schema at
[`/schemas/storage/m5_cache_repair.schema.json`](../../schemas/storage/m5_cache_repair.schema.json),
the contract at
[`/docs/storage/m5_cache_repair_contract.md`](../../docs/storage/m5_cache_repair_contract.md),
and the scenario corpus under
[`/fixtures/storage/m5_cache_repair_cases/`](../../fixtures/storage/m5_cache_repair_cases/).

A cache-repair plan is the operator-facing object the shell shows when a derived
cache or semantic index is detected corrupt or stale. It replaces vague
delete-everything advice with a targeted repair bounded to one storage class.
Every storage-class and posture value re-exports verbatim from
[`/artifacts/runtime/storage_classes.yaml`](../runtime/storage_classes.yaml); the
fault, repair-action, repair-scope, quarantine-disposition, repair-label,
repair-state, and fallback columns are the only sets this plan introduces, and
none of them carry a global / factory-reset value.

## What a plan states

- **Storage class + scope** — the affected class and the single-class scope
  (one workspace, or all workspaces that share the class). Never global.
- **Fault** — `corrupt_index`, `checksum_mismatch`, `partial_write_torn`,
  `schema_version_drift`, `stale_against_source`, `missing_backing_object`, or
  `orphaned_entries`.
- **Repair action** — the narrowest sufficient remedy for that class and fault.
- **Quarantine disposition** — whether the suspect copy is preserved (user-owned
  data, forensic value, or pending export) before any clear, or disposable-only.
- **Propagated labels** — the stale / rebuild-needed / corrupt label every
  affected surface keeps until the repair completes.
- **Fallback** — the narrower-or-equal action offered when a repair fails.
- **Open inspector / run targeted repair** — the actions that move the user from
  the plan into the storage inspector and the targeted repair (never a reset).

## Repair action per class and fault

| Storage class | Typical fault | Targeted action | Quarantine |
| --- | --- | --- | --- |
| interactive_hot_cache | torn / stale | re-derive on demand | none (disposable) |
| knowledge_cache | corrupt / drift / stale | rebuild / reindex from source | none (disposable) |
| artifact_cache | checksum mismatch / missing | refetch by digest | none (content-addressed) |
| prebuild_environment_cache | missing backing | refetch by digest | none (rebuildable) |
| evidence_support_cache | corrupt | quarantine, then class-specific review | **preserved (forensic / export)** |
| user_owned_recovery_state | torn | repair in place from checkpoint | **preserved (user-owned)** |

A non-protected class that happens to hold user-owned data is repaired with
`quarantine_then_rebuild`: the suspect copy is quarantined first, then rebuilt.

## Stale-label propagation

While a repair is outstanding, every surface that reads the affected class keeps
showing the plan's label — `corrupt`, `stale`, `rebuild_needed`,
`reindex_needed`, `repair_in_progress`, or `quarantined` — and the matching
runtime posture (`rebuild_pending` or `retained_for_evidence`). The label stays
active until `repair_state` reaches `repair_complete_healthy`; only then does
every surface clear to `healthy`.

## Scenarios covered

`knowledge_cache_corrupt_index_reindex`,
`artifact_pack_checksum_mismatch_refetch`, `generated_preview_torn_rederive`,
`evidence_trace_corrupt_quarantined_for_review`,
`recovery_state_torn_repair_in_place`, and
`prebuild_missing_backing_repair_failed_fallback`.

## Guardrails enforced by the plan

- The repair is targeted to one class; no plan ever offers a factory reset or a
  reset-everything fallback.
- A suspect copy that still holds user-owned data or forensic value is
  quarantined before any clear, and is never deleted to fix a cache.
- Evidence and user-owned recovery classes are never auto-rebuilt from a derived
  source and never cleared without preservation.
- Every affected surface keeps its stale / rebuild-needed / corrupt label until
  the repair actually completes; the labels are never hidden in logs-only
  diagnostics.
- A failed repair offers a narrower fallback (retry, widen-under-review,
  open-without-cache, or class-specific review), never a delete-all.
