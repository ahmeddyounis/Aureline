# scope_provenance_truth fixture corpus

Fixture corpus for the M4 stable hidden-scope, partial-scope,
archived-item, and imported-provider truth packet
(`schemas/search/scope_provenance_truth.schema.json`).

Each fixture is a `ScopeProvenanceTruthPacketInput` with an `expect` block
that pins the materialized packet's promotion state, finding count,
covered item-class tokens, provenance, downgrade, and imported-outcome
tokens, and exported support posture. Tests in
`crates/aureline-graph/tests/scope_provenance_truth_packet.rs` load each
case and assert that `ScopeProvenanceTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Rows covering every required item class
  (`hidden_scope`, `partial_scope`, `archived_item`, `imported_provider`)
  carry their disclosure ref, provenance class, freshness class,
  downgrade state, and required context. Six consumer projections
  preserve the packet verbatim, so the packet materializes as `stable`.
- `imported_missing_diagnostic_blocks_stable.json` — An
  `imported_provider` row declares an `unsupported` outcome but drops
  the `mapping_diagnostic_ref`; the packet blocks the stable claim with
  `imported_missing_diagnostic`.
- `non_canonical_presented_as_canonical_blocks_stable.json` — An
  `imported_provider` row is mislabeled with
  `downgrade_state == canonical`; the packet blocks the stable claim
  with `non_canonical_presented_as_canonical`.
- `archived_missing_context_blocks_stable.json` — An `archived_item`
  row drops its archived_context (no `archived_at` / register ref); the
  packet blocks the stable claim with `archived_missing_context`.
- `projection_drops_downgrade_blocks_stable.json` — The `graph_topology`
  consumer projection flips `preserves_downgrade_vocabulary` to false;
  the packet blocks the stable claim with `downgrade_vocabulary_dropped`
  and `consumer_projection_drift`.
