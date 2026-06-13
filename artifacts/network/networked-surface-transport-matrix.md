# Networked-Surface Transport, Endpoint, Route, and Trust Matrix — Stable Packet

- Packet: `remote:networked_surface_transport_matrix:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_transport_matrix:v1`
- Schema: `schemas/network/networked_surface_transport_matrix.schema.json`
- Runtime owner: `aureline_remote::networked_surface_transport_matrix`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

This matrix is the canonical control source that newer network-capable
surfaces must share before each lane hardens its own proxy, trust, or mirror
semantics. It freezes one shared transport-governance vocabulary so no surface
ships a private proxy stack, a direct CA override, an undeclared public
fallback, or a hidden direct-connect retry.

## Surface coverage

| Surface | Origin | Endpoint | Egress | Route | Auth (handle-only) | Trust material | Mirror/offline | Policy epoch |
|---|---|---|---|---|---|---|---|---|
| `ai_gateway` | `managed_tenant` | `inference_gateway` | `managed_endpoint` | `direct` | `bearer_token_handle` | `managed_trust_bundle` | `local_core_only` | present |
| `docs_browser_fetcher` | `third_party` | `content_origin` | `public_internet` | `system_proxy` | `anonymous` | `system_trust_store` | `cached_offline` | present |
| `request_api_client` | `user_configured` | `rest_api` | `public_internet` | `manual_proxy` | `api_key_handle` | `system_trust_store` | `deny_all` | present |
| `database_cloud_connector` | `user_configured` | `data_store` | `public_internet` | `direct` | `client_certificate_handle` | `pinned_ca_handle` | `deny_all` | present |
| `registry_read` | `first_party` | `artifact_registry` | `mirror_only` | `mirror_first` | `anonymous` | `mirror_root_handle` | `mirror_first_then_deny` | present |
| `companion_handoff` | `loopback_local` | `peer_device` | `loopback_only` | `direct` | `session_cookie_handle` | `no_tls_loopback` | `local_core_only` | — |
| `provider_mutation` | `managed_tenant` | `vcs_host` | `managed_endpoint` | `direct` | `oauth_delegated_handle` | `managed_trust_bundle` | `deny_all` | present |
| `sync_offboarding` | `managed_tenant` | `sync_service` | `managed_endpoint` | `direct` | `bearer_token_handle` | `managed_trust_bundle` | `offline_grace` | present |
| `remote_preview_route` | `first_party` | `preview_origin` | `managed_endpoint` | `direct` | `bearer_token_handle` | `managed_trust_bundle` | `deny_all` | present |

## Key invariants verified

1. All nine required network-capable surfaces are covered by typed surface records.
2. No raw private material is present on any record (`raw_private_material_excluded: true`); only opaque handles, refs, and closed-vocabulary tokens cross the boundary.
3. No surface permits a silent fall-through to the public internet from a confined egress class (`no_silent_public_fallback: true`).
4. Every surface that defers actions for offline/replay restricts those queues to idempotent actions (`replay_idempotent_only`); provider mutations are not queued at all.
5. Every surface preserves local-core continuity (`local_core_continuity_preserved: true`).
6. Every surface declares trust material and a non-empty trust-proof ref so host proof is anchored to a named input.
7. Every surface declares a non-empty denial vocabulary so refusals quote the same token across UI, CLI, diagnostics, and exports.
8. Every surface carries fully-typed endpoint, egress, route, and auth classifications.
9. Every surface whose egress class requires a policy epoch (`public_internet`, `managed_endpoint`, `mirror_only`) carries a last-known-good `policy_epoch_ref`.
10. Every surface's qualification proof is fresh (or stale only within an accepted grace window).

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false` (narrow reason `raw_private_material_exposed`),
- a record with `no_silent_public_fallback: false` (narrow reason `silent_public_fallback_allowed`),
- a record that queues non-idempotent actions for offline replay (narrow reason `non_idempotent_replay_queued`).

A missing required surface narrows the packet to `preview`. A stale-beyond-window proof or any remaining condition gap narrows the affected row to `beta`, which lets release and support tooling detect and automatically narrow stale or under-qualified rows before publication.

## Drill coverage

| Drill | Surface mutated | Outcome |
|---|---|---|
| `drill_missing_surface_preview` | drops `ai_gateway` | `preview` |
| `drill_raw_material_withdrawn` | `request_api_client` exposes raw material | `withdrawn` |
| `drill_silent_public_fallback_withdrawn` | `registry_read` permits public fall-through | `withdrawn` |
| `drill_non_idempotent_replay_withdrawn` | `sync_offboarding` queues non-idempotent replay | `withdrawn` |
| `drill_stale_proof_beta` | `docs_browser_fetcher` proof expires | row `beta` |

Fixtures: `fixtures/network/networked_surface_transport_matrix/`.
