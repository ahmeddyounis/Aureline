# M5 support-scenario-picker-row primitive

Status: implemented (B106, task M05-901)

This is the first `implement_` lane that narrows the frozen
[M5 support-intake / escalation component matrix](./m5_support_intake_escalation_component_matrix.md)
into one reusable primitive: the **support-scenario picker row**. It closes the
gap between the deeper Project Doctor probe/finding, recovery-ladder, and
support-bundle systems and the reusable intake component a user actually reads
when they begin diagnosis — so issue classification starts from an explicit
scenario family, a user-facing symptom cue, and a claimed launch/deployment/
profile scope instead of a generic "other" form or free-form guesswork.

Truth source (checked in):

- Schema: `schemas/ui/m5-support-scenario-picker-row.schema.json`
- Support export: `artifacts/release/m5-support-scenario-picker-row-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-support-scenario-picker-row-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-support-scenario-picker-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-support-scenario-picker-row-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_support_support_scenario_picker_row_primitive`; the in-code seed
builders, the checked support export, and the fixtures never drift.

## What the primitive implements

The matrix names the support-scenario picker row as one governed family and
freezes its controlled vocabulary (scenario families, incident scopes, Doctor
finding families, surface families, deployment lines, consumer surfaces,
accessibility routes, qualification classes, and downgrade triggers). This lane
implements that contract as one resolver so a user can tell, from the picker row
alone, which stable scenario family a problem belongs to, the user-facing symptom
cue behind it, the claimed launch/deployment/profile scope, the Doctor finding
family it is bound to, and how to begin diagnosis without ever losing a
same-weight local-only route.

### `resolve_support_scenario_picker_row`

Takes one scenario's family, incident scope, bound Doctor finding family,
user-facing symptom cue, claimed scope label, stable row identity, and a
scenario-diagnosis-blocked signal. Derives the **picker posture** in a fixed
blocking-first order:

1. `scenario_diagnosis_blocked` — the scenario-coded live diagnosis path is
   blocked by policy or unavailability; only the same-weight local-only route
   remains (the scenario-coded start is withheld, never faked).
2. `unmapped_scenario` — the scenario or the finding is uncategorized, not yet
   mapped to a committed Doctor finding family; diagnosis starts by gathering
   evidence.
3. `remote_service_scenario` — the incident scope reaches a remote service or is
   not yet determined; scope is confirmed before diagnosis.
4. `account_or_device_scenario` — the scope reaches the account or device/host;
   scope is confirmed before diagnosis.
5. `workspace_scenario` — the scope is the whole workspace; scenario-coded
   diagnosis is ready.
6. `focused_file_scenario` — the scope is a single file; scenario-coded diagnosis
   is ready.

The scenario family, incident scope, Doctor finding family, symptom cue, and
scope label are carried explicitly, never inferred away. The row always offers
**reveal-scenario-lineage** and the same-weight **start-local-only-diagnosis**
route, offers the scenario-coded **start-diagnosis** only when the path is not
blocked, offers **confirm-scope** when the scope reaches beyond the local
workspace, and always offers **export-scenario**.

## Scenario-family coverage

The frozen scenario-family vocabulary (`crash_recovery`, `performance_health`,
`extension_conflict`, `data_integrity`, `connectivity_sync`,
`uncategorized_scenario`) covers, at minimum, the scenario classes the milestone
requires — execution-context / startup mismatch, extension/host regression,
state corruption / schema drift / low-disk recovery, network/CA/proxy/mirror
failure and remote/route/collaboration mismatch, trust/policy/identity block
(surfaced as a diagnosis-blocked scenario), and the uncategorized fallback. The
seeded worked resolutions exercise every scenario family, and the packet's
`scenario_family_coverage_unproven` lint fails if any is left unexercised.

## Acceptance criteria

- **Scenario-coded start never loses the same-weight local-only route.** Every
  worked resolution offers `start_local_only_diagnosis` and reports
  `local_only_route_available = true`; the `local_route_coverage_unproven` and
  `scenario_coded_start_coverage_unproven` lints enforce that a startable and a
  blocked path are both proven while the local-only route always survives.
- **Scenario rows remain stable across desktop, headless/export, and support
  packet consumers.** The matrix binds Doctor intake, support-center intake,
  recovery-center intake, headless/CLI intake, and support-packet export to the
  same anatomy, scenario families, incident scopes, Doctor finding families,
  postures, actions, export fields, and accessibility routes.

## Governance and redaction

Four hard invariants hold on every row: it never masks its scenario family or
incident scope, never hides the bound Doctor finding lineage, never drops the
same-weight local-only route, and never invents an alternate scenario grammar.
Raw log bodies, pasted paths, credentials, and private endpoints never cross the
support boundary; every symptom cue, scope label, and row identity is carried
only as an opaque, export-safe representation.
