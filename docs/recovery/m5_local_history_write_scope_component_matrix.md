# M5 Local-History / Write-Scope Component Matrix (M05-892)

Frozen contract for Aureline's reusable local-history and write-scope components
across every claimed M5 mutation and recovery surface. The authoritative gate is the
Rust validator in
`crates/aureline-history/src/freeze_the_m5_local_history_row_checkpoint_group_card_restore_preview_card_retention_export_card_and_write_scope_preview_tree_component_matrix/`.
This doc describes the shape; the code and the checked-in support export are the truth.

## Why this lane exists

The current sheet already covers filesystem identity, save coordination,
mutation-journal lineage, reversible checkpoints, generated-artifact provenance,
refactor transactions, repair transactions, and Problems/output causality. What it
still lacked was governed truth for the reusable **components users actually read
before restoring or applying broad changes**: the local-history rows, the
checkpoint-group cards, the restore-preview cards, the retention/export cards, the
write-scope preview trees, the restore-granularity selectors, and the history-export
manifests. M5 cannot honestly claim diff-first recovery, attributable local history,
or preview-first multi-file mutation if users still have to infer who created a
checkpoint, what scope will restore or apply, whether generated or managed files are
affected, or how export/redaction and rollback interact.

## Component families (7)

| Family | Owns (family-specific vocabulary) |
| --- | --- |
| `local_history_row` | snapshot origins, actor classes, capture fidelities |
| `checkpoint_group_card` | checkpoint lineage classes, mutation classes |
| `restore_preview_card` | restore granularities, restore drift states |
| `retention_export_card` | retention postures, export-redaction postures |
| `write_scope_preview_tree` | write-scope classes, managed-file caveats |
| `restore_granularity_selector` | restore-selection modes |
| `history_export_manifest` | export-manifest classes (+ export-redaction postures) |

Every row also declares the shared vocabularies: surface families, deployment lines,
consumer surfaces, accessibility routes, required labels, a qualification class, and
downgrade triggers.

## Stable field vocabulary

- **Timestamp / actor / source** — `snapshot_origins` (manual_save, autosave,
  formatter_run, refactor_apply, ai_apply, external_import) and `actor_classes`
  (local_user, pair_participant, ai_agent, automation_task, import_bridge,
  unknown_actor), surfaced by the `timestamp_and_actor` required label.
- **Metadata-only capture** — `capture_fidelities` (full_body_snapshot, metadata_only,
  diff_only, pointer_reference, external_reference, redacted_capture). A metadata-only
  capture is never shown as a full-body snapshot that could restore.
- **Checkpoint lineage / mutation class** — `checkpoint_lineage_classes` (single_action,
  grouped_transaction, session_restore_point, milestone_tag, rollback_point,
  imported_checkpoint) and `mutation_classes` (text_edit, multi_file_refactor,
  generated_artifact, dependency_change, repair_transaction, config_migration). A
  grouped transaction is never collapsed into a single edit.
- **Restore granularity** — `restore_granularities` (whole_snapshot, per_file, per_hunk,
  per_symbol, selection_only, manual_merge) and `restore_drift_states` (clean_apply,
  local_edits_present, source_moved, source_deleted, external_drift, conflict_pending).
  A partial or manual restore is never shown as a whole-snapshot restore.
- **Scope narrowing** — `restore_selection_modes` (all_changes, choose_files,
  choose_hunks, choose_symbols, exclude_generated, dry_run_only). Scope narrowing is a
  first-class choice, never all-or-nothing.
- **Generated / managed-file caveats** — `write_scope_classes` (single_file, multi_file,
  whole_directory, cross_package, generated_tree, out_of_workspace) and
  `managed_file_caveats` (unmanaged, generated_file, managed_lockfile,
  vendored_dependency, protected_readonly, ignored_path).
- **Export / redaction posture** — `retention_postures` (session_only,
  workspace_retained, account_synced, policy_pinned, purge_pending, expired_purged),
  `export_redaction_postures` (full_metadata, paths_redacted, bodies_omitted,
  credentials_scrubbed, policy_restricted, export_blocked), and
  `export_manifest_classes` (support_bundle, recovery_evidence, audit_trail,
  migration_session, offline_mirror, redacted_share).

## Hard invariants (every row)

- `masks_actor_or_timestamp` — MUST be false.
- `hides_generated_or_managed_caveat` — MUST be false.
- `invents_private_history_grammar` — MUST be false.
- `bypasses_restore_scope_review` — MUST be false.

Any true value raises `component_invariant_violated`.

## Consumer obligations

Editor local-history, checkpoint-inspector, restore-review, refactor-preview,
AI-apply-review, recovery-center, and support-desk surfaces inherit the same mutation /
recovery component grammar. Every surface projects from this one canonical packet; no
surface invents a second restore or apply vocabulary. Support/export, CLI/headless, and
recovery-evidence consumers read the same stable field names and downgrade states.

## Artifacts

- Boundary schema: `schemas/ui/m5-local-history-write-scope-component-matrix.schema.json`
- Design matrix report: `artifacts/design/m5-local-history-write-scope-component-matrix.md`
- Release proof (canonical support export + CSV):
  `artifacts/release/m5-local-history-write-scope-component-proof/`
- Narrowed fixtures: `fixtures/ui/m5-local-history-write-scope-components/`

## Regenerating the checked-in artifacts

```sh
cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- support-export > artifacts/release/m5-local-history-write-scope-component-proof/support_export.json
cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- csv > artifacts/release/m5-local-history-write-scope-component-proof/matrix.csv
cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- report > artifacts/design/m5-local-history-write-scope-component-matrix.md
cargo run -q -p aureline-history --bin aureline_history_local_history_write_scope_component_matrix -- validate
```
