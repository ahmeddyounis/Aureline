# aureline-buffer

## Purpose
Editor buffer core: piece-tree storage, selections and multicursor, save
coordination, large-file mode, and undo/redo history.

## Protected-path status
Protected. Source fidelity, save correctness, and undo integrity are
release-bearing obligations and live behind this crate's API.

## Allowed dependencies
- May depend on `aureline-text` for encoding and segmentation.
- May depend on `aureline-telemetry` for hot-path instrumentation.
- Must not depend on `aureline-render`, `aureline-vfs`, `aureline-rpc`, or
  `aureline-shell-spike`.

## Canonical owner path
`crates/aureline-buffer/`

## Work packages
- WP-02 (Editor and buffer core)
