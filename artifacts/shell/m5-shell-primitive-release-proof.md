# M5 shell-primitive release proof

Generated from the seeded packet in
[`crate::m5_shell_primitive_release_proof`](../../crates/aureline-shell/src/m5_shell_primitive_release_proof/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof -- markdown > \
  artifacts/shell/m5-shell-primitive-release-proof.md
```

- Packet id: `m5-shell-primitive-release-proof:stable:0001`
- Source schema ref: `schemas/shell/m5-shell-primitive-release-proof.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 10
- Green: 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Truth pillars covered: ambient_instrumentation, durable_progress, pane_control, transient_inspect
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `primitive_truth`
- `representation_freshness`
- `interaction_reach`
- `exported_proof_parity`

## Rendering profiles

- `standard` — Standard desktop density
- `compact` — Compact / reduced-width density
- `expanded` — Expanded / wide density
- `multi_window` — Multi-window / detached shell
- `high_zoom` — High-zoom / large-text rendering
- `high_contrast` — High-contrast rendering
- `reduced_motion` — Reduced-motion rendering

## Certification rows

| Primitive | Pillar | Status | Qualification | Truth | Representation | Reach | Export | No-hover/spinner/pointer-only | Waiver |
| --------- | ------ | ------ | ------------- | ----- | -------------- | ----- | ------ | ----------------------------- | ------ |
| Status-bar item | `ambient_instrumentation` | `green` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Status overflow menu | `ambient_instrumentation` | `green` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Tooltip | `transient_inspect` | `green` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Hovercard | `transient_inspect` | `yellow` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `disclosed_partial_export_refresh` | `true` | — |
| Peek panel | `transient_inspect` | `green` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Pinned-preview promotion | `transient_inspect` | `yellow` | `stable` | `primitive_truth_certified_and_current` | `disclosed_partial_representation` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Splitter handle | `pane_control` | `green` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Pane-resize preset | `pane_control` | `yellow` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `disclosed_reduced_reach_or_resize` | `exported_surfaces_reflect_current_proof` | `true` | `waiver:pane-resize-reduced-reach:0001` |
| Progress indicator | `durable_progress` | `yellow` | `stable` | `disclosed_reduced_truth_scope` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |
| Durable job row | `durable_progress` | `green` | `stable` | `primitive_truth_certified_and_current` | `source_freshness_representation_preserved` | `keyboard_touch_reach_and_resize_certified` | `exported_surfaces_reflect_current_proof` | `true` | — |

## Auto-narrowed rows

- `hovercard` (`yellow`) — The hovercard's exported proof reflects the current provenance/representation state and discloses a partial refresh of a low-priority decorative-attribution detail while the export queue is throttled; the partial refresh is disclosed and the row is narrowed below green.
- `pinned_preview_promotion` (`yellow`) — Across a promotion the pinned-preview promotion trims a low-priority provenance strip detail to a shorter form while the source, freshness, and representation truth stay preserved and no stale preview reads as live; the reduction is disclosed and the row is narrowed below green.
- `pane_resize_preset` (`yellow`) — The pane-resize preset serves a coarser keyboard resize step on the compact profile while every preset stays keyboard-invokable, serializable, and precise and no resize is pointer-only; the reduced reach is disclosed behind a waiver, so the row is narrowed below green while the reduction is in force.
- `progress_indicator` (`yellow`) — The progress indicator presents a grouped batch summary in place of per-item progress for a small set of high-frequency jobs while every job's primary state stays current, named, and reopenable into durable history; the reduced scope is disclosed and the row is narrowed below green.

## Exact certification causes

- `hovercard` — `proof_stale` (disclosed: `true`) — The export reflects the current proof and discloses a partial refresh (some low-priority primitive detail is trimmed) while the reduction is disclosed and the row is narrowed below green.
- `pinned_preview_promotion` — `source_freshness_hidden` (disclosed: `true`) — A low-priority representation detail is trimmed (a provenance strip abbreviates) while the source, freshness, and representation truth stay preserved; the reduction is disclosed and the row is narrowed below green.
- `pane_resize_preset` — `hover_only_critical_truth` (disclosed: `true`) — A coarser touch target or a reduced keyboard resize step is served while a keyboard/touch path stays present and precise; the reduction is disclosed behind a waiver and the row is narrowed below green.
- `progress_indicator` — `spinner_only_state` (disclosed: `true`) — A low-priority slice of the primitive's typed state truth is presented at a coarser scope (a grouped summary in place of per-item detail) while the primary state stays current and named; the reduction is disclosed and the row is narrowed below green.

## Active waivers

- `waiver:pane-resize-reduced-reach:0001` (`pane_resize_preset`, owner: Shell/layout owner, expires `2026-09-30T00:00:00Z`) — Under the seeded release proof the pane-resize preset serves a coarser keyboard resize step on the compact profile while every preset stays keyboard-invokable, serializable, and precise on the standard profile, and no resize is pointer-only. The reduced reach is disclosed and reversible; the narrowing is disclosed, never hides a state, and keeps the keyboard/touch route.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof -- validate
cargo test -p aureline-shell --test m5_shell_primitive_release_proof_fixtures
```
