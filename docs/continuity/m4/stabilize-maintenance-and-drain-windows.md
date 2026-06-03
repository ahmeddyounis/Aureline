# Maintenance, Drain, and Failover Continuity

## Overview

When Aureline operates in a managed or provider-linked context, planned and
unplanned windows are communicated with **exact time**, **blocked-write
disclosure**, and **local-safe alternatives**. Generic outage copy is never used
when the product knows the boundary, duration, or write class affected.

## What you see during a window

### Exact time

Every notice shows:

- **Start time** — ISO-8601 timestamp.
- **End time** — ISO-8601 timestamp.
- **Timezone** — IANA timezone name (e.g. `UTC`, `America/New_York`).
- **Last refreshed** — when the notice was last updated.

Example:

> Scheduled maintenance: 2026-06-10 02:00–04:00 UTC (last refreshed 2026-06-03 04:08 UTC)

### Affected surfaces

The notice names which surfaces are impacted:

- Desktop IDE shell
- Review surface (diff, comments, approvals)
- Collaboration surface (shared sessions, presence)
- Provider sync and mutation surfaces
- Update center
- Service-health cards
- CLI/headless output

### Blocked writes

For each blocked write class, the notice explains:

- What is blocked (e.g. "Provider mutations")
- Why it is blocked
- What user action is prevented
- Whether a local-draft alternative is offered

Example:

> **Provider mutations blocked** — Publishing reviews, creating issues, or requesting CI reruns are prevented during the maintenance window. You can continue authoring local drafts and queue them for publish-later.

### Local-safe actions

The notice always names at least one safe alternative:

- Continue local-draft authoring
- Inspect cached provider state
- Export an evidence-safe packet
- Queue work for publish-later
- Defer the action until the window closes
- Open the object through a typed browser handoff

## Window states

| State | Meaning | User impact |
|---|---|---|
| Scheduled | Window is known but has not started. | Plan around the window; queue work if desired. |
| Read-only | Managed surfaces are read-only. | No new writes to provider-backed state; local drafts remain safe. |
| Drain in progress | New high-risk work is blocked. | In-flight work completes; new work is blocked with explanation. |
| Migration | Tenant or org migration is underway. | Scoped to affected tenant; other tenants operate normally. |
| Failover | Active failover to secondary region. | Degraded managed capability; local core preserved. |
| Reconciling | Post-window reconciliation is running. | Queued intent is being validated; do not start new high-risk work. |
| Resolved | Normal operation resumed. | All surfaces fully operational. |

## Stale notices

Cached notices downgrade automatically when they age out:

- **Current** — shown at full severity.
- **Stale within grace** — shown with degraded severity and a refresh action.
- **Expired** — hidden or replaced with a "refresh for current status" prompt.

A stale notice never overclaims an ongoing outage or recovery posture.

## Post-window reconciliation

After a window closes, any queued or publish-later intent is revalidated against
six dimensions:

1. Tenant or organization identity
2. Region or datacenter placement
3. Service endpoint
4. Policy epoch
5. Authentication scope
6. Provider target identity

If any dimension drifted, the intent does **not** replay silently. Instead, you
see a reconciliation sheet showing:

- Which dimension drifted
- The pre-window and post-window values
- The recommended next safe action (refresh, reauth, rescope, compare/review, or cancel)

## CLI inspect

Enterprise operators and support staff can audit continuity state via the CLI:

```bash
aureline_service_health_inspect < continuity_page.json
```

The tool validates the page and emits a redaction-safe support export projection.

## See also

- Artifact: `artifacts/continuity/m4/stabilize-maintenance-and-drain-windows.md`
- Schema: `schemas/service/service_health_continuity.schema.json`
- Crate: `crates/aureline-service-health/src/stabilize_maintenance_and_drain_windows/`
