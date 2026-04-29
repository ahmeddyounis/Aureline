# Scheduled-maintenance, drain-window, tenant-migration, and failover-communication contract

This document freezes the object model Aureline uses to communicate
**planned operations** — scheduled maintenance windows, scheduled
read-only periods, scheduled drain windows, scheduled export freezes,
post-window reconciliation, tenant migration, regional or control-plane
failover, drain-before-failover, and post-event reconciliation — to
users and admins **before, during, and after** the event alters their
effective environment.

The goal is to make planned operations communication impossible to
confuse with incident banners or generic offline copy. A user must be
able to read the notice or the event card and tell, at a glance:

- whether this is planned maintenance, a tenant migration, a drain
  before failover, an in-flight failover, or a post-event reconciliation;
- exactly when the window starts and ends, in UTC, in an IANA display
  timezone, and at the UTC offset that applied at window start or
  cutover;
- which deployment profiles, tenants, orgs, workspaces, regions,
  residency scopes, services, and endpoints are affected;
- which managed action classes are blocked or deferred and through
  which deferral path (publish-later, reconnect-later,
  export-before-maintenance);
- what still works locally — the retained local-safe capability set
  the local-first core continues to support;
- which boundary axes (tenant, region, residency, key ownership,
  endpoint identity) changed or require recheck after migration or
  failover, and whether a fresh approval or a route review is required
  before managed routes resume;
- whether the displayed notice is current, stale, or a historical
  record retained because a boundary changed.

If this document, the companion schemas, and the worked fixtures
disagree, the normative sources in `.t2/docs/` win and this document
plus its companions update in the same change.

## Companion artifacts

- [`/schemas/ops/maintenance_notice.schema.json`](../../schemas/ops/maintenance_notice.schema.json)
  — boundary schema for `maintenance_notice_record`. Carries the
  planned-operations notice: scheduled, pre-window export freeze,
  read-only window, drain window, post-window reconciliation, and
  completed states with their immutable lifecycle and retention.
- [`/schemas/ops/tenant_migration_event.schema.json`](../../schemas/ops/tenant_migration_event.schema.json)
  — boundary schema for `tenant_migration_event_record`. Carries the
  migration / drain-before-failover / failover / reconciliation card,
  including the closed list of boundary axes, the fresh-approval and
  route-review posture under the new boundary, and the
  post-event continuity restate.
- [`/fixtures/ops/maintenance_cases/`](../../fixtures/ops/maintenance_cases/)
  — worked fixtures exercising the contract.

This contract composes with (and does not replace):

