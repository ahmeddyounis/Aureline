# Stabilize record-class registry: hold/delete honesty, legal-hold states, and timezone-aware chronology

This stable lane makes record existence, retention ownership, hold status, delete state,
and timezone-aware chronology explicit for managed, enterprise, and support evidence rows.
It covers collaboration session records, join/request/admit/revoke audit events, shared-
terminal and shared-debug transcript classes, observer-only archives, replayable/exportable
session bundles, collaboration review evidence, operational audit/policy history, managed-
workspace metadata, and billing/usage aggregates.

The runtime owner is
`aureline_records::stabilize_record_class_registry_legal_hold_delete_honesty`.

No record class in this lane claims delete completion without a durable destruction receipt.
No local-only artifact is presented as held or deleted by managed controls. Hold logic never
hides user-local continuity or manually required capture steps.

## Unified outcome vocabulary

One `RecordOperationOutcome` enum is used across UI chips, CLI/headless output, support
export packets, and admin audit views:

| Token | Meaning |
|---|---|
| `requested` | The operation was requested but not yet acknowledged. |
| `queued` | The operation is queued and waiting to execute. |
| `blocked_by_hold` | A legal or policy hold is blocking the operation. |
| `completed` | The operation completed successfully. |
| `policy_retained` | The record is retained by policy and cannot be deleted yet. |
| `outside_platform_scope` | The record is outside the platform's management scope. |
| `manual_local_capture_required` | The artifact lives only on a local device; manual capture is required. |
| `partial` | The operation completed for some records but not all. |
| `not_found` | No matching record was found. |
| `omitted_by_redaction` | The record exists but its content was withheld by redaction policy. |

`omitted_by_redaction` must never be presented as equivalent to `not_found`.

## Hold evaluation — fail-closed contract

`HoldEvaluation` resolves to one of three statuses: `active`, `unknown_indeterminate`,
or `cleared`. Both `active` and `unknown_indeterminate` block destructive completion
(fail-closed). The `cleared` status permits the destructive action to proceed if other
checks also pass.

**Hold never grants new read or export rights.** `HoldEvaluation.grants_new_read_rights()`
and `HoldEvaluation.grants_new_export_rights()` return `false` unconditionally. A hold
blocks deletion; it does not imply that the held records become newly accessible or
exportable.

Two evaluation phases exist:
- **Planning**: pre-flight check before the action is scheduled.
- **Execution**: final safety gate immediately before the destructive action executes.

When local-only artifacts are in scope, `local_only_artifact_note` must be set to prevent
false assurances that managed hold covers artifacts the platform does not possess.

## Chronology export

`ChronologyExport` produces an ordered, timezone-aware sequence of `ChronologyEntry`
records. Entries are ordered ascending by `timestamp_utc` (UTC RFC 3339). Consumers
must not re-sort by local timestamps; the UTC ordering in this export is canonical.

Each entry carries:
- `event_id` — stable, opaque event id
- `actor_ref` — opaque actor reference (never raw identity or credential material)
- `timestamp_utc` — UTC timestamp in RFC 3339 format
- `source_timezone_label` — IANA timezone label for the originating clock, when available
- `event_class` — event-class token
- `record_class_id` — governing record class, if applicable
- `is_local_only` — whether the event is outside managed hold/delete scope
- `is_attributed` — whether actor lineage is preserved for this entry

## Export bundle manifest

Every export bundle must carry an `ExportBundleManifest` that lists:

- `scope_selectors` — what was in scope
- `time_range_start` / `time_range_end` — time range, if bounded
- `included_classes` / `excluded_classes` — which record classes were covered
- `omission_reasons` — why specific classes were omitted or redacted
- `policy_version` — policy in effect when the bundle was generated
- `hash_checksum_manifest` — integrity hashes for included refs
- `redaction_profile` — redaction applied (`metadata_safe_default`, `structured_field_only`, `full_content_omit`)
- `signer_ref` — verifier reference
- `refs` — all refs with resolved `RecordOperationOutcome`
- `collab_metadata` — collaboration-specific metadata, when applicable

## Destruction receipt

`DestructionReceipt` carries separate ref buckets for each outcome:

| Bucket | Outcome |
|---|---|
| `deleted_refs` | Deletion confirmed |
| `skipped_refs` | Skipped due to exclusion rules |
| `retained_refs` | Retained by policy |
| `held_refs` | Blocked by active or indeterminate hold |
| `outside_scope_refs` | Outside platform management scope |

The receipt also includes `total_destroyed_count`, `total_retained_count`,
`total_outside_scope_count`, a `hash_checksum_manifest`, and `local_only_not_held_note`
to prevent false assurances of managed deletion for local-only content.

## Collaboration evidence metadata (v22)

`CollabBundleMetadata` is appended to export bundles and destruction receipts when
the evidence covers collaboration records. It preserves:

- `session_ref` — opaque session ref
- `role` — `host`, `participant`, `observer`, or `admin`
- `guest_boundary` — whether the records cross a guest/host boundary
- `consent_envelope_ref` — opaque ref to the consent envelope
- `transcript_classes` — transcript classes present in the bundle
- `local_only_artifacts_excluded` — whether local-only artifacts were excluded
- `local_only_artifact_note` — explicit note when local-only artifacts are excluded

Transcript classes: `terminal_transcript`, `debug_transcript`, `session_recording`,
`text_only_transcript`, `structured_captions_transcript`.

## Registry coverage

The alpha registry (`artifacts/governance/record_class_registry_alpha.yaml`) now
registers 18 record classes including the 9 new managed, enterprise, and collaboration
classes. Each new row declares:

- `residency_scope` — explicit residency (e.g. `local_and_managed`, `managed_service`)
- `redaction_profile` — redaction profile applied to exported evidence
- Separate `local_truth.local_and_managed_actions_are_distinct` where applicable
- `local_only_artifact_note` in the managed copy label for mixed-residency rows

## Boundary

The following material stays outside this lane's boundary:

- Raw buffer text, raw terminal bytes, raw debug payloads, raw URLs, raw absolute paths.
- Raw user identifiers, raw billing-account ids, raw API keys, raw OAuth tokens, raw mTLS material.
- Raw hold bodies, raw policy bundles, raw evidence logs.
- Inline deletion assurances not backed by a durable destruction receipt.

Every exported field carries a closed-vocabulary token, a plain-language label, an opaque
ref, a count, a schema-version integer, or a checksum.

## Truth source

- Rust types: `crates/aureline-records/src/stabilize_record_class_registry_legal_hold_delete_honesty/mod.rs`
- Registry: `artifacts/governance/record_class_registry_alpha.yaml`
- Schema: `schemas/records/record-class-registry.schema.json`
- Fixtures: `fixtures/records/m4/stabilize-record-class-registry-legal-hold-delete-honesty/`
- Artifact: `artifacts/records/m4/stabilize-record-class-registry-legal-hold-delete-honesty.md`
