# Fixtures: Support Center information architecture

This directory contains fixture metadata for the `m5_support_center_ui` packet.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-support-center-ui.json`

It is the one authoritative Support Center layout contract; the typed model and fail-closed
presentation gate live in the `aureline-support` crate (`m5_support_center_ui`).

## Coverage

- The three layout regions — `left_nav`, `center`, and `right_inspector` — are each present, carry a
  unique keyboard order, and satisfy every accessibility invariant (`keyboard_complete`,
  `high_contrast_parity`, `reduced_motion_safe`).
- The one shared inspector persists across modules and declares all four facets — `build` (wired),
  `policy` (degraded), `residency` (unwired), and `export` (wired) — exercising every facet
  availability state.
- All twelve Support Center modules — `doctor`, `safe_mode`, `bisect`, `performance`, `language`,
  `index`, `ai_usage`, `crash`, `network`, `artifacts`, `issue_report_crash_intake`, and
  `support_bundle_export_preview` — carry exactly one nav entry, named from the one module registry.
  Each entry defers per-module readiness to its matrix row and reuses at least one existing source
  (`finding_codes`, `crash_ids`, `install_advisory_rows`, `schema_registry_state`).
- The four nav sections (`diagnose`, `recover`, `inspect`, `intake_export`) and the four center-surface
  kinds (`diagnosis_cards`, `recovery_actions`, `inspector_readout`, `intake_and_export`) are each
  exercised.
- Published presentation covers `presented` (`bisect`, `crash`, `issue_report_crash_intake`),
  `narrowed` (`doctor`, `safe_mode`, `performance`, `language`), and `withheld` (`index`, `ai_usage`,
  `network`, `artifacts`, `support_bundle_export_preview`).
- The three downgrade reasons — `accessibility_unmet` (`network`), `inspector_facet_degraded` (the
  policy-dependent entries), and `inspector_facet_unwired` (the residency-dependent entries) — are each
  exercised, and the three recovery paths — `restore_accessibility`, `restore_inspector_facet`, and
  `none` — are each exercised.
- The gate is exercised in every direction: three entries present cleanly with all invariants met and
  every required facet wired, proving the gate is not a blanket withhold; policy-dependent entries
  narrow on the degraded facet while keeping actions offered; residency-dependent entries are withheld
  on the unwired facet; and `network` is withheld for missing reduced-motion-safe transitions, with
  accessibility restored before the degraded policy facet. Each entry's `presentation`,
  `downgrade_reasons`, and `recovery_path` equal the recomputed gate, so the desktop-shell,
  CLI/headless, docs-help, and support-export surfaces ingest one registry and a withheld module cannot
  stay navigable by inertia.
