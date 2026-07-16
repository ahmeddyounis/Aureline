# Historical-vs-Live Compare Flows: One Vocabulary Across Surfaces

- Packet: `m5-historical-versus-live-compare-flow:stable:0001`
- Surface: `M5 historical-vs-live compare flows (one vocabulary across surfaces)`
- Compare bindings: 15 (11 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Compare bindings

- **Retirement / last-supported snapshot (compare against current line)** [`hvlc-retirement-release`]: object `retirement_snapshot` on `release_center`, outcome `live_target_paired`, identity `same_object_identity`, freshness `in_sync_no_drift`, role `snapshot_labeling`
- **Retirement / last-supported snapshot (compare against current line)** [`hvlc-retirement-shell`]: object `retirement_snapshot` on `shell`, outcome `approximate_pairing`, identity `approximate_identity`, freshness `in_sync_no_drift`, role `snapshot_labeling`
- **Retirement / last-supported snapshot (compare against current line)** [`hvlc-retirement-cli`]: object `retirement_snapshot` on `cli_export`, outcome `policy_blocked_pairing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `snapshot_labeling`
- **Captured support / export evidence (compare against current object)** [`hvlc-support-evidence-support`]: object `support_export_evidence` on `support`, outcome `live_target_paired`, identity `same_object_identity`, freshness `snapshot_behind_live`, role `provenance_attribution`
- **Captured support / export evidence (compare against current object)** [`hvlc-support-evidence-help`]: object `support_export_evidence` on `help_docs`, outcome `live_target_missing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `provenance_attribution`
- **Captured support / export evidence (compare against current object)** [`hvlc-support-evidence-companion`]: object `support_export_evidence` on `companion_export`, outcome `policy_blocked_pairing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `provenance_attribution`
- **Archived runbook execution packet (compare against current run)** [`hvlc-runbook-archive`]: object `archived_runbook_packet` on `runbook_archive`, outcome `live_target_paired`, identity `same_object_identity`, freshness `snapshot_diverged_from_live`, role `live_target_handoff`
- **Archived runbook execution packet (compare against current run)** [`hvlc-runbook-review`]: object `archived_runbook_packet` on `review_incident`, outcome `live_target_missing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `live_target_handoff`
- **Archived runbook execution packet (compare against current run)** [`hvlc-runbook-program`]: object `archived_runbook_packet` on `program_governance`, outcome `approximate_pairing`, identity `approximate_identity`, freshness `snapshot_diverged_from_live`, role `live_target_handoff`
- **Imported / offline route evidence (compare against current route)** [`hvlc-imported-shell`]: object `imported_offline_route_evidence` on `shell`, outcome `live_target_missing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `imported_offline_disclosure`
- **Imported / offline route evidence (compare against current route)** [`hvlc-imported-runbook`]: object `imported_offline_route_evidence` on `runbook_archive`, outcome `policy_blocked_pairing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `imported_offline_disclosure`
- **Imported / offline route evidence (compare against current route)** [`hvlc-imported-cli`]: object `imported_offline_route_evidence` on `cli_export`, outcome `approximate_pairing`, identity `approximate_identity`, freshness `in_sync_no_drift`, role `imported_offline_disclosure`
- **Review / incident snapshot (compare against current object)** [`hvlc-review-review`]: object `review_incident_snapshot` on `review_incident`, outcome `live_target_paired`, identity `same_object_identity`, freshness `snapshot_behind_live`, role `mutation_blocked_posture`
- **Review / incident snapshot (compare against current object)** [`hvlc-review-shell`]: object `review_incident_snapshot` on `shell`, outcome `live_target_missing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `mutation_blocked_posture`
- **Review / incident snapshot (compare against current object)** [`hvlc-review-companion`]: object `review_incident_snapshot` on `companion_export`, outcome `policy_blocked_pairing`, identity `identity_unverifiable`, freshness `freshness_unverifiable`, role `mutation_blocked_posture`
