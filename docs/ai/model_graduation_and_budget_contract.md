# AI graduation packet, rollout-state, and spend-routing contract

This document is the **product-wide contract** for how AI workflows
and AI-driven surfaces are promoted from experiment to stable claim,
how rollout state is pinned, how budget and routing controls are
named, how the cheapest qualifying route rule and its overrides are
disclosed, and how exhausted or blocked routes degrade explicitly to
local, BYOK, manual, or disabled states. It freezes one graduation-
packet shape, one rollout-state pin shape, one budget-routing-policy
shape, one route-selection-disclosure shape, and one set of const
audit-event ids every AI-adjacent surface reads, so a "ship to
stable" decision follows evidence rather than convenience and an
exhausted route degrades explicitly rather than failing opaquely.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / metering
surface's mint of its own copy, this document wins and the surface
is non-conforming.

The companion artifacts are:

- [`/schemas/ai/graduation_packet.schema.json`](../../schemas/ai/graduation_packet.schema.json)
  — boundary schema for the `graduation_packet_record`,
  `model_rollout_state_record`, `budget_routing_policy_record`,
  `route_selection_disclosure_record`, and
  `graduation_audit_event_record` shapes.
- [`/artifacts/ai/model_rollout_states.yaml`](../../artifacts/ai/model_rollout_states.yaml)
  — machine-readable register of the six rollout states, their
  visibility class, their minimum eval-evidence requirements, the
  fallback postures they admit, and the legal promotion transitions.
- [`/fixtures/ai/graduation_packets/`](../../fixtures/ai/graduation_packets/)
  — worked-example corpus with at least one packet per non-disabled
  rollout state, one budget-routing policy, one cheapest-qualifying
  route override, one budget-exhaustion fallback, and one withdrawal.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, local-model pack, and external-tool
  identity, execution locus, transport, auth mode, retention, region,
  quota family, cost visibility, and tainted-return posture are
  authored there. A graduation packet binds to one
  `provider_entry_id`, one `model_entry_id`, optional
  `local_model_pack_refs`, and optional external-tool refs through
  the rows already governed by that contract.
- [`/docs/ai/prompt_composer_contract.md`](./prompt_composer_contract.md) —
  prompt-pack and tool-pack manifests, with stable id, version,
  digest, signing posture, and changelog, are authored there. A
  graduation packet binds to one `prompt_pack_manifest_ref` and one
  `tool_pack_manifest_ref`; the packet does not re-author pack
  versioning.
- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  data-class allowlist, redaction posture, and tainted-fence
  vocabulary. Spend-attribution dimensions reuse the data-class and
  redaction-class vocabulary; this contract does not re-mint them.
