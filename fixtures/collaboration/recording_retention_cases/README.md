# Collaboration recording, transcript / replay retention, and delete-path worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/collaboration/recording_retention_delete_contract.md`](../../../docs/collaboration/recording_retention_delete_contract.md)
and the schemas at
[`/schemas/collaboration/recorded_artifact_row.schema.json`](../../../schemas/collaboration/recorded_artifact_row.schema.json)
and
[`/schemas/collaboration/delete_path_status.schema.json`](../../../schemas/collaboration/delete_path_status.schema.json).

Every file is a YAML document carrying a `__fixture__` prelude
summarising the scenario, the contract sections it exercises, and
the record kinds it produces, plus a `records` array containing
individual `collaboration_recorded_artifact_row_record`,
`collaboration_recorded_artifact_audit_event_record`,
`collaboration_delete_path_status_record`, and
`collaboration_delete_path_audit_event_record` instances that
conform to the schemas. The `record_kind` discriminator on each
record names which schema branch validates it.

No fixture embeds raw buffer text, raw terminal bytes, raw debug
payloads, raw URLs, raw absolute paths, raw user identifiers, raw
billing-account ids, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, raw pack bytes, raw conversation
transcripts, raw legal-hold justification text, or raw policy-bundle
bytes. Every such field is an opaque ref, a reviewable label, or a
coarse bucket.

## Cases

- [`ephemeral_pairing_session.yaml`](./ephemeral_pairing_session.yaml)
  — a pair-programming session under `live_only_no_retention`; every
  visible row resolves to `default_ephemeral_no_capture` /
  `never_admitted_at_this_mode`, `no_retention_owner_live_only`, and
  `no_location_live_only`; the delete-path-status row resolves to
  `deletion_not_applicable_live_only_row` with
  `inspectable_before_session_end_only` (the rows describe the
  active turn only). Acceptance bullet 3.
- [`policy_required_recording.yaml`](./policy_required_recording.yaml)
  — a regulated review session under
  `support_or_regulated_retention_with_hold_eligibility`; the
  recording row resolves to `capturing_now`, retention owner
  `admin_signed_managed_surface`, location
  `hosted_managed_surface_only`, and
  `default_ephemeral_versus_explicit_enable_class =
  explicit_enable_policy_forced_admin_signed`. The delete-path
  status is `deletion_not_yet_requested_inspectable_only` with
  `follow_up_user_can_request_delete` and
  `inspectable_before_and_after_session_end`. Acceptance bullet 1.
- [`user_enabled_transcript.yaml`](./user_enabled_transcript.yaml)
  — a teaching session under `replayable_review_or_teaching` where a
  participant explicitly opted into a text-only transcript; the
  transcript row resolves to `captured_export_available`, retention
  owner `host_org`, location
  `hosted_managed_surface_plus_local_export_copy`, and
  `default_ephemeral_versus_explicit_enable_class =
  explicit_enable_user_required`. The delete-path status is
  `deletion_completed_but_exported_local_copy_surviving` with
  `follow_up_user_must_delete_local_copy_separately` and
  `local_copy_exported_and_disclosed`. Acceptance bullets 1 and 4.
- [`legal_hold_blocker.yaml`](./legal_hold_blocker.yaml)
  — a support / regulated session whose recording is held under a
  legal-hold under external jurisdiction; the recording row
  resolves to `stopped_finalized`, retention owner
  `admin_signed_managed_surface`, location
  `hosted_managed_surface_only`, and `legal_hold_note_class =
  hold_active_user_delete_recorded_not_completed`. The delete-path
  status is `deletion_blocked_legal_hold` with
  `follow_up_legal_hold_review_required`,
  `hold_kind_class = legal_hold_external_jurisdiction`, and
  `inspectable_before_and_after_session_end`. Acceptance bullet 4.
- [`imported_support_evidence_packet.yaml`](./imported_support_evidence_packet.yaml)
  — a support session that admits an imported support-evidence
  packet whose authoritative copy lives at the import origin; the
  row resolves to `imported_or_support_evidence_packet_row`,
  retention owner `third_party_imported_origin`, location
  `third_party_import_origin`, and
  `default_ephemeral_versus_explicit_enable_class =
  imported_origin_not_subject_to_default`. The delete-path status
  is `deletion_not_yet_requested_inspectable_only` with
  `follow_up_third_party_origin_outside_aureline_scope`.
  Acceptance bullet 5.
