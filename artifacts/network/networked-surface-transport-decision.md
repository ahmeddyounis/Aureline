# Networked-Surface Transport-Decision Log — Stable Packet

- Packet: `remote:networked_surface_transport_decision:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_transport_decision:v1`
- Schema: `schemas/network/networked_surface_transport_decision.schema.json`
- Runtime owner: `aureline_remote::networked_surface_transport_decision`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

This decision log is the runtime layer that pairs with the frozen
networked-surface transport matrix. The matrix freezes the per-surface
transport vocabulary; this log proves that every claimed network-capable action
resolves through that one shared transport object model — emitting one
inspectable transport decision (endpoint descriptor + transport-policy snapshot
+ typed outcome) before any side effects leave the current boundary — so no
surface ships a private proxy stack, a direct CA override, an undeclared public
fallback, or a hidden direct-connect retry.

## Decision coverage

| Surface | Origin | Endpoint | Egress | Route | Proxy tier | Auth (handle-only) | Trust material | Outcome | Policy epoch |
|---|---|---|---|---|---|---|---|---|---|
| `ai_gateway` | `managed_tenant` | `inference_gateway` | `managed_endpoint` | `direct` | `direct_no_proxy` | `bearer_token_handle` | `managed_trust_bundle` | `allowed` | present |
| `docs_browser_fetcher` | `third_party` | `content_origin` | `public_internet` | `system_proxy` | `system_proxy` | `anonymous` | `system_trust_store` | `allowed` | present |
| `request_api_client` | `user_configured` | `rest_api` | `public_internet` | `manual_proxy` | `manual_proxy` | `api_key_handle` | `system_trust_store` | `allowed` | present |
| `database_cloud_connector` | `user_configured` | `data_store` | `public_internet` | `direct` | `direct_no_proxy` | `client_certificate_handle` | `pinned_ca_handle` | `allowed` | present |
| `registry_read` | `first_party` | `artifact_registry` | `mirror_only` | `mirror_first` | `mirror_route` | `anonymous` | `mirror_root_handle` | `served_from_mirror` | present |
| `companion_handoff` | `loopback_local` | `peer_device` | `loopback_only` | `direct` | `direct_no_proxy` | `session_cookie_handle` | `no_tls_loopback` | `allowed` | — |
| `provider_mutation` | `managed_tenant` | `vcs_host` | `managed_endpoint` | `direct` | `direct_no_proxy` | `oauth_delegated_handle` | `managed_trust_bundle` | `allowed` | present |
| `sync_offboarding` | `managed_tenant` | `sync_service` | `managed_endpoint` | `direct` | `direct_no_proxy` | `bearer_token_handle` | `managed_trust_bundle` | `offline_deferred` | present |
| `remote_preview_route` | `first_party` | `preview_origin` | `managed_endpoint` | `direct` | `direct_no_proxy` | `bearer_token_handle` | `managed_trust_bundle` | `allowed` | present |

Proxy resolution precedence is PAC → manual → system; the recorded
`proxy_resolution_source` names the tier that actually selected each route.

## Key invariants verified

1. All nine required network-capable surfaces produced a typed transport decision.
2. No raw private material is present on any record (`raw_private_material_excluded: true`); endpoints are named by opaque handle and credentials/trust by opaque ref.
3. Every decision resolved through the shared transport-governance layer (`no_bypass: true`).
4. No decision permits a silent fall-through to the public internet from a confined egress class (`no_silent_public_fallback: true`).
5. The only `offline_deferred` decision (`sync_offboarding`) queues an idempotent action; the non-idempotent `provider_mutation` is allowed inline and never queued.
6. Every decision preserves local-core continuity (`local_core_continuity_preserved: true`).
7. Every denied decision would carry a typed denial reason (the stable packet denies nothing).
8. Every decision carries a non-empty trust-proof ref so host proof is anchored to a named input.
9. Every decision whose egress class requires a policy epoch (`public_internet`, `managed_endpoint`, `mirror_only`) carries a last-known-good `policy_epoch_ref`.
10. Every decision's trust proof is fresh (or stale only within an accepted grace window).
11. Every decision's route choice agrees with its proxy-resolution tier and carries a typed endpoint, outcome, and auth posture.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false` (narrow reason `raw_private_material_exposed`),
- a decision with `no_bypass: false` (narrow reason `bypassed_shared_governance`),
- a decision whose policy has `no_silent_public_fallback: false` (narrow reason `silent_public_fallback_resolved`),
- an `offline_deferred` decision that queues a non-idempotent action (narrow reason `non_idempotent_replay_queued`).

A missing required surface narrows the packet to `preview`. A stale-beyond-window proof, a denial without a reason, or any remaining condition gap narrows the affected row to `beta`, which lets release and support tooling detect and automatically narrow stale or under-qualified rows before publication.

## Drill coverage

| Drill | Decision mutated | Outcome |
|---|---|---|
| `drill_missing_surface_preview` | drops `ai_gateway` | `preview` |
| `drill_raw_material_withdrawn` | `request_api_client` exposes raw material | `withdrawn` |
| `drill_bypass_withdrawn` | `database_cloud_connector` sets `no_bypass: false` | `withdrawn` |
| `drill_silent_public_fallback_withdrawn` | `registry_read` permits public fall-through | `withdrawn` |
| `drill_non_idempotent_replay_withdrawn` | `sync_offboarding` defers a non-idempotent action | `withdrawn` |
| `drill_denied_no_reason_beta` | `provider_mutation` denied without a reason | row `beta` |
| `drill_stale_proof_beta` | `docs_browser_fetcher` trust proof expires | row `beta` |

Fixtures: `fixtures/network/networked_surface_transport_decision/`.
