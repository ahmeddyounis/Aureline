# aureline-render

## Purpose
GPU-accelerated rendering primitives for the desktop shell: frame scheduling,
scene composition, and the pipeline that drives every painted surface.

## Protected-path status
Protected. Owns the rendering hot path; latency, frame-pacing, and color/IME
correctness obligations attach here.

## Allowed dependencies
- May depend on `aureline-text` for shaping/glyph metrics.
- May depend on `aureline-telemetry` for hot-path instrumentation.
- Must not depend on `aureline-buffer`, `aureline-vfs`, `aureline-rpc`, or
  `aureline-shell-spike`.

## Canonical owner path
`crates/aureline-render/`

## Work packages
- WP-01 (Core shell and renderer)
