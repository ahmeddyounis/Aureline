# Stabilize integrated terminal with host-boundary chips, clipboard posture, transcript export, and restore-no-rerun semantics — M4 reviewer artifact

This artifact summarizes the checked-in stable integrated-terminal
stabilization truth packet for release reviewers. The canonical
packet is
[`stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json`](./stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md`](../../../docs/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md).

## What the packet promises

For each of the four terminal-session lanes (`local_lane`,
`remote_helper_lane`, `container_lane`, `restored_lane`) the packet
certifies:

- One `terminal_stabilization_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Four `wedge_admission` rows covering every stabilization wedge:
  `host_boundary_chip`, `clipboard_posture`, `transcript_export`,
  `restore_no_rerun`. Each row binds
  `auto_narrow_on_wedge_admission_gap` automation against
  `conformance_suite_evidence`.
- Five `host_boundary_field_binding` rows covering every required
  typed chip field: `host_or_session_identity`, `route_cue`,
  `trust_state`, `restore_state`, `target_or_cwd_hint`. Each row
  binds `auto_narrow_on_host_boundary_field_gap` automation against
  `automated_functional_evidence`.
- Five `clipboard_posture_binding` rows covering every required
  surface: `clipboard_route_local_vs_remote`,
  `bracketed_paste_state`, `multiline_paste_guardrail`,
  `admin_suppression`, `high_risk_paste_review`. Each row binds
  `auto_narrow_on_clipboard_posture_gap` automation against
  `conformance_suite_evidence`.
- Three `transcript_export_field_binding` rows covering every
  required field: `transcript_versus_live_session`,
  `host_session_boundary_cue`, `redaction_state`. Each row binds
  `auto_narrow_on_transcript_export_field_gap` automation against
  `automated_functional_evidence`.
- One `restore_no_rerun_attestation` row attesting
  `attests_no_silent_rerun: true` with
  `auto_narrow_on_restore_admits_silent_rerun` automation, so
  restored sessions stay transcript-only and never silently rerun.
- One `lineage_admission` row carrying an
  `execution_context_id_binding` (e.g.
  `exec:m4:local:terminal_session_lineage`) so the terminal pane,
  transcript export, restore surface, support export, and Help/About
  proof card all cite one stable lineage object.

Nine consumer projections (`terminal_pane`,
`transcript_export_surface`, `browser_handoff_surface`,
`restore_surface`, `cli_headless`, `support_export`,
`release_proof_index`, `help_about`, `conformance_dashboard`)
preserve the packet id and every vocabulary verbatim.

## Promotion state

`stable` across all four lanes, with zero validation findings. The
support export bundles the packet without raw command lines, raw
process env bytes, raw scrollback bodies, secrets, or ambient
credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/`](../../../fixtures/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/)
exercises six narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   local lane's quality row claims `launch_stable` while its
   evidence is `evidence_unbound`. The packet demotes to
   `blocks_stable` with `missing_evidence_class` +
   `launch_stable_with_unbound_binding` findings.
2. `missing_clipboard_posture_for_launch_stable_blocks_stable.json` —
   the local lane drops the `high_risk_paste_review`
   clipboard-posture binding. `blocks_stable` with
   `missing_clipboard_posture_coverage`.
3. `restore_admits_silent_rerun_blocks_stable.json` — the restored
   lane's attestation row stops attesting no-silent-rerun.
   `blocks_stable` with
   `restore_no_rerun_attestation_admits_silent_rerun` and
   `missing_restore_no_rerun_attestation`.
4. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   local lane's quality row narrows to `launch_stable_below` and
   drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref`.
5. `projection_collapses_clipboard_posture_vocabulary_blocks_stable.json`
   — the `help_about` projection drops the clipboard-posture
   vocabulary. `blocks_stable` with
   `clipboard_posture_vocabulary_collapsed`.
6. `raw_source_material_blocks_stable.json` — the local lane's
   quality row admits raw command lines, env bytes, or scrollback
   bytes past the boundary. `blocks_stable` with
   `raw_source_material_present`.

## Source-of-truth pointers

| Lane / Path | Source of truth |
|---|---|
| Schema | [`schemas/runtime/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json`](../../../schemas/runtime/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json) |
| Rust contract | [`crates/aureline-terminal/src/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/`](../../../crates/aureline-terminal/src/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/) |
| Integration test | [`crates/aureline-terminal/tests/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.rs`](../../../crates/aureline-terminal/tests/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.rs) |
| Generator | [`tools/regenerate_stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.py`](../../../tools/regenerate_stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.py) |
| Reviewer doc | [`docs/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md`](../../../docs/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md) |
