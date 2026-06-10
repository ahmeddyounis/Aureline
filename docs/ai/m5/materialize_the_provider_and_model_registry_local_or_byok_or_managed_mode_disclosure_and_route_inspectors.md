# Provider/model route disclosure and inspectors

This contract materializes the provider/model registry into one export-safe
disclosure packet whose unit of truth is a route inspector row. Shell, docs,
support export, and release tooling consume the packet directly instead of
re-describing route state by hand.

- Packet type: `aureline_ai::ProviderRouteDisclosurePacket`
- Schema: [`schemas/ai/materialize-the-provider-and-model-registry-local-or-byok-or-managed-mode-disclosure-and-route-inspectors.schema.json`](../../../schemas/ai/materialize-the-provider-and-model-registry-local-or-byok-or-managed-mode-disclosure-and-route-inspectors.schema.json)
- Support export: [`artifacts/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/support_export.json`](../../../artifacts/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/support_export.json)
- Fixtures: [`fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/`](../../../fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/)

## The route inspector row

Each `RouteInspectorRow` binds, for one claimed AI route:

| Field | Meaning |
| --- | --- |
| `provider_id`, `model_id` | Identity tokens (never a raw endpoint URL). |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `execution_mode` | The headline badge: `local`, `byok`, or `managed`. |
| `locality` | Where inference actually runs. |
| `region` | Region posture for the route's bytes. |
| `retention` | Retention posture at the provider. |
| `cost_disclosure` | How cost is disclosed (local-free, metered, flat, capped). |
| `tool_side_effect` | The strongest side-effect class the route's tools may have. |
| `automation_authority` | The apply authority the route may exercise. |
| `mode_disclosure_label` | The review-safe label shown alongside the badge. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `evidence_packet_refs` | Evidence backing a claimed route. |

## Invariants enforced by validation

- **Mode matches locality.** The `execution_mode` badge must equal the
  `locality`'s mode. A managed locality may never display a local badge.
- **Local means local.** A `local` route must be `on_device_only` for region,
  `no_retention_local_only` for retention, and `no_cost_local_compute` for cost.
- **No hidden trust posture.** A claimed managed or BYOK route may not leave its
  region, retention, or cost `unknown_unverified`.
- **No self-apply.** A route whose `tool_side_effect` mutates must carry an
  apply authority other than `read_only_no_apply`. There is no autonomous
  self-apply authority; every mutating authority requires a human in the loop.
- **Claimed routes carry evidence.** Stable, Beta, and Preview routes must list
  at least one evidence packet ref.
- **Narrow, never hide.** Every route carries a `proof_stale` downgrade rule and
  every downgrade rule must narrow strictly below the claimed qualification.

## Provenance and freshness

`source_contract_refs` must include this schema, this doc, the provider/model
registry schema this packet materializes, and the frozen M5 AI workflow matrix
schema whose qualification and downgrade vocabularies the packet reuses. The
`proof_freshness` block records the freshness SLO and asserts that stale proof
automatically narrows claimed routes. Reading the checked export through
`current_provider_route_disclosure_export` re-validates every invariant, so a
stale or malformed artifact fails the consuming surface rather than shipping an
optimistic claim.

## Boundary

The packet carries no credential bodies, raw provider payloads, raw endpoint
URLs, exact token counts, exact cost amounts, or raw diff bodies. Validation
rejects obvious credential material in the serialized export.
