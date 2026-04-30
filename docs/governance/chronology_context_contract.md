# Chronology-context, timestamp-timezone, and imported-vs-live evidence-distinction contract

This contract makes history-heavy surfaces honest about *when* an event
happened and *whether the row is live system truth or a retained
artifact*. It binds run history, capture-event lanes, comments and
review threads, deletion-event timelines, audit-event lanes, incident
timelines, enterprise-session inspectors, support-bundle rows,
AI-evidence packet rows, and chronology export previews to one shared
record shape, one closed evidence-source-class vocabulary, and one
closed current-state vocabulary. Imported, mirrored, offline, and
replayed evidence cannot masquerade as live system truth in the UI, in
the export, or in the support flow.

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabulary every consumer reuses, the imported /
offline / live / mirrored / replayed disclosure rules, the export and
support-parity rules, and the fixture corpus. It does not implement
time-sync, retention, import pipelines, or replay backends.

## Companion artifacts

- [`/schemas/governance/chronology_context.schema.json`](../../schemas/governance/chronology_context.schema.json)
  — boundary schema for one `chronology_context_record`.
- [`/artifacts/governance/evidence_source_classes.yaml`](../../artifacts/governance/evidence_source_classes.yaml)
  — machine-readable register of the closed evidence-source-class,
  current-state, consumer-row-kind, enterprise-session-required-field,
  forbidden-phrase, and denial-reason vocabularies. The schema's enums
  are generated from this file verbatim.
- [`/fixtures/governance/chronology_context_cases/`](../../fixtures/governance/chronology_context_cases/)
  — worked records covering live system truth, imported remote-agent
  trace, offline local evidence packet, deletion-timeline row held by a
  legal hold, and an enterprise-session handoff under default-ephemeral
  recording.
- [`./time_semantics.md`](./time_semantics.md) and
  [`/schemas/governance/timestamp_envelope.schema.json`](../../schemas/governance/timestamp_envelope.schema.json)
  remain the canonical timestamp-envelope contract. Every chronology-
  context record cites a `timestamp_envelope_record`,
  `time_range_envelope_record`, or `partial_chronology_segment_record`
  through `timestamp_envelope_ref`; this contract layers source-class
  and current-state vocabulary on top without redefining canonical
  storage, monotonic ordering, skew, or partial-order labelling.
- [`./record_state_and_policy_simulation_models.md`](./record_state_and_policy_simulation_models.md)
  and
  [`/schemas/governance/record_state.schema.json`](../../schemas/governance/record_state.schema.json)
  remain the per-record state machine. Deletion-timeline rows resolve
  through the privacy-history `stable_delete_state` vocabulary verbatim.
- [`./privacy_history_and_lifecycle_contract.md`](./privacy_history_and_lifecycle_contract.md)
  and
  [`/schemas/governance/export_delete_request_summary.schema.json`](../../schemas/governance/export_delete_request_summary.schema.json)
  remain the privacy-history and export/delete-summary contract.
  Deletion-event rows on a chronology context cite a
  `delete_request_state_record` from that contract through
  `deletion_event_extras.delete_request_state_ref`.
- [`/docs/observability/observability_signal_contract.md`](../observability/observability_signal_contract.md)
  remains the observability signal-slice contract. Signal slices that
  carry chronology context embed a `chronology_context_record`; this
  contract is the layer the signal slice cites for source-class and
  current-state posture.
- [`/docs/collaboration/recording_retention_delete_contract.md`](../collaboration/recording_retention_delete_contract.md)
  and
  [`/schemas/collaboration/recorded_artifact_row.schema.json`](../../schemas/collaboration/recorded_artifact_row.schema.json)
  remain the collaboration recording-retention-delete contract.
  Enterprise-session chronology rows reuse the `recording_state_class`
  vocabulary from that contract verbatim.

If this document and the schema disagree, the schema wins and this
document must be updated in the same change.

## Why this contract exists

Aureline already has the timestamp-envelope contract for canonical
storage, the privacy-history contract for delete states, the
observability signal-slice contract for log / metric / trace freshness,
and the collaboration recording-retention contract for session capture.
Without one chronology-context layer on top, every history-heavy
surface still drifts into per-surface labels:

- a build-history row from a remote agent renders as if it were live
  system truth on the local runtime, because the timestamp envelope's
  import-window posture is buried two clicks deep;
- a comment imported from an external review provider renders with a
  relative-time label that counts from "now", overstating the
  freshness of the imported thread;
- an audit-event row imported from a SIEM export renders identically
  to a live local audit event, hiding that the row's clock is the
  audit trail's, not the runtime's;
