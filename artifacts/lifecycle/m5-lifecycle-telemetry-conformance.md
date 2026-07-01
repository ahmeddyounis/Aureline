# M5 lifecycle telemetry conformance: stable lifecycle enums, transition events, and export parity across logs, dashboards, and packets

Generated from the seeded packet in
[`crate::m5_lifecycle_telemetry_conformance`](../../crates/aureline-shell/src/m5_lifecycle_telemetry_conformance/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- markdown > \
  artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md
```

- Packet id: `m5-lifecycle-telemetry-conformance:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-lifecycle-telemetry-conformance.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required telemetry dimensions: `enum_emission`, `transition_event`, `ui_export_parity`, `shared_contract_consumption`
- Object families certified: 13
- Green (full conformance): 9
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Object family | Status | Enum emission | Transition event | UI/export parity | Shared contract | Headless | Waiver |
| ------------- | ------ | ------------- | ---------------- | ---------------- | --------------- | -------- | ------ |
| Workspace / window session | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Installed extension | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Remote / tunnel session | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Collaboration session | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| AI assistant action | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Update / rollback | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Notebook runtime | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Request / API run | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Preview / live-server session | `yellow` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `disclosed_partial_contract_adoption` | `true` | — |
| Pipeline / task run | `yellow` | `stable_enums_emitted_to_every_sink` | `disclosed_coarse_transition_events` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Data / database session | `green` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Profiler / trace capture | `yellow` | `disclosed_reduced_enum_sink_set` | `transition_events_emitted_with_attribution` | `ui_and_export_naming_and_fields_agree` | `shared_contract_consumed_no_local_prose` | `true` | — |
| Companion / paired-device session | `yellow` | `stable_enums_emitted_to_every_sink` | `transition_events_emitted_with_attribution` | `disclosed_export_field_narrowing` | `shared_contract_consumed_no_local_prose` | `true` | `waiver:companion-export-field-narrowing:0001` |

## Auto-narrowed rows

- `preview_session` (`yellow`) — On the legacy preview diagnostics surface the preview session takes a disclosed partial adoption of the shared lifecycle contract — the status code is resolved from the shared contract while one legacy build-detail field still renders a disclosed local label — so the contract consumption is narrowed and disclosed rather than replaced by local prose.
- `pipeline_run` (`yellow`) — On a long fan-out pipeline the run emits disclosed coarse-grained transition events — one event per checkpoint boundary rather than per intermediate stage transition — while still attributing each event to the executing subsystem, so the transition telemetry is narrowed and disclosed rather than anonymous.
- `profiler_capture` (`yellow`) — On a constrained trace-capture build the profiler emits its stable lifecycle and checkpoint enums into a disclosed reduced sink set — the structured-log emission is folded into the telemetry stream while stable enums are still emitted into telemetry, dashboards, and support exports — so the sink coverage is narrowed and disclosed rather than dropping the controlled vocabulary.
- `companion_session` (`yellow`) — On the small companion / paired-device export the session carries a disclosed, waivered reduced field detail — one intermediate checkpoint boundary is collapsed in the compact export while the terminal status code, last-failure reason, and recovery affordance are still exported under the same names the companion UI shows — so the export parity is narrowed and disclosed rather than drifted.

## Exact conformance causes

- `preview_session` — `upstream_dependency_narrowed` (disclosed: `true`) — A downstream consumer takes a disclosed partial adoption of the shared lifecycle contract on a legacy surface — resolving the status code from the shared contract while still rendering a disclosed local label for one legacy field — so the contract consumption is narrowed and disclosed rather than replaced by local prose.
- `pipeline_run` — `upstream_dependency_narrowed` (disclosed: `true`) — The object emits disclosed coarse-grained transition events on a constrained build — one event per checkpoint boundary rather than per intermediate transition — while still attributing each event to a controlled actor or subsystem, so the transition telemetry is narrowed and disclosed rather than anonymous.
- `profiler_capture` — `upstream_dependency_narrowed` (disclosed: `true`) — The object emits its stable lifecycle and checkpoint enums into a disclosed reduced set of telemetry sinks on a constrained build — for example folding the structured-log emission into the telemetry stream while still emitting stable enums into telemetry, dashboards, and support exports — so the sink coverage is narrowed and disclosed rather than dropping the controlled vocabulary.
- `companion_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The export path carries a disclosed, waivered reduced field detail on a compact export — collapsing one intermediate checkpoint boundary while still exporting the terminal status code, last-failure reason, and recovery affordance under the same names the UI shows — so the export parity is narrowed and disclosed rather than drifted.

## Active waivers

- `waiver:companion-export-field-narrowing:0001` (`companion_session`, owner: Companion owner, expires `2026-09-30T00:00:00Z`) — On the small companion / paired-device export the session carries a disclosed reduced field detail — one intermediate checkpoint boundary is collapsed in the compact export while the terminal status code, the last-failure reason, and the recovery affordance are still exported under the same names the companion UI shows — while still disclosing that the export was narrowed. The narrowing is disclosed, never silent, and the full per-transition field detail is restored the moment the companion reattaches to a standard-width export surface.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- validate
cargo test -p aureline-shell --test m5_lifecycle_telemetry_conformance_fixtures
```
