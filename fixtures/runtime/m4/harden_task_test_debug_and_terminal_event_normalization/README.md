# harden_task_test_debug_and_terminal_event_normalization fixture corpus

Fixture corpus for the M4 stable event-normalization truth packet (`schemas/runtime/harden_task_test_debug_and_terminal_event_normalization_truth.schema.json`).

Each fixture is an `EventNormalizationTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, wedge, envelope-field, source-kind, lifecycle-event, consumer-surface-binding, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-terminal/tests/harden_task_test_debug_and_terminal_event_normalization_truth_packet.rs` load each case and assert that `EventNormalizationTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — All four event-normalization lanes (task, test, debug, terminal) carry one `event_normalization_quality` row at `launch_stable` plus the full four-wedge admission coverage (envelope_canonicalization, source_kind_negotiation, lifecycle_normalization, export_preservation), the full ten envelope-field bindings, the full five source-kind bindings, the full nine lifecycle-event bindings, the full nine consumer-surface bindings, a raw_payload_retention_attestation row, and a lineage_admission row binding `execution_context_id`. All twelve required consumer projections preserve the packet verbatim.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — The task lane's quality row claims `launch_stable` while its evidence is `evidence_unbound`; the packet blocks the stable claim.
- `missing_source_kind_for_launch_stable_blocks_stable.json` — The task lane claims `launch_stable` but the `bazel_bep` source-kind binding is missing; the packet blocks the stable claim.
- `missing_lifecycle_event_for_launch_stable_blocks_stable.json` — The test lane claims `launch_stable` but the `test_case_finished` lifecycle binding is missing; the packet blocks the stable claim.
- `retention_admits_flattening_blocks_stable.json` — The task lane's raw-payload retention attestation stops attesting that source_kind, confidence, and raw payload retention are preserved; the packet blocks the stable claim because imported, heuristic, and native event streams must not be flattened into one undifferentiated ledger.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The task lane's quality row narrows to `launch_stable_below` but drops its disclosure ref; the packet blocks the stable claim.
- `projection_collapses_source_kind_vocabulary_blocks_stable.json` — The `help_about` consumer projection drops the source-kind vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — The task lane's quality row admits raw command lines, env bytes, scrollback bodies, or capsule bodies past the boundary; the packet blocks the stable claim because raw runtime material must never leak through the event-normalization boundary.
