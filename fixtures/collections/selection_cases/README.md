# Selection-state contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/selection_and_scope_contract.md`](../../../docs/ux/selection_and_scope_contract.md)
and the schema at
[`/schemas/collections/selection_state.schema.json`](../../../schemas/collections/selection_state.schema.json).

Each JSON file is a single `selection_state_record`. The `__fixture__`
prelude is reviewer metadata; the canonical vocabulary lives in the
record itself.

## Cases

- [`virtualized_table_visible_to_matching.json`](./virtualized_table_visible_to_matching.json)
  - virtualized search table escalates from a visible-row action to an
  all-matching query scope, keeps focus/current/activation separate, and
  requires review before export or remote mutation.
- [`filtered_hidden_selected_disclosure.json`](./filtered_hidden_selected_disclosure.json)
  - explicit custom set survives filtering with hidden selected and
  not-loaded selected counts inspectable from keyboard and assistive
  technology.
- [`range_anchor_sort_virtualization.json`](./range_anchor_sort_virtualization.json)
  - range selection keeps a stable anchor while sorting and
  virtualization churn reorder or unmount rows.
- [`provider_review_cancel_loaded_scope.json`](./provider_review_cancel_loaded_scope.json)
  - provider-backed loaded-set selection offers all-matching escalation,
  keeps the review cancellable, and forbids visible or loaded controls
  from using ambiguous all-language.
