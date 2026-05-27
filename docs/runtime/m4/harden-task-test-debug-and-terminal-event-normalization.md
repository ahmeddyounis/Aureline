# Harden task, test, debug, and terminal event normalization — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
event-normalization truth packet. The cross-tool boundary schema
lives at
[`schemas/runtime/harden_task_test_debug_and_terminal_event_normalization_truth.schema.json`](../../../schemas/runtime/harden_task_test_debug_and_terminal_event_normalization_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-terminal/src/harden_task_test_debug_and_terminal_event_normalization/`](../../../crates/aureline-terminal/src/harden_task_test_debug_and_terminal_event_normalization/),
and the checked-in stable packet at
[`artifacts/runtime/m4/harden_task_test_debug_and_terminal_event_normalization_truth_packet.json`](../../../artifacts/runtime/m4/harden_task_test_debug_and_terminal_event_normalization_truth_packet.json).

The packet pins one boundary truth that the editor run surface,
task panel, test runner surface, debug surface, terminal pane,
CLI/headless inspector, AI tool surface, review surface, support
export, release proof index, Help/About proof card, and the
conformance dashboard all read. Surfaces MUST NOT mint local copies,
paraphrase fields, flatten the canonical envelope into display text,
or collapse the source-kind or lifecycle vocabularies; they project
the packet verbatim.

## Lanes (closed vocabulary)

- `task_lane` — task runner event normalization.
- `test_lane` — test runner event normalization.
- `debug_lane` — debug adapter event normalization.
- `terminal_lane` — terminal scrollback / OSC event normalization.

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `event_normalization_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per wedge
  (`envelope_canonicalization`, `source_kind_negotiation`,
  `lifecycle_normalization`, `export_preservation`). All four
  required for any `launch_stable` lane.
- `envelope_field_binding` — one row per canonical envelope field
  (`event_id`, `workspace_id`, `target_id`, `source_kind`,
  `confidence`, `timestamp`, `execution_context_id`, `payload_kind`,
  `raw_payload_ref`, `provenance`). All ten required for any
  `launch_stable` lane.
- `source_kind_binding` — one row per source kind (`native`, `bsp`,
  `bazel_bep`, `structured_output`, `heuristic_parser`). All five
  required for any `launch_stable` lane.
- `lifecycle_event_binding` — one row per canonical lifecycle event
  (`task_queued`, `target_graph_ready`, `task_started`,
  `progress_updated`, `diagnostic_emitted`, `test_case_started`,
  `test_case_finished`, `artifact_published`, `task_finished`). All
  nine required for any `launch_stable` lane.
- `consumer_surface_binding` — one row per downstream consumer
  surface (`editor_run_surface`, `task_panel`, `test_runner_surface`,
  `debug_surface`, `terminal_pane`, `cli_headless`,
  `ai_tool_surface`, `review_surface`, `support_export`). All nine
  required for any `launch_stable` lane.
- `raw_payload_retention_attestation` — attests that replay,
  export, and support packets preserve `source_kind`, `confidence`,
  and the adapter raw payload reference. Required for every
  `launch_stable` lane and MUST set
  `attests_raw_payload_retained: true`.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into every emitted envelope and
  downstream consumer surface. Required for every `launch_stable`
  lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `envelope_canonicalization` — one envelope shape across task,
  test, debug, and terminal lanes.
- `source_kind_negotiation` — adapter-isolated capability
  negotiation per source kind (native / BSP / Bazel BEP /
  structured-output / heuristic-parser).
- `lifecycle_normalization` — the canonical lifecycle set emitted by
  every lane.
- `export_preservation` — replay, export, and support packets
  preserve `source_kind`, `confidence`, and the adapter raw payload
  reference rather than flattening them into one undifferentiated
  ledger.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Canonical envelope fields (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish an
`envelope_field_binding` row for each canonical envelope field. The
ten closed fields are the truth model downstream UI, CLI/headless,
AI, review, and support-export surfaces all read; they must NOT
invent a second model:

| field token | meaning |
|---|---|
| `event_id` | Stable event id. |
| `workspace_id` | Workspace id the event was emitted in. |
| `target_id` | Target id the event was emitted for. |
| `source_kind` | Source kind (native / bsp / bazel_bep / structured_output / heuristic_parser). |
| `confidence` | Confidence flag on the envelope. |
| `timestamp` | Capture timestamp. |
| `execution_context_id` | Execution-context id threading lineage. |
| `payload_kind` | Payload kind discriminator. |
| `raw_payload_ref` | Reference to the retained raw adapter payload. |
| `provenance` | Provenance object for the envelope. |

A missing envelope field auto-narrows the lane below `launch_stable`
with a typed `missing_envelope_field_coverage` finding.

## Canonical source kinds (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`source_kind_binding` row for each canonical source kind. Source
kinds and their capability negotiation stay adapter-isolated rather
than flattened into one undifferentiated event stream:

- `native` — native adapter source.
- `bsp` — Build Server Protocol source.
- `bazel_bep` — Bazel Build Event Protocol source.
- `structured_output` — machine-readable runner output.
- `heuristic_parser` — pattern-recognized output.

A missing source kind auto-narrows the lane below `launch_stable`
with a typed `missing_source_kind_coverage` finding.

## Canonical lifecycle events (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`lifecycle_event_binding` row for each canonical lifecycle event so
local, remote/helper, and imported-provider lanes serialize into the
same lifecycle set:

- `task_queued`
- `target_graph_ready`
- `task_started`
- `progress_updated`
- `diagnostic_emitted`
- `test_case_started`
- `test_case_finished`
- `artifact_published`
- `task_finished`

A missing lifecycle event auto-narrows the lane below
`launch_stable` with a typed `missing_lifecycle_event_coverage`
finding.

## Consumer-surface bindings (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`consumer_surface_binding` row for each downstream consumer surface
so each surface reads this packet verbatim instead of paraphrasing
into free-form prose:

- `editor_run_surface`
- `task_panel`
- `test_runner_surface`
- `debug_surface`
- `terminal_pane`
- `cli_headless`
- `ai_tool_surface`
- `review_surface`
- `support_export`

A missing consumer-surface binding auto-narrows the lane below
`launch_stable` with a typed `missing_consumer_surface_coverage`
finding.

## Raw-payload retention attestation

A `raw_payload_retention_attestation` row MUST be present on every
`launch_stable` lane with `attests_raw_payload_retained: true`.
Replay, export, and support packets MUST preserve `source_kind`,
`confidence`, and the adapter raw payload reference rather than
flattening imported, heuristic, and native event streams into one
undifferentiated execution ledger.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces
(event streams, support packets, evidence exports, AI tools, review
surfaces) carry the same lineage id so a "why this event?" question
always resolves to the same execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `editor_run_surface`
- `task_panel`
- `test_runner_surface`
- `debug_surface`
- `terminal_pane`
- `cli_headless`
- `ai_tool_surface`
- `review_surface`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the ten vocabularies
verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`, `preserves_wedge_vocabulary`,
`preserves_envelope_field_vocabulary`,
`preserves_source_kind_vocabulary`,
`preserves_lifecycle_event_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to `narrowed_below_stable`.
The closed finding vocabulary covers missing identity, missing lane
coverage, missing wedge / envelope-field / source-kind /
lifecycle-event / consumer-surface coverage, missing
raw-payload-retention attestation, missing lineage admission,
unbound support / known-limit / downgrade-automation / evidence
bindings, missing or collapsed disclosure refs, raw source material
/ secrets / ambient authority leaks, missing or drifted consumer
projections, and promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-terminal/src/harden_task_test_debug_and_terminal_event_normalization/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, and no surface paraphrases event-normalization truth into
free-form prose.

## See also

- Spec row: `.plans/M04-086.md`
- Reviewer artifact:
  [`artifacts/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md`](../../../artifacts/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md)
- Generator:
  [`tools/regenerate_harden_task_test_debug_and_terminal_event_normalization_truth_packet.py`](../../../tools/regenerate_harden_task_test_debug_and_terminal_event_normalization_truth_packet.py)
- Companion task-event truth packet:
  [`docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md`](./stabilize-task-discovery-launch-profiles-rerun-last-behavior.md)
- Companion terminal stabilization packet:
  [`docs/runtime/m4/stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md`](./stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md)
- Canonical task event stream:
  [`crates/aureline-runtime/src/tasks/`](../../../crates/aureline-runtime/src/tasks/)
- Debug session model:
  [`crates/aureline-runtime/src/debug/`](../../../crates/aureline-runtime/src/debug/)
- Test attempt model:
  [`crates/aureline-runtime/src/tests/`](../../../crates/aureline-runtime/src/tests/)
