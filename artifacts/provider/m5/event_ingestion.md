# Provider Event Ingestion

- Packet: `providers:event_ingestion:v1`
- Schema: `schemas/providers/provider_event_ingestion.schema.json`
- Fixture: `fixtures/providers/m5/event_ingestion/packet.json`
- Support export: `artifacts/provider/m5/event_ingestion/support_export.json`
- Contract doc: `docs/providers/m5/event_ingestion.md`

## Coverage

- Webhook, browser-return callback, polling refresh, mirror sync,
  import-session backfill, and deferred-publish queue deliveries all remain
  typed and inspectable.
- Linked objects project one stable freshness and partiality vocabulary:
  `fresh`, `partial`, `delayed`, `backfilled`, `stale`, `mirror_derived`, and
  `callback_denied`.
- Replay ledgers dedupe duplicate deliveries and keep delayed, denied, and
  backfilled deliveries auditable without creating a second user-visible
  mutation for one delivery identity.
- Import sessions keep snapshot time, omissions, rate-limit posture, and truth
  class explicit, so partial, delayed, mirrored, and backfilled state cannot
  masquerade as fresh provider truth.
- Draft and publish-later reconciliation stays gated by the latest provider
  snapshot and blocks on explicit compare or rebase review when drift is
  material.

## Guardrails

The seeded packet proves that provider-linked objects stay fresh only through
typed, deduplicated reconciliation. Delayed, partial, mirrored, stale, and
denied states remain visible to work-item, review, support/export, docs/help,
and audit surfaces, and the checked support export keeps raw provider payloads
and callback URLs out of the default boundary.
