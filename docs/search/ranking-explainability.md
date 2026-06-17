# Ranking explainers, withheld candidates, and query-debug sheets

This document describes the user-visible ranking-explainability layer that backs
the M5 search surfaces. Where the result-truth packet *owns* the structured
result identity and `RankingReason`, this layer promotes that explanation from
debug-only metadata into a first-class `Why this result?` sheet and makes
withheld or policy-hidden candidates inspectable instead of silently dropping
them.

- Schema: `schemas/search/ranking-explainability.schema.json`
- Packet model: `crates/aureline-search/src/ranking_explainability/mod.rs`
- Query-debug sheet: `crates/aureline-shell/src/search/search_debug_sheet.rs`
- Fixtures: `fixtures/search/m5/ranking-partiality/`

## Ranking explainers

Every visible row keeps a `RankingExplainSheet`. The sheet embeds the canonical
`RankingReason` verbatim — fact label, promoted signals, suppressed signals, and
tie-break class — and projects a user-facing headline and prose reason lines on
top of it. Because the same explanation object is embedded, the product UI,
support export, and replay/debug tooling all read **one** explanation rather than
reconstructing a private copy from rendered row text.

The sheet's `explain_state` names the presentation emphasis:

| State | Meaning |
| --- | --- |
| `promoted` | A positive signal lifted the row (exact match, recency/hot-set). |
| `suppressed` | A signal pushed a sibling down (e.g., a generated artifact). |
| `tied` | The order was settled by a tie-break (canonical source, recency, …). |
| `partial_index` | The row was answered off a partial, still-warming index. |

The guardrail is truthful reasons, not raw model weights: every sheet asserts
`raw_score_weights_excluded`, and the reason lines are prose drawn from the
closed signal vocabulary — never numeric scoring internals.

## Partiality and withheld candidates

A candidate that is **not** surfaced is recorded as an `OmittedCandidateRow`
instead of vanishing. Each omission carries the omission reason, a count, the
answering source stratum, and an optional recovery hint — but never literal
query text. Three omission states are claimed:

- `withheld_latency` — the answering lane exceeded its latency budget and some
  candidates were withheld to keep the surface responsive.
- `policy_hidden` — results in a restricted scope were hidden by the active
  trust/policy posture.
- `partial_index` — candidates are not yet indexed and may also match.

Policy-hidden and withheld candidates can therefore never be silent: a non-zero
`hidden_by_policy_rows` or `omitted_by_latency_budget_rows` scope counter on a
surface must be explained by a matching omission row.

## Distinguishing the six states consistently

Across its visible sheets and omitted rows the packet realizes every fact label
— `exact`, `context_promoted`, `semantic`, `partial_index`, `withheld_latency`,
`policy_hidden` — and every explainer state, so a row or session can always state
which kind of match (or omission) it represents.

## One explanation object, three consumers

Three first consumers reuse the same explain sheets and omitted rows verbatim:

| Consumer | Reuse contract |
| --- | --- |
| Product UI | Renders the explain sheets and omitted-candidate cues directly; the `Why this result?` and `Why withheld?` chrome reads one object. |
| Support export | Wraps the same metadata-only sheets, omission reasons, hashes, and counts so a reported ranking is explainable off the bundle. |
| Replay / debug | The CLI/headless inspect and query-debug sheet reuse the same sheets and omitted rows, so an inspect dump matches the product explanation. |

## Consent and export posture

The default export posture is `metadata_only`: hashes, counts, reason summaries,
and omission reasons are exportable, but literal query text is **not**. Literal
query text travels only when `export_consent` is `query_text_elevated`. The lane
never widens retention to capture raw query text by default.

## Auto-narrowing under a partial or stale index

When the live index is partial or stale, semantic coverage narrows (stronger
partiality caveats) and live-retrieval surfaces withhold strictly more
candidates for latency or pending indexing. Result identity, the explainer-state
vocabulary, and the reused explanation object are preserved unchanged, and
saved-query reopen — which reads local material — stays the same. Explainability
survives degraded state instead of collapsing into an opaque omission.
