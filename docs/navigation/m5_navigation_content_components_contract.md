# M5 navigation-content component matrix contract

This document is the human-readable companion to the frozen **M5 tab-strip, breadcrumbs, tree-view,
list-view, table/grid, and panel-header component matrix**.

The authoritative source of truth is the Rust validator and seed builder in
`crates/aureline-shell/src/freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix/`.
The checked-in support export, matrix CSV, design report, and narrowed fixtures are minted from that
seed builder by the `dump_m5_navigation_content_component_matrix` example; the schemas under
`schemas/ui/` document the shape and the JSON Schemas are meta-valid Draft 2020-12.

## What this freezes

Every claimed M5 surface that still ships its own tab set, breadcrumb trail, explorer tree, result
list, dense table/grid, or panel header is named once here and bound to one shared vocabulary, so
active context, hierarchy/path, disclosure, selection-versus-current, item state, counts, density,
and local-action truth stop drifting across claimed M5 shell, explorer, search, review, request/data,
help, and support surfaces.

### Governed component families

| Component family | Canonical schema |
| --- | --- |
| `tab_strip` | `schemas/ui/m5-tab-strip.schema.json` |
| `breadcrumbs` | `schemas/ui/m5-breadcrumbs.schema.json` |
| `tree_view` | `schemas/ui/m5-tree-view.schema.json` |
| `list_view` | `schemas/ui/m5-list-view.schema.json` |
| `table_grid` | `schemas/ui/m5-table-grid.schema.json` |
| `panel_header` | `schemas/ui/m5-panel-header.schema.json` |

## The one controlled disposition vocabulary

Every consumer binds to one navigation/content-disposition vocabulary and no surface invents a
parallel word for any of these:

`preview`, `pinned`, `modified`, `read_only`, `blocked`, `exact_count`, `loaded_count`,
`all_matching_count`, `hidden_by_filter`, `hidden_by_policy`, `overflowed_local_action`,
`stale_or_partial_hierarchy`.

## Family-specific controlled vocabularies

Each family declares only the vocabularies applicable to it:

- **Active-context state** — `active_current`, `active_pinned`, `active_preview`, `background_open`,
  `background_modified`, `context_unresolved` (tab strip, panel header).
- **Hierarchy / path state** — `full_path_shown`, `root_relative`, `truncated_middle`,
  `stale_hierarchy`, `partial_hierarchy`, `path_unresolved` (breadcrumbs, tree view).
- **Disclosure state** — `expanded`, `collapsed`, `partially_expanded`, `leaf_no_children`,
  `lazy_unloaded`, `disclosure_unknown` (tree view).
- **Selection state** — `single_selected`, `multi_selected`, `current_not_selected`,
  `selected_and_current`, `none_selected`, `selection_unknown` (tree view, list view, table/grid).
- **Count scope** — `exact_count`, `loaded_count`, `all_matching_count`, `hidden_by_filter_count`,
  `hidden_by_policy_count`, `count_unresolved` (tree view, list view, table/grid).
- **Item-state flag** — `pinned`, `preview`, `modified`, `read_only`, `blocked`, `state_unknown`
  (tab strip, tree view, list view, table/grid).
- **Density variant** — `comfortable`, `compact`, `dense`, `condensed_overflow`, `single_line`,
  `density_unknown` (tree view, list view, table/grid).
- **Local-action budget** — `no_local_actions`, `within_budget`, `primary_plus_overflow`,
  `overflowed_menu`, `all_overflowed`, `budget_unknown` (tab strip, tree view, list view,
  table/grid, panel header).

## Hard invariants

Every component row asserts (all `false`):

1. `tabs_masquerade_as_top_level_workflow_navigation`
2. `hides_counts_or_blocked_rows_behind_ambiguous_ellipsis`
3. `makes_tree_list_or_table_actions_hover_only`
4. `panel_header_becomes_cluttered_secondary_toolbar`
5. `collapses_exact_loaded_and_all_matching_scopes_into_one_total`

## Non-visual / CLI / export requirements

Every component declares a non-visual accessibility route set (keyboard-focusable,
screen-reader-announced, high-zoom-reflow, reduced-motion-safe, CLI-exportable,
support-packet-present) so none of these components becomes shell-only chrome, and every component
must be present in the support / export packet.

## Acceptance-criteria mapping

- **Shared matrix** — design, schema, QA, docs, and release owners share this one matrix for the
  B132 navigation/content component family.
- **One canonical contract** — every claimed M5 consumer points at one canonical per-component schema
  (or the combined matrix schema) instead of rewording basic navigation/content state locally.
- **Agreed baseline** — future implementation rows inherit this field/state baseline with no open
  ambiguity about count, selection, hierarchy, or header-local action meaning.
