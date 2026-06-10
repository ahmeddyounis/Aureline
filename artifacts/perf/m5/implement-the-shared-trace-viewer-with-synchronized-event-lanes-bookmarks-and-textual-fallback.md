# Implement the Shared Trace Viewer with Synchronized Event Lanes, Bookmarks, and Textual Fallback

**Artifact type:** Performance evidence qualification packet (M5)
**Packet id:** m5_047_trace_viewer_qualification:v1
**As of:** 2026-06-09

## Summary

- Event-lane rows: 4
- Bookmark rows: 3
- Textual-fallback rows: 3
- Stable surfaces: 3
- Below-stable surfaces: 3
- All below-stable surfaces have disclosure: yes

## Claims

| Surface | Claim | Status |
|---|---|---|
| Event lane view | Stable | Certified |
| Bookmark panel | Stable | Certified |
| Textual fallback view | Stable | Certified |
| Trace comparison | Preview | Under qualification |
| Export review | Preview | Under qualification |
| Support export | Preview | Under qualification |

## Evidence

- `fixtures/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback/event_lane_thread.json`
- `fixtures/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback/event_lane_stream.json`
- `fixtures/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback/bookmark_user_marker.json`
- `fixtures/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback/textual_fallback_structured.json`

## Schema and Implementation

- Schema: `schemas/perf/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.schema.json`
- Implementation: `crates/aureline-profiler/src/implement_the_shared_trace_viewer_with_synchronized_event_lanes_bookmarks_and_textual_fallback/`

## Downgrade Rules

1. If a stable surface is missing a required guard, it is narrowed to preview.
2. If an event-lane row does not show mapping quality, the row is flagged as a validation violation.
3. If an event-lane row does not show a degraded-state label, the row is flagged as a validation violation.
4. If a bookmark row does not show provenance, the row is flagged as a validation violation.
5. If a textual-fallback row does not show mapping quality, the row is flagged as a validation violation.
6. If a bookmark references an unknown lane, or a textual-fallback references an unknown source, the row is flagged as a validation violation.
