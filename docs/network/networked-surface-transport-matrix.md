# Networked-surface transport, endpoint, route, and trust matrix

This matrix freezes the object model and vocabulary that every newer
network-capable surface must share before those lanes harden their own proxy,
trust, or mirror semantics. The surfaces it governs are the AI inference
gateway, documentation and in-product browser fetchers, generic request/API
clients, database and cloud connectors, extension and model registry reads,
companion device handoffs, provider mutation lanes, sync and offboarding
traffic, and the richer remote preview routes.

Each surface resolves through **one** shared transport-governance vocabulary
instead of per-feature proxy or certificate folklore. The runtime owner is
`aureline_remote::networked_surface_transport_matrix`; the boundary schema is
`schemas/network/networked_surface_transport_matrix.schema.json`.

The packet does **not** re-derive raw endpoint URLs, raw hostnames, raw ports,
raw credentials, raw bearer or session tokens, raw cookie jars, raw private
certificate bytes, raw SSH private material, or raw PAC bodies. Every credential
and trust input is referenced by opaque handle only.

## What the matrix freezes per surface

- **Origin scope** — first-party, third-party, user-configured, managed-tenant, or loopback-local.
- **Endpoint class** — inference gateway, content origin, REST API, data store, artifact registry, peer device, VCS host, sync service, or preview origin.
- **Egress class** — public internet, managed endpoint, mirror-only, loopback-only, or air-gapped.
- **Route choice** — direct, system proxy, manual proxy, PAC-resolved, mirror-first, or offline. Proxy resolution precedence is PAC → manual → system.
- **Auth posture** — anonymous, or a handle to a bearer token, delegated OAuth credential, API key, client certificate, SSH key, or session cookie.
- **Trust material** — system trust store, pinned CA handle, managed trust bundle, mirror-root handle, SSH known-hosts handle, or no-TLS loopback.
- **Denial vocabulary** — the closed set of typed reasons the surface may refuse a request (policy blocked, egress class forbidden, trust proof missing/expired, auth posture rejected, proxy unreachable, mirror-root mismatch, offline no fallback, non-idempotent replay rejected).
- **Mirror/offline behavior** — mirror-first-then-deny, cached-offline, offline-grace, deny-all, or local-core-only.

## Contract

For the stable claim to hold, **all** of the following conditions must be
verified simultaneously for every covered surface:

1. **All required surfaces covered** — one surface record for each of:
   `ai_gateway`, `docs_browser_fetcher`, `request_api_client`,
   `database_cloud_connector`, `registry_read`, `companion_handoff`,
   `provider_mutation`, `sync_offboarding`, `remote_preview_route`.
2. **No raw private material** — every record carries
   `raw_private_material_excluded: true`.
3. **No silent public fall-through** — every record carries
   `no_silent_public_fallback: true`; mirror-only and deny-all profiles never
   silently reach the public internet.
4. **Idempotent-only replay** — any surface with
   `offline_deferral_allowed: true` also carries `replay_idempotent_only: true`.
5. **Local-core continuity preserved** — every record carries
   `local_core_continuity_preserved: true`.
6. **Trust material declared** — every record carries a typed
   `trust_material` and a non-empty `trust_proof_ref`.
7. **Denial vocabulary declared** — every record carries a non-empty
   `denial_vocabulary`.
8. **Transport classification complete** — every record carries typed
   `endpoint_class`, `egress_class`, `route_choice`, and `auth_posture`.
9. **Policy epoch traceable** — every record whose egress class requires it
   (`public_internet`, `managed_endpoint`, `mirror_only`) carries a
   `policy_epoch_ref`.
10. **Proof fresh** — every record's `proof_freshness` is `fresh` or
    `stale_within_window`.

## Narrowing

The qualification tier is derived, never asserted:

- **Withdrawn (hard, non-overridable):** `raw_private_material_exposed`,
  `silent_public_fallback_allowed`, or `non_idempotent_replay_queued`.
- **Preview:** `required_surface_missing` — a coverage gap prevents any
  verifiable claim for the missing surface.
- **Beta:** `local_core_continuity_not_preserved`, `trust_material_undeclared`,
  `denial_vocabulary_missing`, `transport_classification_incomplete`,
  `policy_epoch_ref_missing`, or `proof_stale_beyond_window`.

Because stale proofs narrow to beta, release and support tooling that ingests
this packet can detect stale or missing transport rows and automatically narrow
the affected network claims before publication. The matrix is bound into the
canonical evidence index at `artifacts/release/m5/xt12-evidence-index.md`.

## Consuming the packet

Dashboards, Help/About surfaces, CLI/headless output, diagnostics, support
exports, and release tooling should ingest the
`NetworkedSurfaceTransportMatrixPage` (and its
`NetworkedSurfaceMatrixSupportExport` envelope) rather than cloning
subsystem-specific status strings. The packet, its rows, summary, defects, and
support export are emitted by the headless example
`dump_networked_surface_transport_matrix_fixtures` and pinned as fixtures under
`fixtures/network/networked_surface_transport_matrix/`.
