# Chronology-row contract: event/history rows, timeline groups, narrative summary cards, and export previews

This document is the **cross-surface chronology contract** for
Aureline. It exists so one history-row primitive, one timeline-
group rule, one narrative-summary-card shape, and one
chronology-export-preview shape serve every surface that renders
"what happened, when, to what, by whom, and how you reach the full
truth" — AI evidence timelines, task/run event histories, policy-
block records, provider-sync audit, publish/revoke lifelines,
restore/recovery flows, and support/export previews — without
minting parallel event models or per-subsystem field names.

The contract is normative. Where this document disagrees with the
UI / UX Spec sections it quotes, the source spec wins and this
document MUST be updated in the same change. Where this document
disagrees with a downstream surface's private chronology, history,
or export story, this document wins and the surface is non-
conforming.

The companion artifacts are:

- [`/schemas/ux/history_row.schema.json`](../../schemas/ux/history_row.schema.json)
  — boundary schema every non-owning surface reads. Freezes the
  `history_row_record`, `timeline_group_record`,
  `narrative_summary_card_record`, and
  `chronology_export_preview_record` shapes and the closed
  vocabularies this document binds.
- [`/fixtures/ux/history_examples/`](../../fixtures/ux/history_examples/)
  — worked examples covering AI evidence, task/run events, policy
  blocks, provider sync, publish/revoke, restore/recovery, and
  support/export previews. Every example renders from the same
  schema without renaming core fields per subsystem.

This contract rides alongside — it does **not** re-mint — the
vocabularies frozen in:

- [`/docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  and [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  — canonical event id, delivery surface class, dismissal verb
  distinctions (acknowledge / resolve / dismiss / snooze / mute /
  suppress), linkback-target vocabulary, and durable-linkback
  rule. A `history_row_record` whose delivery had a cross-surface
  trail SHOULD carry `linked_canonical_event_id_ref` so the row
  shares lineage with its envelope record.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — source subsystem, attention class, interruptibility tier,
  redaction class. Re-exported here verbatim.
- [`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md)
  — entry / restore / audit truth for recovery rows. A
  `recovery_snapshot_row` row MUST cite its
  entry_restore_record by ref.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — redaction pass runs before bytes reach any persistent or
  exportable sink, including chronology export previews.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
  — admin policy MAY narrow; policy MAY NOT silently widen. A
  chronology export that widens redaction relative to the live
  surface is non-conforming.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, `redaction_class` are
  re-exported without modification.

## Who reads this document

- **Editor, review-and-diff, AI apply, task runner, terminal,
  provider, policy, publication, recovery, and support surfaces**
  — to emit chronology rows with the same verbs and field names
  rather than minting per-surface history models.
- **Activity center, history lane, detail sheet, and export
  preview surfaces** — to render rows, timeline groups, narrative
  cards, and export previews over the shared schema without
  renaming core fields per upstream.
- **Support, admin, and release evidence pipelines** — to consume
  chronology exports with a single structured vocabulary whose
  object identity survives round-tripping.

## What problem this contract solves

Chronology is a product primitive, not a per-surface logging
habit. Before this contract, every new subsystem was tempted to
mint its own private event list: AI apply had a trace of
suggestions; the task runner had a run log; the provider broker
had a sync report; the publisher had a release log; the recovery
pipeline had restore attempts; support had an export audit — each
with private field names for the timestamp, the actor, what
happened, the outcome, and where the user goes to see the full
truth. The cost is borne by the user: the same event arrives twice
with different labels; the export preview renames half the fields;
high-consequence rows are silently truncated because some history
surface decided "that's a detail for later."

This contract forbids all of the above by declaring one row shape
with stable verbs, one grouping rule set, one narrative-card
shape, and one export-preview shape — every surface reads and
writes the same fields.

## 1. Four chronology primitives

### 1.1 `history_row_record`

One structured row per **(canonical object, action_verb,
monotonic_timestamp)** triple. Minted by the subsystem that owns
the canonical object. A surface that mints two rows with the same
triple but different ids is non-conforming
(`denial_reason = duplicate_history_row_for_canonical_action`).

