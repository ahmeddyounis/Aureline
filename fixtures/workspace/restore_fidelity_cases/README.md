# Restore fidelity cases

These fixtures exercise the closed restore-fidelity vocabulary used by
startup recovery, restore summaries, diagnostics, and support export.
Each file is a `state_restore_provenance_and_placeholder_record` matching
[`schemas/state/restore_provenance_record.schema.json`](../../../schemas/state/restore_provenance_record.schema.json).

| Fixture | Fidelity | Scenario |
|---|---|---|
| `exact_reopen_closed_editor.yaml` | `exact` | user-closed editor/diff state restored without treating the closure as crash loss |
| `compatible_schema_migration.yaml` | `compatible` | schema translation with equivalence, rollback, compare, and export refs |
| `layout_only_terminal_no_rerun_changed_display.yaml` | `layout_only` | terminal transcript and display-topology adjustment restored without command rerun |
| `recovered_drafts_crash_dirty_buffer.yaml` | `recovered_drafts` | dirty-buffer journals recovered as reviewable drafts |
| `evidence_only_missing_target_after_wake.yaml` | `evidence_only` | remote, debug, and credential-gated surfaces retained as evidence only |
