# Task-event channel and evidence-link contract

This contract freezes the execution lineage layer that connects command
invocations, terminal streams, task runs, attempts, normalized task
events, output viewers, result packets, artifacts, notifications,
support bundles, provider overlays, imported CI logs, and history rows.
It exists so execution truth can move between UI, CLI/headless,
provider, replay, export, notification, and support surfaces without
each surface inventing a private event vocabulary or a private
cross-reference scheme.

Machine-readable companions:

- [`/schemas/execution/task_channel.schema.json`](../../schemas/execution/task_channel.schema.json)
  - `task_channel_record`, the logical stream or mirror that carries
  output, structured events, diagnostics, artifacts, notification
  mirrors, imported CI logs, or provider-backed overlays.
- [`/schemas/execution/task_event.schema.json`](../../schemas/execution/task_event.schema.json)
  - `task_event_record`, the chronology-bearing event emitted on a
  task channel.
- [`/schemas/execution/evidence_link.schema.json`](../../schemas/execution/evidence_link.schema.json)
  - `evidence_link_record`, the stable edge connecting a command or
  task session to output viewers, result packets, artifacts, support
  bundles, notification lineage, and timeline/history rows.
- [`/fixtures/execution/task_channel_cases/`](../../fixtures/execution/task_channel_cases/)
  - worked fixtures covering live terminal stdout, structured task
  progress, diagnostic feeds, artifact streams, notification mirrors,
  imported CI logs, provider overlays, support export links, and replay
  links.

This contract composes with and does not replace:

- [`/docs/execution/run_and_attempt_contract.md`](./run_and_attempt_contract.md)
  and its run, attempt, outcome, rerun-comparison, and artifact-event
  schemas. Runs and attempts remain the user-visible execution
  identity. Channels and task events are the append-only stream and
  route layer attached to those records.
