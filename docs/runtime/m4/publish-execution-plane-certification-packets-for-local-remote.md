# Execution-plane certification packets for local, remote/helper, and enterprise-network rows — M4 reviewer contract

This document is the reviewer-facing contract for the stable
execution-plane certification truth packet. The canonical packet is
[`publish_execution_plane_certification_packets_for_local_remote_truth_packet.json`](../../../artifacts/runtime/m4/publish_execution_plane_certification_packets_for_local_remote_truth_packet.json);
the human-readable reviewer artifact is at
[`artifacts/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md`](../../../artifacts/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md).

## What the packet promises

For each of the three execution-plane lanes (`local_lane`,
`remote_helper_lane`, `enterprise_network_lane`) the packet certifies:

- One `execution_plane_certification_quality` row at `launch_stable`
  with `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Eleven `surface_binding` rows covering every required surface:
  `terminal`, `task`, `test`, `debug`, `artifact`,
  `request_workspace`, `preview`, `cli_headless`, `docs_help`,
  `support_export`, `conformance_dashboard`. Each binds
  `auto_narrow_on_lineage_break` automation.
- One `target_admission` row attesting requested-vs-materialized
  target identity with `auto_narrow_on_target_unreachable` automation.
- Five `route_admission` rows covering every required route state:
  `local_route`, `remote_helper_route`, `enterprise_network_route`,
  `route_drift`, `blocked_target`. Each binds
  `auto_narrow_on_route_drift` automation.
- One `restore_rerun_honesty` row attesting
  `restore_preserves_no_rerun: true` with
  `auto_narrow_on_silent_rerun` automation so restore never silently
  reruns tasks, reattaches debuggers, or reuses drifted targets.
- Three `reconnect_admission` rows covering every reconnect state:
  `reconnect_required`, `reconnect_honest`, `restore_no_rerun`. Each
  binds `auto_narrow_on_reconnect_gap` automation.
- Three `degraded_helper_admission` rows covering every degraded-helper
  state: `capability_degraded`, `helper_offline`, `helper_skew`. Each
  binds `auto_narrow_on_degraded_helper` automation.
- Two `artifact_provenance_admission` rows covering every provenance
  state: `provenance_tracked`, `provenance_missing`. Each binds
  `auto_narrow_on_artifact_provenance_gap` automation.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g. `exec:m4:local:lineage`) so
  event streams, support packets, approval tickets, and evidence
  exports cite one stable lineage object. Binds
  `auto_narrow_on_lineage_break` against
  `automated_functional_evidence`.

Eight consumer projections (`editor_run_surface`, `terminal_pane`,
`task_panel`, `cli_headless`, `support_export`, `release_proof_index`,
`help_about`, `conformance_dashboard`) preserve the packet id and
every vocabulary verbatim.

## Promotion state

`stable` across all three lanes, with zero validation findings. The
support export bundles the packet without raw command lines, raw
process environment bytes, raw secrets, raw capsule bodies, or ambient
credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/publish_execution_plane_certification_packets_for_local_remote`](../../../fixtures/runtime/m4/publish_execution_plane_certification_packets_for_local_remote)
exercises ten narrowing / blocking postures:

| fixture | what it proves |
|---|---|
| `launch_stable_with_unbound_evidence_blocks_stable.json` | A launch_stable quality row with `evidence_unbound` is refused (`missing_evidence_class`, `launch_stable_with_unbound_binding`). |
| `missing_route_admission_for_launch_stable_blocks_stable.json` | The local_lane dropping its `blocked_target` route admission triggers `missing_route_admission`. |
| `missing_reconnect_admission_for_launch_stable_blocks_stable.json` | The remote_helper_lane dropping its `restore_no_rerun` reconnect admission triggers `missing_reconnect_admission`. |
| `missing_degraded_helper_admission_for_launch_stable_blocks_stable.json` | The enterprise_network_lane dropping its `helper_skew` degraded-helper admission triggers `missing_degraded_helper_admission`. |
| `missing_artifact_provenance_admission_for_launch_stable_blocks_stable.json` | The local_lane dropping its `provenance_missing` artifact-provenance admission triggers `missing_artifact_provenance_admission`. |
| `lineage_admission_missing_execution_context_id_blocks_stable.json` | Dropping the `execution_context_id_binding` on the local_lane lineage row triggers `lineage_admission_missing_execution_context_id` and `missing_lineage_admission`. |
| `narrowed_row_missing_disclosure_ref_blocks_stable.json` | A `launch_stable_below` row without a disclosure ref triggers `narrowed_row_missing_disclosure_ref`. |
| `projection_collapses_route_state_vocabulary_blocks_stable.json` | A Help/About projection that collapses the route-state vocabulary triggers `route_state_vocabulary_collapsed`. |
| `raw_source_material_blocks_stable.json` | Admitting raw command lines or process environment bytes past the boundary triggers `raw_source_material_present`. |

## How to consume

- Rust: import
  `aureline_runtime::current_stable_execution_plane_truth_packet`.
- Cross-tool: load the schema at
  `schemas/runtime/publish_execution_plane_certification_packets_for_local_remote.schema.json`
  and the packet at this directory.
- Surfaces (terminal pane, task panel, test explorer, debug session,
  artifact viewer, request editor, preview surface, CLI/headless,
  support export, release proof index, Help/About, conformance
  dashboard) MUST read this packet verbatim and MUST NOT mint local
  copies or fork runtime semantics.
