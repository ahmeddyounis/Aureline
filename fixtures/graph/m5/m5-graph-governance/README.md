# Fixtures: M5 graph-governance matrix

This directory contains fixture metadata for the `m5_graph_governance_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/graph/m5/m5-graph-governance.json`

## Coverage

- `workset_scope`, `graph_topology`, `impact_query`, `ownership_source`,
  `architecture_explainer`, `graph_freshness`, and `navigation_recall` are the only claimed
  lanes, and each carries exactly one row — no lane inherits an authoritative claim from an
  adjacent one.
- Each row binds to the canonical graph-truth packet it governs via `packet_ref` (validated
  against `GraphDepthLane::source_packet`), so the governance matrix aggregates the landed
  stable-line graph packets rather than a parallel spreadsheet, and each row carries its own
  conformance, evidence, governance-receipt, release-evidence, help-surface, docs-badge, and
  support-export refs.
- Scope mode covers `full_workspace`, `workset`, `hot_set`, and `unscoped`; graph freshness
  covers `fresh`, `lagging`, `stale`, and `expired`; relation fidelity covers `exact`,
  `resolved`, `approximate`, and `unresolved`; evidence backing covers `curated`, `cited`,
  `generated`, and `uncited`. The recovery path covers `widen_scope`, `reindex`,
  `resolve_relations`, `cite_or_curate`, `withhold_claim`, and `none`.
- Published claim covers `authoritative`, `scope_qualified`, `provisional`, and `withheld`,
  and the governance decision covers `publish`, `qualify_scope`, `mark_provisional`, and
  `withhold`.
- The four downgrade reasons — `scope_narrowed`, `stale_graph`, `approximate_relations`, and
  `uncited_explanation` — are each exercised by at least one lane.
- The impact-result class covers `no_impact`, `in_scope_impact`, `out_of_scope`, and
  `policy_limited`, so a no-impact answer stays distinct from an out-of-scope or
  policy-limited one. The `impact_query` lane carries a non-zero `out_of_scope_count`, and the
  `graph_freshness` and `navigation_recall` lanes carry a non-zero `hidden_result_count`,
  while the authoritative `graph_topology` and `ownership_source` lanes hide nothing.
- The governance gate is exercised in every direction: the clean `graph_topology` and
  `ownership_source` lanes publish authoritative claims; the workset-only `workset_scope` lane
  narrows to a scope-qualified claim; the hot-set/approximate `impact_query`, the
  generated/uncited `architecture_explainer`, and the stale `graph_freshness` lanes narrow to
  provisional; and the unscoped, expired, unresolved, uncited `navigation_recall` lane is
  withheld entirely. The `workset_scope` lane is the automatic-downgrade case — a sparse slice
  is dropped from its declared authoritative claim to scope-qualified rather than left implying
  whole-workspace certainty — while `graph_topology` and `ownership_source` prove the gate is
  not a blanket downgrade. The scope-sensitive `workset_scope`, `impact_query`, and
  `navigation_recall` lanes narrow safely instead of inheriting a broader whole-workspace
  claim. Each lane's `published_claim`, `governance_decision`, and `downgrade_reasons` equal
  the recomputed gate decision, so release, help/service-health, docs, and support-export
  surfaces ingest one packet and a narrowed lane cannot stay authoritative by inertia.