- a deletion-timeline row held open by a legal hold renders next to a
  "delete completed" chip somewhere else in the product;
- an incident-timeline that stitches local logs, remote-agent traces,
  and a recovery-snapshot replay renders the replayed segment as if
  the runtime had captured it directly;
- an enterprise-session inspector implies that raw code is being
  captured even when the session is default-ephemeral and no policy
  has admitted capture.

The chronology-context layer fixes this by forcing every row to answer
five questions explicitly:

1. **What is the canonical UTC time?** Always cited through a
   `timestamp_envelope_record` (or its range / segment cousins) on
   `timestamp_envelope_ref`.
2. **What is the rendered timezone basis?** A closed
   `timezone_basis_class` value plus a non-null IANA zone when the
   class names a real zone.
3. **What does the relative-time text count from?** A closed
   `relative_time_basis_class` value (counts from "now" only on live
   rows; imported / replayed rows count from their import window end).
4. **Is the row live system truth or a retained artifact?** A closed
   `current_state_class` value — `live_current_system_state` is
   admissible only on `live_system_truth` and `live_with_bounded_skew`
   evidence; every other source class is a retained artifact.
5. **Where did the evidence come from?** A closed
   `evidence_source_class` value plus an opaque `evidence_source_ref`
   to the authoritative origin (managed surface, remote agent,
   extension host, external audit trail, support bundle, recovery
   snapshot, offline packet).

A row that cannot answer any of these five questions is non-conforming
and the schema rejects it at the boundary.

## 1. Record shape

A `chronology_context_record` carries:

- `chronology_context_id` — stable id quoted by every consuming
  surface that joins on the row.
- `consumer_row_kind` — one of the ten closed kinds (§2).
- `evidence_source_class` — one of the ten closed source classes (§3).
- `current_state_class` — one of the seven closed states (§4).
- `evidence_source_ref` — opaque ref to the authoritative origin
  manifest. Required on every row.
- `timestamp_envelope_ref` plus `timestamp_envelope_record_kind` — the
  bound timestamp-envelope record. Canonical UTC, monotonic ordering,
  skew, partial-order labelling, and import-window posture survive
  verbatim through the envelope.
- `timezone_basis` — closed timezone-basis class plus optional IANA
  zone and reviewable label.
- `relative_time_presentation` — closed relative-time basis,
  absolute-time basis, both-forms-coexist boolean, and an optional
  reviewable relative-time label.
- `imported_artifact_disclosure` — closed imported-artifact-disclosure
  class plus a reviewable disclosure label.
- `import_declared` — declared import window for imported / replayed
  rows. Required on imported / replayed source classes; null on live,
  offline, and synthetic rows.
- `offline_window` — declared offline window for offline rows.
  Required on `offline_local_evidence_packet`; null otherwise.
- `mirror_lag_state_class` — closed mirror-lag class. Only
  `mirrored_managed_surface` rows MAY resolve to a non-default value.
- `deletion_event_extras` — required on `deletion_event_row` rows;
  null otherwise.
- `enterprise_session_extras` — required on `enterprise_session_row`
  rows; null otherwise.
- `linked_evidence` — optional cross-surface back-references.
- `row_summary_label` — short reviewable sentence describing the row.
- `minted_at` — canonical UTC instant the chronology-context record
  was minted.

## 2. Consumer row kinds

The closed `consumer_row_kind` vocabulary is:

| Kind | Required envelope record kinds |
|---|---|
| `run_history_row` | `timestamp_envelope_record` or `time_range_envelope_record` |
| `capture_event_row` | `timestamp_envelope_record` |
| `comment_or_review_thread_row` | `timestamp_envelope_record` |
| `deletion_event_row` | `timestamp_envelope_record` |
| `audit_event_row` | `timestamp_envelope_record` |
| `incident_timeline_row` | `timestamp_envelope_record` plus `partial_chronology_segment_record` per stitched window |
| `enterprise_session_row` | `timestamp_envelope_record` or `time_range_envelope_record` |
| `support_bundle_row` | `timestamp_envelope_record` plus `partial_chronology_segment_record` when the bundle stitches imports |
| `ai_evidence_packet_row` | `timestamp_envelope_record` |
| `chronology_export_preview_row` | `timestamp_envelope_record` plus `partial_chronology_segment_record` per segment |

A surface that mints a private row kind denies with
`consumer_row_kind_unresolved`.

## 3. Evidence source classes

Mirrored verbatim from
[`/artifacts/governance/evidence_source_classes.yaml`](../../artifacts/governance/evidence_source_classes.yaml).
The closed `evidence_source_class` enum is:

