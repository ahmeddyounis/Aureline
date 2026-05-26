# Graph freshness-propagation packet fixtures

These fixtures exercise the stable freshness-propagation packet at
`crates/aureline-graph/src/freshness_propagation_packet/mod.rs`. Each
fixture declares its scenario, the constructor input the runtime
materializes, and the expected promotion state, finding kinds, and
consumer-projection preservation invariants.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | A local-live workspace-graph row and an imported-provider row each carry epoch ref, schema version, producer identity, visibility scope, retention class, smallest-subgraph (or no-invalidation) class, and hidden-graph-dependency disclosure; nine consumer projections preserve the same packet verbatim. |
| `mixed_epoch_unlabeled_blocks_stable.json` | `blocks_stable` | A `mixed_epoch_unresolvable` row that drops the mixed-epoch disclosure blocks promotion with `mixed_epoch_unlabeled`. |
| `full_rebuild_not_surfaced_blocks_stable.json` | `blocks_stable` | A `full_rebuild_workspace_epoch_boundary` invalidation that drops the full-rebuild reason blocks promotion with `full_rebuild_not_surfaced`. |
| `hidden_dependency_undisclosed_blocks_stable.json` | `blocks_stable` | A row whose hidden-graph dependency state is `hidden_dependency_undisclosed` blocks promotion with `hidden_graph_dependency_undisclosed`. |
| `consumer_projection_drops_epoch_label_blocks_stable.json` | `blocks_stable` | A consumer projection that sets `preserves_epoch_label = false` blocks promotion with `epoch_label_dropped`, `consumer_projection_drift`, and `missing_consumer_projection`. |

Run the fixture suite with:

```
cargo test -p aureline-graph --test freshness_propagation_packet_cases
```
