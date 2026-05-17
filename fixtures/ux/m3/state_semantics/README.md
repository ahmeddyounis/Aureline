# Component-State Beta Fixtures

This directory contains the generated beta fixtures for launch-critical
component-state and token conformance. The JSON files are emitted by
`aureline_design_system_beta_contract`, so release review reads the same
records as the Rust validators.

## Fixtures

| File | Purpose |
| --- | --- |
| `component_state_registry.json` | Canonical state registry for `Empty`, `Loading`, `Pending`, `Degraded`, `Blocked`, `Error`, and `Completed`, plus lifecycle, route, readiness, and policy badge/notice families. |
| `screenshot_diff_matrix.json` | Screenshot-diff matrix rows covering launch-critical shell, dialog, search, trust-prompt, notification, Help/About, settings, activity, and entry surfaces across theme, density, and motion axes. |
| `token_conformance_report.json` | Token/state conformance packet used by CI and release review. |

## Regenerate

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- registry > fixtures/ux/m3/state_semantics/component_state_registry.json
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- screenshot-diff > fixtures/ux/m3/state_semantics/screenshot_diff_matrix.json
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- token-conformance > fixtures/ux/m3/state_semantics/token_conformance_report.json
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- screenshot-diff > artifacts/ux/m3/component_state_screenshot_diff/packet.json
```

## Verify

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- validate
python3 ci/check_m3_component_state_token_contract.py --repo-root .
```
