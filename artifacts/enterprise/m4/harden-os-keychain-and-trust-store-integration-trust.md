# Hardened OS Keychain and Trust-Store Integration — Proof Packet

- Packet: `policy:harden-os-keychain-trust-store:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:harden_os_keychain_trust_store:v1`
- Qualification: `stable` (derived, not asserted)
- Trust-store defects: 0
- Withdrawn rows: 0
- Stable rows: all 5

## Lane coverage

| Row | Layer | Health | Governance source | Local continuity | Managed attribution |
|---|---|---|---|---|---|
| `harden-os-keychain-trust-store:os_roots` | `os_roots` | `active` | OS vendor / platform distribution | — | — |
| `harden-os-keychain-trust-store:custom_ca_bundle` | `custom_ca_bundle` | `active` | Admin policy bundle | `true` | `policy:trust-store:custom-ca-bundle:managed-ref:v1` |
| `harden-os-keychain-trust-store:pinned_ssh_host_proof` | `pinned_ssh_host_proof` | `active` | Admin-signed known-hosts declaration | `true` | — |
| `harden-os-keychain-trust-store:client_certificate` | `client_certificate` | `active` | Admin policy / OS keychain enrollment | — | `policy:trust-store:client-cert:managed-ref:v1` |
| `harden-os-keychain-trust-store:mirror_trust_root` | `mirror_trust_root` | `active` | Admin-signed mirror trust bundle | `true` | `policy:trust-store:mirror-trust-root:managed-ref:v1` |

## Change events

| Transaction ID | Layer | Change | Attribution | Session impact | Repair action |
|---|---|---|---|---|---|
| `txn:trust-store:os-bundle-update:2026-06-01:0001` | `os_roots` | `modified` | `os_platform_update` | `revalidation_required` | `revalidate_os_roots` |
| `txn:trust-store:custom-ca-policy-push:2026-06-01:0002` | `custom_ca_bundle` | `replaced` | `admin_policy_push` | `revalidation_required` | `apply_and_revalidate_custom_ca_bundle` |
| `txn:trust-store:ssh-host-proof-added:2026-06-01:0003` | `pinned_ssh_host_proof` | `added` | `admin_policy_push` | `no_impact` | `none_required` |
| `txn:trust-store:client-cert-renewed:2026-06-01:0004` | `client_certificate` | `replaced` | `admin_policy_push` | `revalidation_required` | `reenroll_client_certificate` |
| `txn:trust-store:mirror-trust-root-synced:2026-06-01:0005` | `mirror_trust_root` | `modified` | `mirror_sync` | `revalidation_required` | `refresh_mirror_trust_root` |

## Key invariants verified

1. All five required trust-store layers (`os_roots`, `custom_ca_bundle`, `pinned_ssh_host_proof`, `client_certificate`, `mirror_trust_root`) have rows.
2. Every row and change event carries `raw_trust_material_excluded: true`; no certificate bodies, private keys, or raw fingerprints cross this boundary.
3. Every change event carries a non-empty `attribution_token` from the closed vocabulary.
4. Every change event carries a non-empty `repair_transaction_id`.
5. All change events with a blocking or pausing `session_impact` carry a typed repair action other than `none_required`.
6. `custom_ca_bundle`, `pinned_ssh_host_proof`, and `mirror_trust_root` rows carry `local_continuity_explicit: true`.
7. `custom_ca_bundle`, `client_certificate`, and `mirror_trust_root` rows carry a non-empty `managed_attribution_ref`.
8. Change events with `admin_policy_push` or `mirror_sync` attribution carry a non-empty `managed_policy_ref`.

## Hard guardrail — withdrawal condition

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row or event with `raw_trust_material_excluded: false`
  (narrow reason: `raw_trust_material_exposed`).

## Canonical paths

- Doc: `docs/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust.md`
- Runtime owner: `aureline_policy::harden_os_keychain_and_trust_store_integration_trust`
- Fixtures: `fixtures/enterprise/m4/harden-os-keychain-and-trust-store-integration-trust/`
- Schema: `schemas/enterprise/harden-os-keychain-and-trust-store-integration-trust.schema.json`
