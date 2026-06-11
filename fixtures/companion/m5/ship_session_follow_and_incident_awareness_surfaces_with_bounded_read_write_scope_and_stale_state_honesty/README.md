# Companion Session-Follow and Incident-Awareness Surface Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/ship-session-follow-and-incident-awareness-surfaces-with-bounded-read-write-scope-and-stale-state-honesty.schema.json`.

## relay_unavailable_surface.json

A surface where the companion relay is unavailable, so every surface narrows one
qualification step (stable → beta, beta → preview, preview → experimental) and one
rollout step, and every live/cached item is forced to `stale` with
`stale_label_shown` set. The scope contract, stale-state honesty, security review,
and consumer projection stay fully satisfied, and `degraded_labels` records
`relay_unavailable` and `freshness_downgraded_to_stale`. Demonstrates that losing
the relay narrows the claim and downgrades freshness honestly instead of showing
stale state as live.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_scope_surface -- relay_down
```

## host_inactive_surface.json

A surface where no active desktop host session exists, so every handoff that
requires an active host downgrades from `exact` to `unresolved` while
host-independent handoffs (the incident workspace) stay `exact`. The bounded
light-edit surface narrows because a write can no longer be relayed and applied.
`degraded_labels` records `host_session_inactive` and `handoff_target_unresolved`.
Demonstrates that an exact desktop handoff is never claimed when it can no longer
resolve.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_scope_surface -- host_inactive
```

## incident_attribution_lost_surface.json

A surface where incident attribution to evidence and build identity was lost, so
every incident-awareness item narrows to `unattributed` and the incident-awareness
surface narrows one step. `degraded_labels` records `incident_attribution_lost`.
Demonstrates that an incident packet never claims a provenance it can no longer
prove.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_scope_surface -- attribution_lost
```
