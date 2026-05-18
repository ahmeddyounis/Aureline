# Appearance Session Beta Fixtures

This directory contains the generated beta fixture for appearance-session,
theme-package, token-overlay, imported-theme mapping, and live OS appearance
change parity.

The JSON packet is emitted by `aureline_design_system_beta_contract`, so release
review reads the same records as the Rust validators.

## Fixtures

| File | Purpose |
| --- | --- |
| `appearance_session_beta_contract.json` | Aggregates the active appearance session, live follow-system policy, first-party theme package coverage, token-overlay warning state, imported-theme mapping report, OS live-change matrix, and protected cue preservation rows. |

## Regenerate

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- appearance-session > fixtures/ux/m3/theme_import_and_live_change/appearance_session_beta_contract.json
```

## Verify

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_beta_contract -- validate
cargo test -q -p aureline-design-system --test beta_contract_fixtures
```
