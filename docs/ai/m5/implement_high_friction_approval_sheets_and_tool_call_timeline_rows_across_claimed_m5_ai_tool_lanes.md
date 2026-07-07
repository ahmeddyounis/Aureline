# M5 AI high-friction-approval-sheet and tool-call-timeline-row primitive contract

Task: **M05-879** — Implement high-friction approval sheets and tool-call timeline rows
with requested-action scope, side-effects, boundary, rollback, and provenance-removal
controls across the claimed M5 AI tool lanes.

This lane narrows the `approval_sheet` and `tool_call_timeline_row` families from the
frozen [AI-execution/replay component matrix](./freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md)
(M05-876) into two reusable primitives: an approval-sheet resolver, a tool-call-timeline
resolver, and one shared parity matrix. A user can tell — from the sheet or the row alone
— what action is requested, what scope and side effects it carries, which boundary it
crosses, what rollback or checkpoint backs it, and which governed follow-up actions apply,
before a mutating or boundary-crossing action can slip by as an ordinary status update.

## Primitives

- Approval-sheet resolver: `resolve_approval_sheet(&M5AiApprovalSheetResolutionInput) -> Result<M5ResolvedApprovalSheet, M5AiApprovalSheetResolutionError>`.
- Tool-call resolver: `resolve_tool_call_timeline_row(&M5AiToolCallResolutionInput) -> Result<M5ResolvedToolCallTimelineRow, M5AiToolCallResolutionError>`.
- Parity matrix packet: `M5AiApprovalToolCallPrimitivePacket`, one row per claimed tool
  lane, each carrying worked approval-sheet and tool-call resolution cases.

## Approval sheets (requirement: requested action, scope, side effects, boundary, rollback/checkpoint, and explicit controls)

An approval sheet must show the requested action, scope, side effects, boundary,
rollback / checkpoint, and explicit approve-once / deny / open-plan controls. The
resolver enforces this:

- A mutating or boundary-crossing action can never read as an auto-approved or
  notify-only status update — the resolver rejects that as
  `mutating_action_masked_as_status`.
- The **effective approval gate** is the friction floor the action's side effect,
  boundary, and friction reasons imply, escalated no lower than the declared gate:
  - a `policy_mandated_review` friction reason forces a **two-person review**;
  - `irreversible_side_effect`, `destructive_file_change`, `credential_access`, or
    `cross_tenant_scope` friction forces a **high-friction typed** confirmation;
  - any other mutation, external egress, or boundary crossing forces at least a
    **one-click confirm**;
  - a declared `policy_blocked` gate stays blocked (and offers no approve-once control).
- The explicit controls always carry the approve-once / deny / open-plan triad (deny +
  open-plan only when policy-blocked), plus `review_rollback_checkpoint` when a
  checkpoint backs the action and `escalate_second_reviewer` for a two-person review.
- A `checkpoint_backed` rollback posture must carry a checkpoint ref
  (`checkpoint_claimed_without_ref` otherwise).

## Tool-call timeline rows (requirement: time, tool, side-effect class, boundary, outcome, and governed follow-up actions)

A tool-call timeline row must show the time, tool, side-effect class, boundary, outcome,
and governed follow-up actions. The resolver enforces this:

- The **observed** side-effect class is carried explicitly and compared against the
  prediction, so a call that escalated (for example, a predicted `read_only` call that
  observed a `state_mutation`) is flagged (`effect_escalated`) rather than shown as
  read-only.
- The governed follow-up actions keep provenance and removal controls visible instead of
  buried in a raw log — `view_provenance` is always offered, `remove_from_context`
  whenever the result is still in the active context, `open_output` when output is
  available, and `replay_in_sandbox` / `renew_approval` for mutating or boundary-crossing
  calls.

## Shared parity matrix

One row per claimed tool lane — **read-only tool invocation**, **mutating tool run**,
**test-generation validation**, **branch-agent checkpoint**, and **CLI / support export**
— binds the shared approval-sheet and tool-call anatomy, the action scopes, side-effect
classes, tool boundaries, rollback postures, approval gates, friction reasons, run
outcomes, approval controls, follow-up actions, export fields, and non-visual
accessibility routes, so the action-class and rollback vocabulary stays identical across
every lane and matches the policy and evidence systems.

Each row carries four hard invariants (all must be `false`):

- `masks_mutation_or_boundary_as_status`,
- `buries_provenance_or_removal_in_logs`,
- `drops_rollback_or_checkpoint_vocabulary`,
- `invents_parallel_approval_or_tool_grammar`.

## Acceptance-criteria lints

The packet `validate()` enforces four cross-matrix lints proving the acceptance criteria:

- `mutating_action_review_first_unproven` — at least one approval example proves a
  mutating / boundary-crossing action held review-first at a high-friction gate.
- `approval_control_triad_unproven` — at least one approval example offers the explicit
  approve-once / deny / open-plan control triad.
- `tool_call_provenance_removal_unproven` — at least one tool-call example keeps both the
  provenance and the removal controls visible.
- `tool_call_effect_honesty_unproven` — at least one tool-call example proves an escalated
  (observed worse than predicted) effect.

## Reused vocabulary

The approval gate, friction reason, tool boundary, side-effect class, run outcome,
surface family, deployment line, consumer surface, accessibility route, qualification
class, and downgrade trigger are reused verbatim from the frozen matrix. This lane mints
new vocabulary only for the tool lanes, anatomy parts, action scopes, rollback postures,
approval controls, follow-up actions, and export fields the sheet and row themselves add.
No M5 AI surface invents a second approval or tool-call grammar.

## Source contracts

- Boundary schema: `schemas/ai/m5-ai-high-friction-approval-sheet-and-tool-call-timeline-row.schema.json`.
- Frozen component matrix: `schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json`.
- Approval action classes and rollback vocabulary: `schemas/ai/approval_action_class.schema.json`.
- Tool-call timeline entries: `schemas/ai/tool_call_timeline_entry.schema.json`.

## Artifacts

- Support export: `artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/support_export.json`.
- Matrix CSV: `artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/matrix.csv`.
- Markdown report: `artifacts/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes.md`.
- Narrowed fixtures: `fixtures/ai/m5/implement_high_friction_approval_sheets_and_tool_call_timeline_rows_across_claimed_m5_ai_tool_lanes/`.

The headless emitter
`aureline_ai_high_friction_approval_sheet_tool_call_timeline_row_primitive` is the only
mint-from-truth path for these artifacts.
