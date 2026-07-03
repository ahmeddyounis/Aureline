# M5 Profiler / Topology Component Matrix

Record kind: `m5_profiler_topology_component_matrix`
Schema version: `1`
Status: frozen for M5 first consumers

This matrix freezes Aureline's reusable profiler, trace, workset, topology,
ownership, and explainer component contracts. Profiler workspaces, trace
viewers, heap and regression compare flows, topology maps, search/graph
surfaces, onboarding, review, AI, incident, support exports, and release packets
consume these component families by reference instead of inventing local labels
for capture mode, origin, mapping quality, scope, provenance, confidence, or
role state.

The matrix is metadata-only. Components carry stable refs, controlled labels,
freshness and confidence state, provenance refs, copy/export-safe summaries, and
explicit reduced-capability behavior. They do not carry raw profile samples, raw
trace events, raw heap objects, raw command lines, raw local paths, secrets,
credentials, provider payloads, or private user identifiers. Imported and
support-bundle artifacts remain first-class but always render as imported or
bundle-scoped truth.

Certification bundle:

- Release proof:
  `artifacts/release/m5-profiler-topology-component-proof/proof_packet.json`
- Support export:
  `artifacts/release/m5-profiler-topology-component-proof/support_export.json`
- Profile hotpath consumer packet:
  `artifacts/perf/m5/m5-profile-session-hotpath-components.json`
- Trace/heap/compare consumer packet:
  `artifacts/perf/m5/m5-trace-heap-compare-components.json`
- Workset/topology consumer packet:
  `artifacts/graph/m5/m5-workset-topology-components.json`
- Ownership/explainer consumer packet:
  `artifacts/graph/m5/m5-ownership-explainer-components.json`
- Accessibility fallback / export-safe summary packet:
  `artifacts/perf/m5/m5-component-accessibility-fallback-components.json`
  (schema `schemas/ui/m5-component-accessibility-fallback.schema.json`)
- Cross-surface consumer adoption packet:
  `artifacts/support/m5/m5-cross-surface-component-consumers.json`
  (schema `schemas/ui/m5-cross-surface-component-consumer.schema.json`) — proves
  the families are reusable primitives by adopting them across performance,
  search/graph, onboarding/explainer, AI/review, and incident/support consumers,
  each pointing back to one canonical family with badge/scope/citation/degraded
  labels preserved even when read-only or export-only.
- Surface certification packet:
  `artifacts/perf/m5/m5-profiler-topology-component-certification.json`
  (schema `schemas/ui/m5-profiler-topology-component-certification.schema.json`) —
  the M05-803 certification capstone. It certifies, per claimed consumer surface
  (17 performance, codebase-understanding, and shared-evidence surfaces), that the
  surface either passes the shared component packet (green) or auto-narrows its
  claim (yellow) on capture/execution identity, compare baseline/confounder
  disclosure, workset scope and no-silent-widening, topology/ownership/explainer
  provenance and citation posture, and label/export parity. A surface that hides
  truth or presents partial state as full truth is rejected (red). Every certified
  surface cites this one certification bundle so release, help, and support packets
  reference a single proof of profiler/graph component truth.
- Fixtures:
  `fixtures/ui/m5-profiler-topology-components/`

## Source Bindings

