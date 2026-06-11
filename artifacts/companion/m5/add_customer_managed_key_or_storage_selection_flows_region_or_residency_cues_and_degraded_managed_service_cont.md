# Customer-Managed-Key and Storage Selection Flows, Region/Residency Cues, and Degraded Managed-Service Continuity

- Packet: `key-storage-residency-continuity-surface:stable:0001`
- Label: `Customer-Managed-Key and Storage Selection Flows, Region/Residency Cues, and Degraded Managed-Service Continuity`
- Sections: 4 | Key-custody options: 3 | Storage options: 4 | Residency cues: 3 | Continuity rows: 4
- Exact desktop handoff for every item: yes
- Local-only key fallback offered: yes
- Local-first storage fallback offered: yes
- Encryption claims honestly qualified: yes
- Residency claims honestly qualified: yes
- Local work never stranded: yes
- Stale state honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **key_custody_selection**: `preview` / `early_access` [read_only] (matrix lane `residency_encryption`)
- **storage_selection**: `preview` / `early_access` [read_only] (matrix lane `residency_encryption`)
- **residency_cue**: `preview` / `early_access` [read_only] (matrix lane `residency_encryption`)
- **managed_service_continuity**: `beta` / `staged_rollout` [read_only] (matrix lane `offboarding_continuity`)

## Key custody selection

- `key:0001` [customer_managed_key/active/end_to_end_encrypted_verified] (verified: yes) Customer-managed key active; managed artifacts end-to-end encrypted, claim verified (live) → `review_panel` (exact)
- `key:0002` [provider_managed_key/available/encrypted_at_rest_verified] (verified: yes) Provider-managed key available; encrypted at rest, claim verified (cached) → `review_panel` (exact)
- `key:0003` [local_only_no_key_escrow/available/end_to_end_encrypted_verified] (verified: yes) Local-only key, never escrowed, always available as a fallback; claim verified (live) → `review_panel` (exact)

## Storage selection

- `storage:0001` [hybrid_local_first/active] residency `region:eu-west` (verified: yes) Local-first storage with an EU-west managed mirror; residency verified (live) → `review_panel` (exact)
- `storage:0002` [customer_managed_bucket/available] residency `region:customer-bucket` (verified: yes) Customer-managed storage bucket available; residency verified (cached) → `review_panel` (exact)
- `storage:0003` [provider_managed_region/requires_admin_approval] residency `region:unverified` (verified: no) Provider-managed region offered but residency not yet verified; labeled, admin-gated (unknown) → `review_panel` (exact)
- `storage:0004` [local_only/available] residency `region:local-device` (verified: yes) Local-only storage always available as a fallback; residency verified (live) → `review_panel` (exact)

## Residency cues

- `residency:0001` [managed_snapshot_store/pinned_verified] residency `region:eu-west` (verified: yes) Managed snapshot store pinned to EU-west; pin verified (live) → `review_panel` (exact)
- `residency:0002` [sync_transport/pinned_verified] residency `region:eu-west` (verified: yes) Sync transport pinned to EU-west; pin verified (cached) → `review_panel` (exact)
- `residency:0003` [conflict_history/pinned_unverified] residency `region:unverified` (verified: no) Conflict-history residency claimed but not yet verified; labeled (unknown) → `review_panel` (exact)

## Managed-service continuity

- `continuity:0001` [managed_sync/degraded_local_fallback] local_work_preserved `yes` degraded `no` Managed sync degrades to local-first; edits keep flowing to the local core (live) → `review_panel` (exact)
- `continuity:0002` [key_management/requires_provider_continuity] local_work_preserved `yes` degraded `no` Customer-managed-key rotation requires the key-management provider; local-only key keeps working (live) → `review_panel` (exact)
- `continuity:0003` [device_approval/requires_admin_continuity] local_work_preserved `yes` degraded `no` Managed-tenant device approval requires admin continuity; the local core never depends on it (cached) → `review_panel` (exact)
- `continuity:0004` [managed_audit_log/local_core_continues_unaffected] local_work_preserved `yes` degraded `no` Local activity log is fully local and unaffected by managed-service degradation (live) → `review_panel` (exact)
