# Records-chronology and delete-honesty packet

This packet binds the four already-frozen contracts that govern *what
records exist*, *when those records were captured*, *whether they are
retained or ephemeral*, *whether they are live system truth or an
imported / mirrored / replayed artifact*, and *what happens when a
delete is requested* — into one reviewable surface for shared
collaboration sessions, support exports, and imported-evidence
artifacts. It exists so reviewers can answer, for any one record on a
shared / support / imported surface, four questions in a single read:

1. **What kind of record is this?** Collaboration presence or
   temporary control grant, anchored comment or review thread,
   recording / transcript / replay artifact, support or imported
   evidence packet, or deletion-event timeline row.
2. **Is it retained or ephemeral right now, where does it live, and
   on whose clock?** Local-only or hosted on a managed surface; live
   system truth, mirrored, imported, offline-captured, or replayed;
   under which timezone basis and import / offline / replay window.
3. **What happens when delete is requested?** Which stable
   delete-request state the request can reach next, what gates the
   next state change, and which locations still hold a copy.
4. **Why might delete be delayed, partial, or impossible right now?**
   Which typed partial blocker (legal hold, policy retention floor,
   sync / provider backlog, support investigation, manual local
   capture required, import-source unreachable, outside platform
   scope) applies, and whether an exported local copy survives a
   managed delete.

The packet **does not** mint new vocabulary. Every closed enum it
projects is re-exported verbatim from one of the four upstream
contracts and the schemas they own. Drift between this packet and
the upstream sources is a packet bug; the upstream schemas win and
this packet plus the matrix plus the fixtures update in the same
change.

## Companion artifacts

- [`/artifacts/governance/retained_vs_ephemeral_matrix.yaml`](./retained_vs_ephemeral_matrix.yaml)
  — machine-readable matrix with one row per record kind. Each row
  pins the default retained-vs-ephemeral posture, the admissible
  chronology evidence-source / current-state classes, the
  admissible timezone-basis classes, the admissible local-vs-hosted
  location classes, the admissible legal-hold-note posture, the
  admissible exported-local-copy-disclosure posture, the admissible
  delete-request progression states, and the admissible
  remaining-location classes. Every cell cites the upstream contract
  and schema it inherits from.
- [`/fixtures/governance/record_delete_cases/`](../../fixtures/governance/record_delete_cases/)
  — worked `record_delete_case_record` envelopes. Each fixture
  composes one row from the retained-vs-ephemeral matrix with one
  worked `chronology_context_record`, one worked
  `delete_request_state_record`, and (when the case names a
  collaboration row kind) one worked
  `collaboration_recorded_artifact_row_record` and
  `collaboration_delete_path_status_record`, so a reviewer can read
  the shared / support / imported posture as one packet.

## Composed upstream contracts

This packet is a **composition layer**. It cites and reuses, but does
not redefine, vocabulary from:

- [`/docs/governance/retention_deletion_matrix_contract.md`](../../docs/governance/retention_deletion_matrix_contract.md),
  [`/schemas/governance/retention_matrix_row.schema.json`](../../schemas/governance/retention_matrix_row.schema.json),
  and
  [`/schemas/governance/delete_request_state.schema.json`](../../schemas/governance/delete_request_state.schema.json)
  — retention/deletion matrix rows and per-request delete-state
  records. The five stable delete states
  (`delete_requested`, `policy_retention`, `legal_hold`,
  `delete_completed`, `exported_copy_remains_local`), the closed
  `partial_blocker_class` vocabulary (legal hold / policy retention
  floor / sync backlog / provider backlog / support investigation /
  export-pending hold / managed-service-unavailable / manual local
  capture required / redaction policy / outside platform scope /
  import-source unreachable / user declined / entitlement expired),
  the closed `remaining_location_class` vocabulary
  (`local_device_only`, `local_export_copy`, `managed_archive_held`,
  `managed_archive_policy_retained`,
  `managed_archive_replicated_pending_purge`,
  `destruction_receipt_only`, `import_source_origin`,
  `no_remaining_location`), and the closed
  `expected_next_state_change.change_class` vocabulary all flow
  through this packet verbatim.
