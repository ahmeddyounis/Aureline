# stabilize_execution_context_resolver fixture corpus

Fixture corpus for the M4 stable execution-context resolver truth packet (`schemas/runtime/stabilize_execution_context_resolver_truth.schema.json`).

Each fixture is a `StabilizeExecutionContextResolverTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, surface-binding, resolver-state, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/stabilize_execution_context_resolver_truth_packet.rs` load each case and assert that `StabilizeExecutionContextResolverTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — All four execution-context lanes (local, remote_helper, container, managed) carry one `execution_context_resolution_quality` row at `launch_stable` plus the full ten-surface binding coverage, the full eight-state admission coverage, target_admission, restore_rerun_honesty, capability_skew_admission, and lineage_admission rows binding `execution_context_id`. All eight required consumer projections preserve the packet verbatim.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — The local lane's quality row claims `launch_stable` while its evidence class is `evidence_unbound`; the packet blocks the stable claim.
- `missing_state_admission_for_launch_stable_blocks_stable.json` — The local lane claims `launch_stable` but the `restore_no_rerun` state admission is missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The local lane's quality row narrows to `launch_stable_below` but drops its disclosure ref; the packet blocks the stable claim.
- `projection_collapses_resolver_state_vocabulary_blocks_stable.json` — The `help_about` consumer projection drops the resolver-state vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — The local lane's quality row admits raw command lines, env bytes, or capsule bodies past the boundary; the packet blocks the stable claim because raw runtime material must never leak through the execution-context boundary.
