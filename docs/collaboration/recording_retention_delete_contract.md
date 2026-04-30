# Collaboration recording, transcript / replay retention, and delete-path truth contract

This document is the **product-wide contract** for how a collaboration
session presents — on the join dialog, the session inspector, the
post-session readout, the support-bundle preview, and the offboarding
flow — every visible row that is, or is not, being recorded; who owns
its retention; whether it lives locally or on a hosted managed surface;
whether it can be exported, and along which path; what happens to it on
delete; and whether a hold or an irreversible-share path applies. It
freezes one visible-row shape (`collaboration_recorded_artifact_row_record`),
one delete-path-status shape (`collaboration_delete_path_status_record`),
the closed vocabulary that names every cell on each, and the
default-ephemeral-versus-explicit-enable rule that admits or refuses
recording, transcript, and replay-artifact retention.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source wins
and this document MUST be updated in the same change. Where this
document disagrees with a downstream collaboration / recording /
transcript / replay / comment / support / review surface's mint of its
own visible row or delete chip, this document wins and the surface is
non-conforming.

The companion artifacts are:

- [`/schemas/collaboration/recorded_artifact_row.schema.json`](../../schemas/collaboration/recorded_artifact_row.schema.json)
  — boundary schema for the
  `collaboration_recorded_artifact_row_record` and
  `collaboration_recorded_artifact_audit_event_record` shapes.
- [`/schemas/collaboration/delete_path_status.schema.json`](../../schemas/collaboration/delete_path_status.schema.json)
  — boundary schema for the
  `collaboration_delete_path_status_record` and
  `collaboration_delete_path_audit_event_record` shapes.
- [`/fixtures/collaboration/recording_retention_cases/`](../../fixtures/collaboration/recording_retention_cases/)
  — worked-example corpus covering an ephemeral pairing session, a
  policy-required recording, a user-enabled transcript, a legal-hold
  blocker, and an imported support-evidence packet.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/collaboration/session_authority_contract.md`](./session_authority_contract.md)
  — session lifecycle, shared-object authority, and downgrade
  vocabularies.
- [`/docs/collaboration/consent_retention_contract.md`](./consent_retention_contract.md)
  — session policy manifest, four-mode retention matrix, mid-session
  re-consent rules, and the closed
  `shared_object_class_policy_class`,
  `retention_posture_class`,
  `export_posture_class`,
  `delete_posture_class`, and `visible_consent_cue_class` vocabularies.
  Every recorded-artifact row binds to one
  `collaboration_session_retention_row_record` and inherits the
  policy stance from that row; this contract narrows what the
  participant **sees**, never widens what the policy manifest
  **admits**.
- [`/docs/collaboration/shared_control_contract.md`](./shared_control_contract.md)
  — temporary control-grant authority and lifecycle vocabulary
  for terminal / debugger / runbook lanes. Presence and temporary
  control-grant rows on the recorded-artifact surface bind to these
  grants; this contract never widens grant scope.
- [`/docs/governance/privacy_history_and_lifecycle_contract.md`](../governance/privacy_history_and_lifecycle_contract.md)
  — stable delete-state vocabulary
  (`delete_requested`, `policy_retention`, `legal_hold`,
  `delete_completed`, `exported_copy_remains_local`) reused
  verbatim across privacy, collaboration, support, and offboarding
  flows. The collaboration delete-path-status classes named here
  narrow those states for in-session and post-session inspection;
  the broader privacy / offboarding surface joins through
  `delete_request_state_ref` when a collaboration delete posture is
  part of a wider delete request.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — record state vocabulary and chronology rules. Every retention
  row cites a `record_class_id_ref` into the record-class registry;
  this contract reads that posture without redeclaring it.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — every retention row resolves retention owner, hold eligibility,
  and offboarding posture against this registry; the recorded-artifact
  surface re-renders that posture in user-readable terms but does not
  redefine it.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — broker-owned redaction pass. Recorded-artifact rows carry opaque
  refs and reviewable labels only.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class` re-exported
  without modification.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state and deployment profile. A session inherits
  the joining workspace's trust posture; widening trust through a
  recorded-artifact row is forbidden.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship a recording engine, a transcription
