# Protected-Path Governance Accessibility, Headless, and Export Parity

- Packet: `protected-path-governance-accessibility:stable:0001`
- Surface: `Protected-path governance accessibility, headless, and export parity`
- Accessibility rows: 8 (6 claim-narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Accessibility rows

- **protected_path_row** [`row:protected-path-trusted`]: condition `governance_truth_trusted`, claim `full_governed_authority`
- **ownership_card** [`row:ownership-card-owner-coverage`]: condition `owner_coverage_partial`, claim `owner_backup_coverage_missing`
- **approver_matrix** [`row:approver-matrix-approver-state`]: condition `approver_state_stale_or_partial`, claim `approver_state_narrowed`
- **review_pack_summary** [`row:review-pack-summary-stale`]: condition `review_pack_freshness_stale`, claim `review_pack_stale_disclosed`
- **public_surface_diff_card** [`row:public-surface-diff-partial`]: condition `public_surface_diff_truth_partial`, claim `public_surface_evidence_withheld`
- **merge_control_banner** [`row:merge-control-banner-enforcement`]: condition `provider_enforcement_stale_or_partial`, claim `advisory_enforcement_only`
- **dri_registry_row** [`row:dri-registry-trusted`]: condition `governance_truth_trusted`, claim `full_governed_authority`
- **merge_readiness_strip** [`row:merge-readiness-strip-enforcement`]: condition `provider_enforcement_stale_or_partial`, claim `advisory_enforcement_only`
