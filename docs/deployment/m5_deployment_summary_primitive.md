# M5 deployment-summary primitive

The **deployment-summary primitive** is the reusable deployment summary card,
residual-dependency rows, and control-plane/data-plane status strip that About,
admin, service-health, diagnostics, support, and docs surfaces ingest instead of
cloning an About-page, a diagnostics pane, or an admin-only dashboard. One deployment
context resolves into all three surfaces and they share one deployment identity, so
the operating boundary, the residual vendor dependency, and the split between
control-plane health and local-runtime continuity never blur across them.

It **narrows** the remaining three operational families of the frozen
[deployment/continuity component matrix](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
— `deployment_summary_card`, `residual_dependency_row`, and
`control_plane_data_plane_status_strip` — into one working resolver
(`resolve_deployment_summary`) rather than restating install / deployment truth in
feature-local prose. It reuses the frozen matrix's operating-mode, provenance /
freshness, residual-dependency-class, plane-state, and downgrade-trigger vocabulary;
it adds only the minted vocabulary the resolver needs (deployment scope, residual
failure consequence, residual mitigation, local-safe next step, export field, and the
parity surface families).

- **Boundary schema:**
  [`schemas/ui/m5-deployment-summary-primitive.schema.json`](../../schemas/ui/m5-deployment-summary-primitive.schema.json)
- **Frozen matrix contract it narrows:**
  [`schemas/ui/m5-deployment-continuity-component-matrix.schema.json`](../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
- **Release proof (canonical):**
  [`artifacts/release/m5-deployment-summary-primitive-proof/support_export.json`](../../artifacts/release/m5-deployment-summary-primitive-proof/support_export.json)
- **Protected fixtures:**
  [`fixtures/ui/m5-deployment-summary-primitive/`](../../fixtures/ui/m5-deployment-summary-primitive/)
- **Implementation:**
  `crates/aureline-install/src/implement_the_m5_deployment_summary_residual_dependency_and_control_data_plane_primitive/`

## What the resolver projects

`resolve_deployment_summary(&M5DeploymentSummaryInput)` returns a
`M5ResolvedDeploymentSummary` with three surfaces that all carry the same
`deployment_id`:

| Surface | Resolved type | Carries |
| --- | --- | --- |
| Deployment summary card | `M5ResolvedDeploymentSummaryCard` | deployment scope, operating mode, tenant / org, region, mirror / offline posture, last control-plane sync, and open-details / export actions |
| Residual-dependency rows | `Vec<M5ResolvedResidualDependencyRow>` | each still-vendor-hosted service, its class, its exact failure consequence, and its disable / alternative path |
| Control-plane/data-plane status strip | `M5ResolvedControlDataPlaneStatusStrip` | the distinct control-plane and data-plane health, whether the local runtime is impaired, and the local-safe next step |

## Acceptance criteria the resolver proves

- **AC1 — a self-hosted or sovereign surface never implies a stronger boundary than
  the running deployment provides.** A deployment scope that claims reduced vendor
  dependency (self-hosted, sovereign, local-only) may never hide a required residual
  vendor dependency; an undisclosed required residual dependency under such a scope is
  rejected as `BoundaryOverclaimed`, and the summary card records
  `boundary_honestly_scoped`.
- **AC2 — control-plane degradation is distinguishable from local-runtime continuity
  without opening raw diagnostics.** The status strip keeps the control-plane and
  data-plane states distinct and keeps a local-safe next step visible; a control-plane
  impairment flagged as a local-runtime failure is rejected as
  `ControlPlaneMaskedAsLocal`.
- **AC3 — residual vendor dependency is explicit and exportable.** Every residual row
  names the vendor service, its failure consequence, and its mitigation path, is
  disclosed (an undisclosed residual row is rejected as
  `ResidualDependencyUndisclosed`), and is carried in the export
  (`residual_dependency_exportable`).

## Honesty guarantees

- Raw config bytes, credentials, license keys, mirror URLs, and device identifiers
  never cross this boundary; the resolver carries only opaque refs, typed class
  tokens, booleans, and redacted labels.
- A degraded input must carry a precise, non-generic label; a generic non-answer
  (`unavailable`, `error`, `offline`, …) is rejected.
- The support / export packet reconstructs exactly what each surface would have shown:
  every worked case stores both its input and its resolved projection, and validation
  re-runs the resolver so a stored projection can never drift from the live resolver.

## Parity matrix

The `M5DeploymentSummaryPrimitivePacket` binds each of the six deployment surface
families (About deployment card, admin deployment console, service-health panel,
diagnostics deployment, support / export replay, docs deployment reference) to the
shared contract with worked resolution cases, a frozen controlled-vocabulary set,
governance-review and consumer-projection blocks, and a release / support parity
posture. See the
[matrix CSV](../../artifacts/release/m5-deployment-summary-primitive-proof/matrix.csv)
and [report](../../artifacts/release/m5-deployment-summary-primitive-proof/report.md)
for the per-surface summary.
