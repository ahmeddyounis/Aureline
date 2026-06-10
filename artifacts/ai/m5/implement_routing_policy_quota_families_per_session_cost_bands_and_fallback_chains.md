# Routing Policy, Quota Families, Per-Session Cost Bands, And Fallback Chains

- Packet: `routing-policy:stable:0001`
- Schema: `schemas/ai/implement-routing-policy-quota-families-per-session-cost-bands-and-fallback-chains.schema.json`
- Support export: `artifacts/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/support_export.json`
- Fixture: `fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/`

## Coverage

The packet materializes the route-governance surface into one row per governed
surface. Every surface carries the provider/locality mode its routing policy
resolves to, the quota family that rations it, the per-session cost band that
prices it, and the ordered fallback chain that keeps it reachable.

- The composer surface resolves to the managed mode at Stable: a per-session
  managed entitlement quota within limit, a flat-fee subscription band charged to
  the subscriber, a selected managed hop with a local fallback and a non-AI
  terminal hop, and a fully-reversible verified rollback.
- The review surface resolves to BYOK at Beta: a per-session vendor quota in
  warning, a metered medium band charged to the BYOK owner, a selected BYOK hop
  with an on-device budget-exhaustion fallback, and a checkpoint-reversible
  verified rollback.
- The explain surface resolves to local at Preview: an unmetered local quota, a
  bundled no-charge band, and a selected on-device hop with a non-AI terminal
  fallback.
- The background-agent surface had its per-session vendor quota exhausted: its
  primary BYOK hop is skipped, an on-device fallback is available, it dropped out
  of every claimed lane to `held`, carries no evidence refs, and narrows to
  `unavailable` on stale proof or provider unavailability.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The packet refuses to present a routing policy greener than its cost, quota, and
continuity posture can back. Every fallback chain stays strictly ordered and ends
in a non-AI terminal fallback reachable without any model; a claimed surface
resolves to exactly one selected hop whose mode matches the resolved mode; a
metered or subscription band must disclose who is charged; an estimate-only band
may not back a Stable claim; and an exhausted quota or per-session budget narrows
the claim instead of keeping an optimistic posture. Every surface narrows rather
than hides through the `proof_stale` and `provider_unavailable` triggers, reusing
the frozen M5 AI workflow matrix qualification, downgrade, and rollback-posture
vocabularies so no policy row may stay greener than its evidence. Raw provider
endpoints, credential bodies, raw provider payloads, and exact spend values never
cross the support boundary.
