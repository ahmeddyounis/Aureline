use super::*;

const PACKET_ID: &str = "review-export-bundle:stable:0001";

fn bundle_rows() -> Vec<ExportBundleRow> {
    vec![
        ExportBundleRow {
            bundle_id: "bundle:review-thread".to_owned(),
            scope_class: BundleScopeClass::ReviewThreadBundle,
            bundle_label: "Review thread bundle".to_owned(),
            supports_publish_later: true,
            supports_offline_replay: true,
            supports_redacted_export: true,
            coverage_label: "Publish-later, offline replay, and redacted export".to_owned(),
            disclosure_label: "Review thread bundle with disclosed capabilities".to_owned(),
        },
        ExportBundleRow {
            bundle_id: "bundle:ci-run".to_owned(),
            scope_class: BundleScopeClass::CiRunBundle,
            bundle_label: "CI run bundle".to_owned(),
            supports_publish_later: true,
            supports_offline_replay: true,
            supports_redacted_export: false,
            coverage_label: "Publish-later and offline replay; no redacted export".to_owned(),
            disclosure_label: "CI run bundle with disclosed capabilities".to_owned(),
        },
        ExportBundleRow {
            bundle_id: "bundle:provider-owned".to_owned(),
            scope_class: BundleScopeClass::UnknownScopeProviderOwned,
            bundle_label: "Provider-owned bundle".to_owned(),
            supports_publish_later: false,
            supports_offline_replay: false,
            supports_redacted_export: false,
            coverage_label: "Provider-owned bundle with no disclosed capabilities".to_owned(),
            disclosure_label: "Unknown provider-owned bundle; capabilities not assumed".to_owned(),
        },
    ]
}

