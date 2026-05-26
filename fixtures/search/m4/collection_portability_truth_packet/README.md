# Collection portability truth packet fixtures

These fixtures exercise the stable collection-portability truth packet at
`crates/aureline-search/src/collection_portability_truth/mod.rs`. Each fixture
declares its scenario, the constructor input the runtime materializes, and
the expected promotion state, finding kinds, and support-export safety.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | A user-authored local saved query, query history, scope-pack binding, saved view, column preset, and scope counters round-trip through every required consumer projection without findings. |
| `scope_binding_mismatch_blocks_stable.json` | `blocks_stable` | A row whose saved query, query history, and scope-pack binding disagree on the scope-binding id fails to certify with `scope_pack_binding_mismatch`. |
| `missing_projection_blocks_stable.json` | `blocks_stable` | Dropping the `support_export` consumer projection blocks promotion with `missing_consumer_projection`. |
| `projection_drops_filter_ast_blocks_stable.json` | `blocks_stable` | A consumer projection that drops the filter-AST vocabulary blocks promotion with `projection_filter_ast_dropped` and the consumer-truth drift the projection should honour. |
| `scope_counter_vocabulary_dropped_blocks_stable.json` | `blocks_stable` | Dropping the `all_matching` scope-counter term (one of the eight required) blocks promotion with `scope_counter_vocabulary_dropped`. |
| `batch_review_required_but_missing_blocks_stable.json` | `blocks_stable` | An `all_matching_query` selection without a batch-review sheet blocks promotion with `batch_review_required_but_missing`. |
| `reopen_state_coverage_over_declared_blocks_stable.json` | `blocks_stable` | Declaring a reopen state in `covered_reopen_states` that no row carries blocks promotion with `reopen_state_coverage_over_declared`. |

Run the fixture suite with:

```
cargo test -p aureline-search --test collection_portability_truth_cases
```
