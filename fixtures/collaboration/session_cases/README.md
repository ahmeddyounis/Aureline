# Collaboration session lifecycle, shared-object authority, and downgrade worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/collaboration/session_authority_contract.md`](../../../docs/collaboration/session_authority_contract.md)
and the schemas at
[`/schemas/collaboration/session_state.schema.json`](../../../schemas/collaboration/session_state.schema.json)
and
[`/schemas/collaboration/shared_object.schema.json`](../../../schemas/collaboration/shared_object.schema.json).

Every file is a YAML document carrying a `__fixture__` prelude
summarising the scenario, the contract sections it exercises, and
the record kinds it produces, plus a `records` array containing
individual `collaboration_session_record`,
`collaboration_session_transition_record`,
`collaboration_session_downgrade_record`,
`collaboration_session_audit_event_record`, `shared_object_record`,
`shared_object_authority_transition_record`,
`shared_object_anchor_drift_record`, and
`shared_object_audit_event_record` instances that conform to the
schemas. The `record_kind` discriminator on each record names
which schema validates it.

No fixture embeds raw buffer text, raw terminal bytes, raw debug
payloads, raw URLs, raw absolute paths, raw user identifiers, raw
billing-account ids, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, raw pack bytes, or raw provider
payloads. Every such field is an opaque ref or a reviewable label.

## Cases

- [`solo_to_shared_to_archived_happy_path.yaml`](./solo_to_shared_to_archived_happy_path.yaml)
  — Full lifecycle happy path: `solo` -> `publish_pending` ->
  `shared_active` -> `ended` -> `archived`. Demonstrates the
  admitted transition matrix, the host-locus / transport / relay
  bindings, the editor-buffer shared object, and the final
  archive seal.
- [`relay_loss_local_editing_continues.yaml`](./relay_loss_local_editing_continues.yaml)
  — A `shared_active` session loses relay; the session transitions
  to `shared_degraded`. The editor-buffer shared object on every
  participant flips to `local_authoritative_no_session_authority`;
  local editing remains unfrozen on every side. Recovery path =
  `rejoin_same_session_same_authority`. Acceptance bullet:
  "Presence or relay loss never forces local buffer rollback or
  freezes local editing."
- [`viewer_fallback_unsent_work_preserved.yaml`](./viewer_fallback_unsent_work_preserved.yaml)
  — A participant's edit admission is revoked by policy narrowing.
  Their unsent local edits are preserved in
  `preserved_in_pending_outbound_proposal_queue`. Recovery paths =
  `rejoin_same_session_viewer_only` plus
  `recover_via_local_journal_diff_export`. Acceptance bullet:
  "Permission downgrades preserve unsent local work and make
  rejoin, diff, or recovery paths explicit."
- [`anchor_target_drift_relocation_forbidden.yaml`](./anchor_target_drift_relocation_forbidden.yaml)
  — An anchored comment's target line is deleted. The drift
  record carries
  `anchor_target_drift_target_deleted_relocation_forbidden`; a
  paired denial event names `silent_anchor_relocation_forbidden`.
  No silent relocation; the legal supersede-rather-than-relocate
  path is illustrated by a paired fresh shared_object_record with
  `supersedes_shared_object_ref`. Acceptance bullet:
  "Ambiguous anchor reattachment is forbidden; fixtures show drift
  labelling rather than silent relocation."
- [`archive_seal_class_inventory.yaml`](./archive_seal_class_inventory.yaml)
  — A session reaches `ended` and seals into `archived`. The
  archive-sealing downgrade record carries the per-class
  inventory: `editor_buffer` rows sealed with
  `sealed_in_archive_metadata_only`, `anchored_comment` rows
  sealed with full payload, `follow_or_presenter_state` excluded
  with `excluded_from_archive_local_authority_only`, and
  `shared_terminal_control_metadata` excluded with
  `excluded_from_archive_redaction_required`. The sealed archive
  shared object is minted with the inventory summary label.
