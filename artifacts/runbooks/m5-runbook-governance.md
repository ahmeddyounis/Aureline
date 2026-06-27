# M5 runbook governance matrix

This is the canonical contract inventory for Aureline's runbook object model. It
names every governed runbook object, its owner, its first consumer, the schema
that is its source of truth, and the proof packet that keeps it current, plus the
release-gating behavior that keeps claimed runbook-backed surfaces honest.

Aureline markets runbooks as **governed executable guidance**, not rich-text
suggestions. Every runbook declares where its authority comes from (a source
class), what class of step is being run, what scope or approval each step
requires, what evidence outputs are expected, and how console/browser pivots and
archived execution history stay attributable. Companions may follow or request
within declared scope but cannot mint hidden privileged mutate channels.

The matrix and its instances are produced from one checked-in source of truth in
`crates/aureline-runbooks` (`m5_runbook_governance`). Do not hand-edit the JSON
under `artifacts/` or `fixtures/`; re-mint it with the headless emitter (below).

## Governed runbook objects

| Object | Source class set | Owner | First consumer | Source-of-truth schema | Proof packet |
|--------|------------------|-------|----------------|------------------------|--------------|
| `source_descriptor` | where authority comes from | `runbook_authoring_owner` | `docs_help` | `schemas/runbooks/m5-runbook-source.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` |
| `step_descriptor` | what class of step runs | `runbook_authoring_owner` | `operator_dashboard` | `schemas/runbooks/m5-runbook-step.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` |
| `execution_record` | what actually ran | `incident_operations_owner` | `incident_workspace` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` |
| `deviation_note` | departures from guidance | `incident_operations_owner` | `incident_workspace` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` |
| `control_plane_handoff` | console/browser pivots | `control_plane_boundary_owner` | `operator_dashboard` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` |
| `archival_export` | retained, export-safe history | `support_export_owner` | `support_bundle` | `schemas/runbooks/m5-runbook-execution.schema.json` | `artifacts/release/m5-runbook-proof/runbook-governance.json` |

The execution record embeds deviation notes, control-plane handoff packets, and
the archival/export object, so the execution schema is the source of truth for
those four object classes; the source and step descriptors are also published as
standalone schemas for descriptor-only artifacts.

## Controlled vocabularies

- **Source classes** — `vendored_first_party`, `organization_authored`,
  `imported_vendor_console`, `companion_drafted`, `archived_execution`. Imported
  vendor-console references and companion drafts carry no standing execution
  authority.
- **Step classes** — `inspect`, `diagnose`, `mitigate`, `rollback`,
  `console_handoff`, `approval`, `annotate`. `mitigate` and `rollback` mutate
  target state.
- **Approval scopes** — `no_approval_read_only`, `scoped_self_approve`,
  `requires_human_approval`, `requires_privileged_approval`,
  `prohibited_hidden_mutate`.
- **Deviation classes** — `no_deviation`, `parameter_adjusted`, `step_skipped`,
  `step_added_ad_hoc`, `aborted_mid_step`, `console_pivot_unplanned`.
- **Control-plane boundaries** — `in_app_governed`, `browser_handoff`,
  `vendor_console_handoff`, `auth_boundary_cross`.

## Release gating

Each claimed runbook-backed surface binds the governed objects it depends on. The
matrix derives, per surface:

- a **status** (`mapped` / `provisional` / `unmapped`) reflecting *true* coverage,
  independent of waivers, so the matrix never hides a real gap;
- a **gate decision** (`governed` / `narrowed` / `blocked`) the release center and
  public-truth automation read;
- an **effective claim** after the gate applies.

Stable promotion **fails** when a claimed surface binds an object the matrix does
not govern, or whose proof is missing (`blocked`). A surface whose proof is stale
auto-**narrows** below Stable (`narrowed`). A blocking gap can be accepted only
under a disclosed, time-bounded waiver scoped to a single object; the surface then
ships at the waived claim, but its true status stays red.

## Operator scenarios

The execution-record fixtures under `fixtures/runbooks/m5-operator-scenarios/`
demonstrate the object model end to end:

- `restart_pipeline_governed` — a clean, governed execution (inspect, diagnose,
  human-approved mitigation), all in the governed plane.
- `failover_deviation_lineage` — a skipped declared step plus an ad-hoc rollback
  under privileged approval, with the deviation lineage recorded and attributable.
- `vendor_console_handoff` — an attributable pivot to a vendor console that
  returns to the governed plane and mints no hidden mutate channel.
- `companion_within_scope` — a companion that follows and requests within declared
  read-only/annotate scope and never drives a mutating step.

The governance drills under `fixtures/runbooks/m5-governance-drills/` exercise the
gate: `stale_proof_narrowed`, `missing_proof_blocked`, and `waived_narrowed`.

## Re-minting

All JSON and Markdown here are generated from `crates/aureline-runbooks`:

```sh
BIN="cargo run -q -p aureline-runbooks --bin aureline_runbooks_m5_runbook_governance --"
$BIN validate
$BIN support-export > artifacts/runbooks/m5-runbook-governance.json
$BIN support-export > artifacts/release/m5-runbook-proof/runbook-governance.json
$BIN matrix        > artifacts/runbooks/m5-runbook-governance-matrix.json
$BIN markdown      > artifacts/release/m5-runbook-proof/runbook-governance-proof.md
$BIN fixture-stale-proof-narrowed  > fixtures/runbooks/m5-governance-drills/stale_proof_narrowed.json
$BIN fixture-missing-proof-blocked > fixtures/runbooks/m5-governance-drills/missing_proof_blocked.json
$BIN fixture-waived-narrowed       > fixtures/runbooks/m5-governance-drills/waived_narrowed.json
$BIN scenario restart-pipeline-governed  > fixtures/runbooks/m5-operator-scenarios/restart_pipeline_governed.json
$BIN scenario failover-deviation-lineage > fixtures/runbooks/m5-operator-scenarios/failover_deviation_lineage.json
$BIN scenario vendor-console-handoff     > fixtures/runbooks/m5-operator-scenarios/vendor_console_handoff.json
$BIN scenario companion-within-scope     > fixtures/runbooks/m5-operator-scenarios/companion_within_scope.json
```
