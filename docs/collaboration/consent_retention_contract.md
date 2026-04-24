# Collaboration session consent, retention, export, and delete contract

This document is the **product-wide contract** for how a collaboration
session declares, on admission and at every mid-session amendment,
what it retains, who may export it, who may delete it, which guests
(if any) may attend, how routes are shared, where the payload is
admitted to reside, which redaction profile applies, and what
on-surface cue admits each of those choices. It freezes one
session-policy manifest row, one retention / export / delete matrix
across four retention modes, one closed consent-cue set, one closed
mid-session re-consent trigger set, and the opt-in rules for
shared terminal and shared debugger retention that ordinary
collaboration never implies.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream collaboration / recording
/ transcript / replay / comment / support / review surface's mint
of its own retention or consent state, this document wins and the
surface is non-conforming.

The companion artifacts are:

- [`/schemas/collaboration/session_policy_manifest.schema.json`](../../schemas/collaboration/session_policy_manifest.schema.json)
  — boundary schema for the `collaboration_session_policy_manifest_record`,
  `collaboration_session_retention_row_record`,
  `collaboration_session_consent_event_record`, and
  `collaboration_session_policy_audit_event_record` shapes.
- [`/fixtures/collaboration/retention_profiles/`](../../fixtures/collaboration/retention_profiles/)
  — worked-example corpus covering live-only pairing,
  metadata-audited collaboration, replayable review / teaching, and
  support / regulated retention, plus a mid-session re-consent case.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/collaboration/session_authority_contract.md`](./session_authority_contract.md)
  — session-lifecycle, shared-object authority, and downgrade
  vocabularies. A policy manifest binds to one
  `collaboration_session_record`; the lifecycle states, the admitted
  transitions, and the shared-object authority / durability /
  export-posture classes are **inherited** from that contract and
  not redefined here.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — record state vocabulary (local-only, managed copy, held, delete
  requested, delete complete, export available) and chronology
  rules. A retention row's `delete_posture_class` narrows those
  states; it does not widen them.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — every `collaboration_session_retention_row_record` MUST cite one
  `record_class_id_ref` into this registry so the bound retention
  owner, export packet posture, delete semantics, hold eligibility,
  and offboarding posture are resolved **without redeclaration**.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  — separation rule for support-bundle, analytics, usage-export,
  and offboarding payloads. A session retention row citing a
  support-export or usage-export record class reads the consent
  class, endpoint class, and build-flavor default posture from the
  registry row rather than re-stating them.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — broker-owned redaction pass. Manifest and retention rows carry
  opaque refs and reviewable labels only; `redaction_profile_class`
  names the profile, not the bytes.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class` re-exported
  without modification.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state and deployment profile. A session
  inherits the joining workspace's trust posture; this contract
  never widens trust through a session.

If this document disagrees with those sources, those sources win
and this document plus the schema are updated in the same change.

This document does not ship a live collaboration transport, a
recording or transcription engine, a replay player, or any
provider-side multiplayer service. It freezes the row shape those
implementations will read and write. The eventual collaboration
crate's Rust types are the schema of record; the JSON Schema
export at
`schemas/collaboration/session_policy_manifest.schema.json` is the
cross-tool boundary every non-owning surface reads.

## Why freeze this now

Without one frozen contract, collaboration is free to mint implicit
retention the user was never asked about:

1. *A pairing session quietly writes a full-session recording.* The
   participant assumed "live only"; the managed surface retains
   bytes.
2. *A review session admits a guest with a public link and never
   shows the guest, or the host, that the link is public.* Guest
   attendance becomes policy by absence.
3. *A teaching replay session is shared outside the residency
   scope* because no row pinned residency and no consent cue named
   the scope.
4. *A shared terminal session records keystrokes by default.* The
   participant treated the terminal like a native local shell; the
   product treated it like a cloud recorder.
5. *A session ends and a delete request completes on the server,
   but every participant has already exported a local copy* and
   nobody says so. The managed-copy delete is honest; the overall
   claim is not.

The freeze matters now, ahead of any recording, transcription,
replay, or shared-control work landing, so every later lane can
read **the same** retention mode, **the same** consent cue, and
**the same** delete posture instead of inventing per-feature
equivalents.

## Session policy manifest minimum (frozen)

A `collaboration_session_policy_manifest_record` carries, at minimum:

