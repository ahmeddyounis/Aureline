# M5 AI Workflow Matrix — Inline Assist, Patch Review, and Branch or Worktree Agents

This document freezes the canonical M5 depth qualification for three AI workflow lanes:

1. **Inline Assist** — Composer inline quick-edit and scoped-apply workflows.
2. **Patch Review** — AI-assisted review findings, publish-to-review sheets, and resolution memory.
3. **Branch or Worktree Agents** — Background branch-agent or worktree-isolated long-running tasks.

## Packet

The machine-readable packet is owned by `crates/aureline-ai/src/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents/`.

- Record kind: `freeze_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`
- Schema version: `1`
- Schema: `schemas/ai/freeze-the-m5-ai-workflow-matrix-for-inline-assist-patch-review-and-branch-or-worktree-agents.schema.json`
- Checked-in export: `artifacts/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents/support_export.json`

## Lane Rows

Each lane row binds:

- **Qualification class** — `stable`, `beta`, `preview`, `experimental`, `unavailable`, or `held`.
- **Scope summary** — A review-safe description of what the lane may touch and how it is bounded.
- **Evidence requirement** — `required`, `recommended`, `optional`, or `not_applicable`.
- **Required evidence packet refs** — Opaque refs to upstream qualification packets that must be current.
- **Downgrade triggers** — Closed set of conditions that automatically narrow the lane.
- **Rollback posture** — `fully_reversible`, `checkpoint_reversible`, `evidence_preserved_no_revert`, or `not_applicable`.
- **Source contract refs** — The frozen contracts this lane projects against.
- **Consumer surfaces** — The surfaces that must show qualification truth for this lane.

## Security Review Invariants

The packet enforces six security invariants:

1. `no_self_approved_mutating_tools` — Agents cannot approve their own mutating tool calls.
2. `no_worktree_isolation_bypass` — Worktree isolation cannot be bypassed.
3. `preview_approval_required_before_apply` — Preview and approval are required before any apply path.
4. `evidence_packets_cite_source_contracts` — Evidence packets cite their governing contracts.
5. `downgrade_narrows_instead_of_hides` — Downgrade narrows the public claim rather than hiding the lane.
6. `stale_proof_blocks_promotion` — Stale or missing proof automatically blocks promotion.

## Consumer Projection

All seven consumer surfaces must show qualification truth:

- Desktop composer
- Desktop review workspace
- CLI / headless
- Browser / companion
- Support export
- Diagnostics

Unqualified lanes must carry a visible Preview or Labs label.

## Proof Freshness

- Proof-freshness SLO: 168 hours.
- Last refresh tracked per packet.
- Auto-narrow on stale proof is enabled.

## Downgrade Behavior

When any downgrade trigger fires, the lane narrows to the next lower qualification class rather than disappearing or silently continuing at the claimed level. The narrowed state is visible on all consumer surfaces.

## Rollback Posture

- Inline assist: fully reversible via revert handles.
- Patch review: evidence preserved; revert is not applicable because the lane is advisory.
- Branch or worktree agents: checkpoint reversible via captured checkpoints and operator takeover.

## Source Contracts

The matrix consumes the following frozen contracts:

- `docs/ai/prompt_composer_contract.md` — Inline assist base contract.
- `docs/ai/review_assist_publish_contract.md` — Patch review base contract.
- `artifacts/ai/multifile_patch_review_sequence.md` — Multi-file patch review sequence.
- `docs/ai/background_branch_agent_lifecycle.md` — Branch-agent lifecycle base contract.

## Validation

The Rust module validates:

- All three lanes are present.
- Record kind and schema version match the constants.
- Identity fields are non-empty.
- All required source contracts are cited.
- Stable lanes carry at least one required evidence packet ref.
- Every lane has at least one downgrade trigger.
- Every lane has at least one consumer surface.
- All security-review booleans are true.
- All consumer-projection booleans are true.
- Proof-freshness SLO is non-zero and last refresh is non-empty.
- Export JSON contains no raw boundary material (credentials, secrets, tokens, API keys).
