# M5 status-bar item priority, placement, overflow & inspector back-links

Generated from the seeded packet in
[`crate::m5_status_bar_certification`](../../crates/aureline-shell/src/m5_status_bar_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification -- markdown > \
  artifacts/shell/m5-status-bar-certification.md
```

- Packet id: `m5-status-bar-certification:stable:0001`
- Source schema ref: `schemas/shell/m5-status-bar-certification.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 8
- Green: 4
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Priority classes (stable placement order)

- `recovery_critical` (rank 0)
- `execution_context` (rank 1)
- `ongoing_work` (rank 2)
- `ambient_metadata` (rank 3)

## Certification rows

| Context | Status | Qualification | Placement | Overflow | Back-link | Export | Keyboard | Waiver |
| ------- | ------ | ------------- | --------- | -------- | --------- | ------ | -------- | ------ |
| Notebook lane status bar | `green` | `stable` | `stable_priority_slots_no_jitter` | `keyboard_menu_palette_reachable` | `every_item_backlinks_to_narrowest_inspector` | `visible_and_overflowed_items_reconstructable` | `true` | — |
| Data / API lane status bar | `green` | `stable` | `stable_priority_slots_no_jitter` | `keyboard_menu_palette_reachable` | `every_item_backlinks_to_narrowest_inspector` | `visible_and_overflowed_items_reconstructable` | `true` | — |
| Remote lane status bar | `yellow` | `stable` | `stable_priority_slots_no_jitter` | `disclosed_reduced_overflow_route` | `every_item_backlinks_to_narrowest_inspector` | `visible_and_overflowed_items_reconstructable` | `true` | — |
| Preview lane status bar | `yellow` | `stable` | `stable_priority_slots_no_jitter` | `keyboard_menu_palette_reachable` | `disclosed_grouped_backlink` | `visible_and_overflowed_items_reconstructable` | `true` | — |
| Review lane status bar | `green` | `stable` | `stable_priority_slots_no_jitter` | `keyboard_menu_palette_reachable` | `every_item_backlinks_to_narrowest_inspector` | `visible_and_overflowed_items_reconstructable` | `true` | — |
| Profiler lane status bar | `yellow` | `stable` | `stable_priority_slots_no_jitter` | `keyboard_menu_palette_reachable` | `every_item_backlinks_to_narrowest_inspector` | `disclosed_partial_capture` | `true` | — |
| Incident lane status bar | `yellow` | `stable` | `disclosed_compact_priority_compaction` | `keyboard_menu_palette_reachable` | `every_item_backlinks_to_narrowest_inspector` | `visible_and_overflowed_items_reconstructable` | `true` | `waiver:incident-compact-priority-compaction:0001` |
| Desktop base status bar | `green` | `stable` | `stable_priority_slots_no_jitter` | `keyboard_menu_palette_reachable` | `every_item_backlinks_to_narrowest_inspector` | `visible_and_overflowed_items_reconstructable` | `true` | — |

## Auto-narrowed rows

- `remote_lane` (`yellow`) — The remote lane's status-menu overflow route is temporarily reduced while the remote-target registry re-syncs; keyboard search and the palette route still resolve every visible and overflowed item, and the reduction is disclosed. The row is narrowed below green while the route is reduced.
- `preview_lane` (`yellow`) — The preview lane's provider-freshness status items share one disclosed grouped inspector back-link into the preview provenance panel rather than an individual narrowest target; the grouping is disclosed and the row is narrowed below green.
- `profiler_lane` (`yellow`) — The profiler lane's support export reconstructs the visible status items and discloses a partial capture of the sampled capacity-meter overflow set while the sampler warms up; the partial capture is disclosed and the row is narrowed below green.
- `incident_lane` (`yellow`) — Under the seeded compact incident width the status bar performs a disclosed, waivered priority compaction that drops only ambient-metadata items; recovery-critical and execution-context items stay pinned, and the row is narrowed below green while the compaction is in force.

## Exact certification causes

- `remote_lane` — `hover_only_critical_truth` (disclosed: `true`) — One overflow reach route is temporarily reduced; at least one non-hover route (keyboard search, status menu, or palette) still resolves and the reduction is disclosed.
- `preview_lane` — `grouped_progress_unattributed` (disclosed: `true`) — Some status items share one disclosed grouped inspector back-link rather than an individual narrowest target.
- `profiler_lane` — `proof_stale` (disclosed: `true`) — The support export reconstructs the visible items and discloses a partial capture of the overflowed set.
- `incident_lane` — `vanity_item_reflow` (disclosed: `true`) — Under compact width the status bar performs a disclosed, waivered priority compaction that drops only ambient-metadata items; recovery-critical and execution-context items stay pinned in their stable slots.

## Active waivers

- `waiver:incident-compact-priority-compaction:0001` (`incident_lane`, owner: Incident surface owner, expires `2026-09-30T00:00:00Z`) — Under the seeded compact incident-response width, ambient-metadata status items compact into a disclosed summary chip while the recovery-critical incident state and the execution-context target stay pinned in their stable slots. The compaction is disclosed, never hidden, and every compacted item stays reachable through keyboard search, the status menu, and the palette.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification -- validate
cargo test -p aureline-shell --test m5_status_bar_certification_fixtures
```
