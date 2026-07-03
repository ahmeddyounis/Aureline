use super::*;

const PACKET_ID: &str = "pipeline-viewer:stable:0001";

fn branch_relation(
    branch_ref: &str,
    change_object_ref: &str,
    relation_class: BranchChangeRelationClass,
    stale_base: bool,
) -> PipelineBranchChangeRelation {
    PipelineBranchChangeRelation {
        branch_ref: Some(branch_ref.to_owned()),
        commit_ref: None,
        change_object_ref: Some(change_object_ref.to_owned()),
        relation_class,
        stale_base,
    }
}

fn duration(milliseconds: u64, display_label: &str) -> PipelineRunDuration {
    PipelineRunDuration {
        state: PipelineRunDurationState::Exact,
        milliseconds: Some(milliseconds),
        display_label: display_label.to_owned(),
    }
}

fn artifact_summary(
    run_token: &str,
    artifact_count: u32,
    log_count: u32,
) -> PipelineArtifactSummary {
    PipelineArtifactSummary {
        artifact_count,
        log_count,
        unavailable_count: 0,
        artifact_browser_ref: format!("artifact-browser:{run_token}"),
    }
}

fn run_control_disclosure(
    rerun_available: bool,
    cancel_available: bool,
    authority_state: PipelineRunControlAuthorityState,
    disabled_reason: Option<&str>,
) -> PipelineRunControlDisclosure {
    PipelineRunControlDisclosure {
        rerun_available,
        cancel_available,
        authority_state,
        acting_identity_ref: "identity:reviewer:fixture".to_owned(),
        side_effect_review_required: rerun_available || cancel_available,
        disabled_reason: disabled_reason.map(str::to_owned),
    }
}

fn open_details(run_token: &str) -> PipelineOpenDetailsAction {
    PipelineOpenDetailsAction {
        action_ref: format!("action:open-details:{run_token}"),
        action_label: "Open details".to_owned(),
        details_target_ref: format!("pipeline-details:{run_token}"),
    }
}

fn provider_handoff(required: bool, target_ref: &str, reason: &str) -> PipelineProviderHandoffBar {
    PipelineProviderHandoffBar {
        provider_native_required: required,
        handoff_target_ref: target_ref.to_owned(),
        handoff_reason: reason.to_owned(),
    }
}

