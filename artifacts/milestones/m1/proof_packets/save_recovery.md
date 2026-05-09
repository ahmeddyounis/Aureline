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
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
- `fixtures/editor/tab_cases/`

Evidence storage:

- Smoke outputs: `artifacts/milestones/m1/smoke_outputs/`
- Replay fixtures: `artifacts/milestones/m1/replay_fixtures/`

How to exercise:

- `cargo test -p aureline-shell bootstrap::native_shell::tab_case_tests::tab_case_fixtures_preserve_shared_buffer_authority`
- `cargo run -p aureline-shell --bin aureline_shell`
  - `Ctrl+O` opens a new tab.
  - `Ctrl+Tab` cycles active tabs.
  - `Ctrl+\\` splits the editor group and duplicates the active tab as a second view.
  - `Ctrl+S` saves the active tab (clears `Modified`).
  - `Ctrl+W` closes the active tab (`Ctrl+Shift+W` closes the focused editor group).
