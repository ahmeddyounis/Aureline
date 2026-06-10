# Ship the Hotspot Workspace with Flamegraph, Call Tree, Mapping-Quality Labels, and Source Navigation

This document is the reviewer-facing landing page for the M5 hotspot-workspace,
flamegraph, call-tree, mapping-quality, and source-navigation lane.

## Scope

This lane governs how profiler hotspot surfaces:

- show CPU or time hotspots in a flamegraph or icicle view with self and inclusive
  metrics, depth, thread filter, and mapping quality for every frame;
- show keyboard-first call-tree inspection with function or frame identity,
  file/module/service, thread, symbolization state, and caller/callee navigation;
- lead every view with a session strip that identifies workload, build/runtime,
  capture mode, mapping quality, capture time, duration, and profile posture;
- display mapping-quality badges that prevent false precision by showing exact,
  approximate, partial, unavailable, stale, or mismatched states;
- offer source navigation from hotspots and frames that degrades honestly to raw
  addresses or module ranges when fidelity is incomplete.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation/`
- **Packet:** `artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json`
- **Schema:** `schemas/perf/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.schema.json`
- **Fixtures:** `fixtures/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Flamegraph view | Stable | Shows self and inclusive samples, depth, thread filter, mapping quality, and source navigation for every frame. |
| Call tree view | Stable | Shows function or frame identity, self and inclusive metrics, file/module/service, thread, symbolization state, and caller/callee navigation. |
| Session strip | Stable | Shows workload identity, build/runtime identity, capture mode, mapping quality state, capture time, duration, profile posture, and compare/export actions. |
| Mapping-quality badge | Stable | Shows exact, approximate, partial, unavailable, stale, or mismatched state with unresolved frame count and imported-symbol note. |
| Source navigation | Stable | Shows available actions, default action, source ref, line number, and mapping quality before every jump. |
| Export review | Preview | Redaction-safe export flows for hotspot evidence are still under qualification. |
| Support export | Preview | Support-bundle redaction for hotspot payloads is still under qualification. |

## Mapping-Quality Labels

The module carries a closed mapping-quality vocabulary:

- `exact` — exact symbol and source location;
- `approximate` — approximate match; may be nearest symbol or line;
- `partial` — partial mapping; some inlined or generated frames;
- `unavailable` — no mapping available;
- `stale` — mapping is stale relative to current build;
- `mismatched` — mapping mismatches the current build.

Every frame, call-tree row, and source-navigation action MUST show its mapping
quality. Source navigation is only offered when the mapping quality is `exact`,
`approximate`, or `partial`; `unavailable`, `stale`, or `mismatched` frames MUST
fall back to raw address or module-range views.

## Profile Posture

Session strips carry a closed posture vocabulary:

- `live_capture` — live capture from the current session;
- `imported_artifact` — imported from an external file or bundle;
- `cached_local_evidence` — cached local evidence from a prior session;
- `stale_prior_result` — stale prior result that may no longer reflect current state.

Every session strip MUST show its posture so users never confuse live, imported,
cached, or stale evidence.

## Source-Navigation Actions

The module carries a closed action vocabulary:

- `open_source` — open the source file at the resolved line;
- `open_caller` — open the caller of the current frame;
- `open_callee` — open a callee of the current frame;
- `open_raw` — open the raw symbol or address view.

Every source-navigation row MUST show its mapping quality before the jump and MUST
not silently navigate to a likely file when fidelity is incomplete.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Flamegraph rows MUST show mapping quality; missing labels trigger a validation
  violation.
- Call-tree rows MUST show symbolization state; missing state triggers a validation
  violation.
- Session-strip rows MUST show a degraded-state label; missing labels trigger a
  validation violation.
- Mapping-quality badges MUST show their mapping quality; missing labels trigger a
  validation violation.
- Source-navigation rows MUST show mapping quality before the jump; missing labels
  trigger a validation violation.
- Cross-reference failures (unknown frame or context refs) trigger validation
  violations.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every flamegraph and call-tree row carries mapping quality and shows it.
- Every session strip carries workload, build, capture mode, mapping quality, and
  profile posture.
- Every source-navigation row shows mapping quality before the jump and degrades
  to raw view when fidelity is incomplete.
- Trace bundles are immutable once captured; derived flamegraphs, symbolized call
  trees, and AI summaries are separate derived artifacts with their own provenance.
