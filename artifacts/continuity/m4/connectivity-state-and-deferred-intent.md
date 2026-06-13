# Connectivity State And Deferred Intent Evidence

Evidence packet:

- Rust contract: `crates/aureline-continuity`
- Schema: `schemas/continuity/deferred-intent-and-reconciliation.schema.json`
- Fixture: `fixtures/continuity/m4/connectivity-state-and-deferred-intent/page.json`
- Docs: `docs/continuity/m4/connectivity-state-and-deferred-intent.md`

Coverage:

- All six shared connectivity states are represented.
- Badges and detail cards exist for every required state, and the seeded
  presentation covers provider, managed-workspace, and remote M5 action
  families.
- Local editing, search, local Git, tasks, and cached inspection remain available
  for constrained, offline-local-safe, reauth, reconciliation, and service-family
  outage states unless an explicit deployment policy narrows the profile.
- Queue admission allows explicit idempotent reviewable provider and managed
  intents with idempotency, expiry, actor, target, policy, auth, context,
  data-fingerprint, stale-label, and owner metadata.
- Remote execution and Git push remain never queueable and require a live rerun.
- Deferred-intent outbox rows retain replay prerequisites, idempotency-key refs,
  and redaction-safe effect summaries.
- Reconciliation packets demonstrate replayed, blocked, narrowed, discarded, and
  still-queued outcomes without invisible replay.
- Support export includes redaction-safe lineage, receipt refs, packet refs, and
  actor/target summaries while excluding raw sensitive payloads by default.
