# Ship query history, connection-profile portability, secret-safe auth storage, and mirror or offline truth

## Overview

This document describes the M5 canonical implementation for query history, connection-profile portability, secret-safe auth storage, and mirror or offline truth. The implementation lives in:

- **Rust types**: `crates/aureline-api/src/ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth/`
- **Schema**: `schemas/data/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.schema.json`
- **Checked-in packet**: `artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json`

## Design Principles

1. **Query history is metadata, not a secret cache**: Entries carry opaque refs to statement bodies, safety results, and connection profiles. Raw SQL, bind values, and row payloads stay in local stores.
2. **Connection-profile portability is redaction-first**: Export and migration surfaces never include raw secrets or raw endpoint details by default. Auth handle migration is visible but broker handles remain opaque.
3. **Secret-safe auth storage never observes raw secrets in workspace state**: Storage modes include local encrypted, secret-broker-only, managed rotation, and policy-locked. Any raw secret observation is a violation.
4. **Mirror or offline truth is explicit**: Connectivity state, cache warmth, fallback posture, and disclosure sentences are visible so users understand when they are on a replica, in a grace window, or offline.

## Core Types

### QueryHistoryEntryRow

A query history entry with:
- `connection_profile_ref`, `statement_body_ref`, `statement_safety_result_ref`: opaque refs
- `result_size_class`, `row_count_truth_class`: bounded result summaries
- `retention_posture`: `LocalFirst`, `Bounded`, `Pinned`, `Ephemeral`, `AuditOnly`
- `replay_drift_risk`: `NoDrift`, `LowDrift`, `ModerateDrift`, `HighDrift`, `Blocked`
- `export_safe` and `redaction_class`
- `last_executed_at` and `pinned`

### ConnectionProfilePortabilityRow

Connection-profile portability with:
- `source_profile_ref`, `target_format`
- `includes_raw_secrets`, `includes_raw_endpoint`: must be false for redacted and blocked postures
- `auth_handle_migration_visible`
- `export_posture`, `import_posture`: `LocalOnly`, `RedactedExport`, `FullMigration`, `Blocked`

### SecretSafeAuthStorageRow

Secret-safe auth storage with:
- `auth_handle_class`, `secret_broker_ref`
- `encryption_at_rest`, `rotation_policy_visible`
- `raw_secret_observed`: must be false for valid rows
- `storage_mode`: `LocalEncrypted`, `SecretBrokerOnly`, `ManagedRotation`, `PolicyLocked`
- `export_safe`

### MirrorOrOfflineTruthRow

Mirror or offline truth with:
- `mirror_or_offline_state_class`: `OnlineDefault`, `OnlineReplica`, `OfflineGraceWindow`, `OfflineLocalOnly`, `NetworkDisabled`
- `replica_endpoint_ref`, `offline_since`
- `cache_warmth`, `fallback_posture`, `connectivity_disclosure`

## Qualification Packet

The `ShipQueryHistoryQualificationPacket` binds together:
- `surfaces`: governed surface rows with labels, guards, and proof packets
- `query_history_entries`, `connection_profile_portabilities`, `secret_safe_auth_storages`, `mirror_or_offline_truths`
- `summary`: counts that must match the computed summary

### Validation Rules

- `schema_version` must equal `1`
- `record_kind` must equal the canonical string
- All IDs within a family must be unique
- Stable surfaces must have a proof packet and complete guard truth
- Narrowed stable claims must have `downgrade_if_missing: true`
- Query history entries must cover `LocalFirst`, `Bounded`, `Pinned` retention postures
- Query history entries must cover `NoDrift`, `LowDrift`, `HighDrift`, `Blocked` replay drift risks
- Connection-profile portabilities must cover `LocalOnly`, `RedactedExport`, `Blocked`
- Connection-profile portabilities must not include raw secrets or raw endpoints
- Secret-safe auth storages must cover `LocalEncrypted`, `SecretBrokerOnly`, `ManagedRotation`
- Secret-safe auth storages must not observe raw secrets in workspace state
- Mirror or offline truths must cover `OnlineDefault`, `OnlineReplica`, `OfflineGraceWindow`, `OfflineLocalOnly`

## Downgrade and Rollback

If any of the following conditions are met, the affected surface narrows below `Stable`:
- Proof packet is missing or stale
- Guard truth is incomplete
- Required retention posture, replay drift risk, portability posture, auth storage mode, or offline state is missing
- Connection-profile portability includes raw secrets or raw endpoints
- Secret-safe auth storage observes raw secrets in workspace state
- Mirror or offline truth lacks connectivity disclosure

## Integration

Downstream UI, CLI, support, and export surfaces consume the typed crate (`aureline-api`) and the checked-in JSON packet. They do not re-describe state manually.

## Verification

Run the crate tests and validation:

```bash
cargo check -p aureline-api
cargo test -p aureline-api
```

The embedded JSON artifact is validated at compile time via `include_str!`, so any drift between the artifact and the typed model will fail `cargo check`.
