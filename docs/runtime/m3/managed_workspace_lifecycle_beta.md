# Beta Managed-Workspace Lifecycle and Lineage

This document is the reviewer-facing landing page for the beta managed-workspace
lifecycle contract: every claimed managed-workspace row mints a typed,
redaction-safe record that title bars, remote strips, activity center cards,
docs/help, and support packets all read so users can answer "is this workspace
starting, live, suspended, resuming, degraded, reconnect-required, retiring, or
retired — and how did it get there?" without forking truth per surface.

The machine-readable boundary lives at
[`/schemas/providers/managed_workspace_lifecycle.schema.json`](../../../schemas/providers/managed_workspace_lifecycle.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/managed_workspace_lifecycle_beta/`](../../../crates/aureline-runtime/src/managed_workspace_lifecycle_beta/).
The alpha primitive this beta layer promotes lives at
[`/crates/aureline-runtime/src/managed_alpha/`](../../../crates/aureline-runtime/src/managed_alpha/).

## The beta promise

- every managed-workspace row mints one
  [`ManagedWorkspaceLifecycleBetaRecord`](../../../crates/aureline-runtime/src/managed_workspace_lifecycle_beta/mod.rs)
  that names the current lifecycle phase, the rendered lifecycle state, the
  local-editing continuity class, and the ordered lineage of prior phases;
- title bars, remote strips, activity center cards, docs/help, and support
  packets all consume the same record through typed
  [`ManagedWorkspaceLifecycleBetaSurfaceProjection`](../../../crates/aureline-runtime/src/managed_workspace_lifecycle_beta/mod.rs)
  rows, so the lifecycle copy never forks per surface;
- when remote authority is paused or gone, the record explicitly names the
  preserved local-editing continuity class — local editing is never silently
  blocked, and writes are never silently leaked to a degraded remote.

## Lifecycle phases

The closed `current_phase` vocabulary names the *phase* the workspace is in.

| Phase | Token | Meaning |
| --- | --- | --- |
| Start | `start` | Workspace is being provisioned, allocated, booted, or attached |
| Ready | `ready` | Workspace is live and accepting normal work |
| Suspend | `suspend` | Compute is paused; no live traffic |
| Resume | `resume` | Resume or reattach completed |
| Reconnect | `reconnect` | Reconnect, reauth, or reattach must complete before mutation |
| Retire | `retire` | Workspace is retiring or has retired; no live reopen path |

## Lifecycle states

The closed `current_state` vocabulary is what surfaces *render*. It is derived
from the phase plus the underlying reachability/witness truth, so adding a new
state is a single decision row.

| State | Token | Admits remote mutation | Notes |
| --- | --- | --- | --- |
| Starting | `starting` | no | Workspace is starting; surfaces show booting copy |
| Live | `live` | yes | Workspace is live and mutation is admitted |
| Suspended | `suspended` | no | Compute paused; local-only continuity preserved when contractually allowed |
| Resuming | `resuming` | no | Resume/reattach in progress |
| Degraded | `degraded` | no | Workspace is reachable but narrowed |
| Reconnect required | `reconnect_required` | no | Reconnect/reauth/reattach must complete |
| Retiring | `retiring` | no | Workspace is retiring; reopen path closing |
| Retired | `retired` | no | Workspace is retired; no live reopen path |

## Local-editing continuity

The `local_editing_continuity` field is a closed vocabulary describing what the
user may still do in the editor while remote authority is paused, narrowed, or
gone. It is the contract that keeps "local editing continuity" honest across
suspend / resume / reconnect / retire transitions.

| Continuity | Token | Meaning |
| --- | --- | --- |
| Preserved (full) | `preserved_full_local_editing` | Local editing continues; saves flow through when state returns to `live` |
| Preserved (local-only writes) | `preserved_local_only_writes` | Local editing continues; writes stay local until the workspace recovers |
| Inspect only | `inspect_only_until_recovery` | Inspection only until the workspace recovers |
| Not applicable | `not_applicable` | Reserved for `starting` and `retired` rows |

## Lineage

The `lineage` array is ordered: every entry names the phase, state, transition
reason, observed-at timestamp, and a review-safe summary. The tail entry's
phase and state MUST equal the record's `current_phase` and `current_state`;
the runtime
[`validate`](../../../crates/aureline-runtime/src/managed_workspace_lifecycle_beta/mod.rs)
method rejects any record that contradicts its own lineage tail.

Support exports therefore never have to reconstruct the path the workspace
took: they read it from the lineage directly.

## Shared row id

A `ManagedWorkspaceLifecycleBetaRecord` exposes a stable `row_id` of the form
`managed-workspace-lifecycle-beta-row:<scoped-name>`. The same id appears in:

- the [`ManagedWorkspaceLifecycleBetaSurfaceProjection`](../../../crates/aureline-runtime/src/managed_workspace_lifecycle_beta/mod.rs)
  rows that title bars, remote strips, activity center cards, docs/help, and
  support packets consume;
- the [`ManagedWorkspaceLifecycleBetaSupportExport`](../../../crates/aureline-runtime/src/managed_workspace_lifecycle_beta/mod.rs)
  bundle support packets embed.

Reviewers and support can therefore cross-reference all surfaces by id without
re-deriving lifecycle truth.

## Reviewer fixtures

The canonical fixture set lives under
[`/fixtures/runtime/m3/managed_workspace_lifecycle/`](../../../fixtures/runtime/m3/managed_workspace_lifecycle/)
and exercises one case per lifecycle phase:

- `start_starting.json` — workspace is starting; not yet live;
- `ready_live.json` — workspace is live; mutation admitted; local editing preserved;
- `suspend_suspended.json` — workspace is suspended; local-only writes
  preserved when contractually allowed;
- `resume_live.json` — workspace just resumed; mutation admitted;
- `reconnect_required.json` — reconnect/reauth required; mutation refused;
- `retire_retired.json` — workspace is retired; not-applicable continuity.

The CI validator lives at
[`/ci/check_managed_workspace_lifecycle_beta.py`](../../../ci/check_managed_workspace_lifecycle_beta.py)
and validates the schema, the manifest coverage, and the lineage tail
consistency.

Run the validator:

```sh
python3 ci/check_managed_workspace_lifecycle_beta.py --repo-root .
```

The integration test that replays the fixtures through the runtime module lives
at
[`/crates/aureline-runtime/tests/managed_workspace_lifecycle_beta.rs`](../../../crates/aureline-runtime/tests/managed_workspace_lifecycle_beta.rs).

## Out of scope for this beta

- Starting a real managed workspace runtime, control plane, or tunnel broker;
  the contract is data-only.
- Carrying raw endpoints, hostnames, paths, ports, credentials, container
  image digests, provider payloads, transport frames, logs, or support
  bundle bodies.
- Redefining the broader managed-workspace lifecycle contract at
  [`docs/managed/managed_workspace_lifecycle_contract.md`](../../managed/managed_workspace_lifecycle_contract.md)
  or the alpha [`managed_alpha`](../../../crates/aureline-runtime/src/managed_alpha/)
  preview/runtime inspection lane; this beta only adds the lifecycle/lineage
  contract.
- M6-class collaborative attach control, cross-org sharing, or
  cloud-control-plane productization beyond the bounded beta foundations.
