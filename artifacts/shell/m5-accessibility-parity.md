# M5 shell-primitive accessibility parity

Generated from the seeded packet in
[`crate::m5_accessibility_parity`](../../crates/aureline-shell/src/m5_accessibility_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity -- markdown > \
  artifacts/shell/m5-accessibility-parity.md
```

- Packet id: `m5-accessibility-parity:stable:0001`
- Source schema ref: `schemas/shell/m5-accessibility-parity.schema.json`
- Certifies matrix packet: `m5-shell-primitives:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 7
- Green: 3
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification dimensions

- `non_visual_reach`
- `zoom_contrast_stability`
- `motion_touch_alternative`
- `accessibility_export`

## Certification rows

| Condition | Status | Qualification | Reach | Zoom/contrast | Motion/touch | Export | No-pointer/hover-only | Waiver |
| --------- | ------ | ------------- | ----- | ------------- | ------------ | ------ | --------------------- | ------ |
| Keyboard-only reach | `green` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_and_contrast` | `durable_text_and_touch_alternatives_present` | `accessibility_posture_and_state_reconstructable` | `true` | — |
| Focus return after dismiss | `green` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_and_contrast` | `durable_text_and_touch_alternatives_present` | `accessibility_posture_and_state_reconstructable` | `true` | — |
| Screen-reader narration | `yellow` | `stable` | `disclosed_reduced_reach_detail` | `legible_stable_under_zoom_and_contrast` | `durable_text_and_touch_alternatives_present` | `accessibility_posture_and_state_reconstructable` | `true` | — |
| Touch / context-action alternatives | `green` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_and_contrast` | `durable_text_and_touch_alternatives_present` | `accessibility_posture_and_state_reconstructable` | `true` | — |
| High-zoom / large-text rendering | `yellow` | `stable` | `keyboard_focus_and_narration_reachable` | `disclosed_reduced_zoom_contrast_detail` | `durable_text_and_touch_alternatives_present` | `accessibility_posture_and_state_reconstructable` | `true` | — |
| High-contrast rendering | `yellow` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_and_contrast` | `durable_text_and_touch_alternatives_present` | `disclosed_partial_capture` | `true` | — |
| Reduced-motion rendering | `yellow` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_and_contrast` | `disclosed_reduced_alternative_detail` | `accessibility_posture_and_state_reconstructable` | `true` | `waiver:reduced-motion-alternative:0001` |

## Auto-narrowed rows

- `screen_reader_narration` (`yellow`) — Under the seeded screen-reader narration condition a small set of long hovercard/peek narrations is disclosedly summarized (the full detail stays reachable on focus) while every primitive stays keyboard-focusable and announced and focus returns after dismiss; the reduction is disclosed and the row is narrowed below green.
- `high_zoom` (`yellow`) — Under the seeded high-zoom condition a few status-item labels wrap to a shorter form and a decorative accent drops to fit the large-text layout, while every primitive stays legible and keeps its truth-bearing content and reopen path; the reduction is disclosed and the row is narrowed below green.
- `high_contrast` (`yellow`) — Under the seeded high-contrast condition the support export reconstructs the primitive state and accessibility posture but discloses a partial capture of some low-priority decorative-contrast detail while the export queue is throttled; the partial capture is disclosed and the row is narrowed below green.
- `reduced_motion` (`yellow`) — The reduced-motion condition serves a summarized durable text alternative for a small set of high-frequency progress rows and a coarser splitter touch target to avoid animating many primitives at once, while every primitive keeps a durable text and touch path and no truth is motion- or pointer-only; the reduced alternative is disclosed behind a waiver, so the row is narrowed below green while the reduction is in force.

## Exact certification causes

- `screen_reader_narration` — `hover_only_critical_truth` (disclosed: `true`) — Under this condition a non-visual reach detail is disclosedly reduced (a longer narration abbreviates, or a focus-return lands one level up) while every primitive stays keyboard- and screen-reader-reachable; the reduction is disclosed and the row is narrowed below green.
- `high_zoom` — `vanity_item_reflow` (disclosed: `true`) — Under this condition a zoom/contrast detail is disclosedly reduced (a label wraps to a shorter form, or a decorative accent drops) while every primitive stays legible; the reduction is disclosed and the row is narrowed below green.
- `high_contrast` — `proof_stale` (disclosed: `true`) — The support export reconstructs the accessibility posture and discloses a partial capture (some low-priority primitive detail is trimmed) while the reduction is disclosed and the row is narrowed below green.
- `reduced_motion` — `spinner_only_state` (disclosed: `true`) — Under this condition an alternative is disclosedly reduced (a coarser touch target, or a summarized text alternative) while a durable text and touch path stays present; the reduction is disclosed and waivered, and the row is narrowed below green.

## Active waivers

- `waiver:reduced-motion-alternative:0001` (`reduced_motion`, owner: Shell/accessibility owner, expires `2026-09-30T00:00:00Z`) — Under the seeded reduced-motion condition the shell serves a summarized durable text alternative for a small set of high-frequency progress rows (a batched count in place of per-item motion) and a coarser touch target for the splitter, while every primitive keeps a durable text and touch path, no truth is motion- or pointer-only, and the reduced alternative is disclosed and reversible. The narrowing is disclosed, never hides a state, and keeps the keyboard/focus route.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity -- validate
cargo test -p aureline-shell --test m5_accessibility_parity_fixtures
```
