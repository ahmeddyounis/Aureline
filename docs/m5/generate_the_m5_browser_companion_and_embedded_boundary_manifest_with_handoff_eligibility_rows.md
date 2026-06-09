# M5 Browser, Companion, and Embedded-Boundary Manifest with Handoff Eligibility Rows

This document describes the canonical manifest that governs handoff eligibility
between browser surfaces, companion surfaces, and embedded-boundary surfaces for
Milestone 5.

## Scope

The manifest covers:
- Browser to desktop handoffs
- Companion to desktop handoffs
- Embedded-boundary to external browser handoffs
- Cross-surface eligibility state and gap reasons

## Usage

The checked-in artifact at
`artifacts/release/m5/generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows.json`
is the canonical source. Downstream docs, Help/About, support exports, and CI
gates should ingest it directly rather than clone status text.

## Verification

Run the protected tests in `crates/aureline-release/tests/` to validate the
typed model against the checked-in artifact and fixtures.
