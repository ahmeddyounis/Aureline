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

This proof packet is the release/support companion for tasks M05-796,
M05-797, and M05-798.
