# Effective control-stack fixtures

Worked control-stack scenarios that validate against:

- [`/schemas/settings/effective_control_stack_row.schema.json`](../../../schemas/settings/effective_control_stack_row.schema.json)

Each fixture is an `effective_control_stack_matrix` packet. The packet is the
local-offline proof of control resolution: a reviewer can reconstruct the final
decision and every contributing/capped/blocked/expired layer from the fixture
alone.

## Index

| Fixture | Exercises |
|---|---|
| [`offline_last_known_good_admin_bundle.yaml`](./offline_last_known_good_admin_bundle.yaml) | Offline last-known-good signed bundle; managed override unreachable |
| [`expired_experiment_binding.yaml`](./expired_experiment_binding.yaml) | Expired experiment binding + deterministic fallback |
| [`policy_ceiling_overrides_local_preference.yaml`](./policy_ceiling_overrides_local_preference.yaml) | Policy ceiling narrows a broader local preference |
| [`signed_bundle_mismatch_last_known_good.yaml`](./signed_bundle_mismatch_last_known_good.yaml) | Signed-bundle mismatch blocks candidate + keeps last-known-good |
| [`emergency_kill_switch_narrowing.yaml`](./emergency_kill_switch_narrowing.yaml) | Emergency kill switch narrows/disable path remains explicit |

