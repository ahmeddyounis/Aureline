# Execution-context beta fixtures

These fixtures pin the four-lane beta execution-context resolver contract
and the ticket-drift evaluator. They are replayed end-to-end through the
integration test at
[`/crates/aureline-runtime/tests/execution_context_beta.rs`](../../../crates/aureline-runtime/tests/execution_context_beta.rs).

| Fixture | What it proves |
| --- | --- |
| `local_lane.json` | Terminal seed resolves through the `local_host` lane with no boundary cue and no degraded fields. |
| `remote_lane.json` | SSH remote task seed resolves through the `remote_attach` lane and lights the boundary cue. |
| `container_lane.json` | Devcontainer test seed resolves through the `container` lane with the containerised runtime toolchain. |
| `request_workspace_lane.json` | Managed-workspace task seed resolves through the `request_workspace` lane with restricted trust narrowing visible on the same record. |
| `ticket_drift_invalidated.json` | A stored local-host binding is invalidated against a fresh remote-attach context; `target_class`, `canonical_target_id`, and `working_directory` drift rows MUST be present. |
| `beta_lane_coverage.json` | Canonical lane coverage manifest the runtime emits; round-trips through serde. |

Verify:

```sh
cargo test -p aureline-runtime --test execution_context_beta
```
