# Connection browsers, schema trees, and target-context envelopes for database tooling — Artifact Summary

## Packet identity

- `packet_id`: `m5_037_database_browser_qualification:v1`
- `as_of`: `2026-06-09`
- `schema_version`: `1`
- `record_kind`: `connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling`

## Surfaces

3 promoted surfaces, 3 stable, 0 narrowed.

## Rows

- 5 connection browsers (embedded local, local dev, remote staging, cloud warehouse, imported snapshot)
- 4 schema trees (PostgreSQL live, MySQL stale, SQLite cached, MongoDB imported)
- 7 target-context envelopes (covering all statement safety classes, transaction postures, result scopes, and redaction modes)

## Validation

The packet passes `DatabaseBrowserQualificationPacket::validate()` with zero violations.

## Downgrade behavior

All surfaces have `downgrade_if_missing: true`. If proof artifacts are stale or removed, stable claims narrow to `preview` automatically.