The history row is the **atom** of the chronology: every timeline
group, narrative summary card, and export preview references rows
by id. A row is **never erased** by a group or card overlay — the
row stays reachable by id even when a higher-level surface
collapses it.

### 1.2 `timeline_group_record`

One structural overlay over one or more history rows that share a
grouping rule (same canonical object, same grouped-burst id, same
scope within a window, same actor within a window, same policy
epoch, same execution context, same linked canonical event id, or
an explicit user pin). A group **never replaces** its members — it
adds a header over them. A group containing any `consequential` or
`safety_critical` member MUST set `allow_routine_row_truncation =
false`; setting true on such a group is non-conforming
(`denial_reason = group_truncates_consequential_member`).

### 1.3 `narrative_summary_card_record`

One human-readable (first-party synthesized, imported, or AI-
generated) summary over one or more cited history rows. A
narrative card that cites no rows is non-conforming
(`denial_reason = narrative_card_without_cited_rows`). An
`ai_generated_summary` card MUST cite every row whose content it
summarized AND MUST disclose its confidence class; a summary
without disclosure is non-conforming
(`denial_reason = ai_narrative_without_confidence_disclosure`).

### 1.4 `chronology_export_preview_record`

One in-product preview of a chronology export (support bundle, AI-
evidence packet, policy-audit export, publication proof, recovery
proof, or admin audit). The preview MUST:

- pin `display_time_policy.time_zone_class = utc` and
  `absolute_time_format_class = iso_8601_utc` so the exported
  payload is independent of the exporter's local time zone;
- preserve `canonical_object_target_ref`, `action_verb`,
  `outcome_class`, `importance_class`, `provenance_badge_class`,
  and `scope_object_kind` on every included row **identical to
  the live surface**; and
- disclose the `excluded_row_count_by_reason` accounting rather
  than silently dropping rows.

An export preview that renames any of those fields relative to
the live surface is non-conforming
(`denial_reason = export_preview_field_renamed_from_live_surface`);
an export preview that silently drops rows is non-conforming
(`denial_reason = export_preview_silently_dropped_rows`); an
export preview that widens redaction relative to the live
surface is non-conforming
(`denial_reason = export_widened_redaction_relative_to_live_surface`).

## 2. Shared required fields

Every `history_row_record` carries the same named fields; a
subsystem that substitutes a private alias for any of these is
non-conforming. The fields are:

| Field | Vocabulary / shape | What it names |
| --- | --- | --- |
| `history_row_id` | `opaque_id` | Stable id for this row. |
| `canonical_object_target_ref` | `opaque_id` | The canonical object this row is about. |
| `source_subsystem` | frozen 25-value enum (re-export) | Owning subsystem. |
| `scope_object_kind` | frozen 10-value enum | What kind of object this is (AI evidence / task run / policy decision / provider grant / publication / recovery snapshot / support bundle / workspace object / extension lifecycle / activity stream). |
| `actor_kind` | frozen 7-value enum | user / system / AI agent / remote service / admin policy / extension / unknown. |
| `actor_identity_ref` | `opaque_id` (nullable only for `system_actor` or `unknown_actor`) | Opaque actor id. |
| `action_verb` | frozen 25-value enum | What happened (see §3). |
| `outcome_class` | frozen 10-value enum | pending / in_progress / succeeded / failed / cancelled / denied / held / superseded / recovered / observed_only. |
| `importance_class` | routine / consequential / safety_critical | Row importance for detail-link rules (see §4). |
| `provenance_badge_class` | frozen 9-value enum | How this row came to be (first-party direct, first-party synthesized, extension reported, remote agent reported, companion reported, AI-assisted, AI-generated summary, imported from external audit, reconstructed from backup). |
| `redaction_class` | frozen 4-value enum (re-export, ADR-0011) | Payload redaction posture. |
| `client_scope` | frozen 6-value enum (re-export, ADR-0011) | Which client scope observed this row. |
| `monotonic_timestamp` | ISO 8601 UTC (monotonic clock) | Canonical storage timestamp. Never tz-local. |
| `display_time_policy` | structured object (see §5) | Absolute, relative, and time-zone presentation rules. |
| `summary_label` | short privacy-safe label (max 200) | Row's one-line text. |
| `detail_link` | structured object (see §4) | Where the user goes for the full truth. |
| `policy_context` | `{policy_epoch, trust_state, execution_context_id?}` | ADR-0001 / ADR-0009 context at mint time. |

