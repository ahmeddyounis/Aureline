# Cross-surface time semantics, clock-source, and timezone/skew contract

This document is the normative companion to the machine-readable
timestamp-envelope schema. It binds logs, metrics, traces, task
events, incident timelines, history rows, chronology exports, support
bundles, AI evidence packets, and publication proofs to **one**
explainable time model. Every surface that answers "when did this
happen" MUST cite the timestamp-envelope contract and MUST use the
vocabulary frozen here.

Companion artifacts:

- [`/schemas/governance/timestamp_envelope.schema.json`](../../schemas/governance/timestamp_envelope.schema.json)
  — the cross-tool boundary schema. Tooling reads that file; the
  narrative below describes the same rules.
- [`/fixtures/governance/time_examples/`](../../fixtures/governance/time_examples/)
  — worked examples showing the envelope on local-only events,
  provider / imported traces, unsynchronized remote-agent history,
  and partial chronology exports.
- [`/schemas/ux/history_row.schema.json`](../../schemas/ux/history_row.schema.json)
  — the chronology-primitive contract that embeds a
  `monotonic_timestamp` and a `display_time_policy`. This vocabulary
  re-uses those fields verbatim; it does not redefine them.
- [`/schemas/traces/trace_event.schema.json`](../../schemas/traces/trace_event.schema.json)
  — the protected-trace boundary whose tick-based timing is wrapped
  by this envelope for cross-surface joins.
