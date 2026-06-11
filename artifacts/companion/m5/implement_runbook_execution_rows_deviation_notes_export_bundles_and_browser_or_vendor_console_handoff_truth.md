# Runbook Execution Rows, Deviation Notes, Export Bundles, and Browser or Vendor-Console Handoff

- Packet: `runbook-execution-surface:stable:0001`
- Label: `Runbook Execution Rows, Deviation Notes, Export Bundles, and Browser or Vendor-Console Handoff`
- Sections: 4 | Execution rows: 3 | Deviation notes: 2 | Export bundles: 2 | External handoffs: 2
- Exact desktop handoff for every item: yes
- External handoffs keep a local fallback: yes
- Stale state honestly labeled: yes
- Export bundles honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **execution_row**: `stable` / `general_availability` [read_only] (matrix lane `incident_workspace`)
- **deviation_note**: `stable` / `general_availability` [read_only] (matrix lane `incident_workspace`)
- **export_bundle**: `beta` / `staged_rollout` [read_only] (matrix lane `incident_workspace`)
- **external_handoff**: `preview` / `early_access` [read_only] (matrix lane `incident_workspace`)

## Execution rows

- `exec:0001` #1 [completed/succeeded/attributed] manual — First mitigation step completed successfully (live) → `incident_workspace` (exact)
- `exec:0002` #2 [in_progress/in_flight/attributed] automated_with_approval — Automated rollback step in progress; awaiting host approval to apply (live) → `incident_workspace` (exact)
- `exec:0003` #3 [skipped/deviated/attributed] assisted_suggestion — Third step skipped by the operator; recorded as a deviation (cached) → `incident_workspace` (exact)

## Deviation notes

- `deviation:0001` [step_skipped/notable/attributed] Operator skipped the cache-flush step; redundant for this incident (live) → `incident_workspace` (exact)
- `deviation:0002` [parameter_changed/major/partially_attributed] Rollback target changed from the runbook default; attribution partial (unknown) → `incident_workspace` (exact)

## Export bundles

- `bundle:0001` [runbook_execution_transcript/ready] Redacted transcript of the runbook execution rows (live) → `incident_workspace` (exact)
- `bundle:0002` [full_incident_archive/partial] Full incident archive; one evidence input missing, labeled partial (cached) → `incident_workspace` (exact)

## External handoffs

- `external:0001` [browser_companion_tab] Browser companion — Resume the incident in a browser companion tab; local desktop fallback kept (live) → external `external:browser:incident-0001` (exact), local `incident_workspace` (exact)
- `external:0002` [vendor_incident_console] Vendor incident console — Open the incident in the vendor incident console; requires provider continuity (cached) → external `external:vendor-console:incident-0001` (exact), local `incident_workspace` (exact)
