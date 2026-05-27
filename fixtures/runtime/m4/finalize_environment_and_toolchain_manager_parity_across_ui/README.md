# finalize_environment_and_toolchain_manager_parity_across_ui fixture corpus

Fixture corpus for the M4 stable inspector-parity truth packet (`schemas/runtime/finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json`).

Each fixture is an `InspectorParityTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, inspector-field, parity-surface, recovery-state, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.rs` load each case and assert that materialization matches the expectation block.

Regenerate via:

```bash
python3 tools/regenerate_finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.py
```
