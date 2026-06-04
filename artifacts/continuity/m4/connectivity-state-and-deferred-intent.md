# Connectivity State And Deferred Intent Evidence

Evidence packet:

- Rust contract: `crates/aureline-continuity`
- Schema: `schemas/continuity/deferred-intent-and-reconciliation.schema.json`
- Fixture: `fixtures/continuity/m4/connectivity-state-and-deferred-intent/page.json`
- Docs: `docs/continuity/m4/connectivity-state-and-deferred-intent.md`

Coverage:

- All six shared connectivity states are represented.
- Local editing, search, local Git, tasks, and cached inspection remain available
  for constrained, offline-local-safe, reauth, reconciliation, and service-family
  outage states unless an explicit deployment policy narrows the profile.
- Queue admission allows explicit idempotent reviewable provider intent with
  idempotency, expiry, actor, target, policy, auth, context, stale-label, and
  owner metadata.
- Git push is declared as never queueable and requires live rerun.
- Reconnect replay is blocked by route, policy, auth, tenant, region, endpoint,
  target, version, entitlement, context, metadata, or expiry drift.
- Support export includes redaction-safe lineage and excludes raw sensitive
  payloads by default.
