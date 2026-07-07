# M5 Local-History-Row, Checkpoint-Group-Card, Restore-Preview-Card, Retention/Export-Card, Write-Scope-Preview-Tree, Restore-Granularity-Selector, and History-Export-Manifest Component Matrix

- Packet: `m5-local-history-write-scope-components:stable:0001`
- Label: `M5 local-history-row, checkpoint-group-card, restore-preview-card, retention/export-card, write-scope-preview-tree, restore-granularity-selector, and history-export-manifest component matrix`
- Component families: 7 (7 stable)
- Snapshot origins: manual_save, autosave, formatter_run, refactor_apply, ai_apply, external_import
- Restore granularities: whole_snapshot, per_file, per_hunk, per_symbol, selection_only, manual_merge
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Component families

- **local_history_row**: `stable`
  - Owner: Local-history row owner
  - Scope: One local-history-row model naming when a snapshot was captured, what produced it — manual save, autosave, formatter run, refactor apply, AI apply, or external import — who authored it, and how much was captured, so a user never has to infer who created a snapshot or whether a metadata-only capture could actually restore
  - Required labels: identity, state, keyboard_route, timestamp_and_actor, file_or_object_identity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **checkpoint_group_card**: `stable`
  - Owner: Checkpoint-group card owner
  - Scope: One checkpoint-group-card model naming whether a checkpoint is a single action, a grouped transaction, a session-restore point, a milestone tag, a rollback point, or an imported checkpoint, and what class of mutation it captured, so a grouped transaction or session-restore point is never collapsed into a single edit
  - Required labels: identity, state, keyboard_route, timestamp_and_actor, file_or_object_identity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **restore_preview_card**: `stable`
  - Owner: Restore-preview card owner
  - Scope: One restore-preview-card model naming how much a restore will restore — the whole snapshot, per-file, per-hunk, per-symbol, the selection only, or a manual merge — and how the target has drifted since capture, so a partial or manual restore is never shown as a whole-snapshot restore and never applies over local edits or a moved / deleted file silently
  - Required labels: identity, state, keyboard_route, file_or_object_identity, scope_or_redaction
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **retention_export_card**: `stable`
  - Owner: Retention/export card owner
  - Scope: One retention/export-card model naming how long local history is kept — session-only, workspace-retained, account-synced, policy-pinned, purge-pending, or expired-purged — and how it redacts on export, so a purge-pending or expired history is never shown as retained and a redacted export is never shown as a full export
  - Required labels: identity, state, keyboard_route, scope_or_redaction
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **write_scope_preview_tree**: `stable`
  - Owner: Write-scope preview tree owner
  - Scope: One write-scope-preview-tree model naming how wide an apply reaches — a single file, several files, a whole directory, across packages, a generated tree, or out of the workspace — and which generated, managed, vendored, protected, or ignored files it touches, so a preview never understates the blast radius of a multi-file apply or restores over a generated or managed file without saying so
  - Required labels: identity, state, keyboard_route, file_or_object_identity, scope_or_redaction
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **restore_granularity_selector**: `stable`
  - Owner: Restore-granularity selector owner
  - Scope: One restore-granularity-selector model naming the selectable apply scope — apply all changes, choose files, choose hunks, choose symbols, exclude generated files, or dry-run only — so scope narrowing is a first-class choice and a broad apply is never forced as all-or-nothing
  - Required labels: identity, state, keyboard_route, scope_or_redaction
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **history_export_manifest**: `stable`
  - Owner: History-export manifest owner
  - Scope: One history-export-manifest model naming what an export bundle contains — a support bundle, recovery evidence, an audit trail, a migration session, an offline mirror, or a redacted share — and how it is redacted, so an export is never mislabelled and a redacted share is never shown as a full-metadata export
  - Required labels: identity, state, keyboard_route, scope_or_redaction
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
