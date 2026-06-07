# Stabilize selection scope and batch-result truth

Dense collections must carry selection scope as product data, not infer it from
row order, focus, the current viewport, or localized status text. The stable
contract lives in
[`schemas/collections/selection-scope.schema.json`](../../schemas/collections/selection-scope.schema.json)
and the generated proof packet lives in
[`artifacts/collections/m4/stabilize-selection-scope-and-batch-result-truth/selection_scope_packet.json`](../../artifacts/collections/m4/stabilize-selection-scope-and-batch-result-truth/selection_scope_packet.json).

## Contract

Every stable dense collection selection exposes:

- a `scope_class` from `current_item_only`, `visible_range`, `loaded_set`,
  `all_matching_query`, or `explicit_custom_set`;
- query/session refs, scope/workset refs, stable item refs, selected count,
  hidden-member count, blocked count, filtered-out count, stale-snapshot state,
  and local/provider/mixed execution origin;
- explicit `select_all_expansion_was_explicit` truth before a visible or loaded
  selection can expand to all matching query results;
- visible traversal-order range semantics for trees, with collapsed descendants
  excluded unless a separate subtree-scoped action is chosen;
- an accessibility summary that includes selected count, scope, hidden members,
  stale snapshot state when present, and the visible/loaded/all-matching target.

Consequential batch actions use `BatchReviewTruth` rather than a generic toast.
The review sheet preserves included, excluded, blocked, hidden, skipped, and
query-derived counts; privacy class; rollback/retry guidance; focus-return
truth; provider/local execution origin; and per-item results for mixed
outcomes.

## Consumers

Desktop UI, CLI/headless output, export packets, support captures, and
accessibility trees consume the same packet. Export and support lanes preserve
the original packet instead of recomputing exact counts from a narrowed local
view.

## Regeneration

Run:

```sh
cargo run -p aureline-collections --example dump_selection_scope_batch_result_truth
```

The output should match the checked-in artifact after formatting. The Rust
validator in `aureline-collections` is the executable contract for invariants
that JSON Schema cannot express, such as member disposition counts matching
their review rows and mixed outcomes preserving per-item truth.