| Component family | Canonical sources consumed by reference | First consumers |
| --- | --- | --- |
| Profile session card | `artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json`, `artifacts/perf/m5/certify-profiler-trace-replay-and-imported-versus-live-truth-on-all-claimed-m5-rows.json`, `schemas/ui/m5-profile-session-card.schema.json` | Desktop profiler workspace, hotspot workspace, trace viewer, profile compare, incident workspace, AI context panel, CLI/headless, support export, release proof |
| Flamegraph / icicle view | `artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json`, `artifacts/perf/m5/m5-profile-session-hotpath-components.json`, `schemas/ui/m5-flamegraph-view.schema.json` | Hotspot workspace, profile compare, review workspace, AI context panel, support export, release proof |
| Call-tree row | `artifacts/perf/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.json`, `artifacts/perf/m5/m5-profile-session-hotpath-components.json`, `schemas/ui/m5-call-tree-row.schema.json` | Hotspot workspace, profile compare, review workspace, AI context panel, support export, release proof |
| Trace timeline | `artifacts/perf/m5/implement-the-shared-trace-viewer-with-synchronized-event-lanes-bookmarks-and-textual-fallback.json`, `artifacts/perf/m5/add-chronology-and-reverse-step-controls-history-partiality-cues-and-import-or-export-packets.json`, `artifacts/perf/m5/m5-trace-heap-compare-components.json`, `schemas/ui/m5-trace-timeline.schema.json` | Trace viewer, chronology replay, incident workspace, support export, release proof |
| Heap/profile compare card | `artifacts/perf/m5/add-memory-analysis-views-snapshot-pairs-retained-or-allocation-diffs-and-leak-hint-confidence.json`, `artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json`, `artifacts/perf/m5/m5-trace-heap-compare-components.json`, `schemas/ui/m5-flamegraph-view.schema.json` | Heap analysis, profile compare, incident workspace, support export, release proof |
| Workset switcher row | `artifacts/graph/m5/m5-workset-scope.json`, `artifacts/search/m4/scope_provenance_truth_packet.json`, `schemas/ui/m5-workset-switcher-row.schema.json` | Search results, topology map, architecture explainer, review workspace, onboarding tour, AI context panel, support export |
| Topology node card | `artifacts/graph/m5/m5-topology-identity.json`, `artifacts/graph/m5/m5-impact-query.json`, `schemas/ui/m5-topology-node-card.schema.json` | Topology map, search results, architecture explainer, review workspace, onboarding tour, support export, release proof |
| Ownership card | `artifacts/graph/m5/m5-ownership-and-contracts.json`, `artifacts/graph/m5/m5-topology-identity.json`, `schemas/ui/m5-ownership-card.schema.json` | Ownership browser, topology map, review workspace, incident workspace, support export, release proof |
| Explainer section card | `artifacts/graph/m5/m5-explainer-and-architecture-maps.json`, `artifacts/graph/topology_and_explainer_packets/support_export_packet.json`, `schemas/ui/m5-explainer-section-card.schema.json` | Architecture explainer, onboarding tour, AI context panel, review workspace, docs/help, support export, release proof |

## Shared Disclosure Fields

All component fixtures include:

- `reduced_capability_banner` - stable banner id, severity, canonical
  `capability_state`, visible label, missing capabilities, preserved fields, and
  action policy.
- `source_refs` - checked-in packets or schemas that define the evidence source;
  raw local paths and payload bodies are excluded.
- `consumer_surfaces` - every claimed consumer using the component vocabulary.
- `copy_export` - text, JSON, and Markdown projections that preserve controlled
  labels and source refs; screenshot-only explanation is prohibited.
- `support_export_join` - stable join id, schema ref, joined object kinds, and
  raw-material exclusion flags.
- `auto_narrowing_contract` - missing/stale/policy-blocked truth triggers that
  cap GUI, CLI/headless, support, docs/help, and release claims.

Consumers may narrow authority or rendering capability, but they must not rename
or drop governed truth. For example, an imported profile may be inspect-only, but
it still carries capture mode, origin, mapping quality, build/runtime identity,
and baseline comparability fields.

## Controlled Vocabulary

