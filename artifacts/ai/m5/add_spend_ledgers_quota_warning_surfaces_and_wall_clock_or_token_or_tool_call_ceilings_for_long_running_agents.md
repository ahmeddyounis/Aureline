# Spend Ledgers, Quota-Warning Surfaces, And Ceilings For Long-Running Agents

- Packet: `agent-budget:stable:0001`
- Schema: `schemas/ai/add-spend-ledgers-quota-warning-surfaces-and-wall-clock-or-token-or-tool-call-ceilings-for-long-running-agents.schema.json`
- Support export: `artifacts/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/support_export.json`
- Fixture: `fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/`

## Coverage

The packet materializes the long-running-agent budget surface into one row per
run. Every run carries the running spend ledger that accrues its cost, the
quota-warning surface that flags an approaching provider limit, and the
wall-clock, token, and tool-call ceilings that keep the run bounded.

- The composer refactor agent resolves to the managed mode at Stable: a metered
  medium running ledger charged to the subscriber that accumulates from low to
  medium across its plan and edit phases, a managed entitlement quota within
  limit, all four ceilings (wall-clock, token, tool-call, and cost) hard-stop
  bounded, and a fully-reversible verified rollback. It completed within budget.
- The review BYOK agent resolves to BYOK at Beta: a metered medium ledger charged
  to the BYOK owner, a per-session vendor quota in `warning` with the warning
  surfaced, and a tool-call ceiling that reached its limit under a soft prompt and
  paused the run awaiting a user decision — the run state agrees that it is
  `paused_awaiting_user`.
- The explain local agent resolves to local at Preview: an unmetered local quota,
  a bundled no-charge ledger, and the three required ceilings hard-stop bounded
  even though the run cannot spend, so it can never run away on-device.
- The background agent had its per-run cost budget and token budget reach their
  ceilings: both were hard-stopped, the run state is `budget_exhausted_stopped`,
  its per-session vendor quota is exhausted, it dropped out of every claimed lane
  to `held`, carries no evidence refs, and narrows to `unavailable` on stale proof
  or provider unavailability.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The packet refuses to present a long-running agent greener than its spend, quota,
and ceiling posture can back. Every claimed run configures and bounds the
wall-clock, token, and tool-call ceilings so it can never run unbounded; a reached
bounding ceiling must have actually stopped or paused the run, and the run state
must agree, so the disclosed stop is the stop that happened; a spend ledger only
accumulates (strictly-ordered entries, a non-decreasing running band, and a total
that matches its last entry) and a metered or subscription band must disclose who
is charged; an estimate-only ledger may not back a Stable claim; a quota in its
warning state must be surfaced; and an exhausted or paused provider quota narrows
the claim instead of keeping an optimistic posture. Every run narrows rather than
hides through the `proof_stale` and `provider_unavailable` triggers, reusing the
frozen M5 AI workflow matrix qualification, downgrade, and rollback-posture
vocabularies and the routing-policy mode, quota, and cost-band vocabularies so no
budget row may stay greener than its evidence. Raw provider endpoints, credential
bodies, raw provider payloads, exact token counts, and exact spend amounts never
cross the support boundary.
