# Harden breakpoint, call stack, variables, watch, evaluate, and debug-console fidelity on launch languages — M4 reviewer artifact

This artifact summarizes the checked-in stable debug-fidelity
(breakpoint / call-stack / variables / watch / evaluate /
debug-console) truth packet for release reviewers. The canonical
packet is
[`harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json`](./harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md`](../../../docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md).

## What the packet promises

For each of the four debug-fidelity lanes (`local_lane`,
`remote_helper_lane`, `container_lane`, `notebook_bridge_lane`) the
packet certifies:

- One `debug_fidelity_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Six `wedge_admission` rows covering every debug-fidelity wedge:
  `breakpoint_fidelity`, `call_stack_fidelity`, `variables_fidelity`,
  `watch_fidelity`, `evaluate_fidelity`, `debug_console_fidelity`.
  Each row binds `auto_narrow_on_wedge_admission_gap` automation
  against `conformance_suite_evidence`.
- Six `inspector_state_admission` rows covering every inspector state:
  `live`, `snapshot`, `stale`, `limited`, `unavailable`,
  `policy_blocked`. Each row binds
  `auto_narrow_on_inspector_state_gap` automation against
  `automated_functional_evidence`.
- Six `mapping_fidelity_badge_admission` rows covering every
  mapping-fidelity badge: `exact`, `approximate`, `partial`,
  `unavailable`, `stale`, `mismatched`. Each row binds
  `auto_narrow_on_mapping_fidelity_badge_gap` automation against
  `automated_functional_evidence`.
- Six `inspector_surface_binding` rows covering every inspector
  surface: `breakpoint_surface`, `call_stack_surface`,
  `variables_surface`, `watch_surface`, `evaluate_surface`,
  `debug_console_surface`. Each row attests the inspector-state and
  mapping-fidelity vocabularies it is required to preserve
  (variables/watch/evaluate/debug_console attest
  `attests_inspector_state_preserved`; call_stack/watch/evaluate/
  debug_console attest `attests_mapping_fidelity_preserved`) and binds
  `auto_narrow_on_inspector_surface_gap` automation against
  `conformance_suite_evidence`.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:local:debug_fidelity_lineage`) so debug-session envelopes,
  variables / watch / evaluate inspector rows, debug-console output,
  evidence exports, and support exports all cite one stable lineage
  object.

Twelve consumer projections (`breakpoint_surface`, `call_stack_surface`,
`variables_surface`, `watch_surface`, `evaluate_surface`,
`debug_console_surface`, `cli_headless`, `evidence_export`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) preserve the packet id and every vocabulary
verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw debugger payloads, raw
stack frames, raw memory bytes, raw watch expressions, raw evaluate
input/output, raw console scrollback bodies, raw command lines, raw
process env bytes, secrets, or ambient credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/`](../../../fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and/)
exercises seven narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its
   evidence is `evidence_unbound`. The packet demotes to
   `blocks_stable` with `missing_evidence_class` +
   `launch_stable_with_unbound_binding` findings.
2. `missing_inspector_state_for_launch_stable_blocks_stable.json` —
   the local lane drops the `policy_blocked` inspector_state
   admission. `blocks_stable` with `missing_inspector_state_coverage`.
3. `missing_mapping_fidelity_badge_for_launch_stable_blocks_stable.json` —
   the local lane drops the `mismatched` mapping_fidelity_badge
   admission. `blocks_stable` with
   `missing_mapping_fidelity_badge_coverage`.
4. `inspector_surface_missing_state_attestation_blocks_stable.json` —
   the local lane's `watch_surface` inspector_surface_binding row
   stops attesting inspector-state preservation. `blocks_stable`
   with `inspector_surface_missing_inspector_state_attestation` +
   `missing_inspector_surface_coverage`.
5. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
6. `projection_collapses_inspector_state_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the inspector-state
   vocabulary. `blocks_stable` with
   `inspector_state_vocabulary_collapsed`.
7. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw debugger payloads, raw stack frames, raw
   memory bytes, raw watch expressions, raw evaluate input/output,
   raw console scrollback bodies, raw command lines, or raw env bytes
   past the boundary. `blocks_stable` with
   `raw_source_material_present`.

## Source-of-truth pointers

| Lane / Path | Source of truth |
|---|---|
| Schema | [`schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json`](../../../schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json) |
| Rust contract | [`crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/`](../../../crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/) |
| Integration test | [`crates/aureline-runtime/tests/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.rs`](../../../crates/aureline-runtime/tests/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.rs) |
| Generator | [`tools/regenerate_harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.py`](../../../tools/regenerate_harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.py) |
| Reviewer doc | [`docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md`](../../../docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md) |
