# AI spend receipt, provider-route receipt, and model-path comparison contract

This document is the **product-wide contract** for how every AI
invocation emits a typed, originless-free receipt of provider /
model identity, spend class, and route decision so users, support,
and policy surfaces can explain *what* provider and model were used,
*why* that path was chosen over alternatives, and at *what* cost
band the run landed — without reading implementation code or
provider logs.

It freezes one **provider-route-receipt** shape covering preview,
post-run, cancelled, and budget-blocked states; one **spend-receipt**
shape carrying the seven minimum spend-attribution dimensions and a
typed cost-band outcome; one **model-path comparison row**
vocabulary explaining why a cheaper, slower, safer, or more local
alternative was or was not selected; one **branch-agent cumulative
rollup** shape so a chained agent run reports identity and spend per
hop and in aggregate; and one set of const audit-event ids every AI-
adjacent surface reads.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / metering
/ support / replay surface's mint of its own copy, this document
wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ai/provider_route_receipt.schema.json`](../../schemas/ai/provider_route_receipt.schema.json)
  — boundary schema for the `provider_route_receipt_record`,
  `model_path_comparison_row_record`,
  `branch_agent_route_rollup_record`, and
  `provider_route_receipt_audit_event_record` shapes.
- [`/schemas/ai/spend_receipt.schema.json`](../../schemas/ai/spend_receipt.schema.json)
  — boundary schema for the `spend_receipt_record`,
  `branch_agent_spend_rollup_record`, and
  `spend_receipt_audit_event_record` shapes.
- [`/fixtures/ai/spend_receipt_cases/`](../../fixtures/ai/spend_receipt_cases/)
  — worked-example corpus covering local-only route, policy-forced
  enterprise route, budget-capped refusal, fallback-to-cheaper-model,
  and branch-agent run with cumulative rollup.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, local-pack, and external-tool
  identity, execution locus, transport, auth mode, retention,
  region, quota family, cost visibility, and tainted-return posture.
  A receipt binds to one `provider_entry_id`, one `model_entry_id`,
  and re-exports the locus / region / retention / quota / cost-
  visibility classes verbatim.
- [`/docs/ai/model_graduation_and_budget_contract.md`](./model_graduation_and_budget_contract.md) —
  rollout-state pin, graduation packet, budget-routing policy, route-
  selection disclosure, budget-scope, agent-ceiling, spend-
  attribution-dimension, fallback-posture, exhaustion-state, and
  cheapest-qualifying-route-rule vocabulary. A receipt re-exports
  these classes without re-minting them; the route-selection-
  disclosure ref is the cross-tool readout that pre-dispatch
  inspectability already records, and the receipt cites it by id.
- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  data-class allowlist, route-path-class, cost-visibility-class,
  redaction-class, freshness-class, client-scope, and the
  placeholder `ai_route_plan_record` / `ai_spend_plan_record` /
  `ai_route_receipt_record` / `ai_spend_receipt_record` rows. The
  rich receipts in this contract are the cross-tool readouts those
  placeholders bind to; an assembly's `route_receipt_ref` and
  `spend_receipt_ref` resolve through this contract.
- [`/docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md) —
  evidence packets quote the route-receipt id and spend-receipt id
  on every AI turn so a replay packet can be reconstructed.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, `deployment_profile_class`, policy epoch.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  the broker-owned redaction pass; receipts never quote raw
  credentials, raw tokens, or raw hostnames.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which provider-class / locus / retention /
  region / cost-band a deployment profile admits in a receipt;
  policy MAY NOT silently widen any axis.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary. A receipt cites the admitting ticket
  by ref when route selection or budget overrides required one.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `redaction_class` and `freshness_class` are re-exported without
  modification.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship a live metering / billing pipeline, a
live provider-arbitration runtime, a live quota enforcement backend,
or a live provider-pricing integration. It freezes the contract
those implementations will read and write. The eventual provider-
arbitration / metering crate's Rust types are the schema of record;
the JSON Schema exports are the cross-tool boundary every non-owning
surface reads.

## Why freeze this now

Without one frozen receipt contract the product is free to invent a
per-workflow notion of "AI used", a per-surface notion of "the run
finished", and a per-lane notion of "the budget ran out". Each
divergence widens a different axis silently:

1. *A turn finishes successfully but the support bundle says only
   "AI completion".* Reviewers cannot tell which provider, which
   model, which locus, which region, or which cost band was hit.
