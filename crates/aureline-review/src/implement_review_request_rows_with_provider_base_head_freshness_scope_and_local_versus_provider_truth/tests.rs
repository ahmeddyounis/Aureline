use super::*;

const PACKET_ID: &str = "review-request-row:stable:0001";

fn rows() -> Vec<ReviewRequestRow> {
    vec![
        ReviewRequestRow {
            row_id: "row:pr-4821".to_owned(),
            backing_kind: ReviewRequestBackingKind::ProviderBackedRequest,
            provider_identity_label: "provider org/repo pull request #4821".to_owned(),
            object_id_label: "PR #4821".to_owned(),
            base_ref_label: "main".to_owned(),
            head_ref_label: "feature/login".to_owned(),
            base_head_freshness: BaseHeadFreshness::Current,
            stack_relation: ReviewRequestStackRelation::Standalone,
            scope: ReviewRequestScope::FullRequest,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            claims_provider_backed: true,
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            actions: vec![
                ReviewRequestAction::OpenInWorkspace,
                ReviewRequestAction::OpenProviderInBrowser,
                ReviewRequestAction::ExportReviewPacket,
            ],
            row_fields_shown: vec![
                "backing_kind".to_owned(),
                "provider_identity".to_owned(),
                "base_head".to_owned(),
                "provider_freshness".to_owned(),
            ],
            source_contract_refs: vec![
                REVIEW_REQUEST_ROW_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
                REVIEW_REQUEST_ROW_MERGE_QUEUE_CONTRACT_REF.to_owned(),
            ],
        },
        ReviewRequestRow {
            row_id: "row:local-estimate".to_owned(),
            backing_kind: ReviewRequestBackingKind::LocalReviewEstimate,
            provider_identity_label: "local workspace bundle".to_owned(),
            object_id_label: "local:feature/report".to_owned(),
            base_ref_label: "main".to_owned(),
            head_ref_label: "feature/report".to_owned(),
            base_head_freshness: BaseHeadFreshness::Current,
            stack_relation: ReviewRequestStackRelation::Standalone,
            scope: ReviewRequestScope::FullRequest,
            provider_freshness: M5ReviewComponentStaleProviderState::LocalOnlyContinuation,
            claims_provider_backed: false,
            browser_handoff_boundary: String::new(),
            local_continue_fallback: "Continue reviewing locally; no hosted request exists yet"
                .to_owned(),
            actions: vec![
                ReviewRequestAction::OpenInWorkspace,
                ReviewRequestAction::ContinueLocalReview,
                ReviewRequestAction::ExportReviewPacket,
            ],
            row_fields_shown: vec![
                "backing_kind".to_owned(),
                "provider_identity".to_owned(),
                "base_head".to_owned(),
                "provider_freshness".to_owned(),
            ],
            source_contract_refs: vec![REVIEW_REQUEST_ROW_REVIEW_WORKSPACE_CONTRACT_REF.to_owned()],
        },
        ReviewRequestRow {
            row_id: "row:offline-packet".to_owned(),
            backing_kind: ReviewRequestBackingKind::OfflineExportedPacket,
            provider_identity_label: "exported review packet (cached)".to_owned(),
            object_id_label: "export:MR-317".to_owned(),
            base_ref_label: "release".to_owned(),
            head_ref_label: "hotfix/crash".to_owned(),
            base_head_freshness: BaseHeadFreshness::StaleBase,
            stack_relation: ReviewRequestStackRelation::Standalone,
            scope: ReviewRequestScope::FullRequest,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderStale,
            claims_provider_backed: false,
            browser_handoff_boundary: String::new(),
            local_continue_fallback: "Review the cached export; provider truth may have advanced"
                .to_owned(),
            actions: vec![
                ReviewRequestAction::OpenInWorkspace,
                ReviewRequestAction::ExportReviewPacket,
            ],
            row_fields_shown: vec![
                "backing_kind".to_owned(),
                "provider_identity".to_owned(),
                "base_head".to_owned(),
                "provider_freshness".to_owned(),
            ],
            source_contract_refs: vec![REVIEW_REQUEST_ROW_REVIEW_WORKSPACE_CONTRACT_REF.to_owned()],
        },
        ReviewRequestRow {
            row_id: "row:handoff-placeholder".to_owned(),
            backing_kind: ReviewRequestBackingKind::BrowserHandoffPlaceholder,
            provider_identity_label: "provider org/repo merge request (unreachable)".to_owned(),
            object_id_label: "MR #902".to_owned(),
            base_ref_label: "develop".to_owned(),
            head_ref_label: "feature/import".to_owned(),
            base_head_freshness: BaseHeadFreshness::Unknown,
            stack_relation: ReviewRequestStackRelation::StackMemberParentBlocked,
            scope: ReviewRequestScope::StackSegment,
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderUnreachable,
            claims_provider_backed: false,
            browser_handoff_boundary:
                "Provider is unreachable; open in browser to inspect hosted status".to_owned(),
            local_continue_fallback: "Keep the local review open while the provider is unreachable"
                .to_owned(),
            actions: vec![
                ReviewRequestAction::OpenProviderInBrowser,
                ReviewRequestAction::ContinueLocalReview,
            ],
            row_fields_shown: vec![
                "backing_kind".to_owned(),
                "provider_identity".to_owned(),
                "stack_relation".to_owned(),
                "provider_freshness".to_owned(),
            ],
            source_contract_refs: vec![
                REVIEW_REQUEST_ROW_CHANGE_LINEAGE_CONTRACT_REF.to_owned(),
                REVIEW_REQUEST_ROW_MERGE_QUEUE_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn trust_review() -> ReviewRequestRowTrustReview {
    ReviewRequestRowTrustReview {
        provider_local_estimate_distinct: true,
        local_estimate_never_claims_hosted: true,
        offline_exported_packet_distinct: true,
        provider_freshness_explicit: true,
        base_head_relation_explicit: true,
        stack_relation_explicit: true,
        browser_handoff_explicit: true,
        local_continue_preserved_on_degraded_freshness: true,
        no_forced_raw_provider_navigation_for_triage: true,
        one_row_contract_no_hidden_provider_meaning: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ReviewRequestRowConsumerProjection {
    ReviewRequestRowConsumerProjection {
        review_list_reuses_one_row_contract: true,
        inbox_reuses_one_row_contract: true,
        switcher_reuses_one_row_contract: true,
        companion_queue_reuses_one_row_contract: true,
        handoff_packet_reuses_one_row_contract: true,
        row_distinguishes_local_provider_offline: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
    }
}

fn proof_freshness() -> ReviewRequestRowProofFreshness {
    ReviewRequestRowProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ReviewRequestRowDowngradeTrigger> {
    vec![
        ReviewRequestRowDowngradeTrigger::ProofStale,
        ReviewRequestRowDowngradeTrigger::ProviderFreshnessStale,
        ReviewRequestRowDowngradeTrigger::StackParentBlocked,
        ReviewRequestRowDowngradeTrigger::BrowserHandoffUnavailable,
        ReviewRequestRowDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<ReviewRequestRowConsumerSurface> {
    vec![
        ReviewRequestRowConsumerSurface::ReviewList,
        ReviewRequestRowConsumerSurface::ReviewInbox,
        ReviewRequestRowConsumerSurface::CompanionQueue,
        ReviewRequestRowConsumerSurface::HandoffPacket,
        ReviewRequestRowConsumerSurface::CliHeadless,
        ReviewRequestRowConsumerSurface::SupportExport,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        REVIEW_REQUEST_ROW_SCHEMA_REF.to_owned(),
        REVIEW_REQUEST_ROW_DOC_REF.to_owned(),
        REVIEW_REQUEST_ROW_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        REVIEW_REQUEST_ROW_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        REVIEW_REQUEST_ROW_MERGE_QUEUE_CONTRACT_REF.to_owned(),
        REVIEW_REQUEST_ROW_CHANGE_LINEAGE_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> ReviewRequestRowPacket {
    ReviewRequestRowPacket::new(ReviewRequestRowPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Review-request rows: local-versus-provider truth".to_owned(),
        rows: rows(),
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
fn review_request_row_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn resolver_derives_hosted_status_from_backing_kind() {
    // A local estimate never asserts hosted status, even if provider freshness looks fresh.
    let local = resolve_review_request_row_disclosure(
        ReviewRequestBackingKind::LocalReviewEstimate,
        M5ReviewComponentStaleProviderState::ProviderFresh,
    );
    assert!(!local.asserts_hosted_status);
    assert!(local.needs_local_continue_fallback);

    // A provider-backed request asserts hosted status.
    let provider = resolve_review_request_row_disclosure(
        ReviewRequestBackingKind::ProviderBackedRequest,
        M5ReviewComponentStaleProviderState::ProviderFresh,
    );
    assert!(provider.asserts_hosted_status);
    assert!(!provider.needs_local_continue_fallback);

    // An unreachable provider forces both a handoff boundary and a local-continue path.
    let unreachable = resolve_review_request_row_disclosure(
        ReviewRequestBackingKind::ProviderBackedRequest,
        M5ReviewComponentStaleProviderState::ProviderUnreachable,
    );
    assert!(unreachable.needs_browser_handoff_boundary);
    assert!(unreachable.needs_local_continue_fallback);
}

#[test]
fn local_estimate_claiming_hosted_status_fails() {
    let mut packet = packet();
    packet.rows[1].claims_provider_backed = true;
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::HostedStatusMisrepresented));
}

#[test]
fn provider_row_falsely_dropping_hosted_status_fails() {
    let mut packet = packet();
    packet.rows[0].claims_provider_backed = false;
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::HostedStatusMisrepresented));
}

#[test]
fn missing_browser_handoff_boundary_fails() {
    let mut packet = packet();
    packet.rows[3].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::BrowserHandoffBoundaryMissing));
}

#[test]
fn missing_local_continue_fallback_fails() {
    let mut packet = packet();
    packet.rows[2].local_continue_fallback = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::LocalContinueFallbackMissing));
}

#[test]
fn forced_raw_provider_navigation_fails() {
    let mut packet = packet();
    packet.rows[0].actions = vec![ReviewRequestAction::OpenProviderInBrowser];
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::ForcedRawProviderNavigation));
}

#[test]
fn missing_backing_kind_coverage_fails() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.backing_kind != ReviewRequestBackingKind::OfflineExportedPacket);
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::BackingKindCoverageMissing));
}

#[test]
fn missing_rows_fails() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::RowsMissing));
}

#[test]
fn incomplete_row_fails() {
    let mut packet = packet();
    packet.rows[0].object_id_label = String::new();
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::RowIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.local_estimate_never_claims_hosted = false;
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .row_distinguishes_local_provider_offline = false;
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ReviewRequestRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Rows"));
    assert!(summary.contains("PR #4821"));
    assert!(summary.contains("local_review_estimate"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_review_request_row_export().expect("checked review-request row export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-request-rows/provider_stale_local_continue.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-review-request-rows/browser_handoff_placeholder.json"
        )),
    ] {
        let packet: ReviewRequestRowPacket =
            serde_json::from_str(raw).expect("fixture parses as review-request row packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
