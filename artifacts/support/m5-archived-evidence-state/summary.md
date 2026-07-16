# Archived-Object Expiry / Removal State: One Vocabulary Across Surfaces

- Packet: `m5-archived-evidence-state:stable:0001`
- Surface: `M5 archived-object expiry / removal state (one vocabulary across surfaces)`
- State bindings: 15 (11 disclosing removal / expiry)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## State bindings

- **Retirement / last-supported snapshot (expiry / removal state)** [`aes-retirement-release`]: object `retirement_snapshot` on `release_center`, state `preserved_available`, content-present `true`, reason `none`, role `expiry_removal_handling`
- **Retirement / last-supported snapshot (expiry / removal state)** [`aes-retirement-shell`]: object `retirement_snapshot` on `shell`, state `expired`, content-present `true`, reason `retention_window_elapsed`, role `expiry_removal_handling`
- **Retirement / last-supported snapshot (expiry / removal state)** [`aes-retirement-cli`]: object `retirement_snapshot` on `cli_export`, state `missing_live_target`, content-present `true`, reason `source_live_target_removed`, role `expiry_removal_handling`
- **Captured support / export evidence (expiry / removal state)** [`aes-support-evidence-support`]: object `support_export_evidence` on `support`, state `preserved_available`, content-present `true`, reason `none`, role `provenance_attribution`
- **Captured support / export evidence (expiry / removal state)** [`aes-support-evidence-help`]: object `support_export_evidence` on `help_docs`, state `removed`, content-present `false`, reason `policy_mandated_deletion`, role `provenance_attribution`
- **Captured support / export evidence (expiry / removal state)** [`aes-support-evidence-companion`]: object `support_export_evidence` on `companion_export`, state `metadata_only`, content-present `false`, reason `metadata_only_by_design`, role `provenance_attribution`
- **Archived runbook execution packet (expiry / removal state)** [`aes-runbook-archive`]: object `archived_runbook_packet` on `runbook_archive`, state `retention_window_ended`, content-present `true`, reason `retention_window_elapsed`, role `snapshot_labeling`
- **Archived runbook execution packet (expiry / removal state)** [`aes-runbook-review`]: object `archived_runbook_packet` on `review_incident`, state `removed`, content-present `false`, reason `storage_reclaimed`, role `snapshot_labeling`
- **Archived runbook execution packet (expiry / removal state)** [`aes-runbook-program`]: object `archived_runbook_packet` on `program_governance`, state `preserved_available`, content-present `true`, reason `none`, role `snapshot_labeling`
- **Imported / offline route evidence (expiry / removal state)** [`aes-imported-shell`]: object `imported_offline_route_evidence` on `shell`, state `expired`, content-present `true`, reason `storage_reclaimed`, role `imported_offline_disclosure`
- **Imported / offline route evidence (expiry / removal state)** [`aes-imported-runbook`]: object `imported_offline_route_evidence` on `runbook_archive`, state `metadata_only`, content-present `false`, reason `metadata_only_by_design`, role `imported_offline_disclosure`
- **Imported / offline route evidence (expiry / removal state)** [`aes-imported-cli`]: object `imported_offline_route_evidence` on `cli_export`, state `missing_live_target`, content-present `true`, reason `source_live_target_removed`, role `imported_offline_disclosure`
- **Review / incident snapshot (expiry / removal state)** [`aes-review-review`]: object `review_incident_snapshot` on `review_incident`, state `preserved_available`, content-present `true`, reason `none`, role `mutation_blocked_posture`
- **Review / incident snapshot (expiry / removal state)** [`aes-review-shell`]: object `review_incident_snapshot` on `shell`, state `retention_window_ended`, content-present `true`, reason `retention_window_elapsed`, role `mutation_blocked_posture`
- **Review / incident snapshot (expiry / removal state)** [`aes-review-companion`]: object `review_incident_snapshot` on `companion_export`, state `removed`, content-present `false`, reason `manual_cleanup_requested`, role `mutation_blocked_posture`
