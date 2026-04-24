# Output / log / result viewer contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/output_log_viewer_contract.md`](../../../docs/ux/output_log_viewer_contract.md)
and the schema at
[`/schemas/ux/output_viewer_object.schema.json`](../../../schemas/ux/output_viewer_object.schema.json).

The fixtures exist so task output panes, shell scrollback, log viewers,
result grids, notebook cell outputs, test / build dashboards, diagnostic
streams, and artifact preview surfaces compare against the same honesty
model for:

- output class vs size bucket vs viewer mode separation;
- live / cached / imported / snapshot origin honesty;
- no shell-freeze, no silent truncation, no silent active-content drop;
- trust / sandbox / active-content posture consistent with notebook,
  preview, and terminal surfaces;
- representation-labeled copy / export and buffered-change visibility.

Each JSON file is a single `output_viewer_object_record`. The
`__fixture__` prelude is reviewer metadata; the canonical vocabulary
lives in the record itself. Raw URLs, raw hostnames, raw absolute
paths, raw cookies, raw tokens, and raw rendered bytes never appear.

## Cases

- [`task_output_live_frozen_tail.json`](./task_output_live_frozen_tail.json)
  - unbounded live task-output stream virtualized with the autoscroll
    follow mode frozen by the reviewer; live-set state owns delivery,
    buffering, and follow posture.
- [`log_stream_provider_retention_truncated.json`](./log_stream_provider_retention_truncated.json)
  - long-running pipeline log viewer reading from a provider-owned
    retention window with provider-owned head truncation, paused away
    from the tail with buffered change visibility excluded until jump.
- [`result_grid_schema_drift_virtualized.json`](./result_grid_schema_drift_virtualized.json)
  - virtualized SQL result grid with anchored selection, approximate
    totals, and schema drift surfaced through the referenced live-set.
- [`notebook_cell_blocked_active_content.json`](./notebook_cell_blocked_active_content.json)
  - cell output preview that blocks untrusted active content on a
    SanitizedRich trust class and offers a typed textual fallback.
- [`build_output_very_large_open_detail.json`](./build_output_very_large_open_detail.json)
  - very-large build-output stream that would freeze the host shell if
    rendered inline and promotes to an open-in-detail panel without
    losing source identity or size disclosure.
- [`imported_artifact_readonly_review.json`](./imported_artifact_readonly_review.json)
  - imported test-artifact preview pinned to a static basis, read-only
    on copy / export, with typed artifact-edit posture.
- [`snapshot_capture_incident_replay.json`](./snapshot_capture_incident_replay.json)
  - incident snapshot replay of a combined stdio stream pinned to a
    captured basis; no live claims, named_snapshot_only export scope.
- [`diagnostic_stream_rate_limited.json`](./diagnostic_stream_rate_limited.json)
  - compiler diagnostic stream with manual scroll and a rate-limited
    producer surfaced through the referenced live-set provider limits.
