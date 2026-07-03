# M5 Profiler / Topology Component Proof

Release and support proof for
`artifacts/design/m5-profiler-topology-component-matrix.md`.

- `proof_packet.json` records the M5 review gates, component families,
  controlled-label coverage, and auto-narrowing triggers.
- `support_export.json` is the support-facing projection with raw profile,
  trace, heap, graph, provider, command-line, and private identity payloads
  excluded by default.
- `artifacts/perf/m5/m5-profile-session-hotpath-components.json` is the
  M05-797 profiler consumer packet for profile-session cards, flamegraph/icicle
  views, and call-tree rows.
- `artifacts/perf/m5/m5-trace-heap-compare-components.json` is the M05-798
  consumer packet for trace timelines, heap/allocation compare panels, and
  profile-compare cards, keeping baseline identity, environment deltas,
  threshold/waiver state, confounder notes, and imported-versus-live truth
  explicit before any regression is claimed.
- `artifacts/graph/m5/m5-workset-topology-components.json` is the M05-799
  consumer packet for workset switcher rows (with repo-lens scope banners),
  topology node cards, and relationship chips, keeping workset scope, index
  coverage, no-silent-widening, and node/edge freshness, confidence, provenance,
  and partial/blocked language explicit across search, topology, and explainer
  consumers.
- `artifacts/graph/m5/m5-ownership-explainer-components.json` is the M05-800
  consumer packet for ownership/contract cards and explainer-section cards. It
  keeps owner, reviewer, maintainer, service-owner, on-call, and approver roles
  distinct (never collapsed into one ambiguous owner), attaches protected-path /
  change-control links, and requires explainer cards to carry concrete
  file/symbol/doc citations with generated-vs-curated provenance. Generated
  summaries automatically narrow when their citations, freshness, or workset
  scope truth is incomplete, so an explanation never masquerades as uncited
  primary truth across topology, onboarding, AI, and review consumers.

- `artifacts/perf/m5/m5-component-accessibility-fallback-components.json` is the
  M05-801 accessibility hardening packet. It certifies, per component family, that
  every canvas-heavy consumer (flamegraph, icicle, heap/profile compare, trace
  timeline, topology map) binds its visual canvas to an equivalent
  list/table/textual path with the same filter/sort/range semantics, that keyboard
  traversal and screen-reader labeling never strand assistive-tech users in a
  view-only chart, that zoom/high-density rendering stays legible or discloses its
  reduction, that an export-safe summary object preserves the component meaning
  without a screenshot, and that narrower rendering surfaces (companion app,
  read-only browser, handoff packet, CLI/headless, support export) narrow
  interactivity explicitly while keeping the same labels and summary vocabulary
  rather than silently dropping state or actions. Its record schema is
  `schemas/ui/m5-component-accessibility-fallback.schema.json`.

- `artifacts/support/m5/m5-cross-surface-component-consumers.json` is the M05-802
  cross-surface consumer adoption packet. It proves the frozen families are
  reusable primitives rather than profile-only or graph-only implementations by
  adopting them across the five claimed consumer classes — performance tooling,
  search/graph understanding, an onboarding/explainer entry path, an AI/review
  consumer, and an incident/support surface — plus the help/support/release
  evidence surfaces. Every consumer points back to exactly one canonical
  component family (schema + producer packet) instead of cloning surface-local
  prose, and preserves the same capture/source badges, workset scope language,
  citation vocabulary, and degraded-state labels even when it is read-only,
  inspect-only, compare-only, or export-only; narrower consumers disclose the
  reduction with a reduced-capability banner and a companion/browser/handoff note
  rather than renaming or dropping governed state. Its record schema is
  `schemas/ui/m5-cross-surface-component-consumer.schema.json`.

- `artifacts/perf/m5/m5-profiler-topology-component-certification.json` is the
  M05-803 surface certification capstone. It certifies, per claimed consumer
  surface (17 performance, codebase-understanding, and shared-evidence surfaces),
  that the surface either passes the shared component packet or auto-narrows its
  claim state. Each row certifies four component-truth axes that apply to the
  families it consumes — capture/execution identity and imported-versus-live
  truth, compare baseline and confounder disclosure, workset scope and
  no-silent-widening, and topology/ownership/explainer provenance and citation
  posture — plus a cross-cutting label/export parity axis. A surface passes
  (green) when every applicable axis is certified, narrows (yellow) with a
  disclosed reduced-capability banner when an axis is a disclosed reduction, and
  is rejected (red) if it hides truth or presents partial graph state as full
  truth. Every certified surface cites this proof packet as its single
  certification bundle, so release, help, and support packets reference one proof
  of profiler/graph component truth. Its record schema is
  `schemas/ui/m5-profiler-topology-component-certification.schema.json`.

This proof packet is the release/support companion for tasks M05-796,
M05-797, M05-798, M05-799, M05-800, M05-801, M05-802, and M05-803.
