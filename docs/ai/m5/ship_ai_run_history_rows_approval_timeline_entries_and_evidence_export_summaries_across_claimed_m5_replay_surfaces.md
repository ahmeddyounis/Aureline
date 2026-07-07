# M5 AI run-history-row, approval-timeline-entry, and evidence-export-summary primitive contract

Task: **M05-880** — Ship AI run-history rows, approval-timeline entries, and evidence-export
summaries with canonical run IDs, provider-model route truth, redaction state, and
support-packet linkage across claimed M5 replay surfaces.

This lane narrows the `run_history_row` and `replay_review` families from the frozen
[AI-execution/replay component matrix](./freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md)
(M05-876) into three reusable history / export primitives and one shared parity matrix, so
prior AI work stays auditable and shareable without reconstructing it from generic logs.

## Primitives

- Run-history resolver: `resolve_run_history_row(&M5AiRunHistoryResolutionInput) -> Result<M5ResolvedRunHistoryRow, M5AiRunHistoryResolutionError>`.
- Approval-timeline resolver: `resolve_approval_timeline_entry(&M5AiApprovalTimelineResolutionInput) -> Result<M5ResolvedApprovalTimelineEntry, M5AiApprovalTimelineResolutionError>`.
- Evidence / export summary resolver: `resolve_evidence_export_summary(&M5AiEvidenceSummaryResolutionInput) -> Result<M5ResolvedEvidenceExportSummary, M5AiEvidenceSummaryResolutionError>`.
- Parity matrix packet: `M5AiRunHistoryExportPrimitivePacket`, one row per claimed replay
  surface, each carrying worked run-history, approval-timeline, and evidence-summary
  resolution cases.

## Run-history rows (requirement: canonical run ID, task label, time, provider/model route, outcome, stable entry points)

A run-history row must show the canonical run ID, task label, time, provider/model route,
outcome, and stable open / replay / export entry points. The resolver enforces this:

- The **canonical run id** is carried through verbatim so the same AI run identity appears
  consistently across history, evidence, export, support, and replay surfaces.
- The provider and model must both be named — a masked route is rejected as
  `route_provider_model_masked` rather than shown as an anonymous run — and the composed
  `route_label` is `"{provider} / {model}"`.
- The row always offers the stable `open_run` / `replay_run` / `export_evidence` entry
  points, adds `view_support_packet` when the run is support-linked, and adds
  `inspect_approvals` when approvals influenced the run.

## Approval-timeline entries (requirement: actor, scope, policy epoch, expiry, inspectability)

An approval-timeline entry must preserve the actor, scope, policy epoch, expiry, and
inspectability for approvals that influenced the run. The resolver enforces this:

- Actor, actor class, grant scope, policy epoch, and gate are preserved distinctly, so
  approval history can be inspected and never collapses multiple distinct grants into one
  vague "approved" badge.
- The **expiry state** is derived with a fixed precedence — `revoked`, then
  `single_use_consumed`, then `expired`, then `expiring_soon`, then `active` (with an
  expiry) or `no_expiry` — and the grant is `effective` only while active, expiring-soon,
  or non-expiring. An entry claiming an expiry with no timestamp is rejected as
  `expiry_claimed_without_timestamp`.
- An approval that influenced the run must be inspectable; a non-inspectable entry is
  rejected as `approval_not_inspectable`.

## Evidence / export summaries (requirement: packet ID, artifact classes, redaction posture, support linkage, export formats)

An evidence / export summary must show the packet ID, included artifact classes, redaction
posture, support-packet linkage, and supported export formats. The resolver enforces this:

- The summary always carries the packet id, the run it belongs to, its artifact classes,
  its redaction posture, its support linkage, and its export formats — so it preserves
  redaction and support-continuity state instead of collapsing to a raw-file download link,
  which is rejected as `raw_download_only`.
- A packet is `shareable` only when its redaction posture has at least removed its
  credentials (`fully_redacted`, `credentials_redacted`, or `pii_redacted`); an
  `unredacted`, `redaction_pending`, or `redaction_failed` packet resolves to
  `is_shareable = false`.

## Shared parity matrix

One row per claimed replay surface — **run-history**, **evidence-packet**, **export**,
**support**, and **replay** — binds the shared run-history, approval-timeline, and
evidence-summary anatomy, the entry points, execution modes, run outcomes, approval actor
classes, grant scopes, expiry states, approval gates, artifact classes, redaction postures,
support linkages, export formats, export fields, and non-visual accessibility routes, so
the same AI run identity, approval grammar, and export vocabulary stay identical across
every surface.

Each row carries four hard invariants (all must be `false`):

- `masks_run_identity_across_surfaces`,
- `collapses_multiple_grants_into_one_badge`,
- `offers_raw_download_links_only`,
- `invents_parallel_history_or_export_grammar`.

## Acceptance-criteria lints

The packet `validate()` enforces four cross-matrix lints proving the acceptance criteria:

- `run_identity_consistency_unproven` — the same canonical run identity appears in a
  run-history example, an evidence example, and an approval example, proving one run
  identity stays consistent across history, evidence, export, support, and replay.
- `multiple_distinct_grants_unproven` — at least one row proves two distinct grants
  (distinct actor class and scope), so approval history never collapses into one vague
  badge.
- `expiry_honesty_unproven` — at least one approval example shows an expired / revoked /
  consumed grant as no longer effective.
- `redaction_support_continuity_unproven` — at least one evidence example proves a
  shareable, support-linked summary that preserves redaction and support-continuity state
  rather than only a raw-file download link.

## Reused vocabulary

The run outcome, execution mode, approval gate, surface family, deployment line, consumer
surface, accessibility route, qualification class, and downgrade trigger are reused verbatim
from the frozen matrix. This lane mints new vocabulary only for the replay surfaces, anatomy
parts, entry points, approval actor classes, grant scopes, expiry states, evidence artifact
classes, redaction postures, support linkages, export formats, and export fields the row,
entry, and summary themselves add. No M5 AI surface invents a second run-history, approval,
or export grammar.

## Source contracts

- Boundary schema: `schemas/ai/m5-ai-run-history-row-approval-timeline-entry-and-evidence-export-summary.schema.json`.
- Frozen component matrix: `schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json`.
- AI run-history entries: `schemas/ai/ai_run_history_entry.schema.json`.
- Approval action classes and grant vocabulary: `schemas/ai/approval_action_class.schema.json`.
- Evidence replay packets: `schemas/ai/evidence_replay_packet.schema.json`.

## Artifacts

- Support export: `artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/support_export.json`.
- Matrix CSV: `artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/matrix.csv`.
- Markdown report: `artifacts/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces.md`.
- Narrowed fixtures: `fixtures/ai/m5/ship_ai_run_history_rows_approval_timeline_entries_and_evidence_export_summaries_across_claimed_m5_replay_surfaces/`.

The headless emitter
`aureline_ai_run_history_approval_timeline_evidence_export_primitive` is the only
mint-from-truth path for these artifacts.
