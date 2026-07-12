# M5 editor-tab and gutter controls

This is the first **implement lane** over the frozen
[M5 editor-inline component matrix](../../schemas/ui/m5-editor-inline-component-matrix.schema.json)
(see the [component contract](m5_editor_inline_components_contract.md)). It turns the two left-edge,
file/session-legibility components — the **editor tab** and the **gutter** — into resolvers that
produce export-safe, honest projections across the claimed M5 editor, diff/merge, notebook,
diagnostics, support, and product surfaces.

- Rust source: `crates/aureline-editor/src/m5_editor_tab_and_gutter_state_and_marker_layering/`
- Combined schema: [`schemas/ui/m5-editor-tab-gutter-controls.schema.json`](../../schemas/ui/m5-editor-tab-gutter-controls.schema.json)
- Per-component schemas: [`m5-editor-tab.schema.json`](../../schemas/ui/m5-editor-tab.schema.json),
  [`m5-gutter-marker.schema.json`](../../schemas/ui/m5-gutter-marker.schema.json)
- Proof packet: `artifacts/release/m5-editor-tab-gutter-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-editor-tab-gutter-controls/`

The Rust validator in `crates/aureline-editor` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_editor_tab`

An editor tab reads as a clean, legible state only when it names:

- the **file/session identity** (which document / session is open), never unstated;
- the **tab context** (current versus merely open), never unresolved;
- the **per-tab item state** — modified, preview, pinned, read-only, blocked, shared, generated, or
  remote — stated with **no-color-only** semantics (a name or icon-with-label, never color alone);
- the **code-pane kind** (single editor, split editor, diff pane, notebook code cell, or peek pane),
  never unresolved, so reopen/reveal continuity and current-versus-selected semantics stay honest
  across panes.

It degrades — never silently passes — when the identity is unstated, the tab context or item state
is unresolved, a **feature-local badge** is invented for a state the shared grammar already names, the
item state is encoded by color alone, a blocked tab is hidden behind a color / ellipsis cue,
reopen/reveal continuity is lost across panes, or no command-backed path to trace the state is
reachable.

### `resolve_gutter`

A gutter reads as a clean, readable state only when it names:

- the **anchor** (line / range identity), never unstated;
- the **marker kind** (breakpoint, added/modified/removed change marker, fold region) with
  **no-color-only** semantics;
- the **marker layer / precedence band** (diagnostic, breakpoint, change marker, blame/trust cue,
  fold affordance), so layered gutter state keeps a stable identity and precedence;
- the **diagnostic severity** behind a diagnostic-layer marker, stated non-color-only.

It degrades when the anchor is unstated, the marker kind or layer is unresolved, a feature-local badge
is invented, the marker or its severity is encoded by color alone, layer precedence is lost, the
marker layering is not readable in a compact / high-zoom / exported representation, or no
command-backed reveal / detail entrypoint is reachable.

## Hard invariants

Every controls row carries four hard invariants that must stay `false`:

- `tabs_invent_feature_local_badges_for_file_session_state`
- `gutter_markers_encode_state_by_color_alone`
- `gutter_marker_layering_loses_identity_or_precedence`
- `reopen_reveal_continuity_breaks_across_panes`

## Acceptance criteria, proven by examples

The packet's `validate()` proves each acceptance criterion against the resolved examples rather than
merely asserting a governance bool:

1. **One shared tab and gutter state grammar with no feature-local badges.** Clean tabs cover at least
   two distinct item states from the shared vocabulary, a feature-local-badge example degrades on both
   the tab and gutter side, and no clean tab or gutter invents a badge.
2. **Marker layering stays readable across representations.** At least one clean gutter preserves
   precedence and stays readable in compact / high-zoom / exported representations, a precedence-loss
   example degrades, an unreadable-layering example degrades, and no clean gutter loses precedence or
   is unreadable.
3. **State traces to one canonical contract and command-backed entrypoint.** At least one clean tab
   and one clean gutter both expose a command-backed detail / reveal entrypoint.

## Regenerating the artifacts

The headless emitter is the only mint-from-truth path:

```text
cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- support-export
cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- report
cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- csv
cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- fixture-editor-ui-beta-narrowed
cargo run -p aureline-editor --example dump_m5_editor_tab_gutter_controls -- fixture-diagnostics-ui-preview-narrowed
```
