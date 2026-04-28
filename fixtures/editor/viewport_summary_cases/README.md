# Editor Viewport Summary Cases

These fixtures exercise
[`docs/ux/editor_viewport_summary_contract.md`](../../../docs/ux/editor_viewport_summary_contract.md)
against the schema at
[`schemas/editor/viewport_summary.schema.json`](../../../schemas/editor/viewport_summary.schema.json).

Coverage:

- `fold_heavy_hidden_state.json` verifies folded-region summaries,
  hidden diagnostics, hidden search hits, and review markers.
- `large_file_summary_suppressed.json` verifies large-file suppression,
  chunked search ticks, and partial diagnostic summaries.
- `degraded_semantics_partial_markers.json` verifies stale or
  low-confidence semantic sources do not render as exact truth.
- `narrow_editor_group_replacements.json` verifies constrained-width
  editor groups hide or compact summary surfaces with replacement
  routes.

All cases assert that viewport summaries are optional accelerators and
never the only route to discover or act on warnings, folds, search
hits, review anchors, or diagnostics.
