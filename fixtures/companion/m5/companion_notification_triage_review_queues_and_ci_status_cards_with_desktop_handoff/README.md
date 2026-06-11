# Companion Notification Triage, Review Queues, and CI-Status Cards Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/companion-notification-triage-review-queues-and-ci-status-cards-with-desktop-handoff.schema.json`.

## relay_unavailable_surface.json

A surface where the companion relay is unavailable, so every section narrows one
qualification step (stable → beta, beta → preview) and one rollout step, and every
CI-status card goes `stale`. The handoff contract, security review, and consumer
projection stay fully satisfied, and `degraded_labels` records `relay_unavailable`.
Demonstrates that losing the relay narrows the claim instead of hiding the surface.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_triage_surface -- relay_down
```

## host_inactive_surface.json

A surface where no active desktop host session exists, so every handoff that
requires an active host downgrades from `exact` to `unresolved` while
host-independent handoffs (CI pipeline, incident workspace) stay `exact`.
`degraded_labels` records `host_session_inactive` and `handoff_target_unresolved`.
Demonstrates that an exact desktop handoff is never claimed when it can no longer
resolve.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_triage_surface -- host_inactive
```
