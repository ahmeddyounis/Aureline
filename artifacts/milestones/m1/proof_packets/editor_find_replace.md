# Proof packet: editor in-file find/replace

Purpose: anchor proof captures for in-file lexical find/replace, highlight
overlay spans, and the live shell wiring that exercises them.

Canonical sources (non-exhaustive):

- `crates/aureline-editor/src/find_replace/`
- `crates/aureline-editor/src/highlight/`
- `crates/aureline-editor/src/paint/mod.rs` (overlay paint consumer)
- `fixtures/editor/find_replace_cases/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (live consumer)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/editor_find_replace_validation_capture.json`
- Command: `cargo test -p aureline-editor -p aureline-shell`

