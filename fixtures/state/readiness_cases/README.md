# Readiness-case fixtures

Each fixture pairs a workspace-readiness input snapshot with the
expected `ReadinessProjection` the live reactive-state store will
produce. The corpus pins the canonical projection for every
readiness label exercised on the M1 protected workspace surfaces.

Schema:

```
record_kind: "readiness_case"
schema_version: 1
case_id: <stable id>
title: <short human description>
input:
  workspace_id: <workspace id>
  lifecycle_phase: <discovered|trust_evaluating|opening|partially_ready|
                    ready|degraded|closing|closed>
  watcher_health: <healthy|warming|degraded|fallback_polling|unavailable|null>
  hot_index_ready: <bool>
  command_graph_ready: <bool>
  observed_at: <stable mono token>
expect:
  freshness: <authoritative|warming|cached|stale|replayed|imported>
  completeness: <full|partial|unloaded|unavailable>
  readiness_label: <exact|imported|heuristic|stale|partial|unavailable|out_of_scope>
  not_ready_reason: <stale-reason token | null>
```

The fixtures are executed by
`crates/aureline-reactive-state/src/runtime.rs` (in-process
verification) via the test
`runtime::tests::readiness_case_fixtures_match_expected_projection`.

Adding or modifying a fixture must be paired with a matching
update to `docs/architecture/reactive_state_contract.md` so the
truth vocabulary stays single-sourced.
