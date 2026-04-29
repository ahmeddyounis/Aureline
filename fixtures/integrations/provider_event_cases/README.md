# Provider Event Ingestion Fixtures

Worked scenario bundles for the contract frozen in
[`/docs/integrations/provider_event_ingestion_contract.md`](../../../docs/integrations/provider_event_ingestion_contract.md).

Each fixture is a self-contained JSON document. The `records` array
contains records valid against one of these schemas:

- [`/schemas/integrations/provider_event.schema.json`](../../../schemas/integrations/provider_event.schema.json)
- [`/schemas/integrations/import_session.schema.json`](../../../schemas/integrations/import_session.schema.json)
- [`/schemas/integrations/webhook_replay_record.schema.json`](../../../schemas/integrations/webhook_replay_record.schema.json)

Coverage:

| Fixture | Main condition | Required behavior |
|---|---|---|
| `valid_delivery_applied_once.json` | Verified webhook delivery | Apply once, trace imported state to provider event and import session. |
| `duplicate_delivery_deduped.json` | Provider redelivery with same delivery id | Suppress duplicate mutation while refreshing freshness. |
| `out_of_order_missing_page_backfill.json` | Sequence gap / missing page | Hold or partially apply with backfill session and partial visibility. |
| `signature_failure_denied_audit.json` | Invalid signature | Deny before state mutation and emit callback-deny/audit refs. |
| `stale_permission_failed_partial_import.json` | Permission snapshot is stale mid-import | Preserve partial state as degraded and require permission refresh. |
| `cursor_reset_manual_replay_dry_run.json` | Cursor reset plus operator replay | Rewind/backfill explicitly, record blast radius, and keep dry-run output non-writing. |
