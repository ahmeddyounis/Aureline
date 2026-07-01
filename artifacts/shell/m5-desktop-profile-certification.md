# M5 desktop-profile certification: shell-zone, adaptive-layout, multi-window, and owning-window routing truth on every claimed desktop profile

Generated from the seeded packet in
[`crate::m5_desktop_profile_certification`](../../crates/aureline-shell/src/m5_desktop_profile_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification -- markdown > \
  artifacts/shell/m5-desktop-profile-certification.md
```

- Packet id: `m5-desktop-profile-certification:stable:0001`
- Source schema ref: `schemas/shell/m5-desktop-profile-certification.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required truth dimensions: `shell_zone_integrity`, `adaptive_layout`, `multi_window_truth`, `owning_window_routing`
- Required profiles: `compact_desktop`, `standard_desktop`, `expanded_desktop`, `mixed_dpi`, `multi_monitor`, `dependency_missing_restore`
- Rows certified: 6
- Green (full continuity): 3
- Yellow (auto-narrowed): 3
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Profile | Status | Shell-zone integrity | Adaptive layout | Multi-window truth | Owning-window routing | Waiver |
| ------- | ------ | -------------------- | --------------- | ------------------ | --------------------- | ------ |
| Compact desktop (narrow width / zoom) | `yellow` | `all_surfaces_in_declared_slots` | `disclosed_collapse_narrowing` | `all_truths_preserved_layout_local` | `routes_to_owning_object_no_focus_theft` | — |
| Standard desktop (default width) | `green` | `all_surfaces_in_declared_slots` | `identity_stable_no_unusable_pane` | `all_truths_preserved_layout_local` | `routes_to_owning_object_no_focus_theft` | — |
| Expanded desktop (wide display) | `green` | `all_surfaces_in_declared_slots` | `identity_stable_no_unusable_pane` | `all_truths_preserved_layout_local` | `routes_to_owning_object_no_focus_theft` | — |
| Mixed-DPI (per-display scale factors) | `green` | `all_surfaces_in_declared_slots` | `identity_stable_no_unusable_pane` | `all_truths_preserved_layout_local` | `routes_to_owning_object_no_focus_theft` | — |
| Multi-monitor (secondary displays / topology change) | `yellow` | `all_surfaces_in_declared_slots` | `identity_stable_no_unusable_pane` | `all_truths_preserved_layout_local` | `disclosed_routing_relocation` | `waiver:multi-monitor-routing-relocation:0001` |
| Dependency-missing restore (crash / restart) | `yellow` | `disclosed_slot_fallback_narrowing` | `identity_stable_no_unusable_pane` | `disclosed_truth_projection_narrowing` | `routes_to_owning_object_no_focus_theft` | — |

## Auto-narrowed rows

- `compact_desktop` (`yellow`) — Under compact width and high zoom, several surfaces take a disclosed docked→sheet/overflow collapse narrowing that preserves the task identity, keeps critical state reachable through a keyboard overflow, and preserves the reopen path, so the profile is narrowed below green while identity stays stable.
- `multi_monitor` (`yellow`) — When a secondary-monitor window is closed or its display is removed while a routed approval is in flight, the approval is deferred to a disclosed, waivered relocation into the primary workspace window with a still-visible prompt rather than being orphaned, so the multi-monitor profile is narrowed and disclosed.
- `dependency_missing_restore` (`yellow`) — On a crash/restart restore where an extension, remote target, or feature pack is unavailable, affected surfaces fall back to their declared fallback shell slot and a workspace truth is projected in a disclosed reduced form until the dependency is restored; every fallback stays a declared slot and every truth stays visible in every window, so the profile is narrowed and disclosed.

## Exact profile causes

- `compact_desktop` — `upstream_dependency_narrowed` (disclosed: `true`) — Responsive collapse takes a disclosed docked→sheet/overflow narrowing under this profile while preserving the task identity and the reopen path, so the claim is narrowed and disclosed.
- `multi_monitor` — `upstream_dependency_narrowed` (disclosed: `true`) — A routed action from a closed window is relocated to a disclosed, waivered still-visible prompt in the primary workspace window under this profile rather than blocking outright, so nothing is orphaned.
- `dependency_missing_restore` — `upstream_dependency_narrowed` (disclosed: `true`) — A claimed surface falls back to its declared fallback shell slot because a dependency is unavailable under this profile; the fallback slot is still a declared shell slot and the narrowing is disclosed.
- `dependency_missing_restore` — `upstream_dependency_narrowed` (disclosed: `true`) — A workspace-global truth is projected in a disclosed reduced form until a dependency is restored under this profile, while staying visible in every window, so the claim is narrowed and disclosed.

## Active waivers

- `waiver:multi-monitor-routing-relocation:0001` (`multi_monitor`, owner: Attention-routing surface owner, expires `2026-09-30T00:00:00Z`) — When a secondary-monitor window is closed or its display is removed while a routed approval is in flight, the approval is relocated to a disclosed, still-visible prompt in the primary workspace window rather than being orphaned; the relocation is disclosed, never silent, and the shared routing contract is unified in the next attention-routing sync.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification -- validate
cargo test -p aureline-shell --test m5_desktop_profile_certification_fixtures
```
