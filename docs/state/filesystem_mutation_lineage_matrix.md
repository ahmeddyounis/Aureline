# Filesystem identity, watch fidelity, mutation lineage, and deferred-intent matrix

This document freezes the shared row vocabulary for the new file-bearing,
generated, virtual, and managed-write surfaces that expand beyond the
ordinary source-file editor model. The matrix is implemented by
[`crates/aureline-vfs/src/filesystem_mutation_lineage_matrix/mod.rs`](../../crates/aureline-vfs/src/filesystem_mutation_lineage_matrix/mod.rs),
validated by
[`crates/aureline-vfs/tests/filesystem_mutation_lineage_matrix.rs`](../../crates/aureline-vfs/tests/filesystem_mutation_lineage_matrix.rs),
serialized to
[`artifacts/state/filesystem_mutation_lineage_matrix.json`](../../artifacts/state/filesystem_mutation_lineage_matrix.json),
and replayed from
[`fixtures/state/filesystem_mutation_lineage_matrix/`](../../fixtures/state/filesystem_mutation_lineage_matrix/).

## Frozen vocabulary

The packet freezes one closed set for each contract axis:

- `root_class`: `local_filesystem`, `remote_agent`, `container_mount`,
  `archive_packaged`, `generated_managed`, `virtual_provider_backed`,
  `managed_offline_bundle`
- `path_identity_class`: `canonical_filesystem_object`,
  `generated_source_identity`, `provider_object_identity`,
  `imported_snapshot_identity`, `local_draft_identity`,
  `offline_bundle_identity`
- `watch_state`: `live_watch`, `reduced_fidelity_watch`,
  `polling_fallback`, `manual_refresh_only`, `provider_refresh_only`,
  `no_external_watch`
- `save_fallback`: `atomic_replace`, `conditional_remote_write`,
  `in_place_write`, `save_as_copy`, `regenerate_from_source`,
  `stage_local_draft`, `compare_only_blocked`
- `undo_class`: `exact_undo`, `compensating_undo`,
  `regenerate_recompute`, `restore_from_checkpoint`,
  `audit_only_non_undoable`
- `corruption_state`: `block_feature_only`,
  `rebuild_automatically`, `open_with_warning`, `repair_flow`,
  `backup_rollback`, `fail_closed_for_privileged_operations`
- `connectivity_state`: `connected`, `constrained`,
  `offline_local_safe`, `reauth_required`,
  `reconciliation_pending`, `service_unavailable`
- `reconciliation_posture`: `not_applicable`,
  `no_invisible_replay`, `revalidate_before_replay`,
  `manual_review_required`, `expire_without_replay`

## Matrix

| Row | Roots | Identity | Watch | Save | Undo | Corruption | Connectivity / reconcile | Coverage summary |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `notebook_document` | local, remote, container | canonical filesystem object | live | atomic replace | exact undo | repair flow | connected / n/a | canonical identity, watch, writable save target, mutation journal |
| `notebook_output_artifact` | generated, remote, container | generated source identity | reduced fidelity | regenerate from source | regenerate / recompute | rebuild automatically | connected / n/a | mutation journal only; no ordinary watch or direct save |
| `request_workspace_document` | local, remote, container | canonical filesystem object | live | atomic replace | exact undo | backup rollback | constrained / revalidate before replay | canonical identity, watch, writable save target, mutation journal, deferred-intent exposure |
| `request_response_snapshot` | provider-backed, archive | provider object identity | provider refresh only | save-as copy | audit-only | open with warning | service unavailable / n/a | inspect-first, no canonical file identity, no direct mutation journal |
| `database_export_artifact` | generated, local, remote | generated source identity | manual refresh only | regenerate from source | regenerate / recompute | rebuild automatically | constrained / n/a | generated lineage, no direct save target |
| `profiler_trace_artifact` | archive, local, offline bundle | imported snapshot identity | no external watch | save-as copy | audit-only | open with warning | offline local-safe / n/a | attributable packet, exportable, no ordinary file identity |
| `preview_output_artifact` | generated, container, provider-backed | generated source identity | reduced fidelity | regenerate from source | regenerate / recompute | rebuild automatically | constrained / n/a | generated preview output, no direct edit authority |
| `sync_packet_artifact` | offline bundle, local | offline bundle identity | manual refresh only | stage local draft | restore from checkpoint | repair flow | offline local-safe / revalidate before replay | writable packet, mutation journal, deferred-intent exposure |
| `provider_local_draft` | provider-backed, offline bundle | local draft identity | provider refresh only | stage local draft | compensating undo | repair flow | reconciliation pending / manual review required | local-first draft continuity, mutation journal, deferred-intent exposure |
| `infrastructure_overlay_document` | provider-backed, remote, container | provider object identity | provider refresh only | compare-only blocked | audit-only | block feature only | service unavailable / manual review required | provider truth layer, no ordinary save target |
| `imported_archive_capture` | archive, offline bundle | imported snapshot identity | no external watch | save-as copy | audit-only | open with warning | offline local-safe / n/a | inspect-only imported capture |

## Contract rules

- Presentation path, logical identity, canonical target, alias set, and save
  target stay distinct when the root can express them.
- Degraded watch or save posture must be named before the user trusts an edit
  or replay path.
- Writable or staged surfaces emit one attributable mutation-journal entry with
  an explicit undo class.
- Corruption routes by class and row; a generated artifact rebuild does not
  imply a user-authored notebook repair, and neither implies a whole-app reset.
- Deferred managed work never replays invisibly. Rows that stage drafts or
  offline packets must use `revalidate_before_replay` or
  `manual_review_required`, not `not_applicable`.

## Fixture coverage

The checked-in fixtures deliberately span all required root families:

- local notebook document
- generated notebook output
- remote request workspace document
- provider response snapshot
- generated database export
- archive profiler trace packet
- container preview output
- offline-safe sync packet
- provider local draft
- infrastructure overlay
- imported archive capture
