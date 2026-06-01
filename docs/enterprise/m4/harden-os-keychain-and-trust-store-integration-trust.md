# Hardened OS keychain and trust-store integration — layered model, change detection, and repair-safe copy

This lane extends trust-store handling to a stable layered model that
distinguishes OS roots, custom CA bundles, pinned SSH host proofs,
client-certificate material, and imported mirror trust roots. Every
trust-store change emits an attributable, repair-safe event instead of
silently changing route posture under active sessions. The runtime owner is
`aureline_policy::harden_os_keychain_and_trust_store_integration_trust`.

For any trust material change, the lane can explain:

1. **Which layer changed** — via a closed `TrustStoreLayerClass` vocabulary
   naming the exact layer tier and its governance source.
2. **Which requests or sessions are affected** — via per-event opaque route
   refs and typed `SessionImpactClass` declarations; no raw hostnames, private
   keys, or certificate bodies cross this boundary.
3. **What can continue locally** — via an explicit `local_continuity_explicit`
   flag and a `local_continuity_label` on each change event and layer row.
4. **Which narrow repair or revalidation step is required** before protected
   network paths resume — via a typed `TrustStoreRepairActionClass` and a
   stable `repair_transaction_id` that ties the change event, affected layer,
   and repair outcome together.

## Contract

For the stable claim to hold, **all eight** of the following conditions must be
verified simultaneously:

1. **All five trust-store layers covered** — at least one row exists for each
   of: `os_roots`, `custom_ca_bundle`, `pinned_ssh_host_proof`,
   `client_certificate`, and `mirror_trust_root`.
2. **Raw trust material excluded** — every row and change event carries
   `raw_trust_material_excluded: true`; no certificate bodies, private keys,
   or raw fingerprints cross this boundary.
3. **Change events carry attribution** — every change event names a non-empty
   `attribution_token` from the closed `TrustStoreChangeAttributionClass`
   vocabulary.
4. **Repair transaction IDs present** — every change event carries a
   non-empty `repair_transaction_id`.
5. **Route-blocking events name repair actions** — change events whose
   `session_impact` is `session_must_pause`, `route_blocked_local_continuity`,
   or `route_blocked_no_local_fallback` must carry a typed repair action other
   than `none_required`.
6. **Local continuity explicit on required layers** — rows whose layer is
   `custom_ca_bundle`, `pinned_ssh_host_proof`, or `mirror_trust_root` must
   carry `local_continuity_explicit: true`.
7. **Managed-authority layers carry attribution refs** — rows whose layer is
   `custom_ca_bundle`, `client_certificate`, or `mirror_trust_root` must carry
   a non-empty `managed_attribution_ref`.
8. **Managed-authority events carry policy refs** — change events whose
   attribution is `admin_policy_push` or `mirror_sync` must carry a non-empty
   `managed_policy_ref`.

## Required behavior

`validate_harden_os_keychain_trust_store_page` rejects a page when its
`defects` list is non-empty.

`audit_harden_os_keychain_trust_store_page` runs the combined check and returns
a typed `Vec<HardenOsKeychainTrustStoreDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row or event with `raw_trust_material_excluded: false` (narrow reason:
  `raw_trust_material_exposed`). The function returns immediately with this
  single defect and skips all other checks.

A missing required trust-store layer narrows to `Preview` rather than `Beta`
because the coverage gap prevents any verifiable claim for that layer class.

A missing local-continuity declaration on a required layer narrows to `Beta`.

## Trust-store layers

| Layer token | Governance source | Managed authority | Local-continuity required |
| --- | --- | --- | --- |
| `os_roots` | OS vendor / platform distribution | No | No |
| `custom_ca_bundle` | Admin policy bundle | Yes | Yes |
| `pinned_ssh_host_proof` | Admin-signed known-hosts declaration | No | Yes |
| `client_certificate` | Admin policy / OS keychain enrollment | Yes | No |
| `mirror_trust_root` | Admin-signed mirror trust bundle | Yes | Yes |

## Trust-store change classes

| Change token | Description |
| --- | --- |
| `added` | New trust material was added to the layer. |
| `removed` | Existing trust material was removed. |
| `modified` | Existing trust material was modified (e.g., renewed). |
| `replaced` | One item replaced another in the layer. |
| `revoked` | Trust material was explicitly revoked; dependent routes are blocked. |

## Change attribution classes

| Attribution token | Managed authority | Description |
| --- | --- | --- |
| `os_platform_update` | No | OS platform vendor pushed an update to the system CA bundle. |
| `admin_policy_push` | Yes | Admin-signed managed policy declared this trust material. |
| `manual_admin_import` | No | Manual import by an admin or privileged user. |
| `mirror_sync` | Yes | Mirror sync over a signed transport updated the layer. |
| `material_expiry` | No | Material expired according to its embedded validity period. |
| `revocation_signal` | No | Explicit revocation signal received (CRL, OCSP, policy). |
| `keychain_relocked` | No | OS keychain or credential store was relocked. |

## Session impact classes

| Impact token | Local work | Description |
| --- | --- | --- |
| `no_impact` | Yes | No impact on active sessions or routes. |
| `revalidation_required` | Yes | Affected routes revalidate before the next request; in-flight sessions complete. |
| `session_must_pause` | Yes | Active sessions pause and revalidate; local-only work is unaffected. |
| `route_blocked_local_continuity` | Yes | Routes blocked until repair; local-only continuity is explicitly preserved. |
| `route_blocked_no_local_fallback` | No | All routes via this layer blocked; no local-only fallback. |

## Repair actions

| Action token | Description |
| --- | --- |
| `none_required` | No repair required; the change is non-blocking. |
| `revalidate_os_roots` | Revalidate affected routes against the updated OS root bundle. |
| `apply_and_revalidate_custom_ca_bundle` | Apply and revalidate the updated custom CA bundle from managed policy. |
| `reenroll_ssh_host_proof` | Re-enroll the SSH host proof from the known-hosts or admin-signed source. |
| `reenroll_client_certificate` | Re-enroll or renew the expired or revoked client certificate. |
| `refresh_mirror_trust_root` | Refresh the mirror trust root over a managed signed transport. |
| `import_signed_mirror_root` | Import a fresh signed mirror root snapshot from an out-of-band channel. |
| `unlock_os_keychain` | Unlock the OS keychain to restore access to client-certificate material. |
| `contact_admin_for_updated_bundle` | Contact the workspace admin to push an updated trust bundle. |
| `wait_for_os_platform_update` | Wait for the OS platform update to complete, then revalidate. |

## Boundary

The following material stays outside this packet's support boundary:

- Raw CA certificate PEM blobs, raw private keys, raw certificate fingerprints.
- Raw SSH known-hosts file content, raw host key bytes.
- Raw policy bundle content or raw admin identity tokens.
- Raw connection hostnames, raw session identifiers, raw IP addresses.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_harden_os_keychain_trust_store_page()` in
[`/crates/aureline-policy/src/harden_os_keychain_and_trust_store_integration_trust/mod.rs`](../../../crates/aureline-policy/src/harden_os_keychain_and_trust_store_integration_trust/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
status text or maintaining parallel trust-store state checks.

## Canonical paths

- Runtime owner: `aureline_policy::harden_os_keychain_and_trust_store_integration_trust`
- Artifact: `artifacts/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust.md`
- Fixtures: `fixtures/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust/`
- Schema: `schemas/enterprise/harden-os-keychain-and-trust-store-integration-trust.schema.json`
