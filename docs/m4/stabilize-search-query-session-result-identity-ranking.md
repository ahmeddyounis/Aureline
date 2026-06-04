# Stabilized Search Query Session, Result Identity, Ranking, and Export Contracts

This row stabilizes the machine-readable contracts used by search, docs,
AI context assembly, CLI/headless output, saved-query reopen, deep links,
and support export.

The canonical Rust contracts live in `aureline-search`:

- `query_session` owns `SearchQuerySession`, including session identity,
  initiating surface, query text mode, redaction-safe parsed query/filter
  evidence, scope/workset refs, planner version, index/graph epochs,
  policy posture, lifecycle timing, and readiness state.
- `result_truth_packet` owns `SearchResultRef`, `RankingReason`,
  `SearchActionBinding`, scope counters, consumer projections, and the
  stable result-truth packet.
- `session_ledger` and `query_artifacts` own saved queries, query history,
  scope bindings, search deep links, collection snapshots, support/docs
  export packets, redaction state, captured-vs-live truth, omitted/truncated
  flags, and evidence refs.

Raw query text, regexes, customer identifiers, incident terms, and provider
filters remain local-only by default. Support and docs exports carry hashes,
scope summaries, result refs, hidden/partial counts, omitted/truncated
flags, and evidence refs unless literal export is explicitly admitted by
policy or consent.

Deep links and handoff packets reopen search intent. They preserve scope
metadata and rerun requirements, but recipients re-resolve results under
their own current permissions and trust posture.

Source refs:

- `schemas/search/search_result_truth_packet.schema.json`
- `schemas/search/search_export_snapshot.schema.json`
- `docs/search/m4/result-identity-ranking-reasons-and-export-packets.md`
- `artifacts/search/m4/stabilize-search-query-session-result-identity-ranking.md`
- `fixtures/search/m4/stabilize-search-query-session-result-identity-ranking/`
