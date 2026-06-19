# Artifact: key-mode and storage-posture inspectors

**Contract ref:** `continuity:m5_key_mode_and_storage_posture:v1`  
**Schema:** `schemas/continuity/key_mode_descriptor.schema.json`  
**Doc:** `docs/m5/continuity/key-mode-and-storage-posture.md`  
**Runtime owner:** `aureline_continuity::m5_key_mode_and_storage_posture`

## Qualification

| Condition | Status |
|---|---|
| Encryption posture disclosed and names its key mode on every row | ✓ Stable |
| Key mode named + trust-root posture declared on managed-scope rows | ✓ Stable |
| Key/trust material available, store unlocked, evidence current | ✓ Stable |
| No self-hosted/sovereign reliance on vendor keys or trust root | ✓ Stable |
| Every required surface projected | ✓ Stable |
| Key + storage vocabulary identical across surfaces | ✓ Stable |
| No raw key material in any record | ✓ Stable |
| **Overall** | **Stable** |

## Key-mode descriptors

| Surface | Profile | Key mode | Trust root | Availability | Store lock | Evidence | Degraded state |
|---|---|---|---|---|---|---|---|
| Managed cloud workspace sync and backup | `managed` | `vendor_managed_keys` | `vendor_managed_trust_root` | `available` | `not_applicable` | `current` | `none_healthy` |
| Managed relay and collaboration failover | `managed` | `vendor_managed_keys` | `vendor_managed_trust_root` | `available` | `not_applicable` | `current` | `none_healthy` |
| Customer self-hosted restore and rebuild | `self_hosted` | `customer_managed_keys` | `customer_managed_trust_root` | `available` | `not_applicable` | `current` | `none_healthy` |
| Sovereign air-gapped snapshot and replication | `sovereign` | `customer_held_root` | `offline_trust_root` | `available` | `not_applicable` | `stale_within_grace` | `none_healthy` |
| Local desktop core continuity | `local_only` | `local_os_keystore` | `os_store_trust_root` | `available` | `unlocked` | `current` | `none_healthy` |

## Storage-posture descriptors

| Surface | Encryption at rest | Protecting key mode | Trust root | Key mode named |
|---|---|---|---|---|
| Managed cloud workspace sync and backup | `vendor_key_encrypted` | `vendor_managed_keys` | `vendor_managed_trust_root` | yes |
| Managed relay and collaboration failover | `vendor_key_encrypted` | `vendor_managed_keys` | `vendor_managed_trust_root` | yes |
| Customer self-hosted restore and rebuild | `customer_key_encrypted` | `customer_managed_keys` | `customer_managed_trust_root` | yes |
| Sovereign air-gapped snapshot and replication | `offline_sealed_encrypted` | `customer_held_root` | `offline_trust_root` | yes |
| Local desktop core continuity | `device_local_encrypted` | `local_os_keystore` | `os_store_trust_root` | yes |

The customer self-hosted row exercises a real customer-managed-key lane and the
sovereign row exercises a real offline-trust-root lane, satisfying the
requirement that the proof packet be exercised by at least one customer-managed
or offline-trust-root lane.

## Fail-closed and narrowing cases

| Fixture | Trigger | Outcome |
|---|---|---|
| `case_customer_key_unavailable_withdrawn.json` | customer-managed key unavailable | withdrawn (fail closed), local-core preserved |
| `case_trust_root_mismatch_withdrawn.json` | offline trust root mismatch | withdrawn (fail closed) |
| `case_key_material_lost_withdrawn.json` | durable key material lost | withdrawn (fail closed) |
| `case_store_locked_preview.json` | local store locked on managed lane | preview (`store_locked_degraded`) |
| `case_encryption_opaque_beta.json` | "encrypted" without a named key mode | beta |
| `case_profile_key_mode_mismatch_preview.json` | self-hosted row leans on vendor keys | preview |

In every fail-closed case only the protected managed lane narrows; the local
desktop core row stays `stable` and `local_core_preserved` remains true on every
row.

## Export safety

All records are metadata-only: closed-vocabulary tokens, export-safe
plain-language labels, and opaque evidence refs. The summary and support-export
records assert `raw_key_material_excluded`. No raw KMS handles, raw trust roots,
raw key bytes, or secret material appear.
