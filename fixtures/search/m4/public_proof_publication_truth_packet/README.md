# public_proof_publication_truth_packet fixture corpus

Fixture corpus for the M4 stable public-proof publication truth packet
(`schemas/search/public_proof_publication_truth.schema.json`).

Each fixture is a `PublicProofPublicationTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, lane and row-class token sets, publication-state,
known-limit, downgrade-automation, and proof-artifact tokens, and the
support-export safety verdict. Tests in
`crates/aureline-graph/tests/public_proof_publication_truth_packet.rs`
load each case and assert that
`PublicProofPublicationTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Every governed publication lane carries a
  public-truth row, a known-limit row, and a downgrade-automation row;
  published-stable rows bind their known limit and downgrade
  automation, narrowed rows carry their disclosure refs, and all eight
  required consumer projections preserve the packet verbatim.
- `published_stable_with_unbound_limit_blocks_stable.json` — A search
  public-truth row claims `published_stable` while its known-limit
  class is `limit_unbound`; the packet blocks the stable claim.
- `published_stable_with_unbound_automation_blocks_stable.json` — A
  graph public-truth row claims `published_stable` while its
  downgrade-automation class is `automation_unbound`; the packet
  blocks the stable claim because the row has no automation to
  defend it.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A docs
  known-limit row is `narrowed_below_stable` but drops its disclosure
  ref; the packet blocks the stable claim.
- `projection_collapses_downgrade_automation_blocks_stable.json` — The
  `help_about` consumer projection drops the downgrade-automation
  vocabulary; the packet blocks the stable claim.
- `raw_query_material_blocks_stable.json` — A search public-truth row
  admits raw query text past the boundary; the packet blocks the
  stable claim because raw material must never leak through the
  public-proof publication boundary.