- [`/docs/collaboration/recording_retention_delete_contract.md`](../../docs/collaboration/recording_retention_delete_contract.md),
  [`/schemas/collaboration/recorded_artifact_row.schema.json`](../../schemas/collaboration/recorded_artifact_row.schema.json),
  and
  [`/schemas/collaboration/delete_path_status.schema.json`](../../schemas/collaboration/delete_path_status.schema.json)
  — collaboration visible-row vocabulary and delete-path-status
  shape. The five `artifact_row_kind` values
  (`presence_or_temporary_control_grant_row`,
  `comment_or_review_thread_row`,
  `recording_or_transcript_or_replay_artifact_row`,
  `deletion_event_or_audit_row`,
  `imported_or_support_evidence_packet_row`), the closed
  `recording_state_class`, `retention_owner_class`, `location_class`,
  `export_availability_class`,
  `default_ephemeral_versus_explicit_enable_class`,
  `legal_hold_note_class`, and `irreversible_share_warning_class`
  vocabularies, plus the seven `delete_path_status_class` values and
  six `follow_up_wording_class` values flow through this packet
  verbatim.
- [`/docs/governance/chronology_context_contract.md`](../../docs/governance/chronology_context_contract.md),
  [`/schemas/governance/chronology_context.schema.json`](../../schemas/governance/chronology_context.schema.json),
  and
  [`/artifacts/governance/evidence_source_classes.yaml`](./evidence_source_classes.yaml)
  — chronology-context record shape and closed evidence-source /
  current-state / timezone-basis / relative-time-basis /
  imported-artifact-disclosure / mirror-lag vocabularies. The ten
  source classes (`live_system_truth`, `live_with_bounded_skew`,
  `mirrored_managed_surface`, `imported_remote_agent`,
  `imported_extension`, `imported_external_audit_trail`,
  `offline_local_evidence_packet`, `support_bundle_replay`,
  `recovery_snapshot_replay`, `synthetic_fixture_only`) and the
  seven current-state classes flow through this packet verbatim.
- [`/docs/governance/records_governance_packet_contract.md`](../../docs/governance/records_governance_packet_contract.md)
  and
  [`/schemas/governance/records_governance_packet.schema.json`](../../schemas/governance/records_governance_packet.schema.json)
  — records-governance packet shape, audience / redaction profile
  pairings, change-significance vocabulary, and open-held-data
  caveat vocabulary. This packet is consumed by the records-
  governance packet's
  `linked_artifact_families.collaboration_evidence_refs[]`,
  `support_export_packet_refs[]`, and `ai_retained_evidence_refs[]`
  buckets when a records-governance review references a
  shared-session record, a support-export record, or an
  imported-evidence record.

The packet additionally cites:

- [`/artifacts/governance/record_class_registry.yaml`](./record_class_registry.yaml)
  for class-level retention / hold / delete / export / offboarding
  posture; the retained-vs-ephemeral matrix narrows this registry
  for the five record kinds in this packet's scope.
- [`/docs/governance/privacy_history_and_lifecycle_contract.md`](../../docs/governance/privacy_history_and_lifecycle_contract.md)
  for the lifecycle vocabulary
  (`Local only`, `Uploaded`, `Redacted`, `Held`, `Destroyed`,
  `Pending`) every renderer must use.
- [`/docs/governance/storage_and_retention_vocabulary.md`](../../docs/governance/storage_and_retention_vocabulary.md)
  for the storage / retention / raw-secret-exclusion vocabulary
  every export path inherits.

If this packet disagrees with any of those upstream sources, the
upstream sources win and this packet plus the matrix plus the
fixtures are updated in the same change.

