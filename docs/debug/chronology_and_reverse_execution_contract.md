# Chronology recording-mode banner, reverse-step control, and time-travel availability contract

This document freezes one user-visible contract for record / replay /
chronology surfaces inside the debugger. It pins the recording-mode
banner, the timeline scrubber, the snapshot-frame row, and the
reverse-step controls so the product never implies time-travel support
without recorded evidence and an explicit support-level boundary.

The contract is pre-implementation. It defines the reusable record
shapes, the closed vocabularies every consumer reuses, the disabled-
with-exact-reason rules, and the chronology-mismatch state set. It
does **not** implement record / replay backends, time-travel debug
engines, snapshot stores, or scrubber widgets.

Every reviewer, AI assist, support reviewer, and debugger UI consumer
reads the same record family and answers the same five questions
before any reverse step, reverse continue, or timeline scrub is
offered:

> 1. **Is there a recording at all?** A live-only debug session is
>    not a chronology source; reverse stepping cannot be implied
>    against an unrecorded run.
> 2. **What runtime / toolchain is supported, and at what level?**
>    The supported tuple is the contract; missing tuples disable
>    controls with a typed reason instead of dropping them.
> 3. **What does the recording cost and where do its bytes live?**
>    Overhead, storage band, retention class, redaction, and export
>    posture are visible before the user starts a recording.
> 4. **What can the user actually do with this recording?** Allowed
>    verbs come from the capture-session contract; reverse-step
>    controls disable with the exact reason instead of falling back
>    to a generic "not available" string.
> 5. **What broke this chronology?** When recording is partial,
>    expired, runtime-unsupported, or artifact-mismatched after a
>    rebuild, the banner names which class with a recovery /
>    restart action — not a silent missing toolbar.

If this document and the schemas disagree, the schemas win and this
document updates in the same change.

## Companion artifacts

- [`/schemas/debug/recording_mode.schema.json`](../../schemas/debug/recording_mode.schema.json)
  — boundary schema for one `recording_mode_banner_record`.
- [`/schemas/debug/reverse_step_control.schema.json`](../../schemas/debug/reverse_step_control.schema.json)
  — boundary schema for one `reverse_step_control_record` (timeline
  scrubber, snapshot-frame rows, four reverse-step control rows).
- [`/fixtures/debug/chronology_cases/`](../../fixtures/debug/chronology_cases/)
  — worked YAML fixtures covering a supported recorded session, an
  unrecorded live-debug session, partial history, expired capture,
  and artifact mismatch after rebuild.

## Composition with existing contracts

This contract layers on top of existing record families; it never
re-mints them.

- [`/schemas/performance/capture_session.schema.json`](../../schemas/performance/capture_session.schema.json)
  and
  [`/docs/performance/profiling_trace_replay_contract.md`](../performance/profiling_trace_replay_contract.md)
  remain the canonical capture-session truth. The recording-mode
  banner cites a `performance_capture_session_record` through
  `capture_session_ref` and reuses its `recording_mode_state`,
  `chronology_support_class`, `reverse_step_availability_reason`,
  `replay_support_level`, and `replay_verb` vocabularies verbatim.
  This contract does not invent a parallel replay vocabulary.
- [`/schemas/execution/debug_session.schema.json`](../../schemas/execution/debug_session.schema.json)
  and
  [`/docs/execution/debug_truth_contract.md`](../execution/debug_truth_contract.md)
  remain the user-visible debug-session truth model. The banner and
  control set cite a `debug_session_record` through
  `debug_session_ref` so reverse-step authority composes with the
  existing restart / reattach / reopen-dump authority instead of
  forking a parallel one.
- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  and
  [`/docs/debug/artifact_resolution_seed.md`](./artifact_resolution_seed.md)
  remain the symbol / source-map / crash artifact resolution
  contract. Artifact-mismatch states inside this contract resolve
  through the existing `mismatch_reason_class` vocabulary; the
  banner does not mint a new mismatch token when an existing one
  applies.
