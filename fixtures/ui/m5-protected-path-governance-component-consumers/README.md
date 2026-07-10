# M5 protected-path governance-component consumer fixtures

Protected fixtures for the consumer-adoption lane
`add_shared_review_workspace_merge_queue_release_center_help_support_shiproom_cli_export_consumers_so_protected_path_governance_components_keep_owner_coverage_approver_and_public_surface_language_aligned`
in `crates/aureline-review`.

Each fixture is a `GovernanceComponentConsumerPacket` validated against
[`schemas/ui/m5-protected-path-governance-component-consumer.schema.json`](../../../schemas/ui/m5-protected-path-governance-component-consumer.schema.json)
and by the module's `validate()` — the same eight components bound to the review
workspace, merge queue, release center, Help surface, support packet, shiproom, and
CLI/export payload, proving that the same governed change presents identical
owner-coverage, approver-state, public-surface-impact, and merge-blocker language
across surfaces.

| Fixture | Scenario |
| --- | --- |
| `enforcement_and_coverage_narrowed.json` | A change narrows to `enforcement_narrowed` (advisory / local estimate) and another to `coverage_narrowed` (owner backup coverage missing); the enforcement-authority and evidence-continuity notes stay explicit and every parity facet is preserved. |
| `public_surface_and_stale_narrowed.json` | A change narrows to `public_surface_narrowed` (machine-generated diff / migration evidence missing) and another to `stale_narrowed` (provider proof stale relative to head); each narrowing is disclosed through its banner and note. |

Regenerate the checked-in support export, summary, and fixtures after a contract
change:

```sh
GEN_GOVERNANCE_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_governance_component_consumer_artifacts
```
