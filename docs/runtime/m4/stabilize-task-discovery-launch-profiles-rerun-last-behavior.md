# Stabilize task discovery, launch profiles, rerun-last behavior, and task-event truth — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
task-discovery / launch-profile / rerun-last / task-event truth
packet. The cross-tool boundary schema lives at
[`schemas/runtime/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json`](../../../schemas/runtime/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/stabilize_task_discovery_launch_profiles_rerun_last_behavior/`](../../../crates/aureline-runtime/src/stabilize_task_discovery_launch_profiles_rerun_last_behavior/),
and the checked-in stable packet at
[`artifacts/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json`](../../../artifacts/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json).

The packet pins one boundary truth that the editor run surface, task
panel, problems panel, output channel, evidence export, rerun
surface, CLI/headless inspector, support export, release proof
index, Help/About proof card, and the conformance dashboard all
read. Surfaces MUST NOT mint local copies, fork their own task-event
semantics, or flatten additive detail into display text; they
project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host runs.
- `remote_helper_lane` — SSH / remote-agent runs.
- `notebook_lane` — notebook (kernel) runs.
- `imported_provider_lane` — imported task-runner runs (CI mirror,
  imported test-runner output).

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `task_event_truth_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per launch wedge
  (`task_discovery`, `launch_profile`, `rerun_last`, `task_event`).
  All four required for any `launch_stable` lane.
- `envelope_field_binding` — one row per canonical task-event
  envelope field (`event_id`, `execution_context_ref`,
  `adapter_identity`, `provider_identity`, `confidence_flag`,
  `fallback_flag`). All six required for any `launch_stable` lane.
- `surface_binding` — one row per downstream consumer surface
  (`problems`, `output_channel`, `evidence_export`,
  `rerun_surface`). All four required for any `launch_stable` lane.
- `additive_detail_preservation` — attests that additive task-event
  detail is preserved instead of flattened into display text.
  Required for every `launch_stable` lane and MUST set
  `additive_detail_preserved: true`.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into every emitted task-event envelope
  and downstream consumer surface. Required for every
  `launch_stable` lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Launch wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `task_discovery` — package scripts, pytest, framework-specific
  discovery flows.
- `launch_profile` — run/debug profile definitions and their stored
  bindings.
- `rerun_last` — rerun-last-task and rerun-last-test command
  bindings.
- `task_event` — canonical task-event envelopes emitted across the
  lane.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Canonical envelope fields (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish an
`envelope_field_binding` row for each canonical envelope field. The
six closed fields are the truth model downstream Problems,
output-channel, evidence-export, and rerun surfaces all read; they
must NOT invent a second model:

| field token | meaning |
|---|---|
| `event_id` | Stable task-event id. |
| `execution_context_ref` | Execution-context reference threading the run lineage. |
| `adapter_identity` | Adapter (runner / toolchain) identity. |
| `provider_identity` | Provider (task / launch-profile / framework) identity. |
| `confidence_flag` | Confidence flag (high/medium/low) on the envelope. |
| `fallback_flag` | Fallback flag marking a structured fallback rather than a primary capture. |

A missing envelope field auto-narrows the lane below
`launch_stable` with a typed `missing_envelope_field_coverage`
finding.

## Downstream surfaces (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `surface_binding`
row for each downstream consumer surface so the downstream surface
reads this packet verbatim instead of paraphrasing into free-form
prose:

- `problems` — Problems panel / diagnostics surface.
- `output_channel` — output channel (task / test / debug output).
- `evidence_export` — evidence export bundle.
- `rerun_surface` — rerun-last / rerun-prepared-attempt surface.

A missing downstream surface auto-narrows the lane below
`launch_stable` with a typed `missing_downstream_surface_coverage`
finding.

## Additive-detail preservation

An `additive_detail_preservation` row MUST be present on every
`launch_stable` lane with `additive_detail_preserved: true`. Local,
remote/helper, notebook, and imported-provider runs MUST serialize
into one task-event vocabulary with additive detail (structured
payload, diagnostics, adapter metadata) preserved instead of
flattened into display text.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces
(event streams, support packets, approval tickets, evidence
exports, rerun surfaces) carry the same lineage id so a
"why this task-event?" question always resolves to the same
execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `editor_run_surface`
- `task_panel`
- `problems_panel`
- `output_channel`
- `evidence_export`
- `rerun_surface`
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
`preserves_envelope_field_vocabulary`,
`preserves_downstream_surface_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to
`narrowed_below_stable`. The closed finding vocabulary covers
missing identity, missing lane coverage, missing wedge / envelope
field / downstream surface coverage, missing additive-detail
preservation, missing lineage admission, unbound support /
known-limit / downgrade-automation / evidence bindings, missing or
collapsed disclosure refs, raw source material / secrets / ambient
authority leaks, missing or drifted consumer projections, and
promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/stabilize_task_discovery_launch_profiles_rerun_last_behavior/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, and no surface paraphrases task-event truth into free-form
prose.

## See also

- Spec row: `.plans/M04-084.md`
- Reviewer artifact:
  [`artifacts/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](../../../artifacts/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md)
- Generator:
  [`tools/regenerate_stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.py`](../../../tools/regenerate_stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.py)
- Companion execution-context resolver packet:
  [`docs/runtime/m4/stabilize-execution-context-resolver.md`](./stabilize-execution-context-resolver.md)
- Beta task-event lane manifest:
  [`crates/aureline-runtime/src/task_events.rs`](../../../crates/aureline-runtime/src/task_events.rs)
- Canonical task event stream:
  [`crates/aureline-runtime/src/tasks/`](../../../crates/aureline-runtime/src/tasks/)
- Rerun surface contracts:
  [`crates/aureline-runtime/src/rerun/`](../../../crates/aureline-runtime/src/rerun/)
- Launch-profile store:
  [`crates/aureline-runtime/src/launch_profiles/`](../../../crates/aureline-runtime/src/launch_profiles/)
- Task discovery (package scripts, pytest):
  [`crates/aureline-runtime/src/discovery/`](../../../crates/aureline-runtime/src/discovery/)