- [`/schemas/governance/chronology_context.schema.json`](../../schemas/governance/chronology_context.schema.json)
  and
  [`/docs/governance/chronology_context_contract.md`](../governance/chronology_context_contract.md)
  remain the chronology-context contract for history-heavy rows.
  The recording-mode banner is a debug-session surface; it does not
  re-implement evidence-source-class or current-state vocabulary.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  remains the build-identity axes every "artifact-mismatch after
  rebuild" claim verifies field-for-field.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — time-travel debugging, performance-
  regression detection, replay capability, and recording posture
  rows.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §16.13 —
  profiling, trace, time-travel, and performance-regression
  architecture, including the rule that reverse-step or source
  fidelity cannot be claimed without verified backend support and
  that unsupported combinations must surface as `record-only`,
  `profile-only`, or `live-debug-only`, not a collapsed generic
  debug promise.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.3 —
  reverse step, reverse continue, and timeline scrubbing appear
  only when active capture / backend explicitly advertise verbs;
  capture start/stop, storage impact, secret posture, and
  exportability must be visible before recording begins.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §22.15 — recording-
  mode banner, execution timeline scrubber, snapshot-frame row,
  reverse-step controls, and chronology-mismatch banner contract;
  the controlled mismatch states `Recording not started`,
  `Runtime unsupported`, `History partial`, `Artifact mismatch`,
  and `Recording expired` must be stable states with copy and
  shortcuts.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §22.15 —
  record/replay mode declares cost (storage growth, performance
  overhead, privacy/export), reverse-step controls disable with an
  exact reason rather than disappear, and time-travel views are
  tied to build identity and mapping quality.

## 1. Scope

This contract freezes:

1. The `recording_mode_banner_record` shape every record/replay
   debugger surface emits: supported runtime / toolchain tuple,
   current recording state, overhead and storage note, privacy /
   export note, start / stop / restart-with-recording actions, and
   chronology-mismatch state plus recovery action when applicable.
2. The `reverse_step_control_record` shape carrying the timeline
   scrubber descriptor, the snapshot-frame row list, and the four
   reverse-step controls (`step_back_over`, `step_back_into`,
   `step_back_out`, `reverse_continue`). Each control is enumerated
   even when disabled; controls never disappear from the row.
3. The closed vocabularies for: supported-runtime class,
   support-level class, recording-state class, chronology-mismatch
   class, mismatch recovery-action class, banner action class,
   timeline-scrubber state class, marker class, snapshot-frame
   posture class, reverse-step control kind class, control-state
   class, and availability-reason class (re-exported from the
   capture-session contract).
4. The composition rules pinning the banner record to one
   `performance_capture_session_record`, one `debug_session_record`,
   and (when artifact-mismatch is the disabling reason) one
   `debug_artifact_manifest_record` entry.
5. The forbidden-collapse list naming UI / support / export
   shortcuts the contract refuses.

Out of scope (deliberately):

- record / replay backends, time-travel debug engines, snapshot
  stores, deterministic-replay capture pipelines, or kernel-level
  rewind support;
- the timeline-scrubber widget itself, snapshot-frame rendering,
  inline-frame elision, or reverse-decoded source jumps;
- privacy / consent flows beyond the banner's typed disclosure
  fields (the privacy contract still owns the consent decision);
- collaboration handoff between live debug and replay control —
  that remains the debug-truth contract's authority.

## 2. Recording-mode banner

A `recording_mode_banner_record` is the single source of truth the
debugger UI, support-bundle preview, and AI assist read before
rendering any record / replay / time-travel chrome. It carries six
sections.

### 2.1 Identity and joins

- `record_kind` — `recording_mode_banner_record`.
- `recording_mode_banner_schema_version` — integer, current value
  `1`.
- `banner_id` — opaque, stable id for one banner instance.
- `debug_session_ref` — opaque ref to the `debug_session_record`
  this banner sits inside.
- `capture_session_ref` — opaque ref to the
  `performance_capture_session_record` carrying the recording
  state, the replay capability, and the allowed verbs. May be null
  only when `recording_state_class = not_recording` or
  `recording_state_class = recording_unavailable`.
