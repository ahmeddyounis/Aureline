# M5 Runbook Certification

- Packet: `m5-runbook-certification:stable:0001`
- Label: `M5 runbook certification`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Proof lanes: 6 (6 current, 0 stale, 0 missing)
- Rows: 7 (7 certified, 0 narrowed, 0 blocked)
- Release gate: pass
- Exposed on: Help/About, shiproom, support exports, incident/operator

## Runbook proof lanes

| Lane | Facet | Owner | Source of truth | Proof | Freshness |
|------|-------|-------|-----------------|-------|-----------|
| `governance` | `source_truth` | runbook_governance_owner | `schemas/runbooks/m5-runbook-governance.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |
| `sources` | `source_truth` | runbook_authoring_owner | `schemas/runbooks/m5-runbook-source-register.schema.json` | `artifacts/release/m5-runbook-proof/runbook-source-register.json` | `current` |
| `steps` | `step_lineage` | runbook_authoring_owner | `schemas/runbooks/m5-runbook-step-library.schema.json` | `artifacts/release/m5-runbook-proof/runbook-step-library.json` | `current` |
| `executions` | `step_lineage` | incident_operations_owner | `schemas/runbooks/m5-runbook-execution-history.schema.json` | `artifacts/release/m5-runbook-proof/runbook-execution-history.json` | `current` |
| `handoffs` | `boundary_honesty` | control_plane_boundary_owner | `schemas/runbooks/m5-runbook-handoff-register.schema.json` | `artifacts/release/m5-runbook-proof/runbook-handoff-register.json` | `current` |
| `companion` | `export_proof` | companion_owner | `schemas/runbooks/m5-runbook-companion-register.schema.json` | `artifacts/release/m5-runbook-proof/runbook-companion-register.json` | `current` |

## Claimed incident/operator rows

| Row | Consumer | Status | Claim → effective | Gate | Binds |
|-----|----------|--------|-------------------|------|-------|
| `incident-runbook-execution-pane` | `incident_workspace` | `mapped` | `stable` → `stable` | `governed` | governance, sources, steps, executions |
| `operator-runbook-history` | `operator_dashboard` | `mapped` | `stable` → `stable` | `governed` | governance, steps, executions, handoffs |
| `operator-console-boundary-pane` | `operator_dashboard` | `mapped` | `stable` → `stable` | `governed` | governance, handoffs |
| `companion-runbook-follow` | `companion` | `mapped` | `stable` → `stable` | `governed` | governance, steps, companion |
| `support-runbook-bundle` | `support_bundle` | `mapped` | `stable` → `stable` | `governed` | executions, handoffs, companion |
| `docs-runbook-reference` | `docs_help` | `mapped` | `stable` → `stable` | `governed` | governance, sources, steps |
| `release-runbook-certification-gate` | `release_center` | `mapped` | `stable` → `stable` | `governed` | governance, sources, steps, executions, handoffs, companion |
