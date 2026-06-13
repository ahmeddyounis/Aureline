# Spend And Route Receipts, Budget-Band Preflights, Cumulative Branch-Agent Ceilings, And Checkpoint-Aware Cancellation

- Packet: `ai-run-receipt:stable:0001`
- Schema: `schemas/ai/implement-spend-and-route-receipts-budget-band-preflights-cumulative-branch-agent-ceilings-and-checkpoint-aware-cancella.schema.json`
- Support export: `artifacts/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/support_export.json`
- Fixture: `fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/`

## Coverage

The packet deepens the long-running-agent budget surface into one receipt row
per AI run, covering the inline assist, patch review, and branch/worktree agent
lanes. Every run carries the budget-band preflight that prices it before
dispatch, the route receipt that records which provider/model/mode served it,
and the post-run spend receipt that reconciles the measured cost against the
preflight; branch agents additionally carry a cumulative ceiling, ordered
checkpoints, and a checkpoint-aware cancellation export.

- The inline assist edit resolves to the managed mode at Stable: a low metered
  band priced and auto-approved before dispatch, a primary managed route with no
  change, and a measured low band that reconciles within projection. It completed
  within budget.
- The patch review BYOK pass resolves to BYOK at Beta: a medium metered band
  acknowledged before dispatch, a route receipt that records a `fallback_downgrade`
  with the precise `quota_exhausted` reason rather than a generic provider error,
  and a measured medium band that reconciles within projection.
- The side-branch refactor agent resolves to managed at Stable: a medium metered
  band auto-approved with a cumulative cost budget, a cumulative ceiling that is
  hard-stop bounded, and two strictly-ordered checkpoints whose cumulative cost
  band accumulates from low to medium. It completed within budget.
- The cancelled side-branch agent dropped to `held`: the user cancelled it at the
  implement checkpoint, its cancellation export names the checkpoint it stopped
  at, carries an export-safe receipt readable on the desktop, CLI, and
  support-export surfaces, and gives the precise `user_requested` reason rather
  than an opaque kill. Its measured cost reconciles under projection.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The packet refuses to present an AI run greener than its receipt posture can
back. Every claimed run carries an acknowledged budget-band preflight that
projects the wall-clock, token, and tool-call ceilings, a route receipt whose
mode agrees with the run, and a post-run spend receipt that reconciles against
the preflight — disclosing overruns and naming who is charged, and never backing
a Stable claim with an estimate-only receipt. A route downgrade or fallback
carries a precise reason rather than collapsing into a generic provider error
when a more precise reason exists. A branch/worktree agent bounds its whole run
with a cumulative ceiling and strictly-ordered, accumulating checkpoints, and its
cancellation names the checkpoint it stopped at, carries an export-safe receipt
with support-export parity, and gives a precise reason. Every run narrows rather
than hides through the `proof_stale` and `provider_unavailable` triggers, reusing
the frozen M5 AI workflow matrix lane, qualification, downgrade, and
rollback-posture vocabularies, the routing-policy mode, cost-band, and
fallback-hop vocabularies, and the spend-ledger ceiling vocabularies so no
receipt row may stay greener than its evidence. Raw provider endpoints,
credential bodies, raw provider payloads, exact token counts, and exact spend
amounts never cross the support boundary.
