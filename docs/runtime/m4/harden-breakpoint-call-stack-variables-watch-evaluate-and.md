# Harden breakpoint, call stack, variables, watch, evaluate, and debug-console fidelity on launch languages — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
debug-fidelity (breakpoint / call-stack / variables / watch / evaluate /
debug-console) truth packet. The cross-tool boundary schema lives at
[`schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json`](../../../schemas/runtime/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/`](../../../crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/),
and the checked-in stable packet at
[`artifacts/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json`](../../../artifacts/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json).

The packet pins one boundary truth that the breakpoint surface,
call-stack surface, variables surface, watch surface, evaluate surface,
debug-console surface, CLI/headless inspector, evidence export, support
export, release proof index, Help/About proof card, and the conformance
dashboard all read. Surfaces MUST NOT mint local copies, flatten
inspector-state into generic error copy, or paraphrase
mapping-fidelity badges; they project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host debug sessions.
- `remote_helper_lane` — SSH / remote-agent debug sessions.
- `container_lane` — container-attached debug sessions.
- `notebook_bridge_lane` — notebook kernel debugger bridge sessions.

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `debug_fidelity_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per debug-fidelity wedge
  (`breakpoint_fidelity`, `call_stack_fidelity`, `variables_fidelity`,
  `watch_fidelity`, `evaluate_fidelity`, `debug_console_fidelity`). All
  six required for any `launch_stable` lane.
- `inspector_state_admission` — one row per inspector state
  (`live`, `snapshot`, `stale`, `limited`, `unavailable`,
  `policy_blocked`). All six required for any `launch_stable` lane.
- `mapping_fidelity_badge_admission` — one row per mapping-fidelity
  badge (`exact`, `approximate`, `partial`, `unavailable`, `stale`,
  `mismatched`). All six required for any `launch_stable` lane.
- `inspector_surface_binding` — one row per inspector surface
  (`breakpoint_surface`, `call_stack_surface`, `variables_surface`,
  `watch_surface`, `evaluate_surface`, `debug_console_surface`). All
  six required for any `launch_stable` lane. Each row MUST attest the
  inspector-state and mapping-fidelity vocabularies it is required to
  preserve.
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

## Debug-fidelity wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `breakpoint_fidelity` — set/clear lifecycle, verified state, hit
  counts, conditions, logpoints, and the distinction between
  source-mapped and unverified breakpoints.
- `call_stack_fidelity` — frame list, source mapping, async/inlined
  frame disclosure, and host-lane identity badge surfacing.
- `variables_fidelity` — scopes, lazy/eager evaluation, inspector-state
  distinction, and redaction posture.
- `watch_fidelity` — watch expressions, inspector-state and
  mapping-fidelity preservation, and restore behavior.
- `evaluate_fidelity` — REPL-style evaluation, inspector-state
  preservation, mapping-fidelity preservation, and redaction posture.
- `debug_console_fidelity` — console-linked inspector output, state and
  mapping-fidelity preservation, and transcript truth.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Inspector states (required per `launch_stable` lane)

Variables, watches, evaluate, and console-linked inspector rows MUST
distinguish these six states on every claimed stable debug lane,
including remote/helper lanes and any notebook-adjacent bridge:

| state token | meaning |
|---|---|
| `live` | inspector value reflects the current paused frame. |
| `snapshot` | inspector value reflects a captured snapshot. |
| `stale` | inspector value is older than the current pause point. |
| `limited` | inspector value is partially available (lazy/limited). |
| `unavailable` | inspector value cannot be retrieved. |
| `policy_blocked` | inspector value retrieval is blocked by policy. |

A missing inspector state auto-narrows the lane below `launch_stable`
with a typed `missing_inspector_state_coverage` finding.

## Mapping-fidelity badges (required per `launch_stable` lane)

Mapping-fidelity and host-lane identity badges MUST remain visible in
stack, watch, evaluate, and debug-console flows and MUST survive
export/support packets rather than being flattened into generic error
copy. The six closed badges are:

| badge token | meaning |
|---|---|
| `exact` | mapping is exact for the frame / variable / watch. |
| `approximate` | mapping is approximate but trustworthy. |
| `partial` | mapping is partial; some detail is missing. |
| `unavailable` | mapping is unavailable for the frame / variable. |
| `stale` | mapping is stale (e.g. source changed since launch). |
| `mismatched` | mapping is mismatched (e.g. binary / source skew). |

A missing badge auto-narrows the lane below `launch_stable` with a
typed `missing_mapping_fidelity_badge_coverage` finding.

## Inspector surfaces (required per `launch_stable` lane)

Every `launch_stable` lane MUST publish an `inspector_surface_binding`
row for each of:

- `breakpoint_surface` — no inspector-state / mapping-fidelity
  attestation required (it carries its own breakpoint-state chips).
- `call_stack_surface` — MUST attest `attests_mapping_fidelity_preserved`.
- `variables_surface` — MUST attest `attests_inspector_state_preserved`.
- `watch_surface` — MUST attest both attestations.
- `evaluate_surface` — MUST attest both attestations.
- `debug_console_surface` — MUST attest both attestations.

A missing surface, missing inspector-state attestation, or missing
mapping-fidelity attestation auto-narrows the lane below `launch_stable`
with a typed
`missing_inspector_surface_coverage` /
`inspector_surface_missing_inspector_state_attestation` /
`inspector_surface_missing_mapping_fidelity_attestation` finding.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces
(debug-session envelopes, support packets, evidence exports, debug
console output, watch/evaluate inspector rows) carry the same lineage
id so a "why this debug session?" question always resolves to the same
execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `breakpoint_surface`
- `call_stack_surface`
- `variables_surface`
- `watch_surface`
- `evaluate_surface`
- `debug_console_surface`
- `cli_headless`
- `evidence_export`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the eleven vocabularies
verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_wedge_vocabulary`,
`preserves_inspector_state_vocabulary`,
`preserves_mapping_fidelity_badge_vocabulary`,
`preserves_inspector_surface_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to
`narrowed_below_stable`. The closed finding vocabulary covers
missing identity, missing lane coverage, missing wedge / inspector
state / mapping-fidelity badge / inspector-surface coverage, missing
lineage admission, missing surface attestations, unbound support /
known-limit / downgrade-automation / evidence bindings, missing or
collapsed disclosure refs, raw debugger payload / secret / ambient
authority leaks, missing or drifted consumer projections, and
promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, and no surface paraphrases debug-fidelity truth into free-form
prose.

## See also

- Reviewer artifact:
  [`artifacts/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md`](../../../artifacts/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md)
- Generator:
  [`tools/regenerate_harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.py`](../../../tools/regenerate_harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.py)
- DAP-host supervisor seed:
  [`crates/aureline-runtime/src/debug/`](../../../crates/aureline-runtime/src/debug/)
- Companion debugger-stabilization packet:
  [`docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md`](./stabilize-debugger-host-adapter-negotiation-attach-launch.md)
- Companion execution-context resolver packet:
  [`docs/runtime/m4/stabilize-execution-context-resolver.md`](./stabilize-execution-context-resolver.md)