- [`/docs/ux/control_data_plane_status_contract.md`](../ux/control_data_plane_status_contract.md)
  and
  [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  — the unified status-strip / banner record that handles the full
  notice union (planned plus unplanned). The
  `outage_notice_record` contract owns the cross-surface rendering
  rules; the records frozen here are the *operational planned-event*
  surfaces that ops uses before, during, and after the boundary
  change.
- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  — control-plane service classes, local data-plane capability classes,
  and the local-core continuity vocabulary every notice quotes.
- [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md)
  — DR / continuity / impairment drill catalog the notices and events
  cite by reference.
- [`/docs/ops/incident_workspace_contract.md`](./incident_workspace_contract.md)
  and
  [`/schemas/ops/incident_workspace.schema.json`](../../schemas/ops/incident_workspace.schema.json)
  — the read-mostly diagnosis and mitigation workspace for *unplanned*
  incidents. Planned operations communication intentionally lives on a
  separate object family so an incident workspace cannot reuse
  scheduled-maintenance language, and a maintenance notice cannot
  inherit incident severity or action-ledger semantics.
- [`/docs/ops/event_provenance_and_route_inspector_contract.md`](./event_provenance_and_route_inspector_contract.md)
  and
  [`/schemas/ops/route_timeline.schema.json`](../../schemas/ops/route_timeline.schema.json)
  — route-drift, approval-renewal, and replay rules every migration /
  failover event composes with when boundary facts change.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` operational supportability,
  managed-service maintenance, tenant migration, residency, and
  continuity passages.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  control-plane / data-plane separation, mirror posture,
  fault-domain, region, and tenant-routing passages.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` operational notice,
  boundary review, status strip, and post-event copy passages.

If this document disagrees with those sources, those sources win and
this document plus the companion schemas update in the same change.

## Why this exists

Operational communication otherwise reaches for one of three failure
modes:

1. **Generic offline banners.** A scheduled maintenance window, a
   tenant migration, a regional failover, and a true outage all collapse
   into the same "Service is currently unavailable" copy. Users cannot
   tell whether they should continue local work, export now, retry
   later, or review a changed boundary.
2. **Incident-shaped maintenance.** A planned maintenance is announced
   in incident voice ("we are investigating an issue"). Users mistake
   planned downtime for a live incident, and admin review surfaces
   accumulate scheduled maintenances inside the incident-workspace
   chronology where they do not belong.
3. **Hidden boundary changes.** A tenant migration silently moves the
   active route to a new region, residency posture, key-ownership
   route, or endpoint identity. The "we're back!" notice does not
   restate which boundary axis changed, so cached approvals, replay
   queues, and reconnect tokens are reused under a boundary the user
   never reviewed.

This contract closes those gaps by freezing two operational object
families:

- **`maintenance_notice_record`** — the planned-operations notice for
  scheduled maintenance windows, scheduled read-only periods,
  scheduled drain windows, scheduled export freezes, and post-window
  reconciliation. The notice cannot use incident or generic-offline
  language, and a completed notice MUST remain inspectable as a
  historical record.
- **`tenant_migration_event_record`** — the migration /
  drain-before-failover / failover / reconciliation card. The card
  MUST restate every boundary axis (tenant, region, residency, key
  ownership, endpoint identity) and MUST require a fresh approval or
  route review before managed routes resume across a changed boundary.
  A completed event with a boundary change MUST NOT render as a
  generic all-clear.

## Scope

Frozen at this revision:

- one `maintenance_notice_record` carrying notice id,
  `maintenance_kind_class`, `maintenance_state_class`, exact
  schedule with display timezone and UTC offset at window start,
  affected scope, separate control-plane / data-plane plane effects,
  retained local-safe continuity, blocked managed action classes with
  explicit deferral paths and resume triggers, follow-up action rows
  bound to a `pre_window` / `in_window` / `post_window` phase, the
  `post_window_restate` block (cached-status posture, boundary-change
  required class, linked migration-event refs), notice lifecycle, and
  the display-copy invariants that forbid incident language, generic
  offline copy, and "all work broken" copy;
- one `tenant_migration_event_record` carrying event id,
  `event_kind_class`, `event_state_class`, exact schedule with
  cutover time and the UTC offset at cutover, affected scope, the
  closed `boundary_axes` list (tenant, region, residency, key
  ownership, endpoint identity), `approval_and_route_review`
  posture, `post_event_continuity` block (with the cached-status
  posture so a stale snapshot cannot be confused with the live
  state), blocked managed action classes, follow-up action rows
  bound to a `pre_cutover` / `during_event` / `post_cutover` phase,
  event lifecycle (with linked maintenance-notice refs), and the
  display-copy invariants that forbid incident language, "all work
  broken" copy, and generic all-clear copy;
- the closed action vocabularies for managed actions, deferral paths,
  resume triggers, follow-up actions, and boundary review status;
- the rule that historical maintenance notices and historical
  migration events remain inspectable after a boundary change and
  carry an explicit stale label.

Out of scope at this revision (named explicitly so reviewers know what
is *not* being decided here):

- orchestrating maintenance, tenant migration, region failover, or
  reconciliation. The records freeze the *communication boundary*; the
  orchestration plane lives elsewhere;
- live wiring against any specific status-page provider, paging
  provider, observability backend, or admin console;
- final user-facing copy, status-page integration, notification
  routing, animation, or iconography. Surfaces compose against the
  records frozen here;
- replacement of the unified `outage_notice_record` rendering contract
  in `docs/ux/control_data_plane_status_contract.md`. That contract
  stays the cross-surface render record for the status strip family;
  the records frozen here are the *planned-event* operational
  surfaces ops uses to publish those notices.

## Notice kind, state, and required disclosure

A `maintenance_notice_record` carries exactly one
`maintenance_kind_class` and one `maintenance_state_class`.

| Kind | Meaning |
| --- | --- |
| `scheduled_maintenance_window` | A future maintenance window has been announced. |
| `scheduled_read_only_window` | A planned read-only period (writes paused, reads available). |
| `scheduled_drain_window` | A planned drain window: existing sessions finish, new starts are blocked. |
| `scheduled_export_freeze` | A scheduled freeze where users MUST export before the window if work must survive it. |
| `post_maintenance_reconciliation` | A planned reconciliation where cached state, replay queues, and managed acknowledgements are compared before the window is closed. |

| State | Required disclosure |
| --- | --- |
| `announced` | Exact UTC start / end, display timezone, UTC offset at start, affected scope, planned blocked actions, and any required pre-window export deadline. |
| `pre_window_export` | Export-before-maintenance follow-up MUST be present, with a non-null `due_before_at` ≤ `schedule.export_deadline_at`. |
| `scheduled_window` | Window start / end, blocked action classes, deferral paths, retained local-safe capabilities, and the post-window restate plan. |
| `read_only_window` | At least one blocked action class with a non-`not_applicable` deferral path. Local capture posture and retained local-safe capabilities are required. |
| `drain_window` | At least one blocked action class with `blocked_drain_new_actions` or `draining_existing_only`, and `drain_deadline_at` set on the schedule. Reconnect-later guidance is required when new joins are blocked. |
| `reconciling_after_window` | `post_window_restate.cached_status_class` MUST resolve to one of `cached_pre_window_stale`, `cached_managed_acknowledgement_pending`, or `cached_replay_review_pending`, and at least one of publish-later, reconnect-later, review-new-boundary, or open-history follow-up is required. |
| `completed` | The window is closed. Lifecycle freshness MUST resolve to a non-`active_current` value and `display_copy.stale_label` MUST be non-null. |

## Migration and failover event kinds and required disclosure

A `tenant_migration_event_record` carries exactly one
`event_kind_class` and one `event_state_class`.

| Kind | Meaning |
| --- | --- |
| `tenant_migration` | Tenant authority is moving. |
| `region_migration` | Control-plane region is moving. |
| `residency_migration` | Residency posture is moving. |
| `key_ownership_migration` | Key-ownership route is moving. |
| `endpoint_migration` | Endpoint identity is moving. |
| `drain_before_failover` | Existing sessions are draining ahead of an upcoming failover. |
| `regional_failover` | Service is rerouting through an alternate region. |
| `control_plane_failover` | Service is rerouting the control plane through an alternate fault domain. |
| `post_event_reconciliation` | A planned reconciliation following a migration or failover, comparing cached state and replay queues before the event is closed. |

| State | Required disclosure |
| --- | --- |
| `announced` | Cutover time, display timezone, UTC offset at cutover, affected scope, the boundary axes that will change, and the pre-cutover export window if applicable. |
| `drain_before_failover` | Drain start time, the cutover target, blocked-new-action posture, retained local-safe capabilities, and review-new-boundary follow-up when the post-cutover boundary differs. |
| `in_progress_migration` | Active scope, the closed list of boundary axes, the review-new-boundary follow-up, and approval-and-route-review posture. |
| `in_progress_failover` | Active scope, the closed list of boundary axes including `unknown_recheck_required` axes, the review-new-boundary follow-up, and approval-and-route-review posture. |
| `reconciling` | `post_event_continuity.cached_status_class` MUST resolve to a non-`cached_resolved_consistent` value, and at least one of publish-later, reconnect-later, review-new-boundary, or open-history follow-up is required. |
| `completed_boundary_changed` | Lifecycle freshness MUST resolve to a non-`active_current` value, `display_copy.stale_label` MUST be non-null, `display_copy.generic_all_clear_used` MUST remain false, and the boundary axes MUST restate every changed axis with `previous_ref` and `current_ref`. |
| `completed_boundary_unchanged` | Lifecycle freshness resolves to `completed_historical`, `display_copy.stale_label` is non-null, and the boundary axes record `unchanged` for every axis. |

## Required disclosure on every notice and event

Every `maintenance_notice_record` and every `tenant_migration_event_record`
MUST carry:

- exact UTC times for announce / start (or cutover) / expected or actual
  end / completed, plus an IANA timezone id and the UTC offset that
  applied at window start or cutover;
- affected scope listing deployment profiles, tenant / org / workspace
  refs, region refs, residency-scope classes, control-plane service
  classes, and endpoint refs;
- separate control-plane and data-plane effect lists (on the
  maintenance notice; the migration event composes the same posture
  through `post_event_continuity` and the boundary-axes block);
- a retained local-safe capability list (at least one product-term
  sentence such as "continue local edits, save files, search, and
  export pending work");
- blocked managed action classes, each with a typed `block_state_class`,
  `deferral_path_class`, `local_capture_class`, and `resume_trigger_class`,
  plus an optional opaque `affected_queue_ref`;
- follow-up action rows from the closed vocabulary, each with a
  `phase_class` (`pre_window` / `in_window` / `post_window` for the
  notice; `pre_cutover` / `during_event` / `post_cutover` for the
  event), a `requirement_class`, and an optional `due_before_at`.

Raw URLs, raw hostnames, raw IP addresses, raw tenant names, raw account
ids, raw endpoint credentials, raw policy bodies, and raw secret material
do not cross either boundary. Records carry opaque refs and
redaction-aware sentences only.

## Local-safe continuity is mandatory

Every notice and every event MUST resolve `local_continuity` /
`post_event_continuity`:

- `local_core_status_class` resolves to one of
  `local_core_unaffected`, `meaningful_safe_subset_available`,
  `local_only_available`, `no_safe_local_subset`, or
  `unknown_requires_review`. The default for managed maintenance and
  managed migration is `meaningful_safe_subset_available`.
- `retained_local_safe_capabilities` MUST contain at least one
  product-term sentence. The notice cannot let "what still works"
  be inferred from the absence of a blocked action.
- `blocked_managed_only_capabilities` lists the managed-only paths
  that are paused so users can tell what is *managed* (paused) versus
  what is *local-first* (unaffected).
- `display_copy.all_work_broken_implied` MUST remain false.

The migration event additionally pins `cached_status_class` so a stale
snapshot of pre-cutover state cannot be confused with the live state.
`cached_pre_event_stale` and `cached_managed_acknowledgement_pending`
require an explicit follow-up to publish-later, reconnect-later, or
review-new-boundary before the cached snapshot is treated as live.

## Boundary axes and approval / route review

A `tenant_migration_event_record` MUST list **every** boundary axis
(tenant, region, residency, key ownership, endpoint identity) in
`boundary_change.boundary_axes`. Each axis carries:

- `axis_state_class`: `unchanged`, `changed`, `unknown_recheck_required`,
  or `not_applicable`;
- `previous_ref` / `current_ref`: opaque refs (or null) for the prior
  and new boundary;
- `required_follow_up_action`: which follow-up class admits the resume
  posture for that axis (`review_new_boundary` is the default for any
  `changed` or `unknown_recheck_required` axis);
- `summary`: a redaction-aware sentence describing the change.

`unknown_recheck_required` is *not* a default-quiet state. It forces
`boundary_change_required = true`, forces `review_status_class` to a
non-`not_required` value, and forces a `review_new_boundary`
follow-up.

`approval_and_route_review` then pins:

- `fresh_approval_required_class`: when boundary changes affect managed
  lifecycle, policy admin, publish writes, remote attach, or all
  managed routes;
- `route_review_required_class`: when boundary changes require
  data-plane reconnect, control-plane reconnect, replay queue resume,
  token reissue, or all managed routes;
- `replay_queue_review_refs` / `reconnect_token_review_refs`: opaque
  refs to queues and tokens that MUST NOT replay or be reused under
  the new boundary until reviewed.

When `boundary_change_required` is true, the schema enforces that
`fresh_approval_required_class` is non-`not_required`. A migration or
failover event with a changed boundary cannot resume managed routes
"automatically".

## Read-only versus drain versus migration versus failover versus reconciled

The five operational states are intentionally distinct from one
another and from unplanned degradation:

- **`read_only_window`** (maintenance notice) — a planned period when
  reads remain available but managed write classes are paused.
  Local-first writes are unaffected. Deferral paths admit publish-later
  for queued intents and export-before-maintenance for intents that
  must survive the window.
- **`drain_window`** (maintenance notice) — existing sessions or in-
  flight writes may finish, but new starts and joins are blocked.
  `drain_deadline_at` is required. The reconnect-later follow-up is
  required when new joins are blocked. Local Git, search, edit, and
  export remain available.
- **`in_progress_migration`** (migration event) — tenant, residency,
  region, key ownership, or endpoint identity is moving. The closed
  list of boundary axes is restated, every changed or
  `unknown_recheck_required` axis points to a `review_new_boundary`
  follow-up, and managed lifecycle / publish writes wait for fresh
  approval and route review.
- **`in_progress_failover`** (migration event) — the control plane or
  region has rerouted through an alternate fault domain. The boundary
  axes record what changed under the failover (often region, key
  ownership route, and endpoint identity), and the
  approval-and-route-review posture is required before managed routes
  resume.
- **`reconciling`** (either family) — cached state, replay queues, and
  managed acknowledgements are being compared before the window or
  event is closed. `cached_status_class` resolves to a non-consistent
  value and at least one publish-later, reconnect-later,
  review-new-boundary, or open-history follow-up is required.

Unplanned degradation continues to flow through the unified
`outage_notice_record` (control-plane / data-plane status contract).
This contract intentionally does **not** admit an
`unplanned_degradation` kind on the maintenance notice and does not
admit a generic-incident state on the migration event. Surfaces that
need to render the full union (planned + unplanned) compose against
the `outage_notice_record` projection of these planned events; surfaces
that ops uses to publish, queue, and review planned operations compose
against the records frozen here.

## Historical retention and stale labelling

Maintenance notices and migration events remain inspectable after the
window or event closes. The `lifecycle.freshness_class` vocabulary
forbids silent deletion:

- `active_current` — the notice or event is in flight.
- `superseded_stale` — a newer notice or event replaced this one;
  `superseded_by_*_id` is non-null and the prior record is preserved.
- `completed_historical` — the window or event closed; the record is
  retained because users may still need to inspect what happened.
- `imported_historical` — the record was imported from a historical
  bundle and is not live.

A `freshness_class` other than `active_current` requires a non-null
`display_copy.stale_label`. A historical migration event with a changed
boundary continues to surface the `review_new_boundary` follow-up for
users who have not yet reviewed the new boundary; the
`display_copy.generic_all_clear_used` invariant remains false until
every boundary axis is reviewed.

The maintenance notice composes with the migration event by reference
through `post_window_restate.linked_migration_event_refs`; the
migration event composes back through
`lifecycle.linked_maintenance_notice_refs`. A reviewer can navigate
from a completed maintenance window to the migration event that
crossed a boundary during the window, and back.

## Display-copy invariants

Every record's `display_copy` block MUST keep three invariants false:

- `all_work_broken_implied` — a planned maintenance or migration cannot
  imply that all work is broken when the local-first core still
  supports a meaningful safe subset.
- `incident_language_used` — planned maintenance and migration events
  cannot reuse incident or unplanned-degradation language. Banners,
  notifications, support exports, and admin review surfaces that render
  these records cannot collapse them into incident voice.
- `generic_offline_banner_used` (maintenance notice) /
  `generic_all_clear_used` (migration event) — the closing copy
  cannot be a generic offline banner or a generic all-clear when the
  local-first core remains available or when a boundary changed.

The schema enforces these invariants as `const: false` so a fixture or
record that flips them is invalid by construction.

## Worked fixtures

See [`/fixtures/ops/maintenance_cases/`](../../fixtures/ops/maintenance_cases/)
for worked fixtures exercising the contract:

- `read_only_window_publish_later.yaml` — a scheduled read-only window
  with publish-later deferral for blocked managed writes;
- `drain_before_failover.yaml` — a planned drain window ahead of a
  regional failover, with the failover surfacing as a linked
  `tenant_migration_event_record`;
- `tenant_migration_new_region.yaml` — a tenant migration to a new
  region with residency change and review-new-boundary follow-up;
- `cached_stale_status_after_event.yaml` — a post-event reconciliation
  card with `cached_pre_event_stale` posture and a stale label;
- `export_before_maintenance.yaml` — a scheduled export freeze with a
  required export-before-maintenance follow-up and an explicit export
  deadline.