2. *A turn is cancelled half-way and disappears from history.* Audit
   loses the fact that the user spent attention on a model invocation
   that never produced output; spend reviewers cannot tell whether
   the cancellation costs anything; replay cannot reconstruct the
   intent.
3. *A budget-blocked refusal returns "AI not available right now".*
   Support cannot tell whether the workflow was over budget, the
   user was over quota, the policy revoked eligibility, or the
   provider was disabled.
4. *A branch-agent run chains five AI invocations and one fails.* If
   each hop emits a separate, uncorrelated receipt, neither the
   cumulative cost band nor the failing hop's identity can be
   surfaced on a single review row.
5. *The product picks a more expensive route than the cheapest
   qualifying route, with no record of why.* Spend reviewers cannot
   tell whether capability, region, retention, taint posture,
   quota, circuit-open, user choice, or policy pinning forced the
   override.

This contract closes that gap with **one route-receipt shape, one
spend-receipt shape, one model-path-comparison-row shape, one
branch-agent rollup shape, and one set of const audit-event ids**
every AI-adjacent surface reads.

## Who reads this document

- **AI / prompt-composer / route-planner / metering authors** who
  mint a `provider_route_receipt_record` before dispatch (preview),
  update it after dispatch (post-run), update it on cancel
  (cancelled), or replace it with a budget-blocked record (refusal).
- **Branch-agent / orchestration authors** who chain AI invocations
  and roll up per-hop receipts into one
  `branch_agent_route_rollup_record` /
  `branch_agent_spend_rollup_record` so review surfaces see the
  agent's total identity / cost band on one row.
- **Review / support / parity-audit / export authors** who quote a
  receipt by ref to explain why a route was chosen, what budget
  band applied, and what the alternatives were — without reading
  implementation code or provider logs.
- **Admin / policy / settings surface authors** narrowing which
  provider-classes, loci, retention stances, cost-bands, run-states,
  and outcome-classes are admitted on a receipt per deployment
  profile.

## 1. The provider-route receipt

### 1.1 Minimum payload

A `provider_route_receipt_record` MUST carry at minimum:

| Field                                | Purpose                                                                  |
|--------------------------------------|--------------------------------------------------------------------------|
| `route_receipt_id`                   | Stable opaque id; the cross-tool readout every other surface cites.       |
| `workflow_or_surface_id`             | Stable opaque id of the workflow/surface (e.g. inline-completion).        |
| `run_state_class`                    | One of `preview_pre_dispatch`, `post_run_completed`, `post_run_failed`, `cancelled_by_user`, `cancelled_by_policy`, `budget_blocked_refusal`, `route_blocked_refusal`. |
| `provider_entry_ref` / `model_entry_ref` | Opaque refs into the provider / model registry (already governed).    |
| `execution_locus_class`              | Re-exported from the provider registry (`local_in_process`, `byok_remote_vendor_direct`, `enterprise_gateway_brokered`, etc.). |
| `region_posture_class`               | Re-exported from the provider registry.                                   |
| `retention_stance_class`             | Re-exported from the provider registry.                                   |
| `quota_family_class`                 | Re-exported from the provider registry.                                   |
| `cost_visibility_class`              | Re-exported from the provider registry.                                   |
| `cost_envelope_class`                | Coarse cost-band bucket re-exported from the budget contract.             |
| `route_origin_class`                 | One of `stayed_local`, `byok_user_credential`, `enterprise_gateway`, `vendor_hosted_managed`, `extension_provided`, `mocked_test`, `disabled_no_route`. |
| `route_selection_reason_class`       | Re-exported from the budget contract.                                     |
| `route_selection_override_reason_class` | Re-exported from the budget contract.                                  |
| `route_selection_disclosure_ref`     | Required (non-empty) when the run was not the cheapest qualifying route; empty otherwise. |
| `budget_routing_policy_ref`          | Opaque ref to the policy the run resolved against.                       |
| `graduation_packet_ref`              | Opaque ref to the packet under which the route was admitted; empty for shadow / experimental. |
| `rollout_state_class`                | Re-exported from the budget contract.                                     |
| `ceiling_summary`                    | Coarse `token_ceiling_class`, `tool_call_ceiling_class`, `wall_time_ceiling_class` buckets the run was admitted under. |
| `outcome_class`                      | One of `outcome_success`, `outcome_partial_with_disclosure`, `outcome_cancelled`, `outcome_provider_error`, `outcome_blocked_by_budget`, `outcome_blocked_by_route`, `outcome_blocked_by_policy`, `outcome_blocked_by_taint_posture`, `outcome_unknown_pre_dispatch`. |
| `model_path_comparison_rows`         | Optional array of `model_path_comparison_row_record` instances naming the alternatives considered. |
| `originating_approval_ticket_ref`    | Required (non-empty) when override or fallback required one; empty otherwise. |
| `assembly_id_ref`                    | Opaque ref to the `ai_context_assembly_record` the run resolved against. |
| `policy_context`                     | Policy epoch, trust state, deployment profile, execution context.        |
| `redaction_class`                    | ADR-0011 redaction posture.                                              |
| `minted_at` / `last_updated_at`      | Monotonic timestamps.                                                    |

