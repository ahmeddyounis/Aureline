use super::*;

const PACKET_ID: &str = "checks-summary-card:stable:0001";

fn cards() -> Vec<ChecksSummaryCard> {
    vec![
        ChecksSummaryCard {
            card_id: "card:pr-4821".to_owned(),
            review_id_label: "PR #4821".to_owned(),
            provider_identity_label: "provider org/repo pull request #4821".to_owned(),
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderFresh,
            presents_single_verdict: false,
            headline_verdict_label: "3 required passing, 1 optional, 2 not run".to_owned(),
            checks: vec![
                CheckEntry {
                    check_id: "check:unit".to_owned(),
                    check_name_label: "unit-tests".to_owned(),
                    check_class: CheckClass::Required,
                    evaluation_reason: String::new(),
                    evidence_links: vec![
                        CheckEvidenceLink {
                            kind: CheckEvidenceKind::Log,
                            label: "unit-tests log".to_owned(),
                            review_id_ref: "PR #4821".to_owned(),
                            check_id_ref: "check:unit".to_owned(),
                        },
                        CheckEvidenceLink {
                            kind: CheckEvidenceKind::Artifact,
                            label: "coverage report".to_owned(),
                            review_id_ref: "PR #4821".to_owned(),
                            check_id_ref: "check:unit".to_owned(),
                        },
                        CheckEvidenceLink {
                            kind: CheckEvidenceKind::Annotation,
                            label: "failing assertion annotation".to_owned(),
                            review_id_ref: "PR #4821".to_owned(),
                            check_id_ref: "check:unit".to_owned(),
                        },
                    ],
                    actions: vec![
                        ChecksSummaryAction::OpenLog,
                        ChecksSummaryAction::OpenArtifact,
                        ChecksSummaryAction::OpenAnnotation,
                        ChecksSummaryAction::RerunCheck,
                    ],
                },
                CheckEntry {
                    check_id: "check:lint".to_owned(),
                    check_name_label: "lint".to_owned(),
                    check_class: CheckClass::Optional,
                    evaluation_reason: String::new(),
                    evidence_links: vec![CheckEvidenceLink {
                        kind: CheckEvidenceKind::Log,
                        label: "lint log".to_owned(),
                        review_id_ref: "PR #4821".to_owned(),
                        check_id_ref: "check:lint".to_owned(),
                    }],
                    actions: vec![ChecksSummaryAction::OpenLog],
                },
                CheckEntry {
                    check_id: "check:e2e".to_owned(),
                    check_name_label: "e2e".to_owned(),
                    check_class: CheckClass::Skipped,
                    evaluation_reason: "Skipped: no UI changes in this diff".to_owned(),
                    evidence_links: vec![],
                    actions: vec![ChecksSummaryAction::RerunCheck],
                },
                CheckEntry {
                    check_id: "check:license".to_owned(),
                    check_name_label: "license-scan".to_owned(),
                    check_class: CheckClass::Suppressed,
                    evaluation_reason: "Suppressed by repo policy for dependency-only changes"
                        .to_owned(),
                    evidence_links: vec![],
                    actions: vec![ChecksSummaryAction::ExportCheckEvidence],
                },
            ],
            browser_handoff_boundary: String::new(),
            local_continue_fallback: String::new(),
            source_contract_refs: vec![
                CHECKS_SUMMARY_CARD_PIPELINE_RUN_CONTRACT_REF.to_owned(),
                CHECKS_SUMMARY_CARD_LOG_VIEW_CONTRACT_REF.to_owned(),
            ],
        },
        ChecksSummaryCard {
            card_id: "card:mr-902".to_owned(),
            review_id_label: "MR #902".to_owned(),
            provider_identity_label: "provider org/repo merge request #902 (unreachable)"
                .to_owned(),
            provider_freshness: M5ReviewComponentStaleProviderState::ProviderUnreachable,
            presents_single_verdict: false,
            headline_verdict_label: "Provider unreachable; last-known check states below"
                .to_owned(),
            checks: vec![
                CheckEntry {
                    check_id: "check:integration".to_owned(),
                    check_name_label: "integration".to_owned(),
                    check_class: CheckClass::TimedOut,
                    evaluation_reason: "Timed out after 30m without a verdict".to_owned(),
                    evidence_links: vec![CheckEvidenceLink {
                        kind: CheckEvidenceKind::Log,
                        label: "integration log (partial)".to_owned(),
                        review_id_ref: "MR #902".to_owned(),
                        check_id_ref: "check:integration".to_owned(),
                    }],
                    actions: vec![
                        ChecksSummaryAction::RerunCheck,
                        ChecksSummaryAction::ContinueLocalReview,
                    ],
                },
                CheckEntry {
                    check_id: "check:security".to_owned(),
                    check_name_label: "security-scan".to_owned(),
                    check_class: CheckClass::Stale,
                    evaluation_reason: "Provider result is stale relative to the current head"
                        .to_owned(),
                    evidence_links: vec![],
                    actions: vec![ChecksSummaryAction::ContinueLocalReview],
                },
                CheckEntry {
                    check_id: "check:deploy-preview".to_owned(),
                    check_name_label: "deploy-preview".to_owned(),
                    check_class: CheckClass::NotEvaluatedHere,
                    evaluation_reason: "Not evaluated in this offline context".to_owned(),
                    evidence_links: vec![],
                    actions: vec![
                        ChecksSummaryAction::ContinueLocalReview,
                        ChecksSummaryAction::OpenProviderInBrowser,
                    ],
                },
            ],
            browser_handoff_boundary:
                "Provider is unreachable; open in browser to inspect hosted check status".to_owned(),
            local_continue_fallback:
                "Keep reviewing from the local diff and cached annotations while the provider is unreachable"
                    .to_owned(),
            source_contract_refs: vec![
                CHECKS_SUMMARY_CARD_ANNOTATION_CONTRACT_REF.to_owned(),
                CHECKS_SUMMARY_CARD_ARTIFACT_CARD_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn trust_review() -> ChecksSummaryCardTrustReview {
    ChecksSummaryCardTrustReview {
        required_optional_distinct: true,
        anomalous_check_states_distinct: true,
        checks_never_flattened_when_richer_evidence: true,
        log_artifact_annotation_identity_preserved: true,
        provider_outage_preserves_local_continuation: true,
        stale_sync_never_collapses_review_lane: true,
        rerun_cancel_only_where_allowed: true,
        no_forced_raw_provider_navigation_for_triage: true,
        not_evaluated_or_stale_never_shown_as_pass: true,
        one_card_contract_no_hidden_provider_meaning: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> ChecksSummaryCardConsumerProjection {
    ChecksSummaryCardConsumerProjection {
        review_workspace_reuses_one_card_contract: true,
        review_list_reuses_one_card_contract: true,
        companion_queue_reuses_one_card_contract: true,
        card_distinguishes_all_check_classes: true,
        evidence_links_preserve_review_and_check_identity: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        export_preserves_check_identity: true,
    }
}

fn proof_freshness() -> ChecksSummaryCardProofFreshness {
    ChecksSummaryCardProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<ChecksSummaryCardDowngradeTrigger> {
    vec![
        ChecksSummaryCardDowngradeTrigger::ProofStale,
        ChecksSummaryCardDowngradeTrigger::ProviderFreshnessStale,
        ChecksSummaryCardDowngradeTrigger::CheckTimedOut,
        ChecksSummaryCardDowngradeTrigger::BrowserHandoffUnavailable,
        ChecksSummaryCardDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<ChecksSummaryCardConsumerSurface> {
    vec![
        ChecksSummaryCardConsumerSurface::ReviewWorkspace,
        ChecksSummaryCardConsumerSurface::ReviewList,
        ChecksSummaryCardConsumerSurface::CompanionQueue,
        ChecksSummaryCardConsumerSurface::CliHeadless,
        ChecksSummaryCardConsumerSurface::SupportExport,
        ChecksSummaryCardConsumerSurface::ChecksSummaryDrawer,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        CHECKS_SUMMARY_CARD_SCHEMA_REF.to_owned(),
        CHECKS_SUMMARY_CARD_DOC_REF.to_owned(),
        CHECKS_SUMMARY_CARD_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        CHECKS_SUMMARY_CARD_REVIEW_WORKSPACE_CONTRACT_REF.to_owned(),
        CHECKS_SUMMARY_CARD_PIPELINE_RUN_CONTRACT_REF.to_owned(),
        CHECKS_SUMMARY_CARD_LOG_VIEW_CONTRACT_REF.to_owned(),
        CHECKS_SUMMARY_CARD_ARTIFACT_CARD_CONTRACT_REF.to_owned(),
        CHECKS_SUMMARY_CARD_ANNOTATION_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> ChecksSummaryCardPacket {
    ChecksSummaryCardPacket::new(ChecksSummaryCardPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Checks-summary cards: check-class and evidence continuity".to_owned(),
        cards: cards(),
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
fn checks_summary_card_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn resolver_forbids_flattening_when_richer_evidence() {
    // A card with richer evidence must not flatten to a single verdict.
    let richer = resolve_checks_summary_card_disclosure(
        M5ReviewComponentStaleProviderState::ProviderFresh,
        true,
    );
    assert!(richer.must_not_flatten_to_single_verdict);
    assert!(!richer.needs_local_continue_fallback);

    // A trivial card with no richer evidence may present a single verdict.
    let trivial = resolve_checks_summary_card_disclosure(
        M5ReviewComponentStaleProviderState::ProviderFresh,
        false,
    );
    assert!(!trivial.must_not_flatten_to_single_verdict);

    // An unreachable provider forces both a handoff boundary and a local-continue path.
    let unreachable = resolve_checks_summary_card_disclosure(
        M5ReviewComponentStaleProviderState::ProviderUnreachable,
        true,
    );
    assert!(unreachable.needs_browser_handoff_boundary);
    assert!(unreachable.needs_local_continue_fallback);
}

#[test]
fn flattening_card_with_richer_evidence_fails() {
    let mut packet = packet();
    packet.cards[0].presents_single_verdict = true;
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::ChecksFlattenedToSingleVerdict));
}

#[test]
fn missing_evaluation_reason_fails() {
    let mut packet = packet();
    // The Skipped check on card 0 must carry an explicit reason.
    packet.cards[0].checks[2].evaluation_reason = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::CheckEvaluationReasonMissing));
}

#[test]
fn evidence_link_dropping_identity_fails() {
    let mut packet = packet();
    packet.cards[0].checks[0].evidence_links[0].check_id_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::EvidenceIdentityNotPreserved));
}

#[test]
fn evidence_link_dropping_review_identity_fails() {
    let mut packet = packet();
    packet.cards[0].checks[0].evidence_links[0].review_id_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::EvidenceIdentityNotPreserved));
}

#[test]
fn missing_browser_handoff_boundary_fails() {
    let mut packet = packet();
    packet.cards[1].browser_handoff_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::BrowserHandoffBoundaryMissing));
}

#[test]
fn missing_local_continue_fallback_fails() {
    let mut packet = packet();
    packet.cards[1].local_continue_fallback = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::LocalContinueFallbackMissing));
}

#[test]
fn forced_raw_provider_navigation_fails() {
    let mut packet = packet();
    // Strip every in-product action from card 1's checks.
    for check in &mut packet.cards[1].checks {
        check.actions = vec![ChecksSummaryAction::OpenProviderInBrowser];
    }
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::ForcedRawProviderNavigation));
}

