# Proof packet: editor viewport compositor

Purpose: anchor proof captures for the canonical editor viewport model (scroll,
caret, selection, IME) plus its compositor and damage-class vocabulary.

Canonical sources (non-exhaustive):

- `crates/aureline-editor/src/viewport/mod.rs`
- `crates/aureline-editor/src/paint/mod.rs`
- `fixtures/editor/viewport_cases/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (live consumer)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/editor_viewport_validation_capture.json`
- Command: `cargo test -p aureline-editor`

