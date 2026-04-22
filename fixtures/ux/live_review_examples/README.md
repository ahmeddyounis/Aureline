# Live-review contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/live_update_review_contract.md`](../../../docs/ux/live_update_review_contract.md)
and the schema at
[`/schemas/ux/live_set_state.schema.json`](../../../schemas/ux/live_set_state.schema.json).

The fixtures exist so dense tables, result grids, log viewers, activity
feeds, incident timelines, support exports, and companion review
surfaces can all compare against the same honesty model for:

- live vs buffered vs stale vs snapshot delivery;
- client-controlled pause/freeze vs provider-owned limitations;
- anchor stability and batch-membership drift;
- copy/export scope while the surface is not fully live.

Each JSON file is a single `live_set_state_record`. The `__fixture__`
prelude is reviewer metadata; the canonical vocabulary lives in the
record itself.

## Cases

- [`result_grid_frozen_buffered_reorder.json`](./result_grid_frozen_buffered_reorder.json)
  - streaming result grid frozen on a keyed row while newer rows buffer
  and provider-owned ordering would otherwise reshuffle the review set.
- [`log_stream_paused_provider_truncated.json`](./log_stream_paused_provider_truncated.json)
  - log stream paused away from the tail with buffered segments plus a
  provider-owned retention/truncation boundary.
- [`activity_stream_frozen_insert_before_anchor.json`](./activity_stream_frozen_insert_before_anchor.json)
  - reverse-chronology activity feed that buffers inserts above the
  current review anchor instead of moving the list underneath the user.
- [`incident_timeline_snapshot_schema_drift.json`](./incident_timeline_snapshot_schema_drift.json)
  - incident timeline opened from an immutable snapshot while the live
  schema has drifted and the current review stays pinned to the captured
  basis.
