# Proof packet: piece-tree buffer core

Purpose: anchor proof captures for the canonical editor buffer core: piece-tree
storage, revisioned snapshots, grouped undo/redo, and snapshot-scoped
line/grapheme coordinate translation.

Canonical sources (non-exhaustive):

- `docs/adr/0003-buffer-undo-large-file.md`
- `docs/editor/piece_tree_contract.md`
- `crates/aureline-buffer/src/piece_tree/`
- `crates/aureline-buffer/tests/buffer/`
- `fixtures/text/large/clean_small_text.txt`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`

Evidence storage:

- Captures: `artifacts/milestones/m1/captures/`

