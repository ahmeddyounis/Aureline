# Networked-surface transport-trust governance

This packet makes the **trust inputs and host proof** behind every
network-capable surface a first-class governed object. The sibling
[transport matrix](./networked-surface-transport-matrix.md) freezes which trust
material *class* a surface may use, and the
[transport-decision log](./networked-surface-transport-decision.md) emits one
decision per action; this packet freezes, per surface, the trust-store source,
the organization CA bundle / pin-set review state, the SSH/TLS host-proof state
and history depth, the client-certificate binding posture, and the trust-root
freshness and rotation cue — so a missing or stale trust input surfaces as a
typed `deny_trust` reason or a typed host-proof state rather than generic
offline copy, and no M5 client, helper, or extension can ship a direct CA
override or silently downgrade trust.

The surfaces it governs are the AI inference gateway, documentation and
in-product browser fetchers, generic request/API clients, database and cloud
connectors, extension and model registry reads, companion device handoffs,
provider mutation lanes, sync and offboarding traffic, and the richer remote
preview routes.

The owner is `aureline_remote::networked_surface_transport_trust`; the boundary
schema is `schemas/network/networked_surface_transport_trust.schema.json`.

The packet does **not** re-derive raw CA bundles, raw certificate bytes, raw
private keys, raw SSH host or private keys, raw credentials, or raw
bearer/session tokens. Every trust input is named by opaque handle only.

## What one record holds

- **Trust-store / CA bundle** (`ca_bundle`) — the trust-store source
  (`system_trust_store`, `pinned_ca_set`, `managed_org_bundle`, `mirror_root`,
  `ssh_known_hosts`, `no_tls_loopback`), the CA bundle / pin-set review state
  (`system_default`, `org_reviewed`, `pinned_set`, `mirror_root`,
  `not_applicable`), the pin count, and an opaque bundle handle. A reviewed
  organization bundle (`org_reviewed`) is distinguishable from a bare system
  default.
- **Host proof** (`host_proof`) — the typed host-proof state (`pinned_match`,
  `known_tofu`, `first_use_pending`, `changed_mismatch`, `revoked`,
  `not_applicable`), an opaque proof handle, and the host-proof history depth.
- **Client certificate** (`client_cert`) — the binding posture (`not_required`,
  `optional_presented`, `required_presented`, `managed_provisioned`,
  `required_absent`) and an opaque binding handle.
- **Trust root** (`trust_root`) — the freshness (`fresh`, `rotation_due`,
  `rotation_in_progress`, `expired`, `pinned_static`) and the rotation cue
  (`none`, `rotate_soon`, `rotating`, `rotate_now`, `pinned_no_rotation`).
- **Outcome** — `trusted`, `trusted_rotation_due`, `host_proof_pending`,
  `deny_trust` (with a typed reason), or `not_applicable_loopback`.
- **`deny_trust` reason** — `trust_store_unavailable`, `ca_bundle_missing`,
  `ca_bundle_stale`, `managed_bundle_unverified`, `host_proof_missing`,
  `host_proof_changed`, `host_proof_revoked`, `client_cert_required_absent`,
  `trust_root_expired`, `pin_set_mismatch`, or `mirror_root_mismatch`. A missing
  or unverifiable trust input is surfaced as one of these typed reasons rather
  than a silent fallback to an untrusted root.
- **Guardrail flags** — `no_direct_ca_override` and `no_silent_trust_downgrade`.

## Contract

For the stable claim to hold, **all** of the following must be verified
simultaneously for every covered record:

1. **All required surfaces evaluated** — one record for each of: `ai_gateway`,
   `docs_browser_fetcher`, `request_api_client`, `database_cloud_connector`,
   `registry_read`, `companion_handoff`, `provider_mutation`,
   `sync_offboarding`, `remote_preview_route`.
2. **No raw trust material** — every record carries
   `raw_trust_material_excluded: true`.
3. **No raw private-key material** — every record carries
   `private_key_material_excluded: true`.
4. **No direct CA override** — every record carries
   `no_direct_ca_override: true` (and `ca_bundle.is_direct_ca_override: false`).
5. **No silent trust downgrade** — every record carries
   `no_silent_trust_downgrade: true`.
6. **Local-core continuity preserved** — every record carries
   `local_core_continuity_preserved: true`.
7. **Denials are typed** — every `deny_trust` record carries a typed
   `denial_reason`.
8. **Trust inputs complete** — every record exposes a typed host-proof state and
   a consistent set of trust inputs; a trusted outcome may not sit on top of a
   host proof in a deny state.
9. **Rotation cues present** — every record whose trust root needs rotation
   (`rotation_due`, `rotation_in_progress`, `expired`) carries an active cue
   (`rotate_soon`, `rotating`, `rotate_now`).
10. **Policy epoch traceable** — every record whose egress class requires it
    (`public_internet`, `managed_endpoint`, `mirror_only`) carries a
    `policy_epoch_ref`.

## Narrowing

The qualification tier is derived, never asserted:

- **Withdrawn (hard, non-overridable):** `raw_trust_material_exposed`,
  `private_key_material_exposed`, `direct_ca_override_shipped`, or
  `silent_trust_downgrade`.
- **Preview:** `required_surface_missing` — a coverage gap prevents any
  verifiable claim for the missing surface.
- **Beta:** `deny_reason_missing`, `trust_input_classification_incomplete`,
  `rotation_cue_missing`, `local_core_continuity_not_preserved`, or
  `policy_epoch_ref_missing`.

Because under-qualified rows narrow to beta, release and support tooling that
ingests this packet can detect them and automatically narrow the affected trust
claims before publication. The packet is bound into the canonical evidence index
at `artifacts/release/m5/xt12-evidence-index.md`.

## Consuming the packet

Dashboards, Help/About surfaces, CLI/headless output, diagnostics, support
exports, and release tooling should ingest the `TransportTrustPage` (and its
`TrustSupportExport` envelope, or the `render_cli_view` rendering for terminal
parity) rather than reconstructing trust from raw certificate material. The
packet, its rows, summary, defects, support export, and CLI view are emitted by
the headless example `dump_networked_surface_transport_trust_fixtures` and
pinned as fixtures under `fixtures/network/networked_surface_transport_trust/`.
