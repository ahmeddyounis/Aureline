# Stable publication pack fixture cases

Negative fixtures for `ci/check_stable_publication_pack.py`. Each `*.json` file is a
complete stable publication pack that is structurally valid **except for one targeted
flaw**, paired in [`cases.json`](./cases.json) with the check id its flaw must trip.

The gate runs every case during `--check` and fails when a case marked `rejected`
validates clean or trips a different check than expected, so the publication-narrowing and
benchmark-budget-protection rejections stay live even as the canonical pack
(`artifacts/release/stable_publication_pack.json`) evolves. The Rust contract test
(`crates/aureline-release/tests/stable_publication_pack.rs`) also parses each case and
asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `narrowing_publication_not_narrowed.json` | A narrowing-state publication still publishes a Stable label | `row.published_not_narrowed` |
| `benchmark_held_over_budget.json` | A backed benchmark publication is over its published p50/p95 budget | `row.held_over_budget` |

To regenerate a case after a deliberate pack shape change, copy the canonical pack,
reintroduce the single flaw, recompute the `summary` and `publication` blocks, and confirm
the gate still trips the expected check id.
