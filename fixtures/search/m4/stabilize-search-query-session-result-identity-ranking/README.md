# Stabilized Search Query Session and Result Identity Fixtures

This corpus proves the stable search contract stays on one schema family across
query sessions, result identity, ranking explanations, action bindings,
saved-query reopen, search deep links, and support export.

Cases:

- `baseline_contract_packet.json` covers a local-first query session whose
  support export redacts literal query text, preserves the stable
  `query_session_id`, records parsed query/filter evidence, keeps dedupe
  lineage, and marks the exported result set as a captured snapshot.