| Vocabulary | Values |
| --- | --- |
| `consumer_surface` | `desktop_profiler_workspace`, `hotspot_workspace`, `trace_viewer`, `heap_analysis`, `profile_compare`, `topology_map`, `ownership_browser`, `architecture_explainer`, `search_results`, `review_workspace`, `onboarding_tour`, `ai_context_panel`, `incident_workspace`, `docs_help`, `cli_headless`, `support_export`, `release_proof` |
| `capture_mode` | `sample_cpu`, `instrumented_cpu`, `allocation`, `heap_snapshot`, `wall_time`, `trace`, `replay_import`, `imported_profile` |
| `execution_origin` | `local_desktop`, `ssh_remote`, `container_workspace`, `managed_workspace`, `browser_runtime`, `ci_runner`, `imported_artifact`, `support_bundle`, `cli_headless` |
| `artifact_origin` | `live_capture`, `replay_capture`, `imported_artifact`, `support_bundle`, `cached_replay`, `unknown` |
| `mapping_quality` | `exact`, `symbolicated`, `source_mapped`, `partial`, `heuristic`, `missing`, `not_applicable` |
| `baseline_environment_state` | `comparable`, `comparable_with_deltas`, `incomparable`, `baseline_missing`, `threshold_pending`, `waived` |
| `threshold_state` | `not_applicable`, `within_threshold`, `regression`, `improvement`, `threshold_pending`, `waived` |
| `workset_scope` | `full_workspace`, `named_workset`, `sparse_slice`, `imported_snapshot`, `support_bundle_scope`, `unknown` |
| `scope_change_state` | `unchanged`, `explicit_widen_available`, `explicit_narrow_available`, `suggested_widen_requires_review`, `policy_blocked`, `unknown` |
| `freshness_state` | `live`, `current`, `warm_cached`, `cached`, `imported`, `stale`, `superseded`, `partial`, `expired`, `policy_limited`, `unknown` |
| `confidence` | `confirmed`, `high`, `medium`, `low`, `unknown` |
| `provenance_class` | `indexed`, `imported`, `inferred`, `provider`, `annotation`, `runtime_capture`, `curated`, `generated` |
| `role_type` | `owner`, `reviewer`, `maintainer`, `subject_matter_expert`, `service_owner`, `oncall`, `approver`, `observer`, `unknown` |
| `summary_generation_mode` | `curated`, `generated`, `generated_reviewed`, `imported`, `unknown` |
| `capability_state` | `full`, `read_only`, `inspect_only`, `compare_only`, `export_only`, `policy_blocked`, `unavailable` |
| `copy_format` | `text`, `json`, `markdown` |

Feature-local labels that conflict with this vocabulary block review. A
consumer may narrow capability, but it may not rename `imported_artifact` as
live, flatten `partial` mapping into source-mapped truth, silently widen a
`sparse_slice`, collapse generated and curated summaries, or call a baseline
regression before baseline identity, environment deltas, threshold state, and
waiver state are visible.

## Component Field Sets

### Profile Session Card

Required truth: profile/session identity, profile kind, capture mode, artifact
origin, execution origin, build identity, runtime identity, target process/config,
captured-at timestamp, duration, storage/export posture, mapping quality,
baseline ref and comparability state when compare is available, trace/profile
refs, compare actions, export actions, source refs, consumer surfaces,
reduced-capability banner, support-export join, and auto-narrowing contract.

### Flamegraph / Icicle View

Required truth: profile ref, session ref, view mode, thread/process context,
thread/process filters, mapping quality, symbol/source-map summary,
imported-versus-live artifact origin, focus node, total samples/time,
self-versus-inclusive metric presentation, sample or cost scope, zoom state,
call-tree availability, source navigation availability, export/open-raw actions,
compare baseline state, threshold/waiver state, textual fallback, copy/export
projections, and reduced-capability disclosure.

### Call-Tree Row

Required truth: row/frame identity, function/frame name, self and inclusive
metrics, file/module/service refs, thread ref, symbolization state, mapping
quality, caller refs, callee refs, caller/callee navigation, source navigation,
copy/export projections, support-export join, and auto-narrowing contract.

### Trace Timeline

Required truth: trace ref, session ref, capture mode, artifact origin, process
and thread lanes, clock/sync basis, event lane summary, mapping quality,
bookmark and selected range refs, partiality note, imported/export packet refs,
textual fallback, chronology/reverse-step capability, copy/export projections,
and reduced-capability disclosure.

### Heap/Profile Compare Card

Required truth: baseline and candidate session refs, baseline identity,
environment deltas, threshold state, waiver state, retained/allocation diff
scope, leak-hint confidence, mapping quality, confounders, action policy,
copy/export projections, and reduced-capability disclosure. This contract uses
the flamegraph schema because compare cards share the profile-cost and mapping
truth grammar.

### Workset Switcher Row

Required truth: workset/snapshot identity, scope mode, included roots or repos,
excluded roots, index coverage, hidden/not-loaded counts, scope source, current
surface binding, no-silent-widening flag, widen/narrow actions, freshness,
source refs, copy/export projections, and reduced-capability disclosure.

### Topology Node Card

Required truth: node identity, node kind, namespace/workspace refs, active
workset snapshot, freshness, confidence, provenance class, source refs,
incoming/outgoing edge summaries, relation fidelity, ownership refs, explainer
refs, export-safe permalink, copy/export projections, and reduced-capability
disclosure.

### Ownership Card

