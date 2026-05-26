# Schema-migration and repair lineage for workspace, profile, and persistent-state artifacts — contract

This is the companion contract document for the schema-migration and
repair lineage record. It explains the closed vocabularies, the
pillar invariants, and the narrowing posture the projection enforces.
The canonical truth source is the projection in
[`/crates/aureline-workspace/src/schema_migration_and_repair_lineage/`](../../../crates/aureline-workspace/src/schema_migration_and_repair_lineage/)
and the proof packet at
[`/artifacts/workspace/m4/finalize-schema-migration-and-repair-flows-for-workspace.md`](../../../artifacts/workspace/m4/finalize-schema-migration-and-repair-flows-for-workspace.md);
this document supports human review and does not redeclare contract
truth that the typed projection already carries.

## Vocabularies

### Artifact classes

The artifact-class enumeration is closed:

- `workspace_state_artifact` — durable workspace state (layout, panes, sessions).
- `profile_artifact` — portable user profile (keymaps, settings, presets).
- `recent_work_registry` — persistent recent-work / entry-restore registry.
- `local_history_corpus` — local-history entry / group corpus.
- `restore_checkpoint` — named restore checkpoint snapshot.
- `persistent_state_envelope` — top-level persistent-state envelope binding the layers above.
- `prebuild_cache_artifact` — *(optional)* prebuild / build-artifact cache row.
- `mutation_journal_artifact` — *(optional)* mutation-journal artifact row.

The first six are required on every Stable corpus.

### Repair flow kinds

The repair-flow enumeration is closed:

- `inspect_repair` — open the typed repair-inspector for an artifact (read-only).
- `rebuild_derived_store` — rebuild a derived store (search/symbol index, derived layout cache) from authoritative sources.
- `rehydrate_from_packet` — rehydrate the artifact from a checked-in packet (local history, support-bundle replay).
- `quarantine_corrupt_artifact` — quarantine a corrupt artifact so it cannot ship into the next session.
- `restore_from_checkpoint` — restore the artifact from a named restore checkpoint.
- `manual_repair_handoff` — hand off to a typed manual repair flow (out-of-band repair console / support ticket).

All six are required on every Stable corpus.

### Schema-compatibility classes

The schema-compatibility enumeration is closed:

- `current_schema` — already on the current schema version; no migration needed.
- `older_supported_schema` — on an older schema with a supported forward migration path.
- `older_unsupported_requires_repair` — on an older schema with no supported forward migration; repair is required.
- `newer_unknown_schema_refused` — on a newer schema version this runtime does not understand; refused.
- `corrupt_artifact` — the artifact is corrupt and cannot be classified safely.

### Migration outcomes

The migration-outcome enumeration is closed:

- `no_migration_needed` — artifact already on the current schema; no migration applied.
- `forward_migrated_lossless` — forward migration applied with no observable user-state loss.
- `forward_migrated_lossy_with_disclosure` — forward migration applied with observable user-state changes shipped behind an explicit override disclosure.
- `migration_refused_unsafe` — migration refused; held for repair.
- `migration_awaiting_user_review` — migration paused awaiting explicit user review.
- `compat_report_only` — compatibility-only report (no migration applied), e.g. for a newer-than-runtime schema.

A `forward_migrated_lossy_with_disclosure`, `migration_refused_unsafe`,
`migration_awaiting_user_review`, or `compat_report_only` outcome
must carry a non-empty `migration_disclosure_ref`. The
`forward_migrated_lossless` and `forward_migrated_lossy_with_disclosure`
outcomes mutate persistent state and must declare a non-empty
`commit_action_id` and `commit_disclosure_id`.

### Repair outcomes

The repair-outcome enumeration is closed:

- `repair_succeeded_lossless` — repair succeeded with no observable user-state loss.
- `repair_succeeded_lossy_with_disclosure` — repair succeeded with observable user-state changes shipped behind an explicit override disclosure.
- `repair_refused_unsafe` — repair refused; held.
- `repair_awaiting_user_action` — repair paused awaiting explicit user action.
- `repair_quarantined` — repair quarantined the artifact rather than mutating it.

A `repair_succeeded_lossy_with_disclosure`, `repair_refused_unsafe`,
`repair_awaiting_user_action`, or `repair_quarantined` outcome must
carry a non-empty `repair_disclosure_ref`.

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

### Inspection hook classes

The pre-action inspection / repair hook table is closed:
`inspect_artifact`, `compare_before_migration`, `preview_migration`,
`preview_repair`, `rollback_migration`, `rollback_repair`,
`export_before_migration`, `export_before_repair`. All eight must be
reachable before any destructive migration or repair commits.

## Pillars

1. **Artifact-class coverage truth.**
2. **Repair-flow coverage truth.**
3. **Schema-version pinning truth.**
4. **Migration-outcome honesty.**
5. **Repair-outcome honesty.**
6. **Redaction honesty.**
7. **Restore-provenance / encoding / trust-state / no-rerun-semantics preservation.**
8. **No-silent-rerun honesty.**
9. **Repair-transaction-id / finding-code pinning.**
10. **Pre-action inspection-hook honesty.**
11. **Support-export honesty.**
12. **Producer attribution.**
13. **Lineage and export honesty.**

Each pillar is enforced by a narrow reason in the projection: a
posture that cannot prove the pillar narrows below Stable with a
named reason, so it never inherits an adjacent green row.

## Replay safety

The projection self-describes its stable qualification, so a posture
that cannot prove the contract sets
`stable_qualification.level = narrowed_below_stable` with a named
list of `narrow_reasons`. Support exports and replay pipelines should
refuse to apply a record whose
`producer_attribution.producer_attribution_complete` is false or
whose `raw_payload_excluded` is not true.
