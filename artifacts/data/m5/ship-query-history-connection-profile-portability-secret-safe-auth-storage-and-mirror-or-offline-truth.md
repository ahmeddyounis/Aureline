# Query history, connection-profile portability, secret-safe auth storage, and mirror or offline truth — Artifact Summary

## Packet identity

- `packet_id`: `m5_043_ship_query_history_qualification:v1`
- `as_of`: `2026-06-09`
- `schema_version`: `1`
- `record_kind`: `ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth`

## Surfaces

4 promoted surfaces, 3 stable, 1 narrowed.

## Rows

- 5 query-history entries (local-first, bounded, pinned, ephemeral, audit-only)
- 3 connection-profile portabilities (local-only, redacted export, blocked)
- 3 secret-safe auth storages (local encrypted, secret broker only, managed rotation)
- 4 mirror or offline truths (online default, online replica, offline grace window, offline local only)

## Validation

The packet passes `ShipQueryHistoryQualificationPacket::validate()` with zero violations.

## Downgrade behavior

All surfaces have `downgrade_if_missing: true`. If proof artifacts are stale or removed, stable claims narrow to `preview` automatically.
