# Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity Fixtures

These fixtures are generated deterministically from the first-consumer surface
builder in `aureline-companion` and validate against
`schemas/companion/add-remote-preview-or-session-handoff-light-remote-edit-and-scoped-collaboration-follow-continuity-on-companio.schema.json`.

## relay_unavailable_surface.json

A surface where the companion relay is unavailable, so every surface narrows one
qualification step (stable → beta, beta → preview, preview → experimental) and one
rollout step, and every live/cached item is forced to `stale` with
`stale_label_shown` set. The scope contract, stale-state honesty, continuity
guarantee, security review, and consumer projection stay fully satisfied, and
`degraded_labels` records `relay_unavailable` and `freshness_downgraded_to_stale`.
Demonstrates that losing the relay narrows the claim and downgrades freshness
honestly instead of showing stale state as live, without stranding local work.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_continuity_surface -- relay_down
```

## host_inactive_surface.json

A surface where no active desktop host session exists, so every handoff that
requires an active host downgrades from `exact` to `unresolved`, the affected
remote-preview items report `handoff_unavailable` continuity, and host-independent
handoffs (collaboration-follow into a review/file scope) stay `exact`. The bounded
light-remote-edit surface narrows because a write can no longer be relayed and
applied. `degraded_labels` records `host_session_inactive` and
`handoff_target_unresolved`. Demonstrates that an exact desktop handoff is never
claimed when it can no longer resolve, and that an in-flight handoff degrades to
local-authoritative rather than stranding work.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_continuity_surface -- host_inactive
```

## collaboration_scope_revoked_surface.json

A surface where the host withdrew the shared collaboration scope, so every
collaboration-follow item narrows to `scope_revoked` and the collaboration-follow
surface narrows one step. `degraded_labels` records `collaboration_scope_revoked`.
Demonstrates that collaboration-follow stays confined to a host-revocable scope and
narrows honestly when that scope is withdrawn.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_companion_continuity_surface -- scope_revoked
```
