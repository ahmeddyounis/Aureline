# Theme pack fixtures and live switching contract

This document freezes the **first-party theme pack** fixtures used by the
desktop shell and any other first-party UI surface that consumes the
[`aureline-ui`] semantic token registry.

Theme packs define semantic token values for the four theme classes:

| Mode | `theme_class` |
| --- | --- |
| `dark` | `dark_reference` |
| `light` | `light_parity` |
| `hc-dark` | `high_contrast_dark` |
| `hc-light` | `high_contrast_light` |

High contrast is a first-party mode. Theme switching must not change layout,
surface structure, or command placement; it changes only the resolved token
values.

## Fixture sources

First-party theme pack fixtures live under:

- `fixtures/design/themes/dark.json`
- `fixtures/design/themes/light.json`
- `fixtures/design/themes/hc-dark.json`
- `fixtures/design/themes/hc-light.json`

Each file carries a `semantic_tokens` map keyed by semantic token name (for
example `al.color.bg.canvas`). The token registry loads these semantic tokens
first, then layers additional domain tokens (status, diff, syntax, charts)
from the design ledgers.

## Runtime consumption

The semantic token registry lives in `crates/aureline-ui`:

- `crates/aureline-ui/src/themes/packs.rs` loads the fixtures as first-party
  theme packs.
- `crates/aureline-ui/src/tokens/registry.rs` exposes the seeded token
  registry per theme class.

## Live switching and persistence

The desktop shell switches theme classes live (no restart) and persists the
active appearance session to:

- `./.logs/appearance/appearance_session.json`
- `./.logs/appearance/live_follow_system_policy.json`

The session record is the durable source of truth for which theme class is in
effect on protected shell surfaces (shell chrome, Start Center, command
palette/search, and embedded docs/help boundary chrome).

