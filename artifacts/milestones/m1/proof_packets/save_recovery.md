# Proof packet: save and recovery lane

Purpose: anchor save/source-fidelity and restore/recovery proof captures in one
indexed location.

Canonical sources (non-exhaustive):

- `docs/verification/source_fidelity_and_undo_packet.md`
- `docs/ux/tabs_editor_groups_contract.md`
- `schemas/runtime/vfs_save_envelope.schema.json`
- `artifacts/io/save_rewrite_classes.yaml`
- `docs/recovery/restore_chooser_contract.md`
- `docs/support/recovery_ladder_packet.md`
- `crates/aureline-workspace/src/save/`
- `crates/aureline-workspace/src/save/drift_detection.rs`
- `crates/aureline-workspace/tests/save_pipeline_tests.rs`
- `crates/aureline-workspace/tests/save/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
- `fixtures/editor/tab_cases/`
- `fixtures/save/external_drift_cases/`

Evidence storage:

- Smoke outputs: `artifacts/milestones/m1/smoke_outputs/`
- Replay fixtures: `artifacts/milestones/m1/replay_fixtures/`

How to exercise:

- `cargo test -p aureline-workspace save_pipeline_tests`
- `cargo test -p aureline-workspace --test save`
- `cargo test -p aureline-shell bootstrap::native_shell::tab_case_tests::tab_case_fixtures_preserve_shared_buffer_authority`
- `cargo run -p aureline-shell --bin aureline_shell`
  - `Ctrl+O` opens a new tab.
  - `Ctrl+Tab` cycles active tabs.
  - `Ctrl+\\` splits the editor group and duplicates the active tab as a second view.
  - `Ctrl+S` saves the active tab (clears `Modified` and reports save outcome/strategy).
  - `Ctrl+W` closes the active tab (`Ctrl+Shift+W` closes the focused editor group).

External-drift failure drill (manual):

- Open a file in the shell, make it dirty in the editor, then modify the same file externally
  (for example from another terminal).
- Press `Ctrl+S` and confirm the shell refuses the save with an `external_change_detected` or
  `wrong_target_prevented` outcome and a next-action hint (compare/reload/reopen).
