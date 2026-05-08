# Proof packet: responsive fallback + split layout (shell)

Purpose: anchor proof that the live desktop shell exercises responsive-fallback
rules, editor-group split behavior, and minimum-width enforcement through one
shared layout contract (zone registry + split tree) rather than per-surface
heuristics.

Canonical sources (non-exhaustive):

- `crates/aureline-shell/src/layout/zone_registry.rs`
- `crates/aureline-shell/src/layout/split_tree.rs`
- `crates/aureline-shell/src/app_frame/desktop_frame.rs`
- `crates/aureline-shell/src/bin/aureline_shell.rs`
- `docs/ux/responsive_shell_rules.md`
- `docs/ux/shell_zone_and_density_contract.md`
- `docs/ux/shell_responsive_fallback_contract.md`
- `docs/ux/tabs_editor_groups_contract.md`
- `fixtures/ux/shell_layout_classes/`
- `fixtures/ux/responsive_fallback_cases/`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Screenshots (optional): `artifacts/milestones/m1/screenshots/`

How to exercise:

- `cargo run -p aureline-shell --bin aureline_shell`

