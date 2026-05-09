# Proof packet: Editor large-file mode

Purpose: anchor proof captures for editor large-file detection and the constrained
viewer surface used to inspect oversized or hostile files without forcing the
normal piece-tree buffer path.

Canonical sources (non-exhaustive):

- `docs/adr/0003-buffer-undo-large-file.md`
- `docs/editor/large_file_mode.md`
- `crates/aureline-editor/src/large_file/`
- `crates/aureline-editor/tests/large_file_cases.rs`
- `fixtures/editor/large_file_cases/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (example consumer wiring)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/editor_large_file_mode_validation_capture.json`
- Command: `cargo test -p aureline-editor -p aureline-shell`

