# Records-chronology and delete-honesty fixture cases

These fixtures exercise the records-chronology and delete-honesty
packet at
[`/artifacts/governance/records_chronology_delete_packet.md`](../../../artifacts/governance/records_chronology_delete_packet.md)
and the matrix at
[`/artifacts/governance/retained_vs_ephemeral_matrix.yaml`](../../../artifacts/governance/retained_vs_ephemeral_matrix.yaml).

Every fixture composes one row from the retained-vs-ephemeral matrix
with one worked `chronology_context_record` (per
[`/schemas/governance/chronology_context.schema.json`](../../../schemas/governance/chronology_context.schema.json)),
one worked `delete_request_state_record` (per
[`/schemas/governance/delete_request_state.schema.json`](../../../schemas/governance/delete_request_state.schema.json),
when a delete request is admissible), and (when the case names a
collaboration row kind) one worked
`collaboration_recorded_artifact_row_record` (per
[`/schemas/collaboration/recorded_artifact_row.schema.json`](../../../schemas/collaboration/recorded_artifact_row.schema.json))
plus one worked `collaboration_delete_path_status_record` (per
[`/schemas/collaboration/delete_path_status.schema.json`](../../../schemas/collaboration/delete_path_status.schema.json)).

## Index

| Case | Fixture | Posture |
| --- | --- | --- |
| Ephemeral collaboration presence row | `presence_grant_ephemeral_no_capture.yaml` | `presence_or_temporary_control_grant_row`, live system truth, no delete admissible |
| Workspace comment thread sync-delayed delete | `comment_thread_sync_delayed_delete.yaml` | `comment_or_review_thread_row`, mirrored managed surface, `delete_requested` with `sync_backlog` |
| Policy-required transcript managed delete completed | `transcript_policy_required_delete_completed.yaml` | `recording_or_transcript_or_replay_artifact_row`, mirrored managed surface, `delete_completed` |
| Transcript exported then managed delete with local copy remaining | `transcript_export_local_copy_remains.yaml` | `recording_or_transcript_or_replay_artifact_row`, mirrored managed surface, `exported_copy_remains_local` |
| Support evidence packet held by tenant legal hold | `support_bundle_archive_legal_hold.yaml` | `deletion_event_or_audit_row`, mirrored managed surface, `legal_hold` |
| Imported support evidence packet outside Aureline delete path | `imported_support_evidence_outside_delete_path.yaml` | `imported_or_support_evidence_packet_row`, imported external audit trail, `deletion_not_yet_requested_inspectable_only` |

## Envelope shape

Each fixture is a YAML envelope with the following top-level fields:

- `__fixture__` — scenario summary, the matrix row id the fixture
  exercises, the upstream contract sections it touches, and the
  honesty-invariant set it asserts.
- `record_delete_case` — header carrying `case_id`, the cited
  `matrix_row_ref`, the cited `record_kind`, the cited
  `default_retention_posture_class`, and a one-sentence
  `case_summary`.
- `chronology_context` — one `chronology_context_record` carrying the
  cited `consumer_row_kind`, `evidence_source_class`,
  `current_state_class`, `timezone_basis`, `relative_time_presentation`,
  `imported_artifact_disclosure`, optional `import_declared` /
  `offline_window` blocks, and optional `deletion_event_extras` /
  `enterprise_session_extras` blocks per the upstream chronology
  schema.
- `delete_request_state` — one `delete_request_state_record` (when
  `delete_request_state_required = true` on the matrix row) or a
  bounded `delete_request_state_omitted_block` carrying the typed
  omitted-reason (when the matrix row admits omission, e.g. live-only
  presence rows or import-origin packets).
- `collaboration_recorded_artifact_row` — one
  `collaboration_recorded_artifact_row_record` (when the case names
  one of the five collaboration `artifact_row_kind` values directly).
- `collaboration_delete_path_status` — one
  `collaboration_delete_path_status_record` (paired with the
  collaboration row).
- `remaining_location_disclosure_review_row` — bounded reviewable
  block summarizing the remaining-location set the bound
  `delete_request_state` discloses, used by reviewers to assess
  what remains, where, and why.
- `delete_request_progression_review_row` — bounded reviewable block
  summarizing the stable-delete-state, the
  `expected_next_state_change.change_class`, and the typed
  partial-blocker set, used by reviewers to assess delete-request
  progression.
- `honesty_assertions` — boolean block re-asserting the cross-cutting
  invariants from the matrix.

## Acceptance coverage

The packet's acceptance criteria are covered as follows:

- **Reviewers can tell for each record class what remains, where it
  remains, and why delete may be delayed, partial, or impossible
  right now.** Every fixture carries a
  `remaining_location_disclosure_review_row` and a
  `delete_request_progression_review_row` that pin the typed
  `stable_delete_state`, the typed `change_class`, the typed
  `partial_blocker_class`, and the typed `remaining_location_class`
  per request.
- **Imported or offline evidence never masquerades as live system
  truth in chronology-heavy reviews.** The
  `imported_support_evidence_outside_delete_path.yaml` fixture
  resolves `evidence_source_class` to
  `imported_external_audit_trail`, `current_state_class` to
  `retained_artifact_imported_origin`, the relative-time basis to
  `since_import_window_end`, and the timezone basis to
  `imported_origin_zone_labeled`. The
  `support_bundle_archive_legal_hold.yaml` fixture pairs
  `mirrored_managed_surface` with the `since_now_with_skew_label`
  basis so the row never claims live-current-system-state.
- **The packet aligns collaboration, governance, and support
  vocabularies instead of requiring per-surface delete wording.**
  Every fixture cites the same vocabulary across the
  `chronology_context`, the `delete_request_state`, and the
  `collaboration_recorded_artifact_row` / `collaboration_delete_path_status`
  blocks; no per-surface synonyms appear.

## Out of scope

Retention-service implementation and legal-system integration are
explicitly out of scope. The fixtures freeze the visible composition
only.
