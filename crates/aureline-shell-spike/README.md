# aureline-shell-spike

## Purpose
Throwaway integration spike that wires the desktop shell, renderer, input loop,
buffer core, and VFS roots together end-to-end. Surfaces integration risk
ahead of the productionized shell crates.

## Protected-path status
Not protected. This crate is explicitly disposable; it must not be relied on by
any production crate and must be removed (or replaced) before stable claims
harden.

## Allowed dependencies
- May depend on any seeded internal crate for the purpose of integration probes.
- May depend on third-party rendering / windowing crates needed for the spike.
- Must not be depended on by any other internal crate.

## Canonical owner path
`crates/aureline-shell-spike/`

## Work packages
- WP-01 (Core shell and renderer)
