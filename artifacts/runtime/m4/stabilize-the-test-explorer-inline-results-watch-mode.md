# Stabilize the test explorer, inline results, watch-mode truth, and rerun/debug-from-test parity — M4 reviewer artifact

This artifact summarizes the checked-in stable
stabilize-the-test-explorer / inline-results / watch-mode / rerun /
debug-from-test truth packet for release reviewers. The canonical
packet is
[`stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json`](./stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md`](../../../docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md).

## What the packet promises

For each of the four test-explorer lanes (`local_lane`,
`remote_helper_lane`, `container_lane`, `notebook_lane`) the packet
certifies:

- One `test_explorer_stabilization_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every test-explorer wedge:
  `test_explorer_identity_truth`, `inline_results_truth`,
  `watch_mode_truth`, `rerun_debug_from_test_parity`. Each row binds
  `auto_narrow_on_wedge_admission_gap` automation against
  `conformance_suite_evidence`.
- Four `test_identity_admission` rows covering every durable identity:
  `suite_identity`, `case_identity`, `template_identity`,
  `invocation_identity`. Each row binds
  `auto_narrow_on_test_identity_gap` automation against
  `automated_functional_evidence`.
- Three `discovery_posture_admission` rows covering every discovery
  posture: `partial_discovery_record`, `loaded_versus_known_counts`,
  `case_enumeration_at_runtime`. Each row binds
  `auto_narrow_on_discovery_posture_gap` automation against
  `automated_functional_evidence`.
- Four `watch_mode_support_admission` rows covering every watch-mode
  support class: `live`, `reduced`, `polling`, `unavailable`. Each row
  binds `auto_narrow_on_watch_mode_support_gap` automation against
  `automated_functional_evidence`.
- Three `selector_durability_admission` rows covering every durable
  selector class: `durable_id_selector`, `trait_selector`,
  `snapshot_scoped_query_selector`. Each row binds
  `auto_narrow_on_selector_durability_gap` automation against
  `automated_functional_evidence`.
- Five `consumer_surface_binding` rows covering every consumer
  surface: `test_explorer_surface`, `inline_results_surface`,
  `watch_mode_surface`, `rerun_surface`, `debug_from_test_surface`.
  Each row attests the test-identity, watch-mode-support, and
  durable-selector vocabularies it is required to preserve
  (all five attest `attests_test_identity_preserved`; the
  watch_mode surface attests `attests_watch_mode_support_preserved`;
  rerun/debug-from-test attest `attests_durable_selector_preserved`)
  and binds `auto_narrow_on_consumer_surface_gap` automation against
  `conformance_suite_evidence`.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:local:test_explorer_lineage`) so test-explorer envelopes,
  inline-result rows, watch-mode session/attempt rows, rerun
  invocations, debug-from-test launches, AI tool plans, evidence
  exports, and support exports all cite one stable lineage object.

Twelve consumer projections (`test_explorer_surface`,
`inline_results_surface`, `watch_mode_surface`, `rerun_surface`,
`debug_from_test_surface`, `ai_tool_surface`, `cli_headless`,
`evidence_export`, `support_export`, `release_proof_index`,
`help_about`, `conformance_dashboard`) preserve the packet id and
every vocabulary verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw test source bodies, raw
runner scrollback bodies, raw stack frames, raw command lines, raw
process env bytes, secrets, or ambient credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode/`](../../../fixtures/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode/)
exercises seven narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its
   evidence is `evidence_unbound`. The packet demotes to
   `blocks_stable` with `missing_evidence_class` +
   `launch_stable_with_unbound_binding` findings.
2. `missing_watch_mode_support_for_launch_stable_blocks_stable.json` —
   the local lane drops the `polling` watch_mode_support admission.
   `blocks_stable` with `missing_watch_mode_support_coverage`.
3. `missing_selector_durability_for_launch_stable_blocks_stable.json` —
   the local lane drops the `snapshot_scoped_query_selector`
   selector_durability admission. `blocks_stable` with
   `missing_selector_durability_coverage`.
4. `consumer_surface_missing_durable_selector_attestation_blocks_stable.json`
   — the local lane's `rerun_surface` consumer_surface_binding row
   stops attesting durable-selector preservation. `blocks_stable`
   with `consumer_surface_missing_durable_selector_attestation` +
   `missing_consumer_surface_coverage`.
5. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
6. `projection_collapses_selector_durability_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the selector-durability
   vocabulary. `blocks_stable` with
   `selector_durability_vocabulary_collapsed`.
7. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw test source bodies, raw runner scrollback
   bodies, raw stack frames, raw command lines, or raw env bytes past
   the boundary. `blocks_stable` with
   `raw_source_material_present`.

## Source-of-truth pointers

| Lane / Path | Source of truth |
|---|---|
| Schema | [`schemas/runtime/stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json`](../../../schemas/runtime/stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json) |
| Rust contract | [`crates/aureline-runtime/src/stabilize_the_test_explorer_inline_results_watch_mode/`](../../../crates/aureline-runtime/src/stabilize_the_test_explorer_inline_results_watch_mode/) |
| Integration test | [`crates/aureline-runtime/tests/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.rs`](../../../crates/aureline-runtime/tests/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.rs) |
| Generator | [`tools/regenerate_stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.py`](../../../tools/regenerate_stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.py) |
| Reviewer doc | [`docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md`](../../../docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md) |
