# Maintenance / failover / reconciliation window contract

This document freezes Aureline's **planned-operation** surfaces: the maintenance,
read-only, drain, migration, failover, and post-window reconciliation **windows**
that make a managed boundary crossing feel *different* from random breakage. Where
the [operator-surface matrix](./m5-operator-surfaces.md) freezes the surface
*families* — including the maintenance notice and the failover notice — and the
[response panes](./m5-response-panes.md) build the first *response* surfaces, this
contract builds the windows a user sees *before* and *while* they cross a planned
boundary. Each window binds a matrix surface family by that matrix's own surface
id, so a maintenance or failover notice renders the shared surface contract rather
than a parallel truth model.

The goal is the one in the spec: name the exact scope, time, blocked action
classes, and local-safe alternatives before users cross the boundary, so planned
maintenance, read-only windows, drain phases, failover, and reconciliation never
read as a generic "something is broken" banner. This contract pins five things:

1. **An exact operational phase and exact times.** A window declares its phase
   (`scheduled`, `read_only`, `drain`, `migration`, `failover`, `reconciling`, or
   `resolved`), an exact start and end timestamp with an IANA time zone and an
   explicit UTC offset, and the latest-refresh stamp and freshness. The phase maps
   to the matrix state vocabulary, and `effective_state` is the computed
   no-silent-green downgrade, so a resolved-but-unconfirmed window never reads as a
   confirmed clear.
2. **Exactly which write classes are blocked.** A window in effect lists the
   blocked write classes and, for each, the local-safe alternative — so an operator
   sees what is refused before they attempt it, not on apply.
3. **What stays safely local and how writes are preserved.** A window names its
   local-safe actions and its write posture — `writes_live`,
   `local_draft_preserved`, `publish_later_queued`,
   `blocked_pending_boundary_recheck`, or `read_only_replay` — so blocked managed
   writes are captured and replayed, never lost.
4. **Changed boundary truth.** A failover or migration that moves the tenant,
   region, residency, key ownership, or endpoint posture restates that axis
   explicitly in its boundary disclosure instead of implying an unchanged route.
5. **Whether queued work must be reviewed before replay.** When queued actions
   would cross a changed or unknown boundary after the window ends, the
   review-before-replay requirement is computed, so the queue is reconciled rather
   than silently replayed against a moved boundary.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update in
the same change.

## Companion artifacts

- [`/schemas/ops/m5-maintenance-windows.schema.json`](../../schemas/ops/m5-maintenance-windows.schema.json)
  — boundary schema for `m5_maintenance_window_set`.
- [`/fixtures/ops/m5-maintenance-windows/canonical_windows.json`](../../fixtures/ops/m5-maintenance-windows/canonical_windows.json)
  — the published canonical set; the freeze gate asserts the in-code builder equals
  it byte-for-byte.
- [`/artifacts/ops/m5-maintenance-windows.md`](../../artifacts/ops/m5-maintenance-windows.md)
  — the human-readable companion (window, timing, blocked-write, and boundary
  tables).
- `crates/aureline-support/src/m5_maintenance_windows/` — the builder, the
  operational-phase model, the computed effective state, the blocked-write and
  publish-later continuity model, the changed-boundary disclosure, the computed
  review-before-replay rule, validation, and the human-readable projection.
- `cargo run -p aureline-support --example dump_m5_maintenance_windows` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Operational phases

A window is in exactly one phase at a time, and each phase maps to one matrix
[state](./m5-operator-surfaces.md#unified-state-vocabulary):

| Phase | Matrix state | Blocks managed writes |
| --- | --- | --- |
| `scheduled` | `scheduled_window` | no (not yet in effect) |
| `read_only` | `read_only_window` | yes |
| `drain` | `drain_window` | yes (new actions queue) |
| `migration` | `migration_in_progress` | yes |
| `failover` | `failover_in_progress` | yes |
| `reconciling` | `reconciling` | yes (until review) |
| `resolved` | `clear` | no |

The window `kind` selects the matrix surface family: `planned_maintenance` binds
the **maintenance notice**, while `regional_failover` and `tenant_migration` bind
the **failover notice**. A phase must be valid for the kind (for example a planned
maintenance window is never in the `failover` phase); `validate()` rejects an
invalid pairing.

`effective_state` is computed from the phase's matrix state and the latest-refresh
freshness with the shared no-silent-green rule, so a `resolved` window whose last
refresh is `stale`, `very_stale`, or `never` is downgraded from `clear` to
`unconfirmed` rather than asserting an unverified all-clear.

## Exact times and time zones

Each window carries a `window_time` with an exact `starts_at` and `ends_at`
(RFC3339 with an explicit UTC offset), the IANA `time_zone` it is announced in
(for example `America/New_York` or `Europe/Berlin`), the matching `utc_offset`, an
`end_is_estimated` flag for in-progress windows, and the `latest_refresh_at` stamp
with its `refresh_freshness`. `validate()` parses the timestamps, requires an
explicit offset, requires the timestamps to agree with the stated offset, and
requires `starts_at <= ends_at`. This is the guardrail made executable: no window
relies on a vague relative time, because these flows cross geographies and support
handoffs.

## Blocked write classes and local-safe continuity

A window that blocks managed writes lists the blocked write classes —
`managed_settings_apply`, `managed_policy_change`, `provider_mutation`,
`remote_workspace_write`, `authority_change`, `ticket_or_incident_publish`, or
`release_publish` — and, for each, the `local_alternative` that preserves the
operator's work. The window's `write_posture` and `publish_later_available` flag
state how blocked writes are kept: queued to publish later, held as a local draft,
or blocked pending a boundary recheck. Every window also lists at least one
`local_safe_action`, so an impaired managed boundary never reads as "the whole
product is down".

## Changed-boundary disclosure

A failover or migration window discloses each boundary axis it touches —
`tenant`, `region`, `residency`, `key_ownership`, or `endpoint_identity` — with its
state (`unchanged`, `changed`, or `unknown`) and a short reviewable sentence. Any
`changed` or `unknown` axis sets the disclosure's `recheck_required` flag and must
carry a non-empty disclosure sentence. A failover/migration window with no
disclosed axis is rejected, so a boundary crossing can never present as an
unchanged route.

## Review before replay

`replay_review.required` is the executable form of the spec's review-before-replay
rule: it is `true` exactly when the window has queued actions *and* it crossed a
changed or unknown boundary. A required review names a `trigger`
(`changed_policy`, `changed_tenant`, `changed_region`, `changed_residency`,
`changed_endpoint`, or `changed_authority`) and a `reconcile_action`. When the
boundary is unchanged, a queue replays without review even if actions are queued;
when the boundary moved, the queue must be reconciled against the new
tenant / region / endpoint / authority before it replays.

## Export safety

The record is support-export safe: it carries no endpoint URLs, hostnames,
credentials, raw payloads, or absolute paths — only opaque `aureline://` object
handles, repo-relative refs, stable tokens, exact timestamps, and short reviewable
sentences. `is_support_export_safe()` enforces the boundary, and `validate()`
re-checks it, so desktop, companion, service-health, and support/export flows can
embed the set verbatim.

## Consumers

Desktop shell UI, CLI/headless inspect, the incident workspace, support export,
managed-service surfaces, and the companion/browser surface all render this one set
instead of restating maintenance, failover, or reconciliation truth by hand. The
checked-in descriptors here are the canonical M5 source for planned-operation
window truth; downstream service-health, help/About, support, companion, and
release surfaces should consume them directly.
