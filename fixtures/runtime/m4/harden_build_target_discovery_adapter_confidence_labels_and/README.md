# harden_build_target_discovery_adapter_confidence_labels_and fixture corpus

Fixture corpus for the M4 stable build-target hardening truth packet (`schemas/runtime/harden_build_target_discovery_adapter_confidence_labels_and_truth.schema.json`).

Each fixture is a `BuildTargetHardeningTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, wedge, discovery-source, discovery-freshness, adapter-confidence label, target-graph snapshot, consumer-surface, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.rs` load each case and assert that materialization matches the expectation block.

Regenerate via:

```bash
python3 tools/regenerate_harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.py
```
