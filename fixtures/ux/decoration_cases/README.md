# Decoration conflict fixtures

Worked examples for
[`docs/ux/decoration_precedence_contract.md`](../../../docs/ux/decoration_precedence_contract.md)
and the legend at
[`artifacts/ux/status_icon_legend.yaml`](../../../artifacts/ux/status_icon_legend.yaml).

Each JSON file is a `decoration_conflict_case` that records the input
state families, the expected primary visual state, collapsed states,
accessible-name requirements, and detail sections. The corpus lets a
reviewer test actual row, tab, card, badge-list, editor, and review
renderings without relying on screenshots alone.

## Cases

| Fixture | Scenario axis |
| --- | --- |
| [`policy_blocked_generated_stale_table_row.json`](./policy_blocked_generated_stale_table_row.json) | A policy block outranks generated, stale, lifecycle, and support badges on a dense table row. |
| [`same_object_multi_surface_projection.json`](./same_object_multi_surface_projection.json) | The same object preserves one dominant meaning across table row, tab, card, and badge list. |
| [`editor_inline_precedence_conflict.json`](./editor_inline_precedence_conflict.json) | Editor inline and gutter decorations suppress lower-priority hints without losing detail. |
| [`review_high_severity_over_freshness.json`](./review_high_severity_over_freshness.json) | A high-severity review cue outranks freshness and lifecycle claims. |
| [`lifecycle_support_axes_not_collapsed.json`](./lifecycle_support_axes_not_collapsed.json) | Lifecycle and support class remain separate axes under a degraded package card. |
| [`compact_high_contrast_summary.json`](./compact_high_contrast_summary.json) | Compact high-contrast rendering keeps trust/blocking text and summarizes lower states accessibly. |
