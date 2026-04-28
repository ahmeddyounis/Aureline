# Table-grid contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/table_grid_contract.md`](../../../docs/ux/table_grid_contract.md)
and the schema at
[`/schemas/ux/table_view_state.schema.json`](../../../schemas/ux/table_view_state.schema.json).

Each JSON file is a single `table_view_state_record`. The
`__fixture__` prelude is reviewer metadata; the canonical vocabulary
lives in the record itself. Raw row bodies, raw cell values, raw query
text, raw provider URLs, raw paths, and secret-bearing values never
appear.

## Cases

| Fixture | Surface | Scenario axis |
|---|---|---|
| [`large_virtualized_grid_selection.json`](./large_virtualized_grid_selection.json) | `result_grid_table` | Sticky headers, frozen columns, row/column virtualization, stable range anchor, and visible-row selection with matching-count disclosure. |
| [`redacted_cells_row_blockers.json`](./redacted_cells_row_blockers.json) | `support_table` | Redacted and policy-hidden cells preserve row-level blockers and safe export placeholders. |
| [`filter_preserving_export_review.json`](./filter_preserving_export_review.json) | `evidence_table` | Export preserves filter/sort/preset refs while naming selected/all-matching scope, hidden columns, redaction, and review packet refs. |
| [`mixed_fresh_stale_rows.json`](./mixed_fresh_stale_rows.json) | `schema_explorer_table` | Mixed fresh, stale, approximate, and provider-lag cell cues survive copy/export and reopened review state. |
