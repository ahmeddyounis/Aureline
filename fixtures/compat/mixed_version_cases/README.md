# Mixed-version negotiation envelope fixtures

Seed fixtures for the generic mixed-version negotiation envelope
published in
[`/schemas/compat/mixed_version_envelope.schema.json`](../../../schemas/compat/mixed_version_envelope.schema.json).
The binding skew-window, upgrade-order, rollback-order, and
unsupported-state declarations live in
[`/artifacts/compat/skew_windows.yaml`](../../../artifacts/compat/skew_windows.yaml).
The narrative companion lives in
[`/docs/compat/upgrade_order_contract.md`](../../../docs/compat/upgrade_order_contract.md).

Each fixture is a minimal, reviewer-facing example of one closed
negotiation outcome. None of the fixtures is a live release packet;
they exist so qualification and compatibility tooling can consume the
same envelope shape without inventing parallel compatibility
identities.

| Fixture | Boundary family | Outcome | Purpose |
|---|---|---|---|
| [`compatible.json`](./compatible.json) | `schema_or_state_bundle` | `compatible` | Happy-path envelope for producer and consumer in the same additive epoch. |
| [`out_of_window.json`](./out_of_window.json) | `managed_control_plane` | `refused_out_of_window` | Client beyond the declared previous-minor/LTS window; cached read-only posture with privileged writes refused. |
| [`downgrade_required.json`](./downgrade_required.json) | `desktop_cli_and_remote_agent` | `downgrade_required` | Client ahead of the declared adjacent window; supported path is client downgrade or review-only continuation. |
| [`partial_feature_narrowing.json`](./partial_feature_narrowing.json) | `extension_host_and_sdk` | `compatible_with_partial_feature_narrowing` | Non-empty intersection with one typed dropped capability recorded. |
| [`repair_reattach.json`](./repair_reattach.json) | `desktop_cli_and_remote_agent` | `reattach_required` | Session envelope drifted after a network partition; typed reattach and capability-manifest refresh hints. |
| [`upgrade_order_violation.json`](./upgrade_order_violation.json) | `launcher_and_local_sidecars` | `refused_upgrade_order_violation` | Worked example showing how a coordinated-artifact-set boundary fails closed when the upgrade order is violated. |

The last fixture satisfies the acceptance criterion that one example
shows how a boundary fails closed when the upgrade order is violated.