pipeline, a replay player, or any provider-side multiplayer service.
It freezes the row shape those implementations will read and write.
The eventual collaboration crate's Rust types are the schema of
record; the JSON Schema exports at
`schemas/collaboration/recorded_artifact_row.schema.json` and
`schemas/collaboration/delete_path_status.schema.json` are the
cross-tool boundary every non-owning surface reads.

## Why freeze this now

Without one frozen row shape, collaboration is free to mint visible
chips that quietly imply durable retention or ambiguous deletion:

1. *A pair-programming session shows a "session" chip with no
   recording state, no retention owner, no location, and no delete
   path.* The participant assumes nothing is retained; the surface
   never said either way.
2. *A teaching replay session shows "Recording on" but not who owns
   retention, whether the artifact lives on a managed surface, on the
   participant's local workspace, or both, or how to delete it.*
   The participant has no way to inspect delete posture before or
   after the session ends.
3. *A support session admits a recording under admin-signed
   policy-forced posture; the chip shows "Recording" but never
   reveals that user-self-service delete will be refused under a
   legal hold for an indefinite review window.*
4. *A managed delete completes on the server; the chip flips to
   "Deleted"; nobody surfaces that the participant exported a local
   copy two minutes earlier.* The "Deleted" claim is technically
   true on the server and dishonest as a whole.
5. *An imported support-evidence packet appears on the session
   inspector with no marker that its retention origin is outside
   Aureline's delete path.* The user requests delete from inside the
   product; the request silently goes nowhere.

Freezing the visible-row shape and the delete-path-status shape now,
ahead of any recording, transcription, replay, or shared-control work
landing, ensures every later lane reads **the same** recording state,
**the same** retention owner, **the same** location class, **the
same** export availability, **the same** delete-path status, and
**the same** follow-up wording instead of inventing per-feature
equivalents.

## Default-ephemeral versus explicit-enable rule (frozen)

Collaboration sessions are **ephemeral by default**. Every recording,
transcript, and replay-artifact row a session might admit is, until
explicitly enabled, in one of two recording states:

- `default_ephemeral_no_capture` — the row is admitted by the
  retention mode but no capture is enabled and none will start
  without an explicit-enable cue.
- `never_admitted_at_this_mode` — the row is not admitted by the
  retention mode at all (e.g. recording rows under
  `live_only_no_retention`).

To leave default-ephemeral, a row MUST resolve
`default_ephemeral_versus_explicit_enable_class` to one of:

- `explicit_enable_user_required` — a participant explicitly opted
  in via the consent cue. The bound retention row's
  `opt_in_required_class` is `opt_in_explicit_user_required`.
- `explicit_enable_policy_forced_admin_signed` — an admin-signed
  policy forces persistence. The bound retention row's
  `opt_in_required_class` is `opt_in_policy_forced_admin_signed`.

Imported support-evidence packets are not subject to the
default-ephemeral rule and resolve through
`imported_origin_not_subject_to_default`; their retention was decided
at the import origin.

A `recording_or_transcript_or_replay_artifact_row` whose
`recording_state_class` is anything but
`default_ephemeral_no_capture` or `never_admitted_at_this_mode` and
whose `default_ephemeral_versus_explicit_enable_class` is anything
but `explicit_enable_user_required` or
`explicit_enable_policy_forced_admin_signed` denies with
`recording_must_be_explicit_enable_or_policy_forced`.

## Visible-row vocabulary (frozen)

A `collaboration_recorded_artifact_row_record` resolves to exactly one
`artifact_row_kind`:

- `presence_or_temporary_control_grant_row` — session-presence rows
  and the typed temporary control grants for terminal / debugger /
  runbook lanes (the grant itself, not the lane payload).
- `comment_or_review_thread_row` — anchored comments and review
  threads.
- `recording_or_transcript_or_replay_artifact_row` — full-session
  recording, text-only transcript, structured-captions transcript,
  and the replay-artifact metadata sealed at archive seal.
- `deletion_event_or_audit_row` — the deletion-event audit row
  itself plus collaboration audit events admitted as visible rows.
- `imported_or_support_evidence_packet_row` — imported support
  evidence packets and externally-sourced compliance imports the
  session admits as visible rows but does not itself originate.

Each row resolves a closed-vocabulary value for every required field:

