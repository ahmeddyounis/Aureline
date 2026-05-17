# Token Conformance Audit

The beta token-conformance packet is generated from
`aureline-design-system` and checked into:

- `fixtures/ux/m3/state_semantics/token_conformance_report.json`
- `artifacts/ux/m3/component_state_screenshot_diff/packet.json`

Current status: `pass`.

## Covered Surfaces

The packet covers shell chrome, Start Center, command palette, search,
dialog sheets, trust prompts, notification envelopes, Help/About rows,
settings root, and activity-center rows.

## Enforced Rules

- Launch-critical rows consume canonical state families and lifecycle, route,
  readiness, and policy cue families.
- Color, spacing, sizing, density, and motion references come from canonical
  token families.
- Raw color literals and local token forks are forbidden on first-party beta
  truth rows.
- Hover-only critical actions, missing focus visibility, spinner-only blocked
  states, and color-only state meaning block the gate.

## Verify

```sh
python3 ci/check_m3_component_state_token_contract.py --repo-root .
cargo test -q -p aureline-design-system
```
