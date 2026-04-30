# Failover-continuity banner, local-safe baseline, and queued/retry continuity contract

This document freezes the in-product banner Aureline shows users
during a service-family outage, scheduled maintenance window,
scheduled drain window, scheduled export freeze, tenant or region
migration, regional or control-plane failover, post-event
reconciliation, or a connectivity-degrade. The banner exists so a
user reading the surface knows, at a glance:

- which features are impacted and which upstream notice / event /
  connectivity snapshot is the source of truth;
- what remains productively local-safe **right now** — not what was
  available before the outage, not what will be available after,
  but the launch-critical workflows the user can still run;
- which user-mintable actions are queued for replay, retryable when
  connectivity returns, blocked pending reconnect or boundary
  recheck, blocked with no safe retry, or require a manual rerun
  because reconciliation already drained with conflict;
- which evidence or export path is one click away (export
  diagnostics, export-before-maintenance, open history, open
  boundary details);
- whether the banner or the underlying notice is stale, retained
  historically, or pinned to a boundary that has changed.

The banner is **not** a parallel incident vocabulary. It composes
the upstream `outage_notice_record`, `maintenance_notice_record`,
`tenant_migration_event_record`, and `connectivity_state_snapshot_record`
state vocabularies directly so a banner cannot say something the
underlying notice does not also say.

If this document, the companion schemas, and the worked fixtures
disagree, the normative sources in `.t2/docs/` win and this
document plus its companions update in the same change.

## Companion artifacts

- [`/schemas/ops/failover_banner.schema.json`](../../schemas/ops/failover_banner.schema.json)
  — boundary schema for `failover_banner_record`.
- [`/schemas/ops/local_safe_baseline.schema.json`](../../schemas/ops/local_safe_baseline.schema.json)
  — boundary schema for `local_safe_baseline_record`, the
  inspectable inventory of launch-critical local-first workflows
  every banner cites.
- [`/fixtures/ops/failover_continuity_cases/`](../../fixtures/ops/failover_continuity_cases/)
  — worked fixtures for service-family outage, regional failover
  with a changed boundary, local-safe-only mode, and partial
  queue/retry continuity.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen in
upstream contracts. It quotes them by name and by value:

