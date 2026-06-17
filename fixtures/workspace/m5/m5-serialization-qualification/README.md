# Fixtures: M5 serialization qualification

This directory contains fixture metadata for the `m5_serialization_qualification` packet.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-serialization-qualification.json`

It is the qualification layer above the serialization-and-restore matrix at:

`artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`

## Coverage

- `remembered_state`, `restore_fidelity`, `portable_state_review`, `migration_remap`, and
  `missing_surface_continuity` are the only claimed families, and each carries at least one
  qualification row — no family inherits a fidelity from an adjacent one.
- Each row is keyed by `(family, profile, deployment_mode)` and carries its own proof, so a profile
  is never marked green because a nearby profile passed a superficially similar restore flow. Each
  row ingests the serialization matrix's published fidelity as its `matrix_claim`, binds to the
  canonical matrix packet via `matrix_packet_ref`, and points at its source matrix row via
  `matrix_row_ref`. The published fidelity can never exceed the matrix claim.
- Every row covers all seven drills — `schema_jump`, `foreign_package`, `display_topology`,
  `missing_extension`, `placeholder_continuity`, `accessibility`, and `downgrade` — exactly once,
  and any drill that ran carries an evidence ref.
- Drill outcomes cover `passed`, `narrowed` (`migration_remap` schema-jump, `missing_surface`
  missing-extension), `failed` (`migration_remap` and `missing_surface` on the harder profiles),
  and `not_run` (the withheld companion/browser downgrade drill). Evidence freshness covers
  `current`, `aging` (companion/browser portable-state), `expired` (managed-fleet migration), and
  `missing` (companion/browser missing-surface).
- Published fidelity covers `exact_restore`, `compatible_restore`, `layout_only`, and
  `manual_review`, and the claim-publication decision covers `published`, `narrowed`, and
  `withheld`. Deployment modes cover `desktop`, `managed_fleet`, and `companion_browser`.
- The four downgrade reasons — `matrix_narrowed`, `evidence_stale`, `drill_narrowed`, and
  `drill_failed` — are each exercised by at least one row, and the five recovery paths —
  `rerun_drills`, `refresh_evidence`, `adopt_matrix_narrowing`, `withhold_claim`, and `none` — are
  each exercised.
- The gate is exercised in every direction: the desktop-stable `remembered_state`,
  `restore_fidelity`, and `portable_state_review` rows publish a full `exact_restore` claim (exact
  matrix claim, current evidence, all drills passed), proving the qualifier is not a blanket
  downgrade; the managed-fleet `restore_fidelity` row adopts the matrix narrowing alone; the
  companion/browser `portable_state_review` row narrows on aging evidence; the desktop-beta
  `migration_remap` row narrows on a schema-jump drill; the desktop-stable
  `missing_surface_continuity` row narrows to slot-preserving placeholders; and the managed-fleet
  `migration_remap` and companion/browser `missing_surface_continuity` rows are withheld with no
  qualified class. Each row's `published_fidelity`, `claim_publication`, `downgrade_reasons`, and
  `downgrade_path` equal the recomputed gate, so the docs/help, support-export,
  companion/browser-handoff, release-center, and shiproom surfaces ingest one packet and a narrowed
  row cannot stay green by inertia.
