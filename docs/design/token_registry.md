# Semantic token registry (review entrypoint)

This repository treats **semantic design tokens** as a governed contract:
first-party UI surfaces reference tokens by name (for example
`al.color.bg.canvas`) rather than embedding raw color, spacing, or timing
literals.

## Source of truth

The canonical ledgers live under `artifacts/design/` and are scoped by the
contracts in:

- `docs/design/design_token_component_state_vocabulary.md`
- `docs/design/semantic_token_domains_and_palette_contract.md`
- `docs/design/token_conformance_gate.md`

In particular:

- `artifacts/design/theme_support_rows.yaml` seeds baseline semantic theme
  tokens (dark reference and light parity) and the theme/posture axes.
- `artifacts/design/semantic_token_domains.yaml` freezes which domains own
  meaning (theme vs syntax vs diff vs status vs trust).
- `artifacts/design/geometry_token_ledger.yaml` publishes spacing/size/radius
  scales.
- `artifacts/design/motion_tokens.yaml` publishes motion duration tokens.

## Runtime consumption

First-party Rust surfaces should load tokens through the shared registry in
`crates/aureline-ui` and then reference token names in consuming code.

- Primary API: `aureline_ui::tokens::seeded_token_registry`
- Registry type: `aureline_ui::tokens::TokenRegistry`

The semantic-token conformance gate (`tools/ci/check_semantic_token_conformance.py`)
ensures consuming code does not introduce raw color literals outside the
design-system sources of truth.

