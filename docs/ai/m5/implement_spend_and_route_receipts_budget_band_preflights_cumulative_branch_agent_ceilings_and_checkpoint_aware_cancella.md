# Spend and route receipts, budget-band preflights, cumulative branch-agent ceilings, and checkpoint-aware cancellation-export parity across inline assist, patch review, and side-branch agents

This contract deepens the long-running-agent budget surface into one export-safe
truth packet whose unit of truth is a receipt row. Shell, docs, support export,
and release tooling consume the packet directly instead of re-describing spend,
route, ceiling, or cancellation state by hand.

- Packet type: `aureline_ai::implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella::AiRunReceiptPacket`
- Schema: [`schemas/ai/implement-spend-and-route-receipts-budget-band-preflights-cumulative-branch-agent-ceilings-and-checkpoint-aware-cancella.schema.json`](../../../schemas/ai/implement-spend-and-route-receipts-budget-band-preflights-cumulative-branch-agent-ceilings-and-checkpoint-aware-cancella.schema.json)
- Support export: [`artifacts/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/support_export.json`](../../../artifacts/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/support_export.json)
- Fixtures: [`fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/`](../../../fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/)

## The receipt row

Each `AiRunReceiptRow` binds, for one AI run:

| Field | Meaning |
| --- | --- |
| `agent_run_id`, `run_label` | Identity token and label for the run. |
| `lane` | Inline assist, patch review, or branch/worktree agent. |
| `resolved_mode` | Local, BYOK, managed, or enterprise-gateway mode the run resolves to. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `run_state` | Running, completed, stopped-at-ceiling, paused, budget-exhausted, or cancelled. |
| `preflight` | Budget-band preflight: projected cost band, charge owner, acknowledgement, and the ceilings the run promises to bound. |
| `route_receipt` | Provider/model/mode that served the run, the selected fallback hop, and a precise downgrade reason. |
| `spend_receipt` | Measured final cost band, measurement, charge owner, exhaustion flag, and reconciliation against the preflight. |
| `cumulative_ceiling` | Cumulative bound across a side-branch run (required for branch agents). |
| `checkpoints` | Strictly-ordered checkpoints whose cumulative cost band accumulates (required for branch agents). |
| `cancellation` | Checkpoint-aware, export-safe cancellation receipt with a precise reason and surface parity. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a budget-policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed run. |

## Invariants enforced by validation

- **Preflight, route, and spend receipts across every lane.** The packet must
  cover the inline assist, patch review, and branch/worktree agent lanes; a
  claimed charged run must acknowledge its budget-band preflight before dispatch,
  and a claimed run must project the wall-clock, token, and tool-call ceilings so
  the preflight promises a bounded run.
- **The route receipt agrees with the run, and a downgrade is precise.** The
  route receipt's resolved mode must equal the run's mode, and a route that
  changed off the primary must carry a precise reason rather than collapsing into
  a generic provider error when a more precise reason exists.
- **The spend receipt reconciles against the preflight.** A run that reached a
  terminal state must reconcile its measured cost; a measured band above the
  projected band must be disclosed as an overrun; a charged band must disclose who
  is charged; and an estimate-only receipt may not back a Stable claim.
- **Side-branch agents are bounded and cancel cleanly.** A branch/worktree agent
  must carry a cumulative ceiling — bounding for a claimed run — and
  strictly-ordered checkpoints whose cumulative cost band only accumulates. A
  cancellation must agree with the run state, name the checkpoint it stopped at,
  carry an export-safe receipt readable on the support-export surface, and give a
  precise reason rather than an opaque kill.
- **Narrow rather than hide.** Every run carries the `proof_stale` and
  `provider_unavailable` downgrade triggers, each narrowing to a strictly lower
  qualification, and a claimed run carries evidence refs and a verified rollback
  path where one exists.
- **Export-safe only.** Raw provider endpoints, credential bodies, raw provider
  payloads, exact token counts, and exact spend amounts never cross the boundary;
  the packet carries modes, lanes, coarse bands, ceiling consumption classes,
  reconciliation classes, and review-safe labels only.

## Reused vocabularies

The packet grows additively on the frozen M5 lanes rather than forking truth. It
reuses the workflow lane, qualification, downgrade-trigger, rollback-posture, and
consumer-surface vocabularies from the M5 AI workflow matrix; the execution mode,
cost-band, cost-measurement, charge-disclosure, and fallback-hop vocabularies
from the routing-policy lane; and the ceiling-kind, consumption, enforcement, and
run-state vocabularies from the spend-ledger lane.