- [`/docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md) —
  evidence packets quote the provider, model, locus, region, and
  retention identity. A stable-rollout packet's `linked_assurance_
  claim_refs` cite the assurance-claim rows backed by those evidence
  packets; this contract does not re-mint evidence packet shape.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, `deployment_profile_class`, policy epoch,
  and trust state on every record.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which rollout states, which fallback
  postures, which route-selection overrides, and which agent
  ceilings are admitted per deployment profile; policy MAY NOT
  silently widen any axis.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary. Every promotion to preview, stable,
  deprecated, or disabled MUST cite an admitting approval ticket.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `redaction_class` is re-exported without modification.

If this document disagrees with those sources, those sources win and
this document plus the schema are updated in the same change.

This document does not ship a live provider-arbitration service, a
live metering / billing pipeline, a live evaluation harness, or a
live red-team corpus runner. It freezes the contract those
implementations will read and write. The eventual provider-
arbitration and metering crates' Rust types are the schema of record;
the JSON Schema export at
`schemas/ai/graduation_packet.schema.json` is the cross-tool
boundary every non-owning surface reads.

## Why freeze this now

Without one frozen contract the product is free to invent a per-
workflow notion of "ready for general availability", a per-surface
notion of "preview", a per-lane notion of "deprecated", and a per-
caller notion of what to do when the route is exhausted. Each
divergence widens a different axis silently:

1. *A surface ships a stable claim without a current evaluation
   packet, a current red-team packet, or a measured latency
   envelope.* Reviewers cannot trace the claim back to evidence.
2. *A workflow picks a more expensive model than the cheapest
   qualifying route admits, with no record of why.* Spend reviewers
   cannot tell whether the override was a capability requirement, a
   user choice, a policy pin, or an unintended widening.
3. *A request hits an exhausted quota or revoked eligibility and
   fails opaquely.* Support cannot tell whether the user lost
   access, the quota was exhausted, the pack was quarantined, or the
   provider was disabled.
4. *A rolled-back model entry stays pinned in the rollout state
   record because no rollback plan was authored.* New invocations
   silently keep routing to a model the rollout review board has
   already declared unsafe.

This contract closes that gap with **one packet shape, one rollout-
state pin, one budget-routing policy, one route-selection
disclosure, and one set of const audit-event ids** every AI-adjacent
surface reads.

## Who reads this document

- **Rollout-review-board / release-management authors** who promote
  a workflow from experiment to stable, demote it to deprecated, or
  withdraw it to disabled, and need a frozen packet shape for the
  decision.
- **Provider-arbitration / route-planner authors** who resolve a
  request against the cheapest qualifying route, emit a
  route-selection-disclosure when the cheapest route was overridden
  or unavailable, and degrade explicitly to a fallback route when
  the primary is exhausted.
- **Metering / spend-receipt authors** who emit per-request,
  per-session, per-agent-invocation, per-workflow, per-user, and
  per-organisation spend receipts with the seven required spend-
  attribution dimensions on every receipt.
- **Admin / policy / settings surface authors** narrowing which
  rollout states, fallback postures, route-selection overrides, and
  agent ceilings are admitted per deployment profile.
- **Evidence / replay / support / parity-audit authors** quoting the
  graduation-packet ref, the rollout-state pin ref, and the budget-
  routing policy ref on every evidence packet so the rollout history
  is reconstructable from the audit stream alone.

## 1. The graduation packet

### 1.1 Minimum payload

A `graduation_packet_record` MUST carry at minimum:

| Field                                | Purpose                                                                  |
|--------------------------------------|--------------------------------------------------------------------------|
| `graduation_packet_id`               | Stable opaque id; supersedes-chains use it.                               |
| `workflow_or_surface_id`             | Stable opaque id of the workflow or surface (e.g. inline-completion).     |
| `promotion_kind`                     | One of `promote_to_experimental`, `_shadow`, `_preview`, `_stable`, `demote_to_deprecated`, `withdraw_to_disabled`, `rollback_to_prior_state`, `no_state_change_refresh`. |
| `target_rollout_state_class`         | The rollout state the packet asks to pin.                                  |
| `provider_entry_ref` / `model_entry_ref` | The provider and model entries the packet binds (from the registry contract). |
| `prompt_pack_manifest_ref` / `tool_pack_manifest_ref` | The prompt-pack and tool-pack manifests the packet binds (from the prompt-composer contract). |
| `eval_set_ref` / `eval_thresholds_ref` | The eval set and threshold artifact the packet was judged against. |
| `cost_profile_ref` / `kill_switch_ref` / `owner_ref` | The cost profile, revocation lever, and accountable owner for packet renewal or rollback. |
| `eval_evidence_entries`              | Eval / red-team / latency / cost evidence rows.                            |
| `latency_envelope_class`             | Coarse latency-envelope bucket the workflow is admitted under.             |
| `cost_envelope_class`                | Coarse cost-envelope bucket the workflow is admitted under.                |
| `fallback_posture_class`             | What happens when the primary route is exhausted.                          |
| `rollback_plan`                      | What is reverted, by whom, and on which trigger.                           |
| `originating_approval_ticket_ref`    | Required (non-empty) for promotion to preview / stable / deprecated / disabled.|
| `linked_assurance_claim_refs`        | Required (non-empty) for stable.                                           |
| `policy_context`                     | Policy epoch, trust state, deployment profile, execution context.          |
| `redaction_class`                    | ADR-0011 redaction posture.                                                |
| `minted_at` / `expires_at`           | Monotonic timestamps; an expired packet denies new sessions.               |

### 1.2 Promotion-kind requirement narrowing

The `eval_evidence_entries` set narrows by `promotion_kind`:

| Promotion kind            | Minimum evidence set                                                                                 |
|---------------------------|------------------------------------------------------------------------------------------------------|
| `promote_to_experimental` | `no_evidence_admitted_with_disclosure` is admitted; no other floor.                                  |
| `promote_to_shadow`       | At least one `parity_harness_passed` row OR `no_evidence_admitted_with_disclosure` with disclosure.  |
| `promote_to_preview`      | At least one `protected_eval_corpus_passed` row AND at least one `red_team_corpus_passed` row.       |
| `promote_to_stable`       | At least one each of `protected_eval_corpus_passed`, `red_team_corpus_passed`, `latency_envelope_measured`, `cost_envelope_measured`. |
| `demote_to_deprecated`    | At least one `protected_eval_corpus_passed` row.                                                     |
| `withdraw_to_disabled`    | No floor; the rollback plan and admitting approval ticket carry the decision.                        |
| `rollback_to_prior_state` | No floor; the rollback plan names the axes reverted.                                                  |
| `no_state_change_refresh` | Re-mints the packet under a new policy epoch without changing the rollout state.                     |

A packet whose `promotion_kind` is `promote_to_stable` and whose
`eval_evidence_entries` does not satisfy the four-row minimum denies
with `promote_to_stable_missing_required_evidence`. The schema's
`allOf` gate enforces this mechanically.

### 1.3 Eval-posture rules

Every `eval_evidence_entry` declares an `eval_posture_class`. A
`promote_to_stable` packet MUST carry `first_party_verified_passed`
or `parity_lab_passed` on every required evidence kind;
`vendor_attested_passed`, `community_reported_passed`, and
`pending_*` are admitted only on shadow / experimental / preview
packets. A first-party / parity-lab posture MUST cite a non-empty
`evaluation_packet_ref`; missing the ref denies through the
`eval_evidence_entry` allOf gate.

### 1.4 Latency and cost envelopes

The `latency_envelope_class` and `cost_envelope_class` are coarse
buckets. Reviewers see the bucket verbatim and never the raw
measurement series or raw cost number in any specific currency.
`envelope_unknown_unverified` is admitted only on
`promote_to_experimental` and `promote_to_shadow`; promotion above
shadow without a measured envelope denies with
`promote_above_shadow_requires_latency_envelope` /
`promote_above_shadow_requires_cost_envelope`.

### 1.5 Fallback contract

Every packet MUST declare a `fallback_posture_class`. The admitted
values are:

| Fallback posture                                | Meaning                                                              |
|-------------------------------------------------|----------------------------------------------------------------------|
| `degrade_to_local_route`                        | Re-resolve to the local-pack-served provider entry on the workflow.  |
| `degrade_to_byok_route`                         | Re-resolve to a BYOK provider entry the user already authorised.    |
| `degrade_to_manual_review`                      | Hand the request to a manual review surface; no AI route taken.     |
| `degrade_to_offline_cached_response`            | Serve a cached response under the offline-snapshot contract.        |
| `disable_with_typed_denial`                     | Deny with the matching typed denial reason.                         |
| `no_fallback_request_fails_with_typed_denial`   | No fallback admitted; deny with the matching typed denial reason.   |
| `fallback_to_prior_rollout_state`               | Re-resolve to the immediately prior rollout-state pin.              |

A fallback that pins a target route MUST carry a non-empty
`fallback_target_provider_entry_ref`; missing the ref denies with
`fallback_target_route_unresolved`. Opaque failure (no typed denial)
is forbidden and denies with `fallback_posture_opaque_failure_
forbidden`.

### 1.6 Rollback plan

Every packet MUST carry a `rollback_plan`. The plan's
`rollback_plan_kind` is one of `rollback_to_prior_provider_entry`,
`_prior_model_entry`, `_prior_prompt_pack_manifest`,
`_prior_tool_pack_manifest`, `_prior_rollout_state`,
`_local_only_route`, `_disabled_state`, or
`rollback_combination_multi_axis`. A multi-axis rollback MUST cite
one ref per reverted axis. A packet whose plan kind names a target
axis but whose corresponding ref is empty denies with
`graduation_packet_missing_rollback_plan`.

## 2. Rollout state

### 2.1 The six allowed states

The rollout-state vocabulary is frozen at six classes, registered in
[`/artifacts/ai/model_rollout_states.yaml`](../../artifacts/ai/model_rollout_states.yaml):

| Rollout state    | User visibility                                          | Requires graduation packet | Requires admitting approval ticket | Requires assurance-claim link |
|------------------|----------------------------------------------------------|----------------------------|------------------------------------|-------------------------------|
| `experimental`   | Behind opt-in flag; no public claim.                     | No                         | No                                 | No                            |
| `shadow`         | Not surfaced; paired with the live route for parity.     | No                         | No                                 | No                            |
| `preview`        | Surfaced under a typed preview chip to opted-in cohort.  | Yes                        | Yes                                | No                            |
| `stable`         | Surfaced to the general population under stable claim.   | Yes                        | Yes                                | Yes                           |
| `deprecated`     | Surfaced with a typed deprecation chip.                  | Yes                        | Yes                                | No                            |
| `disabled`       | Not admitted; typed denial only.                         | No (the row is the pin)    | Yes                                | No                            |

A workflow that claims `stable` without a current
`graduation_packet_ref` denies with
`stable_claim_missing_graduation_packet`. The schema's allOf gate on
`model_rollout_state_record` enforces this mechanically.

### 2.2 Rollout-state vs model-lifecycle

`rollout_state_class` (this contract) names the rollout phase of one
**workflow / surface**. `model_lifecycle_state_class` (the model
registry contract) names the lifecycle of one **model entry**. The
two compose: a stable workflow MUST resolve to a model entry whose
lifecycle state is `preview_general`, `generally_available`, or
`stable_long_term_support`; resolution against any other lifecycle
state denies through the model registry's typed denial. This
contract does not re-mint the model-lifecycle vocabulary.

### 2.3 Admitted transitions

The legal promotion / demotion / withdrawal transitions are
enumerated in the rollout-states artifact. Any other transition
MUST be expressed as `withdraw_to_disabled` followed by a fresh
packet for the next state, so audit / replay consumers do not
observe an undefined transition.

### 2.4 Policy-epoch invalidation

A policy-epoch roll between pin and invocation invalidates the pin
and forces re-resolution against a current packet; the next
invocation MUST re-resolve or deny with
`policy_epoch_rolled_invalidations`. Silent re-use is forbidden.

## 3. Budget and routing controls

### 3.1 Budget caps

A `budget_routing_policy_record` MUST declare at minimum three
budget caps covering `per_request`, `per_session`, and
`per_agent_invocation`. A policy missing any of those scopes denies
with `budget_routing_policy_missing_required_scope`. Policies MAY
declare additional caps on `per_workflow`, `per_user`,
`per_organisation`, `per_deployment_profile`, and
`per_policy_bundle`. Each cap names its `cost_envelope_class`
bucket — reviewers never see a raw cost amount in any specific
currency.

### 3.2 Cheapest-qualifying-route rule

The `cheapest_qualifying_route_rule_class` enumerates how the policy
permits selecting a more expensive route:

| Rule class                                                | Meaning                                                                    |
|-----------------------------------------------------------|----------------------------------------------------------------------------|
| `cheapest_qualifying_route_default`                       | Always pick the cheapest route that meets the qualification axes.          |
| `cheapest_qualifying_route_with_user_override_admitted`   | A user-explicit choice may override.                                       |
| `cheapest_qualifying_route_with_policy_pin_admitted`      | A policy pin may override.                                                 |
| `cheapest_qualifying_route_disabled_pinned_route_only`    | Policy pins exactly one route and ignores cost (admin-only quota family).  |

A request that does not pick the cheapest qualifying route MUST
mint a `route_selection_disclosure_record` before invocation citing
the override reason; the disclosure is the cross-tool readout the
spend receipt and rollout-review board read. Section 4 details the
disclosure shape.

### 3.3 Spend-attribution dimensions

Every `budget_routing_policy_record` MUST declare a
`spend_attribution_dimensions` set with at minimum seven dimensions
present:

1. `workflow_or_surface_id_dimension`
2. `provider_entry_id_dimension`
3. `model_entry_id_dimension`
4. `execution_locus_class_dimension`
5. `region_posture_class_dimension`
6. `retention_stance_class_dimension`
7. `quota_family_class_dimension`

Additional dimensions (`feature_class_dimension`,
`deployment_profile_class_dimension`, `policy_epoch_dimension`,
`agent_invocation_chain_id_dimension`, `session_id_dimension`,
`command_invocation_id_dimension`) narrow; they do not widen.
Missing any minimum dimension denies with
`spend_attribution_minimum_set_missing`.

### 3.4 Agent ceilings

A workflow that admits agent-style chained invocations MUST declare
an `agent_ceiling_class`:

| Ceiling class                                      | Meaning                                                            |
|----------------------------------------------------|--------------------------------------------------------------------|
| `no_agent_invocations_admitted`                    | No chaining; one invocation per request.                           |
| `single_invocation_no_recursion`                   | Exactly one nested invocation per parent.                          |
| `bounded_recursion_depth_admitted`                 | Bounded depth, unbounded count.                                    |
| `bounded_invocation_count_admitted`                | Bounded count, unbounded depth.                                    |
| `bounded_recursion_and_count_admitted`             | Bounded on both axes.                                              |
| `agent_invocation_unbounded_admin_only`            | Unbounded; gated to admin-approved policy bundles.                 |

The unbounded class MUST cite a non-empty
`originating_approval_ticket_ref`; missing the ticket denies with
`agent_ceiling_missing_admin_approval`.

### 3.5 Fallback posture on exhaustion

A `budget_routing_policy_record` declares a top-level
`fallback_posture_class` and one optional per-cap
`exhaustion_fallback_posture_class`. The values are the same closed
set as the graduation-packet fallback posture (section 1.5). Every
fallback action MUST cite an `exhaustion_state_class` so reviewers
see which axis was exhausted; missing the state denies with
`fallback_action_missing_exhaustion_state`.

## 4. Route-selection disclosure

A `route_selection_disclosure_record` is minted before invocation
whenever:

1. The selected route is not the cheapest qualifying route, OR
2. The cheapest qualifying route was unavailable and a fallback was
   taken, OR
3. No route was admitted at all and the request is denied.

The disclosure carries:

- The selected `provider_entry_ref` and `model_entry_ref`.
- The candidate cheapest `provider_entry_ref` and `model_entry_ref`
  the policy considered (empty only when no candidate existed).
- The `route_selection_reason_class` (cheapest_qualifying,
  override_more_expensive, fallback_after_cheapest_exhausted,
  fallback_after_cheapest_blocked, user_chose_specific_route,
  policy_pinned_specific_route, shadow_route_for_parity_only,
  no_route_admitted_disabled_with_typed_denial).
- The `route_selection_override_reason_class` (the eighteen
  override reasons frozen in the schema, including
  `cheapest_route_failed_capability_check`,
  `cheapest_route_failed_region_posture`,
  `cheapest_route_failed_retention_stance`,
  `cheapest_route_failed_data_class_allowlist`,
  `cheapest_route_quota_exhausted`,
  `cheapest_route_budget_exhausted`,
  `cheapest_route_circuit_open_recent_failures`,
  `cheapest_route_pack_unverified_or_quarantined`,
  `user_explicit_choice_overrides_cheapest`,
  `policy_pinned_more_expensive_route`).
- The `exhaustion_state_class` if a fallback was taken.
- The `spend_attribution_dimensions` the disclosure surfaces.
- The `originating_approval_ticket_ref` if the override or fallback
  required one.

A disclosure whose `route_selection_reason_class` is
`override_more_expensive_route_admitted` MUST carry an override
reason other than `no_override_cheapest_was_used`; the schema's
allOf gate enforces this mechanically.

## 5. Exhausted or blocked routes degrade explicitly

### 5.1 Typed denials only

Every fallback path MUST cite an `exhaustion_state_class`. The set
covers per-scope budget exhaustion (`per_request_budget_exhausted`,
`per_session_budget_exhausted`, etc.), quota exhaustion
(`quota_family_exhausted`), agent-ceiling exhaustion
(`agent_ceiling_exhausted`), circuit-open
(`circuit_open_recent_failures`), and four eligibility-revoked
states (`eligibility_revoked_policy`,
`eligibility_revoked_workspace_trust`,
`eligibility_revoked_pack_quarantined`,
`eligibility_revoked_provider_disabled`).

A request that takes a fallback without naming an exhaustion state
denies with `fallback_action_missing_exhaustion_state`. A request
that fails opaquely (no typed denial) denies with
`fallback_posture_opaque_failure_forbidden`. Silent failure is
forbidden.

### 5.2 The four explicit fallback targets

When the primary route is exhausted, the workflow's fallback
posture resolves to one of:

- **Local route.** The pinned `fallback_target_provider_entry_ref`
  resolves to a local-in-process / local-sandbox / local-companion
  provider entry from the registry; the local route inherits the
  data-class allowlist of that entry.
- **BYOK route.** The pinned ref resolves to a BYOK vendor or BYOK
  self-hosted provider entry the user already authorised; if no
  BYOK route is authorised, the workflow degrades further to manual
  review or disabled.
- **Manual review.** The request is handed to a manual review
  surface (a non-AI workflow) that resolves through ordinary
  command / palette / agent UX. No AI route is taken.
- **Disabled with typed denial.** The request denies with the
  matching typed denial; no fallback is taken.

A workflow that admits a fallback target which is not in this list
denies through the schema's `fallback_posture_class` enum.

### 5.3 Worked example

The fixture
[`fixtures/ai/graduation_packets/cheapest_route_budget_exhausted_fallback.yaml`](../../fixtures/ai/graduation_packets/cheapest_route_budget_exhausted_fallback.yaml)
shows the full path: the cheapest qualifying route's per-session
budget is exhausted, the budget-routing policy's
`fallback_posture_class` is `degrade_to_local_route`, the route-
selection disclosure cites
`fallback_after_cheapest_exhausted` /
`cheapest_route_budget_exhausted` /
`per_session_budget_exhausted`, and a paired audit event
(`route_fallback_taken`) emits on the `ai_graduation` audit stream.

## 6. Approval-ticket composition

Every `promote_to_preview`, `promote_to_stable`, `demote_to_
deprecated`, and `withdraw_to_disabled` graduation packet MUST cite
a non-empty `originating_approval_ticket_ref` resolving against the
ADR-0010 approval-ticket schema. Every
`agent_invocation_unbounded_admin_only` budget-routing policy MUST
cite a non-empty admitting approval ticket. Route-selection
overrides under
`user_explicit_choice_overrides_cheapest` /
`policy_pinned_more_expensive_route` cite an approval ticket only
when the workflow's underlying approval posture (from the provider
registry) is above `allowed_without_prompt`.

The approval-ticket vocabulary itself is authored on
[`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md);
this contract does not re-mint it.

