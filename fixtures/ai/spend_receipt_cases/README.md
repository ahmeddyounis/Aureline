# AI spend / route / model-path receipt worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/spend_and_route_receipt_contract.md`](../../../docs/ai/spend_and_route_receipt_contract.md)
and the schemas at
[`/schemas/ai/provider_route_receipt.schema.json`](../../../schemas/ai/provider_route_receipt.schema.json)
and
[`/schemas/ai/spend_receipt.schema.json`](../../../schemas/ai/spend_receipt.schema.json).

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the contract
sections it exercises, and the record kinds it produces. The
remaining documents are individual `provider_route_receipt_record`,
`model_path_comparison_row_record`,
`branch_agent_route_rollup_record`,
`provider_route_receipt_audit_event_record`, `spend_receipt_record`,
`branch_agent_spend_rollup_record`, and
`spend_receipt_audit_event_record` instances that conform to the
two schemas.

No fixture embeds raw URLs, raw endpoint hostnames, raw API keys,
raw OAuth tokens, raw mTLS material, raw provider payloads, raw
cost amounts in any specific currency, raw token counts, raw
provider unit prices, raw user identifiers, raw billing-account
ids, or raw evaluation prompts. Every such field is an opaque ref,
a structured class label, or a coarse bucket.

## Cases

| Scenario file                                            | Axis exercised                                                                | Covered route / state                                                                                              |
|----------------------------------------------------------|-------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------|
| `local_only_route_inline_completion.yaml`                | Local-only route + cheapest qualifying admitted + bundled-no-incremental-cost | Workflow `ai.inline_completion`; local-in-process route; not_charged_local; no override or comparison rows         |
| `policy_forced_enterprise_route_review_flow.yaml`        | Override-more-expensive-route admitted + comparison rows + admitting ticket    | Workflow `ai.review_flow`; enterprise-gateway route policy-pinned over cheaper BYOK candidate; charged_organisation_pooled |
| `budget_capped_refusal_patch_flow.yaml`                  | Budget-blocked refusal + typed exhaustion-state + receipt survives history     | Workflow `ai.patch_flow`; per-organisation budget exhausted; outcome_blocked_by_budget; not_charged_run_blocked    |
| `fallback_to_cheaper_model_explain_flow.yaml`            | Fallback-after-cheapest-blocked + circuit-open + cheaper / slower comparison   | Workflow `ai.explain_flow`; primary BYOK large-model route circuit-open; falls back to BYOK small-model route       |
| `branch_agent_run_cumulative_rollup.yaml`                | Branch-agent route + spend rollup with mixed per-hop outcomes                  | Workflow `ai.branch_agent`; three local-in-process hops; hop 2 fails; aggregate post_run_failed; aggregate not_charged_local |

Every fixture declares its canonical values via the
`exercised_classes` block so later coverage audits can confirm each
vocabulary member is hit at least once.

## Acceptance-criteria coverage

The seeded cases cover every acceptance criterion named in the
task:

- **A user or support packet can explain why a route was chosen
  and what budget class applied without reading implementation
  code or provider logs.** Every fixture renders the seven minimum
  spend-attribution dimensions, the route-origin class, the cost-
  envelope band, the outcome class, and (when an override happened)
  the model-path comparison rows. `policy_forced_enterprise_route_review_flow.yaml`
  and `fallback_to_cheaper_model_explain_flow.yaml` show how the
  comparison rows explain why a cheaper route was passed over.
- **Cancelled, blocked, and fallback runs still emit truthful
  receipts instead of disappearing from audit or UI history.**
  `budget_capped_refusal_patch_flow.yaml` shows the budget-blocked
  refusal record (run_state_class = budget_blocked_refusal,
  outcome_class = outcome_blocked_by_budget, charge locus =
  not_charged_run_blocked) and a paired audit event on the
  `ai_route_receipt` and `ai_spend_receipt` streams.
  `fallback_to_cheaper_model_explain_flow.yaml` shows the typed
  fallback-after-cheapest-blocked path.
- **The contract leaves no path where provider/model identity or
  spend class can be hidden behind generic 'AI used' wording.**
  Every receipt carries a non-empty `provider_entry_ref`,
  `model_entry_ref`, `route_origin_class`, `cost_envelope_class`,
  and `outcome_class`. The schema's `originless_ai_chip_forbidden_resolve_through_receipt`
  denial reason gates surfaces that try to render a generic chip.
- **Worked examples exist for local-only route, policy-forced
  enterprise route, budget-capped refusal, fallback-to-cheaper-
  model, and branch-agent run with cumulative receipt rollup.**
  The five cases above cover each axis explicitly.
