use super::*;

const PACKET_ID: &str = "m5-review-ci-preview-matrix:stable:0001";

fn lane_rows() -> Vec<M5ReviewCiPreviewMatrixLaneRow> {
    vec![
        M5ReviewCiPreviewMatrixLaneRow {
            lane: M5ReviewCiPreviewLane::ReviewWorkspace,
            qualification: M5ReviewCiPreviewQualificationClass::Stable,
            scope_summary: "Review workspace with durable anchors across edits, rebases, and reopens; stale-base and outdated-diff states are labeled, never silently hidden, and approval state resets on base change".to_owned(),
            evidence_requirement: M5ReviewCiPreviewEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:review-workspace-anchor-durability:m5".to_owned(),
                "evidence:stale-base-approval-reset:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewCiPreviewDowngradeTrigger::ProofStale,
                M5ReviewCiPreviewDowngradeTrigger::AnchorDrift,
                M5ReviewCiPreviewDowngradeTrigger::TrustNarrowing,
                M5ReviewCiPreviewDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ReviewCiPreviewRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_REVIEW_CI_PREVIEW_MATRIX_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewCiPreviewConsumerSurface::ReviewWorkspace,
                M5ReviewCiPreviewConsumerSurface::CliHeadless,
                M5ReviewCiPreviewConsumerSurface::SupportExport,
                M5ReviewCiPreviewConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewCiPreviewMatrixLaneRow {
            lane: M5ReviewCiPreviewLane::MergeQueue,
            qualification: M5ReviewCiPreviewQualificationClass::Stable,
            scope_summary: "Merge queue with fresh CI-status truth relative to the head it gates; every rerun and cancel action stays individually attributable and reviewable, with no hidden write scope".to_owned(),
            evidence_requirement: M5ReviewCiPreviewEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:merge-queue-ci-status-freshness:m5".to_owned(),
                "evidence:rerun-cancel-authority-attribution:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewCiPreviewDowngradeTrigger::ProofStale,
                M5ReviewCiPreviewDowngradeTrigger::MergeQueueStatusStale,
                M5ReviewCiPreviewDowngradeTrigger::PolicyBlocked,
                M5ReviewCiPreviewDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ReviewCiPreviewRollbackPosture::AttributableRerunOrCancel,
            source_contract_refs: vec![
                M5_REVIEW_CI_PREVIEW_MATRIX_MERGE_QUEUE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewCiPreviewConsumerSurface::MergeQueuePanel,
                M5ReviewCiPreviewConsumerSurface::CliHeadless,
                M5ReviewCiPreviewConsumerSurface::SupportExport,
                M5ReviewCiPreviewConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewCiPreviewMatrixLaneRow {
            lane: M5ReviewCiPreviewLane::PipelineViewer,
            qualification: M5ReviewCiPreviewQualificationClass::Stable,
            scope_summary: "Pipeline viewer rendering logs and artifacts through the safe-preview boundary; suspicious content stays neutralized and no raw build log or artifact body crosses the support boundary".to_owned(),
            evidence_requirement: M5ReviewCiPreviewEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:pipeline-log-artifact-safe-preview:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewCiPreviewDowngradeTrigger::ProofStale,
                M5ReviewCiPreviewDowngradeTrigger::SafePreviewUnavailable,
                M5ReviewCiPreviewDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5ReviewCiPreviewRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_REVIEW_CI_PREVIEW_MATRIX_PIPELINE_VIEWER_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewCiPreviewConsumerSurface::PipelineViewer,
                M5ReviewCiPreviewConsumerSurface::CliHeadless,
                M5ReviewCiPreviewConsumerSurface::SupportExport,
                M5ReviewCiPreviewConsumerSurface::Diagnostics,
            ],
        },
        M5ReviewCiPreviewMatrixLaneRow {
            lane: M5ReviewCiPreviewLane::RemotePreview,
            qualification: M5ReviewCiPreviewQualificationClass::Beta,
            scope_summary: "Time-bounded, attributable remote preview routes that auto-expire at their bound and stay attributable to their opener and origin; persistent hosting and unbounded sharing stay out of scope".to_owned(),
            evidence_requirement: M5ReviewCiPreviewEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:remote-preview-route-time-bound:m5".to_owned(),
                "evidence:remote-preview-attribution:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ReviewCiPreviewDowngradeTrigger::ProofStale,
                M5ReviewCiPreviewDowngradeTrigger::PreviewRouteExpired,
                M5ReviewCiPreviewDowngradeTrigger::PolicyBlocked,
                M5ReviewCiPreviewDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5ReviewCiPreviewRollbackPosture::TimeBoundedAutoExpire,
            source_contract_refs: vec![
                M5_REVIEW_CI_PREVIEW_MATRIX_REMOTE_PREVIEW_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ReviewCiPreviewConsumerSurface::RemotePreviewPanel,
                M5ReviewCiPreviewConsumerSurface::BrowserCompanion,
                M5ReviewCiPreviewConsumerSurface::SupportExport,
                M5ReviewCiPreviewConsumerSurface::HelpAbout,
            ],
        },
    ]
}

fn trust_review() -> M5ReviewCiPreviewMatrixTrustReview {
    M5ReviewCiPreviewMatrixTrustReview {
        review_workspace_anchors_durable: true,
        stale_base_labels_explicit: true,
        merge_queue_truth_fresh: true,
        rerun_cancel_authority_attributable: true,
        pipeline_logs_artifacts_safe_previewed: true,
        remote_preview_time_bounded: true,
        remote_preview_attributable: true,
        browser_handoff_return_path_safe: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5ReviewCiPreviewMatrixConsumerProjection {
    M5ReviewCiPreviewMatrixConsumerProjection {
        review_workspace_shows_anchor_and_stale_base: true,
        merge_queue_shows_ci_status_and_freshness: true,
        pipeline_viewer_shows_safe_preview_state: true,
        remote_preview_shows_expiry_and_attribution: true,
        cli_headless_shows_qualification: true,
        support_export_shows_qualification: true,
        diagnostics_shows_qualification: true,
        help_about_shows_qualification: true,
        preview_labs_label_for_unqualified_lanes: true,
    }
}

fn proof_freshness() -> M5ReviewCiPreviewMatrixProofFreshness {
    M5ReviewCiPreviewMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_REF.to_owned(),
        M5_REVIEW_CI_PREVIEW_MATRIX_DOC_REF.to_owned(),
        M5_REVIEW_CI_PREVIEW_MATRIX_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_REVIEW_CI_PREVIEW_MATRIX_MERGE_QUEUE_CONTRACT_REF.to_owned(),
        M5_REVIEW_CI_PREVIEW_MATRIX_PIPELINE_VIEWER_CONTRACT_REF.to_owned(),
        M5_REVIEW_CI_PREVIEW_MATRIX_REMOTE_PREVIEW_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5ReviewCiPreviewMatrixPacket {
    M5ReviewCiPreviewMatrixPacket::new(M5ReviewCiPreviewMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Review, Merge-Queue, and Pipeline Maturity Matrix".to_owned(),
        lane_rows: lane_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_review_ci_preview_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.lane != M5ReviewCiPreviewLane::PipelineViewer);
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::RequiredLaneMissing));
}

#[test]
fn stable_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_rows[0].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::StableLaneMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.lane_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .remote_preview_shows_expiry_and_attribution = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReviewCiPreviewMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_lane() {
    let summary = packet().render_markdown_summary();
    for lane in M5ReviewCiPreviewLane::ALL {
        assert!(
            summary.contains(lane.as_str()),
            "summary missing lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_review_ci_preview_matrix_export()
        .expect("checked M5 review/ci/preview matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/merge_queue_status_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/remote_preview_held.json"
        )),
    ] {
        let packet: M5ReviewCiPreviewMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