fn run_rows() -> Vec<PipelineRunRow> {
    vec![
        PipelineRunRow {
            run_id: "run:feature-login".to_owned(),
            provider_run_ref: "provider-run:github-actions:feature-login:1041".to_owned(),
            provider_label: PipelineProviderLabel::GithubActions,
            target_identity_label: "Local branch feature/login vs base main".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            pipeline_label: "rust_workspace_ci".to_owned(),
            workflow_or_job_name: "Rust workspace CI / test".to_owned(),
            branch_change_relation: branch_relation(
                "branch:feature-login",
                "change:review:feature-login",
                BranchChangeRelationClass::CurrentBase,
                false,
            ),
            run_status: PipelineRunStatus::Succeeded,
            duration: duration(382_000, "6m 22s"),
            freshness: RunFreshness::AuthoritativeLive,
            freshness_note: None,
            trigger_attribution_label: "Pushed by signed-in human account".to_owned(),
            trigger_actor_class: PipelineTriggerActorClass::HumanAccount,
            artifact_summary: artifact_summary("feature-login", 1, 1),
            run_control_authority: PipelineViewerRunControlAuthority::ReadOnlyNoControl,
            run_control_disclosure: run_control_disclosure(
                false,
                false,
                PipelineRunControlAuthorityState::ReadOnly,
                Some("review_surface_read_only"),
            ),
            open_details_action: open_details("feature-login"),
            provider_handoff: provider_handoff(
                false,
                "provider-run:github-actions:feature-login:1041",
                "Normalized details are complete in product",
            ),
            status_summary: "fmt, clippy, tests all green".to_owned(),
            attention_reasons: Vec::new(),
            source_contract_refs: vec![
                PIPELINE_VIEWER_PIPELINE_RUN_CONTRACT_REF.to_owned(),
                PIPELINE_VIEWER_RUN_CONTROL_CONTRACT_REF.to_owned(),
            ],
        },
        PipelineRunRow {
            run_id: "run:hotfix-crash".to_owned(),
            provider_run_ref: "provider-run:github-actions:hotfix-crash:1042".to_owned(),
            provider_label: PipelineProviderLabel::GithubActions,
            target_identity_label: "Local branch hotfix/crash vs base release".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            pipeline_label: "release_pipeline".to_owned(),
            workflow_or_job_name: "Release pipeline / integration".to_owned(),
            branch_change_relation: branch_relation(
                "branch:hotfix-crash",
                "change:review:hotfix-crash",
                BranchChangeRelationClass::StaleBase,
                true,
            ),
            run_status: PipelineRunStatus::Failed,
            duration: duration(1_124_000, "18m 44s"),
            freshness: RunFreshness::WarmCached,
            freshness_note: Some(
                "Base advanced after this run; rerun uses reapproval before provider mutation."
                    .to_owned(),
            ),
            trigger_attribution_label: "Rerun requested by signed-in human account".to_owned(),
            trigger_actor_class: PipelineTriggerActorClass::HumanAccount,
            artifact_summary: PipelineArtifactSummary {
                artifact_count: 2,
                log_count: 1,
                unavailable_count: 1,
                artifact_browser_ref: "artifact-browser:hotfix-crash".to_owned(),
            },
            run_control_authority: PipelineViewerRunControlAuthority::AttributableRerunAndCancel,
            run_control_disclosure: run_control_disclosure(
                true,
                true,
                PipelineRunControlAuthorityState::Allowed,
                None,
            ),
            open_details_action: open_details("hotfix-crash"),
            provider_handoff: provider_handoff(
                true,
                "provider-run:github-actions:hotfix-crash:1042",
                "Provider-native pane is available for full rerun attempt history",
            ),
            status_summary: "integration tests failed".to_owned(),
            attention_reasons: vec![
                "Integration test suite failed on the advanced base".to_owned(),
                "Rerun is attributable to the signed-in human account".to_owned(),
            ],
            source_contract_refs: vec![
                PIPELINE_VIEWER_PIPELINE_RUN_CONTRACT_REF.to_owned(),
                PIPELINE_VIEWER_ARTIFACT_CARD_CONTRACT_REF.to_owned(),
                PIPELINE_VIEWER_RUN_CONTROL_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn log_views() -> Vec<LogViewerRow> {
    vec![
        LogViewerRow {
            run_id: "run:feature-login".to_owned(),
            view_id: "log:feature-login:build".to_owned(),
            log_label: "Build and test log".to_owned(),
            stream_state: LogStreamState::CompletedReplay,
            safe_preview_trust_class: SafePreviewTrustClass::RawText,
            safe_open_path: SafeOpenPath::OpenInSafePreviewSanitized,
            truncation_label: String::new(),
        },
        LogViewerRow {
            run_id: "run:hotfix-crash".to_owned(),
            view_id: "log:hotfix-crash:integration".to_owned(),
            log_label: "Integration test log".to_owned(),
            stream_state: LogStreamState::PartialRetained,
            safe_preview_trust_class: SafePreviewTrustClass::RawText,
            safe_open_path: SafeOpenPath::OpenInSafePreviewMetadataOnly,
            truncation_label: "Only the last 5 MB of the log is retained".to_owned(),
        },
    ]
}

fn artifact_cards() -> Vec<ArtifactBrowserRow> {
    vec![
        ArtifactBrowserRow {
            run_id: "run:feature-login".to_owned(),
            artifact_id: "artifact:feature-login:coverage".to_owned(),
            artifact_label: "Coverage report".to_owned(),
            artifact_kind: ArtifactKind::CoverageReport,
            safe_preview_trust_class: SafePreviewTrustClass::SanitizedRich,
            safe_open_path: SafeOpenPath::OpenInStructuredViewer,
            freshness: RunFreshness::AuthoritativeLive,
            size_disclosure_label: "1.2 MB".to_owned(),
            retention_label: "Retained for 90 days".to_owned(),
        },
        ArtifactBrowserRow {
            run_id: "run:hotfix-crash".to_owned(),
            artifact_id: "artifact:hotfix-crash:binary".to_owned(),
            artifact_label: "Release binary".to_owned(),
            artifact_kind: ArtifactKind::BinaryExecutable,
            safe_preview_trust_class: SafePreviewTrustClass::RawText,
            safe_open_path: SafeOpenPath::DownloadOnlyNoInProductOpen,
            freshness: RunFreshness::Stale,
            size_disclosure_label: "48 MB".to_owned(),
            retention_label: "Retention window ends in 3 days".to_owned(),
        },
    ]
}

fn trust_review() -> PipelineViewerTrustReview {
    PipelineViewerTrustReview {
        run_status_explicit: true,
        run_status_never_overstated: true,
        safe_preview_trust_class_explicit: true,
        active_content_never_trusted_local: true,
        log_truncation_labeled_not_hidden: true,
        artifact_retention_labeled_not_hidden: true,
        freshness_explicit_and_narrows_open_path: true,
        rerun_cancel_authority_explicit_and_attributable: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> PipelineViewerConsumerProjection {
    PipelineViewerConsumerProjection {
        pipeline_viewer_shows_run_status: true,
        pipeline_viewer_shows_freshness: true,
        runs_panel_shows_attention_reasons: true,
        log_pane_shows_safe_preview_trust_class: true,
        log_pane_shows_truncation: true,
        artifact_browser_shows_safe_open_path: true,
        artifact_browser_shows_retention: true,
        run_control_shows_authority_and_attribution: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> PipelineViewerProofFreshness {
    PipelineViewerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<PipelineViewerDowngradeTrigger> {
    vec![
        PipelineViewerDowngradeTrigger::ProofStale,
        PipelineViewerDowngradeTrigger::RunStatusUnverified,
        PipelineViewerDowngradeTrigger::LogTruncationUnlabeled,
        PipelineViewerDowngradeTrigger::ArtifactRetentionExpired,
        PipelineViewerDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<PipelineViewerConsumerSurface> {
    vec![
        PipelineViewerConsumerSurface::PipelineViewer,
        PipelineViewerConsumerSurface::LogPane,
        PipelineViewerConsumerSurface::ArtifactBrowser,
        PipelineViewerConsumerSurface::CliHeadless,
        PipelineViewerConsumerSurface::SupportExport,
        PipelineViewerConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PIPELINE_VIEWER_SCHEMA_REF.to_owned(),
        PIPELINE_VIEWER_DOC_REF.to_owned(),
        PIPELINE_VIEWER_PIPELINE_RUN_CONTRACT_REF.to_owned(),
        PIPELINE_VIEWER_LOG_VIEW_CONTRACT_REF.to_owned(),
        PIPELINE_VIEWER_ARTIFACT_CARD_CONTRACT_REF.to_owned(),
        PIPELINE_VIEWER_RUN_CONTROL_CONTRACT_REF.to_owned(),
        PIPELINE_VIEWER_TRUST_CLASS_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> PipelineViewerPacket {
    PipelineViewerPacket::new(PipelineViewerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Normalized pipeline run rows, log viewers, and artifact browsers"
            .to_owned(),
        run_rows: run_rows(),
        log_views: log_views(),
        artifact_cards: artifact_cards(),
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
fn pipeline_viewer_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_run_rows_fails() {
    let mut packet = packet();
    packet.run_rows.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::RunRowsMissing));
}

#[test]
fn non_green_run_without_attention_reason_fails() {
    let mut packet = packet();
    packet.run_rows[1].attention_reasons.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::AttentionReasonMissing));
}

#[test]
fn missing_provider_identity_fails() {
    let mut packet = packet();
    packet.run_rows[0].provider_run_ref.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::ProviderRunIdentityMissing));
}

#[test]
fn missing_branch_or_commit_relation_fails() {
    let mut packet = packet();
    packet.run_rows[0].branch_change_relation.branch_ref = None;
    packet.run_rows[0].branch_change_relation.commit_ref = None;
    packet.run_rows[0].branch_change_relation.change_object_ref = None;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::BranchChangeRelationMissing));
}

#[test]
fn missing_duration_disclosure_fails() {
    let mut packet = packet();
    packet.run_rows[0].duration.display_label.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::DurationDisclosureMissing));
}

#[test]
fn missing_artifact_count_disclosure_fails() {
    let mut packet = packet();
    packet.run_rows[0]
        .artifact_summary
        .artifact_browser_ref
        .clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::ArtifactCountDisclosureMissing));
}

#[test]
fn missing_open_details_action_fails() {
    let mut packet = packet();
    packet.run_rows[0].open_details_action.action_ref.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::OpenDetailsActionMissing));
}

#[test]
fn run_control_disclosure_mismatch_fails() {
    let mut packet = packet();
    packet.run_rows[1].run_control_disclosure.cancel_available = false;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::RunControlDisclosureMismatch));
}

