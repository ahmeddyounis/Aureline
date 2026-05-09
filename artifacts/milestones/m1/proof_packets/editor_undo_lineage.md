# Proof packet: editor undo/redo lineage

Purpose: anchor proof captures for editor undo/redo with named undo classes and
actor/source lineage, including an undoable external reload path.

Canonical sources (non-exhaustive):

- `docs/editor/undo_class_contract.md`
- `docs/editor/piece_tree_contract.md`
- `crates/aureline-buffer/src/piece_tree/`
- `crates/aureline-editor/src/undo/mod.rs`
- `crates/aureline-editor/tests/undo_cases.rs`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (live consumer)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/editor_undo_lineage_validation_capture.json`
- Command: `cargo test -p aureline-buffer -p aureline-editor -p aureline-shell`

