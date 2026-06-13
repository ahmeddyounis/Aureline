# Networked-surface transport-decision log

This decision log is the **runtime layer** that pairs with the frozen
[networked-surface transport matrix](./networked-surface-transport-matrix.md).
The matrix freezes the per-surface transport vocabulary; this log proves that
every new network-capable action a surface takes actually *resolves through*
that shared object model before any side effects leave the current boundary.

For every action, the shared transport-governance layer emits **one**
inspectable transport decision. The surfaces it governs are the AI inference
gateway, documentation and in-product browser fetchers, generic request/API
clients, database and cloud connectors, extension and model registry reads,
companion device handoffs, provider mutation lanes, sync and offboarding
traffic, and the richer remote preview routes.

The runtime owner is
`aureline_remote::networked_surface_transport_decision`; the boundary schema is
`schemas/network/networked_surface_transport_decision.schema.json`.

The packet does **not** re-derive raw endpoint URLs, raw hostnames, raw ports,
raw credentials, raw bearer or session tokens, raw cookie jars, raw private
certificate bytes, raw SSH private material, or raw PAC bodies. Endpoints are
named by opaque handle; every credential and trust input is referenced by
opaque handle or ref only.

## What one decision records

- **Endpoint descriptor** — the surface, origin scope, endpoint class, and an
  opaque endpoint handle, so the contacted endpoint is inspectable without
  reconstructing it from a raw URL.
- **Transport-policy snapshot** — the egress class enforced, the route choice,
  and the proxy-resolution tier that selected it. Proxy resolution precedence is
  PAC → manual → system; the winning tier is recorded as
  `proxy_resolution_source` (`direct_no_proxy`, `pac_resolved`, `manual_proxy`,
  `system_proxy`, `mirror_route`, or `offline_no_route`) alongside the trust
  material, mirror/offline behavior, and last-known-good policy epoch ref.
- **Auth posture** — anonymous, or a handle to a bearer token, delegated OAuth
  credential, API key, client certificate, SSH key, or session cookie.
- **Outcome** — `allowed`, `denied` (with a typed denial reason),
  `served_from_mirror`, `served_from_cache`, `offline_deferred`, or
  `offline_unavailable`.
- **`no_bypass`** — affirms the action resolved through the shared
  transport-governance layer and did not ship a private proxy stack, direct CA
  override, undeclared public fallback, or hidden direct-connect retry.

## Contract

For the stable claim to hold, **all** of the following conditions must be
verified simultaneously for every covered decision:

1. **All required surfaces decided** — one decision for each of: `ai_gateway`,
   `docs_browser_fetcher`, `request_api_client`, `database_cloud_connector`,
   `registry_read`, `companion_handoff`, `provider_mutation`,
   `sync_offboarding`, `remote_preview_route`.
2. **No raw private material** — every record carries
   `raw_private_material_excluded: true`.
3. **No governance bypass** — every decision carries `no_bypass: true`.
4. **No silent public fall-through** — every decision's policy carries
   `no_silent_public_fallback: true`.
5. **Idempotent-only replay** — any `offline_deferred` decision carries
   `action_is_idempotent: true`.
6. **Local-core continuity preserved** — every decision's policy carries
   `local_core_continuity_preserved: true`.
7. **Denials are typed** — every `denied` decision carries a typed
   `denial_reason`.
8. **Trust proof present** — every decision's policy carries a non-empty
   `trust_proof_ref`.
9. **Policy epoch traceable** — every decision whose egress class requires it
   (`public_internet`, `managed_endpoint`, `mirror_only`) carries a
   `policy_epoch_ref`.
10. **Proof fresh** — every decision's `trust_proof_freshness` is `fresh` or
    `stale_within_window`.
11. **Classification complete** — every decision carries a fully-classified
    endpoint, a route choice consistent with its proxy-resolution tier, and a
    typed outcome and auth posture.

## Narrowing

The qualification tier is derived, never asserted:

- **Withdrawn (hard, non-overridable):** `raw_private_material_exposed`,
  `bypassed_shared_governance`, `silent_public_fallback_resolved`, or
  `non_idempotent_replay_queued`.
- **Preview:** `required_surface_missing` — a coverage gap prevents any
  verifiable claim for the missing surface.
- **Beta:** `denial_reason_missing`, `local_core_continuity_not_preserved`,
  `trust_proof_missing`, `policy_epoch_ref_missing`,
  `transport_classification_incomplete`, or `proof_stale_beyond_window`.

Because stale proofs and under-qualified rows narrow to beta, release and
support tooling that ingests this packet can detect them and automatically
narrow the affected network claims before publication. The log is bound into
the canonical evidence index at `artifacts/release/m5/xt12-evidence-index.md`.

## Consuming the packet

Dashboards, Help/About surfaces, CLI/headless output, diagnostics, support
exports, and release tooling should ingest the `TransportDecisionLogPage` (and
its `TransportDecisionSupportExport` envelope) rather than reconstructing route
choices and endpoints from raw URLs or logs. The packet, its rows, summary,
defects, and support export are emitted by the headless example
`dump_networked_surface_transport_decision_fixtures` and pinned as fixtures
under `fixtures/network/networked_surface_transport_decision/`.
