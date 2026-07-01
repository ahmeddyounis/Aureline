# M5 window lifecycle safety: cross-window drag verbs, close-orphan prevention, and safe specialized-window reopen

Generated from the seeded packet in
[`crate::m5_window_lifecycle_safety`](../../crates/aureline-shell/src/m5_window_lifecycle_safety/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- markdown > \
  artifacts/shell/m5-window-lifecycle-safety.md
```

- Packet id: `m5-window-lifecycle-safety:stable:0001`
- Source schema ref: `schemas/shell/m5-window-lifecycle-safety.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required drag verbs: `move_tab`, `copy_editor`, `open_compare_here`, `create_window`
- Required protected resources: `dirty_buffer`, `live_approval`, `collaboration_control`, `evidence_review`
- Rows certified: 10
- Green (full lifecycle): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Lifecycle rows

| Surface | Status | Qualification | Drag verb disclosure | Close-orphan guard | Safe reopen | Waiver |
| ------- | ------ | ------------- | -------------------- | ------------------ | ----------- | ------ |
| Notebook editor / cell surface | `green` | `stable` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Tabular data grid surface | `green` | `stable` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Profiler / performance surface | `green` | `stable` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Pipeline / workflow graph surface | `green` | `stable` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Documentation reader surface | `yellow` | `stable` | `disclosed_verb_reach_narrowing` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Preview surface (render, diff, media) | `green` | `stable` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Review / change-request surface | `green` | `stable` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `reopens_safest_equivalent_layout` | — |
| Incident / operations-response surface | `yellow` | `beta` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `disclosed_reduced_equivalent_fallback` | — |
| Companion assistant surface | `yellow` | `beta` | `verb_disclosed_with_keyboard_parity` | `disclosed_deferred_guard_relocation` | `reopens_safest_equivalent_layout` | `waiver:companion-close-guard-relocation:0001` |
| Operator / control-plane surface | `yellow` | `beta` | `verb_disclosed_with_keyboard_parity` | `close_guarded_no_orphan` | `disclosed_reduced_equivalent_fallback` | — |

## Per-verb cross-window drag plan

| Surface | Drag verb | Disclosed before drop | Keyboard equivalent |
| ------- | --------- | --------------------- | ------------------- |
| Notebook editor / cell surface | `move_tab` | `true` | `true` |
| Notebook editor / cell surface | `copy_editor` | `true` | `true` |
| Notebook editor / cell surface | `open_compare_here` | `true` | `true` |
| Notebook editor / cell surface | `create_window` | `true` | `true` |
| Tabular data grid surface | `move_tab` | `true` | `true` |
| Tabular data grid surface | `copy_editor` | `true` | `true` |
| Tabular data grid surface | `open_compare_here` | `true` | `true` |
| Tabular data grid surface | `create_window` | `true` | `true` |
| Profiler / performance surface | `move_tab` | `true` | `true` |
| Profiler / performance surface | `copy_editor` | `true` | `true` |
| Profiler / performance surface | `open_compare_here` | `true` | `true` |
| Profiler / performance surface | `create_window` | `true` | `true` |
| Pipeline / workflow graph surface | `move_tab` | `true` | `true` |
| Pipeline / workflow graph surface | `copy_editor` | `true` | `true` |
| Pipeline / workflow graph surface | `open_compare_here` | `true` | `true` |
| Pipeline / workflow graph surface | `create_window` | `true` | `true` |
| Documentation reader surface | `move_tab` | `true` | `true` |
| Documentation reader surface | `copy_editor` | `true` | `true` |
| Documentation reader surface | `open_compare_here` | `true` | `true` |
| Documentation reader surface | `create_window` | `true` | `true` |
| Preview surface (render, diff, media) | `move_tab` | `true` | `true` |
| Preview surface (render, diff, media) | `copy_editor` | `true` | `true` |
| Preview surface (render, diff, media) | `open_compare_here` | `true` | `true` |
| Preview surface (render, diff, media) | `create_window` | `true` | `true` |
| Review / change-request surface | `move_tab` | `true` | `true` |
| Review / change-request surface | `copy_editor` | `true` | `true` |
| Review / change-request surface | `open_compare_here` | `true` | `true` |
| Review / change-request surface | `create_window` | `true` | `true` |
| Incident / operations-response surface | `move_tab` | `true` | `true` |
| Incident / operations-response surface | `copy_editor` | `true` | `true` |
| Incident / operations-response surface | `open_compare_here` | `true` | `true` |
| Incident / operations-response surface | `create_window` | `true` | `true` |
| Companion assistant surface | `move_tab` | `true` | `true` |
| Companion assistant surface | `copy_editor` | `true` | `true` |
| Companion assistant surface | `open_compare_here` | `true` | `true` |
| Companion assistant surface | `create_window` | `true` | `true` |
| Operator / control-plane surface | `move_tab` | `true` | `true` |
| Operator / control-plane surface | `copy_editor` | `true` | `true` |
| Operator / control-plane surface | `open_compare_here` | `true` | `true` |
| Operator / control-plane surface | `create_window` | `true` | `true` |

## Auto-narrowed rows

- `docs` (`yellow`) — A docs cross-window drag verb is advertised before the drop but is reachable only through a disclosed command-palette equivalent rather than an inline pre-drop hint; keyboard parity is preserved, so the row is narrowed below green while every drag verb still advertises its resulting action.
- `incident` (`yellow`) — The incident surface is qualified at Beta; when its live war-room feature pack is unavailable after a crash or restore it reopens onto a disclosed reduced but still-safe equivalent layout that preserves the incident identity and reopen path, so the claim is narrowed and disclosed.
- `companion` (`yellow`) — The companion surface is qualified at Beta; when a secondary companion window is closed while it still holds a live approval, the approval is deferred to a disclosed, waivered relocation into the primary workspace window with a still-visible prompt rather than being silently stranded.
- `operator` (`yellow`) — The operator surface is qualified at Beta; when a control-plane extension or remote target is unavailable after a crash or restore it reopens onto a disclosed reduced but still-safe equivalent layout that preserves the control-plane identity and reopen path, so the claim is narrowed and disclosed.

## Exact lifecycle causes

- `docs` — `upstream_dependency_narrowed` (disclosed: `true`) — A cross-window drag verb is still advertised before the drop but is reachable only through a disclosed command-palette equivalent rather than an inline pre-drop hint; keyboard parity is preserved.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — A specialized window reopens onto a disclosed reduced but still-safe equivalent layout because an extension, remote target, or feature pack is unavailable, while preserving the object identity and reopen path.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Closing a secondary window defers a protected resource to a disclosed, waivered relocation into the primary workspace window with a still-visible prompt rather than blocking outright, so nothing is silently orphaned.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — A specialized window reopens onto a disclosed reduced but still-safe equivalent layout because an extension, remote target, or feature pack is unavailable, while preserving the object identity and reopen path.

## Active waivers

- `waiver:companion-close-guard-relocation:0001` (`companion`, owner: Companion surface owner, expires `2026-09-30T00:00:00Z`) — When a secondary companion window is closed while it still holds a live approval, the approval is relocated to a disclosed, still-visible prompt in the primary workspace window rather than being silently stranded; the relocation is disclosed, never silent, and the shared close-guard contract is unified in the next sync.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- validate
cargo test -p aureline-shell --test m5_window_lifecycle_safety_fixtures
```
