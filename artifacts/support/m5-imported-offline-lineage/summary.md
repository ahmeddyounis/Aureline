# Imported / Offline Evidence Lineage: One Vocabulary Across Consumers

- Packet: `m5-imported-offline-lineage:stable:0001`
- Surface: `M5 imported / offline evidence lineage propagation (one vocabulary across consumers)`
- Lineage bindings: 15 (4 live-target joinable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Lineage bindings

- **Retirement / last-supported snapshot (imported / offline lineage)** [`iol-retirement-release`]: object `retirement_snapshot` on `release_center`, disposition `live_target_joinable`, content-available `true`, role `live_target_handoff`
- **Retirement / last-supported snapshot (imported / offline lineage)** [`iol-retirement-shell`]: object `retirement_snapshot` on `shell`, disposition `imported_offline_only`, content-available `true`, role `live_target_handoff`
- **Retirement / last-supported snapshot (imported / offline lineage)** [`iol-retirement-cli`]: object `retirement_snapshot` on `cli_export`, disposition `metadata_only_exit`, content-available `false`, role `live_target_handoff`
- **Captured support / export evidence (imported / offline lineage)** [`iol-support-evidence-support`]: object `support_export_evidence` on `support`, disposition `live_target_joinable`, content-available `true`, role `provenance_attribution`
- **Captured support / export evidence (imported / offline lineage)** [`iol-support-evidence-help`]: object `support_export_evidence` on `help_docs`, disposition `exported_redacted_lineage`, content-available `true`, role `provenance_attribution`
- **Captured support / export evidence (imported / offline lineage)** [`iol-support-evidence-companion`]: object `support_export_evidence` on `companion_export`, disposition `imported_offline_only`, content-available `true`, role `provenance_attribution`
- **Archived runbook execution packet (imported / offline lineage)** [`iol-runbook-archive`]: object `archived_runbook_packet` on `runbook_archive`, disposition `imported_offline_only`, content-available `true`, role `snapshot_labeling`
- **Archived runbook execution packet (imported / offline lineage)** [`iol-runbook-review`]: object `archived_runbook_packet` on `review_incident`, disposition `metadata_only_exit`, content-available `false`, role `snapshot_labeling`
- **Archived runbook execution packet (imported / offline lineage)** [`iol-runbook-program`]: object `archived_runbook_packet` on `program_governance`, disposition `live_target_joinable`, content-available `true`, role `snapshot_labeling`
- **Imported / offline route evidence (imported / offline lineage)** [`iol-imported-shell`]: object `imported_offline_route_evidence` on `shell`, disposition `imported_offline_only`, content-available `true`, role `imported_offline_disclosure`
- **Imported / offline route evidence (imported / offline lineage)** [`iol-imported-runbook`]: object `imported_offline_route_evidence` on `runbook_archive`, disposition `metadata_only_exit`, content-available `false`, role `imported_offline_disclosure`
- **Imported / offline route evidence (imported / offline lineage)** [`iol-imported-cli`]: object `imported_offline_route_evidence` on `cli_export`, disposition `exported_redacted_lineage`, content-available `true`, role `imported_offline_disclosure`
- **Review / incident snapshot (imported / offline lineage)** [`iol-review-review`]: object `review_incident_snapshot` on `review_incident`, disposition `live_target_joinable`, content-available `true`, role `mutation_blocked_posture`
- **Review / incident snapshot (imported / offline lineage)** [`iol-review-shell`]: object `review_incident_snapshot` on `shell`, disposition `exported_redacted_lineage`, content-available `true`, role `mutation_blocked_posture`
- **Review / incident snapshot (imported / offline lineage)** [`iol-review-companion`]: object `review_incident_snapshot` on `companion_export`, disposition `imported_offline_only`, content-available `true`, role `mutation_blocked_posture`
