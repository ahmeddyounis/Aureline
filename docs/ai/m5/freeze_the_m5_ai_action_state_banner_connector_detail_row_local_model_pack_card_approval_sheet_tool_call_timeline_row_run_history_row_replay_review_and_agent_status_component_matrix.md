# M5 AI-execution/replay component matrix contract

**Row:** M05-876 — Freeze the M5 AI-action-state-banner, connector-detail-row,
local-model-pack-card, approval-sheet, tool-call-timeline-row, run-history-row,
replay-review, and agent-status component matrix (batch B103).

This contract freezes the reusable **AI execution and replay component matrix** so
AI mode, route, approval, and replay language stop drifting across M5 consumers. It
is the AI-domain analog of the shell runtime-boundary
(`freeze_the_m5_runtime_boundary_*`), release-center
(`freeze_the_m5_release_candidate_card_*`), and docs-browser
(`freeze_the_m5_docs_search_bar_*`) component freezes.

- **Crate / module:** `aureline-ai`,
  `freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`
- **Schema:** `schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json`
- **Support export (canonical truth):**
  `artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/support_export.json`
- **Matrix CSV / Markdown report:** same directory (`matrix.csv`) and sibling
  `.md`
- **Narrowed fixtures:**
  `fixtures/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/`
- **Headless emitter:** `cargo run -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- <support-export|report|csv|fixture-*|validate>`

## Governed component families (8)

| Family | Owns (family-specific vocabulary) |
| --- | --- |
| `ai_action_state_banner` | action states, execution modes |
| `connector_detail_row` | connector capabilities, auth postures |
| `local_model_pack_card` | local-model pack states |
| `approval_sheet` | approval gates, friction reasons |
| `tool_call_timeline_row` | tool boundaries, side-effect classes |
| `run_history_row` | run outcomes |
| `replay_review` | replay-completeness states, rerun-review reasons |
| `agent_status` | agent lifecycle states, manual-takeover paths |

Every family also declares AI surface families, deployment lines, mandatory plus
truth labels, non-visual accessibility routes, consumer surfaces, and downgrade
triggers.

## Frozen controlled vocabularies

- **Action state:** `idle`, `composing`, `streaming`, `tool_running`,
  `awaiting_approval`, `paused`, `boundary_blocked`, `completed`, `failed`
- **Execution mode:** `foreground_assistant`, `guided_patch`,
  `background_branch_agent`, `review_first_placement`, `headless_automation`
- **Connector capability:** `read_only_query`, `file_mutation`, `network_egress`,
  `shell_execution`, `external_service_call`, `credential_scoped`
- **Auth posture:** `oauth_delegated`, `managed_credential`, `byok_scoped`,
  `service_account`, `token_scoped`, `unauthenticated`
- **Local-model pack state:** `installed`, `mirrored`, `offline_only`,
  `quarantined`, `hardware_unfit`, `update_available`, `provenance_unverified`
- **Approval gate:** `auto_approved`, `notify_only`, `one_click_confirm`,
  `high_friction_typed`, `two_person_review`, `policy_blocked`
- **Friction reason:** `irreversible_side_effect`, `external_network_egress`,
  `credential_access`, `cross_tenant_scope`, `destructive_file_change`,
  `policy_mandated_review`
- **Tool boundary:** `in_process`, `local_sandbox`, `local_shell`,
  `remote_connector`, `external_service`, `host_delegated`
- **Side-effect class:** `read_only`, `file_write`, `network_call`,
  `process_spawn`, `state_mutation`, `destructive`
- **Run outcome:** `succeeded`, `failed`, `cancelled`, `superseded`,
  `partially_applied`, `awaiting_review`
- **Replay completeness:** `fully_replayable`, `partially_replayable`,
  `incomplete_replay`, `non_deterministic`, `missing_inputs`, `provider_drifted`
- **Rerun-review reason:** `model_version_changed`, `tool_contract_changed`,
  `input_context_changed`, `route_or_provider_changed`, `policy_changed`,
  `no_re_review_required`
- **Agent lifecycle state:** `running`, `paused`, `blocked_on_approval`,
  `awaiting_takeover`, `handed_off`, `completed`, `abandoned`
- **Manual-takeover path:** `resume_in_place`, `take_over_locally`,
  `branch_review_handoff`, `abort_with_checkpoint`, `escalate_to_owner`,
  `no_takeover_possible`

Shared/topology vocabularies: AI surface family (8), deployment line (5), consumer
surface (10), accessibility route (6), required label (6, with mandatory `identity`
/ `state` / `keyboard_route` plus `execution_mode` / `route` / `approval_gate`),
qualification class (6), downgrade trigger (12).

## Hard component invariants

Every component row must keep all four `false`:

1. `masks_execution_mode_or_route` — never hide which execution mode or
   route/provider a component is running.
2. `overstates_replay_completeness` — never present a partial, incomplete, or
   non-deterministic replay as fully replayable.
3. `invents_private_ai_status_grammar` — never invent a second AI-status grammar
   outside this matrix.
4. `hides_approval_gate_or_takeover_path` — never hide the approval gate on an
   action or the manual-takeover path on an interrupted agent.

## Non-visual / CLI / export expectations

Every component declares a non-visual accessibility route set (keyboard focus,
screen-reader announcement, non-hover reachability, pointer-optional, high-contrast
safety, support-exportability). AI execution/replay primitives must never become
panel-only or chat-only affordances: the same mode/route/boundary/approval/replay/
takeover truth is reachable via keyboard, screen reader, CLI inspect, and the
support export.

## Auto-narrowing

Qualification narrows below Stable when a downgrade trigger fires (e.g. execution
mode unstated, route/provider masked, tool boundary unstated, auth posture masked,
approval gate hidden, replay completeness overstated, rerun-review reason unstated,
takeover path hidden, proof stale). The two checked-in narrowed fixtures
demonstrate the pattern while keeping every family visible: `replay_review` → Beta,
`agent_status` → Preview.

## Bound source contracts

`schemas/ai/tool_call_timeline_entry.schema.json`,
`schemas/ai/ai_run_history_entry.schema.json`,
`schemas/ai/evidence_replay_packet.schema.json`, and
`schemas/ai/branch_agent_session.schema.json` — this matrix hardens shared
components layered on top of those already-claimed systems; it does not
re-architect AI execution policy, evidence storage, or route selection.

## Consumer rule

Every claimed M5 AI/review/branch-agent/connector/model/run-history/replay consumer
points at this one canonical component contract instead of rewording route,
approval, or replay truth locally. Future AI implementation rows have an agreed
field/state baseline and no open ambiguity about route, drift, or replay
vocabulary.
