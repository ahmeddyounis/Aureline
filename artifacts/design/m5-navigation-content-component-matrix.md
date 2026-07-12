# M5 Tab-Strip, Breadcrumbs, Tree-View, List-View, Table/Grid, and Panel-Header Component Matrix

- Packet: `m5-navigation-content-components:stable:0001`
- Label: `M5 tab-strip, breadcrumbs, tree-view, list-view, table/grid, and panel-header component matrix`
- Component families: 6 (6 stable)
- Navigation / content dispositions: preview, pinned, modified, read_only, blocked, exact_count, loaded_count, all_matching_count, hidden_by_filter, hidden_by_policy, overflowed_local_action, stale_or_partial_hierarchy
- Count scopes: exact_count, loaded_count, all_matching_count, hidden_by_filter_count, hidden_by_policy_count, count_unresolved
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Component families

- **tab_strip**: `stable`
  - Owner: Shell navigation owner
  - Canonical schema: `schemas/ui/m5-tab-strip.schema.json`
  - Scope: One tab-strip model naming the active context (current, pinned, preview, or background), the per-tab item state (pinned, preview, modified, read-only, blocked), and the overflow budget, so a background or preview tab never reads as the active context and a tab set never masquerades as top-level workflow navigation
  - Required labels: identity, state, keyboard_route, active_context_and_hierarchy, selection_and_item_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, reduced_motion_safe, cli_exportable, support_packet_present
- **breadcrumbs**: `stable`
  - Owner: Explorer navigation owner
  - Canonical schema: `schemas/ui/m5-breadcrumbs.schema.json`
  - Scope: One breadcrumbs model naming the hierarchy / path to the current object — full path, root-relative, truncated-middle, stale, or partial — so a truncated, stale, or partial hierarchy is never presented as a complete path
  - Required labels: identity, state, keyboard_route, active_context_and_hierarchy
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, reduced_motion_safe, cli_exportable, support_packet_present
- **tree_view**: `stable`
  - Owner: Explorer navigation owner
  - Canonical schema: `schemas/ui/m5-tree-view.schema.json`
  - Scope: One tree-view model naming hierarchy, disclosure state (expanded, collapsed, partially expanded, leaf, or lazily unloaded), selection versus the current item, item state, exact / loaded / all-matching / hidden counts, density, and a bounded local-action budget, so a lazily-unloaded subtree is never presented as an empty leaf and tree actions are never hover-only
  - Required labels: identity, state, keyboard_route, active_context_and_hierarchy, count_and_scope, selection_and_item_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, reduced_motion_safe, cli_exportable, support_packet_present
- **list_view**: `stable`
  - Owner: Collection surface owner
  - Canonical schema: `schemas/ui/m5-list-view.schema.json`
  - Scope: One list-view model naming selection versus the current item, per-row item state, the exact / loaded / all-matching / hidden-by-filter / hidden-by-policy counts, density, and a bounded local-action budget, so exact, loaded, and all-matching scopes never collapse into one vague total and blocked rows never hide behind an ambiguous ellipsis
  - Required labels: identity, state, keyboard_route, count_and_scope, selection_and_item_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, reduced_motion_safe, cli_exportable, support_packet_present
- **table_grid**: `stable`
  - Owner: Data surface owner
  - Canonical schema: `schemas/ui/m5-table-grid.schema.json`
  - Scope: One table/grid model naming selection, the exact / loaded / all-matching / hidden-by-policy counts, every density variant, per-cell item state, and a bounded local-action budget across a dense structure, so a condensed or overflowed layout is never mistaken for a complete comfortable one and counts stay scoped
  - Required labels: identity, state, keyboard_route, count_and_scope, selection_and_item_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, reduced_motion_safe, cli_exportable, support_packet_present
- **panel_header**: `stable`
  - Owner: Shell navigation owner
  - Canonical schema: `schemas/ui/m5-panel-header.schema.json`
  - Scope: One panel-header model naming the active context and a bounded local-action budget (within budget, primary-plus-overflow, or fully overflowed), so a panel header never becomes a cluttered secondary toolbar and an overflowed action is never silently dropped
  - Required labels: identity, state, keyboard_route, active_context_and_hierarchy
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, reduced_motion_safe, cli_exportable, support_packet_present
