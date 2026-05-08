# Component state and focus-return contract

This document is the reviewer-facing entry point for the **shared component
state** vocabulary and the **focus-return** rules used by protected shell
surfaces.

## Normative sources

- `docs/design/component_state_taxonomy.md` — the canonical shared state
  taxonomy for reusable component contracts.
- `docs/design/design_token_component_state_vocabulary.md` — the design-token
  vocabulary and cross-theme invariants for state and focus visibility.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — selection, focus, and the
  focus-return rules under “Global interaction patterns”.

## Rust contract surface

The cross-surface implementation lives in `crates/aureline-ui/src/components/`:

- `aureline_ui::components::ComponentStateClass` — the closed vocabulary used
  by contracts, fixtures, and future telemetry.
- `aureline_ui::components::ComponentStates` — a lightweight bitmask used by
  renderers to compose common state combinations (hover/selected/focus-visible,
  warning, destructive, …) without minting per-surface flags.
- `aureline_ui::components::ComponentStateRegistry` — loads semantic tokens via
  [`aureline_ui::tokens::TokenRegistry`] and produces a token-backed chrome
  treatment for a surface.
- `aureline_ui::components::FocusReturnStack<T>` — a small shared helper for
  recording and restoring focus-return targets across transient surfaces.

## Focus visibility rules (implementation guidance)

- Focus is always visible for keyboard and assistive-technology journeys.
- A transient overlay that steals focus must render its own focus indicator
  rather than leaving the underlying surface highlighted.
- State styling must be token-backed; protected shell surfaces must not hard-code
  one-off colors for focus, selection, warning, or destructive emphasis.

## Focus-return rules (implementation guidance)

- A transient surface (dialog/sheet/palette/popover) records a focus-return
  target **before** it steals focus.
- On dismissal, focus returns to the invoker (or a safe fallback) and must remain
  visible immediately after the return.

## Current consumers

- `crates/aureline-shell/src/bootstrap/native_shell.rs` uses the shared
  component-state registry to draw visible focus treatment on the command palette
  query field and selected rows, and it records/restores palette focus return
  before switching focus into the transient overlay layer.
- The shell overlay layer renders a visible focus ring when the transient
  overlay owns focus.

