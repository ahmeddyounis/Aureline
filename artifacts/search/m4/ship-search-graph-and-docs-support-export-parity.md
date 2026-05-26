# Search, graph, and docs support-export parity — reviewer artifact

This is the reviewer-facing artifact for the M4 stable support-export
parity, query-session/search-export, retrieval-debug, and operator-truth
inspector truth packet. The contract lives at
`docs/search/m4/ship-search-graph-and-docs-support-export-parity.md`
and is replayed by
`crates/aureline-graph/tests/support_export_parity_truth_packet.rs`.

## Stable claim

For every governed lane class (`search_export`,
`graph_topology_export`, `docs_handoff_export`,
`operator_truth_inspector`, `retrieval_debug`,
`query_session_export`) the packet binds:

- a `query_session_id_ref` (every consumer reuses one query-session
  object instead of inventing private candidate lists),
- a closed `redaction_class` (default support exports omit raw query
  material unless `explicit_literal_consent` is recorded),
- a closed `live_vs_captured_class` (no surface pretends a captured
  snapshot is live),
- a closed `downgrade_state` (truncation, policy withholding,
  provider unavailability, and scope changes are explicit),
- a closed `confidence_class`, and
- a `disclosure_ref` plus at least one `evidence_refs` entry.

Operator-truth inspector rows carry a reconstruction proof. The
query-session deep-link binding forces recipient re-resolution and
forbids frozen recipient certainty.

## Companion artifacts

- Schema: `schemas/search/support_export_parity_truth.schema.json`
- Checked-in packet: `artifacts/search/m4/support_export_parity_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/support_export_parity_truth_packet/`
- Rust contract: `crates/aureline-graph/src/support_export_parity_truth_packet/mod.rs`
- Replay tests: `crates/aureline-graph/tests/support_export_parity_truth_packet.rs`
- Reviewer doc: `docs/search/m4/ship-search-graph-and-docs-support-export-parity.md`