- [`/docs/ux/control_data_plane_status_contract.md`](../ux/control_data_plane_status_contract.md)
  and
  [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  — the unified outage / maintenance notice. The banner's
  `impairment_source.outage_state_class` is the same enum as
  `notice_state_class` on that record.
- [`/docs/ops/maintenance_migration_failover_contract.md`](./maintenance_migration_failover_contract.md)
  and
  [`/schemas/ops/maintenance_notice.schema.json`](../../schemas/ops/maintenance_notice.schema.json)
  /
  [`/schemas/ops/tenant_migration_event.schema.json`](../../schemas/ops/tenant_migration_event.schema.json)
  — planned operations and migration events. The banner's
  `impairment_source.maintenance_state_class` and
  `impairment_source.migration_state_class` re-export those state
  vocabularies verbatim.
- [`/docs/runtime/connectivity_and_reconciliation_contract.md`](../runtime/connectivity_and_reconciliation_contract.md)
  and
  [`/schemas/runtime/connectivity_state.schema.json`](../../schemas/runtime/connectivity_state.schema.json)
  — per-service-family connectivity and the deferred-intent
  outbox. The banner's `continuity_action_state_class` aligns
  with the outbox-admission posture and the reconciliation
  outcomes; the banner does not invent a parallel queueability
  matrix.
- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  and
  [`/schemas/deployment/local_core_continuity_packet.schema.json`](../../schemas/deployment/local_core_continuity_packet.schema.json)
  — control-plane / data-plane degrade vocabulary,
  retained-local-safe / blocked-managed-only, and freshness
  posture. The local-safe baseline record cites continuity
  packets through `composes_with.local_core_continuity_packet_refs`.
- [`/docs/ux/deployment_summary_contract.md`](../ux/deployment_summary_contract.md)
  — deployment summary card and residual dependency rows. The
  baseline composes against deployment summary cards via
  `composes_with.deployment_summary_card_refs`.

## Why this exists

Failover and outage surfaces otherwise reach for one of three
failure modes:

1. **Generic "service unavailable" banner.** The banner names the
   broken service and stops. Users cannot tell whether they should
   keep editing, export now, retry later, review a changed
   boundary, or stop work entirely.
2. **Queue-and-block collision.** The same banner copy covers
   queued idempotent intents, retryable rejections, blocked
   actions waiting on a boundary recheck, and actions that already
   drained with conflict and now need a manual rerun. Reviewers
   cannot tell which queue an item is in or what the next safe
   user step is.
3. **Local-safe invisibility.** The banner mentions only the
   impaired service. The user does not know that local edit, save,
   search, Git, export, and diagnostics still work, so they stop
   working entirely while a managed plane is degraded.

This contract closes those gaps by freezing two operational object
families:

- **`local_safe_baseline_record`** — the inspectable inventory of
  launch-critical local-first workflows the deployment claims.
  The baseline is what banners and support exports cite when they
  say "local edit, save, search, Git, export, and diagnostics
  remain available." Inferring local-safe capability from the
  absence of an impacted feature is non-conforming.
- **`failover_banner_record`** — the in-product continuity banner.
  Every banner pins exactly one source (or composite source),
  references exactly one local-safe baseline, lists impacted
  features with the upstream service vocabulary, narrates every
  user-mintable action class with a typed continuity-action state,
  and surfaces the evidence/export path. The display-copy
  invariants forbid a generic unavailable banner, queued/blocked
  collision, local-safe invisibility, "all work broken" copy, and
  incident language on planned operations.

## Scope

Frozen at this revision:

- one `local_safe_baseline_record` carrying baseline id, a
  `baseline_kind_class` (launch-critical local-first, mirror-backed
  offline, air-gapped offline-only, hybrid remote-attach
  local-first, managed-cloud local-companion), a closed
  `launch_critical_workflow_class` set, per-row availability and
  freshness posture, and opaque back-references into the
  upstream local-core continuity packet, deployment summary card,
  and connectivity snapshot;
- one `failover_banner_record` carrying banner id, title, summary,
  trigger-kind, typed `impairment_source` with one or more upstream
  refs and the upstream state-class re-exports, impacted features
  by control-plane service class and connectivity service family,
  retained local-safe capability sentences, the closed
  `continuity_action_row` set, the typed `evidence_export_action`,
  the boundary-change note, lifecycle (freshness / supersession /
  retention), and display-copy with five `const: false`
  invariants;
- the closed continuity-action vocabulary
  (`runs_locally_now`, `queued_for_replay`,
  `retryable_when_connected`, `blocked_pending_reconnect`,
  `blocked_pending_boundary_recheck`, `blocked_no_safe_retry`,
  `requires_manual_rerun`, `refused_authority_changing`);
- the rule that a banner sourced from a tenant migration / failover
  event with a changed boundary MUST carry at least one
  `blocked_pending_boundary_recheck`, `requires_manual_rerun`, or
  `refused_authority_changing` row;
- the rule that a banner pinned to a non-`active_current`
  lifecycle MUST carry a stale label;
- the rule that a banner triggered by planned maintenance, a
  scheduled drain window, a scheduled export freeze, a tenant or
  region migration, or a post-event reconciliation MUST keep
  `display_copy.incident_language_for_planned_used = false`.

Out of scope at this revision (named explicitly so reviewers know
what is *not* being decided here):

- implementing transport failover, the deferred-intent outbox, the
  reconciler, or any incident-orchestration system. The records
  freeze the *communication boundary*; the orchestration plane
  lives elsewhere;
- final user-facing copy, animation, iconography, accessibility
  affordances, or screen-reader behaviour. Surfaces compose against
  the records frozen here;
- replacement of the unified `outage_notice_record` rendering
  contract or the `maintenance_notice_record` /
  `tenant_migration_event_record` planned-event contract; the
  banner is downstream of those records, not a substitute for
  them;
- live wiring against a status-page provider, paging provider,
  observability backend, or notification routing layer.

## Banner trigger kinds (frozen)

A `failover_banner_record` carries exactly one
`trigger_kind_class`:

| Trigger | Source family |
| --- | --- |
| `service_family_outage` | An `outage_notice_record` whose `notice_state_class` is `read_only`, `drain`, `reconciling`, or a generic `unplanned_degradation` notice covering one or more service families. |
| `control_plane_failover` | A `tenant_migration_event_record` with `event_kind_class = control_plane_failover` (or an `outage_notice_record` whose `notice_state_class = failover` for control-plane services). |
| `regional_failover` | A `tenant_migration_event_record` with `event_kind_class = regional_failover` (or an `outage_notice_record` whose `notice_state_class = failover` for region scope). |
| `tenant_or_region_migration` | A `tenant_migration_event_record` whose kind is `tenant_migration`, `region_migration`, `residency_migration`, `key_ownership_migration`, or `endpoint_migration`. |
| `planned_maintenance_window` | A `maintenance_notice_record` whose kind is `scheduled_maintenance_window` or `scheduled_read_only_window`. |
| `scheduled_drain_window` | A `maintenance_notice_record` whose kind is `scheduled_drain_window`. |
| `scheduled_export_freeze` | A `maintenance_notice_record` whose kind is `scheduled_export_freeze`. |
| `post_event_reconciliation` | A `maintenance_notice_record` whose kind is `post_maintenance_reconciliation` or a `tenant_migration_event_record` whose kind is `post_event_reconciliation`. |
| `connectivity_constrained` | A `connectivity_state_snapshot_record` where one or more service families resolve `constrained` and no upstream notice/event is the primary source. |
| `connectivity_offline_local_safe` | A `connectivity_state_snapshot_record` where one or more managed families resolve `offline_local_safe` (managed plane unreachable, local plane intact). |
| `connectivity_reauth_required` | A `connectivity_state_snapshot_record` where `auth_identity_policy` (or a managed family's bound auth) resolves `reauth_required`. |
| `local_safe_only_mode` | A composite trigger: every managed family resolves `offline_local_safe`, `service_unavailable`, or `not_applicable`, and the banner is the user's only durable indication that managed action paths are unavailable across the board. |

A surface MAY NOT invent a trigger outside this set.

## Required disclosure on every banner

Every `failover_banner_record` MUST carry:

- `title` and `summary` — short product-term sentences.
- `created_at` and `updated_at` — exact UTC times.
- `trigger_kind_class` — exactly one row from the table above.
- `impairment_source` — at least one of `outage_notice_ref`,
  `maintenance_notice_ref`, `migration_event_ref`, or
  `connectivity_snapshot_ref` is non-null. The upstream
  state-class enum (one of `outage_state_class`,
  `maintenance_state_class`, `migration_state_class`,
  `connectivity_state_class`) is non-null for every populated ref
  so the banner restates the same state the source carries.
- `impacted_features` — control-plane service classes and
  connectivity service families. Either list MAY be empty (a
  connectivity-only banner has no control-plane outage rows) but
  `feature_summary` MUST resolve a product-term sentence.
- `local_safe_baseline_ref` — opaque ref to the
  `local_safe_baseline_record` this banner pins.
- `retained_local_safe_capabilities` — at least one product-term
  sentence. Inferring local-safe capability from the absence of an
  impacted feature is non-conforming.
- `continuity_action_rows` — at least one row. Every user-mintable
  action class the banner narrates resolves exactly one
  `continuity_action_state_class`.
- `evidence_export_action` — one typed action; if no evidence /
  export path is admissible the action MUST be
  `no_action_required` rather than absent.
- `boundary_change_note` — present even when no boundary change is
  expected; its `boundary_change_required` boolean simply resolves
  `false` and `boundary_axes_summary` is empty.
- `lifecycle` — the freshness / supersession / retention block.
- `display_copy` — every required line plus the five invariants
  pinned to `const: false`.
- `narrative_refs` — repo-relative pointers into companion
  contracts.

Raw URLs, raw hostnames, raw IP addresses, raw tenant names, raw
account ids, raw endpoint credentials, raw policy bodies, and raw
secret material do not cross either boundary. Records carry
opaque refs and redaction-aware sentences only.

## Continuity-action vocabulary (frozen)

Every user-mintable action the banner narrates resolves to exactly
one `continuity_action_state_class`. The vocabulary is closed; a
surface MAY NOT collapse two states into one display line.

| Continuity action state | Meaning | Queue posture | Required user step |
| --- | --- | --- | --- |
| `runs_locally_now` | Action runs locally; connectivity state is irrelevant. Refusal forbidden. | `no_queue_admitted` | `no_step_required` |
| `queued_for_replay` | Action was admitted to the deferred-intent outbox or the publish-later queue. Drain on reconnect. | One of `publish_later_queue`, `idempotent_outbox_queue`, `background_refresh_queue`, `upload_replication_queue`. `queue_or_intent_ref` is non-null. | Typically `wait_for_window_end` or `wait_for_reconnect`. |
| `retryable_when_connected` | Action was rejected (live-only requirement) but a fresh retry is safe once connectivity returns. No queue admission. | `no_queue_admitted` | `wait_for_reconnect`. |
| `blocked_pending_reconnect` | Action requires the family to leave `offline_local_safe`, `reauth_required`, or `service_unavailable` first. | `no_queue_admitted`. | `wait_for_reconnect` or `reauthenticate`. |
| `blocked_pending_boundary_recheck` | Action requires a boundary review (tenant, region, residency, key ownership, or endpoint identity) before it can resume. | `no_queue_admitted`. | `review_new_boundary`. |
| `blocked_no_safe_retry` | Action cannot proceed and cannot replay. Local capture is forbidden because the boundary is unknown. | `no_queue_admitted`. | Typically `escalate_for_admin_review` or `abandon_action`. |
| `requires_manual_rerun` | Reconciliation already drained the action and produced a conflict, expiry, freshness violation, or authority drift. | One of `drained_with_conflict`, `expired_before_drain`. | A non-`no_step_required` step (`reissue_with_fresh_freshness`, `promote_to_local_draft`, `open_in_provider`, `escalate_for_admin_review`, `abandon_action`). |
| `refused_authority_changing` | Action class is `never_queueable` (collaboration role grant/revoke, irreversible publish, destructive delete, paid model dispatch, auth refresh). The banner names the refusal so users do not assume it queued silently. | `no_queue_admitted`; `queue_or_intent_ref` MUST be null. | `no_step_required` or `escalate_for_admin_review`. |

The schema enforces these pairings on every row:

- `queued_for_replay` ⇒ `queue_or_intent_ref` is non-null and
  `queue_posture_class` is one of the four queue postures.
- `refused_authority_changing` ⇒ `queue_or_intent_ref` is null and
  `queue_posture_class = no_queue_admitted`.
- `runs_locally_now` ⇒ no queue, no required step, no ref.
- `requires_manual_rerun` ⇒ `required_user_step_class` is
  non-`no_step_required`.

## Local-safe baseline rows (frozen)

A `local_safe_baseline_record` lists at least one row from the
closed `launch_critical_workflow_class` set:

- `local_editing`, `local_save`, `local_undo_redo` — the editor's
  in-buffer authoring path.
- `local_search_index_query` — on-device search and symbol
  lookup.
- `local_git_commit_branch` — local Git operations against the
  on-disk repo.
- `local_build_test_debug` — on-device build, test, and debug.
- `local_export_bundle` — exporting workspace bundles, support
  bundles, evidence packets, or a continuity packet.
- `local_diagnostics` — local diagnostics view; `Open continuity
  packet`-style affordances.
- `local_open_recent_workspace` — opening a previously-loaded
  workspace from the on-device list.
- `local_inspect_docs_pack` — reading the local docs pack.
- `local_inspect_cached_provider_snapshot` — viewing a cached
  provider snapshot wrapped in an `imported_provider_snapshot_record`
  with its freshness label.
- `local_inspect_policy_snapshot` — inspecting the cached policy /
  entitlement snapshot under its freshness floor.

Every row resolves one `availability_class` and one
`freshness_posture_class`. A row whose availability is
`available_local_safe` MAY still resolve `bounded_stale` for
freshness if the workflow consumes a cached input; the freshness
posture is therefore independent of availability.

The baseline's `composes_with` block carries opaque
`local_core_continuity_packet_refs`,
`deployment_summary_card_refs`, and `connectivity_snapshot_refs`
so consumers can cross-walk to the upstream record family without
re-naming locality, profile, or service-family vocabulary.

## Reuse rules with upstream notices (frozen)

The banner does not invent a separate incident vocabulary.
Specifically:

- A banner triggered by a planned maintenance, scheduled drain,
  scheduled export freeze, tenant or region migration, or
  post-event reconciliation MUST resolve
  `display_copy.incident_language_for_planned_used = false`. The
  schema enforces this through the conditional in the top-level
  `allOf` block.
- A banner triggered by a connectivity-only condition
  (`connectivity_constrained`,
  `connectivity_offline_local_safe`,
  `connectivity_reauth_required`, `local_safe_only_mode`) MAY have
  no upstream `outage_notice_ref`, `maintenance_notice_ref`, or
  `migration_event_ref`; in that case `connectivity_snapshot_ref`
  is non-null and `connectivity_state_class` is the upstream
  per-family state vocabulary verbatim.
- A banner sourced from a `tenant_migration_event_record` whose
  state is `completed_boundary_changed` MUST set
  `boundary_change_required = true` in `boundary_change_note` and
  MUST carry a continuity row whose state is
  `blocked_pending_boundary_recheck`, `requires_manual_rerun`, or
  `refused_authority_changing`. The schema enforces this through
  the conditional in the top-level `allOf` block.
- A banner whose `impairment_source.outage_state_class` resolves
  `resolved` (or whose `migration_state_class` resolves
  `completed_boundary_unchanged`) and whose lifecycle is
  `superseded_stale`, `completed_historical`, or
  `imported_historical` MUST carry a non-null
  `display_copy.stale_label` and MAY NOT pretend to be live.

## When queued, retryable, blocked, and manual-rerun rows must appear distinctly

The banner's display rules forbid collapsing the four user-facing
continuity outcomes into a single line:

- A row whose state is `queued_for_replay` MUST be visible on the
  banner's queued-or-retry display line. Surfaces that render the
  banner cannot fold it into a single "service unavailable" status
  string.
- A row whose state is `retryable_when_connected` MUST be visible
  on the same queued-or-retry line but distinct from queued items
  (no queue ref).
- A row whose state is `blocked_pending_reconnect`,
  `blocked_pending_boundary_recheck`, or `blocked_no_safe_retry`
  MUST be visible on the blocked-or-manual line.
- A row whose state is `requires_manual_rerun` MUST be visible on
  the blocked-or-manual line and MUST cite a non-`no_step_required`
  user step.
- A row whose state is `refused_authority_changing` MUST be
  visible on the blocked-or-manual line; the banner's copy names
  the refusal so users do not assume it queued silently.

The display-copy invariant `queued_and_blocked_collapsed = false`
makes the rule schema-enforceable: a fixture or record that flips
it is invalid by construction.

## Evidence and export path (frozen)

Every banner MUST resolve exactly one `evidence_export_action`.
The action vocabulary is:

- `export_diagnostics` — open the local diagnostics export with
  the impaired-route metadata pre-populated.
- `export_before_maintenance` — pre-window export against a
  scheduled export freeze; required in the
  `scheduled_export_freeze` trigger.
- `open_history` — open the historical notice / event record so
  the user can see what happened.
- `open_boundary_details` — open the boundary axes detail surface
  when a migration or failover changed the route.
- `open_continuity_packet` — open the
  `local_core_continuity_packet_record` the banner pins so the
  user can inspect the locality, freshness, and restore-class
  posture.
- `no_action_required` — no export or evidence path is admissible
  for this banner. Resolved explicitly rather than by absence.

## Display-copy invariants (frozen)

Every banner's `display_copy` block MUST keep five invariants
false:

- `all_work_broken_implied` — the banner cannot imply that all
  work is broken when local-safe baseline rows remain available.
- `generic_unavailable_banner_used` — the banner cannot collapse
  queued, retryable, blocked, and manual-rerun continuity into a
  single "service unavailable" line.
- `queued_and_blocked_collapsed` — `queued_for_replay` rows and
  `blocked_*` rows cannot share a display line; the banner MUST
  keep them visually distinct.
- `local_safe_invisible` — the banner MUST surface retained
  local-safe capabilities; it cannot only name the broken
  service.
- `incident_language_for_planned_used` — banners triggered by
  planned maintenance, scheduled drain, scheduled export freeze,
  tenant/region migration, or post-event reconciliation cannot
  reuse incident or unplanned-degradation language.

The schema enforces the first four as `const: false` on the
`display_copy` object directly. The fifth is enforced through the
conditional in the top-level `allOf` block so unplanned-degradation
banners (which legitimately carry incident language) are not
penalised.

## Worked fixtures

See [`/fixtures/ops/failover_continuity_cases/`](../../fixtures/ops/failover_continuity_cases/)
for worked fixtures exercising the contract:

- `service_family_outage.yaml` — a single managed family
  (`ai_broker_service` / `paid_model_dispatch`) is unavailable;
  local-safe baseline is intact; idempotent managed writes queue,
  paid-model dispatch is refused, support-bundle upload is
  retryable when connected.
- `regional_failover_changed_boundary.yaml` — a regional failover
  composed with a `tenant_migration_event_record`; region, key
  ownership, and endpoint identity changed; managed lifecycle
  writes are blocked pending boundary recheck and policy admin
  writes are blocked with no safe retry.
- `local_safe_only_mode.yaml` — every managed family resolves
  offline-local-safe; the banner is the user's only durable
  indication; the baseline records launch-critical workflows as
  available locally.
- `partial_queue_retry_continuity.yaml` — a mixed posture:
  publish-later queue is admitting review-comment and merge-queue
  intents, idempotent label changes are queueing through the
  outbox, an earlier intent already drained with a target-drift
  conflict (manual rerun), and a collaboration role-grant is
  refused.