| Field | Required vocabulary |
|---|---|
| `recording_state_class` | `default_ephemeral_no_capture`, `explicit_enable_pending_capture`, `capturing_now`, `captured_no_export`, `captured_export_available`, `paused_temporary`, `stopped_finalized`, `never_admitted_at_this_mode` |
| `retention_owner_class` | `host_org`, `owner_actor`, `participant_actor`, `support_operator`, `admin_signed_managed_surface`, `third_party_imported_origin`, `no_retention_owner_live_only` |
| `location_class` | `local_workspace_only`, `local_export_copy_only`, `hosted_managed_surface_only`, `hosted_managed_surface_plus_local_workspace`, `hosted_managed_surface_plus_local_export_copy`, `sealed_session_archive_only`, `third_party_import_origin`, `no_location_live_only` |
| `export_availability_class` | `no_export_available`, `metadata_only_export`, `local_workspace_export_only`, `session_archive_export`, `support_bundle_export_admin_signed`, `user_self_service_export`, `third_party_re_export_outside_aureline_scope` |
| `default_ephemeral_versus_explicit_enable_class` | `default_ephemeral_not_admitted`, `default_ephemeral_admitted_metadata_only`, `explicit_enable_user_required`, `explicit_enable_policy_forced_admin_signed`, `imported_origin_not_subject_to_default` |
| `legal_hold_note_class` | `no_hold`, `hold_active_user_delete_blocked`, `hold_active_user_delete_recorded_not_completed`, `hold_recently_cleared` |
| `irreversible_share_warning_class` | `no_irreversible_share`, `public_link_share_irreversible`, `external_recipient_share_irreversible`, `third_party_export_irreversible`, `captured_export_already_distributed` |

Every row binds to one
`collaboration_session_policy_manifest_record`, one
`collaboration_session_retention_row_record`, and one
`collaboration_delete_path_status_record` so a participant can
inspect every dimension of the row from a single chip without
re-reading the policy manifest. A row that omits any of those refs
denies with the matching `*_missing` reason from the closed
denial-reason vocabulary.

## Required fields per row kind (frozen)

| Row kind | Default `recording_state_class` | Required `retention_owner_class` floor | Required `location_class` floor | Required `irreversible_share_warning_class` |
|---|---|---|---|---|
| `presence_or_temporary_control_grant_row` | `default_ephemeral_no_capture` (presence); `capturing_now` allowed for granted lanes | `participant_actor` (presence); `host_org` or `admin_signed_managed_surface` (granted lane) | `local_workspace_only` (presence); per granted-lane retention row | `no_irreversible_share` |
| `comment_or_review_thread_row` | `captured_no_export` or `captured_export_available` | `owner_actor` or `participant_actor` (local), `host_org` (managed) | `local_workspace_only`, `local_export_copy_only`, `hosted_managed_surface_only`, `hosted_managed_surface_plus_local_workspace`, or `hosted_managed_surface_plus_local_export_copy` | per row, `external_recipient_share_irreversible` allowed when the comment is shared cross-org |
| `recording_or_transcript_or_replay_artifact_row` | `default_ephemeral_no_capture` until explicit-enable; otherwise as state | `host_org` or `admin_signed_managed_surface` | `hosted_managed_surface_only`, `hosted_managed_surface_plus_local_workspace`, `hosted_managed_surface_plus_local_export_copy`, or `sealed_session_archive_only` | per row, `captured_export_already_distributed` required when an export was already distributed |
| `deletion_event_or_audit_row` | `captured_no_export` or `captured_export_available` | `host_org` (managed-surface row) or `admin_signed_managed_surface` (regulated row) | `hosted_managed_surface_only` or `sealed_session_archive_only` | `no_irreversible_share` |
| `imported_or_support_evidence_packet_row` | `captured_export_available` (typically) | `third_party_imported_origin` (REQUIRED) | `third_party_import_origin` (REQUIRED) | `third_party_export_irreversible` |

A `presence_or_temporary_control_grant_row` whose
`recording_state_class` is `default_ephemeral_no_capture` MUST
resolve `retention_owner_class` to `no_retention_owner_live_only`
and `location_class` to `no_location_live_only`. The same applies
to `never_admitted_at_this_mode` rows. A row that violates this
combination denies with
`live_only_row_must_resolve_to_no_retention_owner_and_no_location`.

## Delete-path status vocabulary (frozen)

A `collaboration_delete_path_status_record` resolves to exactly one
`delete_path_status_class`. The seven admitted classes:

