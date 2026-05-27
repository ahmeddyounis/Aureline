# Stabilize task discovery, launch profiles, rerun-last behavior, and task-event truth — M4 reviewer artifact

This artifact summarizes the checked-in stable task-event truth
packet for release reviewers. The canonical packet is
[`stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json`](./stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](../../../docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md).

## What the packet promises

For each of the four task-event lanes (`local_lane`,
`remote_helper_lane`, `notebook_lane`, `imported_provider_lane`)
the packet certifies:

- One `task_event_truth_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every launch wedge:
  `task_discovery`, `launch_profile`, `rerun_last`, `task_event`.
  Each row binds `auto_narrow_on_wedge_admission_gap` automation
  against `conformance_suite_evidence`.
- Six `envelope_field_binding` rows covering every canonical
  task-event envelope field: `event_id`, `execution_context_ref`,
  `adapter_identity`, `provider_identity`, `confidence_flag`,
  `fallback_flag`. Each row binds
  `auto_narrow_on_envelope_field_gap` automation against
  `automated_functional_evidence`.
- Four `surface_binding` rows covering every downstream consumer
  surface: `problems`, `output_channel`, `evidence_export`,
  `rerun_surface`. Each row binds
  `auto_narrow_on_downstream_surface_gap` automation against
  `conformance_suite_evidence`.
- One `additive_detail_preservation` row attesting
  `additive_detail_preserved: true` with
  `auto_narrow_on_additive_detail_dropped` automation, so
  Local, remote/helper, notebook, and imported-provider runs all
  serialize into one task-event vocabulary with additive detail
  preserved rather than flattened into display text.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:local:task_event_lineage`) so task-event envelopes,
  Problems, output-channel, evidence-export, and rerun surfaces all
  cite one stable lineage object.

Eleven consumer projections (`editor_run_surface`, `task_panel`,
`problems_panel`, `output_channel`, `evidence_export`,
`rerun_surface`, `cli_headless`, `support_export`,
`release_proof_index`, `help_about`, `conformance_dashboard`)
preserve the packet id and every vocabulary verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw command lines, raw
process env bytes, raw capsule bodies, secrets, or ambient
credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior/`](../../../fixtures/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior/)
exercises six narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its
   evidence is `evidence_unbound`. The packet demotes to
   `blocks_stable` with `missing_evidence_class` +
   `launch_stable_with_unbound_binding` findings.
2. `missing_envelope_field_for_launch_stable_blocks_stable.json` —
   the local lane drops the `fallback_flag` envelope-field binding.
   `blocks_stable` with `missing_envelope_field_coverage`.
3. `additive_detail_admits_flattening_blocks_stable.json` — the
   local lane's additive-detail row stops attesting that additive
   detail is preserved. `blocks_stable` with
   `additive_detail_row_admits_flattening` and
   `missing_additive_detail_preservation`.
4. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
5. `projection_collapses_envelope_field_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the envelope-field
   vocabulary. `blocks_stable` with
   `envelope_field_vocabulary_collapsed`.
6. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw command lines or env bytes past the
   boundary. `blocks_stable` with `raw_source_material_present`.

## Source-of-truth pointers

| Lane / Path | Source of truth |
|---|---|
| Schema | [`schemas/runtime/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json`](../../../schemas/runtime/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json) |
| Rust contract | [`crates/aureline-runtime/src/stabilize_task_discovery_launch_profiles_rerun_last_behavior/`](../../../crates/aureline-runtime/src/stabilize_task_discovery_launch_profiles_rerun_last_behavior/) |
| Integration test | [`crates/aureline-runtime/tests/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.rs`](../../../crates/aureline-runtime/tests/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.rs) |
| Generator | [`tools/regenerate_stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.py`](../../../tools/regenerate_stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.py) |
| Reviewer doc | [`docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](../../../docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md) |
