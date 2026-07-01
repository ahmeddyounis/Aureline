# M5 lifecycle telemetry conformance contract

This lane is the **lifecycle-telemetry emission and export-parity conformance capstone** on top of the
frozen [M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes, for every long-lived M5 object family, an explicit state machine, one visible primary status
surface, one exportable status code, one controlled last-failure reason, one named recovery
affordance, and an ordered inventory of milestone checkpoints. This lane certifies that the same
controlled lifecycle vocabulary **survives the machine paths** — telemetry, structured logs,
dashboards, and support-packet exports — so M5 state truth is diagnosable from one shared contract in
logs, dashboards, and packets rather than drifting by surface or disappearing in export paths.

The lane exists so that M5 can honestly ship its growing mix of notebook, data/API, AI, remote,
preview, pipeline, docs, and support surfaces without object state, checkpoint boundaries, and
recovery vocabulary drifting between what a user sees in the UI and what a log, dashboard, support
packet, or claim manifest records. Shiproom and Support Center must be able to diagnose state truth
from one shared contract, not from surface-local prose.

## Certified object families

The certification covers exactly the thirteen governed object families the matrix freezes, and refuses
to ship if any is missing: `workspace`, `extension`, `remote_session`, `collaboration_session`,
`ai_action`, `update_rollback`, `notebook_runtime`, `request_api_run`, `preview_session`,
`pipeline_run`, `data_session`, `profiler_capture`, and `companion_session`.

Every attribute a row certifies over — the driving matrix journey, the object's explicit state machine
(the admitted controlled states, always including `ready`), the one visible primary status surface,
the one exportable status-code field, the one last-failure-reason field, the named recovery affordance
the mandatory-field conformance anchors on, the controlled last-failure reason classes, the checkpoint
lineage the transition events replay, the declared consumer surfaces, and the applicable downgrade
triggers — is pulled straight from the frozen matrix's seeded packet, so this lane mints no parallel
lifecycle vocabulary and cannot certify a family the matrix does not anchor. Only the telemetry sinks
emitted, the mandatory fields kept conformant, the per-family posture, and the scope summary are
authored here.

## Telemetry sinks and mandatory fields

Each row proves it emits its stable lifecycle and checkpoint enums into all four **telemetry sinks** —
the machine paths M5 state truth must survive:

- `telemetry` — the telemetry event stream.
- `structured_logs` — structured application logs.
- `dashboards` — operator / Shiproom dashboards.
- `support_exports` — support-packet exports.

Each row also proves it keeps all three **mandatory fields** conformant across the UI and export paths
— the exact fields the conformance suite fails on when a surface skips one:

- `last_failure_reason` — the controlled last-failure reason field.
- `recovery_affordance` — the named recovery affordance field.
- `checkpoint_boundary` — the milestone checkpoint-boundary field.

## Certified telemetry dimensions

Each row is certified across the four telemetry dimensions the acceptance criteria and implementation
requirements demand (`enum_emission`, `transition_event`, `ui_export_parity`,
`shared_contract_consumption`):

- **enum emission** — `stable_enums_emitted_to_every_sink` (green: every sink carries the stable
  lifecycle and checkpoint enum tokens), a disclosed `disclosed_reduced_enum_sink_set` where the
  stable enums are emitted into a reduced sink set on a constrained build while still covering the rest
  (yellow), or `enums_absent_or_local_prose_emitted` (red: a sink was dropped or human prose was
  emitted instead of the stable enum tokens).
- **transition event** — `transition_events_emitted_with_attribution` (green: every transition emits a
  controlled event carrying the from/to states and the controlled actor or subsystem), a disclosed
  `disclosed_coarse_transition_events` where one event per checkpoint boundary is emitted while still
  attributed (yellow), or `transition_events_missing_or_anonymous` (red: no event, or one with no
  controlled attribution, so a state change is an anonymous jump).
- **ui/export parity** — `ui_and_export_naming_and_fields_agree` (green: the status code, last-failure
  reason, recovery affordance, and checkpoint boundary the UI shows are named and shaped identically in
  the export path), a disclosed `disclosed_export_field_narrowing` where the export carries a reduced
  field detail while the terminal fields stay named identically (yellow, **requires an active
  waiver**), or `ui_export_lifecycle_naming_or_fields_drifted` (red: the paths disagree on a lifecycle
  name or drop a required field).
- **shared contract consumption** — `shared_contract_consumed_no_local_prose` (green: Support Center,
  diagnostics, and claim tooling resolve the object's state through the one shared contract), a
  disclosed `disclosed_partial_contract_adoption` where a legacy surface renders one disclosed local
  label while still resolving the rest from the contract (yellow), or
  `local_prose_replaces_shared_contract` (red: a consumer replaced the shared contract with local
  prose, so Shiproom and Support Center can no longer diagnose state truth from one contract).

A `headless_parity_preserved` flag records that the same state-truth vocabulary survives a headless or
companion-adjacent execution; losing it is a hard blocker. An incomplete telemetry-sink set or
mandatory-field set is likewise a hard blocker — it cannot prove that lifecycle enums appear in every
machine path or that no mandatory field is skipped.

## Auto-narrowing and completeness

Each row's green/yellow/red status is **derived**, never asserted. Any hard blocker — absent enums or
local prose emitted, transition events missing or anonymous, a UI/export lifecycle-naming or field
drift, the shared contract replaced by local prose, a headless/companion-adjacent vocabulary loss, an
incomplete telemetry-sink or mandatory-field set, or a row that did not certify every consumer surface
the matrix declares for the family — forces `red`; any disclosed narrowing forces `yellow`; otherwise
`green`. A disclosed export-field narrowing must carry an active waiver to stay publishable, and every
non-green row must disclose a reason. The consumer-surface, telemetry-sink, and mandatory-field
completeness checks are the conformance lints that fail when a UI and export path drift on lifecycle
naming or required fields.

The seeded certification is **9 green** and **4 yellow** (profiler capture disclosing a reduced
telemetry-sink set on a constrained build, pipeline run disclosing coarse transition events on long
fan-outs, preview session disclosing a partial shared-contract adoption on a legacy diagnostics
surface, and companion session with a waivered export-field narrowing on its small paired-device
export), with **0 red**. Five protected blocked fixtures prove the red path for each
acceptance-criteria failure mode: the notebook runtime emitting local prose with a dropped sink, the
remote session firing missing or anonymous transition events, the data session drifting its UI and
export naming, the AI action's claim tooling replacing the shared contract with local prose, and the
extension losing headless parity.

## Artifacts

- Schema: `schemas/lifecycle/m5-lifecycle-telemetry-conformance.schema.json`
- Report: `artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md`
- Proof packet: `artifacts/release/m5-lifecycle-telemetry-conformance-proof/packet.json`
- Proof dashboard: `artifacts/release/m5-lifecycle-telemetry-conformance-proof/dashboard.json`
- Proof support export: `artifacts/release/m5-lifecycle-telemetry-conformance-proof/support_export.json`
- Proof CSV: `artifacts/release/m5-lifecycle-telemetry-conformance-proof/matrix.csv`
- Fixtures: `fixtures/state/m5-lifecycle-telemetry-conformance/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The Rust validator `validate_m5_lifecycle_telemetry_conformance_packet` in
`crates/aureline-shell/src/m5_lifecycle_telemetry_conformance/` is the authoritative gate; the schema
above documents the shape. The headless emitter `aureline_shell_m5_lifecycle_telemetry_conformance` is
the only mint-from-truth path.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- validate
cargo test -p aureline-shell --test m5_lifecycle_telemetry_conformance_fixtures
cargo test -p aureline-shell --lib m5_lifecycle_telemetry_conformance
```