## Why this packet exists

Collaboration sessions, support exports, and imported evidence
artifacts already each have their own contract. Without one shared
packet, a reviewer asking *"for this row on this shared / support /
imported surface, what is retained, where, on whose clock, with what
delete posture, and why might delete be delayed?"* still has to walk
five different documents and compose the answer manually:

- The collaboration recorded-artifact row says "captured export
  available" but does not say what chronology source class the
  rendered timestamp belongs to.
- The chronology-context record says "imported remote agent under a
  partial-order clock" but does not say which retention/deletion
  matrix row owns it or whether a delete is even possible inside
  Aureline.
- The records-governance packet says "support_bundle_archive picked
  up a tenant_legal_hold" but does not say what an inspector should
  render on a row that quotes the held archive.
- The retention/deletion matrix row says "Legal hold blocks managed
  delete completion" but does not say whether the row is being
  rendered on a shared session, in a support export, or as an
  imported evidence packet whose authoritative copy lives outside
  Aureline.
- The privacy-history contract names the stable delete states but
  does not say which collaboration row kind they apply to in the
  first place.

This packet composes those five answers. Every record kind in scope
gets one row in the retained-vs-ephemeral matrix (§2). Every worked
fixture demonstrates one realistic composition: a record kind plus a
chronology source class plus a retained-vs-ephemeral posture plus a
delete-request progression plus a typed remaining-location
disclosure. Reviewers read one packet and one fixture instead of
hunting across five documents.

## 1. Record kinds in scope

The packet's record-kind vocabulary mirrors the collaboration
`artifact_row_kind` enum verbatim. It is closed:

| Record kind | Description | Default retained-vs-ephemeral posture |
| --- | --- | --- |
| `presence_or_temporary_control_grant_row` | Session-presence and temporary terminal / debugger / runbook control grants. The grant itself, not the lane payload. | **Ephemeral by default**; `default_ephemeral_no_capture` until the granted lane explicitly admits capture. |
| `comment_or_review_thread_row` | Anchored comments and review threads. Both local-cached and managed-archived forms admitted. | **Retained by default** under workspace-owned retention; local cache is regenerable while the managed archive is authoritative. |
| `recording_or_transcript_or_replay_artifact_row` | Full-session recording, text-only transcript, structured-captions transcript, and replay-artifact metadata sealed at archive seal. | **Ephemeral by default** under `default_ephemeral_no_capture`; admitted only under `explicit_enable_user_required` or `explicit_enable_policy_forced_admin_signed`. |
| `deletion_event_or_audit_row` | The deletion-event audit row itself plus collaboration audit events admitted as visible rows. | **Retained**; metadata-only; never collapses to "delete completed" while the bound delete-request is held or partially blocked. |
| `imported_or_support_evidence_packet_row` | Imported support evidence packets and externally-sourced compliance imports the session admits as visible rows but does not itself originate. | **Retained at import origin**; `imported_origin_not_subject_to_default`; the authoritative copy lives outside Aureline's delete path. |

A surface that mints a private record kind denies with
`record_kind_unresolved` and falls back to "evidence kind unknown"
rather than collapsing to one of the five admissible kinds.

## 2. Retained-vs-ephemeral matrix

Every shared-collaboration, support-export, or imported-evidence
record resolves to exactly one row in
[`/artifacts/governance/retained_vs_ephemeral_matrix.yaml`](./retained_vs_ephemeral_matrix.yaml).
Each row carries:

- `record_kind` — one of the five from §1.
- `default_retention_posture_class` — one of:
  - `ephemeral_default_no_capture` — admitted by the mode but no
    capture is enabled and none will start without an explicit-enable
    cue. Pairs with `recording_state_class =
    default_ephemeral_no_capture` on the collaboration row.
  - `ephemeral_admitted_metadata_only` — only metadata is admitted;
    payload bytes are not retained even if the row is.
  - `retained_local_authoritative` — local-only retention; deleting
    the local row destroys the authoritative copy.
  - `retained_managed_authoritative` — managed-surface retention;
    deleting the local cache does not affect the managed
    authoritative copy.
  - `retained_hybrid_local_and_managed` — both forms retained; local
    and managed deletes are distinct actions.
  - `retained_at_import_origin_outside_aureline` — authoritative copy
    lives at the import origin and is outside Aureline's delete path.
