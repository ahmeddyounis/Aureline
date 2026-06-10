use super::*;

const PACKET_ID: &str = "durable-review-header:stable:0001";

fn headers() -> Vec<DurableReviewHeaderRow> {
    vec![
        DurableReviewHeaderRow {
            header_id: "header:feature-branch".to_owned(),
            target_identity_label: "Local branch feature/login vs base main".to_owned(),
            durable_anchor_id: "anchor:review:0001".to_owned(),
            base_freshness: HeaderFreshnessClass::Current,
            approval_state: ReviewApprovalState::Approved,
            mutation_authority: MutationAuthorityClass::ReadOnlyNoMutation,
            header_fields_shown: vec![
                "target_identity".to_owned(),
                "durable_anchor_id".to_owned(),
                "base_freshness".to_owned(),
                "approval_state".to_owned(),
            ],
            source_contract_refs: vec![
                DURABLE_REVIEW_HEADER_REVIEW_WORKSPACE_CONTRACT_REF.to_owned()
            ],
        },
        DurableReviewHeaderRow {
            header_id: "header:hotfix".to_owned(),
            target_identity_label: "Local branch hotfix/crash vs base release".to_owned(),
            durable_anchor_id: "anchor:review:0002".to_owned(),
            base_freshness: HeaderFreshnessClass::StaleBase,
            approval_state: ReviewApprovalState::ResetOnBaseChange,
            mutation_authority: MutationAuthorityClass::ReadOnlyNoMutation,
            header_fields_shown: vec![
                "target_identity".to_owned(),
                "durable_anchor_id".to_owned(),
                "base_freshness".to_owned(),
                "approval_state".to_owned(),
            ],
            source_contract_refs: vec![
                DURABLE_REVIEW_HEADER_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
                DURABLE_REVIEW_HEADER_ANCHOR_STABILITY_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn parity_lanes() -> Vec<LocalCiParityLaneRow> {
    vec![
        LocalCiParityLaneRow {
            check_id: "check:fmt".to_owned(),
            enforcement: CheckEnforcementClass::Required,
            verdict: LocalCiParityVerdict::ParityMatch,
            local_result_label: "rustfmt clean".to_owned(),
            ci_expectation_label: "rustfmt clean".to_owned(),
            divergence_label: String::new(),
        },
        LocalCiParityLaneRow {
            check_id: "check:clippy".to_owned(),
            enforcement: CheckEnforcementClass::Required,
            verdict: LocalCiParityVerdict::DivergenceLabeled,
            local_result_label: "clippy clean (local toolchain)".to_owned(),
            ci_expectation_label: "clippy denies one pedantic lint".to_owned(),
            divergence_label: "Local toolchain lints differ from the CI lint set".to_owned(),
        },
        LocalCiParityLaneRow {
            check_id: "check:integration".to_owned(),
            enforcement: CheckEnforcementClass::Advisory,
            verdict: LocalCiParityVerdict::LocalOnlyAdvisory,
            local_result_label: "integration suite passing locally".to_owned(),
            ci_expectation_label: "not yet reported".to_owned(),
            divergence_label: String::new(),
        },
    ]
}

fn rehydration_events() -> Vec<AnchorRehydrationRow> {
    vec![
        AnchorRehydrationRow {
            anchor_id: "anchor:review:0001".to_owned(),
            trigger: RehydrationTrigger::Rebase,
            resulting_state: AnchorRehydrationState::RehydratedShifted,
            drift_label: String::new(),
        },
        AnchorRehydrationRow {
            anchor_id: "anchor:review:0002".to_owned(),
            trigger: RehydrationTrigger::BaseChange,
            resulting_state: AnchorRehydrationState::DriftedRelabeled,
            drift_label: "Anchored line moved out of the changed range; relabeled as drifted"
                .to_owned(),
        },
    ]
}

fn trust_review() -> DurableReviewHeaderTrustReview {
    DurableReviewHeaderTrustReview {
        anchors_rehydrate_durably: true,
        anchor_drift_labeled_not_hidden: true,
        stale_base_labeled_explicit: true,
        approval_resets_on_base_change: true,
        local_ci_parity_explicit: true,
        divergence_labeled_not_hidden: true,
        target_identity_explicit: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> DurableReviewHeaderConsumerProjection {
    DurableReviewHeaderConsumerProjection {
        header_shows_anchor_and_target_identity: true,
        header_shows_base_freshness: true,
        header_shows_approval_state: true,
        parity_shows_local_vs_ci_verdict: true,
        rehydration_shows_drift_label: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> DurableReviewHeaderProofFreshness {
    DurableReviewHeaderProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<DurableReviewHeaderDowngradeTrigger> {
    vec![
        DurableReviewHeaderDowngradeTrigger::ProofStale,
        DurableReviewHeaderDowngradeTrigger::AnchorDrift,
        DurableReviewHeaderDowngradeTrigger::ParityDivergenceUnlabeled,
        DurableReviewHeaderDowngradeTrigger::StaleBaseUnlabeled,
        DurableReviewHeaderDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<DurableReviewHeaderConsumerSurface> {
    vec![
        DurableReviewHeaderConsumerSurface::ReviewWorkspaceHeader,
        DurableReviewHeaderConsumerSurface::CliHeadless,
        DurableReviewHeaderConsumerSurface::SupportExport,
        DurableReviewHeaderConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        DURABLE_REVIEW_HEADER_SCHEMA_REF.to_owned(),
        DURABLE_REVIEW_HEADER_DOC_REF.to_owned(),
        DURABLE_REVIEW_HEADER_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        DURABLE_REVIEW_HEADER_REVIEW_PACK_CONTRACT_REF.to_owned(),
        DURABLE_REVIEW_HEADER_ANCHOR_STABILITY_CONTRACT_REF.to_owned(),
        DURABLE_REVIEW_HEADER_PIPELINE_RUN_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> DurableReviewHeaderPacket {
    DurableReviewHeaderPacket::new(DurableReviewHeaderPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Durable review header, local-CI parity, and anchor rehydration".to_owned(),
        headers: headers(),
        parity_lanes: parity_lanes(),
        rehydration_events: rehydration_events(),
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
fn durable_review_header_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_headers_fails() {
    let mut packet = packet();
    packet.headers.clear();
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::HeadersMissing));
}

#[test]
fn header_anchor_without_rehydration_fails() {
    let mut packet = packet();
    packet
        .rehydration_events
        .retain(|row| row.anchor_id != "anchor:review:0001");
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::HeaderAnchorMissingRehydration));
}

#[test]
fn labeled_divergence_without_label_fails() {
    let mut packet = packet();
    packet.parity_lanes[1].divergence_label = String::new();
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::DivergenceLabelMissing));
}

#[test]
fn missing_required_parity_lane_fails() {
    let mut packet = packet();
    for lane in &mut packet.parity_lanes {
        lane.enforcement = CheckEnforcementClass::Advisory;
    }
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::RequiredParityLaneMissing));
}

#[test]
fn drifted_anchor_without_label_fails() {
    let mut packet = packet();
    packet.rehydration_events[1].drift_label = String::new();
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::DriftLabelMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.rehydration_shows_drift_label = false;
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&DurableReviewHeaderViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Headers"));
    assert!(summary.contains("## Local-CI parity"));
    assert!(summary.contains("## Anchor rehydration"));
    assert!(summary.contains("anchor:review:0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_durable_review_header_export()
        .expect("checked durable review-header export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/stale_base_anchor_drift.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/ci_parity_offline.json"
        )),
    ] {
        let packet: DurableReviewHeaderPacket =
            serde_json::from_str(raw).expect("fixture parses as durable review-header packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
