# Implement the Shared Trace Viewer with Synchronized Event Lanes, Bookmarks, and Textual Fallback

This document is the reviewer-facing landing page for the M5 trace-viewer,
synchronized event-lane, bookmark, and textual-fallback lane.

## Scope

This lane governs how trace viewer surfaces:

- show synchronized event lanes per thread, stream, category, or merged source
  with a shared time axis, zoom, and scroll state, plus mapping quality for every lane;
- display bookmarks anchored to lanes with timestamp, category, note, creator identity,
  and provenance so markers are always attributable;
- offer textual fallback views that show structured events, raw spans, annotated logs,
  or comparison diffs with source event references and mapping quality when visual
  rendering is unavailable;
- degrade honestly by showing degraded-state labels when mapping fidelity,
  synchronization state, or baseline comparability are weak.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/implement_the_shared_trace_viewer_with_synchronized_event_lanes_bookmarks_and_textual_fallback/`
- **Packet:** `artifacts/perf/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.json`
- **Schema:** `schemas/perf/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.schema.json`
- **Fixtures:** `fixtures/performance/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Event lane view | Stable | Shows synchronized lanes per thread, stream, or category with shared time axis, zoom, scroll state, and mapping quality for every lane. |
| Bookmark panel | Stable | Shows bookmarks with timestamp, lane reference, category, note, creator identity, and provenance. |
| Textual fallback view | Stable | Shows structured events, raw spans, annotated logs, or comparison diffs with source event references and mapping quality when visual rendering is unavailable. |
| Trace comparison | Preview | Side-by-side trace diff and baseline alignment are still under qualification. |
| Export review | Preview | Redaction-safe export flows for trace evidence are still under qualification. |
| Support export | Preview | Support-bundle redaction for trace payloads is still under qualification. |

## Event Lane Kinds

The module carries a closed event-lane vocabulary:

- `thread_lane` — lane bound to a single thread;
- `stream_lane` — lane bound to an async stream or channel;
- `category_lane` — lane grouping events by category;
- `merged_lane` — lane merging multiple sources.

Every event lane MUST show its mapping quality and a degraded-state label.
Lanes that claim synchronization MUST list the lane ids they are synchronized with.

## Mapping-Quality Labels

The module carries a closed mapping-quality vocabulary:

- `exact` — exact symbol and source location;
- `approximate` — approximate match; may be nearest symbol or line;
- `partial` — partial mapping; some inlined or generated frames;
- `unavailable` — no mapping available;
- `stale` — mapping is stale relative to current build;
- `mismatched` — mapping mismatches the current build.

Every event-lane row and textual-fallback row MUST show its mapping quality.

## Textual-Fallback Content Kinds

The module carries a closed content-kind vocabulary:

- `structured_events` — structured event lines with timestamps and names;
- `raw_spans` — raw span records with ids and ranges;
- `annotated_log` — annotated log output;
- `comparison_diff` — diff output from trace comparison.

Every textual-fallback row MUST show its mapping quality and MUST reference the
source event or lane refs that produced the content.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Event-lane rows MUST show mapping quality and a degraded-state label; missing
  labels trigger a validation violation.
- Bookmark rows MUST show provenance; missing provenance triggers a validation
  violation.
- Textual-fallback rows MUST show mapping quality; missing labels trigger a
  validation violation.
- Cross-reference failures (bookmark lane ref unknown, textual-fallback source
  ref unknown) trigger validation violations.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every event lane carries mapping quality and shows it.
- Every bookmark carries creator identity and shows provenance.
- Every textual fallback carries source event refs and shows mapping quality.
- Trace bundles are immutable once captured; derived lanes, bookmarks, and
  textual fallbacks are separate derived artifacts with their own provenance.
