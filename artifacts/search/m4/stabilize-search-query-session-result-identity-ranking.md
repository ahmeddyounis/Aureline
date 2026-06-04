# Stabilize Search Query Session, Result Identity, Ranking, and Export Contracts

## Proof Summary

The stabilized path uses one contract family across search shell, docs/help,
AI context inspector, CLI/headless, saved-query reopen, deep links, replay
fixtures, and support export.

Machine-readable proof:

- Query session: `SearchQuerySession` carries the same `query_session_id`
  through planner output, result packets, saved-query records, history rows,
  deep links, collection snapshots, and support exports.
- Result identity: `SearchResultRef` preserves result id, kind, canonical
  object refs, anchor/span, snapshot/commit/worktree ref, freshness,
  confidence, and dedupe lineage.
- Ranking explanation: `RankingReason` preserves promoted and suppressed
  signals, fact label, tie-break class, withheld-candidate note, and
  partiality note.
- Action binding: `SearchActionBinding` preserves open target, alternate
  behaviors, required surface capabilities, fallback mode, and history
  policy.
- Export/handoff: `SearchExportPacket` and `SearchCollectionSnapshot`
  preserve selected/included result refs, hidden/partial counts,
  redaction state, captured-vs-live truth, omitted/truncated flags, and
  evidence refs.

## Privacy Posture

Local history may retain raw query text only under local-only privacy. Shared,
docs, support, and managed retention projections use hash-only or omitted
query material. Deep links carry intent plus scope metadata and never ambient
data access.

## Fixture Corpus

`fixtures/search/m4/stabilize-search-query-session-result-identity-ranking/baseline_contract_packet.json`
contains a reviewable packet joining a parsed query session, result identity,
ranking explanation, action binding, saved-query privacy posture, deep-link
rerun semantics, and captured support-export snapshot truth.
