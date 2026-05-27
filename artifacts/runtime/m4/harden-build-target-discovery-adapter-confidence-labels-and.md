# Build-target discovery, adapter-confidence labels, and target-graph snapshots — M4 reviewer artifact

This artifact summarizes the checked-in stable build-target hardening
truth packet for release reviewers. The canonical packet is
[`harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json`](./harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/harden-build-target-discovery-adapter-confidence-labels-and.md`](../../../docs/runtime/m4/harden-build-target-discovery-adapter-confidence-labels-and.md).

## What the packet promises

For each of the four build-target hardening lanes (`run_lane`,
`test_lane`, `debug_lane`, `target_graph_snapshot_lane`) the packet
certifies:

- One `build_target_hardening_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every required wedge:
  `build_target_discovery_truth`, `adapter_confidence_label_truth`,
  `target_graph_snapshot_truth`,
  `cross_surface_target_parity_truth`. The
  `cross_surface_target_parity_truth` row attests
  `cross_surface_target_parity_attested: true` with
  `auto_narrow_on_cross_surface_target_drift` automation so run, test,
  debug, and snapshot surfaces never silently fork local target
  identity.
- Six `discovery_source_admission` rows covering every required source:
  `native_protocol`, `structured_adapter`, `heuristic_parser`,
  `imported_metadata`, `user_declared`, `resolver_unavailable`. Each
  row binds `auto_narrow_on_discovery_source_gap` automation against
  `conformance_suite_evidence` so the lane always discloses where each
  target binding originated.
- Five `discovery_freshness_admission` rows covering every freshness
  class: `fresh_probe`, `recent_within_session`, `imported_authoritative`,
  `stale_imported`, `unknown`. Each row binds
  `auto_narrow_on_discovery_freshness_gap` against
  `failure_recovery_drill_evidence` so freshness never collapses into
  a single "ok" badge.
- Five `adapter_confidence_label_admission` rows covering every
  adapter-confidence label: `adapter_authoritative_match`,
  `adapter_probed_consistent`, `adapter_probed_divergent`,
  `adapter_inferred_from_session`, `adapter_unreachable`. Each row
  binds `auto_narrow_on_adapter_confidence_label_gap` against
  `conformance_suite_evidence` so labels survive verbatim into export
  and support packets.
- Five `target_graph_snapshot_admission` rows covering every snapshot
  class: `live_snapshot`, `session_cached_snapshot`,
  `imported_snapshot`, `archived_snapshot`, `snapshot_unavailable`.
  Each row binds `auto_narrow_on_target_graph_snapshot_gap` against
  `fixture_repo_evidence` so snapshot provenance stays explicit
  through restore and replay.
- Seven `consumer_surface_binding` rows covering every consumer
  surface: `run_surface`, `test_surface`, `debug_surface`,
  `cli_headless_inspect`, `support_export`, `help_about`,
  `conformance_dashboard`. Each row binds
  `auto_narrow_on_consumer_surface_gap` automation so a missing
  consumer surface narrows below stable rather than silently
  inheriting an adjacent green row.
- One `lineage_admission` row binding a stable
  `execution_context_id` so emitted target-graph snapshots, support
  packets, and evidence exports thread one lineage object.

## Required consumer projections

Seven consumer projections (`run_surface`, `test_surface`,
`debug_surface`, `cli_headless_inspect`, `support_export`,
`help_about`, `conformance_dashboard`) preserve the lane, row-class,
support-class, wedge, discovery-source, discovery-freshness,
adapter-confidence label, target-graph snapshot, consumer-surface,
known-limit, downgrade-automation, and evidence-class vocabularies
verbatim. Each projection confirms JSON export and excludes raw
private material and ambient authority.

## How to verify

- `cargo test -p aureline-runtime --test harden_build_target_discovery_adapter_confidence_labels_and_truth_packet`
  loads every fixture and asserts the materialization expectations.
- `python3 tools/regenerate_harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.py`
  regenerates the artifact and fixture corpus deterministically.

## Boundary discipline

The packet never admits raw discovery payloads, raw adapter handshake
bodies, raw command lines, raw process environment bytes, raw
secrets, or ambient credentials past the boundary. Every row attests
`raw_source_material_excluded`, `secrets_excluded`, and
`ambient_authority_excluded`.
