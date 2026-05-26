# support_export_parity_truth fixture corpus

Fixture corpus for the M4 stable support-export parity, query-session/
search-export, retrieval-debug, and operator-truth inspector truth packet
(`schemas/search/support_export_parity_truth.schema.json`).

Each fixture is a `SupportExportParityTruthPacketInput` with an `expect`
block that pins the materialized packet's promotion state, finding
count, covered lane tokens, export-packet-class tokens, redaction tokens,
live-vs-captured tokens, downgrade tokens, and exported support posture.
Tests in `crates/aureline-graph/tests/support_export_parity_truth_packet.rs`
load each case and assert that
`SupportExportParityTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Rows covering every required lane class
  (`search_export`, `graph_topology_export`, `docs_handoff_export`,
  `operator_truth_inspector`, `retrieval_debug`, `query_session_export`)
  carry their query-session ref, count summary, evidence refs,
  redaction class, live-vs-captured class, downgrade state, and
  disclosure ref. Operator-truth rows carry a reconstruction proof;
  the query-session export row carries a deep-link binding that forces
  recipient re-resolution. Eight consumer projections preserve the
  packet verbatim, so the packet materializes as `stable`.
- `raw_query_text_leak_blocks_stable.json` — A `search_export` row
  flips `raw_query_text_excluded` to false; the packet blocks the
  stable claim with `raw_query_text_present`.
- `projection_drops_redaction_blocks_stable.json` — The `docs_help`
  consumer projection flips `preserves_redaction_vocabulary` to false;
  the packet blocks the stable claim with
  `redaction_vocabulary_collapsed` and `consumer_projection_drift`.
- `operator_truth_missing_reconstruction_blocks_stable.json` — The
  `operator_truth_inspector` row drops its
  `operator_reconstruction_proof`; the packet blocks the stable claim
  with `operator_truth_missing_reconstruction`.
- `deep_link_freezes_certainty_blocks_stable.json` — The
  `query_session_export` row's deep-link binding flips
  `requires_recipient_resolution` and `frozen_certainty_excluded` to
  false; the packet blocks the stable claim with
  `deep_link_freezes_recipient_certainty`.
