# M5 tree-view and list-view controls

This is the **dense-collection implement lane** over the frozen
[M5 navigation / content component matrix](../../schemas/ui/m5-navigation-content-component-matrix.schema.json)
(see the [component contract](m5_navigation_content_components_contract.md)). It turns the two
hierarchical / queue-like collection components — the **tree view** and the **list view** — into
resolvers that produce export-safe, honest projections across the claimed M5 explorer, search,
review-queue, provider, help, and support surfaces.

- Rust source: `crates/aureline-shell/src/implement_the_m5_tree_view_and_list_view_virtualization_disclosure_selection_focus_inline_action_budget_and_exact_loaded_hidden_scope_primitive/`
- Combined schema: [`schemas/ui/m5-tree-view-list-view-controls.schema.json`](../../schemas/ui/m5-tree-view-list-view-controls.schema.json)
- Per-component schemas: [`m5-tree-view.schema.json`](../../schemas/ui/m5-tree-view.schema.json),
  [`m5-list-view.schema.json`](../../schemas/ui/m5-list-view.schema.json)
- Proof packet: `artifacts/release/m5-tree-view-list-view-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-tree-view-list-view-controls/`

The Rust validator in `crates/aureline-shell` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_tree_view`

A tree view reads as a clean, structure-legible state only when it names:

- the **disclosure state** — expanded, collapsed, partially expanded, a leaf, or a lazily-unloaded
  subtree — with **virtualization honest**, never drawing a lazy subtree as an empty leaf;
- the **selection-versus-current** distinction and a visible **row focus**, never hover-only;
- the **per-row item state**, with a blocked row's state never discoverable only by pointer hover;
- the **count scope** — exact, loaded, all-matching, hidden by filter, hidden by policy, or
  outside the current scope — with the exact / loaded / all-matching scopes never collapsed;
- the **density variant**, the **local-action budget** (never hover-only), the **drag / reorder
  posture** where allowed, and the **cross-window / cross-pane continuity** posture.

It degrades — never masquerading as a clean pass — when the node identity is unstated, disclosure is
unresolved, a lazy subtree reads as an empty leaf, selection collapses into the current item, row
focus is not visible, the current selection / a blocked row / the local actions are hover-only, a
count scope collapses or cannot be resolved, a stale or partial backend is presented as complete,
drag / reorder or cross-surface continuity is overclaimed, or no command-backed scope path is
reachable. A partial or lazily-unloaded backend shown **honestly** (never claimed complete) stays
clean.

### `resolve_list_view`

A list view carries the same row semantics minus disclosure, plus one extra virtualization truth: a
loaded / virtualized subset is **never presented as the exact total**. It degrades on the same
grammar as the tree view, and additionally when a loaded subset reads as the exact count.

## Hard invariants (per controls row, must be `false`)

- `hides_current_selection_blocked_or_actions_behind_hover_only`
- `collapses_selection_versus_current_or_count_scopes`
- `presents_stale_partial_or_lazy_collection_as_complete`
- `overclaims_drag_reorder_or_cross_surface_continuity`

## Acceptance criteria (proven by resolved examples)

1. **Shared row semantics** — explorer, search, review-queue, provider, and support-facing tree /
   list consumers reuse the same row semantics and scope vocabulary (clean tree and list examples
   together cover more than one shared count scope); a count-scope collapse degrades on both sides,
   and no clean example collapses scopes.
2. **Selection and disclosure truth** — deep nesting, compact layouts, and stale or partial backends
   preserve selection and disclosure truth rather than faking a complete tree; a partial backend
   shown honestly stays clean, while a lazy-shown-as-leaf and a stale-shown-complete example both
   degrade.
3. **No hover-only discovery** — no claimed M5 tree / list surface requires pointer hover to discover
   the current selection, blocked state, or available local actions; each hover-only case degrades,
   and no clean example hides one behind hover.

## Regenerating the proof artifacts

```text
cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- support-export
cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- csv
cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- report
cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- fixture-explorer-ui-beta-narrowed
cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- fixture-review-ui-preview-narrowed
```
