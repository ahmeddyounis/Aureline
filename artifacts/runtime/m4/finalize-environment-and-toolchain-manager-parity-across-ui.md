# Environment + toolchain manager and execution-context inspector parity — M4 reviewer artifact

This artifact summarizes the checked-in stable inspector-parity truth
packet for release reviewers. The canonical packet is
[`finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json`](./finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md`](../../../docs/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md).

## What the packet promises

For each of the four execution-context lanes (`local_lane`,
`remote_helper_lane`, `container_lane`, `managed_lane`) the packet
certifies:

- One `inspector_parity_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Eight `inspector_field_admission` rows covering every required
  inspector field: `interpreter`, `sdk`, `shell`, `container_target`,
  `remote_target`, `activator`, `trust_state`, `policy_source`. Each
  row binds `auto_narrow_on_inspector_field_gap` automation against
  `conformance_suite_evidence`.
- Four `parity_surface_binding` rows covering every parity surface:
  `ui`, `cli_headless`, `help_about`, `support_export`. Each row
  binds `auto_narrow_on_parity_surface_break` automation against
  `conformance_suite_evidence` so the inspector returns the same
  resolved fields and the same explanation regardless of where it is
  invoked.
- Five `recovery_admission` rows covering every recovery posture:
  `reconnect`, `restore_no_rerun`, `blocked_target`,
  `degraded_helper`, `artifact_provenance`. The `restore_no_rerun`
  row attests `restore_preserves_no_rerun: true` with
  `auto_narrow_on_silent_rerun` automation. The remaining four rows
  bind `auto_narrow_on_recovery_state_gap` automation. All five rows
  cite `failure_recovery_drill_evidence`.
- One `toolchain_manager_admission` row binding
  `auto_narrow_on_toolchain_manager_drift` against
  `automated_functional_evidence` and carrying a
  `toolchain_manager_id_binding` (e.g.
  `toolchain_manager:m4:local`) so the environment + toolchain
  manager identity stays inspectable.
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
[`fixtures/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui/`](../../../fixtures/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui/)
exercises five narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its evidence
   is `evidence_unbound`. The packet demotes to `blocks_stable` with
   `missing_evidence_class` + `launch_stable_with_unbound_binding`
   findings.
2. `missing_inspector_field_for_launch_stable_blocks_stable.json` —
   the local lane drops the `policy_source` inspector-field
   admission. `blocks_stable` with
   `missing_inspector_field_coverage`.
3. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
4. `projection_collapses_parity_surface_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the parity-surface
   vocabulary. `blocks_stable` with
   `parity_surface_vocabulary_collapsed`.
5. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw command lines / env bytes / capsule bodies
   past the boundary. `blocks_stable` with
   `raw_source_material_present`.

## Where the packet lands

- Editor run surface: per-pane "why this target?" chip reads the
  lane's inspector-field and recovery rows.
- Terminal pane / task panel: read the matching
  `parity_surface_binding` row so the inspector returns the same
  fields and the same explanation as the CLI/headless inspector.
- CLI/headless inspector (`aureline env inspect`): projects the
  packet for `aureline env inspect --explain` and headless launch
  flows.
- Help/About proof card: reads the `parity_surface_binding` row plus
  the `toolchain_manager_admission` and `lineage_admission` rows.
- Support export: bundles the packet verbatim (raw private material
  excluded).
- Release proof index, conformance dashboard: cite the packet id and
  the nine vocabularies.

## How to regenerate

```bash
python3 tools/regenerate_finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.py
cargo test -p aureline-runtime --test finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet
```

The generator is the canonical seed for both the artifact and the
fixture corpus; the Rust contract validates either one and refuses to
publish unless every required row, binding, projection, and disclosure
ref is in place.
