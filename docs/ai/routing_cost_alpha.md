# AI Routing Cost Alpha

This document defines the first bounded provider-routing packet for hosted AI paths. It does not add a dispatcher and does not widen provider trust. It gives shell, diagnostics, evidence, and support-export surfaces one export-safe object to answer: which provider/model would run, why that route was selected, what quota state gates it, and which latency/cost envelope applies.

## Packet

The canonical type is `aureline_ai::routing::AiRoutingPacket`.

Each packet carries:

- Provider and model refs plus display labels for the selected route.
- Execution locus, route origin, region posture, and retention stance tokens.
- Quota family, quota state, quota scope, budget owner ref, quota meter refs, quota explanation, and local-continuity copy.
- Latency envelope, cost envelope, cost visibility, token ceiling, tool-call ceiling, wall-time ceiling, budget-policy ref, rollout packet ref, and envelope evidence ref.
- Route-selection reason, override reason, exhaustion state, selected outcome, and visible route-change lineage when policy or fallback changes the route.
- Policy epoch, deployment profile, workspace trust state, identity-mode baseline ref, capability-lifecycle row ref, and source contract refs.

The packet is metadata only. It must not carry raw provider URLs, raw endpoint hostnames, credential bodies, raw provider payloads, exact cost amounts, exact token counts, raw user identifiers, or billing-account ids.

## Support Projection

`AiRoutingPacket::support_packet()` projects an `AiRoutingSupportPacket` with the same route truth as export-safe tokens and opaque refs. The projection includes validation violation tokens if a packet is malformed, so support bundles can explain refusal state without inspecting provider logs.

Support export consumers should render at minimum:

- Provider and model.
- Execution locus and route origin.
- Quota state, family, and scope.
- Latency envelope, cost envelope, and cost visibility.
- Route-selection reason, override reason, exhaustion state, and route-change lineage.
- Local-continuity label when hosted capacity is blocked.

## Route Changes

The cheapest qualifying route remains the default. Any policy pin, user choice, fallback, circuit-open fallback, provider disablement, quota exhaustion, or budget exhaustion must set a non-default route-selection reason and emit a visible `RouteChangeLineage` row targeting the selected candidate.

For hosted routes, validation fails closed when:

- Provider or model identity is missing.
- Quota state is unknown, exhausted, or policy-paused on the selected hosted route.
- Latency or cost envelope is missing or unverified.
- Identity-mode baseline ref is missing.
- Route-change lineage or route-selection disclosure is missing when a policy or fallback route changes selection.
- Boundary material such as raw URLs or credential tokens appears in exportable fields.

## Fixtures

Fixtures live in `fixtures/ai/routing_cost_alpha/`.

- `managed_hosted_chat_preview.json` shows the happy path for a hosted preview route.
- `policy_forced_enterprise_route_change.json` shows policy pinning a gateway route with visible lineage.
- `quota_exhausted_fallback_visible.json` shows hosted entitlement exhaustion selecting a local fallback while preserving local continuity.
- `manifest.json` indexes the cases and support-export guards.

The crate tests deserialize each packet, validate it, and assert the support projection remains export-safe.

## Lifecycle

The capability-lifecycle row is `capability_lifecycle:alpha.ai.routing_cost`. The hosted-route dependency marker keeps this path in preview posture and requires docs, diagnostics, settings, Help/About, and support exports to disclose that hosted routing is an optional preview dependency.

## Verification

Run:

```sh
cargo test -p aureline-ai routing --no-fail-fast
python3 ci/check_capability_lifecycle_registry.py --repo-root .
```
