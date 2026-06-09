# Notebook-aware search, outline, breadcrumbs, and cell-target navigation

## Purpose

This artifact describes the M05-018 notebook-aware search, outline, breadcrumb, and cell-target navigation data model that keeps notebook navigation honest about cell identity, scope, and degraded state.

## Principles

- Search, outline, breadcrumbs, and cell-target navigation must work without a selected kernel.
- All navigation anchors use stable cell IDs from the canonical document model so they survive reorder, diff, review, and comment anchoring.
- Search scope must be explicit; the chrome must never present a scoped search as a full-document search.
- Outline items anchor to durable cell IDs; headings are extracted from markdown cell content, cell boundaries are structural.
- Breadcrumb segments carry class, label, and target ref so the user always knows where they are in the notebook hierarchy.
- Cell-target navigation must not silently fall back to a different cell when the exact target is unavailable.
- Raw notebook JSON bodies, raw cell source bytes, raw output bytes, and raw URLs MUST NOT appear on any record.

## Model overview

### NotebookSearchQuery

The search query record carries:

- `search_query_id` — stable opaque search identity.
- `document_id_ref` — opaque notebook-document id.
- `search_scope_class` — where the search operates (`current_cell`, `all_cells`, `selected_cells`, `cell_outputs_only`, `markdown_cells_only`, `code_cells_only`).
- `query_label` — export-safe label (raw query text never appears inline).
- `match_class` — `exact`, `fuzzy`, `regex`, or `semantic`.
- `result_cell_id_refs` — opaque refs to matched cells.
- `result_count_visible` / `result_count_total` — truncation state.
- `kernel_required_for_match_class` — whether the match class needs a kernel.
- `degraded_no_kernel` — whether the query is degraded because no kernel is available.

### NotebookOutlineItem

The outline item record carries:

- `outline_item_id` — stable opaque outline identity.
- `item_class` — heading level (1–6) or cell boundary (`markdown_cell_boundary`, `code_cell_boundary`, `raw_cell_boundary`).
- `cell_id_ref` — opaque ref to the anchored cell.
- `heading_level` — 1–6 when item_class is a heading; null otherwise.
- `title_label` — export-safe display label.
- `child_item_refs` — opaque refs to child outline items.
- `collapsed` — whether the item is collapsed in the outline view.

### NotebookBreadcrumb

The breadcrumb segment record carries:

- `breadcrumb_id` — stable opaque breadcrumb identity.
- `segment_index` — position in the trail (0 = root).
- `breadcrumb_class` — `document_root`, `section_heading`, `cell_boundary`, `output_boundary`, or `search_result_set`.
- `label` — export-safe display label.
- `target_ref` — opaque ref to the navigation target.
- `active` — whether this segment is the current location.

### NotebookCellTarget

The cell-target navigation record carries:

- `cell_target_id` — stable opaque target identity.
- `target_class` — `cell_id_anchor`, `cell_index_anchor`, `output_index_anchor`, `heading_anchor`, or `search_match_anchor`.
- `cell_id_ref` — required for `cell_id_anchor` and `output_index_anchor`.
- `cell_index` — required for `cell_index_anchor`.
- `output_index` — required for `output_index_anchor`.
- `heading_anchor_ref` — required for `heading_anchor`.
- `search_match_ref` — required for `search_match_anchor`.
- `scroll_behavior_class` — how the view scrolls (`center_in_view`, `scroll_to_top`, `scroll_to_nearest`, `no_scroll`).
- `focus_cell` — whether the cell receives focus after navigation.

## Closed vocabularies

| Vocabulary | Variants | Location |
|---|---|---|
| `NotebookSearchScopeClass` | `current_cell`, `all_cells`, `selected_cells`, `cell_outputs_only`, `markdown_cells_only`, `code_cells_only` | `aureline-notebook` crate |
| `NotebookSearchMatchClass` | `exact`, `fuzzy`, `regex`, `semantic` | `aureline-notebook` crate |
| `NotebookOutlineItemClass` | `heading_1` … `heading_6`, `markdown_cell_boundary`, `code_cell_boundary`, `raw_cell_boundary` | `aureline-notebook` crate |
| `NotebookBreadcrumbClass` | `document_root`, `section_heading`, `cell_boundary`, `output_boundary`, `search_result_set` | `aureline-notebook` crate |
| `NotebookCellTargetClass` | `cell_id_anchor`, `cell_index_anchor`, `output_index_anchor`, `heading_anchor`, `search_match_anchor` | `aureline-notebook` crate |
| `NotebookScrollBehaviorClass` | `center_in_view`, `scroll_to_top`, `scroll_to_nearest`, `no_scroll` | `aureline-notebook` crate |

## Schema

The boundary schema lives at:

```
/schemas/notebook/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation.schema.json
```

## Checked-in artifact

The typed search/outline/navigation packet is checked in at:

```
artifacts/notebook/m5/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation.json
```

It is embedded in the `aureline-notebook` crate via `include_str!` so consumers and CI agree on every row without a cargo build in CI.

## Fixtures

Worked fixture cases live under:

```
fixtures/notebook/m5/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation/
```

Cases cover:
- `all_cells_search` — full-document fuzzy search with results.
- `semantic_search_degraded` — semantic search degraded because no kernel is available.
- `outline_headings_and_boundaries` — outline with headings and cell boundaries.
- `breadcrumb_trail` — breadcrumb trail from document root to active cell.
- `cell_targets` — navigation targets for cell id, index, output, heading, and search match.

## Integration

The `aureline-notebook` crate exposes:

- `NotebookSearchQuery`, `NotebookOutlineItem`, `NotebookBreadcrumb`, `NotebookCellTarget`
- `NotebookSearchOutlineNavigationPacket` and `current_notebook_search_outline_navigation_packet()`
- Validation methods on every record that return typed `SearchOutlineNavigationFinding` findings
- Closed vocabularies with `as_str()` tokens and `ALL` arrays

## Risks and downgrade behavior

- If the embedded JSON artifact is missing or malformed, `current_notebook_search_outline_navigation_packet()` returns an error and CI must treat the lane as underqualified.
- If a fixture case fails validation, the corpus is incomplete and promotion should narrow the claim.
- Semantic search may require a kernel or local model; when unavailable the chrome must show a degraded label instead of silently falling back to fuzzy search.
- Cell-target navigation must not silently fall back to a different cell; if the target is missing, the chrome must show an explicit error state.
- Search, outline, breadcrumbs, and navigation must remain useful without a selected kernel.
