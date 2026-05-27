# Stabilize the debugger host, adapter negotiation, attach/launch flows, and crash isolation — M4 reviewer artifact

This artifact summarizes the checked-in stable debugger-stabilization
truth packet for release reviewers. The canonical packet is
[`stabilize_debugger_host_and_adapter_negotiation_truth_packet.json`](./stabilize_debugger_host_and_adapter_negotiation_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md`](../../../docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md).

## What the packet promises

For each of the four debugger lanes (`local_lane`,
`remote_helper_lane`, `container_lane`, `notebook_bridge_lane`) the
packet certifies:

- One `debugger_stabilization_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every debugger wedge:
  `debugger_host`, `adapter_negotiation`, `attach_launch_flow`,
  `crash_isolation`. Each row binds
  `auto_narrow_on_wedge_admission_gap` automation against
  `conformance_suite_evidence`.
- Six `adapter_descriptor_field_binding` rows covering every
  canonical adapter / backend descriptor field: `adapter_identity`,
  `transport_class`, `launch_attach_scope`,
  `local_vs_remote_support_class`,
  `chronology_replay_capability_class`,
  `notebook_bridge_or_replay_only_limitation`. Each row binds
  `auto_narrow_on_adapter_descriptor_field_gap` automation against
  `automated_functional_evidence`.
- Four `attach_launch_parity_surface_binding` rows covering every
  parity surface: `ui_surface`, `cli_headless`, `support_export`,
  `docs_help`. Each row carries the same attach/launch posture
  (`supported` in the baseline packet) and binds
  `auto_narrow_on_attach_launch_parity_surface_gap` automation
  against `conformance_suite_evidence`.
- Five `crash_isolation_assertion_binding` rows attesting every
  crash-isolation assertion: `bounded_restart_budget`,
  `session_quarantine_admission`,
  `unrelated_language_host_unaffected`,
  `unrelated_terminal_lane_unaffected`,
  `unrelated_debug_session_unaffected`. Each row sets
  `attests_crash_isolation_assertion: true` and binds
  `auto_narrow_on_crash_isolation_assertion_gap` automation against
  `failure_recovery_drill_evidence`.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:local:debugger_lineage`) so debug-session envelopes,
  debug session panels, breakpoint surfaces, watch/locals surfaces,
  crash-loop quarantine banners, evidence exports, and support
  exports all cite one stable lineage object.

Eleven consumer projections (`editor_debug_surface`,
`debug_session_panel`, `breakpoint_surface`, `watch_locals_surface`,
`crash_loop_quarantine_banner`, `cli_headless`, `evidence_export`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) preserve the packet id and every vocabulary
verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw debugger payloads, raw
stack frames, raw memory bytes, raw command lines, raw process env
bytes, raw scrollback bodies, secrets, or ambient credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/`](../../../fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation/)
exercises seven narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its
   evidence is `evidence_unbound`. The packet demotes to
   `blocks_stable` with `missing_evidence_class` +
   `launch_stable_with_unbound_binding` findings.
2. `missing_adapter_descriptor_field_for_launch_stable_blocks_stable.json` —
   the local lane drops the
   `notebook_bridge_or_replay_only_limitation` descriptor-field
   binding. `blocks_stable` with
   `missing_adapter_descriptor_field_coverage`.
3. `attach_launch_posture_drift_blocks_stable.json` — the local
   lane's `cli_headless` parity surface reports `limited` while
   `ui_surface`, `support_export`, and `docs_help` report
   `supported`. `blocks_stable` with `attach_launch_posture_drift`.
4. `crash_isolation_assertion_not_attested_blocks_stable.json` — the
   local lane's `bounded_restart_budget` assertion row stops
   attesting the assertion. `blocks_stable` with
   `crash_isolation_assertion_not_attested` and
   `missing_crash_isolation_assertion_coverage`.
5. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
6. `projection_collapses_attach_launch_posture_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the attach/launch posture
   vocabulary. `blocks_stable` with
   `attach_launch_posture_vocabulary_collapsed`.
7. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw debugger payloads, raw stack frames, raw
   memory bytes, raw command lines, or env bytes past the boundary.
   `blocks_stable` with `raw_source_material_present`.

## Source-of-truth pointers

| Lane / Path | Source of truth |
|---|---|
| Schema | [`schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json`](../../../schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json) |
| Rust contract | [`crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/`](../../../crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/) |
| Integration test | [`crates/aureline-runtime/tests/stabilize_debugger_host_and_adapter_negotiation_truth_packet.rs`](../../../crates/aureline-runtime/tests/stabilize_debugger_host_and_adapter_negotiation_truth_packet.rs) |
| Generator | [`tools/regenerate_stabilize_debugger_host_and_adapter_negotiation_truth_packet.py`](../../../tools/regenerate_stabilize_debugger_host_and_adapter_negotiation_truth_packet.py) |
| Reviewer doc | [`docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md`](../../../docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md) |
