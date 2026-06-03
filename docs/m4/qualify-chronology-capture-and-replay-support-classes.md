# Qualify chronology capture and replay/reverse-debug support classes across debug lanes — M4 truth packet

This document is the reviewer-facing contract for the M4 chronology-capture and
replay/reverse-debug support-class qualification truth packet. The cross-tool
boundary schema lives at
[`schemas/debug/chronology-replay-support.schema.json`](../../../schemas/debug/chronology-replay-support.schema.json),
the canonical Rust contract at
[`crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/`](../../../crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/),
and the checked-in stable packet at
[`artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json`](../../../artifacts/runtime/m4/qualify_chronology_capture_and_replay_support_classes_truth_packet.json).

The packet pins one boundary truth that the debugger UI, timeline scrubber,
reverse-step toolbar, variable inspector, call-stack panel, evaluate console,
support export, compare card, CLI/headless inspector, evidence export, release
proof index, Help/About proof card, and the conformance dashboard all read.
Surfaces MUST NOT mint local copies of chronology or replay support claims,
flatten capture-state into generic labels, paraphrase mapping-quality or
inspector-state disclosures, or imply M5-class reverse-debug depth on M4.
Surfaces MUST project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host debug sessions with local DAP adapter.
- `remote_helper_lane` — SSH / remote-agent debug sessions.
- `container_lane` — container-attached debug sessions.
- `notebook_bridge_lane` — notebook bridge / kernel-debug sessions.

Adding or removing a lane is a vocabulary change that requires bumping the
schema version and updating the Rust contract, the artifact, the fixture
corpus, and this document together.

## Row classes (closed vocabulary)

- `chronology_quality` — the lane headline. Required at `launch_stable` for any
  lane that claims the M4 grade.
- `support_class_admission` — one row per replay support class
  (`supported`, `limited`, `view_only`, `unsupported`, `policy_blocked`). All
  five required for any `launch_stable` lane.
- `capture_state_admission` — one row per chronology capture state
  (`recorded`, `not_recorded`, `restart_with_recording_available`,
  `capture_unsupported`). All four required for any `launch_stable` lane.
- `mapping_quality_badge_admission` — one row per mapping quality badge
  (`exact`, `approximate`, `partial`, `unavailable`, `stale`, `mismatched`).
  All six required for any `launch_stable` lane.
- `replay_scope_admission` — one row per replay scope class (`local_scope`,
  `remote_scope`, `notebook_bridge_scope`). All three required for any
  `launch_stable` lane.
- `inspector_state_admission` — one row per inspector state class (`live`,
  `snapshot`, `stale`, `limited`, `unavailable`). All five required for any
  `launch_stable` lane.
- `restart_posture_admission` — one row per restart-with-recording posture
  (`available`, `unavailable_unsupported_backend`, `unavailable_policy_blocked`,
  `unavailable_no_live_session`). All four required for any `launch_stable` lane.
- `replay_surface_binding` — one row per replay-exposed surface
  (`debugger_ui_surface`, `timeline_scrubber_surface`,
  `reverse_step_controls_surface`, `variable_inspector_surface`,
  `call_stack_surface`, `evaluate_surface`, `support_export_surface`,
  `compare_card_surface`). All eight required for any `launch_stable` lane.
  Every surface except `support_export_surface` MUST attest
  `attests_replay_read_only=true`; inspector-state surfaces MUST attest
  `attests_inspector_state_preserved=true`; mapping-quality surfaces MUST
  attest `attests_mapping_quality_preserved=true`.
- `lineage_admission` — binds the stable `execution_context_id` into emitted
  chronology packets, session exports, and support bundles. Required for every
  `launch_stable` lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each must carry
  its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`, `beta_grade_only`,
`preview_only`, and `unsupported` are the precise narrowed labels; each narrowed
row MUST surface a disclosure ref. `support_unbound` never qualifies for stable
promotion.

## Replay support classes (required per `launch_stable` lane)

- `supported` — full chronology capture and replay available.
- `limited` — replay available with disclosed limitations.
- `view_only` — recordings can be inspected but reverse-step control is
  unavailable.
- `unsupported` — no replay capability for this lane.
- `policy_blocked` — replay capability blocked by policy; disclosed reason
  required.

## Chronology capture states (required per `launch_stable` lane)

- `recorded` — session is recorded; replay and time-travel may be available.
- `not_recorded` — session is not recorded.
- `restart_with_recording_available` — no current recording but restart with
  recording can be offered.
- `capture_unsupported` — the backend does not support chronology capture.

## Mapping quality badges (required per `launch_stable` lane)

- `exact` — symbol and source-map fidelity is exact.
- `approximate` — symbols are resolved but source maps may be approximate.
- `partial` — partial symbolization; some frames may be raw.
- `unavailable` — mapping is unavailable for this lane state.
- `stale` — source maps are present but stale relative to the live session.
- `mismatched` — symbol/source mismatch detected; disclosed reason required.

## Replay scope classes (required per `launch_stable` lane)

- `local_scope` — replay operates within a single local session.
- `remote_scope` — replay state is synchronized with a remote adapter.
- `notebook_bridge_scope` — replay is mediated through the notebook bridge.

## Inspector state classes (required per `launch_stable` lane)

- `live` — inspector reflects live execution state.
- `snapshot` — inspector reflects a point-in-time snapshot.
- `stale` — inspector data is stale; a reconnect or step is needed.
- `limited` — inspector is available with disclosed limitations.
- `unavailable` — inspector is not available for this lane state.

## Restart-with-recording postures (required per `launch_stable` lane)

- `available` — restart with recording can be offered to the user.
- `unavailable_unsupported_backend` — restart with recording is not supported
  by the backend.
- `unavailable_policy_blocked` — restart with recording is blocked by policy.
- `unavailable_no_live_session` — no live session is present; restart with
  recording cannot be initiated.

## Replay surface bindings (required per `launch_stable` lane)

All eight surfaces are required. Attestation rules:

- Every surface except `support_export_surface` MUST attest
  `attests_replay_read_only=true`. Replay surfaces MUST remain read-only unless
  a separate live-control path is active.
- `variable_inspector_surface`, `call_stack_surface`, `evaluate_surface`, and
  `support_export_surface` MUST attest `attests_inspector_state_preserved=true`.
- `call_stack_surface`, `variable_inspector_surface`, `evaluate_surface`,
  `support_export_surface`, and `compare_card_surface` MUST attest
  `attests_mapping_quality_preserved=true`.

## Consumer projection surfaces (required)

Every stable packet MUST include a preserved consumer projection for:

- `debugger_ui`
- `timeline_scrubber`
- `reverse_step_toolbar`
- `variable_inspector`
- `call_stack_panel`
- `evaluate_console`
- `support_export`
- `compare_card`
- `cli_headless`
- `evidence_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

## Honest M4 contract

The packet keeps M4 honest by qualifying chronology capture and reverse-debug
surfaces explicitly. If a backend or runtime row lacks current capture state,
replay support, or inspector-state packets, the packet narrows that surface
below `launch_stable` instead of letting the debugger chrome imply time-travel
capability. Stable hooks still expose `not_recorded` and
`restart_with_recording_available` truth where adapters support it, while
leaving unqualified replay backends visibly out of the M4 support contract.