- `deletion_not_yet_requested_inspectable_only` — the artifact is
  retained and a participant can inspect its delete posture before
  requesting; this is the steady state for a captured-export-available
  row that nobody has asked to delete yet.
- `deletion_requested_not_yet_completed` — a delete request was
  accepted on the managed surface but completion is pending.
- `deletion_blocked_policy_retained` — retention policy refuses the
  delete; the request is recorded but not completed.
- `deletion_blocked_legal_hold` — a hold refuses the delete; the
  request is recorded but not completed.
- `deletion_completed_receipt_logged` — the managed surface
  completed the delete and a destruction receipt row was logged.
- `deletion_completed_but_exported_local_copy_surviving` — the
  managed surface completed the delete but a local export copy
  survives; the row MUST disclose that copy through
  `exported_local_copy_disclosure_class = local_copy_exported_and_disclosed`
  and follow-up wording
  `follow_up_user_must_delete_local_copy_separately`.
- `deletion_not_applicable_live_only_row` — the bound retention row
  is live-only and there is nothing retained to delete.

Each row carries a stable user-facing label that mirrors the class
verbatim:

| `delete_path_status_class` | `delete_path_status_label` |
|---|---|
| `deletion_not_yet_requested_inspectable_only` | `Inspectable, no delete requested` |
| `deletion_requested_not_yet_completed` | `Delete requested, not yet completed` |
| `deletion_blocked_policy_retained` | `Delete blocked: policy retention` |
| `deletion_blocked_legal_hold` | `Delete blocked: legal hold` |
| `deletion_completed_receipt_logged` | `Delete completed, receipt logged` |
| `deletion_completed_but_exported_local_copy_surviving` | `Delete completed, exported local copy still survives` |
| `deletion_not_applicable_live_only_row` | `Live-only row, nothing retained to delete` |

Surfaces that render a delete chip MUST use the verbatim label.
Synonyms ('Removed', 'Gone', 'Done', 'Cleared') are non-conforming.

### Follow-up wording rules

Every row resolves `follow_up_wording_class` to one of:

- `no_follow_up_needed` — admissible only on
  `deletion_completed_receipt_logged`,
  `deletion_requested_not_yet_completed` (when the managed surface
  is simply working through a queue), and
  `deletion_not_applicable_live_only_row`.
- `follow_up_user_can_request_delete` — admissible on
  `deletion_not_yet_requested_inspectable_only` (and on imported
  packets bound through the third-party-origin clause).
- `follow_up_admin_action_required` — admissible on
  `deletion_blocked_policy_retained` (REQUIRED) and on
  `deletion_requested_not_yet_completed` (when the queue is gated
  behind admin action).
- `follow_up_legal_hold_review_required` — REQUIRED on
  `deletion_blocked_legal_hold`.
- `follow_up_user_must_delete_local_copy_separately` — REQUIRED on
  `deletion_completed_but_exported_local_copy_surviving`.
- `follow_up_third_party_origin_outside_aureline_scope` — REQUIRED
  on delete-path rows bound to imported support-evidence packets
  whose origin is outside Aureline's delete path.

A row whose `delete_path_status_class` is
`deletion_blocked_policy_retained` or `deletion_blocked_legal_hold`
without the matching follow-up wording denies with
`follow_up_wording_must_match_block_state`.

### Hold-kind and hold-note rules

Block states resolve `hold_kind_class` to one of:

- `policy_hold_org_retention_floor` — paired with
  `deletion_blocked_policy_retained`.
- `regulated_review_hold` — paired with either block class on
  regulated review sessions.
- `legal_hold_external_jurisdiction` — paired with
  `deletion_blocked_legal_hold` under an external jurisdiction's
  compliance regime.
- `support_investigation_hold` — paired with
  `deletion_blocked_legal_hold` for an active support case.

`legal_hold_note_class` mirrors the row's recorded-artifact-row
`legal_hold_note_class` so the chip and the inspector show the same
hold posture. A `deletion_blocked_legal_hold` row that resolves
`legal_hold_note_class` to `no_hold` or `hold_recently_cleared`
denies with `legal_hold_note_class_must_match_block_state`.

### Pre- and post-session inspection rule

Every row resolves `inspection_window_class` to one of:

