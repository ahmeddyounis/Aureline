# Editor selection-state contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/editor_selection_contract.md`](../../../docs/ux/editor_selection_contract.md)
and the schema at
[`/schemas/editor/selection_state.schema.json`](../../../schemas/editor/selection_state.schema.json).

Each JSON file is a single `editor_selection_state_record`. The
`__fixture__` prelude is reviewer metadata; the canonical vocabulary
lives in the record itself.

## Cases

- [`primary_selection_replace_scope.json`](./primary_selection_replace_scope.json)
  - single primary caret with a non-empty active selection drives
    in-file replace from selection scope, with explicit widening rules.
- [`multi_cursor_ime_primary_only.json`](./multi_cursor_ime_primary_only.json)
  - multiple carets remain visible while an IME composition narrows
    completion/snippet behavior to the primary caret.
- [`column_and_line_selection_status.json`](./column_and_line_selection_status.json)
  - column selection and line selection use distinct editor vocabulary,
    counts, undo grouping, and status/export labels.
- [`line_selection_whole_line_delete.json`](./line_selection_whole_line_delete.json)
  - whole-line selection names complete logical lines, line-ending
    inclusion, grouped undo, and editor-specific status copy.
- [`structural_modal_snippet_quickfix.json`](./structural_modal_snippet_quickfix.json)
  - structural selection, snippet placeholder state, modal operator
    visibility, and quick-fix preview consume one scope record.
- [`workspace_replace_scope_widening.json`](./workspace_replace_scope_widening.json)
  - selected text can widen to file or workspace matching scope only
    after visible-vs-all-matching disclosure and preview linkage.
- [`compare_generated_readonly_blocked.json`](./compare_generated_readonly_blocked.json)
  - compare/generated/read-only editor state preserves selection for
    copy and inspect while write/apply paths block or route to review.
