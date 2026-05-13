# Monorepo Partial-Index Drill Fixtures

These fixtures protect alpha search behavior when the workspace is useful
before all index shards are current. They exercise:

- partial-index rows where hot-set and lexical results are usable;
- stale shard rows after a branch or shard epoch changes;
- hidden-scope rows caused by sparse worksets, excludes, or policy limits.

Each JSON fixture is consumed by
`crates/aureline-search/tests/partial_index_drill_cases.rs`. The benchmark
packet at `artifacts/benchmarks/m2_partial_index_drill.md` cites the same
fixture ids and register refs so benchmark evidence does not clone a second
search truth model.
