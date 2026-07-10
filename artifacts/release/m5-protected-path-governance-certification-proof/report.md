# Protected-Path Governance-Component Surface Certification

- Packet: `protected-path-governance-certification:stable:0001`
- Label: `Protected-path governance-component surface certification`
- Surfaces: 8 (4 certified, 4 narrowed, 0 blocked)
- All surfaces preserve component truth: true
- Note: 4 surface(s) certified, 4 narrowed; all preserve component truth
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Certified surfaces

- **review_workspace_surface** [`cert:review-workspace`]: `certified_parity` (claimed `full_governed_authority`, certified `full_governed_authority`)
- **merge_queue_surface** [`cert:merge-queue`]: `certified_parity` (claimed `full_governed_authority`, certified `full_governed_authority`)
- **help_governance_surface** [`cert:help`]: `certified_parity` (claimed `full_governed_authority`, certified `full_governed_authority`)
- **cli_headless** [`cert:cli`]: `certified_parity` (claimed `full_governed_authority`, certified `full_governed_authority`)
- **release_center_surface** [`cert:release-center`]: `narrowed_parity` (claimed `full_governed_authority`, certified `public_surface_evidence_withheld`)
- **support_export** [`cert:support-export`]: `narrowed_parity` (claimed `full_governed_authority`, certified `owner_backup_coverage_missing`)
- **exported_governance_packet** [`cert:exported-packet`]: `narrowed_parity` (claimed `full_governed_authority`, certified `advisory_enforcement_only`)
- **shiproom_surface** [`cert:shiproom`]: `narrowed_parity` (claimed `full_governed_authority`, certified `review_pack_stale_disclosed`)
