# M5 navigation-content component consumer contract (M05-1113)

This contract proves that the six frozen M5 navigation-content component families
— **tab strip, breadcrumbs, tree view, list view, table / grid, and panel
header** — are shared product infrastructure rather than one-surface
implementation details. It is the consumer-adoption lane over the frozen
navigation-content component matrix
(`schemas/ui/m5-navigation-content-component-matrix.schema.json`) and the four
B132 implement lanes.

- **Schema:** [`schemas/ui/m5-navigation-content-component-consumer.schema.json`](../../schemas/ui/m5-navigation-content-component-consumer.schema.json)
- **Support export (canonical):** `artifacts/release/m5-navigation-content-component-consumer-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-navigation-content-component-consumer-proof/matrix.csv`
- **Report:** `artifacts/release/m5-navigation-content-component-consumer-proof/report.md`
- **Fixtures:** `fixtures/ui/m5-navigation-content-component-consumers/`

## Consumer classes

Every claimed navigation / content consumer class adopts at least one canonical
component family:

1. **Shell / explorer** — the shell workspace and file explorer.
2. **Search / graph** — search results and the graph / AI-context surface.
3. **Review** — the review surface.
4. **Request / data** — the request / data surface.
5. **Help center** — the docs / help center (AC2 docs reference).
6. **Support / export + release packet** — the portable support export (AC2).

## Controls lanes

The six families group into the four B132 controls contracts. A consumer must
reuse the one canonical controls contract (schema + doc + release-proof artifact)
for its family's lane rather than forking a surface-local one. Panel headers adopt
the deepest dedicated panel-header + local-action-cluster contract (M05-1112).

| Controls lane | Families | Implement lane |
| --- | --- | --- |
| `tab_strip_breadcrumbs` | tab strip, breadcrumbs | M05-1109 |
| `tree_view_list_view` | tree view, list view | M05-1110 |
| `table_grid_panel_header` | table / grid | M05-1111 |
| `panel_header_local_action_cluster` | panel header | M05-1112 |

## Preserved label families

Every consumer keeps the identical controlled label families regardless of
surface, authority, or export shape. Their union must cover the whole set:

- `active_context` (tab strip / panel header primary)
- `hierarchy_path` (breadcrumbs primary)
- `disclosure_state` (tree view primary)
- `selection_versus_current` (list view primary)
- `pinned_preview_read_only`
- `count_scope` (table / grid primary; mandatory on every dense tree/list/table)
- `local_action_budget` (panel header primary)
- `overflow_freshness`

## Frozen disposition vocabulary

Every row keeps the frozen `M5NavigationContentDisposition` vocabulary visible
even when narrowed: `preview`, `pinned`, `modified`, `read_only`, `blocked`,
`exact_count`, `loaded_count`, `all_matching_count`, `hidden_by_filter`,
`hidden_by_policy`, `overflowed_local_action`, `stale_or_partial_hierarchy`.

## Guardrails (must stay false per row)

1. `tabs_masquerade_as_top_level_workflow_navigation`
2. `hides_counts_or_blocked_rows_behind_ambiguous_ellipsis`
3. `makes_tree_list_or_table_actions_hover_only`
4. `panel_header_becomes_cluttered_secondary_toolbar`
5. `collapses_exact_loaded_and_all_matching_scopes_into_one_total`

## Acceptance criteria

- **AC1** — At least the first claimed consumers in each major lane project from
  the same B132 contracts and fixtures, keeping one vocabulary for active
  context, scope counts, hidden rows, overflow, freshness, and local pane
  actions. Cross-surface scans find no drift on count terms, hierarchy labels, or
  header-local action naming.
- **AC2** — Support / export packets can map navigation / content state back to
  one shared contract family. A help-center consumer and a support/export
  consumer both reference the canonical component families rather than cloning
  local prose.

## Narrowing and reconstruction

A consumer may narrow authority (read-only, inspect-only, override-gated,
export-only, policy-blocked), but it discloses the reduction with a
reduced-capability banner (non-generic label, capability state matching the
authority mode) and, when it punts to another surface, a handoff note. Every row
carries an opaque `nav_state_ref` and its canonical controls contract so support
and automation can reconstruct the exact navigation / content state the user saw.
The packet is metadata-only.

Regenerate the checked-in artifacts and fixtures with:

```
GEN_NAV_CONTENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_nav_content_consumer_artifacts
```
