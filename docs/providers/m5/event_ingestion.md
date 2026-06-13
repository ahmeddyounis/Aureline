# Provider Event Ingestion

This packet freezes the canonical M5 provider event-ingestion contract for
external deliveries that keep provider-linked work items, reviews, and related
objects fresh without pretending that delayed, partial, mirrored, or denied
state is live-provider truth.

## Contract

- Rust schema of record:
  [`crates/aureline-provider/src/event_ingestion/mod.rs`](../../../crates/aureline-provider/src/event_ingestion/mod.rs)
- Canonical packet schema:
  [`schemas/providers/provider_event_ingestion.schema.json`](../../../schemas/providers/provider_event_ingestion.schema.json)
- Lower-level provenance packet:
  [`schemas/providers/provider_event_ingestion_and_provenance.schema.json`](../../../schemas/providers/provider_event_ingestion_and_provenance.schema.json)
- Lower-level reconciliation records:
  [`schemas/providers/provider_event_envelope.schema.json`](../../../schemas/providers/provider_event_envelope.schema.json),
  [`schemas/providers/import_session.schema.json`](../../../schemas/providers/import_session.schema.json),
  [`schemas/providers/replay_ledger_item.schema.json`](../../../schemas/providers/replay_ledger_item.schema.json),
  [`schemas/providers/reconciliation_result.schema.json`](../../../schemas/providers/reconciliation_result.schema.json),
  [`schemas/providers/provider_callback_deny_event.schema.json`](../../../schemas/providers/provider_callback_deny_event.schema.json)
- Seeded fixture:
  [`fixtures/providers/m5/event_ingestion/packet.json`](../../../fixtures/providers/m5/event_ingestion/packet.json)
- Seeded support export:
  [`artifacts/provider/m5/event_ingestion/support_export.json`](../../../artifacts/provider/m5/event_ingestion/support_export.json)

## What the packet proves

- Every external delivery still enters through a typed event envelope with
  provider ref, source class, event type, delivery identity, object refs,
  event time, ingest time, and source proof before local state can change.
- Import sessions keep snapshot time, full versus partial versus delayed
  versus backfilled versus mirror-derived truth class, omissions, and
  rate-limit posture explicit instead of collapsing everything into one sync
  badge.
- Replay and redelivery ledgers dedupe duplicate deliveries, keep
  out-of-order or delayed refreshes auditable, and prevent a second
  user-visible mutation for one delivery identity.
- Local drafts and queued publishes reconcile against the latest provider
  snapshot before mutation. Material drift stays blocked behind compare,
  rebase, refresh, or export review instead of silently publishing over newer
  provider truth.
- Provider-linked objects now share one stable linked-object vocabulary across
  work-item detail, review workspace, CLI/headless, support/export, docs/help,
  and audit timeline surfaces: `fresh`, `partial`, `delayed`, `backfilled`,
  `stale`, `mirror_derived`, and `callback_denied`.

## Guardrails

- Unsigned, host-mismatched, or policy-denied callbacks remain audit-only and
  may not mutate user-visible provider-linked state.
- Mirror-derived, delayed, backfilled, and partial imports stay labeled and do
  not upgrade themselves to fresh provider-committed truth.
- Support/export projections preserve the external event lineage needed to
  reconstruct what changed a provider-linked object without exporting raw
  provider payloads or callback URLs by default.
