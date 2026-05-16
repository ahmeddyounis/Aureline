# Runtime evidence packet fixtures

Checked-in fixtures pinning the canonical scenarios the runtime evidence
packet contract replays end-to-end.

Each case file pairs a closed `scenario` identifier with the expected
`lane`, `evidence_kind`, and replay-comparator outcome that
`aureline_runtime::seeded_runtime_evidence_packet` produces. The shell
panel, the support export, and the integration test all replay the same
seed inputs so reviewer runs reproduce these records byte-for-byte.

| Fixture                                       | Lane    | Kind                    | Replay outcome                                |
|-----------------------------------------------|---------|-------------------------|-----------------------------------------------|
| `local_task_compatible.json`                  | task    | task_event              | `compatible_replay`                           |
| `local_test_policy_advanced_clean.json`       | test    | test_attempt            | `compatible_minor_drift` (clean policy bump)  |
| `container_debug_capsule_drift.json`          | debug   | debug_session           | `incompatible_capsule_drift`                  |
| `managed_runtime_trust_downgraded.json`       | runtime | runtime_trace_evidence  | `incompatible_trust_state_downgraded`         |

The boundary schema lives at
[`/schemas/runtime/evidence_packet.schema.json`](../../../../schemas/runtime/evidence_packet.schema.json)
and the reviewer-facing companion doc at
[`/docs/runtime/m3/evidence_packets.md`](../../../../docs/runtime/m3/evidence_packets.md).