- `debug_artifact_manifest_ref` — opaque ref to the
  `debug_artifact_manifest_record` an artifact-mismatch reason
  resolves through. Required non-null when
  `chronology_mismatch_state_class = artifact_mismatch`.

### 2.2 Supported runtime / toolchain (closed)

- `supported_runtime.runtime_class` — one of
  `native_linux_x86_64`, `native_linux_aarch64`,
  `native_macos_x86_64`, `native_macos_aarch64`,
  `native_windows_x86_64`, `web_browser_runtime`,
  `notebook_kernel_runtime`, `wasm_runtime`,
  `remote_agent_runtime`, `imported_runtime_only`, `unknown_runtime`.
- `supported_runtime.support_level_class` — one of
  `record_replay_supported`, `record_only_supported`,
  `profile_only_supported`, `live_debug_only_supported`,
  `import_only_supported`, `unsupported_runtime_segment`,
  `unsupported_no_backend`.
- `supported_runtime.runtime_label` — short redaction-safe label
  (e.g. `Linux x86_64 (rr backend)`, `macOS arm64 (lldb live-only)`).
- `supported_runtime.toolchain_label` — short redaction-safe label.
- `supported_runtime.support_level_note_ref` — opaque note ref
  pointing at the human-readable support-level explanation. Raw
  prose never crosses this boundary.

Rules (frozen):

1. `support_level_class = record_replay_supported` is the **only**
   class that admits `recording_state_class = live_recording` or
   `recorded`. Every other class collapses to `not_recording`,
   `recording_unavailable`, `imported_recording`, or
   `artifact_mismatch`.
2. `unsupported_runtime_segment` and `unsupported_no_backend`
   force every reverse-step control to `disabled_with_reason`
   under `backend_not_supported`. Controls do not disappear; the
   row still enumerates them with the typed reason.
3. `imported_runtime_only` admits `imported_recording` but never
   `live_recording`; the banner cannot promise live-capture in an
   import-only lane.

### 2.3 Recording state (closed, mirrors capture-session)

- `recording_state_class` — re-exported from
  `capture_session.schema.json`:
  `not_recording`, `live_recording`, `recorded`,
  `imported_recording`, `recording_unavailable`, `expired`,
  `artifact_mismatch`.
- `recording_state_label` — short redaction-safe label rendered
  verbatim in the banner.

Rules (frozen):

1. `live_recording` and `recorded` require non-null
   `capture_session_ref` and a capture session whose
   `capture_class = replay_capture` and whose
   `chronology_support_class` is `partial_replay` or
   `deterministic_replay`. The banner cannot claim a recording
   exists when no capture session backs it.
2. `expired` requires the capture session's `expiry_status` to
   be `expired` and pairs with
   `chronology_mismatch_state_class = recording_expired`.
3. `artifact_mismatch` requires non-null
   `debug_artifact_manifest_ref` and pairs with
   `chronology_mismatch_state_class = artifact_mismatch`. The
   manifest entry must carry one of
   `mismatch_module_build_id`, `mismatch_module_uuid`,
   `mismatch_module_debug_id`, `mismatch_source_map_digest`,
   `mismatch_commit_hash`, `mismatch_tree_hash`,
   `mismatch_toolchain_pin_digest`, or
   `mismatch_lockfile_digest` so the banner can name *what*
   mismatched, not just "something mismatched".

### 2.4 Overhead and storage note

- `overhead_storage.overhead_class` — re-exported from
  `capture_session.schema.json`: `negligible`, `low`,
  `moderate`, `high`, `unknown`, `not_measured`.
- `overhead_storage.storage_band_class` — re-exported:
  `metadata_only`, `small_under_10mb`, `medium_under_250mb`,
  `large_under_5gb`, `very_large_over_5gb`, `unknown`.
- `overhead_storage.overhead_note_ref` — opaque note ref.
- `overhead_storage.storage_note_ref` — opaque note ref.
- `overhead_storage.raw_payload_size_bytes` — non-negative integer
  or null.

Rules (frozen):

1. The banner MUST display overhead and storage **before**
   `start_recording` is offered. A null pair is rendered as
   `unknown`; the action button is not enabled while overhead /
   storage are unknown and the support level requires explicit
   user opt-in.
