# Provider/model registry beta

The beta registry is the executable source of truth for AI provider,
model, local-model, and external-tool route facts. It lives in
`aureline_ai::registry` and serializes through
[`/schemas/ai/provider_model_registry.schema.json`](../../../schemas/ai/provider_model_registry.schema.json).

The registry composes the earlier provider, model, and external-tool
schemas into one packet so product surfaces do not maintain separate
copies of route truth. UI rows, docs/help rows, CLI/headless rows, and
support exports read the same `registry_id`, provider rows, model rows,
route policies, and claimed-surface records.

## Runtime contract

`ProviderModelRegistryPacket` carries:

- provider rows with provider family, model refs, execution locus,
  transport, auth mode, region posture, retention stance, quota family,
  route eligibility, policy-allowed route choices, and route disclosure refs;
- model rows with model family, variant, capability version, lifecycle,
  supported feature classes, and serving providers;
- external-tool rows with tool family, transport, execution locus, auth
  posture, allowed side effects, data classes, approval posture, and
  policy-allowed route choices;
- claimed AI surface rows with required feature class, routing policy,
  required disclosure kinds, retrieval/index truth, and UI/docs/support
  projection refs;
- route policies for `local_first_then_cheapest`,
  `cheapest_qualifying`, `policy_pinned`, `manual_only`, and `disabled`.

`ProviderModelRegistryPacket::resolve_route_for_surface()` selects from
that registry state. Local-first policy selects an eligible local locus
before considering hosted alternatives. Cheapest-qualifying policy selects
the lowest-cost eligible route after capability, lifecycle, policy, and
quota checks. No caller needs a hidden provider switch to enforce either
rule.

`ProviderModelRegistryPacket::routing_packet_for_surface()` bridges the
selected registry route into the existing `AiRoutingPacket` support path,
so current routing exports can disclose provider/model identity,
execution location, route reason, quota state, latency/cost envelope, and
visible route-change lineage without duplicating registry facts.

## Promotion guards

Validation blocks claimed beta rows when:

- provider or model family identity is missing;
- execution location or policy-allowed route choices are missing;
- a claimed surface lacks required pre-invocation disclosures;
- local-first policy has no eligible local route;
- cheapest-qualifying policy has no eligible route;
- an external tool declares mutating side effects without an approval gate;
- retrieval/index state is partial without a visible label;
- UI, docs/help, and support-export projections point at different registry
  state refs;
- exportable registry fields contain raw boundary material such as endpoint
  URLs or credential tokens.

## Seed fixture

The fixture at
[`/fixtures/ai/provider_model_registry_beta/registry_packet.json`](../../../fixtures/ai/provider_model_registry_beta/registry_packet.json)
covers:

- an inline chat surface using local-first routing, selecting a local
  companion-service provider even when a cheaper hosted route is available;
- a review chat surface using cheapest-qualifying routing, selecting the
  managed hosted route;
- an enterprise-gateway external tool row whose output remains tainted and
  whose provider-visible side effect requires approval;
- shared UI/docs/support projection refs that all read the same registry
  state.

## Verification

```sh
cargo test -p aureline-ai registry --no-fail-fast
```
