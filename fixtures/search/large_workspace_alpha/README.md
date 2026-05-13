# Large-Workspace Alpha Search Drills

These drill packets describe large workspaces by counts and deterministic
fixture refs instead of checking in generated repository bytes. They are
validated by `crates/aureline-search/tests/partial_index_drill_cases.rs`.

Each drill must:

- cite the existing alpha fixture register and corpus refs;
- set `human_demo_required` to `false`;
- point at one or more monorepo partial-index fixtures;
- carry a benchmark result packet consumed by
  `artifacts/benchmarks/m2_partial_index_drill.md`.