#[test]
fn disabled_control_without_reason_fails() {
    let mut packet = packet();
    packet.run_rows[0].run_control_disclosure.disabled_reason = None;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::RunControlDisabledReasonMissing));
}

#[test]
fn stale_or_superseded_row_without_note_fails() {
    let mut packet = packet();
    packet.run_rows[1].freshness_note = None;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::StaleOrSupersededNoteMissing));
}

#[test]
fn provider_handoff_without_reason_fails() {
    let mut packet = packet();
    packet.run_rows[1].provider_handoff.handoff_reason.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::ProviderHandoffDisclosureMissing));
}

#[test]
fn run_without_log_view_fails() {
    let mut packet = packet();
    packet
        .log_views
        .retain(|row| row.run_id != "run:feature-login");
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::RunMissingLogView));
}

#[test]
fn orphan_artifact_reference_fails() {
    let mut packet = packet();
    packet.artifact_cards[0].run_id = "run:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::OrphanRowReference));
}

#[test]
fn partial_log_without_truncation_label_fails() {
    let mut packet = packet();
    packet.log_views[1].truncation_label = String::new();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::TruncationLabelMissing));
}

#[test]
fn trusted_local_active_on_provider_boundary_fails() {
    let mut packet = packet();
    packet.log_views[0].safe_preview_trust_class = SafePreviewTrustClass::TrustedLocalActive;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::ActiveContentTrustInadmissible));
}

