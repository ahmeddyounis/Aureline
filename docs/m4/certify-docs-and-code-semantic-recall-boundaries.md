# Docs/code semantic-recall boundary — milestone note

Milestone-level pointer to the stable docs/code semantic-recall
boundary truth packet between the M4 stable lane and the v1.x
preview lane.

The packet binds every certified recall row to a closed
`recall_lane_class`, `surface_track`, `locality_class`,
`retrieval_epoch_state`, `pack_signature` state, `downgrade_state`,
and `confidence_class` vocabulary, plus an `embedder_identity`,
`lane_participation` envelope, chunk-or-anchor provenance ref, and
a hybrid `ranking_reason_ref`. Mirrored-pack and managed locality
rows MUST carry a verified pack signature; non-lexical rows MUST
carry an embedder identity; v1.x preview rows MUST carry a recorded
downgrade so they cannot inherit adjacent stable rows.

Reviewer doc:
[`docs/search/m4/certify-docs-and-code-semantic-recall-boundaries.md`](../search/m4/certify-docs-and-code-semantic-recall-boundaries.md)

Stable artifact:
[`artifacts/search/m4/semantic_recall_boundary_truth_packet.json`](../../artifacts/search/m4/semantic_recall_boundary_truth_packet.json)

Schema:
[`schemas/docs/semantic_recall_boundary_truth.schema.json`](../../schemas/docs/semantic_recall_boundary_truth.schema.json)

Rust contract:
[`crates/aureline-docs/src/semantic_recall_boundary_truth_packet/mod.rs`](../../crates/aureline-docs/src/semantic_recall_boundary_truth_packet/mod.rs)
