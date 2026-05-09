# Proof packet: editor continuity lane

Purpose: anchor proof captures for editor-buffer continuity across close/reopen,
split-move, and restore-handoff flows.

Canonical sources (non-exhaustive):

- `artifacts/tests/editor_continuity_matrix.yaml`
- `crates/aureline-shell/src/bootstrap/native_shell/continuity_tests.rs`
- `fixtures/editor/continuity_cases/`
- `crates/aureline-buffer/tests/buffer/property_suite.rs`
- `crates/aureline-buffer/src/piece_tree/`
- `docs/editor/piece_tree_contract.md`

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

How to exercise:

- `cargo test -p aureline-buffer property_suite::edit_sequences_match_naive_model_and_roundtrip_undo_redo`
- `cargo test -p aureline-shell bootstrap::native_shell::continuity_tests::editor_continuity_fixtures_stay_deterministic`

Latest validation capture:

- `artifacts/milestones/m1/captures/editor_continuity_validation_capture.json`