- `policy_manifest_id` — opaque row id.
- `session_ref` — opaque ref to the bound
  `collaboration_session_record`.
- `host_org_ref` — opaque ref to the owning organisation.
- `owner_actor_ref` — opaque ref to the session owner.
- `participant_actor_refs` — opaque refs to admitted participants.
- `guest_actor_refs` — opaque refs to admitted guests (empty when
  `guest_policy_class` is `no_guests_admitted_org_members_only`).
- `role_matrix_profile_class` — one value from the seven-profile
  role vocabulary (solo owner; owner plus named editors; owner plus
  editors plus reviewers; presenter-driven observers only;
  interview or teaching presenter plus observers; support session
  host plus support agent; regulated review with approver gate).
- `retention_mode_class` — one value from the four-mode retention
  vocabulary (see next section).
- `guest_policy_class` — one value from the five-value guest-policy
  vocabulary.
- `route_share_policy_class` — one value from the four-value
  route-visibility vocabulary.
- `residency_scope_class` — one value from the five-value
  residency-scope vocabulary.
- `redaction_profile_class` — one value from the five-value
  redaction-profile vocabulary (layered over ADR-0007 broker-owned
  redaction).
- `audit_level_class` — one value from the four-value audit-level
  vocabulary.
- `export_rights` — a typed per-role export-rights row covering
  owner, participant, guest, and admin posture classes.
- `delete_rights` — a typed per-role delete-rights row covering
  owner, participant, and admin posture classes plus the
  `legal_hold_overrides_user_delete_request` flag.
- `retention_row_refs` — one or more opaque refs to
  `collaboration_session_retention_row_record` rows, one per
  shared-object class admitted to survive past the active turn.
- `archive_refs` — opaque refs to `sealed_session_archive` shared
  objects bound to this manifest (empty on `live_only_no_retention`
  sessions).
- `active_visible_consent_cue_refs` — opaque refs to the consent
  events that admitted the manifest's current posture (REQUIRED
  whenever `retention_mode_class` is
  `replayable_review_or_teaching` or
  `support_or_regulated_retention_with_hold_eligibility`, and
  whenever any row admits recording, transcript, or shared
  terminal / debugger retention).
- `summary_label` — reviewable one-sentence description.

Any row that omits a required field denies with the matching
`*_unresolved` or `*_missing` reason from the closed
`policy_denial_reason` set. No row carries raw bytes — only opaque
refs and reviewable labels cross the boundary.

## Retention / export / delete matrix (frozen)

Every session resolves to exactly one `retention_mode_class`. The
four admitted modes:

### `live_only_no_retention`

| Row kind | Retention posture | Export posture | Delete posture |
|---|---|---|---|
| Recording | `not_retained_live_only` | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |
| Transcript | `not_retained_live_only` | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |
| Replay artifact | `not_retained_live_only` | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |
| Comment | Not admitted at this mode (no `anchored_comment` row minted). | — | — |
| Temporary control grant (terminal / debugger / runbook) | `not_retained_live_only` (the **grant** itself is ephemeral; whether the terminal or debugger is retained at all is governed by its own row). | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |
| Deletion-event record | Not applicable (nothing was retained to delete). | — | — |
| Editor-buffer projection metadata | `not_retained_live_only` | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |

`live_only_no_retention` is the default posture for pair-programming
or quick-look sessions that must not mint any durable row. The
session lifecycle still transitions through `solo` → `publish_pending`
→ `shared_active` → `ended`; the `archived` state is not reached
because no archive is sealed.

### `metadata_audited_no_payload_retention`

| Row kind | Retention posture | Export posture | Delete posture |
|---|---|---|---|
| Recording | `not_retained_live_only` | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |
| Transcript | `not_retained_live_only` | `not_exportable_ephemeral_only` | `deletion_not_applicable_live_only_row` |
| Replay artifact metadata | `retained_metadata_only_managed_surface` | `exportable_metadata_only` | per record-class registry row. |
| Comment | `retained_metadata_only_local_workspace` or `retained_metadata_only_managed_surface` per `residency_scope_class`. | `exportable_metadata_only` | `deletion_only_requested_not_yet_completed` → `deletion_completed_receipt_logged` per record-class. |
| Temporary control grant | `retained_metadata_only_managed_surface` | `exportable_metadata_only` | per record-class registry row. |
| Deletion-event record | `retained_metadata_only_managed_surface` | `exportable_metadata_only` | `deletion_not_applicable_live_only_row` (deletion events do not themselves delete). |

