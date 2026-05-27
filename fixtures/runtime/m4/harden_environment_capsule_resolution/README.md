# harden_environment_capsule_resolution fixture corpus

Fixture corpus for the M4 stable capsule-resolution truth packet (`schemas/runtime/harden_environment_capsule_resolution_truth.schema.json`).

Each fixture is a `CapsuleResolutionTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, capsule-field, prebuild-fingerprint, invalidation-reason, project-doctor-finding, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/harden_environment_capsule_resolution_truth_packet.rs` load each case and assert that materialization matches the expectation block.

Regenerate via:

```bash
python3 tools/regenerate_harden_environment_capsule_resolution_truth_packet.py
```
