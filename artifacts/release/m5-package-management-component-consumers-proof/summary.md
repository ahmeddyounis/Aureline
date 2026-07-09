# Shared Package-Management Component Consumers: Scope, Auth, and Lockfile Parity

- Packet: `package-component-consumer:stable:0001`
- Surface: `Shared package-management-component consumers`
- Consumer bindings: 16 (10 narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-09T00:00:00Z)

## Consumer bindings

- **lodash** [`bind:per-1:explorer`]: component `package_explorer_row` on `package_explorer`, mode `full_parity`
- **lodash** [`bind:per-1:search`]: component `package_explorer_row` on `dependency_search_detail`, mode `full_parity`
- **react (member scope)** [`bind:mss-2:explorer`]: component `manifest_scope_switcher` on `package_explorer`, mode `manifest_range_narrowed`
- **react (member scope)** [`bind:mss-2:search`]: component `manifest_scope_switcher` on `dependency_search_detail`, mode `manifest_range_narrowed`
- **numpy (install review)** [`bind:irs-3:search`]: component `install_review_sheet` on `dependency_search_detail`, mode `full_parity`
- **numpy (install review)** [`bind:irs-3:diagnostics`]: component `install_review_sheet` on `diagnostics`, mode `full_parity`
- **serde (mirror source)** [`bind:rmr-4:search`]: component `registry_or_mirror_row` on `dependency_search_detail`, mode `mirror_or_offline_narrowed`
- **serde (mirror source)** [`bind:rmr-4:support`]: component `registry_or_mirror_row` on `support_packet`, mode `mirror_or_offline_narrowed`
- **node-sass (script risk)** [`bind:srn-5:diagnostics`]: component `script_risk_notice` on `diagnostics`, mode `mirror_or_offline_narrowed`
- **node-sass (script risk)** [`bind:srn-5:support`]: component `script_risk_notice` on `support_packet`, mode `mirror_or_offline_narrowed`
- **pnpm-lock.yaml (impact)** [`bind:lic-6:help`]: component `lockfile_impact_card` on `help_surface`, mode `full_parity`
- **pnpm-lock.yaml (impact)** [`bind:lic-6:export`]: component `lockfile_impact_card` on `exported_view`, mode `full_parity`
- **security grouped update** [`bind:gup-7:help`]: component `grouped_update_planner` on `help_surface`, mode `auth_required_narrowed`
- **security grouped update** [`bind:gup-7:support`]: component `grouped_update_planner` on `support_packet`, mode `auth_required_narrowed`
- **Cargo.lock checkpoint** [`bind:rcs-8:export`]: component `rollback_checkpoint_strip` on `exported_view`, mode `unknown_or_stale_narrowed`
- **Cargo.lock checkpoint** [`bind:rcs-8:diagnostics`]: component `rollback_checkpoint_strip` on `diagnostics`, mode `unknown_or_stale_narrowed`
