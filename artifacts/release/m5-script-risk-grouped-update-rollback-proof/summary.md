# Script-risk notices, grouped-update planners, and rollback/checkpoint strips

- Packet: `script-risk-grouped-update-rollback:stable:0001`
- Surface: `Script-risk, grouped-update, and rollback controls`
- Script-risk notices: 4 (1 policy-blocked)
- Grouped-update planners: 4 (1 broad convergence)
- Rollback/checkpoint strips: 3 (1 remove-blocked)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Script-risk notices

- **left-pad** — source `no_scripts_declared`, risk `no_execution`
- **esbuild** — source `install_lifecycle_script`, risk `review_recommended`
- **node-sass** — source `native_build_step`, risk `policy_blocked`
- **unknown-pkg** — source `postinstall_binary_fetch`, risk `unknown_untrusted`

## Grouped-update planners

- **Bump lodash** — reason `direct_request`, class `direct_bump`, 1 package(s), 2 transitive churn
- **Patch minimist advisory** — reason `security_advisory`, class `security_patch`, 1 package(s), 3 transitive churn
- **Refresh eslint toolchain** — reason `routine_refresh`, class `grouped_refresh`, 3 package(s), 10 transitive churn
- **Converge the dependency tree** — reason `dependency_convergence`, class `broad_convergence`, 5 package(s), 40 transitive churn

## Rollback/checkpoint strips

- **Checkpoint before install** — recovery `fully_revertible`, remove-blocked `removable`
- **Checkpoint before grouped update** — recovery `revert_with_regeneration`, remove-blocked `not_a_remove`
- **Checkpoint before remove** — recovery `compensating_only`, remove-blocked `remove_blocked_required_by`
