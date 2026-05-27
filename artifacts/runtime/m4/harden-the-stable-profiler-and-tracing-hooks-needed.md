# Harden the stable profiler and tracing hooks needed for diagnosis while keeping richer suites out of the M4 contract — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
profiler and tracing-hooks truth packet. The cross-tool boundary schema
lives at
[`schemas/runtime/harden_the_stable_profiler_and_tracing_hooks_needed_truth.schema.json`](../../../schemas/runtime/harden_the_stable_profiler_and_tracing_hooks_needed_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/harden_the_stable_profiler_and_tracing_hooks_needed/`](../../../crates/aureline-runtime/src/harden_the_stable_profiler_and_tracing_hooks_needed/),
and the checked-in stable packet at
[`artifacts/runtime/m4/harden_the_stable_profiler_and_tracing_hooks_needed_truth_packet.json`](../../../artifacts/runtime/m4/harden_the_stable_profiler_and_tracing_hooks_needed_truth_packet.json).

The packet pins one boundary truth that the flamegraph surface,
timeline surface, call-tree surface, regression-summary surface,
replay-controls surface, profile-session surface, CLI/headless
inspector, evidence export, support export, release proof index,
Help/About proof card, and the conformance dashboard all read.
Surfaces MUST NOT mint local copies, flatten capture-state into
generic labels, paraphrase origin or build-mode disclosures, or imply
M5-class capture, replay, or regression depth on M4. Surfaces MUST
project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host profiler sessions.
- `remote_helper_lane` — SSH / remote-agent profiler sessions.
- `container_lane` — container-attached profiler sessions.
- `ci_import_lane` — CI-imported profiler evidence sessions.

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `profiler_quality` — the lane headline. Required at `launch_stable`
  for any lane that claims the M4 grade.
- `wedge_admission` — one row per profiler wedge
  (`profile_session_descriptor`, `trace_bundle_manifest`,
  `capture_mode_label`, `mapping_quality_state`,
  `baseline_comparison_key`, `replay_capability_descriptor`,
  `reverse_step_disabled_reason`, `export_redaction_summary`). All
  eight required for any `launch_stable` lane.
- `capture_state_admission` — one row per capture state (`live`,
  `cached`, `imported`, `stale`, `not_recorded`,
  `disabled_with_reason`). All six required for any `launch_stable`
  lane.
- `origin_class_admission` — one row per origin class (`local_origin`,
  `remote_origin`, `ci_artifact_origin`, `imported_bundle_origin`).
  All four required for any `launch_stable` lane.
- `build_mode_admission` — one row per build mode (`debug_mode`,
  `release_mode`). Both required for any `launch_stable` lane.
- `run_class_admission` — one row per run class (`warm_run`,
  `cold_run`). Both required for any `launch_stable` lane.
- `confounder_admission` — one row per confounder (`hardware_class`,
  `power_state`, `thermal_state`). All three required for any
  `launch_stable` lane.
- `replay_state_admission` — one row per replay state (`supported`,
  `limited`, `record_only`, `profile_only`, `import_view_only`). All
  five required for any `launch_stable` lane.
- `surface_binding` — one row per profiler surface
  (`flamegraph_surface`, `timeline_surface`, `call_tree_surface`,
  `regression_summary_surface`, `replay_controls_surface`,
  `profile_session_surface`). All six required for any `launch_stable`
  lane. Each row MUST attest the capture-state and replay-state
  vocabularies it is required to preserve.
- `lineage_admission` — binds the stable `execution_context_id` into
  emitted profile sessions, trace bundles, comparison packets, and
  support exports. Required for every `launch_stable` lane and MUST
  surface a non-empty `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Profiler wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `profile_session_descriptor` — typed profile-session descriptor
  binding capture mode, source, execution context, build/runtime
  identity, target, capture window, overhead class, mapping quality,
  data posture, trace-bundle ref, comparison key, and export policy.
- `trace_bundle_manifest` — immutable trace-bundle manifest
  distinguishing raw bundles from derived flamegraph, call-tree,
  timeline, regression, or advisory views.
- `capture_mode_label` — honest capture-mode disclosure (cpu_sample,
  instrumented_cpu_profile, wall_time_trace, memory_sample,
  allocation_snapshot, replay_recording, imported_profile).
- `mapping_quality_state` — symbol and source-map fidelity state
  (exact_symbols_and_sources, exact_symbols_partial_sources,
  approximate_symbols, symbolized_with_partial_source_maps,
  raw_addresses_only, stale_source_maps, unresolved).
- `baseline_comparison_key` — baseline/comparison key so any exposed
  performance evidence stays attributable to one execution context,
  build/runtime identity, and target.
- `replay_capability_descriptor` — honest replay-capability disclosure
  with backend family, supported ranges, feature matrix, lane state,
  determinism caveats, and overhead/storage band.
- `reverse_step_disabled_reason` — explicit `disabled_with_reason`
  state where reverse-step is not supported, instead of implying
  time-travel capability.
- `export_redaction_summary` — export/redaction posture binding
  data class, redaction mode, retention class, and support-bundle
  policy.

## Capture states (required per `launch_stable` lane)

- `live` — live capture.
- `cached` — cached snapshot.
- `imported` — imported evidence.
- `stale` — stale evidence.
- `not_recorded` — no recording available.
- `disabled_with_reason` — recording disabled with explicit reason.

## Origin classes (required per `launch_stable` lane)

- `local_origin` — captured locally.
- `remote_origin` — captured remotely.
- `ci_artifact_origin` — imported from CI artifacts.
- `imported_bundle_origin` — imported from a trace/support bundle.

## Build modes (required per `launch_stable` lane)

- `debug_mode` — debug build.
- `release_mode` — release build.

## Run classes (required per `launch_stable` lane)

- `warm_run` — warm run.
- `cold_run` — cold run.

## Confounders (required per `launch_stable` lane)

- `hardware_class` — hardware class or reference profile.
- `power_state` — power mode during capture.
- `thermal_state` — thermal state during capture.

## Replay states (required per `launch_stable` lane)

- `supported` — replay and reverse stepping are supported.
- `limited` — replay is available with explicit limitations.
- `record_only` — recording is supported, but replay is not.
- `profile_only` — profiling is supported, but record/replay is not.
- `import_view_only` — imported recordings can be inspected but not
  controlled live.

## Consumer projection surfaces (required)

Every stable packet MUST include a preserved consumer projection for:

- `flamegraph_surface`
- `timeline_surface`
- `call_tree_surface`
- `regression_summary_surface`
- `replay_controls_surface`
- `profile_session_surface`
- `cli_headless`
- `evidence_export`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

## Honest M4 contract

The packet keeps M4 honest by qualifying richer replay, reverse-debug,
and chronology surfaces explicitly. If a backend/runtime row lacks
current capture, support-class, or export packets, the packet narrows
that surface below `launch_stable` instead of letting profiler/trace
chrome imply time-travel capability. Stable diagnosis hooks still
expose `not_recorded` and `restart with recording` truth where
adapters support it, while leaving unqualified replay backends visibly
out of the M4 support contract.
