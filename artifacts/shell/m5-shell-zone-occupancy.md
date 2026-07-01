# M5 shell-zone occupancy & declared-slot routing

Generated from the seeded packet in
[`crate::m5_shell_zone_occupancy`](../../crates/aureline-shell/src/m5_shell_zone_occupancy/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- markdown > \
  artifacts/shell/m5-shell-zone-occupancy.md
```

- Packet id: `m5-shell-zone-occupancy:stable:0001`
- Source schema ref: `schemas/shell/m5-shell-zone-occupancy.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 10
- Green (fully occupied): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Occupancy rows

| Occupant surface | Status | Qualification | Occupied slot | Attachment | Occupant | Route | Waiver |
| ---------------- | ------ | ------------- | ------------- | ---------- | -------- | ----- | ------ |
| Notebook editor / cell surface | `green` | `stable` | `main_workspace` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Tabular data grid surface | `yellow` | `stable` | `main_workspace` | `attached_to_declared_slot` | `dependency_missing_placeholder` | `all_routes_resolve_to_slot_occupant` | — |
| Profiler / performance surface | `green` | `stable` | `bottom_panel` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Pipeline / workflow graph surface | `green` | `stable` | `main_workspace` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Documentation reader surface | `green` | `stable` | `main_workspace` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Preview surface (render, diff, media) | `green` | `stable` | `right_inspector` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Review / change-request surface | `green` | `stable` | `main_workspace` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Incident / operations-response surface | `yellow` | `beta` | `right_inspector` | `attached_to_declared_slot` | `occupant_available` | `all_routes_resolve_to_slot_occupant` | — |
| Companion assistant surface | `yellow` | `beta` | `right_inspector` | `attached_to_declared_slot` | `occupant_available` | `disclosed_route_fallback` | `waiver:companion-onboarding-route-sync:0001` |
| Operator / control-plane surface | `yellow` | `beta` | `bottom_panel` | `attached_to_declared_slot` | `policy_blocked_placeholder` | `all_routes_resolve_to_slot_occupant` | — |

## Auto-narrowed rows

- `data_grid` (`yellow`) — The data grid's remote source is unavailable on this build, so the surface degrades to a disclosed reconnect placeholder card that keeps its main-workspace slot occupied; the row is narrowed below green while the placeholder is shown.
- `incident` (`yellow`) — The incident surface is qualified at Beta in the frozen shell-zone matrix and, under the seeded compact width, occupies its declared right-inspector fallback slot; the claim is narrowed below Stable and disclosed.
- `companion` (`yellow`) — The companion surface is qualified at Beta; its onboarding route resolves to a disclosed, waivered fallback slot pending the next route-registry sync, while the command/keyboard/docs routes resolve to its declared right-inspector slot.
- `operator` (`yellow`) — The operator surface is qualified at Beta and, when the control-plane is policy-blocked, degrades to a disclosed policy-blocked placeholder card that keeps its bottom-panel slot occupied; the claim is narrowed and disclosed.

## Exact occupancy causes

- `data_grid` — `upstream_dependency_narrowed` (disclosed: `true`) — A missing dependency degrades the occupant to a disclosed in-slot placeholder card that preserves spatial continuity.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `owning_window_routing_lost` (disclosed: `true`) — A command/keyboard/docs/onboarding route resolves to a disclosed, waivered alternative slot for the same occupant.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `operator` — `policy_blocked` (disclosed: `true`) — A policy block degrades the occupant to a disclosed in-slot placeholder card that preserves spatial continuity.

## Active waivers

- `waiver:companion-onboarding-route-sync:0001` (`companion`, owner: Companion surface owner, expires `2026-09-30T00:00:00Z`) — The companion onboarding route temporarily resolves to the right-inspector sheet rather than the companion overlay while the onboarding route registry is unified in the next sync. The fallback is disclosed, never hidden, and the command/keyboard/docs routes already resolve to the declared slot and occupant.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- validate
cargo test -p aureline-shell --test m5_shell_zone_occupancy_fixtures
```
