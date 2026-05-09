# Proof packet: VFS root abstraction and URI model

Purpose: anchor proof captures for the canonical VFS root abstraction and URI
model used to normalize local filesystem, generated, and virtual documents
behind one identity layer.

Canonical sources (non-exhaustive):

- `docs/workspace/vfs_root_contract.md`
- `docs/adr/0006-vfs-save-cache-identity.md`
- `docs/filesystem/filesystem_identity_vocabulary.md`
- `crates/aureline-vfs/src/uri_model.rs`
- `crates/aureline-vfs/src/roots/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (example consumer wiring)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/vfs_root_uri_model_validation_capture.json`
- Command: `cargo test -p aureline-vfs -p aureline-shell`

