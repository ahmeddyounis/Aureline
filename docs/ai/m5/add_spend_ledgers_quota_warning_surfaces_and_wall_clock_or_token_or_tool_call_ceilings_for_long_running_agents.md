# Spend ledgers, quota-warning surfaces, and wall-clock, token, or tool-call ceilings for long-running agents

This contract materializes the long-running-agent budget surface into one
export-safe truth packet whose unit of truth is a budget row. Shell, docs,
support export, and release tooling consume the packet directly instead of
re-describing spend, quota, or ceiling state by hand.

- Packet type: `aureline_ai::add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents::AgentBudgetPacket`
- Schema: [`schemas/ai/add-spend-ledgers-quota-warning-surfaces-and-wall-clock-or-token-or-tool-call-ceilings-for-long-running-agents.schema.json`](../../../schemas/ai/add-spend-ledgers-quota-warning-surfaces-and-wall-clock-or-token-or-tool-call-ceilings-for-long-running-agents.schema.json)
- Support export: [`artifacts/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/support_export.json`](../../../artifacts/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/support_export.json)
- Fixtures: [`fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/`](../../../fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/)

## The budget row

Each `AgentBudgetRow` binds, for one long-running agent run:

| Field | Meaning |
| --- | --- |
| `agent_run_id`, `agent_label` | Identity token and label for the run. |
| `resolved_mode` | Local, BYOK, managed, or enterprise-gateway mode the run resolves to. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `run_state` | Running, completed, stopped-at-ceiling, paused, budget-exhausted, or cancelled. |
| `spend_ledger` | Running cost band, measurement, charge owner, exhaustion flag, and ordered ledger entries. |
| `quota_warning` | Quota family, state, scope, and whether the warning is surfaced (no raw account id). |
| `ceilings` | Wall-clock, token, and tool-call (plus optional cost or step) ceilings with consumption, enforcement, and stop outcome. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a budget-policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed run. |

## Invariants enforced by validation

- **Required ceilings, always bounded.** A claimed run must configure the
  wall-clock, token, and tool-call ceilings, and each must be enforced with a
  bounding stop (`hard_stop_on_reach` or `soft_prompt_on_reach`), so a claimed
  long-running agent can never run, spend, or call tools without limit.
- **A reached ceiling actually stops the run.** A bounding ceiling whose
  consumption is `ceiling_reached` must carry a `hard_stopped` or
  `paused_awaiting_user` outcome, and the run state must agree — a hard stop or an
  exhausted spend budget implies a terminal run state, and a pause implies a
  paused run state — so the disclosed stop is the stop that happened.
- **A ceiling warning is surfaced before the limit.** A ceiling at or past its
  warning threshold must carry a warning label.
- **The ledger only accumulates.** Ledger entries are strictly ordered from zero,
  the running cost band never decreases across entries, and the ledger total
  equals its last entry's running band.
- **Charged ledgers disclose who pays.** A metered or subscription running band
  must carry a disclosed charge owner; the charge is never left
  `charge_unknown_unverified`.
- **Estimates do not back Stable.** A run whose ledger is
  `estimated_unverified_band` or whose measurement is `estimate_band` may not
  claim Stable.
- **Quota warnings are surfaced; exhaustion narrows.** A quota in its `warning`
  state must be surfaced, and a provider quota that is `exhausted` or
  `paused_by_policy` may not keep claiming Stable.
- **Claimed runs carry evidence and a verified reversal.** Stable, Beta, and
  Preview runs must list at least one evidence packet ref, and any reversing
  rollback posture must be `verified`.
- **Narrow, never hide.** Every run carries the `proof_stale` and
  `provider_unavailable` downgrade triggers, and every rule must narrow strictly
  below the claimed qualification.

## Provenance and freshness

`source_contract_refs` must include this schema, this doc, the provider/model
registry schema, the frozen M5 AI workflow matrix schema, and the routing-policy
schema whose mode, quota, and cost-band vocabularies the packet reuses. The
`proof_freshness` block records the freshness SLO and asserts that stale proof
automatically narrows claimed runs. Reading the checked export through
`current_agent_budget_export` re-validates every invariant, so a stale or
malformed artifact fails the consuming surface rather than shipping an optimistic
claim.

## Boundary

The packet carries no provider endpoints, credential bodies, raw provider
payloads, exact token counts, or exact spend amounts. Cost is disclosed as
coarse, review-safe bands and charge owners, and ceiling consumption as coarse
classes, only. Validation rejects obvious credential material and raw URLs in the
serialized export.
