# M5 Chronology Group Primitive: Grouped Phases, Narrative Cards, and Export Previews

- Packet: `m5-chronology-group-primitive:stable:0001`
- Label: `M5 chronology-group primitive: grouped phases, narrative summary cards, timezone-safe export previews, and no-lost-causality ordering`
- History lanes: 6 (6 stable)
- Phases: initiation, execution, review, recovery, resolution
- Anatomy parts: phase_range_label, retained_group_ordering, group_event_count, group_primary_outcome, collapse_expand_control, narrative_current_state, narrative_recent_consequential_event, narrative_next_action, narrative_export_or_details_path, export_selected_range, export_included_fields, export_time_zone, export_redaction_class, export_output_format, relative_time_label, absolute_timestamp
- Redaction classes: metadata_only, pseudonymized_actors, aggregate_counts_only
- Export formats: json, csv, markdown, ndjson_stream
- Export fields: event_verb, provenance, timestamp, object_ref, actor_role, outcome_code, redaction_class
- Proof freshness SLO: 720 hours (last refresh: 2026-06-30T00:00:00Z)

## History lanes

- **AI Evidence**: `stable`
  - Owner: AI evidence owner
  - Scope: The AI-evidence lane groups a run into Initiation / Execution / Resolution phases, summarizes the current state in one sentence, and previews a metadata-only JSON export that keeps causal order
  - Shell zone: `bottom_panel`
  - Worked chronologies: 1
    - 3 event(s) in 3 group(s); next action: review_result
- **Policy Changes**: `stable`
  - Owner: Policy governance owner
  - Scope: The policy-changes lane groups an approval and a denial into one Review phase, keeps their causal order, and previews a pseudonymized-actor Markdown export
  - Shell zone: `bottom_panel`
  - Worked chronologies: 1
    - 2 event(s) in 1 group(s); next action: review_result
- **Task Events**: `stable`
  - Owner: Task lifecycle owner
  - Scope: The task-events lane groups a task's Initiation and Execution phases, summarizes that the run is still pending, proposes awaiting completion, and previews a CSV export
  - Shell zone: `bottom_panel`
  - Worked chronologies: 1
    - 2 event(s) in 2 group(s); next action: await_completion
- **Remote Reconnects**: `stable`
  - Owner: Remote-connector trust owner
  - Scope: The remote-reconnects lane groups a connection failure and its recovery into Execution and Recovery phases, keeping the causal order, and previews an aggregate-counts NDJSON export
  - Shell zone: `bottom_panel`
  - Worked chronologies: 1
    - 2 event(s) in 2 group(s); next action: review_result
- **Update History**: `stable`
  - Owner: Update channel owner
  - Scope: The update-history lane groups a channel's Initiation, a failed Execution, and a Recovery that replayed from history and reverted the change, and previews a metadata-only JSON export
  - Shell zone: `bottom_panel`
  - Worked chronologies: 1
    - 3 event(s) in 3 group(s); next action: acknowledge_resolution
- **Support Exports**: `stable`
  - Owner: Support export owner
  - Scope: The support-exports lane groups a bundle export into one Resolution phase, summarizes that the export completed with no further action needed, and previews a pseudonymized-actor Markdown export
  - Shell zone: `bottom_panel`
  - Worked chronologies: 1
    - 1 event(s) in 1 group(s); next action: no_action_needed
