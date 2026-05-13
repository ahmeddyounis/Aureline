# Filesystem Identity Alpha

This alpha slice makes the existing VFS identity model inspectable from one
workspace fixture lane. It does not replace the filesystem vocabulary in
`schemas/filesystem/save_target_token.schema.json` or the write-guarantee
contract in `docs/io/save_target_token_and_write_guarantee_contract.md`.

## Contract

The VFS-owned record path is:

- `IdentityRecord`: presentation path, logical workspace identity, canonical
  filesystem object, and alias set.
- `SaveTargetToken`: capability flags, write mode, compare-before-write
  generation token, and permission snapshot.
- `FilesystemIdentityReferenceSet`: the shared object reference that editor,
  Git, restore, and mutation records can all cite.
- `ExternalChangeCompareRecord`: a no-write compare result that blocks silent
  overwrite when the canonical target, generation token, or readable bytes no
  longer match the token pinned at open.

## Protected Fixtures

Fixtures live in `fixtures/workspace/filesystem_identity_alpha/` and are
exercised by:

```sh
cargo test -p aureline-vfs --test filesystem_identity_alpha
```

Current coverage:

- `symlink_alias_save_token.json`: the user opens a symlink presentation path;
  alias inspection and save-target review show that writes land on the canonical
  target while the shared identity ref remains stable.
- `case_only_alias_save_token.json`: the user opens a case-only variant on an
  insensitive-preserving root; path truth and save-token review identify the
  canonical case spelling.
- `external_change_compare.json`: a sibling writer changes the target after
  open; compare-before-write detects the generation mismatch, exposes a text
  diff summary, and offers compare/merge/reload/save-as/cancel instead of
  overwrite.

The fixture schema is `schemas/workspace/save_token_alpha.schema.json`.

## Consumer Rule

Editor buffers, Git diff/status rows, restore packets, and mutation journals
should store `filesystem_identity_ref` when they need a compact join key. They
may render surface-specific labels, but they should not derive new identity refs
from path strings. If a surface needs more detail, it resolves the ref back to
the VFS identity object or carries the full `IdentityRecord`.
