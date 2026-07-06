# M5 remote-target-pill / environment-status-strip primitive contract

Task M05-854 · Batch B100 (runtime-boundary and repair components) · crate
`aureline-shell`.

This document is the human-readable contract for the one reusable **remote-target
pill / environment-status strip primitive**. It narrows the `remote_target_pill` and
`environment_status_strip` families frozen by the
[M05-852 runtime-boundary component matrix](m5_runtime_boundary_components_contract.md)
into a working primitive with a real resolver, so a user can tell — from the same
place they launch work — which target and runtime won, where the value came from, and
whether the current state is ready, degraded, or blocked, instead of inferring the
active host / runtime from unrelated logs or settings panels.

The authoritative gate is the Rust validator and resolver in
`crates/aureline-shell/src/implement_the_m5_remote_target_pill_and_environment_status_strip_runtime_source_readiness_and_context_entrypoint_primitive/`.
The export-safe boundary schema is
[`schemas/ui/m5-remote-target-pill.schema.json`](../../schemas/ui/m5-remote-target-pill.schema.json),
with a companion component schema at
[`schemas/ui/m5-environment-status-strip.schema.json`](../../schemas/ui/m5-environment-status-strip.schema.json).
This doc explains intent; the code and schema are the truth.

## The two halves

1. **A resolver** — `resolve_run_context(&M5RunContextResolutionInput) ->
   Result<M5ResolvedRunContext, M5RunContextResolutionError>`. It takes one run
   context's target identity, host boundary, remote connection state, resolved
   runtime kind and label, winning runtime source, resolved scope, and
   effective-value provenance, and derives:
   - the **remote-target posture** — local-inline, connected-healthy, establishing,
     reconnecting, offline-cached, or disconnected, so a degraded or disconnected
     remote target is never masked as connected;
   - the **environment readiness** — the headline verdict a user reads before
     launching work: ready, degraded-cached, degraded-narrowed,
     degraded-unreachable-target, blocked-by-policy, or blocked-unresolved.
2. **A parity matrix** — `M5RemoteTargetEnvironmentPrimitivePacket` — binding one row
   per claimed M5 run-capable surface to the same pill anatomy, strip anatomy, target
   postures, readiness states, provenance states, scopes, export fields, and
   non-visual accessibility routes, so the source / scope / readiness truth stays
   identical everywhere and the support export reconstructs target and runtime
   resolution from one shared model.

## Run-capable surfaces (matrix rows)

The acceptance criteria require identical state vocabulary across nine run-capable
surfaces, each a row in the matrix:

- **Run Console** — `run_console`
- **Test Runner** — `test_runner`
- **Debug Session** — `debug_session`
- **Notebook Runtime** — `notebook_runtime`
- **Request Runner** — `request_runner`
- **Database Session** — `database_session`
- **Preview Server** — `preview_server`
- **Pipeline Run** — `pipeline_run`
- **Incident Surface** — `incident_surface`

## Derived truth

### Remote-target posture (the pill)

`LocalInline` for a local host; otherwise derived from the remote connection state:
`Connected → ConnectedHealthy`, `Connecting → Establishing`,
`Reconnecting → Reconnecting`, `Disconnected → Disconnected`,
`OfflineCached → OfflineCached`. A posture of reconnecting, offline-cached, or
disconnected is **degraded** and must show a degraded / reconnect cue. A local host
must carry no connection state; a remote host must carry one.

### Environment readiness (the strip)

Derived by a priority ladder over the effective-value provenance and the target's
reachability:

1. `PolicyBlocked → BlockedByPolicy`
2. `Unresolved → BlockedUnresolved`
3. `CachedOffline → DegradedCached`
4. `NarrowedApproximate → DegradedNarrowed`
5. a resolved value on a remote target that is reconnecting or disconnected →
   `DegradedUnreachableTarget`
6. otherwise → `Ready`

Only a cleanly resolved, reachable value is `Ready`. A cached, narrowed, or
policy-blocked effective value is therefore **never presented as cleanly ready**, even
when the effective value is the one in use — this is the core acceptance criterion.

### The "Why this context?" entrypoint

Every resolved run context exposes the one-step `Why this context?` entrypoint
(`exposes_why_context_entrypoint` is always `true`), and every row must carry the
`why_this_context_entrypoint` strip part and the `why_context_entrypoint` export
field. Target identity and runtime resolution remain inspectable from the same place
the user launches work.

## Acceptance criteria mapping

- *Users no longer need to infer the active host/runtime from unrelated logs or
  settings panels.* — Every row's mandatory pill parts (`target_identity`,
  `host_or_environment_class`, `connection_state`) and strip parts (`runtime_kind`,
  `resolved_label_version`, `winning_source`, `readiness_state`,
  `why_this_context_entrypoint`) surface the boundary and source inline; hard
  invariants forbid masking the boundary or hiding the entrypoint.
- *Run-capable surfaces expose the same source/scope/readiness truth even when the
  effective value is cached, narrowed, or policy-blocked.* — The
  `cached_or_narrowed_readiness_unproven` and `policy_blocked_readiness_unproven`
  lints require worked resolutions proving degraded / blocked readiness; the shared
  vocabulary set is frozen identically for every surface.
- *Target identity and runtime resolution remain inspectable from the same place the
  user launches work.* — The `why_this_context_entrypoint` part is mandatory and the
  `hides_why_this_context_entrypoint` invariant must be `false` on every row.

## Hard invariants

Each row asserts (all MUST be `false`):

- `masks_host_or_environment_boundary`
- `conflates_ready_and_degraded_or_blocked`
- `invents_private_status_grammar`
- `hides_why_this_context_entrypoint`

## Reused vs minted vocabulary

Reused verbatim from the frozen runtime-boundary matrix: `M5HostBoundaryClass`,
`M5RemoteConnectionState`, `M5RuntimeSourceClass`,
`M5RuntimeBoundaryAccessibilityRoute`, `M5RuntimeBoundaryQualificationClass`,
`M5RuntimeBoundaryDowngradeTrigger`. Reused from the frozen shell-zone matrix:
`M5ResponsiveClass`, `M5ShellConsumerSurface`, `M5ShellZoneSlot`, `M5WindowClass`.

Minted here (only what the frozen matrix left implicit about the pill and the strip):
`M5RunCapableSurface`, `M5RemoteTargetPillPart`, `M5EnvironmentStripPart`,
`M5RemoteTargetPosture`, `M5EnvironmentReadiness`, `M5EffectiveValueProvenance`,
`M5ResolvedScope`, `M5RunContextExportField`.

## Worked resolutions and support export

Every row carries `example_resolutions` — a resolver input paired with the resolved
truth. The stored resolution must equal a fresh resolve of its input
(`example_resolution_drift` otherwise), so the checked-in support export at
`artifacts/release/m5-remote-target-environment-proof/support_export.json`
reconstructs target and runtime resolution from one shared model. Raw URLs,
endpoints, usernames, hostnames, tokens, and credentials never cross this boundary;
target identities, runtime kinds, and resolved runtime labels are carried only as
opaque, export-safe representations.
