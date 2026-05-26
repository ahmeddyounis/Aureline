# Docs/code semantic-recall boundary — stable artifact narrative

Human-readable companion to the checked-in stable docs/code
semantic-recall boundary truth packet
([`semantic_recall_boundary_truth_packet.json`](semantic_recall_boundary_truth_packet.json)).

## What this artifact certifies

The packet is the stable contract that ties together the M4 lane
for docs/code semantic recall and the explicit v1.x preview lane.
Every governed row pins:

- A closed `recall_lane_class`
  (`docs_semantic_recall`, `code_semantic_recall`,
  `hybrid_fused_recall`, `lexical_only_fallback`).
- A closed `surface_track` (`m4_stable`, `v1x_preview`).
- A closed `locality_class`
  (`local`, `remote_helper`, `mirrored_pack`, `managed`).
- A closed `retrieval_epoch_state`
  (`current_epoch_aligned`, `epoch_mismatch_invalidated`,
  `mixed_generation_blocked`, `epoch_not_applicable`).
- An `embedder_identity` on every non-lexical row, binding the
  ARCH-RETR-010 generation tuple
  (model id, model version, tokenizer, chunker, retention policy,
  retrieval-epoch label).
- A verified `pack_signature` on mirrored-pack and managed rows.
- A `lane_participation` envelope that records participating and
  omitted lanes and discloses any policy-hidden omissions.
- A `chunk_or_anchor_provenance_ref` and, for hybrid rows, a
  `ranking_reason_ref` so the retrieval inspector can explain why
  a result was promoted and which lanes were omitted.
- A closed `downgrade_state`, `confidence_class`, evidence refs,
  and an in-surface `disclosure_ref`.

## How surfaces consume the packet

Eight consumer surfaces project this packet verbatim and preserve
the closed vocabularies plus embedder-identity and
lane-participation exposure: `search_shell`, `docs_help`,
`ai_context`, `review_workspace`, `cli_headless`,
`retrieval_inspector`, `support_export`, and
`release_proof_index`. The Rust contract refuses to certify a
projection that drops any of those vocabularies or that drops the
embedder-identity / participating-and-omitted-lanes exposure.

## Refusal posture

The contract refuses to certify stable when:

- A row leaks raw query text, raw source bodies, raw vectors,
  secrets, or ambient credentials past the boundary.
- A mirrored-pack or managed row drops its verified pack
  signature, or the signature state is `signature_mismatch_blocked`.
- A non-lexical row drops or malforms its `embedder_identity`
  (no anonymous managed/vector matches).
- A row claims `epoch_mismatch_invalidated` or
  `mixed_generation_blocked` while `downgrade_state` is `none`.
- A hybrid recall row drops its `ranking_reason_ref`.
- A row omits lanes but does not disclose the omission.
- A v1.x preview row claims `downgrade_state = none`.
- A required lane or surface track has no row.
- A required consumer projection is missing or collapses any
  closed vocabulary.

## Fixture corpus

The checked-in fixture corpus exercises one baseline stable
posture plus four narrowed-below-stable postures:

- `baseline_stable.json`
- `raw_query_text_leak_blocks_stable.json`
- `unsigned_mirrored_pack_blocks_stable.json`
- `mixed_generation_recall_blocks_stable.json`
- `policy_omissions_undisclosed_blocks_stable.json`

See
[`fixtures/search/m4/semantic_recall_boundary_truth_packet/`](../../../fixtures/search/m4/semantic_recall_boundary_truth_packet/)
for the corpus and
[`docs/search/m4/certify-docs-and-code-semantic-recall-boundaries.md`](../../../docs/search/m4/certify-docs-and-code-semantic-recall-boundaries.md)
for the full reviewer contract.