2. The banner does not embed raw payload bytes, raw paths, or
   raw URLs; only digest refs and short notes cross this
   boundary.

### 2.5 Privacy and export note

- `privacy_export_note.redaction_class_ref` — opaque ref to a
  redaction profile (mirrors the capture-session contract).
- `privacy_export_note.export_posture_class` — one of
  `local_only_no_export`, `summary_export_safe`,
  `raw_export_user_consent_required`, `raw_export_admin_only`,
  `raw_export_blocked_by_policy`.
- `privacy_export_note.opt_in_required` — boolean.
- `privacy_export_note.privacy_note_ref` — opaque note ref.

Rules (frozen):

1. `raw_export_user_consent_required` and
   `raw_export_admin_only` MUST set `opt_in_required = true`. A
   raw export that is not opt-in-gated is non-conforming.
2. `local_only_no_export` MUST disable the
   `export_recording_bookmark` and
   `share_read_only_session` banner actions.

### 2.6 Banner actions (closed)

- `banner_actions[]` is an ordered, deduplicated list of
  `banner_action_class` entries. Each entry carries
  `action_state_class` (`enabled`, `disabled_with_reason`,
  `hidden_inadmissible`) and an
  `action_disable_reason_class` when not `enabled`.
- `banner_action_class` (closed):
  `start_recording`, `stop_recording`,
  `restart_with_recording`, `acknowledge_unavailable_reason`,
  `open_support_level_doc`, `open_runtime_support_doc`,
  `open_artifact_resolution_doc`, `open_capture_settings`,
  `open_privacy_disclosure`, `export_recording_bookmark`,
  `share_read_only_session`, `dismiss_inert`.
- `action_disable_reason_class` (closed):
  `recording_already_active`, `recording_already_stopped`,
  `backend_not_supported`, `runtime_unsupported`,
  `recording_expired_recapture_required`,
  `artifact_mismatch_rebuild_required`,
  `policy_disabled`, `consent_pending`, `not_applicable`.

Rules (frozen):

1. `start_recording` MUST be `disabled_with_reason` whenever
   `support_level_class` is anything other than
   `record_replay_supported` or `record_only_supported`. The
   reason is `backend_not_supported` or `runtime_unsupported`.
2. `stop_recording` is `enabled` only while
   `recording_state_class = live_recording`.
3. `restart_with_recording` is the recovery action paired with
   `recording_expired`, `artifact_mismatch`, and
   `history_partial`. It is `disabled_with_reason
   = backend_not_supported` whenever the runtime cannot host a
   record/replay session at all.
4. `acknowledge_unavailable_reason` is the inert-only action
   used when no recovery exists (e.g. import-only lane on a
   `runtime_unsupported` row); it MUST be present so the user
   has a typed dismissal path instead of a vanished banner.

### 2.7 Chronology-mismatch state (closed)

- `chronology_mismatch_state_class` — null or one of the five
  controlled states the UI/UX spec freezes:
  `recording_not_started`, `runtime_unsupported`,
  `history_partial`, `artifact_mismatch`, `recording_expired`.
- `chronology_mismatch_recovery_action_class` — null or one of
  `start_new_recording`,
  `acknowledge_unsupported_runtime_segment`,
  `open_partial_history_only`,
  `rebuild_then_recapture`,
  `restart_with_recording`,
  `open_runtime_support_doc`,
  `open_artifact_resolution_doc`,
  `open_capture_settings`.

Rules (frozen):

1. `chronology_mismatch_state_class != null` requires
   `chronology_mismatch_recovery_action_class != null`. A
   mismatch banner that names a state but no recovery action is
   non-conforming.
2. The five mismatch states MUST be rendered as stable rows:
   the banner does not collapse `history_partial` and
   `recording_expired` into one generic "no chronology" string.
3. `artifact_mismatch` requires `debug_artifact_manifest_ref`
   to resolve a mismatch reason from the
   `mismatch_reason_class` vocabulary. The banner cites the
   specific mismatch token, not a freeform sentence.

## 3. Reverse-step control set

