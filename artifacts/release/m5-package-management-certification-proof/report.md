# Package-Management Component Surface Certification

- Packet: `package-certification:stable:0001`
- Label: `Package-management component surface certification`
- Surfaces: 8 (4 certified, 4 narrowed, 0 blocked)
- All surfaces preserve component truth: true
- Note: 4 surface(s) certified, 4 narrowed; all preserve component truth
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Certified surfaces

- **package_explorer_surface** [`cert:package-explorer`]: `certified_parity` (claimed `full_reviewable_management`, certified `full_reviewable_management`)
- **dependency_search_detail_surface** [`cert:search-detail`]: `certified_parity` (claimed `full_reviewable_management`, certified `full_reviewable_management`)
- **help_package_surface** [`cert:help`]: `certified_parity` (claimed `full_reviewable_management`, certified `full_reviewable_management`)
- **cli_headless** [`cert:cli`]: `certified_parity` (claimed `full_reviewable_management`, certified `full_reviewable_management`)
- **install_review_sheet_surface** [`cert:install-review`]: `narrowed_parity` (claimed `full_reviewable_management`, certified `lockfile_impact_unknown`)
- **support_export** [`cert:support-export`]: `narrowed_parity` (claimed `full_reviewable_management`, certified `auth_required_read_only`)
- **exported_package_review_packet** [`cert:exported-packet`]: `narrowed_parity` (claimed `full_reviewable_management`, certified `mirror_or_offline_sourced`)
- **diagnostics** [`cert:diagnostics`]: `narrowed_parity` (claimed `full_reviewable_management`, certified `manifest_range_scoped`)