- [`/docs/tooling/task_event_contract_seed.md`](../tooling/task_event_contract_seed.md)
  and
  [`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json).
  The tooling envelope remains the lower-level adapter event that
  preserves source kind, confidence, raw-payload retention, and adapter
  provenance. The task-event record here references that envelope by
  trace or event id when one exists.
- [`/docs/execution/terminal_truth_contract.md`](./terminal_truth_contract.md).
  Terminal stdout and stderr channels cite terminal-session records and
  never regain authority from restored or imported transcripts.
- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  and
  [`/schemas/ux/output_viewer_object.schema.json`](../../schemas/ux/output_viewer_object.schema.json).
  Output viewers render channel contents but do not mint separate
  buffering, truncation, imported, or freshness semantics.
- [`/docs/ux/notification_delivery_contract.md`](../ux/notification_delivery_contract.md)
  and
  [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json).
  Notification mirrors reuse canonical event lineage and route refs.
- [`/docs/ux/chronology_row_contract.md`](../ux/chronology_row_contract.md)
  and
  [`/schemas/ux/history_row.schema.json`](../../schemas/ux/history_row.schema.json).
  Timeline and history rows read evidence links rather than scraping
  output viewers or result packets.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Architecture_Document.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those upstream
  documents disagree with this contract, the upstream documents win
  and this contract plus its schemas update in the same change.

## Scope

Frozen at this revision:

- one channel-class vocabulary for terminal stdout, terminal stderr,
  combined terminal stdio, structured task-event streams, diagnostics
  feeds, artifact streams, notification mirrors, imported CI logs, and
  provider-backed execution overlays;
- one task-event type vocabulary for `queued`, `started`, `waiting`,
  `progress`, `retry`, `partial`, `paused`, `cancelled`, `completed`,
  `imported`, `exported`, and `replayed` states;
- chronology fields on every task event: stable sequence, predecessor,
  causal parent, observed / emitted / ingested timestamps, actor, and
  route refs;
- channel-level and event-level truncation, buffering, backlog, and
  imported-versus-live disclosure classes;
- one evidence-link record with stable fields for command invocation,
  terminal session, run, attempt, execution context, task channel, task
  event, output viewer, result packet, artifact event, created artifact,
  support bundle, notification lineage, and history row refs.

Out of scope at this revision:

- implementing task runners, result viewers, provider adapters, or
  support exporters;
- replacing the lower-level tooling task-event envelope;
- allowing imported, replayed, or provider-backed evidence to acquire
  local live authority.

## Channel model

A `task_channel_record` is one logical stream or mirror that can be
joined to runs, attempts, terminal sessions, output viewers, result
packets, artifacts, notifications, support bundles, and history rows.

| Channel class | Meaning | Required honesty |
|---|---|---|
| `terminal_stdout_stream` | stdout from a terminal-backed command or task | cites terminal session, run or attempt, and output viewer refs when rendered |
| `terminal_stderr_stream` | stderr from a terminal-backed command or task | same lineage as stdout; stderr does not become a separate task identity |
| `terminal_combined_stdio_stream` | folded stdout/stderr channel where the producer cannot separate streams | discloses combined-stream posture and cannot be re-split by viewers |
| `structured_task_event_stream` | normalized lifecycle, progress, retry, and outcome events | cites tooling envelope trace refs when adapter events exist |
| `diagnostics_feed` | compiler, linter, test, scanner, or runtime diagnostics | keeps diagnostic producer and freshness separate from task state |
| `artifact_stream` | emitted reports, traces, binaries, bundles, previews, and streamed chunks | cites artifact-event refs and retention posture |
| `notification_mirror` | task state mirrored to toast, banner, status, companion, or digest surfaces | cites event-lineage refs and preserves durable linkback |
| `imported_ci_log` | CI or provider logs imported from a prior external run | read-only imported snapshot, with source and captured-at refs |
| `provider_backed_execution_overlay` | hosted check, pipeline, managed job, or provider execution state layered over local truth | provider source remains visible and cannot overwrite local execution records |

Channel records carry both `channel_origin_class` and
`channel_liveness_class`. A live local or remote channel can be
`live_authoritative`, `live_buffering`, or `live_backlog_lagging`.
Imported CI logs, support-bundle replay, exported snapshots, and
provider overlays use read-only liveness classes even when the provider
is currently reachable. That distinction is what keeps an imported
green check from masquerading as a current local pass.

## Task-event model

A `task_event_record` is the event emitted on a channel. It does not
replace run, attempt, or artifact-event records; it ties those records
to chronology and routes.

| Task-event type | Minimum meaning |
|---|---|
| `queued` | accepted or held in a lane before execution begins |
| `started` | execution, setup, or adapter dispatch has started |
| `waiting` | blocked on bounded user input, approval, dependency, capacity, or provider state |
| `progress` | partial progress, counter update, phase change, or heartbeat |
| `retry` | retry scheduled or started with predecessor refs |
| `partial` | usable subset exists while another declared subset is pending, failed, omitted, or degraded |
| `paused` | execution or stream delivery is intentionally paused |
| `cancelled` | user, policy, supervisor, timeout, quarantine, or provider cancelled work |
| `completed` | declared terminal state was reached, with result packet or outcome refs when available |
| `imported` | external CI, scanner, provider, or bundle evidence was imported as read-only evidence |
| `exported` | this event or channel was included in an evidence, support, or chronology export |
| `replayed` | event was reconstructed from a support bundle, replay corpus, or exported snapshot |

Every task event carries:

- `task_event_id`, `task_channel_ref`, and `task_event_sequence`;
- `chronology` with `observed_at`, `emitted_at`, `ingested_at`,
  `predecessor_event_ref`, and `causal_parent_event_ref`;
- `actor` with typed actor kind and actor ref;
- `route_refs` for command invocation, terminal session, run, attempt,
  output viewer, notification lineage, history row, and tooling-envelope
  refs;
- `buffering_state`, including buffering, backlog, truncation, and
  omitted-count posture;
- `source_disclosure_class`, which preserves live, imported,
  provider-overlay, exported, and replayed distinctions.

## Evidence links

An `evidence_link_record` is a first-class edge. It is the only
contracted way for consumers to reconstruct task lineage across output
viewers, result packets, notifications, support exports, and history
rows. Consumers must not parse labels, URLs, row text, viewer titles,
or provider payloads to infer lineage.

Stable linkage fields are:

- `command_invocation_ref`
- `terminal_session_ref`
- `parent_run_ref`
- `parent_attempt_ref`
- `execution_context_ref`
- `task_channel_ref`
- `task_event_ref`
- `output_viewer_ref`
- `result_packet_ref`
- `artifact_event_ref`
- `created_artifact_ref`
- `support_bundle_ref`
- `history_row_ref`
- `notification_lineage_ref`

The same evidence link may connect a command session to an output
viewer, an attempt to a created artifact, a task event to a history row,
an imported CI log to a result packet, a provider overlay to a channel,
or a replay bundle to reconstructed events. It always carries
`link_route_class` and `live_disclosure_class` so the target consumer
knows whether it is reading live execution, a buffered live stream, a
provider overlay, an imported snapshot, a replayed snapshot, or an
exported snapshot.

## Buffering, truncation, and backlog rules

1. Channel policy declares the expected buffer, backlog, and truncation
   posture. Event records declare what actually happened for each
   emitted event.
2. Output viewers, result packets, support bundles, notification
   mirrors, and history rows must project buffering and truncation from
   task events and evidence links. They must not invent private labels
   such as "large output" or "cached log" when the task-event layer
   already says `live_backlog_lagging`, `truncated_tail`, or
   `provider_retention_boundary`.
3. A backlog state is not a failure. It is a state with ordered
   sequence, known or unknown counts, and an explicit drain or drop
   posture. Dropped events require an emitted task event with
   `buffering_state_class = dropped_overflow_emitted`.
4. Truncation requires a typed `truncation_class`, omitted counts when
   known, and a short export-safe note. Raw stdout, stderr, provider log
   bodies, environment bytes, absolute paths, URLs, and secrets never
   cross these schemas.
5. Imported, replayed, exported, and provider-backed evidence keeps its
   read-only disclosure class in channels, events, evidence links,
   viewers, notifications, and support exports. A later local rerun
   creates a new live channel or attempt; it does not mutate the
   imported channel into live truth.

## Acceptance invariants

- Output viewers, result packets, notifications, and support exports
  reconstruct task lineage through `task_channel_ref`,
  `task_event_ref`, and `evidence_link_record` fields.
- Imported CI logs and provider evidence always carry read-only
  imported or provider-overlay disclosure and cannot bypass the live
  local/remote liveness classes.
- Command, terminal, run, attempt, event, artifact, support, and
  history packets use stable linkage fields, never freeform
  cross-reference text.
- Buffering, backlog, truncation, export, import, and replay posture is
  recorded at the task-event layer so viewers and exports stay
  synchronized.

## Fixture coverage

The fixture corpus under
[`/fixtures/execution/task_channel_cases/`](../../fixtures/execution/task_channel_cases/)
keeps at least:

- one live terminal stdout channel with an output viewer route;
- one structured progress event with backlog disclosure;
- one diagnostic feed channel;
- one artifact stream channel with artifact-event refs;
- one notification mirror channel with event-lineage refs;
- one imported CI log event that is read-only and source-attributed;
- one provider overlay channel that remains separate from local truth;
- one evidence link from execution to a support bundle;
- one replay evidence link back to reconstructed task events.