| Class | Live or retained | Required relative-time basis |
|---|---|---|
| `live_system_truth` | live | `since_now_live` |
| `live_with_bounded_skew` | live | `since_now_with_skew_label` |
| `mirrored_managed_surface` | retained (managed) | `since_now_with_skew_label` |
| `imported_remote_agent` | retained (imported) | `since_import_window_end` |
| `imported_extension` | retained (imported) | `since_import_window_end` |
| `imported_external_audit_trail` | retained (imported) | `since_import_window_end` |
| `offline_local_evidence_packet` | retained (offline) | `since_offline_window_end` |
| `support_bundle_replay` | retained (replay) | `since_replay_window_end` |
| `recovery_snapshot_replay` | retained (replay) | `since_replay_window_end` |
| `synthetic_fixture_only` | retained (synthetic) | `not_rendered_synthetic_fixture` |

Only `live_system_truth` and `live_with_bounded_skew` admit a live chip
on the rendering surface. Every other class is a retained artifact and
the surface MUST render the matching retained-artifact chip.

## 4. Current-state classes

The closed `current_state_class` enum is:

| Class | Admits live chip |
|---|---|
| `live_current_system_state` | yes |
| `retained_artifact_managed_authoritative` | no |
| `retained_artifact_imported_origin` | no |
| `retained_artifact_offline_local` | no |
| `retained_artifact_support_replay` | no |
| `retained_artifact_recovery_replay` | no |
| `retained_artifact_synthetic_fixture` | no |

A row that resolves `current_state_class` to a retained-artifact value
but renders a live chip denies with `live_state_claimed_on_imported_row`,
`live_state_claimed_on_offline_row`,
`live_state_claimed_on_replay_row`, or
`live_state_claimed_on_synthetic_row` according to the source class.

## 5. Timezone basis

The closed `timezone_basis_class` enum is:

- `canonical_utc` — pinned UTC. Required on
  `chronology_export_preview_row` rows.
- `device_local_iana` — the device's IANA zone. The row MUST also
  carry `timezone_basis.display_time_zone_iana`.
- `deployment_pinned_iana` — an admin-pinned IANA zone. The row MUST
  also carry `timezone_basis.display_time_zone_iana`.
- `observer_skew_labeled` — reserved for rows whose observing client's
  wall clock is unreliable; the display label MUST say so.
- `imported_origin_zone_labeled` — the import origin's zone. The row
  MUST also carry `timezone_basis.display_time_zone_iana` and the
  display label MUST name the origin zone explicitly.
- `not_rendered_canonical_utc_only` — surfaces that suppress timezone
  rendering and pin canonical UTC only.

A row whose `timezone_basis_class` names a real zone but which omits
`display_time_zone_iana` denies under the schema's
`display_time_zone_iana` `if/then` rule.

## 6. Relative-time presentation

The closed `relative_time_basis_class` enum is:

- `since_now_live` — counts from "now" on the rendering surface.
  Permitted only on `live_system_truth` rows.
- `since_now_with_skew_label` — counts from "now" but the surface MUST
  render a skew label. Permitted on `live_with_bounded_skew` and
  `mirrored_managed_surface`.
- `since_import_window_end` — counts from `import_window_end_utc`.
  Required on imported source classes.
- `since_replay_window_end` — counts from `import_window_end_utc` of
  the replay source. Required on support-bundle and recovery-snapshot
  replays.
- `since_offline_window_end` — counts from `offline_window_end_utc`.
  Required on offline rows.
- `since_session_end` — counts from the enterprise-session end. Only
  permitted on `enterprise_session_row` whose
  `session_end_reason_class` is not `no_end_yet_session_open`.
- `not_rendered_partial_order_only` — required when the cited
  envelope's `partial_order_label_class` is `unordered_only_grouping`.
  The surface MUST NOT render relative time on the row.
- `not_rendered_synthetic_fixture` — required on
  `synthetic_fixture_only` rows. The surface MUST NOT render the row
  as live or imported truth outside a test harness.

`both_forms_coexist_on_row = true` is required on consequential and
safety-critical rows so a reader does not have to choose between "when
exactly" and "how long ago."

## 7. Imported-artifact disclosure

The closed `imported_artifact_disclosure_class` enum is:

- `not_imported_live_or_local` — only permitted on
  `live_system_truth`, `live_with_bounded_skew`, or
  `offline_local_evidence_packet` (offline rows MAY resolve to
  `offline_window_disclosed` instead, never to
  `not_imported_live_or_local` if the offline window must be visible
  on the chip).
