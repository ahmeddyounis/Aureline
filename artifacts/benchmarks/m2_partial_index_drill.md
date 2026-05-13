# Alpha Partial-Index Search Drill Packet

This packet is the benchmark-facing consumer for the monorepo partial-index
and large-workspace search fixtures. It is methodology and fixture validation
evidence, not a published performance comparison.

## Packet Header

| Field | Value |
|---|---|
| Packet id | `benchmark.search.partial_index_drill.alpha` |
| Packet state | `fixture_drill_passed` |
| Captured at | `2026-05-13T17:30:00Z` |
| Fixture register | `artifacts/benchmarks/m2_fixture_register.yaml` |
| Corpus manifest revision | `1` |
| Validator | `cargo test -p aureline-search --test partial_index_drill_cases` |
| Raw workspace bytes | `excluded; described-count synthetic fixtures only` |

## Drill Results

| Drill result id | Fixture | States exercised | Result | First useful result | Full index required |
|---|---|---|---|---:|---|
| `benchmark.drill.search.tsjs.partial_index.self_capture` | `search.large_workspace.tsjs.partial_index` | `partial_index`, `stale_shard`, `hidden_scope` | `pass` | 74 ms | no |
| `benchmark.drill.search.python.hidden_scope.self_capture` | `search.large_workspace.python.hidden_scope` | `partial_index`, `hidden_scope` | `pass` | 89 ms | no |

## Fixture Binding

| Fixture register row | Corpus refs | Large-workspace drill |
|---|---|---|
| `fixture_register:external_alpha.ts_web_app_reference` | `corpus.reference.ts_web_app_archetype_seed`, `corpus.archetype.ts_web_app_seed` | `fixtures/search/large_workspace_alpha/tsjs_monorepo_partial_index_drill.json` |
| `fixture_register:external_alpha.python_service_data_reference` | `corpus.reference.python_data_app_archetype_seed`, `corpus.archetype.python_data_app_seed` | `fixtures/search/large_workspace_alpha/python_polyrepo_hidden_scope_drill.json` |

## Protected States

| State | Proof fixture | Required truth |
|---|---|---|
| `partial_index` | `fixtures/search/monorepo_partial_index/partial_index_hot_set_checkout.json` | Hot-set and lexical rows are usable, but full-workspace certainty is narrowed. |
| `stale_shard` | `fixtures/search/monorepo_partial_index/stale_shard_branch_switch.json` | Cached rows remain openable only with stale labels and unsafe broad actions blocked. |
| `hidden_scope` | `fixtures/search/monorepo_partial_index/hidden_scope_sparse_workset.json` | Hidden counts and omitted scopes stay inspectable from the search surface. |

## Verification

```sh
cargo test -p aureline-search --test partial_index_drill_cases
```

The validator reads the fixture packets directly, materializes the indexed
lane state through `aureline-search`, checks support-export narrowing, confirms
large-workspace drills are described-count and automation-only, and verifies
that this packet contains at least one result from the new drill fixtures.
