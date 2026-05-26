# Support-export parity truth — milestone note

This is the milestone-level note for the support-export parity truth
lane that binds search-export, graph-topology-export, docs-handoff,
operator-truth inspector, retrieval-debug, and query-session export
packets to one stable knowledge-plane contract. The authoritative
contract document is
`docs/search/m4/ship-search-graph-and-docs-support-export-parity.md`.
The canonical checked-in artifact is
`artifacts/search/m4/support_export_parity_truth_packet.json`.
The schema lives at
`schemas/search/support_export_parity_truth.schema.json`. The fixture
corpus lives under
`fixtures/search/m4/support_export_parity_truth_packet/`.

The implementation lives at
`crates/aureline-graph/src/support_export_parity_truth_packet/mod.rs`
and is replayed by
`crates/aureline-graph/tests/support_export_parity_truth_packet.rs`.
