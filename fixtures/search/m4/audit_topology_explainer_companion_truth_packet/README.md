# audit_topology_explainer_companion_truth_packet fixture corpus

Fixture corpus for the M4 stable audit of topology, explainer, and
companion-adjacent surfaces truth packet
(`schemas/search/audit_topology_explainer_companion_truth.schema.json`).

Each fixture is an `AuditTopologyExplainerCompanionTruthPacketInput`
with an `expect` block that pins the materialized packet's promotion
state, finding count, surface and row-class token sets, qualification
tokens, scope/freshness/provenance/downgrade-state disclosure tokens,
and exported support posture. Tests in
`crates/aureline-graph/tests/audit_topology_explainer_companion_truth_packet.rs`
load each case and assert that
`AuditTopologyExplainerCompanionTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Every governed audit surface carries an
  audited row, every row satisfies all four audit pillars, narrowed
  rows carry their disclosure refs, and every required consumer
  projection preserves the packet verbatim.
- `scope_unbound_blocks_stable.json` — A topology-canvas node claims
  `qualified_stable` while its scope disclosure is `scope_unbound`;
  the packet blocks the stable claim.
- `non_qualified_row_masquerading_stable_blocks_stable.json` — An
  impact-explainer impact edge claims `qualified_stable` while its
  freshness disclosure is `freshness_unbound`; the packet blocks the
  stable claim because the row masquerades as qualified.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A
  companion-history row is `narrowed_below_stable` but drops its
  disclosure ref; the packet blocks the stable claim.
- `audit_pillar_collapsed_blocks_stable.json` — A docs/help consumer
  projection drops the audit-pillar vocabulary; the packet blocks the
  stable claim.
