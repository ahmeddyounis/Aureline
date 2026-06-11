# Runbook Execution Surface Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/implement-runbook-execution-rows-deviation-notes-export-bundles-and-browser-or-vendor-console-handoff-truth.schema.json`.

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
cargo run -p aureline-companion --example dump_runbook_execution_surface -- relay_down
```

## host_inactive_surface.json

A surface where no active desktop host session exists, so every desktop handoff that
requires an active host downgrades from `exact` to `unresolved` while
host-independent handoffs stay `exact`. The execution-row section narrows because an
approved action can no longer be relayed and applied. `degraded_labels` records
`host_session_inactive` and `handoff_target_unresolved`. Demonstrates that an exact
desktop handoff is never claimed when it can no longer resolve.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_runbook_execution_surface -- host_inactive
```

## incident_attribution_lost_surface.json

A surface where incident attribution to evidence and build identity was lost, so
every execution row and deviation note narrows to `unattributed` and the
execution-row and deviation-note sections narrow one step. `degraded_labels` records
`incident_attribution_lost`. Demonstrates that the execution surface never claims a
provenance it can no longer prove.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_runbook_execution_surface -- attribution_lost
```

## export_incomplete_surface.json

A surface where the export is incomplete, so every `ready` bundle narrows to
`partial`, each resulting incomplete bundle is labeled with `incomplete_label_shown`,
and the export-bundle section narrows one step. `degraded_labels` records
`export_bundle_incomplete`. Demonstrates that an incomplete export bundle is recorded
as a first-class fact rather than claimed complete.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_runbook_execution_surface -- export_incomplete
```

## external_unreachable_surface.json

A surface where the browser or vendor console is unreachable, so every external
handoff downgrades from `exact` to `unresolved` while the local desktop fallback
stays `exact`, and the external-handoff section narrows one step. `degraded_labels`
records `external_handoff_unavailable`. Demonstrates that losing provider continuity
narrows the external claim but never strands the user from the local-first desktop
path.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_runbook_execution_surface -- external_unreachable
```
