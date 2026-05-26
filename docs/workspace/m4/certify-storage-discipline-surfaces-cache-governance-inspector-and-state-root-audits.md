# State-root certification lineage for storage-discipline surfaces, cache-governance inspector, and state-root audits — contract

This is the companion contract document for the state-root
certification lineage record. It explains the closed vocabularies,
the pillar invariants, and the narrowing posture the projection
enforces. The canonical truth source is the projection in
[`/crates/aureline-workspace/src/state_root_certification_lineage/`](../../../crates/aureline-workspace/src/state_root_certification_lineage/)
and the proof packet at
[`/artifacts/workspace/m4/certify-storage-discipline-surfaces-cache-governance-inspector-and-state-root-audits.md`](../../../artifacts/workspace/m4/certify-storage-discipline-surfaces-cache-governance-inspector-and-state-root-audits.md);
this document supports human review and does not redeclare contract
truth that the typed projection already carries.

## Vocabularies

### State-root resource classes

The state-root resource enumeration is closed:

- `persistent_state_envelope` — top-level persistent-state envelope binding the layers below.
- `workspace_state_root` — durable workspace state root (layout, panes, sessions).
- `profile_root` — portable user-profile root (keymaps, settings, presets).
- `recent_work_root` — persistent recent-work / entry-restore registry root.
- `local_history_root` — local-history corpus root.
- `restore_checkpoint_root` — named restore-checkpoint root.
- `cache_governance_root` — cache-governance inspector root (the storage-discipline directory the inspector reads from).
- `prebuild_cache_root` — *(optional)* prebuild / build-artifact cache root.
- `mutation_journal_root` — *(optional)* mutation-journal artifact root.

The first seven are required on every Stable corpus.

### Audit surfaces

The audit-surface enumeration is closed:

- `storage_discipline_overview` — Settings / Storage Discipline overview panel.
- `cache_governance_inspector` — cache-governance inspector (per-class drill-in).
- `state_root_audit_panel` — state-root audit panel (per-resource drill-in).
- `cleanup_inventory_audit` — cleanup-inventory audit (covering cleanup-surface reachability).
- `eviction_rule_audit` — eviction-rule audit (cross-checking eviction policy honesty).
- `headless_audit_cli` — headless audit CLI (`aureline audit ...`).
- `support_export_audit_section` — support-export audit section shipped with support bundles.

All seven are required on every Stable corpus.

### Audit findings

The audit-finding enumeration is closed:

- `audit_clean` — resource clean; no cleanup or repair needed.
- `audit_clean_with_disclosure` — resource clean but a disclosure rides on top.
- `audit_dirty_with_disclosure` — resource dirty (drift, residue, schema mismatch) and shipped behind a non-empty disclosure ref.
- `audit_inconclusive_held` — audit could not classify the resource safely; held for manual review.
- `audit_refused_unsafe` — audit refused to evaluate the resource (e.g. trust-policy or license narrowing); held.

A `audit_clean_with_disclosure`, `audit_dirty_with_disclosure`,
`audit_inconclusive_held`, or `audit_refused_unsafe` finding must
carry a non-empty `audit_disclosure_ref`. A
`audit_dirty_with_disclosure`, `audit_inconclusive_held`, or
`audit_refused_unsafe` finding must additionally bind at least one
`cleanup_surface_refs` entry and at least one `inspection_hook_refs`
entry so a destructive cleanup never fires without user-visible
review.

### Rerun postures

- `explicit_user_action_required` — requires an explicit user commit before re-running.
- `terminal_no_further_run` — terminal and does not re-fire.
- `silent_rerun_permitted` — may re-fire silently. **Forbidden on Stable rows.**

### Redaction classes

- `metadata_only` — default; only metadata-safe fields ship.
- `redacted_with_disclosure` — body shipped behind an explicit override-disclosure ref.
- `excluded_by_policy` — body intentionally excluded by policy (trust / license / export control).

`redacted_with_disclosure` requires a non-empty
`redaction_disclosure_ref`.

