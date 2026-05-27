# Stabilize execution-context resolver — M4 reviewer artifact

This artifact summarizes the checked-in stable execution-context
resolver truth packet for release reviewers. The canonical packet is
[`stabilize_execution_context_resolver_truth_packet.json`](./stabilize_execution_context_resolver_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/stabilize-execution-context-resolver.md`](../../../docs/runtime/m4/stabilize-execution-context-resolver.md).

## What the packet promises

For each of the four execution-context lanes (`local_lane`,
`remote_helper_lane`, `container_lane`, `managed_lane`) the packet
certifies:

- One `execution_context_resolution_quality` row at `launch_stable`
  with `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Ten `surface_binding` rows covering every run-capable surface:
  `terminal`, `task`, `test`, `debug`, `request_workspace`,
  `artifact`, `ai_tool`, `cli_headless`, `docs_help`,
  `support_export`. Each row binds
  `auto_narrow_on_lineage_break` automation against
  `conformance_suite_evidence`.
- Eight `state_admission` rows covering every structured resolver
  state: `wrong_target`, `blocked_activator`,
  `stale_capsule_or_prebuild`, `unsupported_skew`,
  `reconnect_required`, `restore_no_rerun`, `degraded_helper`,
  `route_drift`. Each row binds
  `auto_narrow_on_state_admission_gap` automation against
  `failure_recovery_drill_evidence`.
- One `target_admission` row binding
  `auto_narrow_on_target_unreachable` against
  `automated_functional_evidence` so requested-vs-materialized target
  identity stays inspectable.
- One `restore_rerun_honesty` row attesting
  `restore_preserves_no_rerun: true` with
  `auto_narrow_on_silent_rerun` automation.
- One `capability_skew_admission` row binding
  `auto_narrow_on_helper_skew` automation, so a remote/helper attach
  cannot silently assume features outside the published window.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:local:lineage`) so event streams, support packets,
  approval tickets, and evidence exports cite one stable lineage
  object.

Eight consumer projections (`editor_run_surface`, `terminal_pane`,
`task_panel`, `cli_headless`, `support_export`, `release_proof_index`,
`help_about`, `conformance_dashboard`) preserve the packet id and
every vocabulary verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw command lines, raw
process env bytes, raw capsule bodies, secrets, or ambient
credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/stabilize_execution_context_resolver/`](../../../fixtures/runtime/m4/stabilize_execution_context_resolver/)
exercises five narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its evidence
   is `evidence_unbound`. The packet demotes to `blocks_stable` with
   `missing_evidence_class` + `launch_stable_with_unbound_binding`
   findings.
2. `missing_state_admission_for_launch_stable_blocks_stable.json` —
   the local lane drops the `restore_no_rerun` state admission.
   `blocks_stable` with `missing_resolver_state_coverage`.
3. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
4. `projection_collapses_resolver_state_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the resolver-state
   vocabulary. `blocks_stable` with
   `resolver_state_vocabulary_collapsed`.
5. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw command lines / env bytes / capsule bodies
   past the boundary. `blocks_stable` with
   `raw_source_material_present`.

## Where the packet lands

- Editor run surface: per-pane "why this target?" chip reads
  `target_admission` and the latest `state_admission` rows.
- Terminal pane / task panel: read the matching `surface_binding`
  row to render the local-vs-managed boundary cue and the lineage
  chip.
- CLI/headless inspector (`aureline env inspect`): projects the
  packet for `aureline env inspect --explain` and headless launch
  flows.
- Support export: bundles the packet verbatim (raw private material
  excluded).
- Release proof index, Help/About proof card, conformance dashboard:
  cite the packet id and the eight vocabularies.

## How to regenerate

```bash
python3 tools/regenerate_stabilize_execution_context_resolver_truth_packet.py
cargo test -p aureline-runtime --test stabilize_execution_context_resolver_truth_packet
```

The generator is the canonical seed for both the artifact and the
fixture corpus; the Rust contract validates either one and refuses to
publish unless every required row, binding, projection, and disclosure
ref is in place.