### 1.2 Run-state vocabulary

The `run_state_class` enum names where in the lifecycle the receipt
sits:

| Run state                       | Meaning                                                                  |
|---------------------------------|--------------------------------------------------------------------------|
| `preview_pre_dispatch`          | Minted before the request leaves the device; carries planned identity / band. |
| `post_run_completed`            | The run finished and bytes were returned (full or partial).              |
| `post_run_failed`               | The run dispatched but failed without producing usable bytes.            |
| `cancelled_by_user`             | The user cancelled before completion.                                    |
| `cancelled_by_policy`           | A policy / safety check cancelled before or during the run.              |
| `budget_blocked_refusal`        | A budget cap was hit; no dispatch happened.                              |
| `route_blocked_refusal`         | The route was blocked (no admitted route, taint posture, eligibility).   |

A receipt MUST advance through monotonic transitions only:
`preview_pre_dispatch` → one of `post_run_completed`,
`post_run_failed`, `cancelled_by_user`, `cancelled_by_policy`,
`budget_blocked_refusal`, `route_blocked_refusal`. Re-opening a
terminal receipt is forbidden; the next run mints a new receipt.

### 1.3 Route-origin vocabulary

The `route_origin_class` enum is the single answer to "did the run
stay local, BYOK, enterprise, or hosted":

| Route origin                       | Meaning                                                                |
|------------------------------------|------------------------------------------------------------------------|
| `stayed_local`                     | Inference ran on-device (any of the three local loci).                  |
| `byok_user_credential`             | The user's BYOK credential to a vendor or self-hosted endpoint.         |
| `enterprise_gateway`               | A customer-operated / customer-contracted enterprise gateway.          |
| `vendor_hosted_managed`            | First-party-managed relationship with a vendor; user sees no creds.    |
| `extension_provided`               | An extension provider's locus / transport.                             |
| `mocked_test`                      | A mocked locus (parity / record-replay).                               |
| `disabled_no_route`                | No route ran (refusal / cancel before dispatch).                       |

The route-origin class MUST agree with the provider entry's
`execution_locus_class` and `enterprise_gateway_broker_posture_class`;
disagreement denies with `route_origin_disagrees_with_provider_entry`.

### 1.4 Outcome vocabulary

The `outcome_class` enum covers every observable run outcome:

| Outcome                                  | Meaning                                                            |
|------------------------------------------|--------------------------------------------------------------------|
| `outcome_success`                        | Bytes produced and admitted by the surface.                        |
| `outcome_partial_with_disclosure`        | Some bytes produced; remaining ceilings hit (token / tool / time). |
| `outcome_cancelled`                      | User or policy cancellation; not an error.                         |
| `outcome_provider_error`                 | The provider returned an error.                                    |
| `outcome_blocked_by_budget`              | Budget cap hit; the receipt is the refusal record.                 |
| `outcome_blocked_by_route`               | No admitted route at all (or all fallbacks unresolved).            |
| `outcome_blocked_by_policy`              | A policy / approval / trust state denied dispatch.                 |
| `outcome_blocked_by_taint_posture`       | Tainted-fence / approval rules denied dispatch.                    |
| `outcome_unknown_pre_dispatch`           | The receipt is a `preview_pre_dispatch` row; no outcome yet.       |

Schema gates pair `run_state_class` and `outcome_class` so a
post-run receipt with `outcome_unknown_pre_dispatch` denies, and a
pre-dispatch preview with any outcome other than
`outcome_unknown_pre_dispatch` denies. See section 6.1.

