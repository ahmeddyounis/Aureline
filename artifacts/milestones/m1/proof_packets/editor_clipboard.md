# Proof packet: editor clipboard groundwork

Purpose: anchor proof captures for editor copy/cut/paste clipboard planning,
including representation-aware payload metadata and the stable copy-variant
vocabulary.

Canonical sources (non-exhaustive):

- `docs/editor/copy_contract.md`
- `docs/ux/clipboard_history_contract.md` (§5)
- `crates/aureline-editor/src/clipboard/`
- `crates/aureline-editor/src/selection/`
- `crates/aureline-editor/tests/clipboard_cases.rs`
- `fixtures/editor/clipboard_cases/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (live consumer)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/editor_clipboard_validation_capture.json`
- Command: `cargo test -p aureline-editor`

