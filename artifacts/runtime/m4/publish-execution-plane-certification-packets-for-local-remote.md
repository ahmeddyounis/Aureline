# Execution-plane certification packets — M4 reviewer artifact

This artifact summarizes the checked-in stable execution-plane
certification truth packet for release reviewers. The canonical packet
is
[`publish_execution_plane_certification_packets_for_local_remote_truth_packet.json`](./publish_execution_plane_certification_packets_for_local_remote_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md`](../../../docs/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md).

## What the packet promises

For each of the three execution-plane lanes (`local_lane`,
`remote_helper_lane`, `enterprise_network_lane`) the packet certifies:

- One `execution_plane_certification_quality` row at `launch_stable` with
  `release_evidence_review` evidence and `auto_block_on_missing_evidence` automation.
- Eleven `surface_binding` rows covering every required surface. Each binds
  `auto_narrow_on_lineage_break` automation.
- One `target_admission` row attesting requested-vs-materialized target identity.
- Five `route_admission` rows covering every route state. Each binds
  `auto_narrow_on_route_drift` automation.
- One `restore_rerun_honesty` row attesting `restore_preserves_no_rerun: true`.
- Three `reconnect_admission` rows covering every reconnect state. Each binds
  `auto_narrow_on_reconnect_gap` automation.
- Three `degraded_helper_admission` rows covering every degraded-helper state.
- Two `artifact_provenance_admission` rows covering every provenance state.
- One `lineage_admission` row binding `execution_context_id`.

Eight consumer projections preserve the packet id and every vocabulary verbatim.

## Promotion state

`stable` across all three lanes, with zero validation findings.

## Narrowed-below-stable drills

The fixture corpus exercises ten narrowing / blocking postures. See the
reviewer contract doc for the full table.

## How to consume

- Rust: `aureline_runtime::current_stable_execution_plane_truth_packet`.
- Cross-tool: load the schema and the packet at this directory.
