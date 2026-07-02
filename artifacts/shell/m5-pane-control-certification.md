# M5 splitter & resizable-pane control precision, persistence, restore & export

Generated from the seeded packet in
[`crate::m5_pane_control_certification`](../../crates/aureline-shell/src/m5_pane_control_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification -- markdown > \
  artifacts/shell/m5-pane-control-certification.md
```

- Packet id: `m5-pane-control-certification:stable:0001`
- Source schema ref: `schemas/shell/m5-pane-control-certification.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 6
- Green: 2
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `resize_control_precision`
- `proportion_persistence`
- `reset_restore`
- `resize_state_export`

## Certification rows

| Layout | Status | Qualification | Precision | Persistence | Restore | Export | Keyboard-resizable | Waiver |
| ------ | ------ | ------------- | --------- | ----------- | ------- | ------ | ------------------ | ------ |
| Notebook cell / editor / output splitters | `green` | `stable` | `precise_pointer_and_keyboard_resize` | `proportions_or_presets_persisted` | `default_reset_and_topology_restore` | `proportions_and_actions_reconstructable` | `true` | — |
| Data grid query / grid / inspector splitters | `green` | `stable` | `precise_pointer_and_keyboard_resize` | `proportions_or_presets_persisted` | `default_reset_and_topology_restore` | `proportions_and_actions_reconstructable` | `true` | — |
| Review tree / diff / comment splitters | `yellow` | `stable` | `precise_pointer_and_keyboard_resize` | `disclosed_reduced_persistence_fidelity` | `default_reset_and_topology_restore` | `proportions_and_actions_reconstructable` | `true` | — |
| Docs nav / article / preview splitters | `yellow` | `stable` | `disclosed_reduced_hit_target_or_step` | `proportions_or_presets_persisted` | `default_reset_and_topology_restore` | `proportions_and_actions_reconstructable` | `true` | — |
| Profiler timeline / flame-graph / detail splitters | `yellow` | `stable` | `precise_pointer_and_keyboard_resize` | `proportions_or_presets_persisted` | `disclosed_reduced_restore_fidelity` | `proportions_and_actions_reconstructable` | `true` | `waiver:profiler-reduced-restore-fidelity:0001` |
| Incident signal / log / action splitters | `yellow` | `stable` | `precise_pointer_and_keyboard_resize` | `proportions_or_presets_persisted` | `default_reset_and_topology_restore` | `disclosed_partial_capture` | `true` | — |

## Auto-narrowed rows

- `review` (`yellow`) — When the review layout moves from the expanded desktop to a compact sheet its diff/comment preset snaps to the nearest safe ratio rather than the exact prior proportion; the intent stays serialized as proportions rather than pixels, the reduction is disclosed, and the row is narrowed below green.
- `docs` (`yellow`) — Under the seeded compact docs sheet the splitter's enlarged hit target shrinks to a disclosed narrower band and the keyboard step coarsens while both pointer and keyboard resize still resolve and the double-click default-size restore stays reachable; the reduction is disclosed and the row is narrowed below green.
- `profiler` (`yellow`) — The profiler layout resets to its named default and persists proportions, but a detached profiler window that loses its host monitor restores to a safe default layout rather than its exact prior ratios while the window host re-attaches; the fallback is disclosed behind a waiver and never destructive, so the row is narrowed below green while the reduction is in force.
- `incident` (`yellow`) — The incident console's support export reconstructs current pane proportions and discloses a partial capture of the recent resize-action log while the high-volume log is still being trimmed; the partial capture is disclosed and the row is narrowed below green.

## Exact certification causes

- `review` — `resize_state_not_serializable` (disclosed: `true`) — The persistence fidelity is disclosedly reduced under one topology (a preset snaps to the nearest safe ratio) while the intent stays serialized as proportions rather than pixels; the reduction is disclosed and the row is narrowed below green.
- `docs` — `pointer_only_resize` (disclosed: `true`) — Under compact width one precision affordance (the enlarged hit target or a fine keyboard step) is disclosedly reduced while pointer and keyboard resize both still resolve and the default-size restore stays reachable; the reduction is disclosed and the row is narrowed below green.
- `profiler` — `resize_state_not_serializable` (disclosed: `true`) — The restore fidelity is disclosedly reduced (a detached window falls back to a safe default layout after a display change) while the reset-to-default path and a non-destructive restore still resolve; the reduction is disclosed and waivered, and the row is narrowed below green.
- `incident` — `proof_stale` (disclosed: `true`) — The support export reconstructs current pane proportions and discloses a partial capture of the recent resize-action log while it is still being trimmed; the partial capture is disclosed and the row is narrowed below green.

## Active waivers

- `waiver:profiler-reduced-restore-fidelity:0001` (`profiler`, owner: Profiler surface owner, expires `2026-09-30T00:00:00Z`) — Under the seeded profiler capture, resize intent persists as proportions and resets to a named default, but a detached profiler window that loses its host monitor restores to a safe default layout rather than its exact prior ratios while the window host re-attaches. The fallback is disclosed, never destructive, and the pane proportions stay reconstructable from the support export.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification -- validate
cargo test -p aureline-shell --test m5_pane_control_certification_fixtures
```
