# M5 responsive collapse: compact / standard / expanded parity

Generated from the seeded packet in
[`crate::m5_responsive_collapse`](../../crates/aureline-shell/src/m5_responsive_collapse/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- markdown > \
  artifacts/shell/m5-responsive-collapse.md
```

- Packet id: `m5-responsive-collapse:stable:0001`
- Source schema ref: `schemas/shell/m5-responsive-collapse.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 10
- Green (identity-stable): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Collapse rows

| Surface | Status | Qualification | Collapse ladder | Ladder | Identity | Action reach | Zoom/contrast | Waiver |
| ------- | ------ | ------------- | --------------- | ------ | -------- | ------------ | ------------- | ------ |
| Notebook editor / cell surface | `green` | `stable` | `docked|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Tabular data grid surface | `green` | `stable` | `docked|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Profiler / performance surface | `yellow` | `stable` | `docked|sheet|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `disclosed_overflow_reach` | `routes_stable_at_zoom_and_contrast` | — |
| Pipeline / workflow graph surface | `green` | `stable` | `docked|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Documentation reader surface | `green` | `stable` | `docked|sheet|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Preview surface (render, diff, media) | `green` | `stable` | `docked|sheet|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Review / change-request surface | `green` | `stable` | `docked|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Incident / operations-response surface | `yellow` | `beta` | `docked|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | — |
| Companion assistant surface | `yellow` | `beta` | `sheet|overflow|placeholder` | `identity_stable_ladder` | `disclosed_state_rehydration` | `all_critical_and_actions_reachable` | `routes_stable_at_zoom_and_contrast` | `waiver:companion-sheet-rehydration:0001` |
| Operator / control-plane surface | `yellow` | `beta` | `docked|sheet|overflow|placeholder` | `identity_stable_ladder` | `identity_and_state_preserved` | `all_critical_and_actions_reachable` | `disclosed_zoom_narrowing` | — |

## Per-class presentation

| Surface | Compact | Standard | Expanded |
| ------- | ------- | -------- | -------- |
| Notebook editor / cell surface | `overflow` | `docked` | `docked` |
| Tabular data grid surface | `overflow` | `docked` | `docked` |
| Profiler / performance surface | `sheet` | `docked` | `docked` |
| Pipeline / workflow graph surface | `overflow` | `docked` | `docked` |
| Documentation reader surface | `sheet` | `docked` | `docked` |
| Preview surface (render, diff, media) | `sheet` | `docked` | `docked` |
| Review / change-request surface | `overflow` | `docked` | `docked` |
| Incident / operations-response surface | `overflow` | `docked` | `docked` |
| Companion assistant surface | `overflow` | `sheet` | `sheet` |
| Operator / control-plane surface | `sheet` | `docked` | `docked` |

## Auto-narrowed rows

- `profiler` (`yellow`) — Under compact width the profiler moves one low-frequency capture tool to a disclosed keyboard-reachable overflow before the primary capture readout is starved; the row is narrowed below green while critical state stays visible.
- `incident` (`yellow`) — The incident surface is qualified at Beta in the frozen shell-zone matrix; its compact/standard/expanded presentation is identity-stable but the claim is narrowed below Stable and disclosed.
- `companion` (`yellow`) — The companion surface is qualified at Beta; under compact width its docked-to-sheet-to-overflow transition rehydrates in-progress prompt state through a disclosed, waivered restore path while preserving the object identity.
- `operator` (`yellow`) — The operator surface is qualified at Beta; at 400% zoom its bottom-panel controls disclose a stacked, narrowed presentation while exposing the same routes and task state, so the claim is narrowed and disclosed.

## Exact collapse causes

- `profiler` — `upstream_dependency_narrowed` (disclosed: `true`) — A low-frequency action moved to a disclosed keyboard-reachable overflow or drawer before primary navigation was starved.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — The docked-to-sheet transition rehydrates task state through a disclosed, waivered path while preserving the object identity.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — The 400% zoom / high-contrast layout discloses a narrowed presentation while exposing the same routes and task state.

## Active waivers

- `waiver:companion-sheet-rehydration:0001` (`companion`, owner: Companion surface owner, expires `2026-09-30T00:00:00Z`) — Under compact width the companion surface moves from its right-inspector sheet to a keyboard-reachable overflow and rehydrates its in-progress prompt state through a disclosed restore path while the state-serialization contract is unified in the next sync. The object identity is preserved and the rehydration is disclosed, never silent.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- validate
cargo test -p aureline-shell --test m5_responsive_collapse_fixtures
```
