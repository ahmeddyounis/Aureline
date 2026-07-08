# M5 rebase-todo / sequence-editor component fixtures

Protected fixtures for the implemented Git sequence-edit components (task
M05-959): rebase todo rows and sequence-editor headers. Each fixture is a
complete, valid `git_rebase_sequence_edit_component_truth` packet exercising a
narrowed scenario. All fixtures validate clean against both the typed `validate`
and `schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json`.

- `reordered_plan.json` — a rebase plan where two commits move position; each
  reordered row keeps its `original_index` explicit alongside the new
  `display_index`, and the raw todo line stays meaning-equivalent with the
  structured card.
- `dropped_step_recovery.json` — a dropped commit gains a conflict blocker that
  is disclosed, yet the commit stays recoverable through the reflog fallback, so
  recovery remains reachable after the risky mutation.

Regenerate with the canonical export and summary via:

    GEN_REBASE_SEQUENCE_ARTIFACTS=1 cargo test -p aureline-git --lib \
      implement_rebase_todo_rows_and_sequence_editor_headers::tests::generate_artifacts

Do not hand-edit; the generator is the source of truth.
