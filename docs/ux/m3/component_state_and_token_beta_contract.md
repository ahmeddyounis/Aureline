# Component-State And Token Beta Contract

This contract promotes the existing token/state audit into a release-review
packet for launch-critical shell surfaces. It does not replace the `aureline-ui`
token registry or component-state implementation; it binds those primitives to
reviewable beta fixtures and a CI gate.

## Canonical Registry

The canonical registry is:

- `schemas/ux/component_state_registry.schema.json`
- `fixtures/ux/m3/state_semantics/component_state_registry.json`
- `crates/aureline-design-system/src/lib.rs`

It publishes seven user-visible state families: `empty`, `loading`, `pending`,
`degraded`, `blocked`, `error`, and `completed`. Each row maps back to the
shared `aureline_ui::components::ComponentStateClass` vocabulary and carries
semantic token refs plus non-color cues.

Badge and notice families are closed for beta truth: `lifecycle`, `route`,
`readiness`, and `policy`. Both badge and notice carriers require text plus
shape fallback, so support exports and high-contrast captures can preserve
meaning without interpreting color.

## Launch-Critical Surfaces

The beta surface set is shell chrome, Start Center, command palette, search,
dialog sheets, trust prompts, notification envelopes, Help/About rows, settings
root, and activity-center rows. A launch-critical row either consumes the
registry or declares a bounded waiver; the seeded packet has no waivers.

## Screenshot-Diff Harness

`artifacts/ux/m3/component_state_screenshot_diff/packet.json` is the checked
matrix. It carries refs for baseline captures, comparison captures, keyboard
journeys, assistive-technology evidence, and token-conformance rows. The refs
are metadata-safe and do not embed raw screenshots.

The matrix fails when a row permits:

- hover-only critical actions;
- missing focus visibility;
- spinner-only blocked states;
- state meaning carried only by color;
- semantic drift between baseline and comparison.

## Token-Conformance Gate

`fixtures/ux/m3/state_semantics/token_conformance_report.json` is the CI and
release packet. It asserts that launch-critical rows consume `al.color.*`,
`status.*`, `trust.*`, `size.*`, `space.*`, and `motion.*` families through
canonical records, and rejects raw-color literals or local token forks on
first-party beta truth rows.

Run:

```sh
python3 ci/check_m3_component_state_token_contract.py --repo-root .
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- validate
```
