# Connectivity State, Deferred Intent, And Reconciliation

This contract makes connectivity posture one shared model across managed,
provider-linked, request, remote, service-health, diagnostics, and support
surfaces.

Stable states are `Connected`, `Constrained`, `OfflineLocalSafe`,
`ReauthRequired`, `ReconciliationPending`, and `ServiceUnavailable`. Surfaces
must quote these labels rather than inventing local synonyms.

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
expiry, policy epoch, auth scope, context hash, previewed effect summary, state,
available replay/cancel/export actions, and sensitive-payload posture. Raw
sensitive payloads are withheld from default support export.

Reconnect must revalidate target, auth, policy, entitlement, version, service
family, and context before replay. Route, policy, auth, tenant, region,
endpoint, target, version, entitlement, context, missing metadata, or expiry
drift opens reconciliation review or blocks replay.