- `inspectable_before_session_end_only` — admissible only on
  live-only rows whose delete-path posture is purely about the
  active turn.
- `inspectable_after_session_end_only` — admissible only on rows
  minted at archive seal.
- `inspectable_before_and_after_session_end` — REQUIRED for any
  row that backs a captured artifact whose delete posture must
  remain reviewable past session end (recordings, transcripts,
  replay artifacts, comments retained on the managed surface, and
  imported support-evidence packets).

The booleans `before_session_end_inspectable` and
`after_session_end_inspectable` MUST agree with
`inspection_window_class` as documented in the schema. A row that
admits `inspectable_after_session_end_only` for a live-only retention
row, or that admits `inspectable_before_session_end_only` for a
recording / transcript / replay-artifact row, denies with
`before_and_after_session_end_inspection_required`.

### Exported-local-copy disclosure rule

`exported_local_copy_disclosure_class` resolves to:

- `no_local_copy_exported` — no export copy was made.
- `local_copy_exported_and_disclosed` — an export was made and the
  participant has acknowledged the disclosure cue. REQUIRED whenever
  `delete_path_status_class` is
  `deletion_completed_but_exported_local_copy_surviving`.
- `local_copy_exported_disclosure_pending_consent_acknowledgement`
  — admissible while the participant has not yet acknowledged the
  cue; the row remains in
  `deletion_requested_not_yet_completed` or
  `deletion_completed_but_exported_local_copy_surviving` until the
  cue is acknowledged.

A row that claims `deletion_completed_receipt_logged` while a local
copy survives denies with `exported_local_copy_must_be_disclosed`
and MUST be re-minted as
`deletion_completed_but_exported_local_copy_surviving`.

## Acceptance-criteria cross-walk

1. **Collaboration surfaces cannot imply durable recording,
   transcript, or replay retention without explicit state and owner
   disclosure.** Mechanised by the
   `default_ephemeral_versus_explicit_enable_class` `allOf` gate on
   `collaboration_recorded_artifact_row_record` for
   `recording_or_transcript_or_replay_artifact_row` rows whose
   `recording_state_class` is above
   `default_ephemeral_no_capture` and by the
   `recording_must_be_explicit_enable_or_policy_forced` denial
   reason. Worked in
   [`policy_required_recording.yaml`](../../fixtures/collaboration/recording_retention_cases/policy_required_recording.yaml)
   and
   [`user_enabled_transcript.yaml`](../../fixtures/collaboration/recording_retention_cases/user_enabled_transcript.yaml).
2. **Delete/export posture for shared-session artifacts is
   inspectable before and after the session ends.** Mechanised by
   the `inspection_window_class` field on
   `collaboration_delete_path_status_record` plus the
   `before_session_end_inspectable` / `after_session_end_inspectable`
   booleans, the schema-enforced agreement between the booleans and
   the class, and the
   `before_and_after_session_end_inspection_required` denial reason.
   Worked across every fixture.
3. **Default-ephemeral collaboration is honest at the row.**
   Mechanised by the
   `default_ephemeral_no_capture` /
   `never_admitted_at_this_mode` recording-state pairing with
   `no_retention_owner_live_only` and `no_location_live_only`, the
   `live_only_row_must_resolve_to_no_retention_owner_and_no_location`
   denial reason, and the live-only delete-path-status invariant
   that forbids hold / receipt / completion fields. Worked in
   [`ephemeral_pairing_session.yaml`](../../fixtures/collaboration/recording_retention_cases/ephemeral_pairing_session.yaml).
4. **Held or retained artifacts cannot masquerade as deleted.**
   Mechanised by the
   `delete_path_status_label` constant agreement with
   `delete_path_status_class`, the
   `legal_hold_note_class_must_match_block_state` and
   `follow_up_wording_must_match_block_state` denial reasons, and
   the
   `deletion_completed_but_exported_local_copy_surviving` /
   `local_copy_exported_and_disclosed` pairing required by
   `exported_local_copy_must_be_disclosed`. Worked in
   [`legal_hold_blocker.yaml`](../../fixtures/collaboration/recording_retention_cases/legal_hold_blocker.yaml).
