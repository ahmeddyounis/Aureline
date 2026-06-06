# Provider Event Ingestion And Provenance

Inbound provider events enter Aureline through a stable imported-event envelope before any provider-linked object, review row, work-item row, activity row, or support export may describe a state change.

The stable packet is `ProviderEventIngestionProvenancePacket` in `aureline-provider`. Its schema is `schemas/providers/provider_event_ingestion_and_provenance.schema.json`, and the canonical fixture is `fixtures/providers/m4/provider-event-ingestion-and-provenance/packet.json`.

## Envelope Contract

Each `ImportedProviderEventEnvelope` names:

- provider descriptor and provider family
- authority source as connected account, installation grant, delegated credential, or policy-injected service
- ingress class for webhook, browser-return callback, polling refresh, mirror ingress, import session, or publish-later drain
- event class and canonical provider/local object identity
- freshness floor and import-session reference
- external delivery id, scoped object ref, dedupe key, replay key, replay decision, and replay count
- browser-handoff packet origin when the event is a browser-return callback
- policy verdict, policy epoch, effective-scope ref, and audit refs
- overlap with local drafts, publish-later queue items, or unreconciled browser-handoff sessions
- resulting local object refs and resulting state

Raw provider payload refs and raw callback URLs are not part of the stable envelope or support export.

## Surface Labels

Provider-linked surfaces must not fold imported events into generic local changes. Work-item, review, activity-center, diagnostics, and support/export rows use the controlled labels:

- `Imported`
- `Buffered`
- `Replayed`
- `Denied`
- `Stale`
- `Conflict review required`

An event whose policy verdict is denied must be labeled `Denied` and must cite audit refs. A stale event may remain visible, but it cannot claim fresh imported state. A duplicate delivery may refresh freshness metadata, but it cannot create a second user-visible mutation for the same dedupe key.

## Conflict Rule

If an imported event overlaps an unreconciled local draft, publish-later queue item, or browser-handoff session, the envelope must use `Conflict review required` and cite a conflict-review ref. Silent last-writer-wins behavior is invalid for provider-owned mutations.

## Support Export

The support export summarizes provider descriptor, authority source, external delivery id, dedupe key, visible surface state, policy verdict, resulting local object refs, and an export-safe summary. It is intentionally payload-free so diagnostics can explain why state changed without exporting tokens, raw callback URLs, or provider bodies.
