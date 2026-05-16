# Beta Task-Event Model

This document is the reviewer-facing landing page for the beta finalize layer
of the canonical task-event model. It pins the closed set of beta task-event
lanes, the wedges and event kinds each lane is allowed to emit, and the
consumer surfaces that read the typed event stream. The machine-readable
boundary lives at
[`/schemas/runtime/task_event.schema.json`](../../../schemas/runtime/task_event.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/tasks/`](../../../crates/aureline-runtime/src/tasks/)
(canonical model) and
[`/crates/aureline-runtime/src/task_events/`](../../../crates/aureline-runtime/src/task_events/)
(beta lane manifest).

The beta promise:

- run, test, debug, review, AI, and support-export consumers read **one**
  typed event envelope with the same wedge, event-kind, payload, source-kind,
  confidence, and retention vocabularies;
- every consumer surface — shell, activity center, support bundle export, AI
  review, code review — reads the same stream without forking its own
  parser;
- partial or degraded task states surface as the typed
  `degraded_state_reported` event with a closed `task_degradation_reason`
  vocabulary, never as a free-form console string;
- every typed event retains a raw adapter-origin envelope ref so support
  exports can be replayed truthfully.

## Beta lanes

| Lane | Wedges | Consumer surfaces |
| --- | --- | --- |
| `run` | `build`, `terminal`, `package`, `notebook`, `generic` | shell, activity center, support bundle export |
| `test` | `test` | shell, activity center, support bundle export |
| `debug` | `debug` | shell, activity center, support bundle export |
| `review` | `review` | shell, activity center, support bundle export |
| `ai` | `ai_tool` | shell, activity center, support bundle export |
| `support_export` | every wedge above | support bundle export |

The canonical manifest is exposed by
[`TaskEventBetaCoverageManifest::canonical`](../../../crates/aureline-runtime/src/task_events/mod.rs)
and checked in at
[`/fixtures/runtime/task_event_beta/beta_lane_coverage.json`](../../../fixtures/runtime/task_event_beta/beta_lane_coverage.json).

## Event kinds

Every beta lane is allowed to emit the same closed set of event kinds. Adding
or removing a kind is a vocabulary change that must update the schema, the
canonical manifest, and this doc together.

| Kind | Use |
| --- | --- |
| `task_queued` | Scheduler accepted the task |
| `task_started` | Process / adapter / remote run started |
| `task_blocked` | Task blocked on a non-input dependency |
| `input_requested` | Task is waiting for typed input |
| `progress_updated` | Counters or step labels advanced |
| `output_appended` | Stdout, stderr, system, or adapter output |
| `diagnostic_emitted` | Tool diagnostic referenced |
| `artifact_published` | Build output, coverage, report, log slice, profile, or debug artifact |
| `degraded_state_reported` | Typed partial / degraded posture (see below) |
| `task_completed` | Terminal — success |
| `task_failed` | Terminal — failure |
| `task_cancelled` | Terminal — cancelled |

## Typed degraded states

Earlier task lanes expressed partial or degraded posture only through
free-form console text; the beta model promotes the closed
`task_degradation_reason` vocabulary so degraded posture is consumable by the
shell, activity center, support export, AI review, and human review on the
same path as every other typed event.

| Reason | Meaning |
| --- | --- |
| `progress_unavailable` | Output is reaching consumers but progress counters are unavailable |
| `adapter_capability_dropped` | Adapter advertised fewer capabilities than the canonical request set |
| `fallback_parser_active` | Heuristic / fallback parser is filling in for a missing structured source |
| `output_truncated` | Output bytes were truncated by retention or policy |
| `partial_output_only` | Only a subset of streams or artifacts is reaching consumers |
| `transient_target_unreachable` | Target became unreachable but the task is expected to resume |
| `policy_partial_visibility` | Policy is hiding part of the run from this surface |

A degraded event carries the typed reason plus an export-safe `scope_label`
and an optional `recovery_hint_ref` (doctor row, operator note, or runbook).
The current degraded posture is also surfaced on the activity-center row so
support exports and review consumers do not have to scan the event log.

## Support exports

The support-export packet
[`TaskSupportExport`](../../../crates/aureline-runtime/src/tasks/mod.rs)
replays each typed event as a `task_support_event_row` plus the retained
raw adapter envelope. The row carries the event id, run / attempt id,
execution-context ref, target id, wedge, event kind, state after, source
kind, confidence, redaction class, raw envelope ref, payload digest, and the
typed degradation reason when one was reported. AI review, code review, and
human support pull this row set directly; they do not parse adapter output.

## Cross-references

- Alpha model and fixtures —
  [`/fixtures/runtime/task_event_alpha/`](../../../fixtures/runtime/task_event_alpha/)
- Beta lane manifest and review / AI stream fixtures —
  [`/fixtures/runtime/task_event_beta/`](../../../fixtures/runtime/task_event_beta/)
- Execution-context provenance contract —
  [`/docs/runtime/execution_context_seed.md`](../execution_context_seed.md)
- Beta debugger / DAP host — [`debugger_host_beta.md`](debugger_host_beta.md)
- Beta run / debug profile model — [`run_debug_profiles_beta.md`](run_debug_profiles_beta.md)
