# Search Planner Alpha Fixtures

These fixtures protect the shared planner contract used by quick open, file
search, and symbol search. Each case provides a query session plus the
lexical, structural, cached, or graph-backed snapshots available at planner
time, then asserts the selected path, readiness state, fallback disclosure,
and row-level explanation.

Covered paths:

- `quick_open_lexical_hot_set.json` proves hot lexical rows can answer before
  full indexing completes.
- `file_search_cached_fallback.json` proves cached rows remain labeled when
  the lexical index is unavailable.
- `symbol_search_structural_fallback.json` proves graph warm-up degrades to
  structural fallback without claiming semantic certainty.
- `symbol_search_graph_backed.json` proves graph-backed data wins when ready.
