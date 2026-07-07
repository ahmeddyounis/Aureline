# M5 AI rerun-review-sheet, incomplete-replay-banner, and agent-status-card primitive contract

Task: **M05-881** — Implement rerun-review sheets, incomplete-replay banners, paused-or-blocked
cards, takeover summary cards, re-review banners, and agent-run-lineage rows with
drift-checkpoint-continue-manually truth across claimed M5 background-agent flows.

This lane narrows the `replay_review` and `agent_status` families from the frozen
[AI-execution/replay component matrix](./freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md)
(M05-876) into three reusable replay / agent-status primitives and one shared parity matrix, so
background AI work stays honest about rerun drift and safe takeover rather than appearing alive
or reusable by implication alone.

## Primitives

- Rerun-review resolver: `resolve_rerun_review_sheet(&M5AiRerunReviewResolutionInput) -> Result<M5ResolvedRerunReviewSheet, M5AiRerunReviewResolutionError>`.
- Incomplete-replay resolver: `resolve_incomplete_replay_banner(&M5AiIncompleteReplayResolutionInput) -> Result<M5ResolvedIncompleteReplayBanner, M5AiIncompleteReplayResolutionError>`.
- Agent-status resolver: `resolve_agent_status_card(&M5AiAgentStatusResolutionInput) -> Result<M5ResolvedAgentStatusCard, M5AiAgentStatusResolutionError>`.
- Parity matrix packet: `M5AiBackgroundAgentReplayPrimitivePacket`, one row per claimed
  background-agent surface, each carrying worked rerun-review, incomplete-replay, and
  agent-status resolution cases.

## Rerun-review sheets (requirement: compare original lineage vs current state, name approval reuse)

A rerun-review sheet must compare the original run's branch/base/provider/model/policy lineage
with the current state and name whether approval reuse is allowed. The resolver enforces this:

- The **canonical run id** is carried through verbatim so the rerun output links back to its
  source. The current provider and model must both be named — a masked route is rejected as
  `route_provider_model_masked`.
- The **rerun-review reason** is derived from the drifted dimensions with a fixed precedence —
  `model_version_changed`, then `tool_contract_changed`, then `route_or_provider_changed`, then
  `policy_changed`, then (any other drift) `input_context_changed`, else `no_re_review_required`.
- **Approval reuse** is allowed only when the original approvals are still effective and no
  approval-relevant dimension (provider route, model version, policy epoch, tool contract)
  drifted. The **admission** is `blocked_on_provider_drift` on a provider/model drift,
  `blocked_pending_approval` on any other approval-relevant drift, `admit_after_re_review` when
  reuse is allowed but non-approval drift still requires re-review, and
  `admit_with_approval_reuse` for an undrifted rerun.

## Incomplete-replay banners (requirement: retained vs missing, why new approvals)

An incomplete-replay banner must explain which parts were retained versus missing and why new
approvals are required before a rerun. The resolver enforces this:

- The banner always names the **retained** and **missing** replay segments (prompt transcript,
  tool-call log, route receipt, approval lineage, diff packet, provider response), so a user can
  tell which parts of the replay survived.
- A replay marked `fully_replayable` that still declares missing segments is rejected as
  `complete_but_segments_missing` — completeness is never overstated.
- A rerun **requires new approvals** whenever the replay is not fully complete or the approval
  lineage itself is among the missing segments.

## Agent-status cards (requirement: checkpoint, blast radius, last step, pending writes, safe options)

Paused-or-blocked cards, takeover summaries, re-review banners, and agent-run-lineage rows must
show checkpoint state, current blast radius, last successful step, pending writes, and safe
continue-manually or restart options. The resolver enforces this:

- The agent **presents as alive only when it is really running** — a paused, blocked,
  awaiting-takeover, handed-off, completed, or abandoned agent never reads as alive by
  implication.
- An interrupted agent that holds pending writes but carries no checkpoint is rejected as
  `interrupted_with_pending_writes_but_no_checkpoint`, so no work is silently at risk on takeover.
- The safe **continue options** are derived from the checkpoint, the pending-writes state, and
  whether the agent is interrupted, and always include the non-mutating `review_checkpoint` and
  `escalate_to_owner` paths. `restart_clean` is offered only when there are no pending writes to
  lose; `abort_with_checkpoint` only when there are.

## Shared parity matrix

One row per claimed background-agent surface — **rerun-review**, **branch-agent console**,
**run-history**, **support**, and **CLI** — binds the shared rerun-review, incomplete-replay, and
agent-status anatomy, the continue options, execution modes, run outcomes, approval gates,
rerun-review reasons, rerun admissions, drift dimensions, replay-completeness states, replay
segments, agent lifecycle states, takeover paths, blast radii, export fields, and non-visual
accessibility routes, so the same run lineage, rerun grammar, and safe-takeover vocabulary stay
identical across every surface.

Each row carries four hard invariants (all must be `false`):

- `masks_run_lineage_across_surfaces`,
- `presents_interrupted_agent_as_alive`,
- `overstates_replay_completeness`,
- `invents_parallel_rerun_or_agent_grammar`.

## Acceptance-criteria lints

The packet `validate()` enforces four cross-matrix lints proving the acceptance criteria:

- `run_lineage_consistency_unproven` — the same canonical run lineage appears in a rerun-review
  example, an incomplete-replay example, and an agent-status example, proving manual takeover and
  replay/export paths preserve run lineage across UI and support exports.
- `drift_disclosure_unproven` — at least one rerun-review example proves a blocked rerun with
  named drift, so a user can tell why rerun needs re-review and what changed.
- `interrupted_agent_honesty_unproven` — at least one agent-status example shows an interrupted
  agent that is not alive and still offers a safe continue option.
- `incomplete_replay_reapproval_unproven` — at least one incomplete-replay example proves an
  incomplete replay requiring new approvals with named retained and missing segments.

## Reused vocabulary

The run outcome, execution mode, approval gate, replay-completeness state, rerun-review reason,
agent lifecycle state, takeover path, surface family, deployment line, consumer surface,
accessibility route, qualification class, and downgrade trigger are reused verbatim from the
frozen matrix. This lane mints new vocabulary only for the background-agent surfaces, anatomy
parts, rerun drift dimensions, rerun admissions, replay segments, agent blast radii, safe continue
options, and export fields the sheet, banner, and card themselves add. No M5 AI surface invents a
second rerun-review or agent-status grammar.

## Source contracts

- Boundary schema: `schemas/ai/m5-ai-rerun-review-sheet-incomplete-replay-banner-and-agent-status-card.schema.json`.
- Frozen component matrix: `schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json`.
- AI rerun reviews: `schemas/ai/ai_rerun_review.schema.json`.
- Evidence replay packets: `schemas/ai/evidence_replay_packet.schema.json`.
- Background branch-agent runs: `schemas/ai/background-branch-agent-run.schema.json`.

## Artifacts

- Support export: `artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/support_export.json`.
- Matrix CSV: `artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/matrix.csv`.
- Markdown report: `artifacts/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows.md`.
- Narrowed fixtures: `fixtures/ai/m5/implement_rerun_review_sheets_incomplete_replay_banners_paused_or_blocked_cards_takeover_summaries_rereview_banners_and_agent_run_lineage_rows_across_claimed_m5_background_agent_flows/`.

The headless emitter
`aureline_ai_rerun_review_incomplete_replay_agent_status_primitive` is the only
mint-from-truth path for these artifacts.
