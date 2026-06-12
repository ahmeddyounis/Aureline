# M5 Provider-Work-Item Governance Matrix

This contract freezes the canonical M5 vocabulary for provider-linked work items, review-linked change intent, acting identity, typed browser handoff, deferred publish, imported snapshots, and provider-event reconciliation.

## Scope

- Provider-backed work items, issues, tickets, and incidents remain typed engineering objects rather than shallow provider links.
- Review-linked change intent, browser handoff packets, deferred publish packets, imported snapshots, and callback/event envelopes reuse one governed vocabulary across review, work-item, incident, companion, support, docs, and release surfaces.
- Acting identity and effective scope remain first-class: human account, installation grant, delegated credential, browser-only fallback, denied scope, and publish-later local draft never collapse into one generic connected state.
- Truth states remain first-class: local draft, queued publish, provider committed, stale snapshot, partial scope, mirror derived, and callback denied remain visibly distinct.

## Frozen vocabularies

### Object classes

- `provider_work_item`
- `review_linked_change_intent`
- `browser_handoff_packet`
- `deferred_publish_packet`
- `imported_snapshot`
- `provider_event_envelope`

### Acting identity and effective scope

- Acting identity: `human_account`, `installation_grant`, `delegated_credential`, `browser_only_fallback`, `denied_scope`, `publish_later_local_draft`
- Effective scope: `provider_mutation`, `limited_comment_link`, `browser_only_fallback`, `denied_scope`, `publish_later_local_draft`

### Truth states

- `local_draft`
- `queued_publish`
- `provider_committed`
- `stale_snapshot`
- `partial_scope`
- `mirror_derived`
- `callback_denied`

## Lane rows

- `work_item_object_vocabulary`
- `provider_linked_mutation`
- `acting_identity_and_effective_scope`
- `browser_handoff_continuity`
- `deferred_publish_continuity`
- `provider_event_reconciliation`

Each lane binds qualification, evidence refs, downgrade triggers, rollback posture, source contracts, and consumer-surface parity.

## Guardrails

- Provider-owned objects never masquerade as canonical local truth.
- Local draft, queued publish, provider-committed state, and callback-denied audit state stay visibly separate.
- Imported snapshots and mirrors never claim provider-committed freshness.
- Browser handoff remains typed, attributable, and return-anchor safe.
- Callback or webhook events only mutate state through typed, deduplicated reconciliation paths; deny/audit events stay visible when mutation is blocked.
- If provider authority, callback reconciliation proof, or publish-later continuity proof goes stale, claimed rows narrow automatically instead of inheriting maturity from adjacent review or browser-handoff lanes.

## Consumers

- Review workspace
- Work-item detail and transition review
- Incident workspace and export-safe evidence
- Companion triage and desktop handoff
- Browser handoff cards
- CLI/headless and support export
- Docs/help and release evidence packets
