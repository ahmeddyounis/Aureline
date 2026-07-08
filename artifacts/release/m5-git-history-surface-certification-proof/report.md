# Git-History Surface Certification

- Packet: `git-history-certification:stable:0001`
- Label: `Git-history surface certification`
- Surfaces: 8 (4 certified, 4 narrowed, 0 blocked)
- All surfaces preserve component truth: true
- Note: 4 surface(s) certified, 4 narrowed; all preserve component truth
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Certified surfaces

- **history_sidebar** [`cert:history-sidebar`]: `certified_parity` (claimed `recoverable_in_product`, certified `recoverable_in_product`)
- **review_workspace** [`cert:review-workspace`]: `certified_parity` (claimed `recoverable_in_product`, certified `recoverable_in_product`)
- **help_git_surface** [`cert:help`]: `certified_parity` (claimed `recoverable_in_product`, certified `recoverable_in_product`)
- **cli_headless** [`cert:cli`]: `certified_parity` (claimed `recoverable_in_product`, certified `recoverable_in_product`)
- **risky_mutation_sheet** [`cert:risky-mutation-sheet`]: `narrowed_parity` (claimed `recoverable_in_product`, certified `locally_recoverable`)
- **exported_recovery_packet** [`cert:exported-recovery-packet`]: `narrowed_parity` (claimed `recoverable_in_product`, certified `local_continue_only`)
- **support_export** [`cert:support-export`]: `narrowed_parity` (claimed `recoverable_in_product`, certified `reflog_only_recovery`)
- **diagnostics** [`cert:diagnostics`]: `narrowed_parity` (claimed `recoverable_in_product`, certified `partial_history_only`)
