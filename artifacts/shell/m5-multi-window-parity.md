# M5 multi-window truth parity: same identity, trust, remote, profile, and recovery in every window

Generated from the seeded packet in
[`crate::m5_multi_window_parity`](../../crates/aureline-shell/src/m5_multi_window_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity -- markdown > \
  artifacts/shell/m5-multi-window-parity.md
```

- Packet id: `m5-multi-window-parity:stable:0001`
- Source schema ref: `schemas/shell/m5-multi-window-parity.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required continuity truths: `workspace_global_trust`, `remote_target`, `deployment_profile`, `recovery_state`
- Rows certified: 10
- Green (full parity): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Parity rows

| Surface | Status | Qualification | Continuity | Layout locality | Routing | Recovery | Waiver |
| ------- | ------ | ------------- | ---------- | --------------- | ------- | -------- | ------ |
| Notebook editor / cell surface | `green` | `stable` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Tabular data grid surface | `green` | `stable` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Profiler / performance surface | `green` | `stable` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Pipeline / workflow graph surface | `green` | `stable` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Documentation reader surface | `yellow` | `stable` | `all_truths_preserved_in_every_window` | `disclosed_local_only_state` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Preview surface (render, diff, media) | `green` | `stable` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Review / change-request surface | `green` | `stable` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |
| Incident / operations-response surface | `yellow` | `beta` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `disclosed_recovery_narrowing` | — |
| Companion assistant surface | `yellow` | `beta` | `all_truths_preserved_in_every_window` | `layout_density_focus_local_risk_global` | `disclosed_routing_relocation` | `restore_dependency_topology_predictable` | `waiver:companion-routing-relocation:0001` |
| Operator / control-plane surface | `yellow` | `beta` | `disclosed_truth_projection_narrowing` | `layout_density_focus_local_risk_global` | `routes_to_owning_window_object` | `restore_dependency_topology_predictable` | — |

## Per-window continuity plan

| Surface | Window class | Truths preserved | Layout local | Routes to owner |
| ------- | ------------ | ---------------- | ------------ | --------------- |
| Notebook editor / cell surface | `primary_workspace_window` | 4 | `true` | `true` |
| Notebook editor / cell surface | `secondary_detached_window` | 4 | `true` | `true` |
| Tabular data grid surface | `primary_workspace_window` | 4 | `true` | `true` |
| Tabular data grid surface | `secondary_detached_window` | 4 | `true` | `true` |
| Profiler / performance surface | `primary_workspace_window` | 4 | `true` | `true` |
| Profiler / performance surface | `secondary_detached_window` | 4 | `true` | `true` |
| Profiler / performance surface | `floating_utility_window` | 4 | `true` | `true` |
| Pipeline / workflow graph surface | `primary_workspace_window` | 4 | `true` | `true` |
| Pipeline / workflow graph surface | `secondary_detached_window` | 4 | `true` | `true` |
| Documentation reader surface | `primary_workspace_window` | 4 | `true` | `true` |
| Documentation reader surface | `secondary_detached_window` | 4 | `true` | `true` |
| Documentation reader surface | `floating_utility_window` | 4 | `true` | `true` |
| Preview surface (render, diff, media) | `primary_workspace_window` | 4 | `true` | `true` |
| Preview surface (render, diff, media) | `floating_utility_window` | 4 | `true` | `true` |
| Preview surface (render, diff, media) | `companion_overlay_window` | 4 | `true` | `true` |
| Review / change-request surface | `primary_workspace_window` | 4 | `true` | `true` |
| Review / change-request surface | `secondary_detached_window` | 4 | `true` | `true` |
| Incident / operations-response surface | `primary_workspace_window` | 4 | `true` | `true` |
| Incident / operations-response surface | `secondary_detached_window` | 4 | `true` | `true` |
| Companion assistant surface | `primary_workspace_window` | 4 | `true` | `true` |
| Companion assistant surface | `companion_overlay_window` | 4 | `true` | `true` |
| Companion assistant surface | `floating_utility_window` | 4 | `true` | `true` |
| Operator / control-plane surface | `primary_workspace_window` | 4 | `true` | `true` |
| Operator / control-plane surface | `secondary_detached_window` | 4 | `true` | `true` |
| Operator / control-plane surface | `floating_utility_window` | 4 | `true` | `true` |

## Auto-narrowed rows

- `docs` (`yellow`) — A floating docs reference window discloses a purely-local reading state (its own density and scroll position) while keeping workspace-global trust, remote, profile, and recovery risk visible; the row is narrowed below green while global risk stays global.
- `incident` (`yellow`) — The incident surface is qualified at Beta; in the monitor-topology drill a detached war-room display recenters onto the primary display and discloses the narrowed but non-destructive recovery rather than orphaning, so the claim is narrowed and disclosed.
- `companion` (`yellow`) — The companion surface is qualified at Beta; when its owning window is not present an approval prompt relocates to a disclosed, waivered still-visible re-notification affordance in the primary workspace window rather than stealing focus or orphaning, and re-establishes the owning-window route when that window returns.
- `operator` (`yellow`) — The operator surface is qualified at Beta; a floating control utility window shows a compact, disclosed projection of the active deployment profile and remote host rather than the full inline identity strip, while keeping all four workspace-global truths visible, so the claim is narrowed and disclosed.

## Exact parity causes

- `docs` — `upstream_dependency_narrowed` (disclosed: `true`) — A window discloses a purely-local view state (per-window density or a collapsed panel) that never hides workspace-global risk or policy state.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `incident` — `secondary_display_topology_drift` (disclosed: `true`) — A crash-restore, dependency-loss, or monitor-topology drill discloses a narrowed but non-destructive recovery (a window recenters or a detached window rejoins the primary).
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — A routed dialog, notification, or approval relocates to a disclosed, waivered still-visible re-notification affordance when its owning window is not present.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — A detached or utility window shows a narrowed-but-still-visible projection of a workspace-global truth while a peer window shows it in full.

## Active waivers

- `waiver:companion-routing-relocation:0001` (`companion`, owner: Companion surface owner, expires `2026-09-30T00:00:00Z`) — When the companion's owning window is not present, an approval prompt relocates from the owning window to a disclosed, still-visible re-notification affordance in the primary workspace window rather than stealing focus or orphaning; the owning-window route is re-established the moment that window returns, and the relocation is disclosed, never silent, while the shared attention-route contract is unified in the next sync.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity -- validate
cargo test -p aureline-shell --test m5_multi_window_parity_fixtures
```
