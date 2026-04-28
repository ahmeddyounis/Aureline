# Control-plane, data-plane, and local-continuity notice contract

This contract freezes the object model and rendering rules for outage,
maintenance-window, drain, migration, failover, reconciliation, and
resolved notices that can affect Aureline managed services without
invalidating local-first work. It exists so a status strip, banner,
notification, support export, and admin review page all answer the same
question: which plane is impaired, what remains safe locally, which
writes are blocked, and whether the user must publish later, reconnect
later, export before the window, or review a changed boundary.

If this document, the companion schema, and the worked fixtures disagree,
the normative sources in `.t2/docs/` win and this document plus its
companions update in the same change.

## Companion artifacts

- [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  defines the machine-readable `outage_notice_record`.
- [`/fixtures/ops/outage_notices/`](../../fixtures/ops/outage_notices/)
  contains worked notices covering scheduled, read-only, drain,
  migration, failover, reconciling, and resolved states.

This contract composes with:

- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  for the control-plane service and local data-plane continuity
  vocabulary.
- [`/docs/architecture/logical_planes_and_trust_boundaries.md`](../architecture/logical_planes_and_trust_boundaries.md)
  for logical-plane and trust-boundary names.
- [`/docs/ux/status_strip_family_contract.md`](./status_strip_family_contract.md)
  for shared strip anatomy, stale labels, and screenshot-safe state.
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  for transport posture, mirror-only, offline, and deny-all terms.
- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md)
  for incident and evidence-handoff records that may cite a notice.

## Scope

Frozen at this revision:

- one `outage_notice_record` with a closed `notice_kind_class`
  vocabulary: `planned_maintenance`, `tenant_migration`, and
  `unplanned_degradation`;
- one closed `notice_state_class` vocabulary: `scheduled`,
  `read_only`, `drain`, `migration`, `failover`, `reconciling`, and
  `resolved`;
- exact time fields with UTC instants, an IANA display timezone, and
  the UTC offset that applied at window start;
- affected-scope fields for deployment profile, tenant/org/workspace,
  region, residency posture, service class, and endpoint refs;
- separate control-plane and data-plane effect lists;
- explicit retained-local-safe capability and blocked-managed-only
  capability lists;
- blocked write classes with a deferral path for each blocked class;
- guidance actions for continue-local, publish-later, reconnect-later,
  export-before-maintenance, and review-new-boundary;
- boundary-change axes for tenant, region, residency, key ownership,
  and endpoint identity;
- stale-notice and historical-retention fields so past notices remain
  reviewable after resolution or supersession.

Out of scope:

- backend incident-management implementation;
- tenant orchestration, region migration, failover automation, or
  replay queues;
- final visual layout, iconography, animation, or notification routing.

## Notice Kind And State

Every notice carries exactly one `notice_kind_class` and one
`notice_state_class`.

| State | Meaning | Required disclosure |
| --- | --- | --- |
| `scheduled` | A future maintenance or migration window is announced and has not started. | Exact start/end time, timezone, affected scope, planned blocked writes, and any required export-before-maintenance action. |
| `read_only` | Read paths remain available but one or more managed write classes are blocked. | Blocked write classes, local capture posture, publish-later path, and safe local capabilities. |
| `drain` | Existing sessions or writes may finish while new starts or joins are blocked. | Drain deadline, blocked new actions, finish/export options, and reconnect-later guidance. |
| `migration` | Tenant, residency, key, endpoint, or region posture is being moved or rechecked. | Old and new boundary facts where known, unknown axes where not known, and review-new-boundary follow-up. |
| `failover` | Service recovered or rerouted through an alternate fault domain after degradation. | Which plane failed over, which boundary axes changed, local-safe subset, and reconnect/review actions. |
| `reconciling` | Queued local intents, cached state, or managed acknowledgements are being compared before replay. | Queue counts or refs, publish-later review path, boundary drift, and replay hold reason. |
| `resolved` | The active window or degradation ended. | Actual resolved time, stale/historical label when applicable, and explicit boundary restatement if anything changed. |

`resolved` MUST NOT be rendered as a generic all-clear when a migration
or failover changed tenant, region, residency, key ownership, or endpoint
identity. It must restate the changed boundary and keep the
review-new-boundary action visible until the boundary review is complete.

## Required Record Anatomy

An `outage_notice_record` contains:

- identity: schema version, record kind, notice id, title, summary,
  kind, state, created/updated timestamps;
- schedule: observed/announced time, start time, expected or actual end
  time, optional resolved time, display timezone, UTC offset at window
  start, and optional export deadline;
- affected scope: deployment profiles, tenant/org/workspace refs,
  region refs, residency classes, service classes, endpoint refs, and a
  redaction-safe scope summary;
- plane effects: control-plane effects and data-plane effects in
  separate arrays;
- local continuity: local-core status, retained local-safe
  capabilities, blocked managed-only capabilities, and a continuity
  summary;