## 7. Redaction posture

Every graduation packet, rollout-state pin, budget-routing policy,
route-selection disclosure, and audit event declares a
`redaction_class` from the ADR-0011 set (`metadata_safe_default`,
`operator_only_restricted`, `internal_support_restricted`,
`signing_evidence_only`). Raw URLs, raw endpoint hostnames, raw
evaluation prompts, raw evaluation response bodies, raw red-team
prompts, raw red-team transcripts, raw latency or cost measurement
series, raw cost amounts in any specific currency, raw user
identifiers, raw billing-account ids, raw API keys, raw OAuth
tokens, raw mTLS material, raw model weights, raw pack bytes, and
raw provider payloads never cross this boundary on any surface.
Exports, support bundles, mutation-journal entries, evidence
packets, replay captures, and AI context captures carry opaque refs
and structured fields only.

Narrowing is permitted: admin policy MAY remove a rollout state, a
fallback posture, a route-selection override class, or an agent
ceiling from a deployment profile. Widening beyond the frozen rules
is forbidden.

## 8. Audit-event reuse

Every graduation packet mint / promotion / demotion / withdrawal /
rollback / supersede event, every rollout-state pin / invalidation
event, every budget-routing policy mint / update / supersede event,
every route-selection override disclosure, and every fallback /
no-route-available event fires on the dedicated `ai_graduation`
audit stream named in the schema:

