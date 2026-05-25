# Design-token runtime certification — release evidence

Reviewer-facing evidence packet for the lane that certifies the **design-token
runtime across dark, light, high-contrast, reduced-motion, and density rows** on
claimed-stable desktop shell surfaces: one canonical record per appearance
posture that binds one appearance-session value behind every capture, mode
conformance across the five claimed appearance rows, non-color cue survival for
diagnostics / policy locks / trust warnings / execution targets / selection /
focus, per-axis live-apply honesty with no silent lag, runtime-owned motion
suppression, launch-surface conformance with no hard-coded styling, a public
claim ceiling, an automatic narrow-below-Stable verdict, recovery and route
parity across the settings appearance panel / command palette / status bar /
menus, accessibility across normal / high-contrast / zoomed layouts, and rows
that stay available without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light/`](../../../fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light/)
- Schema: [`/schemas/ux/certify-the-design-token-runtime-across-dark-light.schema.json`](../../../schemas/ux/certify-the-design-token-runtime-across-dark-light.schema.json)
- Companion doc: [`/docs/ux/m4/certify-the-design-token-runtime-across-dark-light.md`](../../../docs/ux/m4/certify-the-design-token-runtime-across-dark-light.md)
- Typed source: `aureline_settings::design_token_runtime_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_design_token_runtime_stable`
- Replay + invariant gate: `crates/aureline-settings/tests/design_token_runtime_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal | **stable** | stable | — |
| `forced_colors_reload_disclosed.json` | forced-colors reload disclosed | **stable** | stable | — |
| `density_surface_in_preview.json` | activity-center surface in preview | preview (narrowed) | preview | `surface_not_yet_stable` |
| `hardcoded_styling_drill.json` | hard-coded styling drill | beta (narrowed) | stable | `hardcoded_stable_styling` |

Coverage verdict: **2 Stable, 2 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Dark, light, high-contrast, reduced-motion, and density conformance is green
  on every claimed launch-critical row, with attached visual-diff artifacts.**
  `mode_rows[]` covers all six `AppearanceModeClass` values; each carries
  `token_registry_resolves` proven against `aureline_ui::tokens::seeded_token_registry`,
  the resolved `certified_token_refs`, and a `golden_capture_ref` /
  `accessibility_packet_ref` pair. On the Stable postures every row `conforms`.
- **Theme or contrast changes do not break focus rings, state badges, severity
  cues, or keyboard-visible affordances.** Each mode row asserts
  `focus_ring_preserved`, `state_badges_preserved`, `severity_cues_preserved`,
  and `keyboard_affordances_preserved`; each `protected_cues[]` row binds a
  non-color carrier and survives high-contrast, forced-colors, and
  reduced-motion modes — hue alone is forbidden.
- **Captured artifacts and runtime inspection show the same semantic token names
  and appearance-session values.** `appearance_session.value_ref` is the one
  value every `mode_rows[].appearance_session_value_ref` cites; a mismatch is a
  hard build error (`CaptureSessionMismatch`), so screenshots and shipped
  behavior provably use one source of truth.
- **Live application of OS theme/contrast/accent/text-scale, or an explicit
  reload/restart label.** `live_apply_axes[]` covers all seven appearance axes;
  reload/restart rows set `disclosure_required = true`, and no row may set
  `silently_lags_system` — both are enforced at build time.
- **Reduced-motion and power-saving suppression lives in the token/runtime
  model.** `motion_suppression[]` is projected from the motion presets; each
  posture carries `modeled_in_token_runtime` and `per_surface_improvisation_absent`.
- **Any non-conforming surface is automatically narrowed below Stable in product
  copy, docs/help, and release packets.** `density_surface_in_preview.json`
  narrows to `preview` via `surface_not_yet_stable`;
  `hardcoded_styling_drill.json` narrows to `beta` via `hardcoded_stable_styling`.
  Both carry `honesty_marker_present = true` and a bounded `waiver_ref` on the
  narrowed surface.

## Guardrails honored

- No feature team can hard-code colors, density, or motion for a Stable shell
  row without the lane detecting it: the hard-coded drill surfaces
  `hardcoded_stable_styling` and forces the claim below Stable.
- A surface that cannot honor the runtime narrows the claim (with a `waiver_ref`)
  instead of leaking bespoke styling into Stable.
- The record stays available without an account or managed services
  (`available_without_account`, `available_without_managed_services`).

## Reproduce

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_design_token_runtime_stable -- index

cargo run -q -p aureline-settings \
  --bin aureline_settings_design_token_runtime_stable -- emit-fixtures \
  fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light

cargo test -p aureline-settings --test design_token_runtime_stable_fixtures
```
