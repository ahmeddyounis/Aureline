# Proof packet: IME + dead-key safe text input normalization

Purpose: anchor proof that the desktop shell enables IME-capable text input and
routes IME preedit/commit plus layout-safe text bursts through a shared
normalizer for the editor viewport and command palette query field, and that
the live editor viewport clears and renders composition overlays without
corrupting committed text.

Canonical sources (non-exhaustive):

- `crates/aureline-input/src/text_input/`
- `fixtures/input/ime_cases/`
- `crates/aureline-shell/src/windowing/winit_window.rs`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
- `crates/aureline-shell/src/palette/query_session.rs`
- `crates/aureline-editor/src/viewport/mod.rs`
- `docs/editor/piece_tree_contract.md`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Fixtures: `fixtures/input/ime_cases/`
