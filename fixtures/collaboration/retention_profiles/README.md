# Collaboration session consent, retention, export, and delete worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/collaboration/consent_retention_contract.md`](../../../docs/collaboration/consent_retention_contract.md)
and the schema at
[`/schemas/collaboration/session_policy_manifest.schema.json`](../../../schemas/collaboration/session_policy_manifest.schema.json).

Every file is a YAML document carrying a `__fixture__` prelude
summarising the scenario, the contract sections it exercises, and
the record kinds it produces, plus a `records` array containing
individual `collaboration_session_policy_manifest_record`,
`collaboration_session_retention_row_record`,
`collaboration_session_consent_event_record`, and
`collaboration_session_policy_audit_event_record` instances that
conform to the schema. The `record_kind` discriminator on each
record names which schema branch validates it.

No fixture embeds raw buffer text, raw terminal bytes, raw debug
payloads, raw URLs, raw absolute paths, raw user identifiers, raw
billing-account ids, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, raw pack bytes, or raw provider
payloads. Every such field is an opaque ref, a reviewable label,
or a coarse bucket.

## Cases

- [`live_only_pairing.yaml`](./live_only_pairing.yaml) — a
  pair-programming session admitted under
  `live_only_no_retention`; nothing is retained past the active
  turn; shared terminal / debugger retention is not admitted; join
  dialog cue admits both sides pre-admit. Acceptance bullet 1.
- [`metadata_audited_collaboration.yaml`](./metadata_audited_collaboration.yaml)
  — a review session admitted under
  `metadata_audited_no_payload_retention`; comments, replay
  artifact metadata, and deletion-event rows are retained as
  metadata only on the managed surface; no raw bytes cross the
  boundary. Acceptance bullet 4.
- [`replayable_review_session.yaml`](./replayable_review_session.yaml)
  — a teaching / review session admitted under
  `replayable_review_or_teaching`; recording and transcript are
  retained under explicit user opt-in; mid-session recording start
  surfaces a blocking modal cue; participant self-service export
  is disclosed so a later managed delete is honest. Acceptance
  bullets 1 and 2.
- [`support_regulated_session.yaml`](./support_regulated_session.yaml)
  — a support / regulated-review session admitted under
  `support_or_regulated_retention_with_hold_eligibility`; every
  retention row including shared terminal / debugger rows resolves
  to `opt_in_policy_forced_admin_signed`; delete posture is
  `deletion_blocked_legal_hold`; owner delete request is recorded
  but not completed. Acceptance bullets 3 and 4.
- [`mid_session_re_consent.yaml`](./mid_session_re_consent.yaml) —
  a session amended mid-session from
  `metadata_audited_no_payload_retention` to
  `replayable_review_or_teaching` because the host enabled a
  transcript; the amended manifest supersedes the prior one and a
  fresh `collaboration_session_consent_event_record` is minted
  with `re_consent_trigger_reason_class = transcript_enabled` and
  a modal cue. Acceptance bullet 2.
