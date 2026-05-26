# Finalize search benchmark corpus, ranking evaluation, and certified-archetype query packs — proof packet

This reviewer artifact accompanies the stable
[`search_benchmark_corpus_truth_packet.json`](search_benchmark_corpus_truth_packet.json)
and is the human-readable proof for the M4 stable lane that the
search benchmark corpus, ranking evaluation, and certified-archetype
query packs are bound to one knowledge-plane truth packet.

## Stable claim

Every certified archetype on the M4 stable lane has at least one
certified golden query pack per governed benchmark-corpus class
(`file_lookup_corpus`, `symbol_navigation_corpus`,
`docs_lookup_corpus`, `semantic_recall_corpus`). Each row pins:

- a stable `query_pack_id` and `corpus_id_ref` bound to a
  packet-defined `BenchmarkCorpusDefinition`,
- a per-metric capture covering ranking quality
  (`ndcg_at_10`, `mrr`, `recall_at_50`, `precision_at_5`) and
  first-useful-row latency (`first_useful_row_latency_ms`),
- a `confidence_class` reflecting sample-size strength, and
- a `downgrade_state` that narrows below stable when corpus material is
  redacted, when an imported baseline is in play, or when a regression
  is detected — always with a disclosure ref when the state is not
  `none`.

## Required consumer projections

The packet is preserved verbatim across six consumer projections:

| Projection            | Surface                                                                |
| --------------------- | ---------------------------------------------------------------------- |
| `search_shell`        | Search shell quick-open, file, symbol, command, and benchmark surfaces |
| `docs_help`           | Docs/help explaining corpus retention, provenance, and waivers         |
| `cli_headless`        | CLI/headless inspector                                                 |
| `support_export`      | Support export bundle                                                  |
| `release_proof_index` | Release proof index entry                                              |
| `benchmark_lab`       | Benchmark-lab dashboard and capture surface                            |

A projection that collapses any closed vocabulary, drops the packet
id, drops query-session/corpus provenance, or leaks raw private
material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row's observed metric regressed against the published baseline
  with no waiver ref,
- a row references a `corpus_id_ref` that the packet does not define,
- a row's archetype or corpus-class binding disagrees with the
  referenced corpus,
- a row keeps a non-`none` downgrade state but drops the disclosure
  ref,
- an `imported_corpus_pack` row references a corpus whose provenance
  is not `imported_external`,
- a corpus is `imported_external` but has no `imported_provenance_ref`,
- any of the six required consumer projections is missing or collapses
  one of the closed vocabularies,
- raw query text, source bodies, secrets, or ambient credentials slip
  past the boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`SearchBenchmarkCorpusTruthPacket::materialize` and then read the
projection that matches their surface. The packet is metadata-only and
suitable for inclusion in any support export or release proof bundle.

## Where the packet lives

- Schema: [`schemas/search/search_benchmark_corpus_truth.schema.json`](../../../schemas/search/search_benchmark_corpus_truth.schema.json)
- Reviewer doc: [`docs/search/m4/finalize-search-benchmark-corpus-ranking-evaluation-and-certified.md`](../../../docs/search/m4/finalize-search-benchmark-corpus-ranking-evaluation-and-certified.md)
- Fixture corpus: [`fixtures/search/m4/search_benchmark_corpus_truth/`](../../../fixtures/search/m4/search_benchmark_corpus_truth/)
- Rust module: [`crates/aureline-search/src/search_benchmark_corpus_truth/mod.rs`](../../../crates/aureline-search/src/search_benchmark_corpus_truth/mod.rs)
