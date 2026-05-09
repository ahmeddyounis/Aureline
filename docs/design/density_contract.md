# Density contract (shell geometry + spacing)

Density is a presentation choice that changes **row/control heights** and **spacing budgets** while preserving command semantics, focus order, and information architecture.

This document is the reviewer-facing entrypoint for how first‑party shell surfaces derive density-aware measurements from semantic geometry tokens.

## Frozen vocabulary

- Density classes: `compact`, `standard`, `comfortable` (see `schemas/design/token_export_manifest.schema.json#/$defs/density_class`).
- Geometry tokens: `size.*` and `space.*` (see `artifacts/design/geometry_token_ledger.yaml`).

## Canonical token mapping

The density class selects the following tokens:

| Role | Token |
|---|---|
| Row height | `size.row.compact` / `size.row.standard` / `size.row.comfortable` |
| Control height | `size.control.compact` / `size.control.standard` / `size.control.comfortable` |
| Tab height | `size.tab` |
| Panel padding | `space.3` (compact) / `space.4` (standard) / `space.5` (comfortable) |
| Zone inset | `space.2` (compact) / `space.3` (standard) / `space.4` (comfortable) |
| Gutter | `space.2` (compact) / `space.3` (standard) / `space.4` (comfortable) |

## Runtime bindings (current)

- `crates/aureline-ui/src/density/mod.rs` exposes [`crate::density::DensityClass`] and [`crate::density::DensityProfile`] and loads all measurements via [`crate::tokens::TokenRegistry`].
- `crates/aureline-shell/src/bootstrap/native_shell.rs` consumes the profile in `ShellRenderStyle` and applies it to:
  - Start Center: panel padding and action-row geometry.
  - Command palette (quick‑open/search): panel spacing, query control height, result-row height, and gutters.
  - Embedded docs/help boundary card: panel spacing and density-aware row spacing.
  - Shell slot insets: `DesktopFrame::slot_rects_within_zone` receives a density-derived zone inset so shell slots shrink/expand consistently.

## User control

The shell cycles density with `Cmd/Ctrl+Shift+M` and persists the selection in `.logs/appearance/appearance_session.json` as `density_class`.

## Invariants

Density MUST NOT:

- change command meaning or availability;
- reorder focus traversal routes;
- reduce hit targets below floors owned by the geometry + hit-target contract.

