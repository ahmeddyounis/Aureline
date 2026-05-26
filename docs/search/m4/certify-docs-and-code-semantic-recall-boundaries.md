# Docs/code semantic-recall boundary — stable contract

Status: Stable lane proof for the docs and code semantic-recall
boundary between the M4 stable lane and the v1.x preview lane.

This document is the reviewer-facing contract for the stable
docs/code semantic-recall boundary truth packet. The packet is the
single source of truth that the search shell, docs/help surface,
AI context assembly, review workspace, CLI/headless inspector, the
in-product retrieval inspector, support export bundle, and the
release proof index all read; surfaces MUST NOT mint local copies
or paraphrase recall posture.

## What the packet asserts

For each governed *(recall lane × surface track × locality)* row,
the packet asserts:

1. The **recall lane class** — one of `docs_semantic_recall`,
   `code_semantic_recall`, `hybrid_fused_recall`, or
   `lexical_only_fallback`. Every certified packet MUST carry at
   least one row for each of the four required lane classes.

2. The **surface track** — one of `m4_stable` or `v1x_preview`.
   Every certified packet MUST cover both tracks so the boundary
   is documented, not implied. v1.x preview rows MUST carry a
   recorded `downgrade_state` (any value other than `none`) so
   they cannot inherit adjacent green/stable rows.

3. The **locality class** — one of `local`, `remote_helper`,
   `mirrored_pack`, or `managed`. Mirrored-pack and managed rows
   MUST carry a `pack_signature` whose `signature_state` is
   `signed_and_verified`. Pack-based and mirrored semantic recall
   MUST be signed, versioned, and downgrade-aware so offline or
   mirrored recall cannot masquerade as current managed/local
   recall when compatibility or freshness drifts.

4. The **retrieval-epoch state** — one of
   `current_epoch_aligned`, `epoch_mismatch_invalidated`,
   `mixed_generation_blocked`, or `epoch_not_applicable`
   (lexical-only fallback). Per ARCH-RETR-010, changing the
   embedder model, tokenizer, chunking strategy, or retention
   policy advances the retrieval epoch and invalidates affected
   semantic caches/indexes rather than silently mixing
   generations. Rows whose epoch state is
   `epoch_mismatch_invalidated` or `mixed_generation_blocked` MUST
   record a non-`none` downgrade state.

5. The **embedder identity** — every non-lexical row MUST carry
   an `embedder_identity` block that pins
   `embedder_model_id`, `embedder_model_version`, `tokenizer_id`,
   `chunking_strategy_id`, `retention_policy_id`, and the derived
   `retrieval_epoch_label`. Managed and vector matches that drop
   their embedder identity fail the
   `unlabeled_managed_or_vector_match` and
   `missing_embedder_identity` blockers, so the retrieval
   explainability corpus reaches zero unlabeled managed/vector
   matches.

6. The **lane participation** — every row carries the
   `participating_lane_refs` and `omitted_lane_refs` lists plus a
   `policy_hidden_omissions_disclosed` boolean. Rows that omit
   lanes MUST also disclose those omissions through
   `policy_hidden_omissions_disclosure_ref` so users can inspect
   why hybrid recall promoted a result and which lanes were
   omitted without leaving the product.

7. The **chunk-or-anchor provenance ref** — every row carries a
   repo-relative ref to its chunk-or-anchor provenance evidence.

8. The **ranking-reason ref** — `hybrid_fused_recall` rows MUST
   carry a `ranking_reason_ref` so the retrieval inspector and AI
   context can explain *why* a hybrid recall row was promoted.

9. The **downgrade state** — one of `none`,
   `locality_downgraded_to_local`,
   `embedder_unavailable_fallback_lexical`,
   `epoch_drift_invalidated`, `pack_signature_failed`,
   `policy_omitted_candidates`, `mixed_generation_blocked`, or
   `compatibility_drift_disclosed`.

10. The **confidence class** — one of `high`, `medium`, `low`, or
    `heuristic`.

