# M5 table / grid and panel-header controls

This is the **dense-structure implement lane** over the frozen
[M5 navigation / content component matrix](../../schemas/ui/m5-navigation-content-component-matrix.schema.json)
(see the [component contract](m5_navigation_content_components_contract.md)). It turns the two
dense-collection / header components — the **table / grid** and the **panel header** — into resolvers
that produce export-safe, honest projections across the claimed M5 request/data, review, search,
governance, and support surfaces.

- Rust source: `crates/aureline-shell/src/implement_the_m5_table_grid_and_panel_header_sort_filter_provenance_selection_bar_pinned_column_identity_value_qualification_and_count_scope_primitive/`
- Combined schema: [`schemas/ui/m5-table-grid-panel-header-controls.schema.json`](../../schemas/ui/m5-table-grid-panel-header-controls.schema.json)
- Per-component schemas: [`m5-table-grid.schema.json`](../../schemas/ui/m5-table-grid.schema.json),
  [`m5-panel-header.schema.json`](../../schemas/ui/m5-panel-header.schema.json)
- Proof packet: `artifacts/release/m5-table-grid-panel-header-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-table-grid-panel-header-controls/`

The Rust validator in `crates/aureline-shell` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_table_grid`

A table / grid reads as a clean, structure-legible state only when it names:

- the **selection-versus-current** distinction and a visible **row focus**, never hover-only;
- the **per-row item state**, with a blocked row's state never discoverable only by pointer hover;
- the **sort / filter provenance** — user-sorted, default sort, relevance-ranked, imported order, a
  filter-applied subset, or explicitly unsorted — never left unstated;
- the **pinned-column identity** — an identity / leading / trailing column that stays anchored under
  virtualization and column overflow, never scrolled off and lost;
- the **per-value qualification** — exact canonical, estimated, imported, stale, partial, or
  policy-limited — never presenting a qualified value as exact canonical truth;
- the **count scope** — exact, loaded, all-matching, hidden by filter, hidden by policy, or outside
  the current scope — with the exact / loaded / all-matching scopes never collapsed and a loaded
  subset never presented as the exact total;
- the **density variant** and the **local-action budget** (never hover-only).

It degrades — never masquerading as a clean pass — when the grid identity is unstated, selection
collapses into the current row, row focus is not visible, the current selection / a blocked row / the
local actions are hover-only, the sort / filter provenance is unstated, a pinned column loses its
identity or cannot be resolved, a qualified value reads as canonical or the qualification cannot be
resolved, a count scope collapses or cannot be resolved, a loaded subset reads as the exact count, a
stale or partial backend is presented as complete, or no command-backed scope path is reachable. A
partially-loaded backend shown **honestly** (never claimed complete) stays clean.

### `resolve_panel_header`

A panel header reads as clean only when it names its **identity** and **active context** (never
presenting a background / preview context as the active one), keeps a **bounded local-action budget**
(never hover-only, never overloading into a cluttered secondary toolbar, never silently dropping an
overflowed action), and points back to one **canonical count / selection model** instead of
re-encoding counts in surface-local copy.

## Hard invariants (per controls row, must be `false`)

- `hides_current_selection_blocked_or_actions_behind_hover_only`
- `collapses_selection_versus_current_or_count_scopes`
- `presents_qualified_stale_or_partial_grid_as_canonical`
- `panel_header_overloads_or_re_encodes_counts`

## Acceptance criteria (proven by resolved examples)

1. **Shared sort / filter and count semantics** — request/data, review, governance, and support grid
   consumers reuse the same sort / filter and count semantics (clean grid examples cover more than
   one shared count scope and more than one provenance); a count-scope collapse and a
   provenance-unstated case both degrade, and no clean example collapses scopes or hides provenance.
2. **Pinned and identity column stability** — pinned and identity columns stay stable under
   virtualization and column overflow without losing provenance or scope truth; at least one clean
   grid pins an identity column while honestly qualifying its values, and a pinned-column-lost and a
   qualified-value-shown-as-canonical example both degrade.
3. **One canonical header and selection model** — grid / export consumers point back to one canonical
   header and selection model instead of re-encoding counts in surface-local copy; a re-encode and a
   toolbar-overload case both degrade, and no clean header re-encodes counts or overloads.

## Regenerating the proof artifacts

```text
cargo run -p aureline-shell --example dump_m5_table_grid_panel_header_controls -- support-export
cargo run -p aureline-shell --example dump_m5_table_grid_panel_header_controls -- csv
cargo run -p aureline-shell --example dump_m5_table_grid_panel_header_controls -- report
cargo run -p aureline-shell --example dump_m5_table_grid_panel_header_controls -- fixture-data-ui-beta-narrowed
cargo run -p aureline-shell --example dump_m5_table_grid_panel_header_controls -- fixture-review-ui-preview-narrowed
```