Required truth: ownership object identity, owned object refs, role assignments,
role types, authority boundary, freshness, confidence, provenance, escalation
and review refs, service/on-call separation, generated/imported/curated origin,
copy/export projections, and reduced-capability disclosure.

### Explainer Section Card

Required truth: explainer section identity, topic/object refs, workset snapshot,
summary generation mode, cited file/symbol/doc refs, generated-versus-curated
truth, freshness, confidence, provenance, missing or partial citation state,
topology and ownership refs, copy/export projections, and reduced-capability
disclosure.

## Consumer Projection Rules

- Desktop profiler workspaces may capture live sessions, but imported/support
  sessions remain inspect-only and must retain artifact origin and mapping
  quality labels.
- Hotspot, flamegraph, call-tree, and heap views must keep thread/process
  context, mapping quality, source navigation availability, baseline identity,
  thresholds, waivers, and confounders visible before claiming regressions.
- Trace viewers must keep imported-versus-live truth, lane partiality, clock
  basis, and textual fallback visible in GUI and support exports.
- Search, topology, review, onboarding, AI, and explainer surfaces must keep
  workset scope, excluded roots, hidden/not-loaded counts, and no-silent-widening
  state visible.
- Topology and ownership cards must preserve freshness, confidence, provenance,
  role separation, and export-safe permalink refs across canvas, list, review,
  support, and release projections.
- Explainer cards must cite concrete files, symbols, docs, topology nodes, or
  ownership refs, and must distinguish generated, generated-reviewed, curated,
  and imported summaries.
- CLI/headless, support export, docs/help, and release proof consume the same
  controlled labels as the GUI. Missing proof freshness, missing source refs,
  stale artifacts, policy limits, or unavailable mappings narrow the claim.

## Review Gates

| Gate | Scope | Failure effect |
| --- | --- | --- |
| `required_field_parity` | Every schema-required field on every claimed consumer | Narrow the consumer claim. |
| `controlled_vocabulary_parity` | Capture mode, origin, mapping quality, baseline state, scope, freshness, confidence, provenance, role type, summary generation, and capability labels | Block review until labels match or narrow the surface. |
| `imported_vs_live_truth` | Profile sessions, flamegraphs, timelines, compare cards, support bundles | Narrow to inspect-only or export-only. |
| `baseline_confounder_disclosure` | Compare and regression consumers | Block regression claim. |
| `workset_no_silent_widening` | Search, topology, onboarding, review, AI, explainer consumers | Block full-workspace claim. |
| `citation_and_provenance_parity` | Topology, ownership, explainer consumers | Narrow to available evidence only. |
| `copy_export_parity` | GUI, CLI/headless, support export, docs/help, release proof | Narrow the consumer claim. |
| `accessibility_fallback_parity` | Every canvas-heavy family (flamegraph, icicle, heap/profile compare, trace timeline, topology map) plus ownership/explainer views on every claimed consumer | Bind a keyboard/screen-reader-reachable list/table/textual path with the same filter/sort/range semantics; a view-only chart/map that strands assistive tech blocks review. |
| `export_safe_summary_parity` | GUI, companion, browser, handoff packet, CLI/headless, support export, release proof | Every component carries an export-safe summary object that reconstructs its meaning without a screenshot; screenshot-only export narrows the claim. |
| `narrowed_surface_disclosure` | Companion, read-only browser, handoff packet, CLI/headless, support export | Narrower surfaces disclose reduced interactivity and keep the same labels/summary vocabulary; silently dropping state or actions blocks review. |
| `cross_surface_consumer_reuse` | Every claimed consumer class (performance, search/graph, onboarding/explainer, AI/review, incident/support) plus help/support/release evidence surfaces | Each consumer points back to exactly one canonical component family rather than cloning surface-local prose; renaming, omitting, or under-disclosing a canonical badge/scope/citation/degraded label on a narrower consumer blocks review. |
| `surface_component_certification` | Every claimed M5 performance, codebase-understanding, and shared-evidence surface (17 surfaces) | Each surface either passes the shared component packet on every applicable truth axis (capture/execution identity, compare baseline/confounder, workset scope, graph provenance/citation, label/export parity) or auto-narrows its claim with a disclosed reduced-capability banner; a surface that hides truth or presents partial graph state as full truth is rejected, and every certified surface cites one certification bundle for release/help/support. |
