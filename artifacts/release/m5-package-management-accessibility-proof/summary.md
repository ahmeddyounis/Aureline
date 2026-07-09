# Package-Management Component Accessibility, Headless, and Export Parity

- Packet: `package-management-component-accessibility:stable:0001`
- Surface: `Package-management component accessibility, headless, and export parity`
- Accessibility rows: 8 (6 claim-narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Accessibility rows

- **package_explorer_row** [`row:package-explorer-trusted`]: condition `package_truth_trusted`, claim `full_reviewable_management`
- **manifest_scope_switcher** [`row:manifest-scope-switcher-scope-partial`]: condition `manifest_scope_partial`, claim `manifest_range_scoped`
- **install_review_sheet** [`row:install-review-lockfile-unavailable`]: condition `lockfile_impact_unavailable`, claim `lockfile_impact_unknown`
- **registry_or_mirror_row** [`row:registry-or-mirror-freshness-stale`]: condition `registry_freshness_stale`, claim `mirror_or_offline_sourced`
- **script_risk_notice** [`row:script-risk-notice-trusted`]: condition `package_truth_trusted`, claim `full_reviewable_management`
- **lockfile_impact_card** [`row:lockfile-impact-card-rollback-unavailable`]: condition `rollback_checkpoint_unavailable`, claim `rollback_unavailable_manual_recovery`
- **grouped_update_planner** [`row:grouped-update-planner-auth-unsatisfied`]: condition `auth_state_unsatisfied`, claim `auth_required_read_only`
- **rollback_checkpoint_strip** [`row:rollback-checkpoint-strip-rollback-unavailable`]: condition `rollback_checkpoint_unavailable`, claim `rollback_unavailable_manual_recovery`
