# Proof packet: keybinding resolver precedence + winning-source attribution

Purpose: anchor proof that the keybinding resolver exists as a runtime-owned
product object with deterministic precedence, winning-source attribution, and
conflict explainability, and that a live shell path routes at least one command
through the resolver.

Canonical sources (non-exhaustive):

- `docs/ux/keybinding_precedence.md`
- `docs/ux/keybinding_resolver_contract.md`
- `docs/migration/keymap_presets.md`
- `schemas/commands/keybinding_resolver.schema.json`
- `schemas/config/keybindings.schema.json`
- `crates/aureline-input/`
- `crates/aureline-input/src/presets/`
- `fixtures/input/keybinding_cases/`
- `crates/aureline-shell/src/bin/aureline_shell.rs`
- `crates/aureline-shell/src/help/keybinding_inspector.rs`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Smoke outputs (optional): `artifacts/milestones/m1/smoke_outputs/`
- Screenshots (optional): `artifacts/milestones/m1/screenshots/`
