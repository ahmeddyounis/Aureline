# M5 Table / Grid and Panel-Header Controls

- Packet: `m5-table-grid-panel-header-controls:stable:0001`
- Label: `M5 table/grid and panel-header controls with sort/filter provenance, selection bars, pinned-column identity, per-value qualification, bounded panel-header action budgets, and exact/loaded/all-matching/hidden/outside-scope count truth aligned across request/data, review, search, governance, and support surfaces`
- Consumer surfaces: 6
- Count scope kinds: exact_count, loaded_count, all_matching_count, hidden_by_filter, hidden_by_policy, outside_current_scope, scope_unresolved
- Sort / filter provenances: user_sorted, default_sort, relevance_ranked, imported_order, filter_applied, unsorted, provenance_unknown
- Value qualifications: exact_canonical, estimated, imported, stale, partial, policy_limited, qualification_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **data_ui**: `stable`
  - Owner: Request / data grid owner
  - Scope: The request/data grid names selection-versus-current, sort/filter provenance, pinned identity columns, per-value qualification, and exact/loaded/all-matching/hidden/outside-scope counts, degrading when the provenance is unstated or a count scope collapses
  - Table / grid examples: 4 / panel-header examples: 2
- **review_ui**: `stable`
  - Owner: Review-queue grid owner
  - Scope: The review queue reuses the shared grid selection and scope grammar for queued items, keeping blocked state keyboard-discoverable and degrading when a loaded subset is shown as the exact total or a header overloads into a toolbar
  - Table / grid examples: 3 / panel-header examples: 2
- **search_ui**: `stable`
  - Owner: Search results grid owner
  - Scope: The search surface reuses the same grid row semantics for result collections, honestly naming a relevance-ranked loaded subset and degrading when the current selection is hover-only or the header's active context is unresolved
  - Table / grid examples: 2 / panel-header examples: 2
- **shell_ui**: `stable`
  - Owner: Governance / shell grid owner
  - Scope: Governance surfaces reuse the same grid grammar, keeping pinned identity columns stable under virtualization and never presenting an estimated value as canonical, degrading when a pinned column is lost or a background context reads as active
  - Table / grid examples: 3 / panel-header examples: 2
- **support_export**: `stable`
  - Owner: Support/export grid owner
  - Scope: The support export carries the same resolved grid and header truth, so a stale-shown-complete grid, an unresolved count scope, an unresolved value qualification, hover-only header actions, or a dropped overflow action is visible in evidence rather than hidden behind compact chrome
  - Table / grid examples: 4 / panel-header examples: 3
- **product_ui**: `stable`
  - Owner: In-product grid owner
  - Scope: In-product surfaces reuse the same selection, sort/filter, and scope grammar a user sees in the request/data and review grids, always offering the command-backed scope detail and degrading honestly when the trace path is missing or the pinned column is unresolved
  - Table / grid examples: 5 / panel-header examples: 3
