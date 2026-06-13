# Networked-Surface Transport-Explainability — Stable Packet

- Packet: `remote:networked_surface_transport_explainability:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_transport_explainability:v1`
- Schema: `schemas/network/networked_surface_transport_explainability.schema.json`
- Runtime owner: `aureline_remote::networked_surface_transport_explainability`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

This explainability packet is the product-grade layer over the networked-surface
transport-decision log. It projects the decision stream into three views —
current transport-posture inspectors, a recent network-event ledger, and
per-action explain sheets — so users, admins, CLI/headless callers, diagnostics,
and support exports all see the effective proxy mode, trust source, mirror/
offline state, recent allow/deny history, and per-action denial or route
explanation in one shared vocabulary.

## Posture coverage

| Surface | Effective proxy mode | Trust source | Mirror/offline state | Disposition |
|---|---|---|---|---|
| `ai_gateway` | `direct_no_proxy` | `managed_trust_bundle` | `local_core_only` | `allowed` |
| `docs_browser_fetcher` | `system_proxy` | `system_trust_store` | `cached_offline` | `allowed` |
| `request_api_client` | `manual_proxy` | `system_trust_store` | `deny_all` | `allowed` |
| `database_cloud_connector` | `direct_no_proxy` | `pinned_ca_handle` | `deny_all` | `allowed` |
| `registry_read` | `mirror_route` | `mirror_root_handle` | `mirror_first_then_deny` | `served_without_egress` |
| `companion_handoff` | `direct_no_proxy` | `no_tls_loopback` | `local_core_only` | `allowed` |
| `provider_mutation` | `direct_no_proxy` | `managed_trust_bundle` | `deny_all` | `allowed` |
| `sync_offboarding` | `direct_no_proxy` | `managed_trust_bundle` | `offline_grace` | `deferred` |
| `remote_preview_route` | `direct_no_proxy` | `managed_trust_bundle` | `deny_all` | `allowed` |

Proxy resolution precedence is PAC → manual → system; `effective_proxy_mode`
names the tier that actually selected each route.

## Stable field catalog (product / CLI / support parity)

Every per-action explain sheet renders these fields, in this order, on every
surface, so the decision codes and field names match across product UI,
CLI/headless output, and support exports:

`surface`, `origin_scope`, `endpoint_class`, `egress_class`, `route_choice`,
`proxy_resolution_source`, `auth_posture`, `trust_material`,
`mirror_offline_behavior`, `outcome`, `denial_reason`.

## Recent network-event ledger filters

The ledger is filterable without exposing raw secrets or payloads by:

- endpoint class (`filter_by_endpoint_class`),
- origin scope (`filter_by_origin_scope`),
- allow/deny disposition (`filter_by_disposition`, `allowed_events`,
  `denied_events`).

Dispositions: `allowed`, `denied`, `served_without_egress`, `deferred`,
`unavailable`.

## Key invariants verified

1. All nine required network-capable surfaces have a posture inspector, a ledger event, and an explain sheet.
2. No raw private material is present on any record (`raw_private_material_excluded: true`); endpoints are named by opaque handle and credentials/trust by opaque ref.
3. Every projected decision resolved through the shared transport-governance layer (`no_bypass: true`).
4. No decision permits a silent fall-through to the public internet from a confined egress class (`no_silent_public_fallback: true`).
5. The only `deferred` event (`sync_offboarding`) queues an idempotent action; the non-idempotent `provider_mutation` is allowed inline and never queued.
6. Every posture inspector preserves local-core continuity (`local_core_continuity_preserved: true`).
7. Every denied event carries a typed denial explanation (the stable packet denies nothing).
8. Every posture inspector carries a non-empty trust-proof ref.
9. Every decision's trust proof is fresh (or stale only within an accepted grace window).
10. Every explain sheet renders at field-catalog parity (`explain_fields_at_parity: true`).

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false` (narrow reason `raw_private_material_exposed`),
- a decision with `no_bypass: false` (narrow reason `bypassed_shared_governance`),
- a policy with `no_silent_public_fallback: false` (narrow reason `silent_public_fallback_resolved`),
- an offline-deferred decision that queues a non-idempotent action (narrow reason `non_idempotent_replay_queued`).

A missing required surface narrows the packet to `preview`. A denial without a reason, a stale-beyond-window proof, a broken explain-field parity, or any remaining condition gap narrows the affected row to `beta`, which lets release and support tooling detect and automatically narrow stale or under-qualified rows before publication.

## Drill coverage

| Drill | Decision mutated | Outcome |
|---|---|---|
| `drill_missing_surface_preview` | drops `ai_gateway` | `preview` |
| `drill_raw_material_withdrawn` | `database_cloud_connector` exposes raw material | `withdrawn` |
| `drill_bypass_withdrawn` | `provider_mutation` sets `no_bypass: false` | `withdrawn` |
| `drill_denied_no_reason_beta` | `provider_mutation` denied without a reason | row `beta` |
| `drill_stale_proof_beta` | `docs_browser_fetcher` trust proof expires | row `beta` |

Fixtures: `fixtures/network/networked_surface_transport_explainability/`.