### 1.5 Coarse ceiling bands

Each receipt declares three coarse ceiling buckets so reviewers see
the bound the run was admitted under without seeing the raw number:

- `token_ceiling_class`: `tokens_under_2k`, `tokens_under_8k`,
  `tokens_under_32k`, `tokens_under_128k`, `tokens_over_128k`,
  `tokens_no_explicit_ceiling`, `tokens_unknown_unverified`.
- `tool_call_ceiling_class`: `no_tool_calls_admitted`,
  `bounded_tool_calls_under_4`, `bounded_tool_calls_under_16`,
  `bounded_tool_calls_under_64`, `unbounded_tool_calls_admin_only`,
  `tool_calls_unknown_unverified`.
- `wall_time_ceiling_class`: `wall_time_under_5s`,
  `wall_time_under_30s`, `wall_time_under_5m`,
  `wall_time_under_30m`, `wall_time_long_running_admin_only`,
  `wall_time_unknown_unverified`.

Reviewers see the bucket verbatim; the raw measurement series never
crosses this boundary.

### 1.6 Cumulative-receipt rollup for branch agents

A branch-agent run that chains multiple AI invocations MUST mint
one `branch_agent_route_rollup_record` per chain. The rollup carries:

- `branch_agent_chain_id` — stable opaque id of the chain.
- `per_hop_route_receipt_refs` — opaque refs to every per-hop
  receipt (length ≥ 1).
- `aggregate_run_state_class` — the worst-of-state across hops
  (`post_run_failed` dominates `post_run_completed`,
  `cancelled_by_*` dominates failed when a cancel was the reason
  the chain ended).
- `aggregate_outcome_class` — the worst-of-outcome across hops.
- `aggregate_cost_envelope_class` — coarse rollup of the per-hop
  cost-envelope classes (`bundled_no_incremental_cost` at every hop
  rolls up to `bundled_no_incremental_cost`; any metered hop bumps
  the rollup to the metered band the highest hop was in).
- `aggregate_token_ceiling_class` and
  `aggregate_wall_time_ceiling_class` — the largest band hit across
  hops.
- `agent_ceiling_class` — re-exported from the budget contract for
  the workflow.
- `originating_budget_routing_policy_ref` and
  `originating_graduation_packet_ref`.
- `originating_approval_ticket_ref` — required when
  `agent_ceiling_class` = `agent_invocation_unbounded_admin_only`.

A chain whose `agent_ceiling_class` is bounded but whose
`per_hop_route_receipt_refs` count exceeds the bound denies with
`agent_chain_exceeds_bounded_count`.

## 2. The spend receipt

### 2.1 Minimum payload

A `spend_receipt_record` MUST carry at minimum:

| Field                                 | Purpose                                                                |
|---------------------------------------|------------------------------------------------------------------------|
| `spend_receipt_id`                    | Stable opaque id.                                                      |
| `workflow_or_surface_id`              | Stable opaque id of the workflow/surface.                              |
| `route_receipt_ref`                   | Opaque ref to the matching `provider_route_receipt_record`.            |
| `assembly_id_ref`                     | Opaque ref to the `ai_context_assembly_record`.                        |
| `run_state_class`                     | Mirrors the route receipt's run-state.                                 |
| `cost_envelope_class`                 | Coarse cost-band bucket (re-exported from the budget contract).        |
| `cost_visibility_class`               | Re-exported from the provider registry.                                |
| `spend_attribution_dimensions`        | Set of dimensions emitted; MUST cover the seven minimum.               |
| `spend_attribution_values`            | Per-dimension typed value rows (the dimension class + opaque ref).     |
| `budget_scope_outcomes`               | Per-budget-scope outcome rows (`scope_under_band`, `scope_at_band`, `scope_over_band_blocked`). |
| `quota_family_class`                  | Re-exported from the provider registry.                                |
| `was_charged_to_user_class`           | One of `not_charged_local`, `not_charged_bundled`, `charged_user_byok_metered`, `charged_user_byok_subscription`, `charged_organisation_pooled`, `charged_organisation_subscription`, `charge_unknown_unverified`, `not_charged_run_blocked`. |
| `originating_budget_routing_policy_ref` | Opaque ref to the budget-routing policy.                              |
| `originating_graduation_packet_ref`   | Opaque ref to the packet; empty for shadow / experimental.             |
| `originating_route_selection_disclosure_ref` | Required when the run was not the cheapest qualifying route. |
| `policy_context`                      | Policy epoch, trust state, deployment profile, execution context.      |
| `redaction_class`                     | ADR-0011 redaction posture.                                            |
| `minted_at` / `last_updated_at`       | Monotonic timestamps.                                                  |

