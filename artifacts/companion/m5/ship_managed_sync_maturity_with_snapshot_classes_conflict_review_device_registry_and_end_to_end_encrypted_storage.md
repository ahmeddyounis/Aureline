# Managed Sync Maturity: Snapshot Classes, Conflict Review, Device Registry, and End-to-End Encrypted Storage

- Packet: `managed-sync-maturity-surface:stable:0001`
- Label: `Managed Sync Maturity: Snapshot Classes, Conflict Review, Device Registry, and End-to-End Encrypted Storage`
- Sections: 4 | Snapshot classes: 3 | Conflicts: 2 | Devices: 3 | Encrypted scopes: 4
- Exact desktop handoff for every item: yes
- No silent server authority: yes
- Encryption claims honestly qualified: yes
- Stale state honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **snapshot_class**: `beta` / `staged_rollout` [read_only] (matrix lane `managed_sync`)
- **conflict_review**: `beta` / `staged_rollout` [read_only] (matrix lane `managed_sync`)
- **device_registry**: `beta` / `staged_rollout` [read_only] (matrix lane `managed_sync`)
- **encrypted_storage**: `preview` / `early_access` [read_only] (matrix lane `residency_encryption`)

## Snapshot classes

- `snapshot:0001` [settings/bidirectional/reconciled] Settings snapshot class reconciled with the local core (live) → `review_panel` (exact)
- `snapshot:0002` [profile/bidirectional/diverged_pending_review] Profile snapshot class diverged on two devices; pending conflict review (cached) → `review_panel` (exact)
- `snapshot:0003` [device_registry/managed_to_local/reconciled] Device registry snapshot class reconciled with the local core (live) → `review_panel` (exact)

## Conflict review

- `conflict:0001` [concurrent_edit/pending_review/diverged_pending_review] Profile edited concurrently on two devices; awaiting user review (live) → `review_panel` (exact)
- `conflict:0002` [clock_skew/resolved_keep_local/reconciled] Clock-skew ordering reviewed by the user; local version kept (cached) → `review_panel` (exact)

## Device registry

- `device:0001` [trusted/this_device/reconciled] This device, trusted and reconciled with the local core (live) → `review_panel` (exact)
- `device:0002` [trusted/reconciled] Trusted laptop participating in managed sync (cached) → `review_panel` (exact)
- `device:0003` [pending_approval/diverged_pending_review] New device awaiting approval before it may sync (unknown) → `review_panel` (exact)

## Encrypted storage

- `encrypted:0001` [managed_snapshot_store/end_to_end_encrypted_verified/customer_managed_key] residency `region:eu-west` (verified: yes) Managed snapshot store end-to-end encrypted with a customer-managed key; claim verified (live) → `review_panel` (exact)
- `encrypted:0002` [sync_transport/encrypted_at_rest_verified/provider_managed_key] residency `region:eu-west` (verified: yes) Sync transport encrypted at rest with a provider-managed key; claim verified (cached) → `review_panel` (exact)
- `encrypted:0003` [conflict_history/claimed_unverified/provider_managed_key] residency `region:unverified` (verified: no) Conflict-history encryption claimed but not yet verified; labeled as unverified (unknown) → `review_panel` (exact)
- `encrypted:0004` [device_registry_store/end_to_end_encrypted_verified/local_only_no_key_escrow] residency `region:local-device` (verified: yes) Device registry store end-to-end encrypted with a local-only key, never escrowed; claim verified (live) → `review_panel` (exact)
