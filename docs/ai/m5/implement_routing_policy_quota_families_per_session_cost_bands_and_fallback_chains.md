# Routing policy, quota families, per-session cost bands, and fallback chains

This contract materializes the route-governance surface into one export-safe
truth packet whose unit of truth is a routing-policy row. Shell, docs, support
export, and release tooling consume the packet directly instead of re-describing
quota, per-session cost, or fallback state by hand.

- Packet type: `aureline_ai::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::RoutingPolicyPacket`
- Schema: [`schemas/ai/implement-routing-policy-quota-families-per-session-cost-bands-and-fallback-chains.schema.json`](../../../schemas/ai/implement-routing-policy-quota-families-per-session-cost-bands-and-fallback-chains.schema.json)
- Support export: [`artifacts/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/support_export.json`](../../../artifacts/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/support_export.json)
- Fixtures: [`fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/`](../../../fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/)

## The routing-policy row

Each `RoutingPolicyRow` binds, for one governed surface:

| Field | Meaning |
| --- | --- |
| `surface_id`, `surface_label` | Identity token and label for the governed surface. |
| `resolved_mode` | Local, BYOK, managed, or enterprise-gateway mode the policy resolves to. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `quota` | Quota family, state, scope, and budget owner (no raw account id). |
| `session_cost_band` | Per-session cost band, measurement, charge owner, and exhaustion flag. |
| `fallback_chain` | Ordered hops, each with a mode, reason, and outcome, ending in a non-AI terminal fallback. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed surface. |

## Invariants enforced by validation

- **Fallback chains stay ordered.** Hop orders must start at zero and strictly
  increase, so the chain reads as an unambiguous sequence.
- **Every chain ends in a non-AI fallback.** A surface must carry a
  `non_ai_terminal_fallback` hop reachable without any model, so the surface is
  never stranded when AI routes run out — preserving managed and offline
  continuity.
- **One selected hop, matching the mode.** A claimed surface must resolve to
  exactly one `selected` hop, and that hop's mode must equal `resolved_mode`, so
  the disclosed route is the route that ran.
- **Charged bands disclose who pays.** A metered or subscription band must carry
  a disclosed charge owner; the charge is never left `charge_unknown_unverified`.
- **Estimates do not back Stable.** A surface whose per-session band is
  `estimated_unverified_band` or whose measurement is `estimate_band` may not
  claim Stable.
- **Exhaustion narrows, never hides.** A surface whose quota is exhausted or
  paused, or whose per-session budget is spent, may not keep claiming Stable.
- **Claimed surfaces carry evidence and a verified reversal.** Stable, Beta, and
  Preview surfaces must list at least one evidence packet ref, and any reversing
  rollback posture must be `verified`.
- **Narrow, never hide.** Every surface carries the `proof_stale` and
  `provider_unavailable` downgrade triggers, and every rule must narrow strictly
  below the claimed qualification.

## Provenance and freshness

`source_contract_refs` must include this schema, this doc, the provider/model
registry schema, and the frozen M5 AI workflow matrix schema whose qualification,
downgrade, and rollback-posture vocabularies the packet reuses. The
`proof_freshness` block records the freshness SLO and asserts that stale proof
automatically narrows claimed surfaces. Reading the checked export through
`current_routing_policy_export` re-validates every invariant, so a stale or
malformed artifact fails the consuming surface rather than shipping an optimistic
claim.

## Boundary

The packet carries no provider endpoints, credential bodies, raw provider
payloads, or exact spend values. Cost is disclosed as coarse, review-safe bands
and charge owners only. Validation rejects obvious credential material and raw
URLs in the serialized export.