fn export_rows() -> Vec<BundleExportRow> {
    vec![
        BundleExportRow {
            export_id: "export:review-draft".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            bundle_id: "bundle:review-thread".to_owned(),
            subject_label: "Review thread bundle for feature/login dashboard".to_owned(),
            provenance: BundleProvenance {
                scope_class: BundleScopeClass::ReviewThreadBundle,
                trust_class: BundleTrustClass::FirstPartyTrusted,
                freshness_class: BundleFreshnessClass::FreshCurrentTruth,
                source_label: "First-party review thread source".to_owned(),
                identity_disclosed: true,
            },
            redaction: BundleRedactionDisclosure {
                redaction_class: BundleRedactionClass::MetadataOnly,
                redaction_disclosed: true,
                redaction_label: "Metadata-only export, no bodies".to_owned(),
            },
            publish_disposition: PublishDisposition {
                publish_state: PublishStateClass::HeldDraft,
                publish_disclosed: true,
                read_only: true,
                publish_label: "Held as a read-only draft".to_owned(),
                publish_ref: None,
            },
            follow_up_action: FollowUpAction {
                connectivity_class: FollowUpConnectivityClass::Online,
                disposition_class: FollowUpDispositionClass::NoPendingFollowUp,
                action_disclosed: true,
                replay_ready: false,
                action_label: "No pending follow-up on an online surface".to_owned(),
            },
            blocked_class: ExportBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:export-review-draft:0001".to_owned(),
            attention_reasons: Vec::new(),
            review_summary: "Held review bundle draft with a fresh, first-party source".to_owned(),
            source_contract_refs: vec![
                REVIEW_EXPORT_BUNDLE_HANDOFF_CONTRACT_REF.to_owned(),
                REVIEW_EXPORT_BUNDLE_TRUST_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
        BundleExportRow {
            export_id: "export:ci-publish-later".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            bundle_id: "bundle:ci-run".to_owned(),
            subject_label: "CI run bundle queued to publish later".to_owned(),
            provenance: BundleProvenance {
                scope_class: BundleScopeClass::CiRunBundle,
                trust_class: BundleTrustClass::ProviderVerified,
                freshness_class: BundleFreshnessClass::FreshCurrentTruth,
                source_label: "Provider-verified CI run source".to_owned(),
                identity_disclosed: true,
            },
            redaction: BundleRedactionDisclosure {
                redaction_class: BundleRedactionClass::MetadataOnly,
                redaction_disclosed: true,
                redaction_label: "Metadata-only export, no log bodies".to_owned(),
            },
            publish_disposition: PublishDisposition {
                publish_state: PublishStateClass::QueuedToPublish,
                publish_disclosed: true,
                read_only: false,
                publish_label: "Queued to publish the CI run bundle later".to_owned(),
                publish_ref: Some("publish:ci-run-bundle-route".to_owned()),
            },
            follow_up_action: FollowUpAction {
                connectivity_class: FollowUpConnectivityClass::OfflineQueued,
                disposition_class: FollowUpDispositionClass::ReplayOnReconnect,
                action_disclosed: true,
                replay_ready: false,
                action_label: "Queued offline; will replay once reconnected and authorized"
                    .to_owned(),
            },
            blocked_class: ExportBlockedClass::NotBlocked,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:export-ci-publish-later:0002".to_owned(),
            attention_reasons: vec![
                "Follow-up was queued while offline and awaits reconnect authority".to_owned(),
            ],
            review_summary:
                "Attributed publish-later for a fresh CI run bundle with an offline follow-up"
                    .to_owned(),
            source_contract_refs: vec![
                REVIEW_EXPORT_BUNDLE_MERGE_QUEUE_CONTRACT_REF.to_owned(),
                REVIEW_EXPORT_BUNDLE_PIPELINE_CONTRACT_REF.to_owned(),
            ],
        },
        BundleExportRow {
            export_id: "export:provider-blocked".to_owned(),
            durable_anchor_id: "anchor:review:0003".to_owned(),
            bundle_id: "bundle:provider-owned".to_owned(),
            subject_label: "Provider-owned bundle with no resolvable origin".to_owned(),
            provenance: BundleProvenance {
                scope_class: BundleScopeClass::UnknownScopeProviderOwned,
                trust_class: BundleTrustClass::UnknownTrustProviderOwned,
                freshness_class: BundleFreshnessClass::UnknownFreshnessProviderOwned,
                source_label: "Unknown provider-owned source".to_owned(),
                identity_disclosed: true,
            },
            redaction: BundleRedactionDisclosure {
                redaction_class: BundleRedactionClass::UnredactedBlocked,
                redaction_disclosed: true,
                redaction_label: "Unredacted export blocked from leaving the boundary".to_owned(),
            },
            publish_disposition: PublishDisposition {
                publish_state: PublishStateClass::PublishBlocked,
                publish_disclosed: true,
                read_only: true,
                publish_label: "Publish blocked; unredacted provider-owned bundle".to_owned(),
                publish_ref: None,
            },
            follow_up_action: FollowUpAction {
                connectivity_class: FollowUpConnectivityClass::UnknownConnectivityProviderOwned,
                disposition_class: FollowUpDispositionClass::BlockedPendingTruth,
                action_disclosed: true,
                replay_ready: false,
                action_label: "Follow-up blocked pending fresh, trusted truth".to_owned(),
            },
            blocked_class: ExportBlockedClass::BlockedUnredactedExport,
            actor_attribution_label: "Signed-in human account".to_owned(),
            audit_row_ref: "audit:export-provider-blocked:0003".to_owned(),
            attention_reasons: vec![
                "Provider-owned scope, trust, and freshness are not assumed".to_owned(),
                "Export is unredacted and blocked from leaving the boundary".to_owned(),
                "Follow-up is blocked pending fresh, trusted truth".to_owned(),
            ],
            review_summary: "Blocked export for an unredacted, provider-owned bundle".to_owned(),
            source_contract_refs: vec![
                REVIEW_EXPORT_BUNDLE_PIPELINE_CONTRACT_REF.to_owned(),
                REVIEW_EXPORT_BUNDLE_TRUST_CLASS_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn trust_review() -> ReviewExportBundleTrustReview {
    ReviewExportBundleTrustReview {
        bundle_provenance_disclosed: true,
        bundle_trust_explicit: true,
        bundle_redaction_disclosed: true,
        truth_freshness_disclosed: true,
        publish_disposition_disclosed: true,
        follow_up_disclosed: true,
        publish_read_only_unless_attributed: true,
        every_export_anchored: true,
        every_action_attributable: true,
        no_hidden_publish_scope: true,
        stale_truth_narrows_publish: true,
        offline_replay_requires_authority: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ReviewExportBundleConsumerProjection {
    ReviewExportBundleConsumerProjection {
        review_workspace_header_shows_anchor: true,
        merge_queue_panel_shows_freshness: true,
        pipeline_run_viewer_shows_provenance: true,
        export_bundle_panel_shows_redaction: true,
        publish_later_queue_shows_disposition: true,
        offline_follow_up_tray_shows_connectivity: true,
        command_palette_shows_bundle_state: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        label_for_unqualified: true,
    }
}

fn proof_freshness() -> ReviewExportBundleProofFreshness {
    ReviewExportBundleProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ReviewExportBundleDowngradeTrigger> {
    vec![
        ReviewExportBundleDowngradeTrigger::ProofStale,
        ReviewExportBundleDowngradeTrigger::PublishAttributionMissing,
        ReviewExportBundleDowngradeTrigger::TruthStale,
        ReviewExportBundleDowngradeTrigger::BundleRedactionUnverified,
        ReviewExportBundleDowngradeTrigger::BundleTrustUnknown,
        ReviewExportBundleDowngradeTrigger::OfflineReplayUnauthorized,
        ReviewExportBundleDowngradeTrigger::FollowUpUnattributed,
        ReviewExportBundleDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<ReviewExportBundleConsumerSurface> {
    vec![
        ReviewExportBundleConsumerSurface::ReviewWorkspaceHeader,
        ReviewExportBundleConsumerSurface::MergeQueuePanel,
        ReviewExportBundleConsumerSurface::PipelineRunViewer,
        ReviewExportBundleConsumerSurface::ExportBundlePanel,
        ReviewExportBundleConsumerSurface::PublishLaterQueue,
        ReviewExportBundleConsumerSurface::OfflineFollowUpTray,
        ReviewExportBundleConsumerSurface::CliHeadless,
        ReviewExportBundleConsumerSurface::SupportExport,
        ReviewExportBundleConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REVIEW_EXPORT_BUNDLE_SCHEMA_REF.to_owned(),
        REVIEW_EXPORT_BUNDLE_DOC_REF.to_owned(),
        REVIEW_EXPORT_BUNDLE_HANDOFF_CONTRACT_REF.to_owned(),
        REVIEW_EXPORT_BUNDLE_MERGE_QUEUE_CONTRACT_REF.to_owned(),
        REVIEW_EXPORT_BUNDLE_PIPELINE_CONTRACT_REF.to_owned(),
        REVIEW_EXPORT_BUNDLE_TRUST_CLASS_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> ReviewExportBundlePacket {
    ReviewExportBundlePacket::new(ReviewExportBundlePacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Review/export bundles, publish-later packets, and offline follow-up flows"
            .to_owned(),
        bundle_rows: bundle_rows(),
        export_rows: export_rows(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn review_export_bundle_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_bundle_rows_fails() {
    let mut packet = packet();
    packet.bundle_rows.clear();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::BundleRowsMissing));
}

#[test]
fn incomplete_bundle_row_fails() {
    let mut packet = packet();
    packet.bundle_rows[0].bundle_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::BundleRowIncomplete));
}

#[test]
fn missing_export_rows_fails() {
    let mut packet = packet();
    packet.export_rows.clear();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::ExportRowsMissing));
}

#[test]
fn incomplete_export_row_fails() {
    let mut packet = packet();
    packet.export_rows[0].subject_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::ExportRowIncomplete));
}

#[test]
fn orphan_bundle_reference_fails() {
    let mut packet = packet();
    packet.export_rows[0].bundle_id = "bundle:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::OrphanBundleReference));
}

#[test]
fn undisclosed_provenance_fails() {
    let mut packet = packet();
    packet.export_rows[0].provenance.identity_disclosed = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::BundleProvenanceUndisclosed));
}

#[test]
fn empty_source_label_fails() {
    let mut packet = packet();
    packet.export_rows[0].provenance.source_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::BundleProvenanceUndisclosed));
}

#[test]
fn undisclosed_redaction_fails() {
    let mut packet = packet();
    packet.export_rows[0].redaction.redaction_disclosed = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::BundleRedactionUndisclosed));
}

#[test]
fn undisclosed_publish_disposition_fails() {
    let mut packet = packet();
    packet.export_rows[0].publish_disposition.publish_disclosed = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::PublishDispositionUndisclosed));
}

