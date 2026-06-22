# Maintenance / failover / reconciliation windows — evidence companion

Human-readable companion to
[`/fixtures/ops/m5-maintenance-windows/canonical_windows.json`](../../fixtures/ops/m5-maintenance-windows/canonical_windows.json)
and its boundary schema
[`/schemas/ops/m5-maintenance-windows.schema.json`](../../schemas/ops/m5-maintenance-windows.schema.json).
It gives reviewers the frozen window, timing, blocked-write, boundary, and
invariant tables without reading the JSON. The contract narrative lives in
[`/docs/ops/m5-maintenance-windows.md`](../../docs/ops/m5-maintenance-windows.md).

- Set id: `m5-maintenance-windows:set:0001`
- Record kind: `m5_maintenance_window_set`
- Bound matrix: `fixtures/ops/m5-operator-surfaces/canonical_matrix.json`
  (`m5_operator_surface_matrix`)
- Windows: 7 (4 maintenance notice · 3 failover notice) · Invariants: 14
- Phases exercised: scheduled, read-only, drain, migration, failover,
  reconciling, resolved

## Windows, exact times, and computed state

Each window binds a matrix surface family (maintenance notice or failover notice)
by that matrix's own surface id and points at one canonical `aureline://`
service-health object. `effective_state` is the computed no-silent-green downgrade
of the phase's matrix state and the latest-refresh freshness, so a
resolved-but-unconfirmed window is never reported `clear`.

| Window | Kind | Phase | → Effective | Start → end (zone) | Refreshed (freshness) | Write posture | Blocked writes | Boundary axes | Review? |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0001 | planned_maintenance | scheduled | scheduled_window | 2026-06-23T02:00 → 04:00 (America/New_York −04:00) | 08:00 (fresh) | writes_live | — | — | no |
| 0002 | planned_maintenance | read_only | read_only_window | 2026-06-22T01:00 → 03:00 (UTC +00:00) | 01:05 (fresh) | publish_later_queued | managed_settings_apply, managed_policy_change | endpoint_identity=unchanged | no |
| 0003 | planned_maintenance | drain | drain_window | 2026-06-22T00:30 → 01:00 (America/New_York −04:00) | 00:35 (recent) | publish_later_queued | managed_settings_apply | endpoint_identity=unchanged | no |
| 0004 | regional_failover | failover | failover_in_progress | 2026-06-22T00:10 → 02:10* (UTC +00:00) | 00:12 (fresh) | blocked_pending_boundary_recheck | authority_change, managed_settings_apply, provider_mutation | region=**changed**, endpoint_identity=**changed** | **yes** |
| 0005 | tenant_migration | migration | migration_in_progress | 2026-06-22T03:00 → 05:00* (Europe/Berlin +02:00) | 03:02 (fresh) | blocked_pending_boundary_recheck | authority_change, managed_policy_change, managed_settings_apply | tenant=**changed**, residency=**changed**, key_ownership=**changed** | **yes** |
| 0006 | regional_failover | reconciling | reconciling | 2026-06-22T02:10 → 02:40* (UTC +00:00) | 02:12 (recent) | publish_later_queued | authority_change, managed_settings_apply | region=**changed**, endpoint_identity=**changed** | **yes** |
| 0007 | planned_maintenance | resolved | clear | 2026-06-21T02:00 → 03:00 (America/New_York −04:00) | 07:50 (recent) | writes_live | — | endpoint_identity=unchanged | no |

`*` end timestamp is an estimate (`end_is_estimated: true`).

## What stays local, what is blocked, and the next safe action

Every window names at least one local-safe action, so a window never reads as a
total outage. A window that blocks managed writes offers publish-later / draft
capture so blocked writes queue rather than fail.

- **0001 — Scheduled control-plane maintenance** — local-safe: continue_local_edit,
  save_local, search, export_before_window, open_continuity_packet; next:
  `export_before_window`; nothing blocked yet.
- **0002 — Read-only window in effect** — local-safe: continue_local_edit,
  save_local, search, export_diagnostics, inspect_evidence, publish_later; next:
  `publish_later`; blocked managed settings/policy writes save as local drafts.
- **0003 — Drain window** — local-safe: continue_local_edit, save_local, search,
  export_diagnostics, inspect_evidence, publish_later; next: `publish_later`;
  in-flight work finishes and new applies queue.
- **0004 — Regional failover in progress** — local-safe adds `review_new_boundary`;
  next: `review_new_boundary`; authority-changing writes are refused, not retried.
- **0005 — Tenant migration in progress** — local-safe adds `review_new_boundary`;
  next: `review_new_boundary`; authority/policy writes held until the new tenant is
  reviewed.
- **0006 — Reconciling after failover** — local-safe adds `review_new_boundary`;
  next: `review_new_boundary`; queue reviewed against the settled region before
  replay.
- **0007 — Resolved** — local-safe: continue_local_edit, save_local, search,
  export_diagnostics, inspect_evidence; next: `continue_local`; writes live again.

## Changed-boundary disclosure and review-before-replay

The failover and migration windows restate exactly which boundary axis moved
instead of implying an unchanged route. Review-before-replay is computed: it is
required exactly when queued actions would cross a changed or unknown boundary
after the window ends.

- **0002 / 0003** have queued actions but an **unchanged** endpoint, so the queue
  replays against the same route — no review (`no_review_needed`).
- **0004 / 0006** changed the region and endpoint → review required, trigger
  `changed_region`.
- **0005** changed the tenant, residency, and key ownership → review required,
  trigger `changed_tenant`.

## Invariants

All 14 invariants are computed from the built data and frozen as `holds: true`:

- `maintenance_windows.surface_binding`
- `maintenance_windows.canonical_object_identity`
- `maintenance_windows.exact_time_and_zone`
- `maintenance_windows.latest_refresh_visible`
- `maintenance_windows.effective_state_computed`
- `maintenance_windows.blocked_writes_named`
- `maintenance_windows.local_safe_explicit`
- `maintenance_windows.publish_later_when_blocked`
- `maintenance_windows.boundary_disclosed_on_failover`
- `maintenance_windows.review_before_replay_computed`
- `maintenance_windows.distinguishable_from_outage`
- `maintenance_windows.all_phases_present`
- `maintenance_windows.both_surfaces_present`
- `maintenance_windows.stable_ids_unique`

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, exact timestamps, and short reviewable sentences.
`is_support_export_safe()` enforces the boundary, so the set is safe to embed in a
support export verbatim.
