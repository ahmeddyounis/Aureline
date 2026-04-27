# Form-validation contract fixtures

Worked YAML records for the contract frozen in
[`/docs/ux/forms_validation_contract.md`](../../../docs/ux/forms_validation_contract.md)
and the schemas at
[`/schemas/ux/form_probe_state.schema.json`](../../../schemas/ux/form_probe_state.schema.json)
and
[`/schemas/ux/staged_review_state.schema.json`](../../../schemas/ux/staged_review_state.schema.json).

Each fixture is one record:

- `form_probe_state_record` for a single validation/probe result and
  its apply gate.
- `staged_review_state_record` for a whole review sheet or CLI/headless
  review packet.

The fixtures intentionally use opaque refs and redaction labels. They
do not embed raw endpoints, raw connection strings, raw package registry
addresses, raw policy bundles, raw request bodies, raw filesystem paths,
or raw secret values.

## Cases

- [`settings_policy_locked_no_apply.yaml`](./settings_policy_locked_no_apply.yaml)
  - workspace setting staged review blocked by signed policy.
- [`connection_setup_pending_probe.yaml`](./connection_setup_pending_probe.yaml)
  - connection setup review waiting on a required async broker probe.
- [`package_install_stale_probe_ack.yaml`](./package_install_stale_probe_ack.yaml)
  - package install review where a stale advisory probe can proceed only
    with explicit acknowledgement.
- [`policy_edit_simulation_blocked.yaml`](./policy_edit_simulation_blocked.yaml)
  - policy edit review blocked by a failed policy simulation.
- [`repair_probe_skipped_allowed.yaml`](./repair_probe_skipped_allowed.yaml)
  - repair dry-run probe skipped by user while apply remains allowed
    because the repair transaction has its own preview checkpoint.
- [`transport_policy_blocked_probe.yaml`](./transport_policy_blocked_probe.yaml)
  - transport validation policy-blocked before a network probe can run.
- [`request_runtime_stale_schema.yaml`](./request_runtime_stale_schema.yaml)
  - request/runtime form with a stale schema snapshot that must refresh
    before replay.