- `admissible_chronology_evidence_source_class[]` — non-empty subset
  of the chronology contract's ten-class enum.
- `admissible_current_state_class[]` — non-empty subset of the
  chronology contract's seven-class current-state enum. A row whose
  `default_retention_posture_class` is anything but
  `retained_managed_authoritative` MUST NOT include
  `live_current_system_state` for its admissible state set unless a
  live-with-bounded-skew row is also explicitly admitted.
- `admissible_timezone_basis_class[]` — non-empty subset of the
  six-class chronology timezone-basis enum.
- `admissible_location_class[]` — non-empty subset of the
  collaboration recorded-artifact-row eight-class location enum.
- `admissible_legal_hold_note_class[]` — non-empty subset of the
  recorded-artifact-row four-class hold-note enum.
- `admissible_exported_local_copy_disclosure_class[]` — non-empty
  subset of the collaboration delete-path-status three-class
  disclosure enum.
- `admissible_stable_delete_state[]` — non-empty subset of the five
  stable delete states from the privacy-history / retention-matrix
  contracts.
- `admissible_partial_blocker_class[]` — subset of the
  retention-matrix `partial_blocker_class` vocabulary; empty for
  rows whose default posture is `ephemeral_default_no_capture` or
  `retained_at_import_origin_outside_aureline` (those rows answer
  through the typed follow-up wording instead).
- `admissible_remaining_location_class[]` — subset of the
  retention-matrix `remaining_location_class` vocabulary.
- `inherits_from` — list of `{contract_doc_ref, schema_ref}` pairs
  pointing at the upstream contract(s) the cell vocabulary comes
  from. No new vocabulary is minted in the matrix.

A row whose admissible-set cardinality drops below the floor cited
above denies with `admissible_set_under_floor` rather than
collapsing to a default.

## 3. Chronology layer

Every `record_delete_case_record` cites exactly one
`chronology_context_record` (per
[`/schemas/governance/chronology_context.schema.json`](../../schemas/governance/chronology_context.schema.json))
on `chronology_context`. The chronology record carries:

- `consumer_row_kind` — exactly one of the chronology contract's
  ten consumer-row kinds. The packet narrows the admissible kinds
  per record-kind row in §1:
  - `presence_or_temporary_control_grant_row` ↔
    `enterprise_session_row` or `capture_event_row`.
  - `comment_or_review_thread_row` ↔
    `comment_or_review_thread_row`.
  - `recording_or_transcript_or_replay_artifact_row` ↔
    `enterprise_session_row` or `capture_event_row`.
  - `deletion_event_or_audit_row` ↔ `deletion_event_row` or
    `audit_event_row`.
  - `imported_or_support_evidence_packet_row` ↔
    `support_bundle_row`, `audit_event_row`, or
    `incident_timeline_row`.
- `evidence_source_class`, `current_state_class`, `timezone_basis`,
  `relative_time_presentation`, `imported_artifact_disclosure`,
  `import_declared`, `offline_window`, and
  `mirror_lag_state_class` — verbatim from the chronology schema.

The honesty invariants from the chronology contract apply unchanged:
a row whose `evidence_source_class` is anything other than
`live_system_truth` or `live_with_bounded_skew` MUST NOT resolve
`current_state_class` to `live_current_system_state`. The packet
re-asserts:

