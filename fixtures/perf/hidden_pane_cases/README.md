# Hidden-pane efficiency cases

Reviewable hidden-pane and off-screen scenarios that freeze how the
efficiency-state policy narrows render, polling, preview, graph, and
extension work. These fixtures are not a latency benchmark or a test
suite. They are the policy examples runtime, shell, preview, graph,
extension, support, and benchmark lanes cite when they need to reason
about:

- which workload family was throttled;
- which work class and queue lane it belongs to;
- which hidden or off-screen state applied;
- which user-visible explanation contract must appear; and
- which instrumentation points must prove the suppression happened.

The canonical policy sources are:

- `docs/perf/efficiency_state_policy.md`
- `artifacts/perf/worker_budget_rules.yaml`
- `docs/runtime/resource_governor_contract.md`
- `docs/benchmarks/spike_metric_names.md`

## Fixture rules

- Every fixture names one efficiency state and one hidden or off-screen
  visibility state.
- Every fixture maps the throttled behavior to one workload family and
  one shared work class.
- Every fixture names the visible explanation contracts users should
  see and the instrumentation points support or benchmark evidence
  should capture.
- Fixtures never set benchmark thresholds. They freeze *what* must be
  throttled and *how* it must be explained, not the final numeric lab
  budgets.

## Index

| Fixture | What it exercises | Expected truth |
|---|---|---|
| `hidden_terminal_visuals_efficiency_aware.json` | Hidden terminal tab while battery saver is active | PTY correctness remains; cursor blink and hidden render work stop; runtime exposes `EfficiencyAware` |
| `offscreen_preview_refresh_thermal_constrained.json` | Detached preview window moved off-screen under thermal pressure | Preview freezes at last truthful snapshot; hidden refresh stops; snapshot age becomes visible |
| `collapsed_graph_enrichment_protect_core.json` | Collapsed graph pane during critical battery protection | Non-visible graph enrichment pauses; search/graph results project `partial` or `overloaded`, not `ready` |
| `hidden_extension_poller_recovery_resume.json` | Recovery after a thermal clamp while the extension pane remains hidden | Recovery stays staged; hidden extension polling does not stampede back on until the pane is visible |
