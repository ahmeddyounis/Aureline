# Incident Workspace Surface Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/add-incident-workspace-headers-evidence-timelines-resource-slices-and-runbook-packets.schema.json`.

## relay_unavailable_surface.json

A surface where the companion relay is unavailable, so every section narrows one
qualification step (stable → beta, beta → preview, preview → experimental) and one
rollout step, and every live/cached item is forced to `stale` with
`stale_label_shown` set. The scope, attribution, stale-state honesty, security
review, and consumer projection blocks stay fully satisfied, and `degraded_labels`
records `relay_unavailable` and `freshness_downgraded_to_stale`. Demonstrates that
losing the relay narrows the claim and downgrades freshness honestly instead of
showing stale state as live.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_incident_workspace_surface -- relay_down
```

## host_inactive_surface.json

A surface where no active desktop host session exists, so every handoff that
requires an active host downgrades from `exact` to `unresolved` while
host-independent handoffs stay `exact`. The runbook section narrows because an
approved action can no longer be relayed and applied. `degraded_labels` records
`host_session_inactive` and `handoff_target_unresolved`. Demonstrates that an exact
desktop handoff is never claimed when it can no longer resolve.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_incident_workspace_surface -- host_inactive
```

## incident_attribution_lost_surface.json

A surface where incident attribution to evidence and build identity was lost, so
every header and evidence span narrows to `unattributed` and the header and
evidence-timeline sections narrow one step. `degraded_labels` records
`incident_attribution_lost`. Demonstrates that an incident packet never claims a
provenance it can no longer prove.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_incident_workspace_surface -- attribution_lost
```

## evidence_incomplete_surface.json

A surface where the evidence timeline is incomplete, so every present span narrows
to `partial`, each resulting gap is labeled with `gap_label_shown`, and the
evidence-timeline section narrows one step. `degraded_labels` records
`evidence_incomplete`. Demonstrates that a missing or partial evidence span is
recorded as a first-class fact rather than silently hidden.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_incident_workspace_surface -- evidence_incomplete
```