A `reverse_step_control_record` carries the timeline scrubber, the
snapshot-frame rows, and the four reverse-step controls. The record
is bound to one banner; banner and control set are emitted together.

### 3.1 Identity and joins

- `record_kind` — `reverse_step_control_record`.
- `reverse_step_control_schema_version` — integer, current
  value `1`.
- `control_set_id` — opaque, stable id.
- `recording_mode_banner_ref` — opaque ref to the parent
  `recording_mode_banner_record`. Required non-null.
- `debug_session_ref` — must equal the banner's
  `debug_session_ref`.
- `capture_session_ref` — must equal the banner's
  `capture_session_ref` (or both null).

### 3.2 Timeline scrubber

- `timeline_scrubber.scrubber_state_class` (closed):
  `live_active`, `replay_active`,
  `disabled_no_recording`, `disabled_recording_in_progress`,
  `disabled_recording_expired`,
  `disabled_artifact_mismatch`,
  `disabled_runtime_unsupported`, `disabled_partial_history`.
- `timeline_scrubber.event_ordering_class` (closed):
  `monotonic_single_host`, `wall_clock_utc`,
  `partial_order_only`, `unknown_ordering`.
- `timeline_scrubber.marker_classes[]` (closed; deduplicated):
  `breakpoint_marker`, `bookmark_marker`,
  `signal_marker`, `crash_marker`,
  `recording_start_marker`, `recording_stop_marker`,
  `partial_history_gap_marker`,
  `artifact_mismatch_marker`,
  `expired_segment_marker`,
  `current_frame_marker`.
- `timeline_scrubber.current_frame_time_basis_class` (closed):
  `iso_8601_utc`, `iso_8601_local_with_offset`,
  `monotonic_offset_from_recording_start`,
  `partial_order_label_only`, `not_applicable_no_recording`.
- `timeline_scrubber.current_frame_time_label` — short
  redaction-safe label rendered next to the scrubber.
- `timeline_scrubber.current_recording_note_ref` — opaque note
  ref. Required non-null when
  `scrubber_state_class != disabled_no_recording`.

Rules (frozen):

1. `live_active` requires the banner's
   `recording_state_class = live_recording` and a non-null
   `capture_session_ref`.
2. `replay_active` requires
   `recording_state_class = recorded` or `imported_recording`
   and a capture session whose
   `replay_capability.support_level` is one of
   `replay_view_only`, `replay_read_only`, or
   `replay_interactive`.
3. `disabled_partial_history` MUST include a
   `partial_history_gap_marker` so the gap is visible on the
   scrubber, not silently hidden.
4. `disabled_runtime_unsupported`,
   `disabled_artifact_mismatch`, and
   `disabled_recording_expired` each pair with the matching
   chronology-mismatch state on the banner.

### 3.3 Snapshot-frame rows

`snapshot_frame_rows[]` is an ordered list of frame rows the
scrubber may surface. Each row carries:

- `snapshot_frame_id` — opaque stable id.
- `frame_time_label` — short redaction-safe label.
- `frame_sequence_index` — non-negative integer; monotonic
  inside one recording session.
- `thread_or_process_label` — short redaction-safe label.
- `frame_summary_note_ref` — opaque note ref.
- `mapping_quality_ref` — opaque ref to the
  `mapping_quality_record` for the frame's source/symbol jump.
- `posture_class` (closed):
  `live_frame_no_replay`,
  `recorded_frame_replay_view_only`,
  `recorded_frame_replay_read_only`,
  `recorded_frame_replay_interactive`,
  `partial_history_frame_outside_recording`,
  `expired_frame_inadmissible`,
  `artifact_mismatch_frame_inadmissible`.

Rules (frozen):

1. `recorded_frame_replay_*` postures require the capture
   session's `replay_session.allowed_verbs` to include at
   least `inspect_frame`.
2. `partial_history_frame_outside_recording` rows are still
   listed; the row carries the typed posture so the user can
   see exactly where the recording's coverage stopped.
3. `expired_frame_inadmissible` and
   `artifact_mismatch_frame_inadmissible` rows MUST cite a
   reason (chronology-mismatch state or artifact-manifest
   mismatch token) rather than disappearing from the list.

