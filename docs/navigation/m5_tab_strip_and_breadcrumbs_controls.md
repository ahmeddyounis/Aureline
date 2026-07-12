# M5 tab-strip and breadcrumbs controls

This is the first **implement lane** over the frozen
[M5 navigation / content component matrix](../../schemas/ui/m5-navigation-content-component-matrix.schema.json)
(see the [component contract](m5_navigation_content_components_contract.md)). It turns the two
active-context / local-structure components — the **tab strip** and the **breadcrumb trail** — into
resolvers that produce export-safe, honest projections across the claimed M5 shell, explorer, search,
review, request/data, help, and support surfaces.

- Rust source: `crates/aureline-shell/src/implement_the_m5_tab_strip_and_breadcrumbs_active_context_item_state_hierarchy_path_source_aware_context_and_no_top_level_navigation_drift_primitive/`
- Combined schema: [`schemas/ui/m5-tab-strip-breadcrumbs-controls.schema.json`](../../schemas/ui/m5-tab-strip-breadcrumbs-controls.schema.json)
- Per-component schemas: [`m5-tab-strip.schema.json`](../../schemas/ui/m5-tab-strip.schema.json),
  [`m5-breadcrumbs.schema.json`](../../schemas/ui/m5-breadcrumbs.schema.json)
- Proof packet: `artifacts/release/m5-tab-strip-breadcrumbs-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-tab-strip-breadcrumbs-controls/`

The Rust validator in `crates/aureline-shell` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_tab_strip`

A tab strip reads as a clean, context-legible state only when it names:

- the **active-context state** (which context is current versus merely open), never unstated or
  unresolved;
- the **per-tab item state** — pinned, preview, modified, read-only, blocked, shared, or reopened —
  stated with **no-color-only** semantics (a name or icon-with-label, never color alone);
- the **local-action budget** for the strip.

It degrades — never masquerading as a clean pass — when the strip reads as **top-level workflow
navigation**, invents a **surface-local badge** for a context the shared grammar already names, hides
a **blocked tab behind an ambiguous ellipsis**, leaves the item state color-only or unresolved, or
offers no command-backed path to trace the active context.

### `resolve_breadcrumbs`

A breadcrumb trail reads as clean only when it names:

- the **leaf / current-object identity**;
- the **ancestry kind** — file path, symbol ancestry, logical root, search scope, or mixed — so a
  symbol ancestry is never presented with the same weight as a filesystem path;
- the **hierarchy / path state**, staying **explicit in compact, expanded, and exported views**.

It degrades when the trail reads as **top-level navigation**, **collapses missing scope into an
ambiguous ellipsis**, presents a **partial or stale hierarchy as a complete path**, is not explicit
across views, or offers no command-backed path to trace the ancestry. A partial hierarchy shown
**honestly** (never claimed complete) stays clean.

## Hard invariants (per controls row, must be `false`)

- `tabs_masquerade_as_top_level_workflow_navigation`
- `breadcrumbs_masquerade_as_top_level_workflow_navigation`
- `invents_surface_local_badges_for_shared_context`
- `collapses_missing_scope_or_hides_blocked_behind_ellipsis`

## Acceptance criteria (proven by resolved examples)

1. **Tab state grammar** — tabs across claimed M5 panes show the same state grammar (clean examples
   cover more than one shared item state) and do not invent surface-local badges; a masquerade and a
   badge-invention example both degrade.
2. **Breadcrumb explicitness** — breadcrumb paths remain explicit in compact, expanded, and exported
   views; an ellipsis-collapse and a partial/stale-shown-complete example both degrade, and no clean
   breadcrumb collapses missing scope or shows partial/stale as complete.
3. **Context and ancestry traceability** — a user can trace current context and local ancestry
   through one canonical component contract and command-backed detail entrypoints (at least one clean
   tab and one clean breadcrumb expose a command-backed detail path).

## Regenerating the proof artifacts

```text
cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- support-export
cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- csv
cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- report
cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- fixture-shell-ui-beta-narrowed
cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- fixture-search-ui-preview-narrowed
```
