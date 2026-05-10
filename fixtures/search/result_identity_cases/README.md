# Result Identity Fixture Cases

Worked `result_identity_case` fixtures backing
[`/docs/search/result_identity_and_ranking.md`](../../../docs/search/result_identity_and_ranking.md)
and the lexical row identity built by `aureline_search::build_lexical_identity`.

Each fixture seeds a workspace lifecycle, scope, file list, and query and
asserts the exact `result_id`, `ranking_reasons`, and `partiality_class` the
shell MUST attach to every row. The fixture-driven test
(`crates/aureline-search/tests/result_identity_cases.rs`) loads every JSON
file and replays the projection so the protected truth vocabulary cannot
drift without a fixture update.

| Fixture | Coverage |
|---|---|
| `ready_exact_basename_identity.json` | Ready provider, exact-basename match, authoritative partiality, no row caveat. |
| `warming_partial_label_identity.json` | Failure-drill: partially-ready provider streams rows; row identity carries both `prefix_basename_match` and `partial_coverage_caveat`, partiality class is `partial`. |
| `stale_lexical_label_identity.json` | Stale (heuristic / cached) provider; row identity stays exportable but partiality class is `stale` and the row carves out a `partial_coverage_caveat`. |
| `generated_artifact_deprioritized_identity.json` | Generated lockfile row picks up `generated_artifact_deprioritized` reason while the canonical sibling does not. |

Every fixture preserves the URN-style `result_id` so reviewers can confirm
two rows for the same path on different lanes (filename vs. path) keep
distinct identities, and one row reproduced in two ranking passes keeps the
same identity.