### Claimed stable profiles

The claimed-stable-profile enumeration is closed:

- `stable_default` — default Stable profile.
- `stable_support_export` — Stable profile under the support / shiproom export lane.
- `stable_restricted_mode` — Stable profile under restricted mode (read-only narrowing).
- `narrowed_below_stable` — explicitly narrowed below Stable.

The projection refuses to certify on `narrowed_below_stable` and
narrows the record with `claimed_profile_not_stable` so a row that
no longer qualifies cannot inherit adjacent green rows.

### Inspection hook classes

The pre-action inspection / cleanup / repair hook set is closed:

- `inspect_state_root`
- `compare_before_cleanup`
- `preview_cleanup`
- `preview_repair`
- `rollback_cleanup`
- `rollback_repair`
- `export_before_cleanup`
- `export_before_repair`

All eight are required reachable on a Stable corpus.

## Pillars

The pillar projection re-derives the contract claims from the input
envelope verbatim. Each pillar narrows the record below Stable with
a named reason when the input cannot be proven.

| Pillar | Narrow reason |
|---|---|
| Resource-class coverage | `required_resource_class_missing` |
| Audit-surface coverage | `required_audit_surface_missing` |
| Storage-class taxonomy pinning | `storage_class_ref_missing` |
| Audit-finding honesty | `audit_disclosure_missing` |
| Redaction honesty | `redaction_disclosure_missing` |
| Cleanup-precondition truth | `cleanup_precondition_missing` |
| Rerun posture honesty | `rerun_silent_forbidden` |
| Commit-action metadata | `commit_action_metadata_missing` |
| Restore-provenance preservation | `restore_provenance_not_preserved` |
| Encoding fidelity preservation | `encoding_fidelity_not_preserved` |
| Trust-state preservation | `trust_state_not_preserved` |
| Lineage-refs preservation | `lineage_refs_not_preserved` |
| Audit-transaction id pinning | `audit_transaction_id_not_pinned` |
| Finding-code pinning | `finding_code_missing` |
| Audit-surface reachability | `audit_surface_unreachable` |
| Audit-surface disclosure | `audit_surface_disclosure_gap` |
| Inspection-hook reachability | `inspection_hook_unavailable` |
| Support-export field honesty | `support_export_fields_dropped` |
| Support-export redaction honesty | `support_export_redaction_unsafe` |
| Claimed-profile honesty | `claimed_profile_not_stable` |
| Producer attribution | `producer_attribution_incomplete` |
| Lineage export safety | `lineage_export_unsafe` |
| Corpus emptiness | `corpus_empty` |

## Composition with the rest of the M4 lineage lane

The state-root certification lineage layers on top of the existing
lineages rather than redeclaring their truth:

- `storage_class_ref` is an opaque ref into the cache / storage-class
  lineage and binds the audit to a governed eviction policy without
  copying the policy enum.
- The audit hooks reuse the same `compare_before_*`,
  `rollback_*`, `export_before_*` posture the schema-migration /
  repair lineage and local-history export/replay lineages already
  enforce.
- Cleanup-surface refs reuse the cache / storage-class lineage
  `REQUIRED_CLEANUP_SURFACES` vocabulary as opaque refs.

## See also

- Cache / storage-class lineage:
  [`/crates/aureline-workspace/src/cache_storage_class_lineage/`](../../../crates/aureline-workspace/src/cache_storage_class_lineage/)
- Schema-migration and repair lineage:
  [`/crates/aureline-workspace/src/schema_migration_and_repair_lineage/`](../../../crates/aureline-workspace/src/schema_migration_and_repair_lineage/)
- Portable-state lineage:
  [`/crates/aureline-workspace/src/portable_state_lineage/`](../../../crates/aureline-workspace/src/portable_state_lineage/)
- Recovery-ladder lineage:
  [`/crates/aureline-workspace/src/recovery_ladder_lineage/`](../../../crates/aureline-workspace/src/recovery_ladder_lineage/)
