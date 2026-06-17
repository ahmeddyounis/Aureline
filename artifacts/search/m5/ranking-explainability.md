# Review artifact — ranking explainers, withheld candidates, query-debug sheets

Packet id: `search.m5.ranking_explainability.v1`

This artifact is the reviewer-facing summary of the user-visible
ranking-explainability layer for the M5 search surfaces. It is produced from the
seeded packet and is metadata-only.

## What this lane delivers

- User-visible `Why this result?` explain sheets across palette, sidebar, docs
  results, graph-backed results, and saved-query reopen. Each sheet embeds the
  canonical `RankingReason` verbatim and headlines a `promoted`, `suppressed`,
  `tied`, or `partial_index` explainer state.
- Inspectable omitted-candidate rows for `withheld_latency`, `policy_hidden`, and
  `partial_index` candidates, each carrying an omission reason, a count, and the
  answering source stratum — never literal query text.
- A query-debug sheet projection (`crates/aureline-shell/src/search/search_debug_sheet.rs`)
  that reuses the same explain sheets and omitted rows for the desktop and for
  CLI/headless replay.

## Acceptance evidence

| Acceptance criterion | Evidence |
| --- | --- |
| Rows distinguish exact, context-promoted, semantic, partial-index, withheld-latency, and policy-hidden states | `covered_fact_labels` and `covered_states` cover all six, and every one is realized by a sheet or omission row. |
| The same ranking explanation object is reused by UI, support export, and replay/debug | Each sheet embeds the canonical `RankingReason`; the three consumer projections set `preserves_explain_sheets/omitted_candidates/counts_and_hashes=true`. |
| Policy-hidden and withheld candidates are no longer silent omissions | A non-zero `hidden_by_policy_rows` / `omitted_by_latency_budget_rows` must be explained by a matching omitted-candidate row. |

## Guardrails enforced (fail-closed)

- A visible explain sheet may not headline a withheld or policy-hidden state, and
  its headlined state must agree with the embedded ranking reason.
- Result identity must be a durable URN; it may not collapse into a display label
  or a transient list index.
- Truthful reasons, not raw model weights: every sheet asserts
  `raw_score_weights_excluded` and carries prose reason lines.
- Omitted rows exclude literal query text; the packet stays `metadata_only` and
  literal query text travels only under elevated consent.

## Sources

- Contract doc: `docs/search/ranking-explainability.md`
- Schema: `schemas/search/ranking-explainability.schema.json`
- Fixtures: `fixtures/search/m5/ranking-partiality/`
- Model + tests: `crates/aureline-search/src/ranking_explainability/`