### 2.2 Spend-attribution minimum

Every spend receipt MUST emit at minimum the seven dimensions named
in the budget contract (workflow_or_surface_id, provider_entry_id,
model_entry_id, execution_locus_class, region_posture_class,
retention_stance_class, quota_family_class). Missing any denies with
`spend_attribution_minimum_set_missing`. Additional dimensions
(`feature_class_dimension`, `deployment_profile_class_dimension`,
`policy_epoch_dimension`, `agent_invocation_chain_id_dimension`,
`session_id_dimension`, `command_invocation_id_dimension`) narrow;
they do not widen.

### 2.3 Coarse cost-band only

The `cost_envelope_class` is the single cost readout reviewers see.
Raw cost amounts in any specific currency, raw token counts, raw
provider unit prices, raw billing-account ids, and raw user
identifiers never cross this boundary. The receipt carries
`opaque_provider_cost_unit_ref` only when the underlying metering
backend produced a typed receipt; the ref is opaque and resolves
through the eventual metering crate's Rust types.

### 2.4 Charge-locus vocabulary

`was_charged_to_user_class` answers "did this run cost the user
anything, and through which path":

| Charge locus                              | Meaning                                                  |
|-------------------------------------------|----------------------------------------------------------|
| `not_charged_local`                       | Local route; no incremental cost.                        |
| `not_charged_bundled`                     | Bundled subscription; no per-call charge.                |
| `charged_user_byok_metered`               | User's BYOK credential, metered band.                    |
| `charged_user_byok_subscription`          | User's BYOK credential, fixed subscription.              |
| `charged_organisation_pooled`             | Org-pooled enterprise quota.                             |
| `charged_organisation_subscription`       | Org subscription.                                        |
| `charge_unknown_unverified`               | Provider has not exposed cost metadata.                  |
| `not_charged_run_blocked`                 | Run was blocked / cancelled before any charge accrued.   |

A receipt whose `run_state_class` is `budget_blocked_refusal`,
`route_blocked_refusal`, `cancelled_by_user`, or `cancelled_by_policy`
MUST carry `was_charged_to_user_class = not_charged_run_blocked`;
disagreement denies with `charge_locus_disagrees_with_run_state`.

### 2.5 Branch-agent spend rollup

A `branch_agent_spend_rollup_record` mirrors the route rollup
(section 1.6) and aggregates per-hop spend receipts into one
cross-tool readout. It carries `aggregate_cost_envelope_class`,
`aggregate_was_charged_to_user_class` (the worst-of-locus across
hops), `aggregate_quota_family_class` (the most expensive band hit),
and `per_hop_spend_receipt_refs`.

## 3. Model-path comparison rows

### 3.1 Why minimum

A receipt that admits an alternative not-cheapest route MUST
enumerate the alternatives that were considered so the rollout-
review board, support packet, and replay harness see *why* the
cheaper / slower / safer / more local alternative was passed over.

A `model_path_comparison_row_record` carries:

| Field                                  | Purpose                                                              |
|----------------------------------------|----------------------------------------------------------------------|
| `comparison_row_id`                    | Stable opaque id.                                                    |
| `alternative_provider_entry_ref`       | Opaque ref to the candidate provider entry.                          |
| `alternative_model_entry_ref`          | Opaque ref to the candidate model entry.                             |
| `alternative_execution_locus_class`    | Locus of the candidate.                                              |
| `alternative_region_posture_class`     | Region of the candidate.                                             |
| `alternative_retention_stance_class`   | Retention of the candidate.                                          |
| `alternative_cost_envelope_class`      | Coarse cost-band of the candidate.                                   |
| `comparison_dimension_class`           | One of `cheaper_alternative`, `slower_alternative`, `safer_alternative_more_local`, `safer_alternative_lower_retention`, `safer_alternative_pinned_region`, `more_capable_alternative`, `more_deterministic_alternative`, `equivalent_alternative_parity`. |
| `selected_outcome_class`               | One of `selected_this_path`, `not_selected_failed_capability`, `not_selected_failed_region_posture`, `not_selected_failed_retention_stance`, `not_selected_failed_data_class_allowlist`, `not_selected_failed_quota_family`, `not_selected_failed_offline_posture`, `not_selected_failed_determinism_posture`, `not_selected_failed_taint_posture`, `not_selected_failed_approval_posture`, `not_selected_quota_exhausted`, `not_selected_budget_exhausted`, `not_selected_circuit_open`, `not_selected_deprecated_or_withdrawn`, `not_selected_pack_unverified`, `not_selected_user_explicit_choice`, `not_selected_policy_pin`, `not_selected_shadow_only_route`. |
| `notes_summary`                        | Reviewable sentence explaining the row.                              |