- blocked writes: one row per blocked write class, including local
  capture posture, deferral path, resume trigger, and optional queue ref;
- guidance actions: continue-local, publish-later, reconnect-later,
  export-before-maintenance, review-new-boundary, retry-after-window,
  open-boundary-details, and export-diagnostics entries as applicable;
- boundary change summary: changed or unknown axes for tenant, region,
  residency, key ownership, and endpoint identity;
- lifecycle and retention: freshness class, supersession refs,
  retained-until time, retention reason, and stale label when the notice
  is no longer current;
- display copy: short product-facing lines plus invariants that forbid
  "all work broken" and generic all-clear copy.

Raw URLs, raw hostnames, raw IP addresses, raw tenant names, raw account
identifiers, raw endpoint credentials, raw policy bodies, and raw secret
material do not cross this boundary. Records carry opaque refs and
redaction-aware labels only.

## Plane Separation Rules

Every notice MUST name control-plane and data-plane effects separately.
The control plane covers identity, policy, sync, registry, relay, AI
broker, catalog, telemetry/support ingest, hosted review, merge queue,
and workspace orchestration services. The data plane covers the work a
developer can still perform: local editing, save, search, Git, tasks,
docs inspection, export, diagnostics, remote attach, managed runtime,
collaboration, hosted review publish, merge queue, AI prompt routes,
extension install/update, profile sync, and support-bundle upload.

The UI is non-conforming if it collapses either plane into generic
"service degraded" copy when the record names a more precise state. A
control-plane outage may block policy refresh, hosted publish, or relay
joins while local editing, save, search, Git, export, and diagnostics
remain available. The notice must say that directly.

## Local Continuity Rules

`local_continuity` is required on every notice. If a meaningful safe
subset remains, `retained_local_safe_capabilities` must contain at least
one product-term sentence such as "continue local edits and save files"
or "export queued review drafts". If no safe local subset exists, the
notice must state why; it may not let the absence of local capability be
inferred from a generic blocked state.

Surfaces MUST keep continue-local visible whenever
`continue_local_guidance_required` is true. Continue-local is not a
dismissal of the outage. It is the bounded path that keeps local-first
work honest while managed-only work is paused.

## Blocked Writes And Deferral

Read-only and drain notices MUST list blocked write classes. Each row
must say whether local capture is allowed and which deferral path applies:

- `publish_later` when the user can capture the intent locally and
  replay or publish after the block lifts;
- `reconnect_later` when an attach, relay, or managed-runtime route must
  reconnect before work can resume safely;
- `export_before_maintenance` when a scheduled window may block a
  user-owned export, review packet, diagnostics packet, or portability
  packet and the safe action is to export before the window starts;
- `no_deferral_requires_export` when the write cannot be replayed and
  the only safe path is an explicit export;
- `blocked_no_safe_retry` when retry would be unsafe until policy or
  boundary review completes.

A read-only or drain surface that says only "try again later" is
non-conforming. The blocked write classes and deferral paths are part of
the notice, not hidden detail text.

## Boundary Changes

Migration and failover notices MUST carry a `boundary_change_summary`.
The summary names every relevant axis:

- `tenant`
- `region`
- `residency`
- `key_ownership`
- `endpoint_identity`

Each axis resolves to `unchanged`, `changed`,
`unknown_recheck_required`, or `not_applicable`. When any axis is
`changed` or `unknown_recheck_required`, the notice must include a
required `review_new_boundary` guidance action and must keep the boundary
line visible after resolution. Prior approvals, cached policy results,
publish queues, and reconnect tokens cannot be silently replayed across a
changed or unknown boundary.

Resolved notices with changed boundaries must preserve the prior notice
in history and label stale copies. The stale label must explain whether
the notice is superseded, expired, retained for boundary-change history,
or imported from an offline/exported source.

## Rendering Invariants

Every UI, CLI, support export, and admin view that renders an
`outage_notice_record` MUST preserve these invariants:

- It must not imply "all work is broken" when
  `local_core_status_class` reports a meaningful local-safe subset.
- It must not say "recovered" or "all clear" without restating changed
  tenant, region, residency, key ownership, or endpoint identity.
- It must not hide blocked write classes behind generic read-only copy.
- It must not auto-replay queued writes across a boundary change.
- It must keep stale notices visibly stale in history, exports, and
  support packets.
- It must keep exact times and timezone visible for scheduled windows,
  read-only windows, drain deadlines, export deadlines, and resolved
  times.

## Fixture Coverage

The fixture directory covers:

- scheduled planned maintenance with an export-before-maintenance
  deadline;
- active read-only window with publish-later capture;
- drain window with reconnect-later and finish/export options;
- tenant migration with changed region/residency boundary;
- regional failover with endpoint identity and key-ownership review;
- reconciliation of queued local drafts after a blocked window;
- resolved historical notice that remains retained and stale because a
  boundary changed.