- `imported_window_disclosed` — required on imported rows whose clock
  is `unsynchronized_import_only` or
  `unsynchronized_partial_order_only`.
- `imported_window_disclosed_with_skew_label` — required on imported
  rows whose clock is `synchronized_with_bounded_skew`.
- `replay_window_disclosed` — required on `support_bundle_replay` and
  `recovery_snapshot_replay`.
- `offline_window_disclosed` — required on
  `offline_local_evidence_packet`.
- `synthetic_fixture_disclosed` — required on `synthetic_fixture_only`.

The `disclosure_label` reviewable sentence MUST name the source class
explicitly. Free-text substitutes ("imported", "offline copy", "live
data") are non-conforming on their own.

## 8. Deletion-event rows

A `consumer_row_kind = deletion_event_row` MUST carry a non-null
`deletion_event_extras` block. The block re-exports the
`stable_delete_state` vocabulary from
[`/docs/governance/privacy_history_and_lifecycle_contract.md`](./privacy_history_and_lifecycle_contract.md):

| State class | Can render as `delete_completed`? |
|---|---|
| `delete_requested` | no |
| `policy_retention` | no |
| `legal_hold` | no |
| `delete_completed` | yes |
| `exported_copy_remains_local` | no |

Held or policy-retained rows that render as `delete_completed` deny
with `deletion_event_held_row_rendered_as_completed`. Every deletion-
event row MUST cite the bound `delete_request_state_record` through
`deletion_event_extras.delete_request_state_ref`.

## 9. Enterprise-session rows

A `consumer_row_kind = enterprise_session_row` MUST carry a non-null
`enterprise_session_extras` block with five typed fields:

- `participant_presence_class` — closed presence vocabulary.
- `elevated_control_grant_class` — closed grant vocabulary.
- `recording_state_class` — re-exported from
  `schemas/collaboration/recorded_artifact_row.schema.json` verbatim.
- `session_end_reason_class` — closed end-reason vocabulary.
- `raw_code_capture_admitted` — boolean. Default false.

`raw_code_capture_admitted = true` MUST cite the admitting policy
bundle through `policy_enabled_capture_ref`. A row that admits raw-code
capture without citing the policy ref denies with
`raw_code_capture_admitted_without_policy_ref`. A row whose
`raw_code_capture_admitted` is false MUST set `policy_enabled_capture_ref`
to null.

The default-ephemeral rule is preserved: an enterprise-session row that
has not been admitted to recording resolves
`recording_state_class` to `default_ephemeral_no_capture` or
`never_admitted_at_this_mode` and `raw_code_capture_admitted` to false.

## 10. Export and support parity

Every export produced by a surface bound to this contract preserves:

- `evidence_source_class` and `current_state_class` verbatim.
- `import_declared`, `offline_window`, and `mirror_lag_state_class`
  blocks as they appeared on the source surface.
- The cited timestamp-envelope's `canonical_export_time_zone_class =
  utc` pin and `iso_8601_utc` absolute-time format on chronology-export
  previews.
- `evidence_source_ref` opaque id (managed surface manifest, remote-
  agent manifest, extension manifest, external-audit-trail manifest,
  support-bundle id, recovery-snapshot id, offline-packet id).
- The `linked_evidence` block — exports MUST NOT drop linked refs
  during projection.

A human-readable export (PDF / printable transcript) MUST render the
chip text from `imported_artifact_disclosure.disclosure_label` and the
absolute timestamp from the cited envelope. A machine-readable export
(JSON / packet) MUST emit the full chronology-context record verbatim.

Re-exports of imported audit trails or external compliance archives
keep their `import_declared` block — the authoritative copy remains at
the import origin and is outside Aureline's delete path.

## 11. Honesty invariants

Every chronology-context row asserts:

- **Live state cannot be claimed on imported / offline / replayed /
  synthetic rows.** A row that resolves `evidence_source_class` to
  anything other than `live_system_truth` or `live_with_bounded_skew`
  cannot resolve `current_state_class` to `live_current_system_state`,
  by schema invariants. Surfaces that try render the live chip anyway
  deny with the matching `live_state_claimed_on_*` reason.
- **Held / retained delete states cannot render as completed.** A
  `deletion_event_row` whose `deletion_event_state_class` is
  `policy_retention` or `legal_hold` MUST NOT render with the
  `delete_completed` chip; the row remains visible and named through
  the bound `delete_request_state_record`.
- **Raw code is not stored unless policy explicitly enables it.**
  Enterprise-session rows default `raw_code_capture_admitted = false`;
  any true value MUST cite the admitting policy bundle.
- **Imports preserve provenance.** Imported / replayed rows MUST cite
  the import origin manifest and MUST NOT erase the import window on
  re-export.
- **Synthetic rows cannot enter release packets, public truth, or
  claim manifests.** A `synthetic_fixture_only` row that ships in any
  release-bearing surface denies with
  `synthetic_fixture_in_release_packet`.

## 12. Forbidden user-facing phrases

The following phrases describe an evidence row's source without naming
the closed vocabulary above and are non-conforming on any surface
governed by this contract. Drift scans MUST reject them:

- **"live data"** — only permitted as the rendered text of a live
  chip on rows whose `evidence_source_class` is `live_system_truth` or
  `live_with_bounded_skew`.
- **"real-time"** — only permitted as the rendered text of a live
  chip on rows whose `evidence_source_class` is `live_system_truth`.
- **"imported"** — only permitted as the rendered text of an
  imported chip on rows whose `evidence_source_class` is in the
  imported family.
- **"offline copy"** — only permitted as the rendered text of an
  offline chip on rows whose `evidence_source_class` is
  `offline_local_evidence_packet`.
- **"source unknown"** — never permitted; surfaces MUST resolve
  `evidence_source_class`.
- **"from cache"** — not a substitute for
  `retained_artifact_managed_authoritative` or
  `retained_artifact_imported_origin`; never permitted on its own.

The forbidden time phrases from
[`./time_semantics.md`](./time_semantics.md#forbidden-user-facing-phrases)
also apply unchanged.

## 13. Denial reasons

The schema's `chronology_context_denial_reason` enum is mirrored from
[`/artifacts/governance/evidence_source_classes.yaml`](../../artifacts/governance/evidence_source_classes.yaml#denial_reasons).
A row that fails the contract MUST emit one of these stable labels
rather than admit silent drift:

- `evidence_source_class_unresolved`
- `current_state_class_unresolved`
- `consumer_row_kind_unresolved`
- `timestamp_envelope_ref_missing`
- `live_state_claimed_on_imported_row`
- `live_state_claimed_on_offline_row`
- `live_state_claimed_on_replay_row`
- `live_state_claimed_on_synthetic_row`
- `imported_row_missing_import_declared_block`
- `imported_row_missing_evidence_source_ref`
- `support_bundle_replay_missing_support_bundle_ref`
- `recovery_snapshot_replay_missing_recovery_manifest_ref`
- `synthetic_fixture_in_release_packet`
- `enterprise_session_field_missing`
- `raw_code_capture_admitted_without_policy_ref`
- `deletion_event_held_row_rendered_as_completed`
- `chronology_export_preview_not_utc`
- `relative_time_rendered_on_unordered_only_grouping_row`
- `skew_label_not_rendered_on_unsynchronized_row`
- `forbidden_phrase_emitted`
- `chronology_context_schema_version_lagging`

## 14. Versioning

`chronology_context_schema_version` bumps only on breaking payload
changes. Adding a new `evidence_source_class`, `current_state_class`,
`consumer_row_kind`, `timezone_basis_class`,
`relative_time_basis_class`, `imported_artifact_disclosure_class`,
`mirror_lag_state_class`, `participant_presence_class`,
`elevated_control_grant_class`, `recording_state_class`,
`session_end_reason_class`, `deletion_event_state_class`, or
`chronology_context_denial_reason` value is additive-minor and requires
a decision row under
[`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml).
Removing or repurposing an existing value is breaking and requires a
new decision row under the register.

## 15. Worked fixtures

Five worked fixtures under
[`/fixtures/governance/chronology_context_cases/`](../../fixtures/governance/chronology_context_cases/)
exercise the contract across the representative consumer-row kinds:

- `live_event_run_history_local.yaml` — a build-run history row
  recorded by the observing runtime on a synchronized clock.
- `imported_remote_agent_trace.yaml` — a capture-event trace row
  imported from a remote agent under
  `unsynchronized_partial_order_only` clock posture.
- `offline_local_evidence_packet.yaml` — an audit-event row captured
  while the observing runtime was disconnected from any managed
  surface.
- `deletion_timeline_legal_hold.yaml` — a deletion-event row held
  open by a legal hold; the row renders next to a `legal_hold` chip
  and MUST NOT collapse into `delete_completed`.
- `enterprise_session_handoff.yaml` — an enterprise-session row
  covering host handoff under default-ephemeral recording with no
  raw-code capture.

Every fixture embeds `__fixture__` metadata naming the contract
sections it exercises and the vocabulary values it carries.
