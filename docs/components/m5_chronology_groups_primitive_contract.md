# M5 Chronology Group Primitive Contract — Grouped Phases, Narrative Cards, and Export Previews

> Task: M05-760 · Batch B88 · Delivery class: high-trust component contract +
> reusable primitive implementation + support/export parity.

This contract implements Aureline's **one reusable chronology-grouping primitive**
across every M5 history that leaves the live surface — AI, policy, task, remote,
update, and support — so grouped phases, one-sentence state summaries, absolute /
relative time parity, and export previews that keep causality visible behave the
same way everywhere. Where the
[evidence / activity row primitive](m5_evidence_row_primitive_contract.md) (M05-759)
turned one event into a stable, copyable row, this lane takes the next step: it
turns a *sequence* of events into a usable chronology *surface*. It narrows the
timeline-group, narrative-summary-card, and chronology-export-preview families named
by the frozen
[M5 trust-chronology component matrix](m5_trust_chronology_components_contract.md)
(M05-756) into a working primitive with a resolver and a per-lane parity matrix, and
is the chronology capstone of the
[settings-row](m5_settings_row_primitive_contract.md) (M05-757),
[capability-sheet](m5_capability_sheet_primitive_contract.md) (M05-758), and
[evidence-row](m5_evidence_row_primitive_contract.md) (M05-759) primitives.

- **Boundary schema:** [`schemas/ui/m5-chronology-export-preview.schema.json`](../../schemas/ui/m5-chronology-export-preview.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/implement_the_m5_chronology_group_narrative_card_and_export_preview_primitive/`
- **Headless emitter:** `aureline_shell_m5_chronology_group_primitive`
- **Checked support export:** [`artifacts/release/m5-chronology-groups-proof/support_export.json`](../../artifacts/release/m5-chronology-groups-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-chronology-groups-proof/matrix.csv`](../../artifacts/release/m5-chronology-groups-proof/matrix.csv)
- **Report:** [`artifacts/components/m5-chronology-groups-primitive.md`](../../artifacts/components/m5-chronology-groups-primitive.md)
- **Narrowed fixtures:** [`fixtures/ui/m5-chronology-groups-primitive/`](../../fixtures/ui/m5-chronology-groups-primitive/)

The stable chronology verbs, provenance badges, chronology detail states,
chronology export fields, non-visual accessibility routes, qualification classes,
and downgrade triggers are reused verbatim from the frozen
[M5 trust-chronology component matrix](../../schemas/ui/m5-trust-chronology-components.schema.json);
the shell topology — zones, responsive classes, window classes, and consumer
surfaces — is reused verbatim from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). This lane
mints new vocabulary only for what those matrices left implicit about grouping a
chronology: its **history lanes**, its **phases**, its resolver-side **outcomes**,
its **next actions**, its **redaction classes**, its export **output formats**, its
**surface anatomy**, and its **focus behaviors**. No M5 surface invents a second
chronology grammar.

The primitive also projects from the existing chronology / export contracts:
[`schemas/support/evidence_timeline.schema.json`](../../schemas/support/evidence_timeline.schema.json),
[`schemas/support/export_redaction_profile.schema.json`](../../schemas/support/export_redaction_profile.schema.json),
and [`schemas/governance/m5_evidence_chronology_lineage.schema.json`](../../schemas/governance/m5_evidence_chronology_lineage.schema.json).

## Track invariant

One evidence / chronology model carries stable verbs, provenance badges, and
portable detail / export semantics. History never flattens into ambiguous text: a
long sequence reads as labeled phase bands with retained ordering; the narrative
card explains what state we are in and what to do next; relative time stays
available for scanning while absolute timestamps survive in detail and export; and
every export preview declares its range, fields, time zone, redaction class, and
output format while preserving causal order.

## The primitive: two halves

### 1. Resolver — `resolve_chronology`

Given one history lane's ordered events (each a `phase`, a monotonic `sequence`, an
`absolute_timestamp`, a `relative_label`, a stable `verb`, a `provenance` badge, a
controlled `outcome`, an `object_repr`, a `consequential` flag, and an optional
`detail_ref`) plus an export request, the resolver produces one
`M5ResolvedChronology` carrying:

