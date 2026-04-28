# Modal-editing status fixtures

Worked examples for
[`docs/ux/modal_editing_status_contract.md`](../../../docs/ux/modal_editing_status_contract.md)
and
[`schemas/ux/modal_state.schema.json`](../../../schemas/ux/modal_state.schema.json).

Each fixture is one `modal_state_record` and includes fixture-only
`$schema` and `__fixture__` keys for humans.

## Cases

- [`source_editor_operator_macro_leader.json`](./source_editor_operator_macro_leader.json)
  - full modal support in a source editor with visible mode, pending
  operator, leader path, macro recording state, and sequence-help refs.
- [`rename_field_modeless_fallback.json`](./rename_field_modeless_fallback.json)
  - rename-field fallback where destructive modal input is not silently
  captured and Escape behavior is explicit.
- [`dialog_escape_recovery.json`](./dialog_escape_recovery.json)
  - dialog focus temporarily suspends modal dispatch, records the prior
  mode and focus target, and recovers predictably on Escape.
- [`search_text_input_exemption.json`](./search_text_input_exemption.json)
  - search field preserves literal text input during macro recording and
  exports only metadata for the exempt entry.
- [`form_field_review_fallback.json`](./form_field_review_fallback.json)
  - a form field that can affect downstream artifacts stays modeless,
  records field review posture, and blocks modal macro capture.
- [`terminal_passthrough_unavailable.json`](./terminal_passthrough_unavailable.json)
  - terminal focus visibly narrows modal editing to passthrough and
  exposes help, palette, and retry-in-editor paths.
