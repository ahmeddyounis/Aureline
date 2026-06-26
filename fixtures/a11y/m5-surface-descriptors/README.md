# M5 Accessibility-Surface Descriptor Fixtures

These fixtures are valid, export-safe descriptor catalogs that exercise the
auto-narrowing behavior the canonical support export keeps green. Each one keeps a
descriptor for every claimed custom surface family, the shared and descriptor
vocabulary sets intact, and the conformance-review, consumer-projection, and
release-posture invariants satisfied — the difference is which surface narrows and
why. They are minted from the same seed builder as the canonical export by
`aureline_shell_m5_surface_descriptors`.

## bridge_degraded.json

The terminal-canvas descriptor's OS accessibility bridge has gone `partial`. The
descriptor narrows from Stable to Preview, sets its bridge `degradation_reason` to
`partial_tree_mapping`, drops its non-visual fidelity to `degraded_accessible`, and
keeps its `bridge_partial_or_stale` downgrade trigger. Every other surface stays at
its canonical qualification with a `bridged_active` mapping. Demonstrates that a
degraded bridge is disclosed and narrows the claim rather than implying silent
screen-reader completeness.

## proof_stale_narrowed.json

The editor-canvas descriptor's assistive-tech proof has gone stale. The descriptor
narrows from Stable to Beta and keeps its `proof_stale` downgrade trigger, so the
gap is disclosed while the surface stays present with its full semantic structure,
focus order, and bridge mapping. Demonstrates that stale proof narrows the claim
rather than hiding the surface.
