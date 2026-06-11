# Fixtures: Doctor/repair/container maturity matrix

This directory contains fixture metadata for the
`doctor_repair_container_maturity_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/doctor-repair-container-maturity-matrix.json`

## Coverage

- `project_doctor`, `guided_repair`, and `container_boundary` are the only claimed
  recovery capabilities, and every (capability, profile) cell across
  `local_workspace`, `remote_ssh`, `container`, and `devcontainer` carries exactly
  one row — no cell inherits trust from an adjacent one.
- Each cell carries its own feature scorecard, diagnosis-latency corpus, rollback
  path, and compatibility story.
- Published maturity covers `certified`, `provisional`, `underqualified`, and
  `unsupported`, and the narrowing action covers `none`, `narrow_to_provisional`,
  `narrow_to_underqualified`, and `withhold_from_publication`.
- Evidence freshness covers `current`, `stale`, `expired`, and `unknown`.
- Reversal classes cover `reversible`, `checkpointed`, `irreversible`, and
  `not_applicable`, and every `guided_repair` cell carries a concrete reversal
  class.
- Support parity covers `full`, `desktop_cli`, `desktop_only`, and `unavailable`.
- Blocking reasons cover `stale`, `engine_unavailable`, `latency_slo_breached`,
  `missing_proof_corpus`, `missing_rollback_path`, and `boundary_unverified`.
- The promotion gate is exercised in both directions: clean cells promote to a
  full certification, while stale, engine-unavailable, latency-breached, expired,
  and evidence-missing cells narrow automatically. Each row's `published_maturity`
  and `narrowing_action` equal the recomputed gate decision, so release tooling can
  prove underqualified cells narrow before publication.