#[test]
fn missing_check_class_coverage_fails() {
    let mut packet = packet();
    // Drop the only NotEvaluatedHere check so coverage is incomplete.
    packet.cards[1]
        .checks
        .retain(|check| check.check_class != CheckClass::NotEvaluatedHere);
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::CheckClassCoverageMissing));
}

#[test]
fn missing_cards_fails() {
    let mut packet = packet();
    packet.cards.clear();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::CardsMissing));
}

#[test]
fn incomplete_card_fails() {
    let mut packet = packet();
    packet.cards[0].headline_verdict_label = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::CardIncomplete));
}

#[test]
fn incomplete_check_fails() {
    let mut packet = packet();
    packet.cards[0].checks[0].check_name_label = String::new();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::CheckIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .checks_never_flattened_when_richer_evidence = false;
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .evidence_links_preserve_review_and_check_identity = false;
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ChecksSummaryCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_cards() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Cards"));
    assert!(summary.contains("PR #4821"));
    assert!(summary.contains("not_evaluated_here"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_checks_summary_card_export().expect("checked checks-summary card export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-checks-summary-cards/provider_stale_local_continue.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-checks-summary-cards/timed_out_and_not_evaluated.json"
        )),
    ] {
        let packet: ChecksSummaryCardPacket =
            serde_json::from_str(raw).expect("fixture parses as checks-summary card packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
