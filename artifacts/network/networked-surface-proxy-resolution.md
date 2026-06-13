# Networked-Surface Proxy-Resolution Governance — Stable Packet

- Packet: `remote:networked_surface_proxy_resolution:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_proxy_resolution:v1`
- Schema: `schemas/network/networked_surface_proxy_resolution.schema.json`
- Owner: `aureline_remote::networked_surface_proxy_resolution`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

This packet makes proxy resolution a first-class governed object. For every
claimed network-capable surface it freezes the ordered candidate chain
resolution walked (PAC → manual → environment → system → declared direct, plus
the out-of-ladder mirror-pinned and offline tiers), the winning tier, and a
typed `deny_proxy_resolution` reason when no tier may be honored — so no surface
ships a private proxy stack, a direct CA override, or a hidden direct-connect
fallback, and a contradictory or unresolvable proxy state is surfaced as a
labeled degraded or denied record rather than a silent public-direct fallback.

## Resolution coverage

| Surface | Origin | Egress | Candidate chain (precedence) | Selected tier | Outcome | Deny reason | Policy epoch |
|---|---|---|---|---|---|---|---|
| `ai_gateway` | `managed_tenant` | `managed_endpoint` | `manual_pinned > system_proxy > direct_no_proxy` | `manual_pinned` | `resolved` | — | present |
| `docs_browser_fetcher` | `third_party` | `public_internet` | `system_proxy > direct_no_proxy` | `system_proxy` | `resolved` | — | present |
| `request_api_client` | `user_configured` | `public_internet` | `environment_proxy > system_proxy > direct_no_proxy` | `environment_proxy` | `resolved` | — | present |
| `database_cloud_connector` | `user_configured` | `public_internet` | `direct_no_proxy` | `direct_no_proxy` | `resolved` | — | present |
| `registry_read` | `first_party` | `mirror_only` | `mirror_pinned` | `mirror_pinned` | `mirror_pinned_no_proxy` | — | present |
| `companion_handoff` | `loopback_local` | `loopback_only` | `direct_no_proxy` | `direct_no_proxy` | `resolved` | — | — |
| `provider_mutation` | `managed_tenant` | `managed_endpoint` | `pac_script > manual_pinned > system_proxy` | `pac_script` | `resolved` | — | present |
| `sync_offboarding` | `managed_tenant` | `managed_endpoint` | `system_proxy > direct_no_proxy` | `system_proxy` | `resolved` | — | present |
| `remote_preview_route` | `first_party` | `managed_endpoint` | `manual_pinned > environment_proxy` | — | `denied_proxy_resolution` | `contradictory_proxy_state` | present |

The seeded packet exercises every in-ladder tier (PAC, manual, environment,
system, direct), the out-of-ladder mirror-pinned tier, and a typed
`deny_proxy_resolution` outcome on `remote_preview_route` where a manual proxy
and an environment proxy disagree.

## Key invariants verified

1. All nine required network-capable surfaces produced a typed proxy-resolution record.
2. No raw private material is present on any record (`raw_private_material_excluded: true`); every proxy source is named by opaque handle.
3. No record ships a private/undeclared proxy stack (`no_private_proxy_stack: true`).
4. No record ships a direct CA override (`no_direct_ca_override: true`).
5. No record permits a silent direct-to-public fallback (`no_silent_direct_fallback: true`).
6. Every record preserves local-core continuity (`local_core_continuity_preserved: true`).
7. The one `denied_proxy_resolution` record (`remote_preview_route`) carries a typed `contradictory_proxy_state` reason rather than silently direct-connecting.
8. Every record's selected tier is the highest-precedence available candidate (`precedence_respected`).
9. Every record whose egress class requires a policy epoch (`public_internet`, `managed_endpoint`, `mirror_only`) carries a `policy_epoch_ref`; `companion_handoff` (`loopback_only`) is exempt.
10. Every record carries a non-empty candidate chain, a typed outcome, and a consistent selected tier.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false` (narrow reason `raw_private_material_exposed`),
- a record with `no_private_proxy_stack: false` (narrow reason `private_proxy_stack_shipped`),
- a record with `no_direct_ca_override: false` (narrow reason `direct_ca_override_shipped`),
- a record with `no_silent_direct_fallback: false` (narrow reason `silent_direct_fallback_resolved`).

A missing required surface narrows the packet to `preview`. A denial without a
reason, a precedence violation, or any remaining condition gap narrows the
affected row to `beta`, which lets release and support tooling detect and
automatically narrow under-qualified rows before publication.

## Drill coverage

| Drill | Record mutated | Outcome |
|---|---|---|
| `drill_missing_surface_preview` | drops `ai_gateway` | `preview` |
| `drill_raw_material_withdrawn` | `request_api_client` exposes raw material | `withdrawn` |
| `drill_private_stack_withdrawn` | `docs_browser_fetcher` adds a private proxy stack | `withdrawn` |
| `drill_ca_override_withdrawn` | `database_cloud_connector` sets `no_direct_ca_override: false` | `withdrawn` |
| `drill_silent_fallback_withdrawn` | `registry_read` permits a silent direct fallback | `withdrawn` |
| `drill_denied_no_reason_beta` | `sync_offboarding` denied without a reason | row `beta` |
| `drill_precedence_not_respected_beta` | `request_api_client` selects the lower system proxy | row `beta` |

Fixtures: `fixtures/network/networked_surface_proxy_resolution/`.
