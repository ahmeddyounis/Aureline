# Provider Event Ingestion And Publish-Later Reconciliation

Provider callbacks, webhooks, polling refreshes, mirror syncs, imports, and deferred publish drains now share one contract family in `aureline-provider`.

Machine-readable contracts:

- [`schemas/providers/provider_event_envelope.schema.json`](../../../schemas/providers/provider_event_envelope.schema.json)
- [`schemas/providers/import_session.schema.json`](../../../schemas/providers/import_session.schema.json)
- [`schemas/providers/replay_ledger_item.schema.json`](../../../schemas/providers/replay_ledger_item.schema.json)
- [`schemas/providers/reconciliation_result.schema.json`](../../../schemas/providers/reconciliation_result.schema.json)
- [`schemas/providers/provider_callback_deny_event.schema.json`](../../../schemas/providers/provider_callback_deny_event.schema.json)

Worked fixture:

- [`fixtures/providers/m3/event_replay_and_draft_reconcile/page.json`](../../../fixtures/providers/m3/event_replay_and_draft_reconcile/page.json)

Rust owner:

- [`crates/aureline-provider/src/reconciliation/mod.rs`](../../../crates/aureline-provider/src/reconciliation/mod.rs)

## Contract Shape

`ProviderEventEnvelope` is the attribution object for every inbound provider delivery. It preserves provider descriptor, source class, event type, source proof, event time, ingest time, provider object refs, freshness, truth completeness, retryability, final disposition, and the replay ledger item that decided the delivery.

`ReplayLedgerItem` is the redelivery and replay ledger. It records first seen and last seen times, replay count, dedupe window, final disposition, audit refs, and the same delivery identity used by the envelope.

`ImportSession` is the lineage object for provider-derived state. A session names its source events, object scope, snapshot time, freshness, full/partial/mirror/dry-run truth class, omissions, rate-limit posture, imported object refs, retryability, and final disposition.

`ReconciliationResult` is the local-draft and deferred-publish gate. It compares the queued-time snapshot with the latest provider snapshot before mutation and records drift, conflicts, counts, retryability, next safe action, and whether provider mutation may proceed.

`ProviderCallbackDenyEvent` is the exportable audit object for callback and webhook denies. It preserves reason, route class, policy source, actor or install ref, remediation hint, and audit refs while enforcing no user-visible provider mutation.

## Rules

- Event idempotency is based on `external_delivery_id` plus `scoped_object_ref`, with provider host and tenant/org scope retained to prevent cross-target collisions.
- Duplicate deliveries may refresh freshness, but at most one envelope for a delivery identity may mutate user-visible provider-linked state.
- Partial imports must carry explicit omissions; mirror-derived and delayed imports must keep their non-canonical truth labels.
- Denied callbacks must cite a deny event and must end with `denied_no_mutation`.
- Deferred publish and local drafts must reconcile against the latest provider snapshot before mutation.
- Material drift forces refresh, reauth, rescope, compare/rebase/review, provider handoff, or export. It may not silently publish over provider state.

## Verification

Run the provider reconciliation lane:

```bash
cargo test -p aureline-provider event_reconciliation
cargo run -p aureline-provider --bin aureline_provider_event_reconciliation -- validate
```