Optional back-references survive the live surface → history lane
→ export preview round-trip:

- `linked_canonical_event_id_ref` — notification-delivery lineage id.
- `linked_evidence_packet_id_ref` — evidence packet id on AI / publication / support rows.
- `linked_task_event_id_ref` — task event envelope id on task/run rows.
- `linked_recovery_snapshot_id_ref` — recovery snapshot id on recovery rows.
- `supersedes_history_row_id_ref` — required (non-null) when `action_verb = superseded`; the prior row MUST still be reachable.

## 3. Stable verbs

The `action_verb` vocabulary is closed at 25 values:

```
started        progressed     succeeded       failed          cancelled
blocked        unblocked      held            released        granted
narrowed       widened        revoked         presented       superseded
proposed       accepted       rejected        restored        recovered
published      unpublished    exported        acknowledged    dismissed
```

A subsystem that needs a verb outside this set MUST raise an
additive-minor change (schema version bump) rather than inventing
a private alias on one surface. Re-using an existing verb with a
different meaning is a breaking change and requires a new
decision row.

Verb distinctions preserved from the notification-delivery
contract:

- `acknowledged` removes the badge but not the row.
- `dismissed` removes a transient surface without mutating the row.
- `succeeded` / `failed` / `cancelled` record outcome and MAY
  co-occur with `outcome_class` values of the same name; the row
  is the canonical truth, the envelope is a derivation of it.
- `superseded` MUST carry `supersedes_history_row_id_ref`; the
  prior row MUST remain reachable. An erased predecessor is non-
  conforming (`denial_reason = dismissal_erased_durable_history`
  on the event-lineage audit stream).
- `widened` is reserved for rebaseline events; a widening row
  without an accompanying policy-decision row is non-conforming
  (`denial_reason = widening_without_policy_basis`).

## 4. Detail link and high-importance disclosure rule

Every row carries exactly one `detail_link`. Generic home-screen,
search-results, and external-URL fallbacks are forbidden
(`denial_reason = detail_link_silently_downgraded`).

The `detail_link_kind` vocabulary is:

```
canonical_object_target_exact   evidence_packet_row
review_sheet                    diff_view
durable_job_row_exact           activity_center_item
history_lane_row                export_preview_row
full_detail_sheet               audit_trail_only
not_available_linkback_lost
```

**High-importance disclosure rule.** Rows whose `importance_class`
is `consequential` or `safety_critical` MUST carry a
`detail_link.kind` from the non-truncating durable set:

```
canonical_object_target_exact   evidence_packet_row
review_sheet                    diff_view
durable_job_row_exact           full_detail_sheet
```

and MUST set `detail_link.is_durable = true`. Setting
`audit_trail_only`, `not_available_linkback_lost`,
`activity_center_item`, `history_lane_row`, `export_preview_row`,
or a non-durable link on a consequential or safety-critical row
is non-conforming
(`denial_reason = high_importance_row_truncated_without_detail_link`).
This rule exists so consequential truth is never hidden behind
truncation: a user reading a policy block, a publish / revoke
row, a safety-critical recovery row, or an AI-evidence row whose
confidence was later revised can always reach the full-detail
sheet.

`audit_trail_only` and `not_available_linkback_lost` remain
permitted on **routine** rows whose exact durable target was
policy-blocked, redacted, or permanently lost; both require an
`unavailability_reason_label` so the row names why it cannot link
to the exact object.

## 5. Time: relative, absolute, and time zone

The canonical storage timestamp on every row is
`monotonic_timestamp` — ISO 8601 UTC from a monotonic clock
source. Wall-clock presentation is a separate axis carried on
`display_time_policy`:

| Field | Vocabulary | What it names |
| --- | --- | --- |
| `absolute_time_format_class` | `iso_8601_utc`, `iso_8601_local_with_offset`, `local_date_time_short`, `local_date_time_long`, `local_time_only_when_today` | Absolute rendering. |
| `relative_time_format_class` | `since_now_short`, `since_now_long`, `since_last_observed`, `wall_clock_delta_short`, `not_rendered` | Relative rendering. |
| `time_zone_class` | `utc`, `device_local_iana`, `deployment_pinned_iana`, `observer_skew_labeled` | Time-zone posture. |
| `display_time_zone_iana` | IANA tz name (when applicable) | IANA zone. |
| `both_forms_coexist_on_row` | boolean | Whether absolute and relative coexist on the row (hover, focus, inline). |

Presentation rules:

- **Storage is always UTC monotonic.** Relative and absolute
  labels are derived; the row storage never renames the timestamp
  to local time.
- **Consequential and safety-critical rows MUST set
  `both_forms_coexist_on_row = true`.** A reader should not have
  to choose between "when exactly" and "how long ago" on a
  row that matters.
- **Signing-evidence-only rows** (where `redaction_class =
  signing_evidence_only`) MUST pin `time_zone_class = utc` and
  `absolute_time_format_class = iso_8601_utc`. Local rendering on
  a signing-evidence row is non-conforming.
- **Export previews** MUST pin `time_zone_class = utc` and
  `absolute_time_format_class = iso_8601_utc` — exports are
  independent of the exporter's local zone.
- **`observer_skew_labeled`** is reserved for rows whose display
  zone is derived from an observing client whose wall clock is
  unreliable; the label MUST say so rather than silently using
  the client's time.

## 6. Narrative summary cards and AI honesty

A `narrative_summary_card_record` is a first-party or AI summary
over one or more cited rows. It carries `cited_history_row_ids`
(minItems 1) and MUST NOT replace the cited rows — a reader can
always reach the underlying rows by id.

When `provenance_badge_class = ai_generated_summary` or
`ai_assisted`, the card MUST carry a non-null `ai_confidence_class`
drawn from `{low_confidence_disclosed,
medium_confidence_disclosed, high_confidence_disclosed}`. A card
whose provenance is AI but whose confidence is undisclosed is
non-conforming (`denial_reason =
ai_narrative_without_confidence_disclosure`).

Consequential and safety-critical narrative cards carry the same
non-truncating durable `detail_link` rule as consequential and
safety-critical rows (§4).

## 7. Chronology export previews

A `chronology_export_preview_record` is an in-product preview of
an export the user is about to generate (support bundle, AI-
evidence packet, policy-audit, publication proof, recovery proof,
admin audit). The preview carries:

- `export_preview_kind` — closed 6-value enum.
- `subject_canonical_object_target_ref` — the canonical object
  the export is about.
- `included_history_row_ids`, `included_timeline_group_ids`,
  `included_narrative_summary_card_ids` — opaque refs, structured
  vocabulary preserved identical to the live surface.
- `effective_redaction_class` — pinned value.
- `export_redaction_posture` — one of
  `preserve_live_surface_redaction`,
  `narrow_further_operator_only`,
  `narrow_further_internal_support_only`,
  `narrow_further_signing_evidence_only`. Widening relative to
  the live surface is forbidden
  (`denial_reason =
  export_widened_redaction_relative_to_live_surface`).
- `excluded_row_count_by_reason` — structured accounting with
  reasons `redaction_narrower_than_row`,
  `policy_blocked_under_export_posture`,
  `window_filter_out_of_range`, `scope_filter_not_applicable`.
  Silent drops are forbidden
  (`denial_reason = export_preview_silently_dropped_rows`).
- `display_time_policy` — pinned to
  `time_zone_class = utc` and
  `absolute_time_format_class = iso_8601_utc`.

The preview's summary_label and detail_link follow the same rules
as any row; a preview whose detail_link is a generic home screen
is non-conforming.

## 8. Redaction and privacy posture

- `redaction_class` is re-exported from ADR-0011 verbatim:
  `metadata_safe_default`, `operator_only_restricted`,
  `internal_support_restricted`, `signing_evidence_only`.
- The broker-owned redaction pass (ADR-0007) runs before bytes
  reach any persistent or exportable sink. `summary_label` and
  every label text MUST be short (max 200 chars) and privacy-
  safe; raw paths, raw URLs, raw secret material, raw prompt
  text, and raw customer-owned identifiers MUST NOT appear.