#[test]
fn stale_artifact_with_live_open_path_fails() {
    let mut packet = packet();
    packet.artifact_cards[1].safe_open_path = SafeOpenPath::OpenInStructuredViewer;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::SafeOpenPathOverstatesFreshness));
}

#[test]
fn download_only_binary_opening_in_product_fails() {
    let mut packet = packet();
    packet.artifact_cards[1].freshness = RunFreshness::AuthoritativeLive;
    packet.artifact_cards[1].safe_open_path = SafeOpenPath::OpenInSafePreviewSanitized;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::DownloadOnlyArtifactOpensInProduct));
}

#[test]
fn partial_log_with_live_open_path_fails() {
    let mut packet = packet();
    packet.log_views[1].safe_open_path = SafeOpenPath::OpenInStructuredViewer;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::SafeOpenPathOverstatesFreshness));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .log_pane_shows_safe_preview_trust_class = false;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&PipelineViewerViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Run rows"));
    assert!(summary.contains("## Log viewers"));
    assert!(summary.contains("## Artifact browsers"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_pipeline_viewer_export().expect("checked pipeline viewer export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/log_retention_expired_offline.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/run_unknown_status_provider_owned.json"
        )),
    ] {
        let packet: PipelineViewerPacket =
            serde_json::from_str(raw).expect("fixture parses as pipeline viewer packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
