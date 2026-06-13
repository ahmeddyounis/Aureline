# Connectivity State, Deferred Intent, And Reconciliation

This contract makes connectivity posture one shared model across managed,
provider-linked, request, remote, service-health, diagnostics, and support
surfaces.

Stable states are `Connected`, `Constrained`, `OfflineLocalSafe`,
`ReauthRequired`, `ReconciliationPending`, and `ServiceUnavailable`. Surfaces
must quote these labels rather than inventing local synonyms.

M5 disclosure surfaces read the same contract directly:

- connectivity badges keep the current state, affected family, local-safe
  promise, and one recovery action visible;
- connectivity cards expand that strip into what still works locally plus the
  next safe actions;
- deferred-intent outbox rows preserve queue time, expiry, actor, target,
  idempotency-key ref, data fingerprint, previewed effect summary, and exact
  replay prerequisites;
- idempotency-key receipts prove whether an intent stayed queued, replayed once,
  narrowed, blocked, deduped, or was discarded; and
- reconciliation packets plus review sheets keep replay, block, narrowing, and
  discard outcomes visible so reconnect never becomes an invisible replay path.

Every networked command declares:

- offline-read class
- queueability class
- replay-safety class
- idempotency-key shape when queueable
- expiry policy when queueable
- stale-label semantics
- reconciliation owner

Only explicit idempotent reviewable intent with bounded blast radius can enter
the deferred-intent queue. Terminal input, Git push, destructive API mutation,
publish/deploy, collaboration control, and unbounded AI jobs fail clearly and
require a fresh manual rerun.

Deferred intent captures command id, target identity, actor, queued time,
expiry, policy epoch, auth scope, context hash, data fingerprint, preserved
idempotency-key ref, previewed effect summary, exact replay prerequisites,
state, available replay/cancel/export actions, and sensitive-payload posture.
Raw sensitive payloads are withheld from default support export.

Reconnect must revalidate target, auth, policy, entitlement, version, service
family, context, and data fingerprint before replay. Route, policy, auth,
tenant, region, endpoint, target, version, entitlement, context, data,
missing metadata, or expiry drift opens reconciliation review, narrows the
intent to a safer local posture, or discards it with typed disclosure instead
of replaying invisibly.