No raw buffer / terminal / debug payload is retained. The
audit-event rows on the `collaboration_session` and
`collaboration_shared_object` streams survive per ADR-0007 broker
redaction and per the session-authority contract.

### `replayable_review_or_teaching`

| Row kind | Retention posture | Export posture | Delete posture |
|---|---|---|---|
| Recording | `retained_full_payload_managed_surface_consent_required` | `exportable_in_session_archive` or `exportable_in_user_self_service_export`. | `deletion_only_requested_not_yet_completed` → `deletion_completed_receipt_logged`; export-bearing completion MUST resolve to `deletion_completed_but_exported_local_copy_surviving` when a local copy survives. |
| Transcript (text or captions) | `retained_full_payload_managed_surface_consent_required` | `exportable_in_session_archive` or `exportable_in_user_self_service_export`. | as recording. |
| Replay artifact | `retained_in_sealed_archive_only` | `exportable_in_session_archive` | per sealed-archive rules in the session-authority contract. |
| Comment | `retained_full_payload_local_workspace` (per-side). | `exportable_in_local_workspace_only` or `exportable_in_session_archive`. | owner and author each honour a local delete; the managed archive honours its own; `legal_hold_overrides_user_delete_request` may apply. |
| Temporary control grant | `retained_metadata_only_managed_surface` | `exportable_metadata_only` | per record-class registry row. |
| Deletion-event record | `retained_metadata_only_managed_surface` | `exportable_metadata_only` | not itself deletable via the user-delete path. |

A `replayable_review_or_teaching` manifest MUST carry at least one
`active_visible_consent_cue_ref`; the recording and transcript rows
MUST resolve to `opt_in_explicit_user_required` or
`opt_in_policy_forced_admin_signed` on every admitted participant
and guest. A missing cue denies with
`recording_must_carry_visible_consent_cue`.

### `support_or_regulated_retention_with_hold_eligibility`

| Row kind | Retention posture | Export posture | Delete posture |
|---|---|---|---|
| Recording | `retained_full_payload_support_bundle_admin_signed` | `exportable_in_support_bundle_admin_signed` | `deletion_blocked_policy_retained` or `deletion_blocked_legal_hold` per record-class registry row. |
| Transcript | `retained_full_payload_support_bundle_admin_signed` | `exportable_in_support_bundle_admin_signed` | as recording. |
| Replay artifact | `retained_in_sealed_archive_only` | `exportable_in_session_archive` | per sealed-archive rules. |
| Comment | `retained_full_payload_managed_surface_consent_required` | `exportable_in_session_archive` | `deletion_blocked_policy_retained` or `deletion_blocked_legal_hold` per record-class; user delete is a request, not a completion, until the hold clears. |
| Temporary control grant (terminal / debugger / runbook) | `retained_full_payload_support_bundle_admin_signed` | `exportable_in_support_bundle_admin_signed` | `deletion_blocked_policy_retained` or `deletion_blocked_legal_hold`. |
| Deletion-event record | `retained_full_payload_support_bundle_admin_signed` | `exportable_in_support_bundle_admin_signed` | not deletable until the governing hold clears. |

A support or regulated session MUST resolve `audit_level_class` to
`audit_regulated_admin_signed` and MUST carry at least one
admin-signed `active_visible_consent_cue_ref` (cue class
`admin_forced_consent_cue_admin_signed`).
`legal_hold_overrides_user_delete_request` MUST resolve to `true`;
a row that resolves it to `false` while the retention mode is
support or regulated denies with
`legal_hold_overrides_user_delete_request`.

### Opt-in rule for shared terminal and shared debugger retention

A retention row whose `shared_object_class_policy_class` is any of
`shared_terminal_control_metadata`,
`shared_debug_control_metadata`,
`temporary_control_grant_terminal`, or
`temporary_control_grant_debugger`, and whose
`retention_posture_class` is above
`retained_metadata_only_local_workspace`, MUST resolve
`opt_in_required_class` to `opt_in_explicit_user_required` or
`opt_in_policy_forced_admin_signed`. Ordinary collaboration (any
manifest that does not explicitly opt in) MUST NOT retain terminal
or debugger bytes or control grants beyond metadata; a row that
asserts such retention without opt-in denies with
`shared_terminal_or_debugger_retention_must_be_opt_in`.

