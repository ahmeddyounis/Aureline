# Fixtures: M5 target-discovery matrix

This directory contains fixture metadata for the `m5_target_discovery_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/execution/m5/m5-target-discovery.json`

## Coverage

- `build_target`, `notebook_kernel`, `preview_runtime`, `profiler_session`,
  `framework_generator`, `request_runtime`, `api_runtime`, and `incident_rerun` are the
  only claimed execution lanes, and each carries exactly one row — no lane inherits a
  confident target from an adjacent exact one.
- Each lane carries its own selected-target, target-graph, provenance, execution, and
  support-export ref; every changed lane also carries a previous-target and
  discovery-diff ref so the change is reviewable instead of silently replacing the
  current target.
- Published confidence covers `exact`, `structured`, `imported`, `heuristic`, and
  `unresolved`, and the discovery decision covers `publish`, `narrow`,
  `flag_for_review`, and `withhold`.
- Discovery path covers `native_adapter`, `protocol_backed`, `build_event_stream`,
  `structured_import`, `heuristic`, and `undiscovered`; verification covers `verified`,
  `corroborated`, `single_signal`, and `unverified`; exactness covers `exact` and
  `approximate`; change trigger covers `unchanged`, `workspace_changed`,
  `profile_changed`, `build_metadata_changed`, `managed_runtime_changed`, and
  `manual_reselection`; diff review covers `not_applicable`, `pending_review`,
  `reviewed_accepted`, `reviewed_rejected`, and `auto_applied_unreviewed`; target graph
  covers `snapshotted`, `stale_snapshot`, `missing_snapshot`, and `not_applicable`; and
  provenance covers `propagated`, `partial`, `dropped`, and `not_applicable`.
- The six canonical narrowing reasons — `target_unresolved`, `low_verification`,
  `heuristic_fallback`, `unreviewed_target_change`, `provenance_dropped`, and
  `missing_graph_snapshot` — are each exercised by at least one lane.
- The confidence gate is exercised in every direction: the clean `build_target` and
  the reviewed `notebook_kernel` publish exact targets; the `preview_runtime`,
  `profiler_session`, and `api_runtime` lanes narrow to a lower confidence; the
  `framework_generator` and `request_runtime` lanes flag a pending or unreviewed target
  change for review; and the undiscovered `incident_rerun` has its target withheld. The
  `api_runtime` row is the guardrail case — an unverified `native_adapter` target is
  capped at `heuristic` and labelled `approximate` so it cannot masquerade as a
  confident exact native target. Each row's `published_confidence`, `exactness`,
  `discovery_decision`, and `narrowing_reasons` equal the recomputed gate decision, so
  release and desktop/CLI tooling can prove underqualified lanes narrow before
  publication.
