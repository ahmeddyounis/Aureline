# M5 AI-execution/replay component matrix (design QA)

Shared design / schema / QA / release matrix for the reusable M5 AI execution and
replay components (row **M05-876**, batch B103). Design, schema, QA, and release
owners consume this one matrix instead of rewording AI route/approval/replay truth
per surface.

**Canonical truth (do not re-key):**

- Contract doc:
  `docs/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix.md`
- Schema:
  `schemas/ai/freeze-the-m5-ai-action-state-banner-connector-detail-row-local-model-pack-card-approval-sheet-tool-call-timeline-row-run-history-row-replay-review-and-agent-status-component-matrix.schema.json`
- Support export + CSV + report:
  `artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/`
- Emitter: `cargo run -p aureline-ai --bin aureline_ai_execution_replay_component_matrix -- report`

## Component families and the truth each must always show

| Family | Must always show | Never allowed to |
| --- | --- | --- |
| `ai_action_state_banner` | live action state + execution mode | leave the mode/state implicit |
| `connector_detail_row` | connector capability class + auth posture | hide what a connector can do or how it authenticates |
| `local_model_pack_card` | pack state + provenance | present a quarantined/unverified pack as freely ready |
| `approval_sheet` | approval gate + friction reason | show a high-friction/blocked action as a quiet auto-approval |
| `tool_call_timeline_row` | tool boundary + side-effect class | show a destructive/network call as a benign in-process read |
| `run_history_row` | run outcome + route/mode | list a partial/superseded run as a clean success |
| `replay_review` | replay completeness + rerun-review reason | show an incomplete/drifted replay as a faithful re-run |
| `agent_status` | lifecycle state + manual-takeover path | leave an interrupted agent without a safe takeover path |

## Design acceptance gates

1. **One vocabulary.** Action state, execution mode, connector capability, auth
   posture, pack state, approval gate, friction reason, tool boundary, side-effect
   class, run outcome, replay completeness, rerun-review reason, agent lifecycle
   state, and takeover path use only the frozen tokens in the schema/contract. No
   surface invents parallel labels.
2. **Mandatory labels.** Every component exposes `identity`, `state`, and
   `keyboard_route`, plus the truth labels relevant to it (`execution_mode`,
   `route`, `approval_gate`).
3. **Non-visual parity.** Every component is keyboard-focusable, screen-reader
   announced, non-hover reachable, pointer-optional, high-contrast safe, and
   support-exportable. Nothing is panel-only or chat-only.
4. **Deployment parity.** The same truth survives local-OSS, self-hosted, managed,
   air-gapped, and mirror/offline lines.
5. **Auto-narrowing.** When a downgrade trigger fires the component drops below
   Stable while staying visible (fixtures: `replay_review` → Beta, `agent_status` →
   Preview).

See `matrix.csv` in the canonical artifact directory for the per-family
surface-family / deployment-line / required-label / consumer-surface /
downgrade-trigger grid.
