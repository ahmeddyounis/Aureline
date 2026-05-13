# Filesystem Identity Alpha Fixtures

Protected workspace fixtures for the alpha filesystem-identity lane. These
cases are consumed by `crates/aureline-vfs/tests/filesystem_identity_alpha.rs`
and prove that canonical identity, alias inspection, save-token review,
external-change compare, and editor/Git/restore references project from the
same VFS identity object.

The fixtures intentionally use synthetic `file:///alpha/...` URIs so test
results do not depend on host paths, inode values, or local filesystem case
behavior.
