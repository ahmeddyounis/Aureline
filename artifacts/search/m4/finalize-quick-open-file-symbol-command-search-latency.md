# Quick-open / file / symbol / command-palette latency truth — proof packet

## Summary

The stable search-latency packet at
`artifacts/search/m4/quick_open_latency_truth_packet.json` covers every
combination of the six certified archetypes and four governed latency
surfaces:

| Archetype                         | quick_open | file_search | symbol_search | command_palette |
|-----------------------------------|-----------:|------------:|--------------:|----------------:|
| TypeScript / Javascript web       |   28 / 95  |   88 / 305  |   142 / 540   |    20 / 72      |
| Python service or data app        |   26 / 102 |   82 / 290  |   130 / 488   |    19 / 70      |
| Rust workspace                    |   24 / 95  |   84 / 288  |   128 / 472   |    18 / 68      |
| Go service or monorepo slice      |   27 / 99  |   90 / 310  |   138 / 520   |    19 / 70      |
| Java / Kotlin service             |   30 / 108 |   96 / 318  |   152 / 565   |    21 / 74      |
| C / C++ native project            |   31 / 110 |  102 / 332  |   168 / 588   |    22 / 80      |

Numbers are observed p50 / p95 in milliseconds. The published budgets per
surface are:

- `quick_open`: 40 / 120 ms
- `file_search`: 120 / 350 ms
- `symbol_search`: 180 / 600 ms
- `command_palette`: 30 / 90 ms

Every row stays within budget without waivers and labels its partial-index
truth class.

## How the packet stays useful before fully warm

Every non-`fully_indexed` row in the packet keeps its partial-index state
visible (`hot_set_only`, `partial_index`) and references a disclosure
anchor in
`docs/search/m4/finalize-quick-open-file-symbol-command-search-latency.md`.
Quick-open rows expose a `warming → hot_set_ready` transition at 14–21 ms
so the user sees a first useful row before the cold lane finishes
indexing.

## How the packet protects budgets

The validator emits `observed_p50_exceeded_budget` /
`observed_p95_exceeded_budget` blockers when an observed latency exceeds
its published budget without a `waiver_ref`. The narrowed corpus case
`fixtures/search/m4/quick_open_latency_truth/budget_breach_blocks_stable.json`
exercises this path: a Java/Kotlin symbol-search row at 220 / 990 ms blocks
the stable claim.

## How the packet protects session truth across projections

The packet binds a durable `query_session_id` per row. Each of the five
required consumer projections (`search_shell`, `docs_help`,
`cli_headless`, `support_export`, `release_proof_index`) must preserve
the same packet id, query-session ids, readiness states, and
partial-index labels and must support JSON export. The narrowed corpus
case
`fixtures/search/m4/quick_open_latency_truth/session_state_collapsed_narrowed.json`
exercises the inverse: a docs/help projection that flips
`preserves_readiness_states` to false emits both
`consumer_projection_drift` and `session_state_collapsed` blockers.

## How the packet protects partial-index honesty

Every row with non-`fully_indexed` truth must reference a
`partial_index_disclosure_ref` in product copy and exports. The narrowed
corpus case
`fixtures/search/m4/quick_open_latency_truth/partial_index_unlabeled_narrowed.json`
exercises the inverse: a TypeScript/Javascript file-search row that drops
its disclosure ref narrows below stable with `partial_index_not_labeled`.

## Source contract refs

- Schema: `schemas/search/quick_open_latency_truth.schema.json`
- Reviewer doc: `docs/search/m4/finalize-quick-open-file-symbol-command-search-latency.md`
- Rust implementation: `crates/aureline-search/src/quick_open_latency_truth/mod.rs`
- Fixture corpus: `fixtures/search/m4/quick_open_latency_truth/`
- Certified-archetype scorecards: `artifacts/compat/m3/archetype_scorecards/scorecard_index.yaml`
