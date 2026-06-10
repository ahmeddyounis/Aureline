use super::*;

const PACKET_ID: &str = "ai-review-evidence-cards:stable:0001";

fn evidence_rows() -> Vec<EvidenceCardEvidenceRow> {
    vec![
        EvidenceCardEvidenceRow {
            evidence_id: "evidence:model:0001".to_owned(),
            source: EvidenceSourceClass::ModelGenerated,
            freshness: EvidenceFreshnessClass::Fresh,
            provenance_label: "Review model run over the diff for change object cobj-0001"
                .to_owned(),
            summary_label: "Flags an unchecked unwrap in the new parser path".to_owned(),
            source_contract_refs: vec![EVIDENCE_CARD_AI_EVIDENCE_CONTRACT_REF.to_owned()],
        },
        EvidenceCardEvidenceRow {
            evidence_id: "evidence:static:0002".to_owned(),
            source: EvidenceSourceClass::StaticAnalysis,
            freshness: EvidenceFreshnessClass::StaleDiff,
            provenance_label: "Static analysis pass; diff advanced after the run".to_owned(),
            summary_label: "Unused import in the changed module".to_owned(),
            source_contract_refs: vec![
                EVIDENCE_CARD_AI_EVIDENCE_CONTRACT_REF.to_owned(),
                EVIDENCE_CARD_REVIEW_PACK_CONTRACT_REF.to_owned(),
            ],
        },
    ]
}

fn finding_cards() -> Vec<EvidenceCardFindingRow> {
    vec![
        EvidenceCardFindingRow {
            finding_id: "finding:0001".to_owned(),
            severity: FindingSeverityClass::Blocking,
            status: FindingStatusClass::Open,
            anchor_id: "anchor:review:0001".to_owned(),
            evidence_ref: "evidence:model:0001".to_owned(),
            apply_posture: FindingApplyPosture::SafeApplyPreviewed,
            change_object_ref: "cobj-0001".to_owned(),
            title_label: "Unchecked unwrap can panic on malformed input".to_owned(),
            apply_block_label: String::new(),
        },
        EvidenceCardFindingRow {
            finding_id: "finding:0002".to_owned(),
            severity: FindingSeverityClass::Low,
            status: FindingStatusClass::Acknowledged,
            anchor_id: "anchor:review:0002".to_owned(),
            evidence_ref: "evidence:static:0002".to_owned(),
            apply_posture: FindingApplyPosture::ApplyBlocked,
            change_object_ref: "cobj-0001".to_owned(),
            title_label: "Unused import".to_owned(),
            apply_block_label: "Apply blocked: evidence is stale against the current diff"
                .to_owned(),
        },
    ]
}

fn review_pack_bindings() -> Vec<EvidenceCardReviewPackBindingRow> {
    vec![EvidenceCardReviewPackBindingRow {
        review_pack_id: "pack:default".to_owned(),
        change_object_id: "cobj-0001".to_owned(),
        binding_state: ReviewPackBindingState::BoundCurrent,
        required_check_coverage: ReviewPackCheckCoverage::AllRequiredPresent,
        attribution_label: "Bound by local review session for feature/parser".to_owned(),
        detach_label: String::new(),
    }]
}