### Deletion-event row discipline

Deletion is a five-state vocabulary:

- `deletion_only_requested_not_yet_completed` — a user or admin has
  requested deletion; the managed surface has not yet completed it.
- `deletion_blocked_policy_retained` — retention policy refuses the
  delete; the request is recorded but not completed.
- `deletion_blocked_legal_hold` — a hold refuses the delete; the
  request is recorded but not completed.
- `deletion_completed_receipt_logged` — the managed surface
  completed the delete; a destruction receipt row was logged
  (per the `destruction_receipt_record` record class).
- `deletion_completed_but_exported_local_copy_surviving` — the
  managed surface completed the delete but a local copy was
  previously exported through
  `exportable_in_local_workspace_only` or
  `exportable_in_user_self_service_export` and that copy survives.
  A row that silently claims `deletion_completed_receipt_logged`
  while a local copy survives denies with
  `exported_local_copy_must_be_disclosed`.

Deletion events themselves are retained under the same retention
mode's "deletion-event record" row; they are not recursively
deletable.

## Consent cues and mid-session re-consent rules (frozen)

Consent cues resolve to one value from the six-value
`visible_consent_cue_class` vocabulary:

- `join_dialog_inline_disclosure_pre_admit` — the join dialog
  surfaces the manifest's retention, guest, route, residency, and
  audit postures **before** the participant is admitted.
- `participant_badge_persistent_during_session` — a persistent
  session-inspector badge restates the same postures for the
  duration of the session.
- `mid_session_banner_with_typed_change_summary` — when the
  manifest is amended, a banner surfaces the typed diff (what
  changed, from what, to what).
- `mid_session_modal_block_until_acknowledged` — the banner is
  upgraded to a modal when the change raises retention, expands
  guest admission, or widens route visibility.
- `host_initiated_per_guest_consent_prompt` — the host explicitly
  prompts each guest at admission.
- `admin_forced_consent_cue_admin_signed` — an admin-signed cue is
  applied (support / regulated sessions).

A manifest whose `retention_mode_class` is above
`live_only_no_retention` MUST carry at least one cue from this set;
a manifest that carries a `retention_mode_class` of
`replayable_review_or_teaching` or higher MUST carry at least one
cue strictly stronger than
`participant_badge_persistent_during_session` at admission time.

### Mid-session re-consent triggers

Any mid-session change to `retention_mode_class`,
`guest_policy_class`, `route_share_policy_class`,
`residency_scope_class`, `audit_level_class`, or to a retention row
admitting a new `shared_object_class_policy_class` MUST mint a
fresh `collaboration_session_consent_event_record` citing exactly
one `re_consent_trigger_reason_class` from the closed set:

- `retention_mode_changed`
- `guest_policy_changed`
- `route_share_policy_changed`
- `residency_scope_changed`
- `audit_level_raised`
- `recording_started`
- `recording_stopped`
- `transcript_enabled`
- `transcript_disabled`
- `temporary_control_grant_requested_terminal`
- `temporary_control_grant_requested_debugger`
- `temporary_control_grant_requested_runbook`
- `participant_role_promoted_to_driver_or_approver`
- `admin_forced_policy_change`

A mid-session retention, guest, or route change without a matching
consent event denies with `mid_session_consent_cue_missing`. A
consent event without a cue denies with
`visible_consent_cue_missing`.

## Row classes the contract explicitly admits

The `shared_object_class_policy_class` vocabulary distinguishes the
policy-relevant sub-kinds that the session-authority contract's
`shared_object_class_class` intentionally collapses:

- `recording_full_session_replay` — full-session replayable
  recording (video, audio, or structured event stream).
- `transcript_text_only` — speech-to-text transcript.
- `transcript_structured_captions_only` — structured captions / cue
  list without free-form text.
- `replay_artifact_metadata_and_events` — the structured replay
  artifact minted on archive seal.
- `comment_anchored_or_thread` — anchored-comment or thread-style
  review content (narrower than
  `shared_object_class_class = anchored_comment` because the row is
  for the policy manifest and does not carry an anchor).
- `temporary_control_grant_terminal` / `_debugger` / `_runbook` —
  the typed grant, not the lane's own control metadata.
- `deletion_event_record` — the deletion-event row itself.
- `editor_buffer_projection_metadata` — the projection metadata,
  not the buffer bytes.