5. **Imported support-evidence packets are not silently re-owned.**
   Mechanised by the imported-row pairing requiring
   `imported_evidence_packet_ref`,
   `retention_owner_class = third_party_imported_origin`,
   `location_class = third_party_import_origin`,
   `default_ephemeral_versus_explicit_enable_class = imported_origin_not_subject_to_default`,
   and `export_availability_class` restricted to
   `no_export_available` or
   `third_party_re_export_outside_aureline_scope`, plus the
   `imported_packet_must_cite_third_party_import_origin` denial
   reason. Worked in
   [`imported_support_evidence_packet.yaml`](../../fixtures/collaboration/recording_retention_cases/imported_support_evidence_packet.yaml).

## Visual / export parity floor

Every consuming surface (session inspector, join dialog, post-session
readout, support-bundle preview, offboarding flow, admin-collaboration
console, claim manifest export) MUST render, for every visible row:

- `artifact_row_kind`
- `recording_state_class`
- `retention_owner_class`
- `location_class`
- `export_availability_class`
- `default_ephemeral_versus_explicit_enable_class`
- `legal_hold_note_class`
- `irreversible_share_warning_class`
- the bound `delete_path_status_label` and `follow_up_label`

Forbidden collapses:

- Rendering a `default_ephemeral_no_capture` row without naming the
  `default_ephemeral_versus_explicit_enable_class` (the chip would
  imply "off forever" rather than "off until explicitly enabled").
- Rendering a `captured_export_available` recording / transcript /
  replay-artifact row without naming `retention_owner_class`,
  `location_class`, and the bound delete-path label.
- Collapsing `deletion_completed_but_exported_local_copy_surviving`
  to `Delete completed, receipt logged` or any synonym ('Done',
  'Removed', 'Gone').
- Collapsing `deletion_blocked_policy_retained` or
  `deletion_blocked_legal_hold` to a generic `Pending` chip without
  the follow-up wording and hold-kind disclosure.
- Rendering an imported support-evidence packet row without the
  `third_party_imported_origin` retention-owner cue and the
  `third_party_re_export_outside_aureline_scope` export-availability
  cue.
- Emitting role IDs, raw email addresses, raw phone numbers, raw
  chat-room URLs, raw meeting refs, raw legal-hold justification
  text, raw policy-bundle bytes, or any other raw payload across the
  schema boundary.

## Redaction posture

Raw editor buffer text, raw terminal bytes, raw debug payloads, raw
provider responses, raw URLs, raw endpoint hostnames, raw absolute
paths, raw user identifiers, raw billing-account ids, raw API keys,
raw OAuth tokens, raw mTLS material, raw model weights, raw pack
bytes, raw conversation transcripts, raw legal-hold justification
text, and raw policy-bundle bytes never cross this schema boundary
on any surface. Records carry opaque refs, structured fields,
reviewable sentence labels, and coarse buckets only. The
broker-owned redaction pass (ADR-0007) governs the byte-level
redaction; this contract names the boundary fields the redaction
pass populates.

## Additive-minor change discipline

Adding a new `artifact_row_kind`, `recording_state_class`,
`retention_owner_class`, `location_class`,
`export_availability_class`,
`default_ephemeral_versus_explicit_enable_class`,
`legal_hold_note_class`, `irreversible_share_warning_class`,
`delete_path_status_class`, `follow_up_wording_class`,
`hold_kind_class`, `inspection_window_class`,
`exported_local_copy_disclosure_class`,
`recorded_artifact_audit_event_id`,
`recorded_artifact_denial_reason`,
`delete_path_audit_event_id`, or `delete_path_denial_reason` value
is **additive-minor** and bumps the matching
`recorded_artifact_row_schema_version` /
`delete_path_status_schema_version`. Repurposing an existing value,
removing a value, or rewriting a denial gate's logic is
**breaking** and requires a new decision row in
`artifacts/governance/decision_register.yaml`.

## Out of scope at this revision

- Implementing collaboration transport, relay, recording,
  transcription, replay, or any multiplayer service.
- Implementing the legal-hold enforcement pipeline or the
  compliance-audit backend.
- Implementing the admin-signed approval flow that admits
  `admin_forced_consent_cue_admin_signed`.
- Rendering the recorded-artifact UI (session inspector chip, join
  dialog row, post-session readout) beyond naming the admitted
  vocabulary.
- Defining provider-side retention, export, or delete SLAs.

These are explicitly deferred. This revision freezes the row shape
those implementations will read and write.