### 3.4 Reverse-step controls (closed; always enumerated)

`reverse_step_controls[]` is an ordered list of exactly four
entries, one per `control_kind_class`:

- `step_back_over`
- `step_back_into`
- `step_back_out`
- `reverse_continue`

Each entry carries:

- `control_kind_class` — one of the four above.
- `control_state_class` — `enabled` or `disabled_with_reason`.
- `availability_reason_class` — re-exported from
  `capture_session.schema.json`'s
  `reverse_step_availability_reason`:
  `supported`, `not_replay_capture`,
  `backend_not_supported`, `recording_not_started`,
  `recording_expired`, `artifact_mismatch`,
  `symbols_or_runtime_mismatch`, `policy_disabled`,
  `capture_mode_summary_only`.
- `keyboard_shortcut_label` — short redaction-safe label.
- `current_recording_note_ref` — opaque note ref.

Rules (frozen):

1. `reverse_step_controls[]` MUST contain **all four**
   `control_kind_class` entries. Controls never disappear; they
   disable with a typed `availability_reason_class`.
2. `control_state_class = enabled` requires
   `availability_reason_class = supported`. Any other reason
   forces `disabled_with_reason`.
3. `control_state_class = enabled` further requires:
   - the banner's `support_level_class` is
     `record_replay_supported`,
   - the banner's `recording_state_class` is `live_recording`,
     `recorded`, or `imported_recording`,
   - the capture session's
     `replay_capability.support_level` is one of
     `replay_read_only` or `replay_interactive`,
   - the capture session's `replay_capability.allowed_verbs`
     includes `reverse_step` (for the three step-back kinds)
     or `reverse_continue` (for `reverse_continue`).
4. `recording_not_started`, `recording_expired`,
   `artifact_mismatch`, and `backend_not_supported` are the
   four reasons most often matched against the chronology-
   mismatch banner; the control row MUST cite the reason that
   matches the banner's `chronology_mismatch_state_class` when
   one is present, instead of falling back to a generic
   `policy_disabled`.

## 4. Composition rules

The banner record and control record compose into one rendered
surface. Parity rules are explicit so debugger UI, support-bundle
preview, and AI assist render the same answers:

1. Banner and control set MUST cite the same
   `debug_session_ref` and the same `capture_session_ref` (or
   both null).
2. The control set's
   `timeline_scrubber.scrubber_state_class` MUST be consistent
   with the banner's `recording_state_class` and
   `chronology_mismatch_state_class`. The schema enforces the
   pairing through closed conditional branches; renderers do
   not "decide" whether to show a scrubber.
3. The control set's `reverse_step_controls[*].availability_reason_class`
   MUST be consistent with the banner's
   `chronology_mismatch_state_class` when the banner names one.
4. Support-bundle exports MAY include the banner record by
   reference; the control record's
   `snapshot_frame_rows[*].frame_summary_note_ref` resolves
   only under the banner's `privacy_export_note` posture.
5. AI assist MAY surface a recording-mode banner only after
   reading both records. Synthesised "rewind to here" prompts
   that are not backed by an `enabled` control row MUST be
   refused at the boundary; the assist surfaces the typed
   `availability_reason_class` instead.

## 5. Forbidden collapses

The contract refuses these shortcuts. Each is named so reviewers
can cite the rule in code review and AI rules can refuse to
generate the collapse.

- **Reverse-step buttons disappear when disabled.** Disabled
  controls remain in the row with a typed reason; they are
  never silently removed.
- **Reverse-step buttons enabled without a recording.** A
  control row whose `control_state_class = enabled` requires a
  capture session, allowed verb, and recorded / live-recording
  state. The schema rejects every other combination.
- **Generic "Time travel not available" string.** Mismatch
  states resolve to one of the five closed classes
  (`recording_not_started`, `runtime_unsupported`,
  `history_partial`, `artifact_mismatch`, `recording_expired`).
- **Recording-state banner without overhead / storage.** The
  banner displays overhead and storage before `start_recording`
  is offered; an `unknown` pair disables the start button under
  `consent_pending` until disclosure resolves.