11. The **disclosure ref** — every row carries a repo-relative
    reference to the in-surface disclosure shown to the user,
    plus at least one `evidence_refs` entry that proves the
    recall claim.

12. The **redaction guarantees** — every row asserts
    `raw_query_text_excluded`, `secrets_excluded`,
    `ambient_authority_excluded`, and `raw_source_bodies_excluded`.
    A row that admits raw query text, raw source bodies, raw
    vectors, secrets, or ambient authority/credentials fails to
    certify stable.

## Required consumer projections

Every certified packet MUST carry a consumer projection for each
of the eight required surfaces:

| Surface               | Token                |
| --------------------- | -------------------- |
| Search shell          | `search_shell`       |
| Docs/help surface     | `docs_help`          |
| AI context assembly   | `ai_context`         |
| Review workspace      | `review_workspace`   |
| CLI / headless        | `cli_headless`       |
| Retrieval inspector   | `retrieval_inspector`|
| Support export        | `support_export`     |
| Release proof index   | `release_proof_index`|

Each projection MUST preserve the same packet id, the recall-lane
vocabulary, surface-track vocabulary, locality vocabulary,
retrieval-epoch vocabulary, pack-signature vocabulary, downgrade
vocabulary, embedder-identity exposure, lane-participation
exposure, JSON export support, and the redaction guarantees on
raw private material and ambient authority. Any projection that
collapses one of those vocabularies, drops embedder identity, or
drops the participating/omitted lane exposure fails to certify
stable.

## Validator vocabulary

The Rust contract emits a closed `FindingKind` set; the schema
mirrors it. Notable blockers:

- `raw_query_text_present`, `raw_source_bodies_present`,
  `secrets_present`, `ambient_authority_present` — refuse to
  certify when raw material crosses the boundary.
- `missing_pack_signature`, `pack_signature_not_verified` —
  refuse to certify mirrored or managed recall without a
  verified pack signature.
- `epoch_mismatch_presented_as_current` — refuse to certify when
  a row's epoch state is `epoch_mismatch_invalidated` or
  `mixed_generation_blocked` while its downgrade state is `none`.
- `unlabeled_managed_or_vector_match`,
  `missing_embedder_identity` — refuse to certify managed or
  vector matches that drop their embedder identity.
- `missing_ranking_reason` — refuse to certify hybrid recall rows
  without a ranking-reason ref.
- `policy_omissions_undisclosed` — refuse to certify when omitted
  lanes are not disclosed.
- `preview_row_claims_m4_stable_certainty` — refuse to certify a
  v1.x preview row that claims no downgrade.
- `embedder_identity_dropped`, `lane_participation_dropped` —
  refuse to certify when a consumer projection drops embedder
  identity or the participating/omitted lane exposure.

## Pinned artifacts

- Rust contract:
  [`crates/aureline-docs/src/semantic_recall_boundary_truth_packet/mod.rs`](../../../crates/aureline-docs/src/semantic_recall_boundary_truth_packet/mod.rs)
- Schema:
  [`schemas/docs/semantic_recall_boundary_truth.schema.json`](../../../schemas/docs/semantic_recall_boundary_truth.schema.json)
- Checked-in stable packet:
  [`artifacts/search/m4/semantic_recall_boundary_truth_packet.json`](../../../artifacts/search/m4/semantic_recall_boundary_truth_packet.json)
- Fixture corpus:
  [`fixtures/search/m4/semantic_recall_boundary_truth_packet/`](../../../fixtures/search/m4/semantic_recall_boundary_truth_packet/)
- Milestone-level note:
  [`docs/m4/certify-docs-and-code-semantic-recall-boundaries.md`](../../m4/certify-docs-and-code-semantic-recall-boundaries.md)

The checked-in stable packet covers all four required recall lanes
across both surface tracks, all eight required consumer surfaces,
and exercises every locality (`local`, `remote_helper`,
`mirrored_pack`, `managed`).