The `selected_outcome_class` enum is a strict superset of the budget
contract's `route_selection_override_reason_class` so the comparison
row is mechanically correlatable to the route-selection disclosure
the run was admitted under.

### 3.2 Comparison-dimension narrowing

Every `comparison_dimension_class` is a single axis. A route that
beat its alternative on more than one axis emits one comparison row
per axis. A receipt that omits comparison rows when the route was
not the cheapest qualifying route denies with
`route_receipt_missing_alternative_comparison`.

A receipt whose route was the cheapest qualifying route MAY omit
comparison rows; the route-selection-disclosure-ref is empty in
that case and reviewers know no override happened.

## 4. Pre- and post-dispatch disclosure vocabulary

### 4.1 Disclosure-state matrix

Every receipt advertises which disclosure-state class it occupies
so the UI, support packet, and replay harness render the same band:

| Disclosure state                           | Meaning                                                                    |
|--------------------------------------------|----------------------------------------------------------------------------|
| `preview_disclosure_pre_dispatch`          | Minted before bytes leave the device; identity + cost band shown verbatim. |
| `post_run_disclosure_completed`            | Minted on success.                                                         |
| `post_run_disclosure_partial`              | Bytes returned, but a ceiling clipped them.                                |
| `post_run_disclosure_failed`               | Provider error; identity still legible.                                    |
| `cancelled_disclosure_user`                | User cancelled; receipt survives in audit / UI history.                    |
| `cancelled_disclosure_policy`              | Policy cancelled; typed denial cited.                                      |
| `budget_blocked_disclosure`                | Refusal record under a budget cap; typed exhaustion cited.                 |
| `route_blocked_disclosure`                 | Refusal record under a route block; typed denial cited.                    |

Every receipt MUST carry exactly one `disclosure_state_class`. A
post-run receipt whose `disclosure_state_class` is
`preview_disclosure_pre_dispatch` denies with
`disclosure_state_disagrees_with_run_state`.

### 4.2 Receipts never disappear

A cancelled, failed, or blocked run MUST emit a receipt with the
matching state. The receipt survives in audit / UI history with the
same `route_receipt_id` namespace as completed runs; deleting a
cancelled / blocked receipt to keep history "clean" is forbidden and
denies with `receipt_delete_for_cancel_or_block_forbidden`. The
record-class registry's retention rules (governed elsewhere) decide
when a receipt rolls off audit; the receipt itself does not author
its own retention.

### 4.3 Generic 'AI used' wording is forbidden

A surface that wants to render "AI used" / "AI assisted" / "AI ran"
on a row MUST resolve through a `provider_route_receipt_record` and
render at least:

- the `display_label` of the provider entry,
- the `display_label` of the model entry,
- the `route_origin_class` band,
- the `cost_envelope_class` band,
- the `outcome_class` band.

A surface that hides any of those bands behind a generic 'AI used'
chip denies with `originless_ai_chip_forbidden_resolve_through_receipt`.
This denial reason is the mechanical close of the
"AI work never appears free or originless" acceptance criterion.

## 5. Approval-ticket composition

A receipt cites an `originating_approval_ticket_ref` only when the
underlying decision required an approval ticket per the budget
contract:

- `route_selection_override_reason_class` =
  `policy_pinned_more_expensive_route` or
  `user_explicit_choice_overrides_cheapest`, and the workflow's
  approval posture is above `allowed_without_prompt`, OR
- `agent_ceiling_class` = `agent_invocation_unbounded_admin_only`.

