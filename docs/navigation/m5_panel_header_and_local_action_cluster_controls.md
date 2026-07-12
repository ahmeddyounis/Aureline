# M5 panel-header and local-action-cluster controls

This is the **panel-header / local-action-cluster implement lane** over the frozen
[M5 navigation / content component matrix](../../schemas/ui/m5-navigation-content-component-matrix.schema.json)
(see the [component contract](m5_navigation_content_components_contract.md)). It turns the panel
header — and the bounded local-action cluster that lives inside it — into resolvers that produce
export-safe, honest projections across the claimed M5 shell, request/data, review, search, support,
and product surfaces.

- Rust source: `crates/aureline-shell/src/implement_the_m5_panel_header_and_local_action_cluster_stable_title_overflow_rule_source_freshness_cue_and_command_backed_action_primitive/`
- Combined schema: [`schemas/ui/m5-panel-header-local-action-cluster-controls.schema.json`](../../schemas/ui/m5-panel-header-local-action-cluster-controls.schema.json)
- Per-component schema: [`m5-panel-header.schema.json`](../../schemas/ui/m5-panel-header.schema.json)
- Proof packet: `artifacts/release/m5-panel-header-local-action-cluster-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-panel-header-local-action-cluster-controls/`

The Rust validator in `crates/aureline-shell` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_panel_header`

A panel header reads as a stable, low-noise context marker only when it names:

- a **stable title slot** — never unstated, never swapped for a transient status string;
- its **active context** — active / pinned / preview / background — never presenting a background or
  preview context as the active one;
- its **source / freshness at the pane boundary** — a cached, partial, stale, remote, or
  provider-owned pane labels that truth directly at the boundary and is never presented as current,
  live, first-party, ready content;
- a reference back to one **canonical count / selection model**, never re-encoding counts in
  surface-local copy;
- **command-backed refresh** and **reveal / detail** affordances.

A current, first-party pane needs no freshness cue; a partially-loaded or cached pane shown
**honestly** (labelled at the boundary) stays clean.

### `resolve_local_action_cluster`

A local-action cluster reads as clean only when it keeps its **local-action budget** resolved, its
actions **keyboard-reachable** and never hover-only, its **advanced actions in a structured or overflow
menu** rather than persistent clutter, its **overflowed actions routed to overflow** rather than
silently dropped, and its **compaction / responsive-collapse mode** preserving the panel identity and
action semantics instead of re-instantiating a different surface. A full (non-compacted) header has
nothing compacted to preserve, so its preservation flags are moot.

## Hard invariants (per controls row, must be `false`)

- `hides_actions_behind_hover_only_or_loses_keyboard_access`
- `overstates_readiness_or_hides_source_freshness_cue`
- `overloads_header_or_keeps_advanced_actions_as_persistent_clutter`
- `compaction_reinstantiates_surface_or_loses_panel_identity`

## Acceptance criteria (proven by resolved examples)

1. **One header grammar** — claimed M5 panes show one header grammar for title, local actions,
   overflow, and freshness / source cues (clean header examples cover more than one source-freshness
   posture); a freshness-cue-missing and a readiness-overstated case both degrade, and no clean header
   hides its cue or overstates readiness.
2. **Compaction preserves identity and action semantics** — compaction and responsive collapse
   preserve the same panel identity and action semantics instead of re-instantiating a different
   surface; at least one clean cluster is actively compacted while preserving both, and a
   reinstantiate-surface and a loses-identity case both degrade.
3. **Low-noise but sufficient headers** — a clean header explains what the pane owns (references the
   canonical model, offers a refresh command) and whether it is current enough to trust (names its
   source / freshness); a re-encode header and a persistent-clutter cluster both degrade.

## Regenerating the proof artifacts

```text
cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- support-export
cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- csv
cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- report
cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- fixture-shell-ui-beta-narrowed
cargo run -p aureline-shell --example dump_m5_panel_header_local_action_cluster_controls -- fixture-support-export-preview-narrowed
```