- **Imported-vs-live distinction.** An
  `imported_or_support_evidence_packet_row` MUST resolve
  `evidence_source_class` to one of the imported family
  (`imported_remote_agent`, `imported_extension`,
  `imported_external_audit_trail`) or, when the record represents a
  replayed import, to `support_bundle_replay` /
  `recovery_snapshot_replay`. The matching `current_state_class` is
  one of the retained-artifact values; rendering a live chip denies
  with `live_state_claimed_on_imported_row`.
- **Offline-vs-live distinction.** A row captured during an offline
  window MUST resolve `evidence_source_class` to
  `offline_local_evidence_packet`, `current_state_class` to
  `retained_artifact_offline_local`, and the relative-time basis to
  `since_offline_window_end`. Rendering a live chip denies with
  `live_state_claimed_on_offline_row`.
- **Mirrored-vs-live distinction.** A row mirrored from a managed
  surface MUST resolve `evidence_source_class` to
  `mirrored_managed_surface`, the relative-time basis to
  `since_now_with_skew_label`, and the `mirror_lag_state_class` to
  one of the chronology contract's mirror-lag values. Rendering a
  raw "live" chip without the skew label denies with
  `skew_label_not_rendered_on_unsynchronized_row`.

## 4. Delete-request progression rows

Every `record_delete_case_record` that admits a delete request cites
exactly one `delete_request_state_record` (per
[`/schemas/governance/delete_request_state.schema.json`](../../schemas/governance/delete_request_state.schema.json))
on `delete_request_state`. Records whose `record_kind` is
`presence_or_temporary_control_grant_row` and whose
`default_retention_posture_class` resolves to
`ephemeral_default_no_capture` MAY omit the
`delete_request_state` block; in that case the case record carries
`delete_request_progression_summary.delete_request_state_omitted_reason
= no_request_admissible_live_only_row` and the
`collaboration_delete_path_status` block (when present) resolves to
`deletion_not_applicable_live_only_row`. Records whose
`default_retention_posture_class` is
`retained_at_import_origin_outside_aureline` carry a
`delete_request_state` whose `stable_delete_state` is
`delete_requested` only when a request was actually opened against
Aureline; otherwise they pair with
`delete_path_status_class =
deletion_not_yet_requested_inspectable_only` and
`follow_up_wording_class =
follow_up_third_party_origin_outside_aureline_scope`.

The packet's review row for delete-request progression always
discloses, per the upstream delete-request schema:

- `stable_delete_state` and the verbatim `stable_state_label` (no
  synonyms; "Removed", "Done", "Cleared", "Gone" are
  non-conforming).
- `expected_next_state_change.change_class` — exactly one of
  `definite_at_chronology`, `by_chronology_or_sooner`,
  `after_propagation_completes`, `after_provider_backlog_clears`,
  `after_sync_backlog_clears`, `on_hold_review_or_clear`,
  `on_policy_floor_reached`, `on_export_window_close`,
  `no_further_state_change_expected`, or
  `indeterminate_pending_admin`. Substituting "should be quick" or
  "still working on it" is non-conforming.
- `partial_blockers[].blocker_class` — typed family per blocker; a
  blocker whose class is not resolvable denies with
  `partial_blocker_class_unresolved` rather than collapsing to a
  generic "pending".

## 5. Remaining-location disclosure rows

Every `record_delete_case_record` whose
`stable_delete_state` is `policy_retention`, `legal_hold`, or
`exported_copy_remains_local` MUST list at least one entry on
`delete_request_state.remaining_location_disclosure.locations[]`
whose `location_class` is **not** `no_remaining_location`. The
packet narrows the admissible remaining-location classes per record
kind:

