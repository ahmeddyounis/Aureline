# Provider Event Ingestion And Provenance Artifact

This artifact records the stable imported-provider event contract for callback, webhook, mirror, browser-handoff, import-session, and publish-later ingress.

Canonical records:

- Runtime model: `crates/aureline-provider/src/provider_event_ingestion_and_provenance/mod.rs`
- Schema: `schemas/providers/provider_event_ingestion_and_provenance.schema.json`
- Fixture: `fixtures/providers/m4/provider-event-ingestion-and-provenance/packet.json`
- Reviewer docs: `docs/provider/m4/provider-event-ingestion-and-provenance.md`

Acceptance evidence:

- The packet validates connected-account and installation-grant authority as distinct source classes.
- Duplicate deliveries are keyed by external delivery id plus scoped object ref and may not mutate provider-linked state twice.
- Browser-return callbacks cite their handoff packet origin before they can affect local objects.
- Denied events cite policy and audit refs while preserving no-mutation outcomes.
- Stale and mirror-derived events stay labeled instead of masquerading as live provider truth.
- Publish-later and browser-handoff overlaps force conflict review.
- Work-item, review, activity-center, and support/export projections use controlled labels: `Imported`, `Buffered`, `Replayed`, `Denied`, `Stale`, and `Conflict review required`.
- Support export packets omit raw provider payloads and callback URLs.
