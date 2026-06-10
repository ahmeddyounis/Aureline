# REST and GraphQL response viewers, assertions, timing tabs, and browser-runtime trust classes — Artifact Summary

## Packet identity

- `packet_id`: `m5_036_response_viewer_qualification:v1`
- `as_of`: `2026-06-09`
- `schema_version`: `1`
- `record_kind`: `ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes`

## Surfaces

5 promoted surfaces, 4 stable, 1 narrowed.

## Rows

- 2 response viewers (REST, GraphQL)
- 4 assertions (pass, fail, error, skipped)
- 2 timing tabs (REST GET, GraphQL query)
- 4 browser-runtime trust rows (DOM, network, storage, console)

## Validation

The packet passes `ResponseViewerQualificationPacket::validate()` with zero violations.

## Downgrade behavior

All surfaces have `downgrade_if_missing: true`. If proof artifacts are stale or removed, stable claims narrow to `preview` automatically.
