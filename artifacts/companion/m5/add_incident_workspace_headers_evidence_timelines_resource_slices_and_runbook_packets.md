# Incident Workspace Headers, Evidence Timelines, Resource Slices, and Runbook Packets

- Packet: `incident-workspace-surface:stable:0001`
- Label: `Incident Workspace Headers, Evidence Timelines, Resource Slices, and Runbook Packets`
- Sections: 4 | Headers: 2 | Evidence spans: 3 | Resource slices: 2 | Runbook packets: 2
- Exact handoff for every item: yes
- Stale state honestly labeled: yes
- Evidence gaps honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **header**: `stable` / `general_availability` [read_only] (matrix lane `incident_workspace`)
- **evidence_timeline**: `stable` / `general_availability` [read_only] (matrix lane `incident_workspace`)
- **resource_slice**: `beta` / `staged_rollout` [read_only] (matrix lane `incident_workspace`)
- **runbook_packet**: `preview` / `early_access` [read_only] (matrix lane `incident_workspace`)

## Headers

- `header:incident:0001` [critical/attributed] investigating — Critical incident raised from a crash trail, under investigation (live) → `incident_workspace` (exact)
- `header:incident:0002` [high/attributed] mitigating — High-severity incident under mitigation on the host (cached) → `incident_workspace` (exact)

## Evidence timeline

- `evidence:0001` #1 [crash_trail/present] attributed — Crash trail captured at the moment of failure (live) → `incident_workspace` (exact)
- `evidence:0002` #2 [log_window/present] attributed — Log window around the failure (cached) → `incident_workspace` (exact)
- `evidence:0003` #3 [metric_series/missing] partially_attributed — Metric series for the window is missing and recorded as a gap (unknown) → `incident_workspace` (exact)

## Resource slices

- `slice:0001` [cpu_profile] CPU profile slice attributed to the incident (live) → `incident_workspace` (exact)
- `slice:0002` [memory_snapshot] Memory snapshot slice attributed to the incident (cached) → `incident_workspace` (exact)

## Runbook packets

- `runbook:0001` [manual] 5 steps, next `in_progress` — Manual mitigation runbook for the crash incident (live) → `incident_workspace` (exact)
- `runbook:0002` [automated_with_approval] 3 steps, next `pending` — Automated rollback runbook; each action requires host approval (live) → `incident_workspace` (exact)