- `graduation_packet_minted`
- `graduation_packet_promoted`
- `graduation_packet_demoted`
- `graduation_packet_withdrawn`
- `graduation_packet_superseded`
- `graduation_packet_rolled_back`
- `model_rollout_state_pinned`
- `model_rollout_state_invalidated_by_policy_epoch`
- `model_rollout_state_invalidated_by_pack_change`
- `budget_routing_policy_minted`
- `budget_routing_policy_updated`
- `budget_routing_policy_superseded`
- `route_selection_override_disclosed`
- `route_fallback_taken`
- `route_fallback_denied_no_route_available`
- `graduation_packet_schema_version_bumped`

No new audit-event id is introduced by this contract on the
`ai_provider_registry`, `ai_model_registry`,
`ai_external_tool_registry`, `ai_context`, or `provider_handoff`
streams; those streams keep their existing ids.

## 9. Acceptance-criteria cross-walk

| Acceptance criterion                                                                                                                            | Where enforced                                                                                                                                                                                                                                                                |
|-------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| No AI path can claim stable status without a current graduation-packet fixture.                                                                  | Section 2.1 (rollout-state requirement matrix) + section 1.2 (promotion-kind narrowing). Schema: `model_rollout_state_record` allOf gate forcing `graduation_packet_ref` non-empty for `stable`, `preview`, `deprecated`; `graduation_packet_record` allOf forcing `promote_to_stable` to carry the four-row evidence minimum and a non-empty admitting approval ticket and at least one linked assurance-claim. Denial reasons `stable_claim_missing_graduation_packet`, `promote_to_stable_missing_required_evidence`, `graduation_packet_expired`. |
| Routing controls can express why a more expensive model was chosen when the cheapest qualifying route was not used.                              | Section 3.2 (cheapest-qualifying-route rule) + section 4 (route-selection disclosure). Schema: `route_selection_reason_class` and `route_selection_override_reason_class` enums + `route_selection_disclosure_record` allOf gates forcing override-more-expensive to cite a non-trivial override reason and forcing fallback reasons to cite a non-not_exhausted exhaustion state. Audit event `route_selection_override_disclosed`. |
| Exhausted or blocked routes degrade explicitly to local, BYOK, manual, or disabled states rather than failing opaquely.                          | Section 5 (typed denials + four explicit fallback targets). Schema: `fallback_posture_class` enum (degrade_to_local_route, degrade_to_byok_route, degrade_to_manual_review, degrade_to_offline_cached_response, disable_with_typed_denial, no_fallback_request_fails_with_typed_denial, fallback_to_prior_rollout_state) + `exhaustion_state_class` enum + denial reasons `fallback_action_missing_exhaustion_state`, `fallback_posture_opaque_failure_forbidden`, `fallback_target_route_unresolved`. Audit events `route_fallback_taken`, `route_fallback_denied_no_route_available`. |
| Per-request, per-session, per-agent-invocation, per-workflow, per-user, and per-organisation budgets are expressible.                            | Section 3.1 (budget caps) + 3.3 (spend-attribution). Schema: `budget_scope_class` enum + `budget_routing_policy_record` `budget_caps` `minItems: 3` + `spend_attribution_dimensions` `minItems: 7` + denial reasons `budget_routing_policy_missing_required_scope`, `spend_attribution_minimum_set_missing`. |
| Agent ceilings can be expressed and admin-approval is required for unbounded chains.                                                              | Section 3.4. Schema: `agent_ceiling_class` enum + `budget_routing_policy_record` allOf gate forcing `agent_invocation_unbounded_admin_only` to carry a non-empty `originating_approval_ticket_ref`. Denial reason `agent_ceiling_missing_admin_approval`. |
| The graduation packet carries protected-eval, red-team, latency-envelope, and cost-envelope evidence.                                            | Section 1.1 + 1.2 + 1.3 + 1.4. Schema: `eval_evidence_kind` enum + `eval_posture_class` enum + `eval_evidence_entry` + `graduation_packet_record` `promote_to_stable` allOf gate forcing `minItems: 4` on `eval_evidence_entries`. |
| Worked examples exist for each non-disabled rollout state, for one route-selection override, for one budget-exhaustion fallback, and for one withdrawal. | Fixtures under `/fixtures/ai/graduation_packets/` (see README).                                                                                                                                                                                                          |

