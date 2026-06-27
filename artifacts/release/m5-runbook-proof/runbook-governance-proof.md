# M5 Runbook-Governance Matrix

- Packet: `m5-runbook-governance:stable:0001`
- Label: `M5 runbook governance matrix`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Governed objects: 6 (4 schemas under `schemas/runbooks/`)
- Surfaces: 6 (6 mapped, 0 provisional, 0 unmapped)
- Release gate: pass (0 blocked, 0 narrowed, 6 governed)
- Active waivers: 0

## Governed runbook objects

| Object | Owner | First consumer | Source of truth | Proof | Freshness |
|--------|-------|----------------|-----------------|-------|-----------|
| `source_descriptor` | runbook_authoring_owner | `docs_help` | `schemas/runbooks/m5-runbook-source.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |
| `step_descriptor` | runbook_authoring_owner | `operator_dashboard` | `schemas/runbooks/m5-runbook-step.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |
| `execution_record` | incident_operations_owner | `incident_workspace` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |
| `deviation_note` | incident_operations_owner | `incident_workspace` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |
| `control_plane_handoff` | control_plane_boundary_owner | `operator_dashboard` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |
| `archival_export` | support_export_owner | `support_bundle` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` | `current` |

## Claimed runbook-backed surfaces

- **incident-runbook-pane** (`incident_workspace`): `mapped` (green), claim `stable` → `stable`, gate `governed`
  - Owner: incident_operations_owner
  - Binds: source_descriptor, step_descriptor, execution_record, deviation_note
- **operator-runbook-console** (`operator_dashboard`): `mapped` (green), claim `stable` → `stable`, gate `governed`
  - Owner: operator_console_owner
  - Binds: source_descriptor, step_descriptor, execution_record, control_plane_handoff
- **docs-runbook-reference** (`docs_help`): `mapped` (green), claim `stable` → `stable`, gate `governed`
  - Owner: docs_help_owner
  - Binds: source_descriptor, step_descriptor
- **companion-runbook-assist** (`companion`): `mapped` (green), claim `stable` → `stable`, gate `governed`
  - Owner: companion_owner
  - Binds: source_descriptor, step_descriptor, deviation_note
- **support-runbook-export** (`support_bundle`): `mapped` (green), claim `stable` → `stable`, gate `governed`
  - Owner: support_export_owner
  - Binds: execution_record, deviation_note, archival_export
- **release-runbook-gate** (`release_center`): `mapped` (green), claim `stable` → `stable`, gate `governed`
  - Owner: release_center_owner
  - Binds: source_descriptor, step_descriptor, execution_record, deviation_note, control_plane_handoff, archival_export
