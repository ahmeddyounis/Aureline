# Fixtures: M5 lifecycle actions

This directory contains fixture metadata for the `m5_lifecycle_actions` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-lifecycle-actions.json`

## Coverage

- Fifteen lifecycle records cover the framework-pack, docs-pack, local-model-pack,
  recipe-pack, template, bridge-backed, and side-loaded families, so distinct
  reviewed lifecycle actions are proven across the marketed M5 artifact families.
- Every `action_kind` is exercised: `disable_workspace`, `disable_global`,
  `uninstall`, `rollback`, `quarantine`, `reenable`, and `apply_registry_status`.
- Every resulting `lifecycle_status` is exercised: `active`, `disabled_workspace`,
  `disabled_global`, `uninstalled`, `rolled_back`, `quarantined`, `revoked`,
  `yanked`, `deprecated`, and `publisher_transferred`.
- Every `trigger` is exercised, including the reactive triggers `crash_loop`,
  `integrity_failure`, `performance_budget_exceeded`, `moderation`, `policy`,
  `revocation`, `yank`, `deprecation`, and `publisher_transfer`, each routing through
  an explicit lifecycle state rather than a generic banner.
- Every `continuity` disposition is exercised, including placeholder conversion of
  M5-contributed surfaces (`converts_to_placeholder_at_next_activation`).
- Each record carries a `governance_family_ref` that resolves to its row in
  `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`.
- The data-retention guardrail is proven: protected user-owned data removed without
  captured consent recomputes to `blocked`
  (`framework_pack_uninstall_blocked_unconsented`), while a consented removal is
  disclosed and reviewed (`side_loaded_uninstall_purge`).
- The rollback honesty rule is proven: a `not_reversible` rollback recomputes to
  `blocked` with its primary action disabled (`bridge_rollback_blocked`), a
  `state_loss_possible` rollback is reviewed (`model_pack_rollback_stateloss`), and a
  `clean` rollback proceeds directly (`recipe_pack_rollback_clean`).
- Each record's `review_reasons` and `action_disposition` equal the recomputation
  from its facts; the clean workspace disable, the clean rollback, and the docs-pack
  re-enable recompute to `proceed_allowed`.

| corpus id | record id | proves |
| --- | --- | --- |
| framework-pack-disable-workspace | `lifecycle:framework_pack_disable_workspace` | `disable_workspace`, `keeps_running_temporarily`, `proceed_allowed` |
| docs-pack-disable-global | `lifecycle:docs_pack_disable_global` | `disable_global`, placeholder conversion, `review_required` |
| side-loaded-uninstall-purge | `lifecycle:side_loaded_uninstall_purge` | `uninstall`, `disclosed_protected_data_removal`, `irreversible_action` |
| recipe-pack-rollback-clean | `lifecycle:recipe_pack_rollback_clean` | `rollback`, `clean`, `proceed_allowed` |
| model-pack-rollback-stateloss | `lifecycle:model_pack_rollback_stateloss` | `rollback`, `rollback_not_risk_free`, `review_required` |
| bridge-rollback-blocked | `lifecycle:bridge_rollback_blocked` | `rollback`, `not_reversible`, `blocked` |
| framework-pack-quarantine-crashloop | `lifecycle:framework_pack_quarantine_crashloop` | `quarantine`, `crash_loop`, `automated_health_trigger` |
| template-quarantine-moderation | `lifecycle:template_quarantine_moderation` | `quarantine`, `moderation`, placeholder conversion |
| model-pack-quarantine-perf | `lifecycle:model_pack_quarantine_perf` | `quarantine`, `performance_budget_exceeded` |
| recipe-pack-revoked | `lifecycle:recipe_pack_revoked` | `revoked`, `registry_status_trigger`, retained recipes |
| model-pack-yanked | `lifecycle:model_pack_yanked` | `yanked`, not restorable |
| framework-pack-deprecated | `lifecycle:framework_pack_deprecated` | `deprecated`, keeps running |
| bridge-publisher-transferred | `lifecycle:bridge_publisher_transferred` | `publisher_transferred`, `publisher_transfer_trigger` |
| docs-pack-reenable | `lifecycle:docs_pack_reenable` | `reenable`, `active`, `proceed_allowed` |
| framework-pack-uninstall-blocked-unconsented | `lifecycle:framework_pack_uninstall_blocked_unconsented` | `unconsented_protected_data_removal`, `blocked` |
