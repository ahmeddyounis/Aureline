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

## What ships in this revision

- `src/lib.rs` — module surface covering `hooks`, `zones`, `input_path`,
  `render_path`, `frame_timing`, `fixture_scene`, `capabilities`, `trace`.
  The hook names are the ADR 0002 vocabulary verbatim; the zone set is
  the four placeholder zones the spike composites.
- `src/bin/shell_spike.rs` — binary that runs the fixture scene and
  either prints the resulting JSON or writes it under a directory. The
  binary runs headless in this revision; the window-wiring task will
  swap the headless backend for a native `winit` surface behind the
  same seams.
- `tests/fixture_repeatability.rs` — asserts that two runs of the same
  fixture scene with the same deterministic clock produce byte-stable
  capability manifests and trace samples.

Composition-path notes live at
[`docs/design/shell_spike_composition_notes.md`](../../docs/design/shell_spike_composition_notes.md).

## Running

From the workspace root, after bootstrap:

```
cargo run --bin shell_spike -- --print
cargo run --bin shell_spike -- --scene-only
cargo run --bin shell_spike -- --emit-artifacts artifacts/render
cargo test -p aureline-shell-spike
```

The committed artifacts under `artifacts/render/spike_capabilities.json`
and `artifacts/render/spike_trace_samples/*.json` are regenerated from
the last command; commit the regenerated files when the fixture scene
or capability schema changes intentionally.

## Work packages
- WP-01 (Core shell and renderer)
