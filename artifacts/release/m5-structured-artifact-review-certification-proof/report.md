# Structured-Artifact Review-Component Surface Certification

- Packet: `structured-artifact-certification:stable:0001`
- Label: `Structured-artifact review-component surface certification`
- Surfaces: 8 (4 certified, 4 narrowed, 0 blocked)
- All surfaces preserve component truth: true
- Note: 4 surface(s) certified, 4 narrowed; all preserve component truth
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Certified surfaces

- **diff_toolbar_surface** [`cert:diff-toolbar`]: `certified_parity` (claimed `full_structured_fidelity`, certified `full_structured_fidelity`)
- **merge_sheet_surface** [`cert:merge-sheet`]: `certified_parity` (claimed `full_structured_fidelity`, certified `full_structured_fidelity`)
- **help_artifact_surface** [`cert:help`]: `certified_parity` (claimed `full_structured_fidelity`, certified `full_structured_fidelity`)
- **cli_headless** [`cert:cli`]: `certified_parity` (claimed `full_structured_fidelity`, certified `full_structured_fidelity`)
- **review_workspace_surface** [`cert:workspace`]: `narrowed_parity` (claimed `full_structured_fidelity`, certified `structured_compare_only`)
- **support_export** [`cert:support-export`]: `narrowed_parity` (claimed `full_structured_fidelity`, certified `metadata_withheld`)
- **exported_artifact_packet** [`cert:exported-packet`]: `narrowed_parity` (claimed `full_structured_fidelity`, certified `raw_fallback_disclosed`)
- **diagnostics** [`cert:diagnostics`]: `narrowed_parity` (claimed `full_structured_fidelity`, certified `partial_structure`)