- `follow_or_presenter_state_metadata` — follow / presenter state
  as policy rows (always ephemeral and never exportable per the
  session-authority contract).
- `shared_terminal_control_metadata` / `shared_debug_control_metadata`
  — the control-metadata rows from the session-authority contract.
- `sealed_session_archive_inventory` — the inventory row bound to
  the sealed archive.

## Acceptance-criteria cross-walk

1. **Join fixtures show retention mode, guest status, export
   posture, and delete posture before acceptance.** Mechanised by
   the required fields (`retention_mode_class`,
   `guest_policy_class`, `export_rights`, `delete_rights`) on
   `collaboration_session_policy_manifest_record` and by the
   `join_dialog_inline_disclosure_pre_admit` cue. Worked in
   [`live_only_pairing.yaml`](../../fixtures/collaboration/retention_profiles/live_only_pairing.yaml)
   and
   [`replayable_review_session.yaml`](../../fixtures/collaboration/retention_profiles/replayable_review_session.yaml).
2. **Any mid-session retention or guest-policy change requires a
   fresh visible consent event in the contract examples.**
   Mechanised by the `collaboration_session_consent_event_record`
   requirement on any amended manifest, by the
   `re_consent_trigger_reason_class` vocabulary, and by the
   `mid_session_consent_cue_missing` denial reason. Worked in
   [`mid_session_re_consent.yaml`](../../fixtures/collaboration/retention_profiles/mid_session_re_consent.yaml).
3. **Shared terminal / debugger retention is opt-in or
   policy-forced, never implied by ordinary collaboration.**
   Mechanised by the `opt_in_required_class` `allOf` gate on
   `collaboration_session_retention_row_record` for the four
   terminal / debugger / temporary-control-grant classes and by the
   `shared_terminal_or_debugger_retention_must_be_opt_in` denial
   reason. Worked in
   [`support_regulated_session.yaml`](../../fixtures/collaboration/retention_profiles/support_regulated_session.yaml).
4. **Collaboration packets preserve chronology, retention owner,
   and delete / export posture for later support, admin, and
   compliance review.** Mechanised by the `minted_at` /
   `amended_at` / `supersedes_policy_manifest_ref` fields, by the
   `record_class_id_ref` binding each retention row into the
   record-class registry (which owns retention-owner and
   hold-posture claims), and by the
   `collaboration_session_policy_audit_event_record` stream
   covering manifest mint / amend, retention-row mint / supersede,
   consent event record, delete request / completion, and exported
   local-copy disclosure. Worked across every fixture.

## Redaction posture

Raw editor buffer text, raw terminal bytes, raw debug payloads, raw
provider responses, raw URLs, raw endpoint hostnames, raw absolute
paths, raw user identifiers, raw billing-account ids, raw API keys,
raw OAuth tokens, raw mTLS material, raw model weights, raw pack
bytes, and raw conversation transcripts never cross this schema
boundary on any surface. Records carry opaque refs, structured
fields, reviewable sentence labels, and coarse buckets only. The
broker-owned redaction pass (ADR-0007) governs the byte-level
redaction; this contract names the boundary fields the redaction
pass populates.

## Additive-minor change discipline

Adding a new `role_matrix_profile_class`, `retention_mode_class`,
`guest_policy_class`, `route_share_policy_class`,
`residency_scope_class`, `redaction_profile_class`,
`audit_level_class`, `shared_object_class_policy_class`,
`retention_posture_class`, `export_posture_class`,
`delete_posture_class`, `visible_consent_cue_class`,
`re_consent_trigger_reason_class`, `policy_audit_event_id`, or
`policy_denial_reason` value is **additive-minor** and bumps
`session_policy_manifest_schema_version`. Repurposing an existing
value, removing a value, or rewriting a denial gate's logic is
**breaking** and requires a new decision row in
`artifacts/governance/decision_register.yaml`.

## Out of scope at this revision

- Implementing collaboration transport, relay, recording,
  transcription, replay, or any multiplayer service.
- Implementing the legal-hold enforcement pipeline or the
  compliance-audit backend.
- Implementing the admin-signed approval flow that admits
  `admin_forced_consent_cue_admin_signed`.
- Rendering the consent-cue UI (join dialog, banner, modal,
  persistent badge) beyond naming the admitted vocabulary.
- Defining provider-side retention, export, or delete SLAs.

These are explicitly deferred. This revision freezes the row shape
those implementations will read and write.
