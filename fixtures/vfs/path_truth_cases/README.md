# Path-truth fixtures (alias inspector + chip + pre-write review)

This directory is the reviewable corpus for the **path-truth** projection set
the shell renders next to a file label: the chip, the alias inspector body,
and the pre-write save-target review.

Each fixture pairs a synthetic-root scenario (presentation URI, canonical URI,
alias set, capability flags, permission snapshot, trust posture) with the
expected projection record. The shell crate's
`crates/aureline-shell/src/path_truth/` module loads every `*.json` file here
and asserts byte-for-byte equality against `materialize_path_truth_projection`.

Vocabulary anchors:

- `docs/adr/0006-vfs-save-cache-identity.md` — five-layer identity model.
- `docs/filesystem/filesystem_identity_vocabulary.md` — alias-kind / token-kind
  vocabulary.
- `crates/aureline-vfs/src/identity/path_truth.rs` — chip class enumeration.
- `crates/aureline-vfs/src/identity/save_target_review.rs` — pre-write
  blocker enumeration.

The cases are intentionally small and deterministic: no machine-specific
inode, mtime, or filesystem-casing values appear; the synthetic root replays
the same `dev:.../ino:.../gen:N` shape on every host.
