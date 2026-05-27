# Stabilize integrated terminal with host-boundary chips, clipboard posture, transcript export, and restore-no-rerun semantics — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
integrated-terminal stabilization truth packet. The cross-tool
boundary schema lives at
[`schemas/runtime/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json`](../../../schemas/runtime/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-terminal/src/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/`](../../../crates/aureline-terminal/src/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/),
and the checked-in stable packet at
[`artifacts/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json`](../../../artifacts/runtime/m4/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json).

The packet pins one boundary truth that the terminal pane chip strip,
transcript export surface, browser handoff surface, restore surface,
CLI/headless inspector, support export, release proof index,
Help/About proof card, and the conformance dashboard all read.
Surfaces MUST NOT mint local copies, fork their own host-boundary or
clipboard semantics, or blur the transcript-versus-live distinction;
they project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host terminal sessions.
- `remote_helper_lane` — SSH / remote-agent terminal sessions.
- `container_lane` — container-attached terminal sessions.
- `restored_lane` — restored transcript-only sessions (the
  restore-no-rerun lane).

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `terminal_stabilization_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per stabilization wedge
  (`host_boundary_chip`, `clipboard_posture`, `transcript_export`,
  `restore_no_rerun`). All four required for any `launch_stable` lane.
- `host_boundary_field_binding` — one row per typed host-boundary
  field (`host_or_session_identity`, `route_cue`, `trust_state`,
  `restore_state`, `target_or_cwd_hint`). All five required for any
  `launch_stable` lane.
- `clipboard_posture_binding` — one row per clipboard-posture surface
  (`clipboard_route_local_vs_remote`, `bracketed_paste_state`,
  `multiline_paste_guardrail`, `admin_suppression`,
  `high_risk_paste_review`). All five required for any
  `launch_stable` lane.
- `transcript_export_field_binding` — one row per transcript-export
  field (`transcript_versus_live_session`,
  `host_session_boundary_cue`, `redaction_state`). All three required
  for any `launch_stable` lane.
- `restore_no_rerun_attestation` — attests that restored sessions
  are transcript-only and never silently rerun. Required for every
  `launch_stable` lane and MUST set `attests_no_silent_rerun: true`.
- `lineage_admission` — binds the stable `execution_context_id`
  threading through every surface that consumes the session.
  Required for every `launch_stable` lane and MUST surface a
  non-empty `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Stabilization wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `host_boundary_chip` — the typed chip carrying host, route, trust,
  restore, and target/cwd hints shown before any mutating action
  (copy, paste, rerun, browser handoff, transcript export).
- `clipboard_posture` — local-vs-remote clipboard route, bracketed
  paste, multiline guardrail, admin suppression, and high-risk paste
  review.
- `transcript_export` — transcript-vs-live distinction, host/session
  boundary cue carried in the export header, redaction state.
- `restore_no_rerun` — restored sessions are transcript-only and
  never silently rerun.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Host-boundary fields (required per `launch_stable` lane)

| field token | meaning |
|---|---|
| `host_or_session_identity` | Host id, session id, and attached transport identity surfaced on the chip. |
| `route_cue` | Local-vs-remote route cue distinguishing where command bytes land. |
| `trust_state` | Trust state (trusted, untrusted, degraded). |
| `restore_state` | Live, restored transcript, reconnecting, or blocked. |
| `target_or_cwd_hint` | Target / cwd hint surfaced beside the chip. |

A missing host-boundary field auto-narrows the lane below
`launch_stable` with a typed
`missing_host_boundary_field_coverage` finding.

## Clipboard-posture surfaces (required per `launch_stable` lane)

| posture token | meaning |
|---|---|
| `clipboard_route_local_vs_remote` | Local-vs-remote clipboard route applied to copy + paste + OSC writes. |
| `bracketed_paste_state` | Bracketed-paste mode tracked and escape sequences handled safely. |
| `multiline_paste_guardrail` | Multiline / newline-terminating paste requires confirmation. |
| `admin_suppression` | Admin / privileged-shell suppression of clipboard writes from protocol escapes. |
| `high_risk_paste_review` | Sensitive target / broad-input review step before paste lands. |

A missing posture surface auto-narrows the lane below `launch_stable`
with a typed `missing_clipboard_posture_coverage` finding.

## Transcript-export fields (required per `launch_stable` lane)

| field token | meaning |
|---|---|
| `transcript_versus_live_session` | Whether the surface holds a live session or a frozen transcript, surfaced before export / reopen. |
| `host_session_boundary_cue` | Host/session boundary cue carried in the exported header so an exported transcript never loses provenance. |
| `redaction_state` | Which scrollback redaction classes are present in the exported transcript. |

A missing transcript-export field auto-narrows the lane below
`launch_stable` with a typed
`missing_transcript_export_field_coverage` finding.

## Restore-no-rerun attestation

A `restore_no_rerun_attestation` row MUST be present on every
`launch_stable` lane with `attests_no_silent_rerun: true`. Restored
sessions are always transcript-only; auto-rerun on restore is
forbidden. A restore row that drops the attestation auto-narrows the
lane below `launch_stable` with a typed
`restore_no_rerun_attestation_admits_silent_rerun` finding.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces
(terminal pane, transcript export, restore surface, support exports,
release proof index) carry the same lineage id so a "why this
terminal session?" question always resolves to the same
execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `terminal_pane`
- `transcript_export_surface`
- `browser_handoff_surface`
- `restore_surface`
- `cli_headless`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the nine
vocabularies verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_wedge_vocabulary`,
`preserves_host_boundary_field_vocabulary`,
`preserves_clipboard_posture_vocabulary`,
`preserves_transcript_export_field_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to `narrowed_below_stable`.
The closed finding vocabulary covers missing identity, missing lane
coverage, missing wedge / host-boundary / clipboard-posture /
transcript-export coverage, missing restore-no-rerun attestation
(or attestation admitting silent rerun), missing lineage admission,
unbound support / known-limit / downgrade-automation / evidence
bindings, missing or collapsed disclosure refs, raw source material /
secrets / ambient authority leaks, missing or drifted consumer
projections, and promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-terminal/src/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, no surface paraphrases host-boundary or clipboard truth into
free-form prose, and no restored session pretends to be live.

## See also

- Spec row: `.plans/M04-085.md`
- Reviewer artifact:
  [`artifacts/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md`](../../../artifacts/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md)
- Generator:
  [`tools/regenerate_stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.py`](../../../tools/regenerate_stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.py)
- Companion task-event truth packet:
  [`docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](./stabilize-task-discovery-launch-profiles-rerun-last-behavior.md)
- Terminal header chips:
  [`crates/aureline-terminal/src/headers.rs`](../../../crates/aureline-terminal/src/headers.rs)
- Terminal restore semantics:
  [`crates/aureline-terminal/src/restore.rs`](../../../crates/aureline-terminal/src/restore.rs)
- Protocol corpus (paste review, clipboard writes, restore proofs):
  [`crates/aureline-terminal/src/protocol_corpus/`](../../../crates/aureline-terminal/src/protocol_corpus/)
- Bounded scrollback with redaction classes:
  [`crates/aureline-terminal/src/scrollback/`](../../../crates/aureline-terminal/src/scrollback/)
