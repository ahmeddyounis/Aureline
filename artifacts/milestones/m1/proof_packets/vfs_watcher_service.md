# Proof packet: VFS watcher service and structured change events

Purpose: anchor proof captures for the canonical watcher service responsible for
emitting structured filesystem change events plus watcher-health frames, and for
the first live shell consumer that uses the service rather than bespoke polling.

Canonical sources (non-exhaustive):

- `docs/adr/0006-vfs-save-cache-identity.md` (watcher-source + watcher-health vocabulary)
- `docs/filesystem/filesystem_identity_vocabulary.md`
- `crates/aureline-vfs/src/watcher.rs` (watcher source + health state machine)
- `crates/aureline-vfs/src/watchers/` (watcher service + fallback polling + event types)
- `crates/aureline-vfs/tests/watcher_polling_fallback.rs` (failure drill: force polling)
- `crates/aureline-shell/src/palette/query_session.rs` (first live consumer wiring)
- `fixtures/vfs/watcher_cases/` (fixture vocabulary/examples)

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/vfs_watcher_service_validation_capture.json`
- Command: `cargo test -p aureline-vfs -p aureline-shell`

