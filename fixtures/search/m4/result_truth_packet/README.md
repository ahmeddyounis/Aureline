# Search result-truth packet fixtures

These fixtures exercise the stable result-truth packet at
`crates/aureline-search/src/result_truth_packet/mod.rs`. Each fixture
declares its scenario, the constructor input the runtime materializes,
and the expected promotion state, finding kinds, and consumer-projection
preservation invariants.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | A workspace-file row collapses two lexical strata into one visible row, every contributing stratum is preserved, and the six required consumer projections inherit the full closed truth. |
| `dedupe_anchor_dropped_blocks_stable.json` | `blocks_stable` | A dedup contributor with an empty canonical anchor blocks promotion with `dedupe_dropped_source_stratum` and `dedupe_dropped_canonical_anchor`. |
| `withheld_latency_direct_fallback_blocks_stable.json` | `blocks_stable` | A withheld-latency row whose action binding collapses to `direct` blocks promotion with `dedupe_dropped_fallback_mode`. |
| `captured_vs_live_dropped_blocks_stable.json` | `blocks_stable` | A consumer projection that drops captured-vs-live status blocks promotion with `consumer_projection_drift`, `captured_vs_live_dropped`, and `missing_consumer_projection`. |
| `fact_label_dropped_blocks_stable.json` | `blocks_stable` | Dropping `policy_hidden` from `covered_fact_labels` blocks promotion with `fact_label_silently_dropped` and `missing_fact_label_coverage`. |

Run the fixture suite with:

```
cargo test -p aureline-search --test result_truth_packet_cases
```
