# M5 AI/docs/recall row certification — memory classes, prompt-result-cache and session-artifact governance, hybrid retrieval or embedding locality, and spend receipts

This document is the contract for the recall-row certification capstone of the AI
memory, prompt-result-cache, hybrid-retrieval, embedding-locality, and
spend-receipt batch. It binds **every claimed M5 AI/docs/recall row** to four
interlocking proof pillars and makes them release-bearing, so Milestone 5 can ship
this depth area with canonical implementation, proof, downgrade behavior, and
operator-facing truth instead of ad hoc prototypes, side spreadsheets, or feature
copy that outruns evidence.

The packet is canonical: no product, help, diagnostics, or release surface may
present a greener claim than this certification, and any row that lacks current
memory-class, cache/locality, or spend-receipt proof auto-narrows before it
publishes.

## Source of truth

- Packet type: `M5RecallRowCertificationPacket`
  (`crates/aureline-ai/src/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/`).
- Boundary schema:
  `schemas/ai/certify-ai-memory-classes-prompt-result-cache-and-session-artifact-governance-hybrid-retrieval-or-embedding-locality-and.schema.json`.
- Checked support export:
  `artifacts/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/support_export.json`.
- Markdown summary:
  `artifacts/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and.md`.
- Protected fixtures:
  `fixtures/ai/m5/certify_ai_memory_classes_prompt_result_cache_and_session_artifact_governance_hybrid_retrieval_or_embedding_locality_and/`.
- Conformance dump: `cargo run -p aureline-ai --example dump_m5_recall_row_certification [support|fixture|summary]`.

## Certified surfaces

Each claimed AI/docs/recall surface carries one certified row:

`composer_inline_assist`, `patch_review`, `branch_worktree_agent`,
`docs_browser_recall`, `code_understanding`, `semantic_hybrid_search`,
`managed_offline_report`, and `support_export`.

## Proof pillars

Every row carries exactly one proof per pillar. Each pillar is bound to the
canonical source schema of the first-consumer packet that materializes it, and
that schema is required in `source_contract_refs` so a pillar can never claim more
than its source admits.

| Pillar | Canonical source |
| --- | --- |
| `memory_class` | turn/thread/workspace/org memory-class materialization schema (and the cross-scope memory-fence schema) |
| `prompt_cache_session_artifact` | prompt-composer draft and session-artifact records schema |
| `hybrid_retrieval_locality` | retrieval-locality inspector schema and reusable semantic-memory/embedding-index schema |
| `spend_receipt` | spend-and-route receipt (AI run receipt) schema |

Each [`PillarProof`] declares its [`ProofState`] (`current`, `stale`, or
`missing`), the [`LocalityClass`] where the data lived (`local_on_device`,
`managed_hosted`, `mirrored_offline`, or `mixed_labeled`), whether a durable
artifact declares its retention/delete/export posture, and whether a mixed
retrieval generation is labeled rather than passed off as current.

## Auto-narrowing gate

A claimed row may not outrun current proof:

- If every pillar is `current`, the `effective_qualification` equals the
  `claimed_qualification`.
- If any pillar is `stale` or `missing`, the `effective_qualification` must rank
  strictly below the claim, the row records a narrowing trigger
  (`narrow_trigger`), and it carries a precise, non-generic `degraded_label`.

`M5RecallRowCertificationPacket::validate` rejects a packet that:

- omits a required AI/docs/recall surface, or demonstrates no auto-narrowing case;
- has a row that does not carry exactly one proof per pillar, or names a schema
  outside the pillar's canonical sources;
- hides managed or mixed locality, mixes retrieval generations without labeling,
  or leaves a durable pillar without a retention/delete/export posture;
- keeps a public claim while a pillar's proof is stale or missing without
  narrowing the row below its claim;
- fails any guardrail, consumer-projection, or proof-freshness invariant; or
- carries raw boundary material in the export.

## Guardrails

- No cross-workspace or cross-tenant recall happens by default.
- Prompt-result caches never behave like shadow-telemetry stores.
- Mixed-generation retrieval is always labeled, never passed as current.
- Managed locality is always disclosed, never implied local.
- Every durable artifact declares its retention/delete/export posture.
- Spend or route failures keep a precise fallback rather than a generic error.
- Any row lacking current pillar proof auto-narrows below its claim.

## Consumers

Product, docs/help, diagnostics, and release surfaces ingest this certification
result directly instead of cloning AI-memory or retrieval behavior text by hand,
and they label narrowed rows below current in every surface.
