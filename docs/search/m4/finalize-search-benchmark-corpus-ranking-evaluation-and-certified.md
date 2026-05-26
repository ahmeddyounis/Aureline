# Search benchmark corpus, ranking evaluation, and certified-archetype query packs — stable contract

Status: Stable lane proof for the M4 search benchmark corpus, ranking
evaluation, and certified-archetype query-pack truth packet.

This document is the reviewer-facing contract for the stable
benchmark-corpus packet. The packet is the single source of truth that
the search shell, docs/help, CLI/headless inspector, support export,
release proof index, and benchmark-lab dashboard all read; surfaces
MUST NOT mint local copies or paraphrase status text.

## What the packet asserts

For each governed *certified archetype × benchmark-corpus class* row,
the packet asserts:

1. A **stable query pack** (`query_pack_id`) of a closed
   `query_pack_class` (`golden_query_pack`, `regression_query_pack`,
   `edge_case_query_pack`, `imported_corpus_pack`) plus a `query_count`.
2. A **stable corpus binding** (`corpus_id_ref`) to a packet-defined
   `BenchmarkCorpusDefinition` carrying its own
   `retention_policy_class`, `provenance_class`, `sample_size`,
   `source_ref`, and (when imported) an `imported_provenance_ref`.
3. A **closed metric vocabulary** of ranking-evaluation observations
   (`ndcg_at_10`, `mrr`, `recall_at_50`, `precision_at_5`,
   `first_useful_row_latency_ms`), each carrying a published baseline,
   an observed value, a sample size, a benchmark-lab capture ref, and an
   optional waiver ref.
4. A **confidence class** for the row's evaluation
   (`high_confidence`, `medium_confidence`, `low_confidence`).
5. A **downgrade state** (`none`, `narrowed_below_baseline`,
   `imported_baseline`, `regression_detected`, `corpus_redacted`) plus a
   disclosure ref whenever the state is not `none`.
6. A **raw-material exclusion** flag on every row and every corpus;
   stable packets never admit raw query text, raw source bodies, raw
   corpus payloads, secrets, or ambient credentials.

## Closed vocabulary

**Certified archetypes** — `typescript_javascript_web`,
`python_service_or_data_app`, `rust_workspace`,
`go_service_or_monorepo_slice`, `java_or_kotlin_service`,
`c_or_cpp_native_project`.

**Benchmark corpus classes** — `file_lookup_corpus`,
`symbol_navigation_corpus`, `command_palette_corpus`,
`docs_lookup_corpus`, `semantic_recall_corpus`,
`hybrid_retrieval_corpus`.

**Query-pack classes** — `golden_query_pack`,
`regression_query_pack`, `edge_case_query_pack`,
`imported_corpus_pack`.

**Ranking metrics** — `ndcg_at_10`, `mrr`, `recall_at_50`,
`precision_at_5`, `first_useful_row_latency_ms`. Ratio metrics are
expressed in basis points (10000 = 1.0). The latency metric is
expressed in milliseconds.

**Retention policy classes** — `local_only`, `tenant_only`,
`shared_internal`, `published_external`.

**Provenance classes** — `internally_authored`, `imported_external`,
`community_contributed`, `synthesized_seed`.

**Required consumer projections** — `search_shell`, `docs_help`,
`cli_headless`, `support_export`, `release_proof_index`,
`benchmark_lab`. Each projection MUST preserve the same packet id,
metric vocabulary, query-pack vocabulary, corpus provenance vocabulary,
and downgrade-state vocabulary; MUST support JSON export; and MUST
exclude raw private material and ambient authority.

## Promotion states

A materialized packet is one of:

- `stable` — every row meets its baseline, declares a disclosure when
  required, carries a defined corpus binding with consistent
  provenance, and every required projection preserves the packet
  verbatim.
- `narrowed_below_stable` — a warning-class finding is present (for
  example, a metric waiver is exercised and the row is intentionally
  narrowed below stable until the waiver clears).
- `blocks_stable` — a blocker finding is present (for example, an
  observed metric regressed against baseline without a waiver, a row
  references an undefined corpus, a downgraded row drops its
  disclosure ref, an imported-corpus-pack row references a corpus that
  is not `imported_external`, a projection collapses any of the closed
  vocabularies, or raw query material is admitted past the boundary).

## Why this matters

The track invariant for this lane is *keep search, graph, and docs
surfaces useful before fully warm and explicit about scope, freshness,
provenance, and downgrade state at all times*. The packet's validation
rules implement that invariant directly: a stable ranking-evaluation
row cannot ship with an unlabeled regression, with an undefined or
mismatched corpus binding, with an imported pack that masquerades as
internally authored, with a projection that collapses the metric or
provenance vocabulary, or with raw query material on the boundary.
When delivery proves a narrower stable claim — for example, when an
imported baseline replaces an internal capture for a row — the packet
narrows below stable instead of inheriting the adjacent green rows.

## Where the packet lives

- Schema: `schemas/search/search_benchmark_corpus_truth.schema.json`
- Reviewer artifact: `artifacts/search/m4/finalize-search-benchmark-corpus-ranking-evaluation-and-certified.md`
- Stable packet artifact: `artifacts/search/m4/search_benchmark_corpus_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/search_benchmark_corpus_truth/`
- Rust module: `crates/aureline-search/src/search_benchmark_corpus_truth/mod.rs`

### Anchors

#### regression

The `regression_detected` downgrade-state disclosure: an observed
metric regressed against the published baseline. Until a waiver is
attached or the regression is recovered, the row narrows below stable
and surfaces show the regression alongside the captured benchmark
reference.

#### imported-baseline

The `imported_baseline` downgrade-state disclosure: the row inherits
its published baseline from an imported corpus. Surfaces MUST display
the imported provenance ref so reviewers can distinguish internal
captures from externally imported baselines.

#### corpus-redacted

The `corpus_redacted` downgrade-state disclosure: the underlying
benchmark corpus is held back by a retention policy and only the
ranking-metric numerics are exported. The disclosure ref points the
reader to the retention rule that gates the corpus.