The approval-ticket vocabulary itself is authored on
[`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md);
this contract does not re-mint it.

## 6. Schema gates and denial reasons

### 6.1 Run-state ↔ outcome / disclosure / charge gates

The schema's `allOf` blocks freeze the gates:

- `run_state_class = preview_pre_dispatch` ⇒
  `outcome_class = outcome_unknown_pre_dispatch` AND
  `disclosure_state_class = preview_disclosure_pre_dispatch`.
- `run_state_class = post_run_completed` ⇒
  `outcome_class ∈ {outcome_success, outcome_partial_with_disclosure}`
  AND `disclosure_state_class ∈
  {post_run_disclosure_completed, post_run_disclosure_partial}`.
- `run_state_class = post_run_failed` ⇒
  `outcome_class ∈ {outcome_provider_error, outcome_partial_with_disclosure}`
  AND `disclosure_state_class = post_run_disclosure_failed`.
- `run_state_class = cancelled_by_user` ⇒
  `outcome_class = outcome_cancelled` AND
  `disclosure_state_class = cancelled_disclosure_user`.
- `run_state_class = cancelled_by_policy` ⇒
  `outcome_class ∈ {outcome_cancelled, outcome_blocked_by_policy, outcome_blocked_by_taint_posture}`
  AND `disclosure_state_class = cancelled_disclosure_policy`.
- `run_state_class = budget_blocked_refusal` ⇒
  `outcome_class = outcome_blocked_by_budget` AND
  `disclosure_state_class = budget_blocked_disclosure`.
- `run_state_class = route_blocked_refusal` ⇒
  `outcome_class ∈ {outcome_blocked_by_route, outcome_blocked_by_policy, outcome_blocked_by_taint_posture}`
  AND `disclosure_state_class = route_blocked_disclosure`.

### 6.2 Override / fallback ⇒ disclosure ref required

A receipt whose `route_selection_reason_class` is anything other
than `cheapest_qualifying_route_admitted` /
`no_cheaper_qualifying_route_existed` MUST cite a non-empty
`route_selection_disclosure_ref`. Missing the ref denies with
`route_selection_override_missing_disclosure`.

### 6.3 Comparison rows required when override happened

A receipt whose `route_selection_reason_class` is
`override_more_expensive_route_admitted` MUST carry
`model_path_comparison_rows` with at least one row whose
`selected_outcome_class` is `selected_this_path` (the chosen route)
and at least one row whose `selected_outcome_class` is one of the
`not_selected_*` classes. Missing the comparison row denies with
`route_receipt_missing_alternative_comparison`.

### 6.4 Branch-agent ⇒ rollup required

A surface that emits more than one route-receipt under the same
`branch_agent_chain_id` MUST also emit one
`branch_agent_route_rollup_record` and one
`branch_agent_spend_rollup_record` covering every hop. Missing the
rollup denies with `branch_agent_rollup_missing`.

### 6.5 Frozen denial-reason set

The full denial-reason vocabulary lives on the schema. The schema
adds these closed denials beyond the budget contract's set:

- `route_origin_disagrees_with_provider_entry`
- `disclosure_state_disagrees_with_run_state`
- `outcome_disagrees_with_run_state`
- `charge_locus_disagrees_with_run_state`
- `route_selection_override_missing_disclosure`
- `route_receipt_missing_alternative_comparison`
- `branch_agent_rollup_missing`
- `agent_chain_exceeds_bounded_count`
- `receipt_delete_for_cancel_or_block_forbidden`
- `originless_ai_chip_forbidden_resolve_through_receipt`
- `spend_attribution_minimum_set_missing`
- `policy_epoch_rolled_invalidations`
- `raw_cost_amount_forbidden_on_boundary`
- `raw_user_identifier_forbidden_on_boundary`
- `raw_billing_account_id_forbidden_on_boundary`
- `provider_route_receipt_schema_version_lagging`
- `spend_receipt_schema_version_lagging`

Denials fail closed; silent downgrade to a best-effort receipt or
to a generic 'AI used' chip is forbidden.

## 7. Redaction posture

Every receipt, comparison row, rollup, and audit event declares a
`redaction_class` from the ADR-0011 set (`metadata_safe_default`,
`operator_only_restricted`, `internal_support_restricted`,
`signing_evidence_only`). Raw URLs, raw endpoint hostnames, raw
provider payloads, raw API keys, raw OAuth tokens, raw mTLS
material, raw cost amounts in any specific currency, raw token
counts, raw provider unit prices, raw user identifiers, raw billing-
account ids, and raw evaluation prompts never cross this boundary
on any surface. Exports, support bundles, mutation-journal entries,
evidence packets, replay captures, and AI context captures carry
opaque refs and structured fields only.

Narrowing is permitted: admin policy MAY remove a `route_origin_class`,
a `cost_envelope_class`, an `outcome_class`, or a `was_charged_to_user_class`
from a deployment profile. Widening beyond the frozen rules is
forbidden.

## 8. Audit-event reuse

Every receipt mint / update / cancel / block / rollup event fires
on a dedicated `ai_route_receipt` audit stream with const ids
authored on the schemas:

- `provider_route_receipt_minted_preview`
- `provider_route_receipt_completed_post_run`
- `provider_route_receipt_failed_post_run`
- `provider_route_receipt_cancelled_by_user`
- `provider_route_receipt_cancelled_by_policy`
- `provider_route_receipt_budget_blocked`
- `provider_route_receipt_route_blocked`
- `branch_agent_route_rollup_emitted`
- `provider_route_receipt_schema_version_bumped`

And on the `ai_spend_receipt` audit stream:

- `spend_receipt_minted_preview`
- `spend_receipt_finalised_post_run`
- `spend_receipt_finalised_blocked`
- `branch_agent_spend_rollup_emitted`
- `spend_receipt_schema_version_bumped`

No new audit-event id is introduced by this contract on the
`ai_provider_registry`, `ai_model_registry`, `ai_graduation`,
`ai_external_tool_registry`, `ai_context`, or `provider_handoff`
streams; those streams keep their existing ids and the receipts
cross-reference them by ref.

## 9. Acceptance-criteria cross-walk

| Acceptance criterion                                                                                                                       | Where enforced                                                                                                                                                                                                                                  |
|--------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| A user or support packet can explain why a route was chosen and what budget class applied without reading implementation code or provider logs. | Sections 1, 2, 3 (typed receipt + comparison rows + cost-band readout). Schema: `provider_route_receipt_record` + `spend_receipt_record` + `model_path_comparison_row_record` with re-exported provider / model / locus / region / retention / cost-envelope vocabulary. |
| Cancelled, blocked, and fallback runs still emit truthful receipts instead of disappearing from audit or UI history.                       | Sections 1.2, 4.2, 6.1. Schema: `run_state_class` enum covers `cancelled_by_user`, `cancelled_by_policy`, `budget_blocked_refusal`, `route_blocked_refusal`; allOf gates pair run-state to outcome / disclosure / charge; denial `receipt_delete_for_cancel_or_block_forbidden`. |
| The contract leaves no path where provider/model identity or spend class can be hidden behind generic 'AI used' wording.                   | Sections 1.1, 4.3, 6.5. Schema: required `provider_entry_ref`, `model_entry_ref`, `route_origin_class`, `cost_envelope_class`, `outcome_class`; denial `originless_ai_chip_forbidden_resolve_through_receipt`.                                  |
| Worked examples exist for local-only route, policy-forced enterprise route, budget-capped refusal, fallback-to-cheaper-model, and branch-agent run with cumulative receipt rollup. | Fixtures under `/fixtures/ai/spend_receipt_cases/` (see README).                                                                                                                                                                                |

## 10. Schema-of-record posture

Rust types in the eventual provider-arbitration / metering crate
are the source of truth. The JSON Schema exports at
`schemas/ai/provider_route_receipt.schema.json` and
`schemas/ai/spend_receipt.schema.json` are the cross-tool boundary
every non-owning surface reads.

Adding a new `run_state_class`, `route_origin_class`, `outcome_class`,
`disclosure_state_class`, `was_charged_to_user_class`,
`token_ceiling_class`, `tool_call_ceiling_class`,
`wall_time_ceiling_class`, `comparison_dimension_class`,
`selected_outcome_class`, `audit_event_id`, or `denial_reason` value
is additive-minor and requires a `provider_route_receipt_schema_version`
or `spend_receipt_schema_version` bump; repurposing an existing
value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the AI provider / model registry contract,
the AI graduation / budget contract, the AI prompt-composer
contract, and the ADRs cited above.

## 11. Out of scope at this revision

- Live metering / spend-receipt emission and billing systems.
- Quota enforcement backends.
- Provider-pricing integration in any specific currency.
- Cross-organisation spend reconciliation; the receipt's
  `was_charged_to_user_class` band is the cross-tool readout
  reconciliation will read.
- Long-term retention rules for receipts; retention is governed by
  the record-class registry and is not re-authored here.
