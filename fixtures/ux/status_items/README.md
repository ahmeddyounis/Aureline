# Status item fixtures

Seed corpus for the contract frozen in
[`/docs/ux/status_bar_contract.md`](../../../docs/ux/status_bar_contract.md)
and the schema at
[`/schemas/ux/status_item.schema.json`](../../../schemas/ux/status_item.schema.json).

Each fixture is a single JSON document validating as one of the status
item record families. The corpus covers priority order, stable slots,
compact overflow, search/menu parity, anti-jitter posture, and extension
contribution budget review.

## Cases

| Fixture | Record kind | Scenario axis |
| --- | --- | --- |
| [`canonical_status_item_catalog.json`](./canonical_status_item_catalog.json) | `status_item_catalog_record` | Frozen priority ladder, stable slots, overflow policy, extension budget, and anti-jitter policy. |
| [`save_failed_recovery_item.json`](./save_failed_recovery_item.json) | `status_item_record` | Recovery-critical save failure keeps stable placement and opens a narrow inspector. |
| [`extension_lint_work_budgeted.json`](./extension_lint_work_budgeted.json) | `status_item_record` | Extension-contributed ongoing work fits the budget and routes to an extension task detail. |
| [`compact_crowded_overflow_parity.json`](./compact_crowded_overflow_parity.json) | `status_bar_layout_snapshot_record` | Compact crowded layout keeps severe state visible while ambient and extension items overflow with parity. |
| [`extension_vanity_budget_denied.json`](./extension_vanity_budget_denied.json) | `extension_status_item_budget_decision_record` | Branding-only extension request is denied instead of displacing first-party status truth. |