fn trust_review() -> EvidenceCardTrustReview {
    EvidenceCardTrustReview {
        ai_evidence_provenance_explicit: true,
        evidence_freshness_labeled_not_hidden: true,
        finding_cards_cite_real_evidence: true,
        finding_severity_and_status_explicit: true,
        apply_posture_explicit_no_silent_write: true,
        review_pack_binds_to_change_object: true,
        binding_detach_labeled_not_hidden: true,
        change_object_attribution_explicit: true,
        no_hidden_write_scope: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> EvidenceCardConsumerProjection {
    EvidenceCardConsumerProjection {
        finding_cards_show_severity_status: true,
        finding_cards_show_evidence_provenance: true,
        finding_cards_show_apply_posture: true,
        review_pack_shows_change_object_binding: true,
        binding_shows_detach_label: true,
        cli_headless_shows_truth: true,
        support_export_shows_truth: true,
        diagnostics_shows_truth: true,
        help_about_shows_truth: true,
        preview_labs_label_for_unqualified: true,
    }
}

fn proof_freshness() -> EvidenceCardProofFreshness {
    EvidenceCardProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<EvidenceCardDowngradeTrigger> {
    vec![
        EvidenceCardDowngradeTrigger::ProofStale,
        EvidenceCardDowngradeTrigger::EvidenceStale,
        EvidenceCardDowngradeTrigger::FindingEvidenceMissing,
        EvidenceCardDowngradeTrigger::BindingDetachUnlabeled,
        EvidenceCardDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<EvidenceCardConsumerSurface> {
    vec![
        EvidenceCardConsumerSurface::ReviewWorkspaceFindings,
        EvidenceCardConsumerSurface::ReviewPackPanel,
        EvidenceCardConsumerSurface::CliHeadless,
        EvidenceCardConsumerSurface::SupportExport,
        EvidenceCardConsumerSurface::Diagnostics,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        EVIDENCE_CARD_SCHEMA_REF.to_owned(),
        EVIDENCE_CARD_DOC_REF.to_owned(),
        EVIDENCE_CARD_AI_EVIDENCE_CONTRACT_REF.to_owned(),
        EVIDENCE_CARD_REVIEW_PACK_CONTRACT_REF.to_owned(),
        EVIDENCE_CARD_CHANGE_OBJECT_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> EvidenceCardPacket {
    EvidenceCardPacket::new(EvidenceCardPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label:
            "AI review evidence, finding cards, and review-pack change-object integration"
                .to_owned(),
        evidence_rows: evidence_rows(),
        finding_cards: finding_cards(),
        review_pack_bindings: review_pack_bindings(),
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
fn evidence_card_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_rows_fails() {
    let mut packet = packet();
    packet.evidence_rows.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::EvidenceRowsMissing));
}

#[test]
fn duplicate_evidence_id_fails() {
    let mut packet = packet();
    packet.evidence_rows[1].evidence_id = "evidence:model:0001".to_owned();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::EvidenceIdDuplicated));
}

#[test]
fn finding_citing_missing_evidence_fails() {
    let mut packet = packet();
    packet.finding_cards[0].evidence_ref = "evidence:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::FindingEvidenceRefMissing));
}

#[test]
fn finding_with_unbound_change_object_fails() {
    let mut packet = packet();
    packet.finding_cards[0].change_object_ref = "cobj-unbound".to_owned();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::FindingChangeObjectUnbound));
}

#[test]
fn blocked_apply_without_label_fails() {
    let mut packet = packet();
    packet.finding_cards[1].apply_block_label = String::new();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::ApplyBlockLabelMissing));
}

#[test]
fn detached_binding_without_label_fails() {
    let mut packet = packet();
    packet.review_pack_bindings[0].binding_state = ReviewPackBindingState::DetachedRelabeled;
    packet.review_pack_bindings[0].detach_label = String::new();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::DetachLabelMissing));
}

#[test]
fn missing_review_pack_bindings_fails() {
    let mut packet = packet();
    packet.review_pack_bindings.clear();
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceCardViolation::ReviewPackBindingsMissing));
    // Findings can no longer be bound when there are no bindings.
    assert!(violations.contains(&EvidenceCardViolation::FindingChangeObjectUnbound));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_hidden_write_scope = false;
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.binding_shows_detach_label = false;
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&EvidenceCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_section() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Evidence"));
    assert!(summary.contains("## Finding cards"));
    assert!(summary.contains("## Review-pack / change-object bindings"));
    assert!(summary.contains("evidence:model:0001"));
    assert!(summary.contains("cobj-0001"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_evidence_card_export().expect("checked evidence-card export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/stale_evidence_apply_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/detached_binding_offline.json"
        )),
    ] {
        let packet: EvidenceCardPacket =
            serde_json::from_str(raw).expect("fixture parses as evidence-card packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
