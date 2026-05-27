# Harden task, test, debug, and terminal event normalization — M4 reviewer artifact

This artifact summarizes the checked-in stable event-normalization
truth packet for release reviewers. The canonical packet is
[`harden_task_test_debug_and_terminal_event_normalization_truth_packet.json`](./harden_task_test_debug_and_terminal_event_normalization_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md`](../../../docs/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md).

## What the packet promises

For each of the four event-normalization lanes (`task_lane`,
`test_lane`, `debug_lane`, `terminal_lane`) the packet certifies:

- One `event_normalization_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every wedge:
  `envelope_canonicalization`, `source_kind_negotiation`,
  `lifecycle_normalization`, `export_preservation`. Each row binds
  `auto_narrow_on_wedge_admission_gap` automation against
  `conformance_suite_evidence`.
- Ten `envelope_field_binding` rows covering every canonical
  envelope field: `event_id`, `workspace_id`, `target_id`,
  `source_kind`, `confidence`, `timestamp`, `execution_context_id`,
  `payload_kind`, `raw_payload_ref`, `provenance`. Each row binds
  `auto_narrow_on_envelope_field_gap` automation against
  `automated_functional_evidence`.
- Five `source_kind_binding` rows covering every canonical source
  kind: `native`, `bsp`, `bazel_bep`, `structured_output`,
  `heuristic_parser`. Each row binds
  `auto_narrow_on_source_kind_gap` automation against
  `conformance_suite_evidence`.
- Nine `lifecycle_event_binding` rows covering every canonical
  lifecycle event: `task_queued`, `target_graph_ready`,
  `task_started`, `progress_updated`, `diagnostic_emitted`,
  `test_case_started`, `test_case_finished`, `artifact_published`,
  `task_finished`. Each row binds
  `auto_narrow_on_lifecycle_event_gap` automation against
  `automated_functional_evidence`.
- Nine `consumer_surface_binding` rows covering every downstream
  consumer surface: `editor_run_surface`, `task_panel`,
  `test_runner_surface`, `debug_surface`, `terminal_pane`,
  `cli_headless`, `ai_tool_surface`, `review_surface`,
  `support_export`. Each row binds
  `auto_narrow_on_consumer_surface_gap` automation against
  `conformance_suite_evidence`.
- One `raw_payload_retention_attestation` row attesting
  `attests_raw_payload_retained: true` with
  `auto_narrow_on_export_flattening` automation, so replay, export,
  and support packets preserve `source_kind`, `confidence`, and
  the adapter raw payload reference rather than flattening
  imported, heuristic, and native event streams into one
  undifferentiated execution ledger.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:task:event_normalization`) so envelopes, AI tools,
  review, support export, and downstream consumer surfaces all
  cite one stable lineage object.

Twelve consumer projections (`editor_run_surface`, `task_panel`,
`test_runner_surface`, `debug_surface`, `terminal_pane`,
`cli_headless`, `ai_tool_surface`, `review_surface`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) preserve the packet id and every
vocabulary verbatim.

## How the packet refuses unsafe stable claims

The validator emits typed blocker findings whenever a stable claim
would mask a runtime gap. The seven fixture cases in
[`fixtures/runtime/m4/harden_task_test_debug_and_terminal_event_normalization/`](../../../fixtures/runtime/m4/harden_task_test_debug_and_terminal_event_normalization/)
exercise the most important ones:

- `baseline_stable.json` — full coverage of all four lanes, all
  required rows, all twelve required consumer projections.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — a
  lane that claims `launch_stable` while leaving its evidence
  unbound is refused.
- `missing_source_kind_for_launch_stable_blocks_stable.json` — a
  lane that drops the `bazel_bep` source-kind binding is refused.
- `missing_lifecycle_event_for_launch_stable_blocks_stable.json`
  — a lane that drops the `test_case_finished` lifecycle binding
  is refused.
- `retention_admits_flattening_blocks_stable.json` — a
  retention attestation that admits flattening
  `source_kind`/`confidence`/raw payload retention is refused.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — a
  row narrowed below `launch_stable` without a disclosure ref is
  refused.
- `projection_collapses_source_kind_vocabulary_blocks_stable.json`
  — a downstream surface projection that collapses the
  source-kind vocabulary is refused.
- `raw_source_material_blocks_stable.json` — a row that admits
  raw command lines, raw env bytes, raw scrollback bodies, or raw
  capsule bodies past the boundary is refused.

## Where the packet is the truth

- Editor run surface, task panel, test runner surface, debug
  surface, terminal pane, CLI/headless inspector, AI tool surface,
  review surface, support export, release proof index, Help/About
  proof card, and conformance dashboard all read this packet
  verbatim — they do not paraphrase, collapse, or fork the
  canonical envelope, lifecycle, or source-kind vocabularies.

## How to regenerate

Run
[`tools/regenerate_harden_task_test_debug_and_terminal_event_normalization_truth_packet.py`](../../../tools/regenerate_harden_task_test_debug_and_terminal_event_normalization_truth_packet.py)
to rebuild the artifact and the fixture corpus from the same seed
the Rust unit-test sample input uses. The integration tests in
[`crates/aureline-terminal/tests/harden_task_test_debug_and_terminal_event_normalization_truth_packet.rs`](../../../crates/aureline-terminal/tests/harden_task_test_debug_and_terminal_event_normalization_truth_packet.rs)
load every fixture and assert that
`EventNormalizationTruthPacket::materialize` agrees.
