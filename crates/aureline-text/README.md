# aureline-text

## Purpose
Foundational text primitives: encoding handling, normalization, grapheme and
word segmentation, and the inputs that drive shaping inside the renderer.

## Protected-path status
Protected. Source-fidelity, encoding correctness, and segmentation invariants
flow from this crate; downstream correctness depends on it.

## Allowed dependencies
- No internal dependencies. This crate is a leaf foundation.
- Third-party Unicode/text crates are permitted.

## Canonical owner path
`crates/aureline-text/`

## Work packages
- WP-01 (Core shell and renderer)
- WP-02 (Editor and buffer core)