| Record kind | Required remaining-location class when delete is held / partial / exported |
| --- | --- |
| `presence_or_temporary_control_grant_row` | n/a — live-only rows resolve to `deletion_not_applicable_live_only_row` and emit no remaining-location entry. |
| `comment_or_review_thread_row` | At least one of `local_device_only`, `local_export_copy`, `managed_archive_held`, `managed_archive_policy_retained`, `managed_archive_replicated_pending_purge`, or `destruction_receipt_only`. |
| `recording_or_transcript_or_replay_artifact_row` | At least one of `local_export_copy`, `managed_archive_held`, `managed_archive_policy_retained`, `managed_archive_replicated_pending_purge`, or `destruction_receipt_only`. A row that exported a downloadable copy MUST list `local_export_copy`. |
| `deletion_event_or_audit_row` | At least one of `managed_archive_held`, `managed_archive_policy_retained`, or `destruction_receipt_only`. |
| `imported_or_support_evidence_packet_row` | At least one of `import_source_origin` or `local_export_copy`. The row MUST NOT claim `no_remaining_location` while the import origin still holds the authoritative copy. |

A row whose remaining-location set drops to
`no_remaining_location` while the bound `stable_delete_state` is
held or exported-local-copy denies with
`remaining_location_disclosure_required` (verbatim from the
upstream delete-request schema's denial vocabulary).

The **exported-local-copy-remains** rule is preserved end to end:
when a managed delete completes but a downloaded export survives on
the user's device, the request resolves to
`stable_delete_state = exported_copy_remains_local`, the
collaboration `delete_path_status_class =
deletion_completed_but_exported_local_copy_surviving`, the
`exported_local_copy_disclosure_class =
local_copy_exported_and_disclosed`, and the chronology row's
relative-time presentation cites `since_now_with_skew_label` (the
managed delete) plus a separate row for the local export packet
under `local_export_copy`. A row that flips to "Delete completed"
while a local export survives denies with the upstream
`exported_local_copy_must_be_disclosed` reason.

## 6. Local-vs-hosted distinction

Every `record_delete_case_record` resolves
`location_class` (per the collaboration recorded-artifact-row
vocabulary). The packet enforces:

- Rows with `location_class` in
  `{local_workspace_only, local_export_copy_only}` MUST resolve the
  chronology `evidence_source_class` to `live_system_truth` /
  `live_with_bounded_skew` / `offline_local_evidence_packet` only,
  and MUST NOT claim `mirrored_managed_surface`.
- Rows with `location_class` in
  `{hosted_managed_surface_only,
  hosted_managed_surface_plus_local_workspace,
  hosted_managed_surface_plus_local_export_copy,
  sealed_session_archive_only}` MUST resolve the chronology
  `evidence_source_class` to `mirrored_managed_surface` (when the
  surface mirrors the managed copy locally) or to
  `live_with_bounded_skew` (when the row is rendered directly on the
  managed surface).
- Rows with `location_class = third_party_import_origin` MUST resolve
  the chronology `evidence_source_class` to one of the imported
  family or to `support_bundle_replay` /
  `recovery_snapshot_replay`.
- Rows with `location_class = no_location_live_only` MUST pair with
  `default_retention_posture_class = ephemeral_default_no_capture`,
  with `stable_delete_state` omitted (per §4), and with the
  collaboration `delete_path_status_class =
  deletion_not_applicable_live_only_row`.

## 7. Timezone basis

Every chronology-context row in the packet preserves the upstream
timezone-basis honesty rule:

- A row whose `timezone_basis_class` names a real zone
  (`device_local_iana`, `deployment_pinned_iana`, or
  `imported_origin_zone_labeled`) MUST carry the
  `timezone_basis.display_time_zone_iana` IANA zone.
- A row whose `evidence_source_class` is `mirrored_managed_surface`
  MUST resolve the relative-time basis to
  `since_now_with_skew_label`.
- An export of any record in this packet pins
  `timezone_basis_class = canonical_utc` and the absolute-time
  format to `iso_8601_utc` per the chronology contract's
  export-parity rule.

A row that elides the IANA zone while naming a real-zone basis
denies with the upstream `display_time_zone_iana` `if/then` rule.

## 8. Legal-hold honesty

Every `record_delete_case_record` whose retention posture admits a
hold pairs three signals consistently:

- `delete_request_state.stable_delete_state = legal_hold` ↔
  `collaboration_delete_path_status.delete_path_status_class =
  deletion_blocked_legal_hold` (when the case names a collaboration
  row) ↔
  `recorded_artifact_row.legal_hold_note_class =
  hold_active_user_delete_blocked` or
  `hold_active_user_delete_recorded_not_completed`.
- `delete_request_state.expected_next_state_change.change_class =
  on_hold_review_or_clear` with a non-null `gate_ref` to the
  hold record.
- `delete_request_state.partial_blockers[]` carries an entry whose
  `blocker_class = legal_hold` and whose `hold_ref` cites the
  hold; if the hold is also a support investigation, the row carries
  a second entry with `blocker_class = support_investigation`.

A held row that flips to `delete_completed` denies with
`held_or_retained_not_reported_destroyed` (verbatim from the
upstream retention-matrix and delete-request schemas) and, when the
case names a collaboration row, with the upstream
`legal_hold_note_class_must_match_block_state` /
`follow_up_wording_must_match_block_state` denial reasons.

## 9. Imported / offline / replayed evidence cannot masquerade as live

A `record_delete_case_record` whose `record_kind` is
`imported_or_support_evidence_packet_row` MUST:

- Resolve `chronology_context.evidence_source_class` to one of
  `imported_remote_agent`, `imported_extension`,
  `imported_external_audit_trail`, `support_bundle_replay`, or
  `recovery_snapshot_replay`.
- Resolve `chronology_context.current_state_class` to one of the
  retained-artifact values (`retained_artifact_imported_origin`,
  `retained_artifact_support_replay`, or
  `retained_artifact_recovery_replay`).
- Carry a non-null
  `chronology_context.import_declared` block when the source class
  is in the imported family, with explicit
  `import_window_start_utc` / `import_window_end_utc` and an
  `import_origin_label`.
- Carry a non-null `evidence_source_ref` opaque id pointing at the
  import-origin manifest, support-bundle id, or recovery-snapshot id.
- Resolve the collaboration row (when present) to
  `artifact_row_kind = imported_or_support_evidence_packet_row`,
  `retention_owner_class = third_party_imported_origin`,
  `location_class = third_party_import_origin`,
  `default_ephemeral_versus_explicit_enable_class =
  imported_origin_not_subject_to_default`,
  `export_availability_class` in
  `{no_export_available,
  third_party_re_export_outside_aureline_scope}`, and
  `delete_path_status_class =
  deletion_not_yet_requested_inspectable_only` paired with
  `follow_up_wording_class =
  follow_up_third_party_origin_outside_aureline_scope`.

A row that violates any of these denies with the matching upstream
denial reason
(`live_state_claimed_on_imported_row`,
`live_state_claimed_on_replay_row`,
`imported_row_missing_import_declared_block`,
`imported_row_missing_evidence_source_ref`,
`support_bundle_replay_missing_support_bundle_ref`,
`recovery_snapshot_replay_missing_recovery_manifest_ref`, or
`imported_packet_must_cite_third_party_import_origin`).

## 10. Vocabulary alignment across collaboration / governance / support

The packet's purpose is to align — not to re-mint — the collaboration,
governance, and support vocabularies for these record kinds. The
alignment table below pins how each axis flows from the upstream
contract through this packet without renaming:

| Axis | Collaboration vocabulary | Governance vocabulary | Support / records-governance vocabulary |
| --- | --- | --- | --- |
| Retained vs ephemeral default | `recording_state_class` | `retention_matrix_row.default_retention.trigger_class` and `duration_class` | `records_governance_packet.record_class_registry_diff` |
| Local vs hosted | `location_class` | `retention_matrix_row.location_class` | `records_governance_packet.export_delete_contract_status` |
| Stable delete state | `delete_path_status_class` | `delete_request_state.stable_delete_state` | `records_governance_packet.export_delete_contract_status.delete_request_state_status_class` |
| Partial blocker | `hold_kind_class` (block branch) | `delete_request_state.partial_blockers[].blocker_class` | `records_governance_packet.open_held_data_caveats[].caveat_class` |
| Remaining location | `exported_local_copy_disclosure_class` | `delete_request_state.remaining_location_disclosure.locations[].location_class` | `records_governance_packet.open_held_data_caveats[].affected_matrix_row_refs` |
| Imported / live distinction | `default_ephemeral_versus_explicit_enable_class` (`imported_origin_not_subject_to_default` branch) | `chronology_context.evidence_source_class` and `current_state_class` | `records_governance_packet.linked_artifact_families.collaboration_evidence_refs[]` and `support_export_packet_refs[]` |

A surface that mints "shared session retention", "support archive
hold", or "imported evidence delete" wording instead of resolving
through this table is non-conforming.

## 11. Honesty invariants

Every `record_delete_case_record` MUST assert (`true` constants on
the boolean fields):

- `held_or_retained_not_reported_destroyed` — held / policy-retained
  rows never flip to `delete_completed` (verbatim from the
  retention-matrix and delete-request schemas).
- `raw_payloads_not_embedded` — no raw editor buffers, raw terminal
  bytes, raw debug payloads, raw provider responses, raw URLs, raw
  hostnames, raw email addresses, raw phone numbers, raw chat-room
  URLs, raw legal-hold justification text, or raw policy-bundle
  bytes cross any record in the packet.
- `local_export_copy_disclosed_when_required` — when a managed
  delete completes but a downloaded export survives, the row
  resolves to `exported_copy_remains_local` and lists
  `local_export_copy` on the remaining-location disclosure.
- `imported_origin_not_silently_re_owned` — imported rows preserve
  their `import_declared` block and never resolve a
  `retention_owner_class` other than `third_party_imported_origin`.
- `policy_owner_named` — every retained row names the typed
  retention and delete owner classes from the upstream
  retention-matrix vocabulary.
- `expected_next_state_change_disclosed` — every delete request
  cites a typed `change_class`; "should be quick" / "still working
  on it" substitutes are non-conforming.
- `handoff_does_not_widen_scope` — typed support / admin /
  compliance handoff packets never include payloads outside the
  matrix rows in scope.

A `record_delete_case_record` that asserts any of these as `false`
is rejected.

## 12. Acceptance crosswalk

The packet meets the spec's acceptance criteria as follows:

- **"Reviewers can tell for each record class what remains, where
  it remains, and why delete may be delayed, partial, or impossible
  right now."** — every fixture binds the typed
  `stable_delete_state`, the typed `partial_blocker_class`, and the
  typed `remaining_location_class` together with one
  `expected_next_state_change.change_class` per request. A reviewer
  reads one record and sees what remains, where, and why.
- **"Imported or offline evidence never masquerades as live system
  truth in chronology-heavy reviews."** — §3 and §9 forbid the
  imported / offline / replayed source classes from resolving
  `current_state_class` to `live_current_system_state`. The
  imported-evidence fixture and the offline-capture fixture exercise
  the deny gates verbatim.
- **"The packet aligns collaboration, governance, and support
  vocabularies instead of requiring per-surface delete wording."** —
  §10 pins one alignment table that the collaboration, governance,
  and support surfaces all read; no per-surface wording is admitted.

## 13. Out of scope

This packet does not implement:

- Retention enforcement, deletion backends, legal-hold tooling,
  collaboration recording / transcription / replay, or compliance
  automation.
- Legal-system integration; "legal hold" here is the visible-row
  posture, not the back-office workflow.
- The retention-service, the support-bundle pipeline, the
  collaboration transport, or the import / replay backend.

These are explicitly deferred. This packet freezes the visible
composition that those implementations will read and write.
