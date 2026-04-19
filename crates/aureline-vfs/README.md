# aureline-vfs

## Purpose
Virtual filesystem and workspace truth: roots and watchers, canonical path
identity, ignore resolution, and the abstraction that unifies local, remote,
and overlay filesystems behind a single API.

## Protected-path status
Protected. Workspace identity, alias and save-target resolution, and reactive
truth obligations attach to this crate.

## Allowed dependencies
- May depend on `aureline-text` for path encoding helpers.
- May depend on `aureline-telemetry` for instrumentation.
- Must not depend on `aureline-render`, `aureline-buffer`, `aureline-rpc`, or
  `aureline-shell-spike`.

## Canonical owner path
`crates/aureline-vfs/`

## Work packages
- WP-03 (Workspace, VFS, and persistence)
