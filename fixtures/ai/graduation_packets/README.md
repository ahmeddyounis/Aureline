# AI graduation packet, rollout-state, budget-routing, and route-selection worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/model_graduation_and_budget_contract.md`](../../../docs/ai/model_graduation_and_budget_contract.md)
and the schema at
[`/schemas/ai/graduation_packet.schema.json`](../../../schemas/ai/graduation_packet.schema.json),
with the rollout-state vocabulary registered at
[`/artifacts/ai/model_rollout_states.yaml`](../../../artifacts/ai/model_rollout_states.yaml).

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the contract
sections it exercises, and the record kinds it produces. The
remaining documents are individual `graduation_packet_record`,
`model_rollout_state_record`, `budget_routing_policy_record`,
`route_selection_disclosure_record`, and
`graduation_audit_event_record` instances that conform to the
schema.

No fixture embeds raw URLs, raw endpoint hostnames, raw evaluation
prompts, raw evaluation response bodies, raw red-team prompts, raw
red-team transcripts, raw latency or cost measurement series, raw
cost amounts in any specific currency, raw user identifiers, raw
billing-account ids, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, raw pack bytes, or raw provider
payloads. Every such field is an opaque ref or structured readout.

## Cases

| Scenario file                                            | Axis exercised                                                                 | Covered route / state                                                                                          |
|----------------------------------------------------------|--------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `stable_inline_completion_packet.yaml`                   | `promote_to_stable` packet + rollout-state pin + audit event                   | Workflow `ai.inline_completion`; local-in-process route; four-row eval evidence; offline-cached fallback        |
| `preview_patch_flow_packet.yaml`                         | `promote_to_preview` packet + rollout-state pin                                 | Workflow `ai.patch_flow`; BYOK vendor route; two-row eval evidence; degrade-to-local fallback                  |
| `shadow_review_flow_packet.yaml`                         | `promote_to_shadow` packet + rollout-state pin (no admitting approval ticket)   | Workflow `ai.review_flow`; vendor-hosted shadow paired with the live BYOK route; disable-with-typed-denial      |
| `deprecated_explain_flow_packet.yaml`                    | `demote_to_deprecated` packet + rollout-state pin                               | Workflow `ai.explain_flow`; BYOK vendor route on older variant; rollback plan to prior model entry              |
| `withdraw_to_disabled_packet.yaml`                       | `withdraw_to_disabled` packet + rollout-state pin + withdraw audit event        | Workflow `ai.search_flow`; withdrawn under blocking red-team finding; no fallback route                         |
| `budget_routing_policy_inline_completion.yaml`           | `budget_routing_policy_record` covering six budget scopes                       | Workflow `ai.inline_completion`; cheapest-qualifying-route default with user override; bounded agent ceiling    |
| `cheapest_route_override_disclosure.yaml`                | `route_selection_disclosure_record` for capability-required override + audit    | Cheapest local route fails capability check; BYOK vendor route admitted with typed override reason              |
| `cheapest_route_budget_exhausted_fallback.yaml`          | `route_selection_disclosure_record` for budget-exhausted fallback + audit       | BYOK route's per-session budget exhausted; degrade-to-local-route fallback taken; typed exhaustion state cited  |

Every fixture declares its canonical values via the
`exercised_classes` block so later coverage audits can confirm
each vocabulary member is hit at least once.

## Acceptance-criteria coverage

The seeded cases cover every acceptance criterion named in the
task:

- **No AI path can claim stable status without a current graduation
  packet.** `stable_inline_completion_packet.yaml` carries the four-
  row eval evidence (protected_eval_corpus_passed,
  red_team_corpus_passed, latency_envelope_measured,
  cost_envelope_measured), one admitting approval-ticket ref, and
  one linked assurance-claim ref; the paired
  `model_rollout_state_record` cites the packet ref.
- **Routing controls can express why a more expensive model was
  chosen.** `cheapest_route_override_disclosure.yaml` mints a
  `route_selection_disclosure_record` with
  `route_selection_reason_class =
  override_more_expensive_route_admitted` and
  `route_selection_override_reason_class =
  cheapest_route_failed_capability_check`, naming the cheapest
  candidate verbatim.
- **Exhausted or blocked routes degrade explicitly to local, BYOK,
  manual, or disabled states rather than failing opaquely.**
  `cheapest_route_budget_exhausted_fallback.yaml` shows the budget-
  exhausted path with `fallback_after_cheapest_exhausted` /
  `cheapest_route_budget_exhausted` /
  `per_session_budget_exhausted` and a paired `route_fallback_taken`
  audit event. `withdraw_to_disabled_packet.yaml` shows the
  no-fallback path with `no_fallback_request_fails_with_typed_denial`.
- **Per-request, per-session, per-agent-invocation, per-workflow,
  per-user, and per-organisation budgets are expressible.**
  `budget_routing_policy_inline_completion.yaml` declares six budget
  caps covering all six scopes plus the seven minimum spend-
  attribution dimensions, the bounded-recursion-and-count agent
  ceiling, and the degrade-to-local fallback posture.
- **Graduation-packet vocabulary covers all six rollout states.**
  Five worked packets cover `experimental` (implicitly via the prior
  state on `shadow_review_flow_packet.yaml`), `shadow`, `preview`,
  `stable`, `deprecated`, and `disabled`. The artifact register at
  `/artifacts/ai/model_rollout_states.yaml` declares the full
  vocabulary and the legal promotion transitions.
