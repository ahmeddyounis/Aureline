# Tree view interaction fixtures

Seed corpus for the hierarchy interaction contract frozen in
[`/docs/ux/tree_view_contract.md`](../../../docs/ux/tree_view_contract.md)
and the schema at
[`/schemas/ux/tree_row.schema.json`](../../../schemas/ux/tree_row.schema.json).

Each fixture is a single JSON document validating as a
`tree_view_snapshot_record`. The cases exercise tree virtualization,
lazy hydration, keyboard navigation, active/current/selected/open state,
hidden and filtered counts, batch-selection truth, drag or move posture,
inline versus deferred action exposure, and provider fallback.

## Cases

| Fixture | Surface | Scenario axis |
|---|---|---|
| [`file_tree_deep_lazy_virtualized.json`](./file_tree_deep_lazy_virtualized.json) | `file_tree` | Deep nesting, virtualized offscreen counts, and lazy child hydration preserve focus and selected identity. |
| [`schema_tree_filtered_generated_hidden.json`](./schema_tree_filtered_generated_hidden.json) | `schema_tree` | Filtered schema hierarchy keeps generated-hidden and policy-hidden counts explicit while preserving hidden selected rows. |
| [`outline_tree_filter_active_open_selection.json`](./outline_tree_filter_active_open_selection.json) | `outline_tree` | Filtered outline keeps focus, current item, active target, open row, selection, and range anchor separate. |
| [`package_tree_provider_partially_available.json`](./package_tree_provider_partially_available.json) | `package_tree` | Package provider outage keeps cached rows inspectable, provider-backed branches expandable as placeholders, and blocked rows reviewable. |
