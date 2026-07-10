# Shared Protected-Path Governance Component Consumers: Owner, Approver, and Public-Surface Parity

- Packet: `governance-component-consumer:stable:0001`
- Surface: `Shared protected-path governance-component consumers`
- Consumer bindings: 16 (12 narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Consumer bindings

- **protected path src/payments/*** [`bind:pp-1:workspace`]: component `protected_path_row` on `review_workspace`, mode `full_parity`
- **protected path src/payments/*** [`bind:pp-1:shiproom`]: component `protected_path_row` on `shiproom`, mode `full_parity`
- **ownership of data/pipelines/*** [`bind:oc-2:workspace`]: component `ownership_card` on `review_workspace`, mode `coverage_narrowed`
- **ownership of data/pipelines/*** [`bind:oc-2:support`]: component `ownership_card` on `support_packet`, mode `coverage_narrowed`
- **approver matrix for release/*** [`bind:am-3:queue`]: component `approver_matrix` on `merge_queue`, mode `approval_narrowed`
- **approver matrix for release/*** [`bind:am-3:help`]: component `approver_matrix` on `help_surface`, mode `approval_narrowed`
- **review pack for feature/checkout** [`bind:rp-4:workspace`]: component `review_pack_summary` on `review_workspace`, mode `stale_narrowed`
- **review pack for feature/checkout** [`bind:rp-4:cli`]: component `review_pack_summary` on `cli_export`, mode `stale_narrowed`
- **public surface diff for sdk v3** [`bind:ps-5:release`]: component `public_surface_diff_card` on `release_center`, mode `public_surface_narrowed`
- **public surface diff for sdk v3** [`bind:ps-5:support`]: component `public_surface_diff_card` on `support_packet`, mode `public_surface_narrowed`
- **merge control for hotfix/logging** [`bind:mc-6:queue`]: component `merge_control_banner` on `merge_queue`, mode `enforcement_narrowed`
- **merge control for hotfix/logging** [`bind:mc-6:shiproom`]: component `merge_control_banner` on `shiproom`, mode `enforcement_narrowed`
- **DRI registry for auth service** [`bind:dr-7:release`]: component `dri_registry_row` on `release_center`, mode `full_parity`
- **DRI registry for auth service** [`bind:dr-7:cli`]: component `dri_registry_row` on `cli_export`, mode `full_parity`
- **merge readiness for web/*** [`bind:mr-8:queue`]: component `merge_readiness_strip` on `merge_queue`, mode `enforcement_narrowed`
- **merge readiness for web/*** [`bind:mr-8:help`]: component `merge_readiness_strip` on `help_surface`, mode `enforcement_narrowed`
