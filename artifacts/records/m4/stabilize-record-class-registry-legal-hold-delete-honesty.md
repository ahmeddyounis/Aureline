# Record-Class Registry Stabilization — Hold/Delete Honesty and Chronology

- Packet: `records:stabilize_record_class_registry_legal_hold_delete_honesty:default`
- Schema version: `1`
- Contract ref: `records:stabilize_record_class_registry_legal_hold_delete_honesty:v1`
- Qualification: `stable` (derived, not asserted)
- Upstream defects: 0
- Stabilize defects: 0
- Withdrawn rows: 0
- Stable rows: all

## Coverage

### New record classes registered (9)

| Class id | Scope | Residency | Hold eligible | Local/managed distinct |
|---|---|---|---|---|
| `collaboration_session_record` | `collaboration_session` | `local_and_managed` | ✓ | ✓ |
| `collaboration_join_audit_event` | `audit_trail` | `managed_service` | ✓ | ✓ |
| `collaboration_transcript_record` | `collaboration_session` | `local_and_managed` | ✓ | ✓ |
| `observer_only_archive` | `collaboration_session` | `managed_service` | ✓ | — |
| `replayable_session_bundle` | `collaboration_session` | `local_and_managed` | ✓ | ✓ |
| `collaboration_review_evidence` | `managed_copy` | `managed_service` | ✓ | ✓ |
| `operational_audit_record` | `audit_trail` | `managed_service` | ✓ | — |
| `managed_workspace_metadata` | `managed_copy` | `managed_service` | ✓ | — |
| `billing_usage_aggregate` | `export_packet` | `managed_service` | ✓ | — |

### State vocabulary coverage

All 10 outcome tokens implemented and tested:

`requested` · `queued` · `blocked_by_hold` · `completed` · `policy_retained` ·
`outside_platform_scope` · `manual_local_capture_required` · `partial` ·
`not_found` · `omitted_by_redaction`

### Hold-evaluation coverage

- Fail-closed on `active` and `unknown_indeterminate` (both block destructive completion)
- `grants_new_read_rights()` always `false`
- `grants_new_export_rights()` always `false`
- Planning vs execution phase distinction
- Explicit `local_only_artifact_note` field prevents false managed-hold assurances

### Destruction receipt coverage

Receipt fields: `receipt_id`, `executed_action`, `executed_at`, `policy_version`,
`scope_selectors`, `included_classes`, `excluded_classes`, `deleted_refs`,
`skipped_refs`, `retained_refs`, `held_refs`, `outside_scope_refs`,
`total_destroyed_count`, `total_retained_count`, `total_outside_scope_count`,
`hash_checksum_manifest`, `redaction_profile`, `verifier_ref`,
`local_only_not_held_note`, `collab_metadata`

### Export bundle manifest coverage

Manifest fields: `bundle_id`, `created_at`, `scope_selectors`, `time_range_start`,
`time_range_end`, `included_classes`, `excluded_classes`, `omission_reasons`,
`policy_version`, `hash_checksum_manifest`, `redaction_profile`, `signer_ref`,
`refs`, `collab_metadata`

### Chronology export coverage

- `ChronologyEntry`: `event_id`, `actor_ref`, `timestamp_utc`,
  `source_timezone_label`, `event_class`, `record_class_id`,
  `is_local_only`, `is_attributed`
- `ChronologyExport`: UTC-ordered entries, scope selector, policy version,
  `actor_lineage_preserved`, `timezone_aware`

### Collaboration metadata coverage (v22)

`CollabBundleMetadata` fields: `session_ref`, `role`, `guest_boundary`,
`consent_envelope_ref`, `transcript_classes`, `local_only_artifacts_excluded`,
`local_only_artifact_note`

Transcript classes: `terminal_transcript` · `debug_transcript` ·
`session_recording` · `text_only_transcript` · `structured_captions_transcript`

## Key invariants verified

1. All 18 registry rows parse and round-trip through serde without data loss.
2. All 9 new record classes are present in the registry.
3. `collaboration_session_record` and `collaboration_transcript_record` declare
   `residency_scope: local_and_managed` and `local_and_managed_actions_are_distinct: true`.
4. Hold evaluation is fail-closed: `unknown_indeterminate` blocks destructive completion.
5. `HoldEvaluation.grants_new_read_rights()` and `grants_new_export_rights()` return
   `false` unconditionally.
6. Destruction receipts are durable and diffable YAML/JSON and carry separate buckets
   for deleted, skipped, retained, held, and outside-scope refs.
7. Export bundle manifests list scope selectors, time range, included/excluded classes,
   omission reasons, policy version, and hash/checksum manifest.
8. Collaboration metadata in receipts and bundles preserves role, guest-boundary,
   consent-envelope ref, and transcript-class vocabulary.

## Hard guardrails — withdrawal conditions

The following conditions force `Withdrawn` immediately:

- A destruction receipt that claims `completed` without a non-empty `hash_checksum_manifest`.
- A hold evaluation that sets `status: cleared` but lists non-empty `active_hold_refs`.
- A collaboration transcript row that does not declare `residency_scope`.
- Any row claiming `managed_authoritative` with `local_and_managed_actions_are_distinct: true`
  and `managed_copy_posture: forbidden` (contradicts itself).

## Canonical paths

- Doc: `docs/records/m4/stabilize-record-class-registry-legal-hold-delete-honesty.md`
- Runtime owner: `aureline_records::stabilize_record_class_registry_legal_hold_delete_honesty`
- Fixtures: `fixtures/records/m4/stabilize-record-class-registry-legal-hold-delete-honesty/`
- Schema: `schemas/records/record-class-registry.schema.json`
- Registry: `artifacts/governance/record_class_registry_alpha.yaml`
