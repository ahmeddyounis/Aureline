# Task-channel, task-event, and evidence-link worked cases

These fixtures anchor the execution channel and evidence-link contract
defined in
[`/docs/execution/task_event_and_evidence_contract.md`](../../../docs/execution/task_event_and_evidence_contract.md)
and the boundary schemas:

- [`/schemas/execution/task_channel.schema.json`](../../../schemas/execution/task_channel.schema.json)
- [`/schemas/execution/task_event.schema.json`](../../../schemas/execution/task_event.schema.json)
- [`/schemas/execution/evidence_link.schema.json`](../../../schemas/execution/evidence_link.schema.json)

Each fixture is a single record. The corpus is intentionally small and
pre-implementation: it shows the identity and disclosure spine that
future runners, viewers, notifications, exports, and provider adapters
must share.

## Scope rules

- Fixtures use opaque ids, class labels, hashes, counts, and UTC
  timestamps only.
- Raw command lines, stdout/stderr bytes, provider log bodies, env
  bodies, absolute paths, URLs, hostnames, and secrets are not allowed.
- Imported CI logs, provider overlays, support replay, and exported
  snapshots keep read-only disclosure classes.
- Output viewers, result packets, notification lineage, support
  bundles, and history rows are linked through stable fields, not
  freeform prose.

## Index

| Fixture | Schema | Key coverage |
|---|---|---|
| [`live_terminal_stdout_channel.yaml`](./live_terminal_stdout_channel.yaml) | `task_channel_record` | terminal stdout stream with live local liveness, terminal/run/attempt refs, output viewer route, and viewer/export sync |
| [`structured_progress_backlog_event.yaml`](./structured_progress_backlog_event.yaml) | `task_event_record` | progress event with known backlog and no truncation, routed to output viewer and history row |
| [`diagnostic_feed_channel.yaml`](./diagnostic_feed_channel.yaml) | `task_channel_record` | diagnostics feed separate from task state and tied to a structured result packet |
| [`artifact_stream_completed_event.yaml`](./artifact_stream_completed_event.yaml) | `task_event_record` | completed event with result packet and artifact-event refs |
| [`notification_mirror_channel.yaml`](./notification_mirror_channel.yaml) | `task_channel_record` | notification mirror with canonical event-lineage route refs |
| [`imported_ci_log_event.yaml`](./imported_ci_log_event.yaml) | `task_event_record` | imported CI evidence, source-attributed and read-only |
| [`provider_overlay_channel.yaml`](./provider_overlay_channel.yaml) | `task_channel_record` | provider-backed execution overlay that cites provider and local truth refs without replacing local truth |
| [`support_bundle_evidence_link.yaml`](./support_bundle_evidence_link.yaml) | `evidence_link_record` | task session to support bundle export with exported-snapshot disclosure |
| [`replay_bundle_evidence_link.yaml`](./replay_bundle_evidence_link.yaml) | `evidence_link_record` | replay bundle to reconstructed task event with replayed-snapshot disclosure |

Removing one of these coverage classes is a breaking change.
