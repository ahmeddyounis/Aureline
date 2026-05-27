# stabilize_task_discovery_launch_profiles_rerun_last_behavior fixture corpus

Fixture corpus for the M4 stable task-discovery / launch-profile / rerun-last / task-event truth packet (`schemas/runtime/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json`).

Each fixture is a `TaskEventTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, wedge, envelope-field, downstream-surface, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.rs` load each case and assert that `TaskEventTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — All four task-event lanes (local, remote_helper, notebook, imported_provider) carry one `task_event_truth_quality` row at `launch_stable` plus the full four-wedge admission coverage (task_discovery, launch_profile, rerun_last, task_event), the full six envelope-field bindings (event_id, execution_context_ref, adapter_identity, provider_identity, confidence_flag, fallback_flag), the full four downstream surface bindings (problems, output_channel, evidence_export, rerun_surface), an additive_detail_preservation row, and a lineage_admission row binding `execution_context_id`. All eleven required consumer projections preserve the packet verbatim.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — The local lane's quality row claims `launch_stable` while its evidence is `evidence_unbound`; the packet blocks the stable claim.
- `missing_envelope_field_for_launch_stable_blocks_stable.json` — The local lane claims `launch_stable` but the `fallback_flag` envelope-field binding is missing; the packet blocks the stable claim.
- `additive_detail_admits_flattening_blocks_stable.json` — The local lane's additive-detail row stops attesting that additive detail is preserved; the packet blocks the stable claim because local, remote/helper, notebook, and imported-provider runs must preserve additive detail rather than flatten it into display text.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The local lane's quality row narrows to `launch_stable_below` but drops its disclosure ref; the packet blocks the stable claim.
- `projection_collapses_envelope_field_vocabulary_blocks_stable.json` — The `help_about` consumer projection drops the envelope-field vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — The local lane's quality row admits raw command lines, env bytes, or capsule bodies past the boundary; the packet blocks the stable claim because raw runtime material must never leak through the task-event boundary.