- [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — the support-bundle carrier whose timestamps cite this envelope
  before they are exported.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — the control-artifact index that wires the contract, the schema,
  and the worked fixtures into governed ownership.

Out of scope at this revision: OS time synchronization, NTP peer
configuration, and any external observability infrastructure (APM,
log aggregator, tracing backend). This contract governs the envelope
those systems embed; it does not govern how those systems are
deployed.

## Why this contract exists

Before this contract, each surface minted its own time fields:

- The editor chronology used `monotonic_timestamp` plus a
  `display_time_policy` to separate canonical storage from local
  rendering.
- Trace events used `started_tick` / `finished_tick` integers with a
  synthetic-or-nanosecond-origin comment attached.
- Evidence packets used `captured_at` as a single RFC 3339 string
  with no skew, no clock source, and no synchronization state.
- Task-event envelopes carried a raw `timestamp` field with no
  declared origin.
- Imported remote-agent histories silently adopted the importing
  runtime's wall clock, so ordering against local rows was unsafe.

The drifts compounded. A support bundle that stitched a chronology
export together from local rows plus a remote-agent import plus a
replayed recovery snapshot had three different time semantics inside
the same payload and no way to say so. A reader could not tell
whether two rows one second apart were truly ordered or only
coincidentally rendered that way.

The timestamp envelope fixes this by forcing every row to answer six
questions explicitly:

1. **What is the canonical storage time?** Always a UTC wall-clock
   timestamp in ISO 8601 / RFC 3339 form.
2. **What is the monotonic reference?** A non-negative nanosecond
   reading from the observing runtime's monotonic clock, tagged with
   a `monotonic_epoch_ref` that a consumer can use to detect
   restarts.
3. **What produced the time?** A closed `clock_source_class` value.
4. **Is the clock synchronized?** A closed `clock_sync_state_class`
   value plus a measured skew block when the state is
   `synchronized_with_bounded_skew`.
5. **What ordering guarantee does the row carry?** A closed
   `partial_order_label_class` value.
6. **Was the row imported, and if so, from where and over what
   window?** A closed `import_origin_class` value and a declared
   import window.

A row that cannot answer any of these six questions is non-conforming.

## The envelope shape

One shared schema, three record kinds, all sharing the six answers.

### `timestamp_envelope_record`

The canonical single-point carrier of "when this happened." Every
log line, metric point, trace event, task event, history row,
activity badge, and evidence row that carries a time value embeds
exactly one of these. Required fields:

- `canonical_utc_wall_clock` — ISO 8601 / RFC 3339 UTC timestamp.
  MUST use the `Z` suffix (or `+00:00`). MUST be emitted in UTC
  regardless of the observer's local time zone.
- `monotonic_reference_nanoseconds` — non-negative integer reading
  from the observing runtime's monotonic clock. Meaningful only as a
  difference from another reading taken on the same runtime within
  the same `monotonic_epoch_ref`.
- `monotonic_epoch_ref` — opaque id naming the monotonic-clock epoch
  the reading belongs to. A monotonic-clock reset (process restart,
  runtime reattach, host reboot, remote-agent reconnect, VM
  migration) mints a new ref so consumers see that two readings
  from different epochs are not directly subtractable.
- `clock_source_class` — one of nine closed values:
  `local_monotonic_reference_clock`, `local_system_wall_clock`,
  `ntp_synchronized_wall_clock`,
  `trusted_platform_authenticated_clock`,
  `remote_agent_reported_clock`, `extension_reported_clock`,
  `imported_external_audit_trail_clock`,
  `reconstructed_from_backup_clock`, `synthetic_fixture_tick_source`.
- `clock_sync_state_class` — one of six closed values:
  `synchronized`, `synchronized_with_bounded_skew`, `unsynchronized`,
  `unsynchronized_partial_order_only`, `unsynchronized_import_only`,
  `synthesized_for_fixture`.
- `partial_order_label_class` — one of five closed values:
  `fully_ordered_monotonic`, `partially_ordered_same_runtime`,
  `partially_ordered_causal_only`, `partially_ordered_import_window`,
  `unordered_only_grouping`.
- `import_origin_class` — one of seven closed values:
  `not_imported`, `remote_agent_export`, `extension_export`,
  `external_audit_trail`, `companion_surface_export`,
  `support_bundle_replay`, `recovery_snapshot_replay`.
- `canonical_export_time_zone_class` — closed-world pin, always
  `utc`. Declared explicitly so an exporter that tries to widen
  export to a local time zone is caught by schema validation rather
  than silently emitted.

Optional fields:

- `skew` — measured skew attestation. Required (non-null) when
  `clock_sync_state_class` is `synchronized_with_bounded_skew`.
  Permitted on other sync states. Carries
  `skew_direction_class` (`none`, `ahead_of_canonical`,
  `behind_canonical`, `direction_unknown`),
  `skew_magnitude_duration`, `skew_tolerance_duration`,
  `skew_last_verified_at`, and an optional privacy-safe
  `skew_note_label`.
- `import_declared` — required (non-null) when `clock_source_class`
  is any imported-family value. MUST be null when
  `import_origin_class` is `not_imported`. Carries the declared
  import window (`import_window_start_utc`,
  `import_window_end_utc`) and an opaque `import_source_ref`.
- `local_render` — optional per-surface rendering policy. Null when
  the envelope is stored but not yet rendered. When non-null it
  carries the surface's committed
  `absolute_time_format_class`, `relative_time_format_class`,
  `time_zone_class`, `display_time_zone_iana` (when the time-zone
  class requires it), `both_forms_coexist_on_row`, and
  `skew_label_rendered`.

### `time_range_envelope_record`

A paired start-and-end envelope with an authoritative monotonic
duration. Trace spans, task-event start/finish pairs, and benchmark
spike timings carry this record. Required fields:

- `range_start_envelope` — a full `timestamp_envelope_record`.
- `range_end_envelope` — a full `timestamp_envelope_record`.
- `monotonic_duration_nanoseconds` — non-negative integer duration.
- `range_duration_authoritative_source_class` — one of four closed
  values: `monotonic_same_epoch` (the default, required when both
  endpoints share a `monotonic_epoch_ref`),
  `utc_wall_clock_crossed_epoch` (required when the endpoints are on
  different monotonic epochs on the same runtime),
  `utc_wall_clock_import_only` (required when either endpoint is
  imported), `synthesized_for_fixture`.

### `partial_chronology_segment_record`

One declared window of canonical UTC time, with its completeness
class, its import origin, and every declared gap inside the window.
Imported or partial histories carry this record so a consumer can
render "incomplete" without pretending to be complete. Required
fields:

- `segment_window_start_utc` / `segment_window_end_utc` — UTC
  timestamps bounding the segment.
- `segment_completeness_class` — one of five closed values:
  `complete_no_gaps` (permitted only when `declared_gaps` is empty
  AND the segment's observing runtime attests to continuous
  capture), `complete_with_declared_gaps` (covers the window but
  carries at least one declared gap),
  `known_incomplete_partial_import`,
  `known_incomplete_policy_redacted`,
  `known_incomplete_source_unavailable`.
- `segment_import_origin_class` — same closed vocabulary as the
  single-point envelope.
- `segment_import_declared` — required (non-null) when the origin
  class is anything other than `not_imported`.
- `declared_gaps` — zero or more `declared_gap_record` entries.
  Present as an empty array when there are none, so the renderer
  knows the gap list was considered explicitly.
- `segment_completeness_label` — short privacy-safe label the
  surface MUST render. "Complete capture" is permitted only when
  `segment_completeness_class` is `complete_no_gaps`.

## Rules

The following rules are normative. A surface that violates one of
them is non-conforming and its payload MUST fail schema validation
at the boundary.

### Canonical storage

1. Every envelope MUST carry `canonical_utc_wall_clock` in UTC with
   the `Z` suffix (or `+00:00`). Local time-zone offsets MUST NOT
   appear in the canonical field. Local rendering lives only in
   `local_render`.
2. Every envelope MUST carry `monotonic_reference_nanoseconds` and
   `monotonic_epoch_ref`. A surface that cannot attest to a
   monotonic reading — for example an imported audit trail — MUST
   still carry a monotonic reading on the **importing runtime** at
   the moment of import, so that the envelope remains comparable
   within the import window.
3. `canonical_export_time_zone_class` is closed to `utc`. The schema
   rejects any other value. Export previews inherit this pin; a
   surface that tries to emit a local-zone export is blocked.

### Clock source and synchronization

4. `clock_source_class` MUST NOT be omitted or aliased. A surface
   that does not know the source MUST pin
   `imported_external_audit_trail_clock` and declare the import
   window rather than silently defaulting to
   `local_system_wall_clock`.
5. `clock_sync_state_class = synchronized_with_bounded_skew` MUST
   carry a non-null `skew` block with a non-null
   `skew_tolerance_duration` and a non-null `skew_last_verified_at`.
6. `clock_sync_state_class = unsynchronized_partial_order_only` MUST
   pin `partial_order_label_class` to one of
   `partially_ordered_same_runtime`,
   `partially_ordered_causal_only`,
   `partially_ordered_import_window`, or
   `unordered_only_grouping`. A row flagged partial-order MUST NOT
   claim `fully_ordered_monotonic`.
7. `clock_source_class = synthetic_fixture_tick_source` MUST pin
   `clock_sync_state_class` to `synthesized_for_fixture`. Harness
   fixtures MUST NOT be presented as wall-clock evidence to a
   reviewer.

### Import origin and windows

8. `clock_source_class` in the imported-family
   (`remote_agent_reported_clock`, `extension_reported_clock`,
   `imported_external_audit_trail_clock`,
   `reconstructed_from_backup_clock`) MUST carry a non-null
   `import_declared` block.
9. `import_origin_class = not_imported` MUST NOT carry an
   `import_declared` block.
10. `import_source_ref` is opaque. Raw hostnames, raw NTP peer
    addresses, raw IP addresses, raw device serial numbers, and raw
    user clock-offset material MUST NOT appear in the envelope or
    in any `short_label_text` field.

### Partial order and ordering guarantees

11. A consumer joining two envelopes MAY assert wall-clock ordering
    only when both envelopes carry `fully_ordered_monotonic` and
    share the same `monotonic_epoch_ref`. Otherwise the consumer
    MUST treat the ordering as partial and render a partial-order
    label.
12. Causal ordering via `linked_canonical_event_id_ref`,
    `linked_history_row_id_ref`, `linked_trace_span_id_ref`, or
    `linked_task_event_id_ref` is preserved across envelopes even
    when clock ordering is not. A consumer that drops a linked
    reference during projection is non-conforming.

### Relative-time and absolute-time display

13. `local_render.absolute_time_format_class = iso_8601_utc` and
    `time_zone_class = utc` are required on every
    `render_surface_class = chronology_export_preview` envelope so
    the exported payload is independent of the exporter's local
    time zone.
14. `local_render.time_zone_class` in `device_local_iana` or
    `deployment_pinned_iana` MUST carry a non-null
    `display_time_zone_iana`.
15. `local_render.skew_label_rendered` MUST be true when the
    envelope's `clock_sync_state_class` is
    `synchronized_with_bounded_skew`, `unsynchronized`,
    `unsynchronized_partial_order_only`, or
    `unsynchronized_import_only`. A surface that hides skew on a
    row whose clock is unsynchronized is non-conforming.
16. Relative-time text (`since_now_short`, `since_now_long`) is a
    presentation of the difference between "now" on the rendering
    surface and the envelope's `canonical_utc_wall_clock`. A
    surface MUST NOT present relative time for an envelope whose
    `partial_order_label_class` is `unordered_only_grouping`; only
    absolute time is permitted in that case, and the surface MUST
    render a partial-order label.
17. `both_forms_coexist_on_row = true` is required on consequential
    and safety-critical rows so a reader does not have to choose
    between "when exactly" and "how long ago."

### Time-range envelopes

18. `range_duration_authoritative_source_class = monotonic_same_epoch`
    requires that `range_start_envelope.monotonic_epoch_ref` equal
    `range_end_envelope.monotonic_epoch_ref`. When the two epochs
    differ the surface MUST pin `utc_wall_clock_crossed_epoch`.
19. `monotonic_duration_nanoseconds` MUST be non-negative. A
    surface that computes a negative duration has crossed a
    monotonic epoch unknowingly and MUST re-source the duration
    from the canonical wall-clock difference with
    `utc_wall_clock_crossed_epoch`.
20. A trace span, task-event pair, or benchmark spike timing that
    straddles an import boundary (one endpoint local, one endpoint
    imported) MUST pin `utc_wall_clock_import_only`.

### Partial chronology segments

21. `segment_completeness_class = complete_no_gaps` is permitted
    only when `declared_gaps` is empty **and** the segment's
    observing runtime attests to continuous capture over the
    segment window. A surface that stitches across an unseen gap
    and marks the result complete is non-conforming (denial_reason
    = `silent_gap_stitch`).
22. `segment_completeness_class = complete_with_declared_gaps` MUST
    carry at least one `declared_gap_record`. Every gap MUST name
    exactly one closed `gap_reason_class` and a short privacy-safe
    `gap_reason_label`. Silent stitching across a declared gap is
    non-conforming.
23. `segment_completeness_class` in the
    `known_incomplete_*` family MUST NOT be rendered with a
    "Complete capture" label. The `segment_completeness_label`
    text is user-visible and a linter MUST reject completeness
    labels that misrepresent the class.
24. Imported segments
    (`segment_import_origin_class != not_imported`) MUST carry a
    non-null `segment_import_declared` block whose window contains
    the segment window.

## Denial reasons (reserved labels)

Schema-rejecting conditions are enumerated here so tooling can emit
a stable `denial_reason` when an envelope is rejected at the
boundary. Every reserved label is lowercase, snake_case, and stable:

- `canonical_utc_wall_clock_missing_or_non_utc`
- `monotonic_reference_missing`
- `monotonic_epoch_ref_missing`
- `clock_source_class_missing_or_aliased`
- `clock_sync_state_class_missing_or_aliased`
- `synchronized_with_bounded_skew_missing_skew_block`
- `unsynchronized_partial_order_only_claims_full_order`
- `synthetic_fixture_presented_as_wall_clock_evidence`
- `imported_source_missing_import_declared_block`
- `not_imported_carries_import_declared_block`
- `raw_hostname_or_peer_leak`
- `raw_device_identifier_leak`
- `raw_clock_offset_leak`
- `wall_clock_ordering_asserted_across_monotonic_epoch`
- `linked_reference_dropped_during_projection`
- `chronology_export_preview_not_utc`
- `display_time_zone_iana_missing_for_named_zone_class`
- `skew_label_not_rendered_on_unsynchronized_row`
- `relative_time_rendered_on_unordered_only_grouping_row`
- `both_forms_missing_on_consequential_or_safety_critical_row`
- `monotonic_same_epoch_declared_across_epoch_boundary`
- `negative_monotonic_duration_not_resolved_to_wall_clock`
- `import_straddle_not_declared_import_only`
- `segment_marked_complete_with_declared_gaps`
- `segment_marked_complete_without_continuous_capture_attestation`
- `silent_gap_stitch`
- `known_incomplete_segment_labeled_complete`
- `imported_segment_missing_segment_import_declared`

## Forbidden user-facing phrases

The following phrases describe time without naming any of the six
answers above and are non-conforming on any surface governed by
this contract. Drift scans MUST reject them:

- "just now" (as a substitute for a relative-time class; permitted
  only as the rendered text of `since_now_short` on a row whose
  clock is synchronized)
- "a moment ago" (same constraint as above)
- "recently"
- "in real time" (as a synonym for "live"; permitted only when the
  surface's envelope is `fully_ordered_monotonic` and
  `clock_sync_state_class` is `synchronized` or
  `synchronized_with_bounded_skew`)
- "as of now"
- "last known" (as a substitute for a declared-gap label;
  permitted only when the row carries a
  `declared_gap_record` whose `gap_reason_class` is
  `source_unavailable_temporarily`)
- "seen around" (as a substitute for a skew label)

## Cross-surface consumer mapping

One row per retained-time-carrier family fixes which record kinds
of the envelope it MUST cite.

| Consumer family | Record kind citations |
| --- | --- |
| Log line (`log_line`) | `timestamp_envelope_record` |
| Metric point (`metric_point`) | `timestamp_envelope_record` |
| Trace span (`trace_span`) | `time_range_envelope_record` wrapping two `timestamp_envelope_record` endpoints |
| Task-event row (`task_event_row`) | `timestamp_envelope_record` for discrete events; `time_range_envelope_record` for start/finish pairs |
| Incident timeline row (`incident_timeline_row`) | `timestamp_envelope_record` per row plus one `partial_chronology_segment_record` per window reconstructed from imports |
| History-lane row (`history_lane_row`) | `timestamp_envelope_record` (wraps the row's existing `monotonic_timestamp` and `display_time_policy`) |
| Narrative summary card (`narrative_summary_card`) | `timestamp_envelope_record` plus cited history-row envelopes |
| Chronology export preview (`chronology_export_preview`) | One `partial_chronology_segment_record` per segment plus one `timestamp_envelope_record` per exported row |
| Activity-center item (`activity_center_item`) | `timestamp_envelope_record` |
| Support-bundle row (`support_bundle_row`) | `timestamp_envelope_record` per row plus a `partial_chronology_segment_record` when the bundle stitches imported or replayed content |
| AI-evidence packet row (`ai_evidence_packet_row`) | `timestamp_envelope_record` per row |
| Publication proof row (`publication_proof_row`) | `timestamp_envelope_record` pinned to `utc` |

A row that minted a private time field instead of citing this
envelope is non-conforming.

## Versioning

`timestamp_envelope_schema_version` bumps only on breaking payload
changes. Adding a new `clock_source_class`, `clock_sync_state_class`,
`partial_order_label_class`, `skew_direction_class`,
`import_origin_class`, `gap_reason_class`,
`absolute_time_format_class`, `relative_time_format_class`,
`time_zone_class`, or `render_surface_class` value is additive-minor
and requires a decision row under
[`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml).
Removing or repurposing an existing value is breaking and requires a
new decision row under the register.

## Worked fixtures

Seven worked fixtures under
[`/fixtures/governance/time_examples/`](../../fixtures/governance/time_examples/)
exercise the envelope across the representative consumer families:

- `local_event_fully_synchronized.json` — a routine local history
  row recorded by the observing runtime on a synchronized clock.
- `local_event_synchronized_with_bounded_skew.json` — the same row
  recorded on a clock with a measured skew bound.
- `trace_span_local_monotonic.json` — a `time_range_envelope_record`
  for a trace span whose start and end share one monotonic epoch.
- `trace_span_crossed_monotonic_epoch.json` — a trace span whose
  endpoints straddle a monotonic epoch reset and source its
  duration from the canonical wall-clock difference.
- `imported_remote_agent_unsynchronized.json` — an imported
  remote-agent row whose clock is unsynchronized-partial-order-only
  and whose partial-order label is
  `partially_ordered_import_window`.
- `imported_external_audit_trail_with_skew.json` — an imported
  external-audit-trail row with a direction-unknown skew
  attestation.
- `partial_chronology_export_with_declared_gap.json` — a chronology
  export segment whose completeness is
  `complete_with_declared_gaps` and which declares one gap under
  `remote_runtime_disconnected`.

Every fixture embeds `__fixture__` metadata naming the contract
sections it exercises and the vocabulary values it carries.
