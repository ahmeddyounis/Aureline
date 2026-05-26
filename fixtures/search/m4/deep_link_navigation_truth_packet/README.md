# Deep-link / navigation continuity truth packet fixtures

These fixtures exercise the stable deep-link / navigation-continuity truth
packet at `crates/aureline-search/src/deep_link_navigation_truth/mod.rs`.
Each fixture declares its scenario, the constructor input the runtime
materializes, and the expected promotion state, finding kinds, and support
export safety.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | A moved-file bookmark joins a remap packet and continuity record; every required consumer projection inherits the full closed truth. |
| `continuity_mismatch_blocks_stable.json` | `blocks_stable` | A continuity record citing the wrong remap packet id blocks promotion with `continuity_remap_packet_mismatch`. |
| `missing_projection_blocks_stable.json` | `blocks_stable` | Dropping the support_export consumer projection blocks promotion with `missing_consumer_projection`. |
| `projection_drops_drift_state_blocks_stable.json` | `blocks_stable` | A projection that drops the drift-state vocabulary blocks promotion with `projection_drift_state_dropped` (and the consumer-truth invariants the same projection should honour). |
| `outcome_coverage_over_declared_blocks_stable.json` | `blocks_stable` | Declaring an outcome in `covered_outcomes` that no row carries blocks promotion with `outcome_coverage_over_declared`. |
| `destination_visibility_drift_blocks_stable.json` | `blocks_stable` | A cross-root row whose remap and continuity destination-visibility rows disagree blocks promotion with `destination_visibility_drift`. |

Run the fixture suite with:

```
cargo test -p aureline-search --test deep_link_navigation_truth_cases
```