## 10. Schema-of-record posture

Rust types in the eventual provider-arbitration / metering crate are
the source of truth. The JSON Schema export at
`schemas/ai/graduation_packet.schema.json` is the cross-tool
boundary every non-owning surface reads.

Adding a new `rollout_state_class`, `promotion_kind`,
`eval_evidence_kind`, `eval_posture_class`, `latency_envelope_class`,
`cost_envelope_class`, `budget_scope_class`,
`agent_ceiling_class`, `spend_attribution_dimension_class`,
`route_selection_reason_class`,
`route_selection_override_reason_class`, `fallback_posture_class`,
`exhaustion_state_class`, `rollback_plan_kind`,
`audit_event_id`, or `denial_reason` value is additive-minor and
requires a `graduation_packet_schema_version` bump; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, ADR 0010, ADR 0011, the existing AI context-
assembly contract, the AI provider / model registry contract, and
the AI prompt-composer contract.

## 11. Out of scope at this revision

- Live provider arbitration and route planning (the service that
  picks which entry to invoke).
- Live metering / spend-receipt emission and billing systems.
- Running the real protected-evaluation program, red-team corpus,
  parity harness, fairness audit, or user-study program. This
  contract freezes the row shape those programs will read and
  write.
- The rollout-review board's UI; this contract freezes the row
  shape the review-board reads.
- Cost / pricing tables in any specific currency. The cost
  envelope is a coarse bucket; cost numbers are out of scope.
- Long-term retention rules for graduation packets, rollout-state
  pins, and audit events. Retention is governed by the support /
  release / replay contracts and is not re-authored here.
