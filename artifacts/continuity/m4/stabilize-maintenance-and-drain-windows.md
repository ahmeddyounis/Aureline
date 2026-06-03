# Stabilize Maintenance and Drain Windows — Artifact

## Purpose

This artifact is the canonical proof packet for M04-199. It declares that Aureline
can tell users exactly when a planned window starts and ends, what becomes
read-only or blocked, what remains safely local, and how publish-later or local
drafts reconcile afterward.

## Checked-in truth sources

- Crate: `crates/aureline-service-health/src/stabilize_maintenance_and_drain_windows/`
- Schema: `schemas/service/service_health_continuity.schema.json`
- Fixtures: `fixtures/continuity/m4/stabilize-maintenance-and-drain-windows/`
- CLI inspect: `aureline_service_health_inspect`

## State model

### Maintenance window states

- `scheduled` — window is known but has not started.
- `read_only` — managed surfaces are read-only.
- `drain_in_progress` — new high-risk writes are blocked; in-flight work completes.
- `migration` — tenant or org migration is underway.
- `failover` — active failover to a secondary region or host.
- `reconciling` — post-window reconciliation is running.
- `resolved` — normal operation resumed.

### Blocked write classes

- `provider_mutation` — publish, update, delete on provider-linked objects.
- `review_publish` — publish reviews or review comments.
- `work_item_create_or_update` — create or update issues / work items.
- `comment_publish` — publish comments on provider-backed threads.
- `issue_report_create` — create incident or issue reports.
- `share_or_join` — share or join collaborative sessions.
- `collaboration_sync_push` — push collaboration sync state.
- `settings_sync_mutation` — mutate settings sync state.

### Local-safe actions

- `local_draft_authoring` — continue editing local drafts.
- `inspect_only` — view cached provider state without mutation.
- `export_evidence_packet` — export an evidence-safe packet.
- `queue_publish_later` — queue intent for post-window drain.
- `defer_to_post_window` — defer the action until the window closes.
- `open_in_provider_browser_handoff` — open the object via typed browser handoff.

### Stale-notice downgrade classes

- `current` — notice is within its freshness window; show at full severity.
- `stale_within_grace` — notice is stale but inside a bounded grace window; show with degraded severity and a refresh action.
- `expired_requires_refresh` — notice is past the grace window; hide or require explicit refresh before display.

### Post-window reconciliation states

- `queued` — intent is still queued awaiting drain.
- `replayed` — intent was replayed and committed after the window.
- `expired` — intent expired before the window closed.
- `cancelled` — intent was cancelled by user or policy.
- `needs_explicit_review` — a revalidation dimension drifted; explicit review required.

## Revalidation dimensions

After any maintenance, migration, or failover window, the system revalidates:

1. `tenant` — tenant or organization identity.
2. `region` — region or datacenter placement.
3. `endpoint` — service endpoint URL or identity.
4. `policy` — effective policy epoch or bundle.
5. `auth` — authentication scope or session validity.
6. `target_identity` — provider target identity (repository, project, org).

If any dimension drifted, the reconciliation result enters
`needs_explicit_review` and the user sees a reconciliation sheet rather than
silent replay.

## Guardrails

- No vague copy such as `Service interruption soon` or `Try again later` where
the product knows exact time, boundary, or blocked write class.
- No user is dropped mid-session or loses prepared provider-backed work because
a drain window began.
- No continuity notice implies full-cloud blockage when desktop-local work is
still safe.
- No write intent replays invisibly after a window closes.
- Stale cached notices visibly downgrade and do not overclaim current outage or
recovery posture.

## Acceptance criteria

- [x] Scheduled maintenance, read-only/drain windows, tenant migration, and
failover notices use exact time and timezone, identify affected write classes
and boundaries, and state what remains safely local.
- [x] Provider-backed actions entered during drain or read-only windows
preserve local draft or publish-later capture rather than failing silently or
pretending the mutation completed.
- [x] Stale cached notices visibly downgrade and do not overclaim current
outage or recovery posture.
- [x] Managed continuity claims narrow automatically when maintenance/drain/
failover proof, stale-notice handling, or local-safe next-step guidance is not
current on a claimed lane.
- [x] Post-window replay revalidates tenant, region, endpoint, policy, auth,
and target identity and shows a reconciliation sheet whenever any of those
drifted during the window.
- [x] Maintenance and failover drills produce identical queued/replayed/expired/
cancelled state vocabulary in service-health, provider/work-item flows, and
support exports; no write intent replays invisibly after the window closes.
