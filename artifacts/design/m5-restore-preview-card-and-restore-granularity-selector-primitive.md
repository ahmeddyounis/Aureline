# M5 Restore-Preview-Card and Restore-Granularity-Selector Primitive

- Packet: `m5-restore-preview-card-restore-granularity-selector-primitive:stable:0001`
- Label: `M5 restore-preview-card and restore-granularity-selector primitive: past-vs-current comparison, exact object identity, external-drift baseline, generated/managed-file caveat, restore granularity, selectable apply scope, retention/export posture, preview and selector postures, and no-history-erasure truth with bounded inspect/restore/resolve/export and inspect/apply/narrow/exclude actions`
- Mutation / recovery consumers: 5 (5 stable)
- Preview postures: clean_restore_preview, local_drift_preview, managed_file_preview, external_drift_preview, conflict_preview, restore_blocked_preview
- Selector postures: whole_scope_selector, file_scoped_selector, range_scoped_selector, exclude_generated_selector, dry_run_only_selector, selector_blocked
- Preview actions: inspect_diff, restore_whole_file, restore_selected_range, resolve_conflict, export_as_patch, export_as_evidence
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Mutation / recovery consumers

- **Editor Restore**: `stable`
  - Owner: Editor restore owner
  - Scope: The editor restore surface renders the shared restore-preview card and restore-granularity selector so a clean restore compares past and current state, discloses exact object identity, and offers both a whole-file and a selected-range restore, and an external-drift restore surfaces the diverged baseline before any apply — every restore recording a new attributable checkpoint rather than an invisible rewrite of local history
  - Worked previews: 2
    - `src/editor/buffer.rs` (`clean_apply`) → `clean_restore_preview` (restore `true`, drift `false`, managed `false`, new-checkpoint `true`)
    - `src/editor/view.rs` (`external_drift`) → `external_drift_preview` (restore `true`, drift `true`, managed `false`, new-checkpoint `true`)
  - Worked selectors: 2
    - `restore scope: buffer.rs` → `whole_scope_selector` (default `all_changes`, apply `true`, narrow `false`)
    - `restore scope: editor selection (3 files)` → `range_scoped_selector` (default `choose_hunks`, apply `true`, narrow `true`)
- **AI Apply Restore**: `stable`
  - Owner: AI apply restore owner
  - Scope: The AI apply restore surface renders the shared restore-preview card and restore-granularity selector so a restore that reaches a generated or managed file discloses the managed caveat and defaults the selector to exclude generated files, never silently overwriting a generated artifact, and always records a new attributable checkpoint
  - Worked previews: 1
    - `src/ai/bindings.rs` (`clean_apply`) → `managed_file_preview` (restore `true`, drift `false`, managed `true`, new-checkpoint `true`)
  - Worked selectors: 1
    - `restore scope: regenerate bindings (3 files)` → `exclude_generated_selector` (default `exclude_generated`, apply `true`, narrow `true`)
- **Import Restore**: `stable`
  - Owner: Import restore owner
  - Scope: The import restore surface renders the shared restore-preview card and restore-granularity selector so an imported restore that would land over local edits discloses the local drift and offers a file-scoped narrowing, preserving the existing history trail without masquerading as an invisible rewrite
  - Worked previews: 1
    - `config/settings.toml` (`local_edits_present`) → `local_drift_preview` (restore `true`, drift `false`, managed `false`, new-checkpoint `true`)
  - Worked selectors: 1
    - `restore scope: imported settings (2 files)` → `file_scoped_selector` (default `choose_files`, apply `true`, narrow `true`)
- **Repair Restore**: `stable`
  - Owner: Repair restore owner
  - Scope: The repair restore surface renders the shared restore-preview card and restore-granularity selector so a restore blocked behind a pending conflict offers resolve-conflict rather than a false restore, and the selector stays dry-run-only until the conflict clears — the same drift-first vocabulary a support reviewer reads elsewhere
  - Worked previews: 1
    - `src/repair/transaction.rs` (`conflict_pending`) → `conflict_preview` (restore `false`, drift `false`, managed `false`, new-checkpoint `true`)
  - Worked selectors: 1
    - `restore scope: repair transaction (2 files)` → `dry_run_only_selector` (default `dry_run_only`, apply `false`, narrow `false`)
- **Recovery Center**: `stable`
  - Owner: Recovery center owner
  - Scope: The recovery center renders the shared restore-preview card and restore-granularity selector so a restore whose source was deleted and whose restore path is unavailable reads as restore-blocked rather than falsely offering a restore, and the selector can only dry-run — the export/redaction and no-history-erasure vocabulary staying identical across every mutation and recovery surface
  - Worked previews: 1
    - `docs/notes.md` (`source_deleted`) → `restore_blocked_preview` (restore `false`, drift `true`, managed `false`, new-checkpoint `true`)
  - Worked selectors: 1
    - `restore scope: recovered notes.md` → `selector_blocked` (default `dry_run_only`, apply `false`, narrow `false`)
