# Artifact manager, preview/runtime inspectors, and evidence export — M4 reviewer artifact

This artifact summarizes the checked-in stable artifact-manager /
preview-runtime-inspector / evidence-export truth packet for release
reviewers. The canonical packet is
[`stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json`](./stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/stabilize-the-artifact-manager-preview-runtime-inspectors-and.md`](../../../docs/runtime/m4/stabilize-the-artifact-manager-preview-runtime-inspectors-and.md).

## What the packet promises

For each of the four lanes (`artifact_manager_lane`,
`preview_runtime_inspector_lane`, `signal_slice_lane`,
`evidence_export_lane`) the packet certifies:

- One `evidence_export_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every required wedge:
  `artifact_chronology_replay_truth`, `signal_slice_identity_truth`,
  `evidence_export_review_truth`,
  `cross_surface_evidence_lineage_truth`. The
  `cross_surface_evidence_lineage_truth` row attests
  `cross_surface_evidence_lineage_attested: true` with
  `auto_narrow_on_cross_surface_evidence_lineage_drift` automation so
  artifact-manager, inspector, and export surfaces never silently fork
  local artifact / slice identity.
- Four `signal_slice_kind_admission` rows covering every required
  kind: `logs_slice`, `metrics_slice`, `traces_slice`,
  `test_artifact_slice`. Each row binds
  `auto_narrow_on_signal_slice_kind_gap` automation against
  `conformance_suite_evidence` so the lane always discloses every
  slice kind it observes.
- Six `slice_freshness_admission` rows covering every freshness state:
  `live_stream`, `buffered_replay`, `cached_snapshot`,
  `imported_evidence`, `truncated_view`, `exported_copy`. Each row
  binds `auto_narrow_on_slice_freshness_gap` against
  `failure_recovery_drill_evidence` so freshness never collapses into a
  single "ok" badge and users never mistake an exported copy for live
  runtime truth.
- Five `replay_chronology_admission` rows covering every chronology
  state: `recorded`, `not_recorded`, `unsupported`,
  `restart_with_recording_available`, `partially_recorded`. Each row
  binds `auto_narrow_on_replay_chronology_gap` against
  `fixture_repo_evidence` so chronology posture stays explicit through
  restore and replay.
- Five `retention_class_admission` rows covering every retention
  class: `session_only_retention`, `session_plus_window_retention`,
  `policy_bounded_retention`, `archived_retention`,
  `imported_external_retention`. Each row binds
  `auto_narrow_on_retention_class_gap` automation so reviewers can audit
  retention posture without support-only knowledge.
- Seven `consumer_surface_binding` rows covering every consumer
  surface: `artifact_manager_surface`,
  `preview_runtime_inspector_surface`,
  `evidence_export_sheet_surface`, `cli_headless_inspect`,
  `support_export`, `help_about`, `conformance_dashboard`. Each row
  binds `auto_narrow_on_consumer_surface_gap` automation so a missing
  consumer surface narrows below stable rather than silently inheriting
  an adjacent green row.
- One `lineage_admission` row binding a stable
  `execution_context_id` so emitted artifacts, signal slices, evidence
  exports, and support packets thread one lineage object.

## Required consumer projections

Seven consumer projections (`artifact_manager_surface`,
`preview_runtime_inspector_surface`,
`evidence_export_sheet_surface`, `cli_headless_inspect`,
`support_export`, `help_about`, `conformance_dashboard`) preserve the
lane, row-class, support-class, wedge, signal-slice kind,
slice-freshness, replay-chronology state, retention-class,
consumer-surface, known-limit, downgrade-automation, and evidence-class
vocabularies verbatim. Each projection confirms JSON export and
excludes raw private material and ambient authority.

## How to verify

- `cargo test -p aureline-runtime --test stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet`
  loads every fixture and asserts the materialization expectations.
- `python3 tools/regenerate_stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.py`
  regenerates the artifact and fixture corpus deterministically.

## Boundary discipline

The packet never admits raw log bodies, raw trace payloads, raw
test-artifact bytes, raw command lines, raw process environment
bytes, raw secrets, or ambient credentials past the boundary. Every
row attests `raw_source_material_excluded`, `secrets_excluded`, and
`ambient_authority_excluded`.
