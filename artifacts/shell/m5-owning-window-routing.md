# M5 owning-window routing: dialogs, approvals, and notifications bound to the owning window and object

Generated from the seeded packet in
[`crate::m5_owning_window_routing`](../../crates/aureline-shell/src/m5_owning_window_routing/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- markdown > \
  artifacts/shell/m5-owning-window-routing.md
```

- Packet id: `m5-owning-window-routing:stable:0001`
- Source schema ref: `schemas/shell/m5-owning-window-routing.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required routing expectations: `route_to_owning_window_object`, `preserve_object_anchor_on_return`, `no_focus_theft`, `no_orphan_on_detach`
- Rows certified: 10
- Green (full routing): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Routing rows

| Surface | Status | Qualification | Dialog binding | Reopen | Focus | OS notification | Waiver |
| ------- | ------ | ------------- | -------------- | ------ | ----- | --------------- | ------ |
| Notebook editor / cell surface | `green` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Tabular data grid surface | `green` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Profiler / performance surface | `green` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Pipeline / workflow graph surface | `green` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Documentation reader surface | `yellow` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `disclosed_deferral_to_badge_or_center` | `privacy_safe_summary_preserves_reopen` | — |
| Preview surface (render, diff, media) | `green` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Review / change-request surface | `green` | `stable` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Incident / operations-response surface | `yellow` | `beta` | `bound_to_owning_window_object` | `disclosed_placeholder_narrowing` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | — |
| Companion assistant surface | `yellow` | `beta` | `disclosed_binding_relocation` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `privacy_safe_summary_preserves_reopen` | `waiver:companion-binding-relocation:0001` |
| Operator / control-plane surface | `yellow` | `beta` | `bound_to_owning_window_object` | `reopens_exact_object_or_truthful_placeholder` | `no_focus_steal_on_typing` | `disclosed_minimal_summary` | — |

## Per-window routed-action plan

| Surface | Window class | Binds to owner | Preserves focus | Single reopen |
| ------- | ------------ | -------------- | --------------- | ------------- |
| Notebook editor / cell surface | `primary_workspace_window` | `true` | `true` | `true` |
| Notebook editor / cell surface | `secondary_detached_window` | `true` | `true` | `true` |
| Tabular data grid surface | `primary_workspace_window` | `true` | `true` | `true` |
| Tabular data grid surface | `secondary_detached_window` | `true` | `true` | `true` |
| Profiler / performance surface | `primary_workspace_window` | `true` | `true` | `true` |
| Profiler / performance surface | `secondary_detached_window` | `true` | `true` | `true` |
| Profiler / performance surface | `floating_utility_window` | `true` | `true` | `true` |
| Pipeline / workflow graph surface | `primary_workspace_window` | `true` | `true` | `true` |
| Pipeline / workflow graph surface | `secondary_detached_window` | `true` | `true` | `true` |
| Documentation reader surface | `primary_workspace_window` | `true` | `true` | `true` |
| Documentation reader surface | `secondary_detached_window` | `true` | `true` | `true` |
| Documentation reader surface | `floating_utility_window` | `true` | `true` | `true` |
| Preview surface (render, diff, media) | `primary_workspace_window` | `true` | `true` | `true` |
| Preview surface (render, diff, media) | `floating_utility_window` | `true` | `true` | `true` |
| Preview surface (render, diff, media) | `companion_overlay_window` | `true` | `true` | `true` |
| Review / change-request surface | `primary_workspace_window` | `true` | `true` | `true` |
| Review / change-request surface | `secondary_detached_window` | `true` | `true` | `true` |
| Incident / operations-response surface | `primary_workspace_window` | `true` | `true` | `true` |
| Incident / operations-response surface | `secondary_detached_window` | `true` | `true` | `true` |
| Companion assistant surface | `primary_workspace_window` | `true` | `true` | `true` |
| Companion assistant surface | `companion_overlay_window` | `true` | `true` | `true` |
| Companion assistant surface | `floating_utility_window` | `true` | `true` | `true` |
| Operator / control-plane surface | `primary_workspace_window` | `true` | `true` | `true` |
| Operator / control-plane surface | `secondary_detached_window` | `true` | `true` | `true` |
| Operator / control-plane surface | `floating_utility_window` | `true` | `true` | `true` |

## Auto-narrowed rows

- `docs` (`yellow`) — A docs update notification defers to a disclosed badge and activity-center row rather than stealing focus from an active typing surface; the row is narrowed below green while every routed action still binds to the owning window and object.
- `incident` (`yellow`) — The incident surface is qualified at Beta; a durable incident notification reopens onto a truthful placeholder that discloses the live war-room sub-state could not be restored while preserving the incident identity and the single reopen path, so the claim is narrowed and disclosed.
- `companion` (`yellow`) — The companion surface is qualified at Beta; when its owning window is not present an approval dialog relocates to a disclosed, waivered still-visible re-notification affordance in the primary workspace window rather than stealing focus or orphaning, and re-establishes the owning-window binding when that window returns.
- `operator` (`yellow`) — The operator surface is qualified at Beta; its OS-notification summary discloses a narrowed minimal projection (an even more redacted control-plane summary) while still routing to the single exact in-app reopen path without bypassing in-app review, so the claim is narrowed and disclosed.

## Exact routing causes

- `docs` — `upstream_dependency_narrowed` (disclosed: `true`) — A routed action defers to a disclosed badge or activity-center row rather than stealing focus while a protected typing path is active.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — A durable reopen lands on a truthful placeholder that discloses a narrowed context (a live sub-state that could not be restored) while preserving the object identity and the single reopen path.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — A routed dialog or approval relocates to a disclosed, waivered still-visible re-notification affordance when its owning window is not present, and re-establishes the owning-window route when it returns.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — The OS-notification summary discloses a narrowed minimal projection (an even more redacted summary) while still preserving the single exact in-app reopen path.

## Active waivers

- `waiver:companion-binding-relocation:0001` (`companion`, owner: Companion surface owner, expires `2026-09-30T00:00:00Z`) — When the companion's owning window is not present, an approval dialog relocates from the owning window to a disclosed, still-visible re-notification affordance in the primary workspace window rather than stealing focus or orphaning; the owning-window binding is re-established the moment that window returns, and the relocation is disclosed, never silent, while the shared attention-route contract is unified in the next sync.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- validate
cargo test -p aureline-shell --test m5_owning_window_routing_fixtures
```
