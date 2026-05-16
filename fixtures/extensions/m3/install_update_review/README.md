# Extension mutation review beta fixtures

This fixture set exercises the shell review surface that joins extension
install, update, disable, and rollback truth before mutation.

The Rust consumer lives at
`crates/aureline-shell/src/extensions/review/` and reads the lower-level
extension review, install-review, permission-manifest delta, publisher
continuity, and state-preservation contracts. The same source-lane vocabulary
is used for public registry, mirror, offline bundle, and manual import cases.

Run:

```bash
cargo test -p aureline-shell extensions::review
```

