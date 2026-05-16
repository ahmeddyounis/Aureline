# Runtime replay packs — checked-in fixtures

These fixtures pin one canonical replay-pack outcome per closed
[`ReplayFidelityClass`] vocabulary token. Each fixture binds the seeded
runtime evidence packet scenario, the captured-action privilege class, and
the expected reopen decision. The
[`crates/aureline-support/tests/runtime_replay_packs.rs`](../../../../crates/aureline-support/tests/runtime_replay_packs.rs)
integration test loads every fixture, runs the seeded replay-pack builder,
and asserts the comparator + fidelity + reopen-decision outcome matches.

| Fixture | Fidelity | Privilege | Reopen decision |
|---------|----------|-----------|------------------|
| `local_task_exact_read_only.json` | `exact` | `read_only` | `allow_replay` |
| `local_test_compatible_read_only.json` | `compatible` | `read_only` | `allow_replay` |
| `container_debug_layout_only_mutating.json` | `layout_only` | `mutating` | `allow_inspect_no_rerun` |
| `managed_runtime_layout_only_privileged.json` | `layout_only` | `privileged` | `allow_inspect_no_rerun` |

The `evidence_only` fidelity class is exercised by unit tests in
`crates/aureline-support/src/runtime_evidence/mod.rs`; no seeded scenario
is checked in yet because the closed source runtime scenarios already cover
target-class drift via the existing
`runtime_evidence_packet_case` fixtures.
