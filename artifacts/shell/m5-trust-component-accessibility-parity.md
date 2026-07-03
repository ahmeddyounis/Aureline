# M5 trust-component accessibility parity

Generated from the seeded packet in
[`crate::m5_trust_component_accessibility_parity`](../../crates/aureline-shell/src/m5_trust_component_accessibility_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_accessibility_parity -- markdown > \
  artifacts/shell/m5-trust-component-accessibility-parity.md
```

- Packet id: `m5-trust-component-accessibility-parity:stable:0001`
- Source schema ref: `schemas/shell/m5-trust-component-accessibility-parity.schema.json`
- Certifies matrix packet: `m5-trust-chronology-components:stable:0001`
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
- `zoom_contrast_density`
- `motion_alternative`
- `support_export_parity`

## Certification rows

| Condition | Status | Qualification | Reach | Zoom/contrast/density | Motion | Support-export | No-hover/color-only/compaction-lost | Waiver |
| --------- | ------ | ------------- | ----- | --------------------- | ------ | -------------- | ----------------------------------- | ------ |
| Keyboard-only reach | `green` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_contrast_density` | `durable_text_alternative_present` | `component_truth_reconstructable` | `true` | — |
| Focus order after open / dismiss | `green` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_contrast_density` | `durable_text_alternative_present` | `component_truth_reconstructable` | `true` | — |
| Screen-reader narration & live announcements | `yellow` | `stable` | `disclosed_reduced_reach_detail` | `legible_stable_under_zoom_contrast_density` | `durable_text_alternative_present` | `component_truth_reconstructable` | `true` | — |
| High-zoom / large-text rendering | `yellow` | `stable` | `keyboard_focus_and_narration_reachable` | `disclosed_reduced_zoom_contrast_density_detail` | `durable_text_alternative_present` | `component_truth_reconstructable` | `true` | — |
| High-contrast rendering | `yellow` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_contrast_density` | `durable_text_alternative_present` | `disclosed_partial_capture` | `true` | — |
| Reduced-motion rendering | `yellow` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_contrast_density` | `disclosed_reduced_alternative_detail` | `component_truth_reconstructable` | `true` | `waiver:reduced-motion-alternative:0001` |
| Compact / comfortable density | `green` | `stable` | `keyboard_focus_and_narration_reachable` | `legible_stable_under_zoom_contrast_density` | `durable_text_alternative_present` | `component_truth_reconstructable` | `true` | — |

## Auto-narrowed rows

- `screen_reader_narration` (`yellow`) — Under the seeded screen-reader narration condition a small set of long capability-scope and chronology-detail narrations is disclosedly summarized (the full detail stays reachable on focus) while every component stays keyboard-focusable and announced and focus returns in order after dismiss; the reduction is disclosed and the row is narrowed below green.
- `high_zoom` (`yellow`) — Under the seeded high-zoom condition a few settings-row source pills and chronology verb labels wrap to a shorter form and a decorative accent drops to fit the large-text layout, while every component stays legible, keeps a non-color-only affordance, and keeps its truth-bearing content and reopen path; the reduction is disclosed and the row is narrowed below green.
- `high_contrast` (`yellow`) — Under the seeded high-contrast condition the support export reconstructs the component state and accessibility posture but discloses a partial capture of some low-priority decorative-contrast detail while the export queue is throttled; the partial capture is disclosed and the row is narrowed below green.
- `reduced_motion` (`yellow`) — The reduced-motion condition serves a summarized durable text alternative for a small set of high-frequency chronology live updates to avoid animating many components at once, while every component keeps a durable static text path and no truth is motion-only; the reduced alternative is disclosed behind a waiver, so the row is narrowed below green while the reduction is in force.

## Exact certification causes

- `screen_reader_narration` — `audit_truth_lost_off_primary_surface` (disclosed: `true`) — Under this condition a non-visual reach detail is disclosedly reduced (a longer narration abbreviates, or a focus-return lands one level up) while every component stays keyboard- and screen-reader-reachable; the reduction is disclosed and the row is narrowed below green.
- `high_zoom` — `audit_truth_lost_off_primary_surface` (disclosed: `true`) — Under this condition a zoom/contrast/density detail is disclosedly reduced (a label wraps to a shorter form, or a decorative accent drops in compact density) while every component stays legible and keeps a non-color-only affordance; the reduction is disclosed and the row is narrowed below green.
- `high_contrast` — `proof_stale` (disclosed: `true`) — The support export reconstructs the component truth and discloses a partial capture (some low-priority component detail is trimmed) while the reduction is disclosed and the row is narrowed below green.
- `reduced_motion` — `audit_truth_lost_off_primary_surface` (disclosed: `true`) — Under this condition a motion alternative is disclosedly reduced (a summarized text alternative for a small set of high-frequency updates) while a durable text path stays present; the reduction is disclosed and waivered, and the row is narrowed below green.

## Active waivers

- `waiver:reduced-motion-alternative:0001` (`reduced_motion`, owner: Shell/accessibility owner, expires `2026-09-30T00:00:00Z`) — Under the seeded reduced-motion condition the trust components serve a summarized durable text alternative for a small set of high-frequency live updates (a batched count in place of a pulsing chronology badge) while every component keeps a durable static text path, no truth is motion-only, and the reduced alternative is disclosed and reversible. The narrowing is disclosed, never hides a state, and keeps the keyboard/focus route.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_accessibility_parity -- validate
cargo test -p aureline-shell --test m5_trust_component_accessibility_parity_fixtures
```
