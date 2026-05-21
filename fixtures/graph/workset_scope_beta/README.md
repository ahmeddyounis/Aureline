# Graph workset / scope beta corpus

Frozen corpus that promotes the graph surface's workset / sparse-scope /
policy-limited-view truth from alpha to beta. It proves the graph
impact-explainer honours a declared scope boundary the same way the workspace,
search, and refactor surfaces do: in-scope results stay visible, results that
escape the scope are **labeled** (counted as out-of-scope), and policy-hidden
members are **disclosed** through the scope descriptor — never silently dropped.

## What each case asserts

Each `*_boundary.json` case is replayed by
`crates/aureline-graph/tests/workset_scope_beta.rs`. The test builds a graph
from the named `source_graph_scenario`, scopes its impact edge to the case's
`in_scope_id` under the declared `graph_scope_class`, clones a sibling impact
edge into `out_of_scope_id`, and (for the policy case) projects a policy view
that hides members. It then builds the impact-explainer packet for the active
scope and asserts:

- every visible impact edge carries the active scope id (no result escapes the
  declared scope — a leak fails the test);
- the sibling edge is present in the store but counted in `out_of_scope_count`
  and absent from the visible set (labeled, not dropped);
- the scope descriptor's `hidden_result_count` equals
  `out_of_scope_count + policy_hidden_members`, so policy-limited results are
  disclosed.

## Shared scope vocabulary

`manifest.json`'s `scope_class_vocabulary_map` pins the 1:1 mapping between the
graph `WorksetScopeClass` vocabulary (`aureline-graph-proto`) and the
`aureline-workspace` `ScopeClass` vocabulary. `graph_only_scope_classes` lists
the two graph surface extensions (`review_workspace`, `companion_surface`) that
have no core workspace scope class. The Rust test proves the mapping is a
bijection over the shared classes; `ci/check_beta_workset_scope_coverage.py`
re-derives both vocabularies from crate source and fails closed if either
drifts from this map, so the two crates cannot fork the scope vocabulary
without breaking the gate.