- **Timeline groups** — one per contiguous same-phase run, so ordering is retained
  exactly. Each group carries a phase / range label, an absolute and relative range,
  an event count, a primary outcome (the terminal event's), the first and last
  sequence, a default `collapse_state` (`expanded` for the most recent group or any
  group holding a failure/denial, `collapsed` otherwise), and its events in causal
  order.
- **Narrative summary card** — the one-sentence `current_state_sentence`, the most
  recent consequential event, a controlled `next_action` (with a one-sentence hint),
  the `open_details_ref` reopen path, and `export_path_available`.
- **Export preview** — the selected absolute range, included fields, time zone,
  redaction class, output format, the `event_order` (event sequences), and
  `preserves_causal_order`.

The resolver rejects malformed input: no events, an empty timestamp, relative label,
or object, a non-monotonic sequence (`non_monotonic_sequence` — causality would be
ambiguous), an empty export range or time zone, an export request missing a mandatory
truth field (`missing_mandatory_export_field`), and any representation carrying URLs,
credentials, or other forbidden material.

### 2. Parity matrix — one row per history lane

Each of the six history lanes carries the same shared grouping / narrative / export
anatomy, the same phases, stable verbs, provenance badges, outcomes, detail states,
next actions, redaction classes, output formats, and export fields, plus worked
resolution cases proving the resolver on that lane. Every lane renders in the
`bottom_panel` zone — the execution, output, problems, terminal, and timeline zone.

| History lane | Worked resolution highlight |
| --- | --- |
| `ai_evidence` | Initiation → Execution → Resolution; metadata-only JSON export |
| `policy_changes` | approval + denial in one Review phase; pseudonymized Markdown export |
| `task_events` | Initiation → Execution, still pending → await completion; CSV export |
| `remote_reconnects` | failed Execution → Recovery; aggregate-counts NDJSON export |
| `update_history` | Initiation → failed Execution → replayed Recovery/revert; JSON export |
| `support_exports` | one Resolution phase; `exported` → no action needed; Markdown export |

## Anatomy (shared surface)

All sixteen anatomy parts are mandatory on every lane: the group's
`phase_range_label`, `retained_group_ordering`, `group_event_count`,
`group_primary_outcome`, and `collapse_expand_control`; the narrative card's
`narrative_current_state`, `narrative_recent_consequential_event`,
`narrative_next_action`, and `narrative_export_or_details_path`; the export
preview's `export_selected_range`, `export_included_fields`, `export_time_zone`,
`export_redaction_class`, and `export_output_format`; and the time-parity pair
`relative_time_label` and `absolute_timestamp`.

## Phases, verbs, and provenance

Phases are a closed vocabulary — `initiation`, `execution`, `review`, `recovery`,
`resolution` — grouping contiguous runs. The verb vocabulary (`created`, `updated`,
`ran`, `approved`, `rejected`, `failed`, `recovered`, `exported`) and provenance
badges (`human_initiated`, `ai_initiated`, `automation_initiated`, `remote_actor`,
`system_initiated`, `replayed_from_history`) are reused verbatim from the frozen
matrix. Outcomes (`succeeded`, `failed`, `pending`, `denied`, `reverted`) are a
resolver-side concept kept orthogonal to the verb.

## Time parity and export previews

Every event carries both a `relative_label` for scanning and an `absolute_timestamp`
that survives into group ranges and the export preview's selected range. Export
previews declare their `redaction_class` (`metadata_only`, `pseudonymized_actors`,
`aggregate_counts_only`) and `output_format` (`json`, `csv`, `markdown`,
`ndjson_stream`) and carry the event `event_order` so causality is preserved instead
of flattened. The export fields `event_verb`, `provenance`, `timestamp`,
`object_ref`, `actor_role`, and `outcome_code` are mandatory; `redaction_class`
completes the record.

## Support / export reconstruction

Each lane carries its worked resolution cases in the export, and the validator
re-runs the resolver on every stored input and asserts it equals the stored output.
Packet-level lints require that the worked cases collectively (a) prove every lane
renders grouped chronology and an export preview (`grouped_chronology_unproven`),
(b) exercise every phase (`phase_vocabulary_unproven`), (c) prove relative + absolute
time parity into the export (`time_parity_unproven`), and (d) prove a multi-event
export preserves causal order with redaction intent
(`causality_preservation_unproven`) — so the support export reconstructs the grouped
chronology from the same shared model, no screenshot required.

## Hard invariants (per surface row, all MUST be false)

- `flattens_causal_ordering`
- `drops_absolute_timestamp`
- `drops_redaction_intent`
- `drops_export_or_audit_truth`

The Rust validator and resolver in `crates/aureline-shell` are the authoritative
gate; the schema and this doc document the shape. Regenerate the checked export,
CSV, report, and fixtures with the headless emitter subcommands (`support-export`,
`csv`, `report`, `fixture-update-history-beta-narrowed`,
`fixture-support-exports-preview-narrowed`).
