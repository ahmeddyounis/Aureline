# Design-token runtime certification across dark, light, high-contrast, reduced-motion, and density — contract

This is the reviewer-facing companion for the stable lane that certifies the
**design-token runtime** as a governed launch property: one governed record per
appearance posture that binds **one appearance-session value** behind every
capture, **mode conformance** across dark, light, high-contrast (dark and
light), reduced-motion, and density rows, **non-color cue survival** for
diagnostics / policy locks / trust warnings / execution targets / selection /
focus, **per-axis live-apply honesty**, **runtime-owned motion suppression**,
and **launch-surface conformance with no hard-coded styling** — all to a public
claim ceiling and an automatic narrow-below-Stable verdict.

This lane is a settings-side certification that *projects* the design-system
appearance runtime. It does not render a theme; it proves the token/runtime
object model resolves and stays attributable to one source of truth. It builds
on the appearance-session beta contract
(`aureline_design_system::appearance_session`), the component-state launch
surface registry (`aureline_design_system::seeded_component_state_registry`),
the per-theme semantic token registry (`aureline_ui::tokens`), and the motion
presets (`aureline_ui::motion`).

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light/`](../../../fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light/)
- Schema:
  [`/schemas/ux/certify-the-design-token-runtime-across-dark-light.schema.json`](../../../schemas/ux/certify-the-design-token-runtime-across-dark-light.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/certify-the-design-token-runtime-across-dark-light.md`](../../../artifacts/ux/m4/certify-the-design-token-runtime-across-dark-light.md)
- Typed source: `aureline_settings::design_token_runtime_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_design_token_runtime_stable`
- Replay + invariant gate:
  `crates/aureline-settings/tests/design_token_runtime_stable_fixtures.rs`

## Why one governed certification record

The design-token runtime is consumed by every launch-critical shell surface. If
each surface improvises colors, density, or motion, then a theme or contrast
change can silently break a focus ring, a state badge, a severity cue, or a
keyboard-visible affordance — and the screenshots in the release packet might be
captured from a different appearance state than the one that actually ships. The
result is a green "theme parity" claim that is really an average over surfaces
that each diverge a little, with evidence that cannot be tied back to one runtime
state.

A `design_token_runtime_certification_record` closes that gap. For one
appearance posture it binds:

- **One appearance-session value.** `appearance_session.value_ref` records the
  active appearance-session id and revision. Every mode row's `golden_capture_ref`
  and `accessibility_packet_ref` is attributed to that same value, so captured
  artifacts and runtime inspection provably use one source of truth. A capture
  attributed to a different value is a hard build error, not a silent average.
- **Mode conformance.** One `mode_rows[]` entry per dark, light, high-contrast
  dark, high-contrast light, reduced-motion, and density mode. Each proves the
  semantic token registry resolves for the row's theme (`token_registry_resolves`
  with the actual `certified_token_refs`) and that the focus ring, state badges,
  severity cues, and keyboard affordances survive the mode change.
- **Non-color cue survival.** One `protected_cues[]` entry per diagnostics,
  policy lock, trust warning, execution target, selection, and focus cue. Each
  carries a non-color carrier (`label_text`, `icon`, `border`, `shape`, or
  `focus_ring`) and proves it survives high-contrast, forced-colors, and
  reduced-motion modes. Hue alone is forbidden.
- **Live-apply honesty.** One `live_apply_axes[]` entry per appearance axis. Each
  declares whether an OS change applies live, applies live behind a checkpoint,
  requires confirmation, or requires a disclosed reload/restart. A reload/restart
  that does not disclose — or any axis that silently lags the system — is a hard
  build error.
- **Runtime-owned motion suppression.** One `motion_suppression[]` entry per
  posture, projected from the motion presets, proving non-essential motion
  suppression is modeled in the token runtime rather than improvised per surface.
- **Launch-surface conformance.** One `launch_surfaces[]` entry per
  launch-critical shell surface, proving it honors the token runtime with no
  hard-coded styling. A surface that cannot yet must carry a bounded `waiver_ref`
  and narrows the claim instead of leaking bespoke styling into Stable.

## The public claim ceiling and automatic narrowing

`pillars` are *derived* from the rows, never asserted. `claim_ceiling` records
what the posture is allowed to claim; the builder refuses to mint a record whose
ceiling exceeds its proof. `stable_qualification` then narrows the posture below
Stable with a named `narrowing_reasons[]` entry whenever a pillar fails or the
lowest surface marker is below Stable — so a posture never inherits Stable by
adjacency.

| Narrowing reason | Meaning |
| --- | --- |
| `mode_conformance_not_proven` | A required mode row does not conform. |
| `protected_cue_hue_only_risk` | A protected cue could rely on hue alone or fails a mode. |
| `captures_not_one_session` | Captures are not attributable to one appearance-session value. |
| `live_apply_silent_lag` | An axis silently lags or hides a reload/restart. |
| `motion_suppression_not_in_runtime` | Motion suppression is not modeled in the runtime. |
| `hardcoded_stable_styling` | A launch-critical Stable surface hard-codes styling. |
| `surface_not_yet_stable` | The lowest surface marker is below Stable. |

## Reachability, accessibility, and availability

The same record is reachable from the settings appearance panel, the command
palette, the status bar, and a menu command (`routes[]`), keyboard-first, and
the recovery routes (`recovery_routes[]`) hold across normal, high-contrast, and
zoomed layouts (`accessibility.layout_modes[]`). Every record stays available
without an account or managed services.

## Regenerating the records

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_design_token_runtime_stable -- emit-fixtures \
  fixtures/ux/m4/certify-the-design-token-runtime-across-dark-light
```

The replay gate
`crates/aureline-settings/tests/design_token_runtime_stable_fixtures.rs` fails if
the on-disk JSON drifts from the in-code projection.
