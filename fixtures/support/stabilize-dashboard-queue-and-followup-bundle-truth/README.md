# Dashboard, Queue, and Follow-up Bundle Truth Fixtures

This corpus pins the stable support/export packet for dashboard freshness,
queue-order explainability, and provider-linked follow-up bundles.

Files:

- `canonical_packet.json` is the full packet consumed by support, copy/export,
  queue, and follow-up surfaces.
- `support_export_projection.json` is the export-only projection. It preserves
  scope, owner, filter state, freshness, ownership, checklist semantics, and
  reviewed provider commands outside the live shell.

Fixture invariants:

- A green dashboard card with cached, stale, imported, truncated, or blocked
  data is visibly downgraded and keeps a canonical evidence ref.
- Every queue row carries order and grouping explanations; hidden scope names
  the policy or provider narrowing reason and count.
- Checklist completion is local-only. Provider-owned mutation is represented
  only by a separate reviewed command with exact target, actor, and one of
  `draft`, `publish-later`, `publish-now`, or `handoff-only`.
