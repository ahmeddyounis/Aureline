# Stabilize the debugger host, adapter negotiation, attach/launch flows, and crash isolation — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
debugger-host / adapter-negotiation / attach-launch / crash-isolation
truth packet. The cross-tool boundary schema lives at
[`schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json`](../../../schemas/runtime/stabilize_debugger_host_and_adapter_negotiation_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/`](../../../crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/),
and the checked-in stable packet at
[`artifacts/runtime/m4/stabilize_debugger_host_and_adapter_negotiation_truth_packet.json`](../../../artifacts/runtime/m4/stabilize_debugger_host_and_adapter_negotiation_truth_packet.json).

The packet pins one boundary truth that the editor debug surface,
debug session panel, breakpoint surface, watch/locals surface,
crash-loop quarantine banner, CLI/headless inspector, evidence
export, support export, release proof index, Help/About proof card,
and the conformance dashboard all read. Surfaces MUST NOT mint local
copies or fork their own debugger semantics; they project the packet
verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host debug sessions.
- `remote_helper_lane` — SSH / remote-agent debug sessions.
- `container_lane` — container-attached debug sessions.
- `notebook_bridge_lane` — notebook kernel debugger bridge sessions.

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `debugger_stabilization_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per debugger wedge
  (`debugger_host`, `adapter_negotiation`, `attach_launch_flow`,
  `crash_isolation`). All four required for any `launch_stable` lane.
- `adapter_descriptor_field_binding` — one row per adapter/backend
  descriptor field (`adapter_identity`, `transport_class`,
  `launch_attach_scope`, `local_vs_remote_support_class`,
  `chronology_replay_capability_class`,
  `notebook_bridge_or_replay_only_limitation`). All six required for
  any `launch_stable` lane.
- `attach_launch_parity_surface_binding` — one row per attach/launch
  parity surface (`ui_surface`, `cli_headless`, `support_export`,
  `docs_help`) binding the propagated
  `attach_launch_posture_class`. All four required for any
  `launch_stable` lane, and all four MUST report the same posture.
- `crash_isolation_assertion_binding` — one row per crash-isolation
  assertion (`bounded_restart_budget`, `session_quarantine_admission`,
  `unrelated_language_host_unaffected`,
  `unrelated_terminal_lane_unaffected`,
  `unrelated_debug_session_unaffected`). All five required for any
  `launch_stable` lane and MUST set
  `attests_crash_isolation_assertion: true`.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into emitted debug-session envelopes and
  downstream consumer surfaces. Required for every `launch_stable`
  lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Debugger wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `debugger_host` — the DAP-style supervisor lifecycle owning launch,
  attach, breakpoint sync, stop/continue, and shutdown.
- `adapter_negotiation` — explicit adapter capability and transport
  negotiation; the supervisor MUST decide what subset of breakpoints,
  watch expressions, and step modes the adapter declares.
- `attach_launch_flow` — the attach/launch lifecycle including
  pre-launch validation and the explicit attach handshake.
- `crash_isolation` — the bounded restart budget, typed exit reasons,
  session quarantine on exceeded budget, and the no-spillover
  invariant for unrelated hosts/lanes/sessions.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Adapter / backend descriptor fields (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish an
`adapter_descriptor_field_binding` row for each canonical descriptor
field. The six closed fields are the truth model downstream reviewers
read; reviewers MUST NOT infer support from button presence:

| field token | meaning |
|---|---|
| `adapter_identity` | Adapter id, version, and vendor. |
| `transport_class` | Adapter transport class (stdio, socket, named pipe, websocket). |
| `launch_attach_scope` | Launch-only, attach-only, both, or launch-with-attach-fallback. |
| `local_vs_remote_support_class` | Local-only, remote-only, both, or helper-only. |
| `chronology_replay_capability_class` | Live-only, chronology-supported, replay-only, or chronology-and-replay-supported. |
| `notebook_bridge_or_replay_only_limitation` | None, notebook-bridge, replay-only, or notebook-bridge-and-replay-only. |

A missing descriptor field auto-narrows the lane below
`launch_stable` with a typed
`missing_adapter_descriptor_field_coverage` finding.

## Attach/launch posture and parity surfaces (required per `launch_stable` lane)

Attach/launch negotiation MUST degrade explicitly per runtime/backend
row, and the chosen posture label MUST propagate verbatim to UI,
CLI/headless, support export, and docs/help without drift.

Posture vocabulary (closed):

- `supported` — fully supported attach/launch.
- `limited` — supported with disclosed gaps; requires a disclosure
  ref.
- `view_only` — view-only (e.g., inspection without resume/step);
  requires a disclosure ref.
- `unsupported` — not supported on the lane; requires a disclosure
  ref.
- `policy_blocked` — blocked by policy on the lane; requires a
  disclosure ref.

Parity surfaces (closed): `ui_surface`, `cli_headless`,
`support_export`, `docs_help`. Every `launch_stable` lane MUST
publish exactly one binding per parity surface, and the posture MUST
agree across all four bindings on that lane. A mismatch
auto-narrows the lane below `launch_stable` with a typed
`attach_launch_posture_drift` finding.

## Crash-isolation assertions (required per `launch_stable` lane)

Every `launch_stable` lane MUST attest each of the five closed
crash-isolation assertions:

- `bounded_restart_budget` — the supervisor applies a bounded restart
  budget on adapter crash / protocol violation / hang.
- `session_quarantine_admission` — on exceeded budget, the session
  moves into a typed quarantine state instead of spinning.
- `unrelated_language_host_unaffected` — unrelated language hosts are
  untouched.
- `unrelated_terminal_lane_unaffected` — unrelated terminal lanes are
  untouched.
- `unrelated_debug_session_unaffected` — unrelated debug sessions are
  untouched.

Each assertion row MUST set `attests_crash_isolation_assertion: true`.
A missing or unattested assertion auto-narrows the lane below
`launch_stable` with a typed
`missing_crash_isolation_assertion_coverage` or
`crash_isolation_assertion_not_attested` finding.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces
(debug-session envelopes, support packets, approval tickets,
evidence exports, crash-loop quarantine banners) carry the same
lineage id so a "why this debug session?" question always resolves
to the same execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `editor_debug_surface`
- `debug_session_panel`
- `breakpoint_surface`
- `watch_locals_surface`
- `crash_loop_quarantine_banner`
- `cli_headless`
- `evidence_export`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the eleven
vocabularies verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_wedge_vocabulary`,
`preserves_adapter_descriptor_field_vocabulary`,
`preserves_attach_launch_parity_surface_vocabulary`,
`preserves_attach_launch_posture_vocabulary`,
`preserves_crash_isolation_assertion_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to
`narrowed_below_stable`. The closed finding vocabulary covers
missing identity, missing lane coverage, missing wedge / adapter
descriptor field / parity surface / crash-isolation assertion
coverage, missing lineage admission, unbound support /
known-limit / downgrade-automation / evidence bindings, missing or
collapsed disclosure refs, attach/launch posture drift, unattested
crash-isolation assertions, raw debugger payload / secret / ambient
authority leaks, missing or drifted consumer projections, and
promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, and no surface paraphrases debugger truth into free-form
prose.

## See also

- Reviewer artifact:
  [`artifacts/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md`](../../../artifacts/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md)
- Generator:
  [`tools/regenerate_stabilize_debugger_host_and_adapter_negotiation_truth_packet.py`](../../../tools/regenerate_stabilize_debugger_host_and_adapter_negotiation_truth_packet.py)
- DAP-host supervisor seed:
  [`crates/aureline-runtime/src/debug/`](../../../crates/aureline-runtime/src/debug/)
- Beta shared debug control:
  [`crates/aureline-runtime/src/shared_debug_alpha/`](../../../crates/aureline-runtime/src/shared_debug_alpha/)
- Companion execution-context resolver packet:
  [`docs/runtime/m4/stabilize-execution-context-resolver.md`](./stabilize-execution-context-resolver.md)
- Companion task-event truth packet:
  [`docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](./stabilize-task-discovery-launch-profiles-rerun-last-behavior.md)
