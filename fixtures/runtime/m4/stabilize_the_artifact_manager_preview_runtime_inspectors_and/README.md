# stabilize_the_artifact_manager_preview_runtime_inspectors_and fixture corpus

Fixture corpus for the M4 stable artifact-manager / preview-runtime-inspector / evidence-export truth packet (`schemas/runtime/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth.schema.json`).

Each fixture is an `EvidenceExportTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, wedge, signal-slice kind, slice-freshness, replay-chronology state, retention class, consumer-surface, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.rs` load each case and assert that materialization matches the expectation block.

Regenerate via:

```bash
python3 tools/regenerate_stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.py
```
