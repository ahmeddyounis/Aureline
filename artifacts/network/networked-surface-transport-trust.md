# Networked-Surface Transport-Trust Governance — Stable Packet

- Packet: `remote:networked_surface_transport_trust:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_transport_trust:v1`
- Schema: `schemas/network/networked_surface_transport_trust.schema.json`
- Owner: `aureline_remote::networked_surface_transport_trust`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

This packet makes the trust inputs and host proof behind every network-capable
surface a first-class governed object. For each claimed surface it freezes the
trust-store source, the organization CA bundle / pin-set review state, the
SSH/TLS host-proof state and history depth, the client-certificate binding
posture, and the trust-root freshness and rotation cue — so no surface ships a
direct CA override or silently downgrades trust, and a missing or unverifiable
trust input is surfaced as a typed `deny_trust` reason or a typed host-proof
state rather than generic offline copy.

## Trust coverage

| Surface | Origin | Egress | Trust store | CA review | Host proof | Client cert | Trust root | Rotation cue | Outcome | Deny reason |
|---|---|---|---|---|---|---|---|---|---|---|
| `ai_gateway` | `managed_tenant` | `managed_endpoint` | `managed_org_bundle` | `org_reviewed` | `pinned_match` | `required_presented` | `fresh` | `none` | `trusted` | — |
| `docs_browser_fetcher` | `third_party` | `public_internet` | `system_trust_store` | `system_default` | `known_tofu` | `not_required` | `rotation_due` | `rotate_soon` | `trusted_rotation_due` | — |
| `request_api_client` | `user_configured` | `public_internet` | `pinned_ca_set` | `pinned_set` | `pinned_match` | `not_required` | `fresh` | `none` | `trusted` | — |
| `database_cloud_connector` | `user_configured` | `public_internet` | `pinned_ca_set` | `pinned_set` | `pinned_match` | `required_presented` | `fresh` | `none` | `trusted` | — |
| `registry_read` | `first_party` | `mirror_only` | `mirror_root` | `mirror_root` | `pinned_match` | `not_required` | `pinned_static` | `pinned_no_rotation` | `trusted` | — |
| `companion_handoff` | `loopback_local` | `loopback_only` | `no_tls_loopback` | `not_applicable` | `not_applicable` | `not_required` | `pinned_static` | `pinned_no_rotation` | `not_applicable_loopback` | — |
| `provider_mutation` | `managed_tenant` | `managed_endpoint` | `managed_org_bundle` | `org_reviewed` | `pinned_match` | `required_presented` | `rotation_in_progress` | `rotating` | `trusted` | — |
| `sync_offboarding` | `managed_tenant` | `managed_endpoint` | `managed_org_bundle` | `org_reviewed` | `known_tofu` | `managed_provisioned` | `fresh` | `none` | `trusted` | — |
| `remote_preview_route` | `first_party` | `managed_endpoint` | `pinned_ca_set` | `pinned_set` | `changed_mismatch` | `not_required` | `fresh` | `none` | `deny_trust` | `host_proof_changed` |

The seeded packet exercises every trust-store source, host-proof state
(pinned, trust-on-first-use, not-applicable loopback, and a changed-mismatch
deny), client-certificate posture (mutual TLS presented, managed-provisioned,
not-required), and trust-root freshness / rotation cue (fresh, rotation-due with
a `rotate_soon` cue, rotation-in-progress, and pinned-static), plus a typed
`deny_trust` outcome on `remote_preview_route` where the host proof changed.

## Key invariants verified

1. All nine required network-capable surfaces produced a typed trust-evaluation record.
2. No raw trust material is present on any record (`raw_trust_material_excluded: true`); every trust input is named by opaque handle.
3. No raw private-key material is present (`private_key_material_excluded: true`).
4. No record ships a direct CA override (`no_direct_ca_override: true`).
5. No record silently downgrades trust (`no_silent_trust_downgrade: true`).
6. Every record preserves local-core continuity (`local_core_continuity_preserved: true`).
7. The one `deny_trust` record (`remote_preview_route`) carries a typed `host_proof_changed` reason rather than silently accepting the new proof.
8. Every record exposes a typed host-proof state and a consistent set of trust inputs.
9. Every record whose trust root needs rotation (`docs_browser_fetcher`, `provider_mutation`) carries an active rotation cue.
10. Every record whose egress class requires a policy epoch (`public_internet`, `managed_endpoint`, `mirror_only`) carries a `policy_epoch_ref`; `companion_handoff` (`loopback_only`) is exempt.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_trust_material_excluded: false` (narrow reason `raw_trust_material_exposed`),
- a record with `private_key_material_excluded: false` (narrow reason `private_key_material_exposed`),
- a record with `no_direct_ca_override: false` or `ca_bundle.is_direct_ca_override: true` (narrow reason `direct_ca_override_shipped`),
- a record with `no_silent_trust_downgrade: false` (narrow reason `silent_trust_downgrade`).

A missing required surface narrows the packet to `preview`. A denial without a
reason, an incomplete trust-input classification, a missing rotation cue, or any
remaining condition gap narrows the affected row to `beta`, which lets release
and support tooling detect and automatically narrow under-qualified rows before
publication.

## Drill coverage

| Drill | Record mutated | Outcome |
|---|---|---|
| `drill_missing_surface_preview` | drops `ai_gateway` | `preview` |
| `drill_raw_material_withdrawn` | `request_api_client` exposes raw trust material | `withdrawn` |
| `drill_private_key_withdrawn` | `database_cloud_connector` exposes raw private-key material | `withdrawn` |
| `drill_ca_override_withdrawn` | `docs_browser_fetcher` ships a direct CA override | `withdrawn` |
| `drill_silent_downgrade_withdrawn` | `registry_read` silently downgrades trust | `withdrawn` |
| `drill_denied_no_reason_beta` | `sync_offboarding` denied without a reason | row `beta` |
| `drill_missing_rotation_cue_beta` | `docs_browser_fetcher` rotation-due with no active cue | row `beta` |

Fixtures: `fixtures/network/networked_surface_transport_trust/`.