- **Reverse-step support implied by a CPU profile or trace.**
  Capture classes other than `replay_capture` cannot surface
  reverse-step controls as enabled; the banner's
  `support_level_class` reflects this directly.
- **Live-debug session implies chronology.** A live-only
  session resolves to `recording_state_class = not_recording`
  and disables every reverse-step control with
  `recording_not_started` instead of pretending a chronology
  exists.
- **Imported recording rendered as live capture.** Import-only
  lanes resolve to `recording_state_class = imported_recording`
  and the banner cites the imported origin; live-recording
  cannot be claimed in an import-only lane.
- **Artifact-mismatch hidden behind the runtime-unsupported
  row.** When a rebuild invalidated the recording, the banner
  cites `artifact_mismatch`, the artifact-manifest mismatch
  token, and the `rebuild_then_recapture` recovery action; it
  does not collapse into `runtime_unsupported`.
- **Partial history rendered as full chronology.** A capture
  session whose `chronology_support_class = partial_replay`
  surfaces `history_partial` on the banner whenever a
  scrubber gap is reachable, with a
  `partial_history_gap_marker` on the scrubber row.
- **Snapshot-frame rows missing a posture.** Every snapshot
  frame row carries a typed `posture_class`; rows never
  render with implicit "this is a frame" semantics.
- **Recording note carried inline.** Notes cross this boundary
  as opaque refs only; raw prose, raw URLs, raw paths, and raw
  command lines never appear inside the banner or control
  records.

## 6. Change rules

- Adding a new class to any closed vocabulary
  (`runtime_class`, `support_level_class`,
  `recording_state_class`, `chronology_mismatch_state_class`,
  `chronology_mismatch_recovery_action_class`,
  `banner_action_class`, `action_disable_reason_class`,
  `scrubber_state_class`, `event_ordering_class`,
  `marker_class`, `current_frame_time_basis_class`,
  `posture_class`, `control_kind_class`,
  `control_state_class`, `availability_reason_class`,
  `export_posture_class`) is additive-minor and bumps the
  matching `*_schema_version`. The change must update this
  document, both schemas, and at least one fixture in the same
  change.
- Removing a class or repurposing an existing token is
  breaking; it requires a new decision row and companion
  updates to the capture-session contract, the debug-truth
  contract, and the artifact-resolution seed.
- Debugger UI, support-bundle preview, and AI assist surfaces
  MUST adopt new tokens by reference, not by duplication. A
  surface that mints a parallel mismatch label or a parallel
  reverse-step control kind is non-conforming.

## 7. Fixture corpus

The five required fixtures cover the acceptance axes:

| Case | Records demonstrated |
|---|---|
| `supported_recorded_session` | `recording_state_class = recorded`, all four reverse-step controls `enabled`, scrubber `replay_active`, no chronology-mismatch state. |
| `unrecorded_live_debug_session` | `recording_state_class = not_recording`, all four controls `disabled_with_reason = recording_not_started`, scrubber `disabled_no_recording`, banner mismatch `recording_not_started` with recovery `start_new_recording`. |
| `partial_history_recording` | `recording_state_class = recorded`, scrubber `disabled_partial_history` carrying a `partial_history_gap_marker`, three controls `enabled`, one `disabled_with_reason` for the gap segment, banner mismatch `history_partial` with recovery `open_partial_history_only`. |
| `expired_capture` | `recording_state_class = expired`, every control `disabled_with_reason = recording_expired`, scrubber `disabled_recording_expired`, banner mismatch `recording_expired` with recovery `restart_with_recording`. |
| `artifact_mismatch_after_rebuild` | `recording_state_class = artifact_mismatch`, every control `disabled_with_reason = artifact_mismatch`, scrubber `disabled_artifact_mismatch`, banner mismatch `artifact_mismatch` with recovery `rebuild_then_recapture`, `debug_artifact_manifest_ref` cites `mismatch_module_build_id`. |

Each fixture is a single YAML document containing both the
`recording_mode_banner_record` and the matching
`reverse_step_control_record` so reviewers read the banner and
the control set as one surface.
