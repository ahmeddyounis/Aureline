# Recovery-route decision + factory-reset review fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/reliability/recovery_route_and_factory_reset_review_contract.md`](../../../docs/reliability/recovery_route_and_factory_reset_review_contract.md)
and validated by:

- [`/schemas/recovery/recovery_route_decision.schema.json`](../../../schemas/recovery/recovery_route_decision.schema.json)
- [`/schemas/recovery/factory_reset_review.schema.json`](../../../schemas/recovery/factory_reset_review.schema.json)

Each `*.yaml` file is either:

- one `recovery_route_decision_record` (scenario router output), or
- one `factory_reset_review_record` (factory reset confirmation sheet).

**Scope rules**

- Fixtures validate only against the two schemas above; refs to scenario
  cards, compare sheets, export-before-reset checklists, verification
  results, restore chooser state, or admin seat recovery packets are
  opaque linkage and do not redefine those upstream contracts.
- Monotonic timestamps and stable ids are illustrative and do not
  represent any real clock.

**Index**

| Fixture | Record kind | Scenario family | Recommended route / outcome | Doc section |
|---|---|---|---|---|
| [`profile_corruption_routes_to_rescue.yaml`](./profile_corruption_routes_to_rescue.yaml) | router decision | `profile_corruption` | `corruption_rescue_compare_route` | §2, §4, §5 |
| [`workspace_index_corruption_routes_to_rebuild.yaml`](./workspace_index_corruption_routes_to_rebuild.yaml) | router decision | `workspace_index_corruption` | `corruption_rescue_compare_route` | §2, §4, §5 |
| [`failed_update_routes_to_rollback.yaml`](./failed_update_routes_to_rollback.yaml) | router decision | `failed_update` | `update_and_rollback_route` | §2, §4, §5 |
| [`seat_loss_routes_to_entitlement_recovery.yaml`](./seat_loss_routes_to_entitlement_recovery.yaml) | router decision | `seat_loss` | `entitlement_or_account_recovery_route` | §2, §4, §5 |
| [`control_plane_outage_routes_to_outage_posture.yaml`](./control_plane_outage_routes_to_outage_posture.yaml) | router decision | `control_plane_outage` | `control_plane_outage_route` | §2, §4, §5 |
| [`device_replacement_routes_to_restore_chooser.yaml`](./device_replacement_routes_to_restore_chooser.yaml) | router decision | `device_replacement` | `restore_chooser_route` | §2, §4, §5 |
| [`device_replacement_factory_reset_routes_to_review.yaml`](./device_replacement_factory_reset_routes_to_review.yaml) | router decision | `device_replacement` | `factory_reset_review_route` | §2, §3, §6 |
| [`factory_reset_review_verified.yaml`](./factory_reset_review_verified.yaml) | factory reset review | `device_replacement` | overall `verified` | §6, §7, §8 |
| [`factory_reset_review_partial.yaml`](./factory_reset_review_partial.yaml) | factory reset review | `device_replacement` | overall `partial` + user skipped | §6, §7, §8 |
| [`factory_reset_review_policy_blocked.yaml`](./factory_reset_review_policy_blocked.yaml) | factory reset review | `profile_corruption` | overall `blocked_by_policy` | §6, §7, §8 |
| [`factory_reset_review_unsupported.yaml`](./factory_reset_review_unsupported.yaml) | factory reset review | `seat_loss` | overall `unsupported_class` | §6, §7, §8 |