#[test]
fn publish_read_only_mismatch_fails() {
    let mut packet = packet();
    packet.export_rows[0].publish_disposition.read_only = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::PublishReadOnlyMismatch));
}

#[test]
fn publish_without_ref_fails() {
    let mut packet = packet();
    packet.export_rows[1].publish_disposition.publish_ref = None;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::PublishRefMissing));
}

#[test]
fn blocked_publish_not_marked_fails() {
    let mut packet = packet();
    packet.export_rows[2].blocked_class = ExportBlockedClass::NotBlocked;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::PublishBlockedNotMarked));
}

#[test]
fn undisclosed_follow_up_fails() {
    let mut packet = packet();
    packet.export_rows[0].follow_up_action.action_disclosed = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::FollowUpUndisclosed));
}

#[test]
fn offline_replay_without_authority_fails() {
    let mut packet = packet();
    packet.export_rows[1].follow_up_action.replay_ready = true;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::OfflineReplayWithoutAuthority));
}

#[test]
fn missing_attribution_fails() {
    let mut packet = packet();
    packet.export_rows[0].actor_attribution_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::AttributionMissing));
}

#[test]
fn missing_audit_row_ref_fails() {
    let mut packet = packet();
    packet.export_rows[1].audit_row_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::AttributionMissing));
}

#[test]
fn attention_required_without_reason_fails() {
    let mut packet = packet();
    packet.export_rows[1].attention_reasons.clear();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::AttentionReasonMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_publish_scope = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .publish_later_queue_shows_disposition = false;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ReviewExportBundleViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Export bundles"));
    assert!(summary.contains("## Exports"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_review_export_bundle_export()
        .expect("checked review export bundle export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/stale_truth_publish_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/offline_replay_held.json"
        )),
    ] {
        let packet: ReviewExportBundlePacket =
            serde_json::from_str(raw).expect("fixture parses as review export bundle packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