- Admin policy MAY narrow redaction on an export; it MAY NOT
  widen (ADR-0008).

## 9. Denial reasons (audit boundary)

This contract reserves the following denial reasons on the
chronology audit stream. A surface that violates any of these
MUST emit the matching denial rather than silently fall back to a
generic home screen, a truncated summary, or a widened redaction:

- `scope_object_kind_aliased`
- `widening_without_policy_basis`
- `duplicate_history_row_for_canonical_action`
- `group_truncates_consequential_member`
- `narrative_card_without_cited_rows`
- `ai_narrative_without_confidence_disclosure`
- `high_importance_row_truncated_without_detail_link`
- `detail_link_silently_downgraded`
- `export_preview_field_renamed_from_live_surface`
- `export_preview_silently_dropped_rows`
- `export_widened_redaction_relative_to_live_surface`

The notification-delivery contract's denial reasons
(`dismissal_silently_mutated_source_object`,
`dismissal_erased_durable_history`, etc.) continue to apply to
the envelopes that share lineage with history rows.

## 10. Worked examples

The companion fixtures under
[`/fixtures/ux/history_examples/`](../../fixtures/ux/history_examples/)
cover:

1. **AI evidence presented then superseded** —
   `ai_evidence_presented_then_superseded.json`. One AI-evidence
   row presented with `ai_assisted` provenance, then superseded
   by a higher-confidence row; the prior row remains reachable
   by id; a narrative summary card cites both.
2. **Task run: started, progressed, failed** —
   `task_run_started_progressed_failed.json`. Three rows sharing
   a canonical task object grouped by `same_canonical_object`;
   the failure row is `consequential` and carries a
   `full_detail_sheet` detail link.
3. **Policy block emitted then waived** —
   `policy_block_emitted_then_waived.json`. Two rows with
   `safety_critical` importance, `admin_policy_actor`, bound to
   one canonical policy-decision object; the waive row carries
   `widened` verb with an explicit supersedes reference.
4. **Provider sync narrowed a grant** —
   `provider_sync_narrowed_grant.json`. A `provider_grant_row`
   observed by a remote service with `observed_only` outcome
   and `observer_skew_labeled` time-zone class.
5. **Publish then revoke** — `publish_then_revoke.json`.
   Publication lifecycle across two rows with
   `evidence_packet_row` detail link; exported as a publication
   proof preview that preserves identity.
6. **Restore applied after recovery** —
   `restore_applied_after_recovery.json`. Recovery snapshot row
   plus the restore row; detail link is
   `canonical_object_target_exact`; provenance is
   `reconstructed_from_backup` on the snapshot row and
   `first_party_direct_observation` on the restore row.
7. **Support export preview** — `support_export_preview.json`.
   One `chronology_export_preview_record` of kind
   `support_bundle_chronology` that includes rows from fixtures
   1, 2, 3, and 6 by id, preserving every structured field, and
   discloses an `excluded_row_count_by_reason` of 2 under
   `redaction_narrower_than_row`.

These seven examples render from one shared schema without
renaming core fields per subsystem, satisfying the acceptance
criteria.

## 11. Adding a new vocabulary value

Adding a new `action_verb`, `scope_object_kind`, `outcome_class`,
`importance_class`, `provenance_badge_class`, `detail_link_kind`,
`timeline_group_rule`, `export_preview_kind`, or
`export_redaction_posture` is **additive-minor** and MUST bump
`history_row_schema_version`. Repurposing an existing value is
**breaking** and requires a new decision row on the launch
decision register. Every consumer surface that resolves a vocab
value it does not recognize MUST deny with
`record_schema_version_lagging` rather than silently map to a
default.

## 12. Out of scope at this revision

- The full timeline UI layout, motion, density, and keyboard
  traversal rules. Those live in the UI / UX Spec and the Design
  System Style Guide; this contract is the data boundary, not
  the rendering contract.
- Analytics on event frequency, chronology-row counts, or
  retention SLOs. A future change MAY add structured retention
  policy rows to the support bundle contract; this revision
  does not.
