# M5 Tree-View and List-View Controls

- Packet: `m5-tree-view-list-view-controls:stable:0001`
- Label: `M5 tree-view and list-view controls with virtualization, keyboard-complete disclosure, selection-versus-current distinction, capped inline-action budgets, and exact/loaded/hidden/outside-scope count truth aligned across explorer, search, review-queue, provider, help, and support surfaces`
- Consumer surfaces: 6
- Count scope kinds: exact_count, loaded_count, all_matching_count, hidden_by_filter, hidden_by_policy, outside_current_scope, scope_unresolved
- Drag / reorder postures: reorder_enabled, reorder_within_scope_only, reorder_disabled_by_policy, reorder_read_only, reorder_not_supported, reorder_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **explorer_ui**: `stable`
  - Owner: Explorer tree owner
  - Scope: The explorer tree names disclosure, selection-versus-current, per-row item state, and exact/loaded/hidden/outside-scope counts with virtualization honest under deep nesting, degrading when a lazily-unloaded subtree is drawn as an empty leaf or a count scope collapses
  - Tree-view examples: 6 / list-view examples: 3
- **search_ui**: `stable`
  - Owner: Search results owner
  - Scope: The search surface reuses the same tree and list row semantics for result collections, keeping exact-versus-loaded-versus-all-matching scopes distinct and degrading when a count scope collapses or the current selection is hover-only
  - Tree-view examples: 2 / list-view examples: 2
- **review_ui**: `stable`
  - Owner: Review-queue owner
  - Scope: The review queue reuses the shared list selection and scope grammar for queued items, keeping blocked state and local actions keyboard-discoverable and degrading when a loaded subset is shown as the exact total or a blocked row hides its state behind hover
  - Tree-view examples: 2 / list-view examples: 4
- **data_ui**: `stable`
  - Owner: Provider / request-data owner
  - Scope: The provider surface reuses the shared tree and list semantics for request/data collections, keeping loaded subsets honest and degrading when the current selection is hover-only or a stale backend is presented as complete
  - Tree-view examples: 2 / list-view examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved tree and list truth, so a faked-complete tree, an unresolved count scope, hover-only local actions, or an overclaimed continuity is visible in evidence rather than hidden behind compact chrome
  - Tree-view examples: 3 / list-view examples: 2
- **product_ui**: `stable`
  - Owner: In-product collection owner
  - Scope: In-product surfaces reuse the same disclosure, selection, and scope grammar a user sees in the explorer and review queue, always offering the command-backed scope detail and degrading honestly when the trace path is missing
  - Tree-view examples: 2 / list-view examples: 3
