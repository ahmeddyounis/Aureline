# search_benchmark_corpus_truth fixture corpus

Fixture corpus for the M4 stable search benchmark corpus, ranking
evaluation, and certified-archetype query-pack truth packet
(`schemas/search/search_benchmark_corpus_truth.schema.json`).

Each fixture is a `SearchBenchmarkCorpusTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, archetype and corpus-class token sets, query-pack
token set, metric token set, downgrade-state tokens, retention-policy
tokens, provenance tokens, and exported support posture. Tests in
`crates/aureline-search/tests/search_benchmark_corpus_truth_cases.rs`
load each case and assert that
`SearchBenchmarkCorpusTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — A Rust workspace file-lookup golden pack hits
  every published baseline, the corpus is internally authored with a
  shared-internal retention policy, and every required consumer
  projection preserves the packet verbatim.
- `metric_regression_without_waiver_blocks_stable.json` — A
  Java/Kotlin symbol-navigation golden pack's observed nDCG@10 regresses
  against its baseline with no waiver ref; the packet blocks the stable
  claim.
- `imported_pack_without_provenance_blocks_stable.json` — A
  TypeScript/JavaScript imported_corpus_pack references a corpus whose
  provenance is `internally_authored`; the packet blocks the stable
  claim.
- `corpus_redacted_undisclosed_blocks_stable.json` — A Rust
  semantic-recall golden pack carries `corpus_redacted` downgrade state
  with no disclosure ref; the packet blocks the stable claim.
- `metric_vocabulary_collapsed_blocks_stable.json` — A Go
  service file-lookup golden pack ships with a docs/help projection
  that drops the metric vocabulary; the packet blocks the stable claim.
