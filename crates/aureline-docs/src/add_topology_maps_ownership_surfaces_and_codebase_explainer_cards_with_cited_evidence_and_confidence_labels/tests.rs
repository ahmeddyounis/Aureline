use super::*;

fn packet() -> CodebaseUnderstandingCardsPacket {
    CodebaseUnderstandingCardsPacket::materialize(seeded_stable_codebase_understanding_cards_input())
}

#[test]
fn seeded_cards_are_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, UNDERSTANDING_CARDS_RECORD_KIND);
    assert_eq!(packet.schema_version, UNDERSTANDING_CARDS_SCHEMA_VERSION);
}

#[test]
fn packet_carries_all_three_card_kinds() {
    let kinds: BTreeSet<UnderstandingCardKind> =
        packet().cards.iter().map(|card| card.card_kind).collect();
    for required in REQUIRED_CARD_KINDS {
        assert!(kinds.contains(&required), "missing card kind {required:?}");
    }
}

#[test]
fn every_card_carries_chips_reason_evidence_and_escapes() {
    for card in packet().cards {
        assert!(!card.title.trim().is_empty());
        assert!(!card.headline.trim().is_empty());
        assert!(!card.confidence_reason.trim().is_empty());
        assert!(!card.evidence.is_empty());
        assert!(!card.open_raw_escape_ref.trim().is_empty());
        assert!(!card.open_source_escape_ref.trim().is_empty());
        // Touch each chip so the tokens stay stable across refactors.
        let _ = (
            card.chips.source_class.as_str(),
            card.chips.version_match.as_str(),
            card.chips.freshness.as_str(),
            card.chips.locality.as_str(),
            card.chips.confidence.as_str(),
            card.card_kind.as_str(),
        );
        for evidence in card.evidence {
            assert!(!evidence.open_raw_escape_ref.trim().is_empty());
            assert!(!evidence.open_source_escape_ref.trim().is_empty());
            let _ = (evidence.subject_kind.as_str(), evidence.derivation.as_str());
        }
    }
}

#[test]
fn missing_required_card_kind_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    // Drop the ownership card, then prune its export row + projection refs.
    let dropped = input
        .cards
        .iter()
        .position(|card| card.card_kind == UnderstandingCardKind::OwnershipSurface)
        .expect("ownership card present");
    let dropped_id = input.cards.remove(dropped).card_id;
    input
        .evidence_export
        .rows
        .retain(|row| row.card_id_ref != dropped_id);
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        UnderstandingPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::RequiredCardKindMissing));
}

#[test]
fn missing_confidence_reason_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.cards[0].confidence_reason = "  ".to_owned();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::ConfidenceReasonMissing));
}

#[test]
fn empty_card_evidence_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.cards[0].evidence.clear();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::CardEvidenceMissing));
}

#[test]
fn uncited_derived_card_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    // The topology card is a derived summary; dropping its citation must block.
    input.cards[0].provenance.cited = false;
    input.cards[0].provenance.citation_ref = None;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::CardNotCited));
}

#[test]
fn uncited_inferred_evidence_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    let explainer = input
        .cards
        .iter_mut()
        .find(|card| card.card_kind == UnderstandingCardKind::CodebaseExplainer)
        .expect("explainer card present");
    explainer.evidence[0].derivation = EvidenceDerivation::DerivedSummary;
    explainer.evidence[0].cited = false;
    explainer.evidence[0].citation_ref = None;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::EvidenceNotCited));
}

#[test]
fn inferred_card_presented_as_high_confidence_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    let explainer = input
        .cards
        .iter_mut()
        .find(|card| card.card_kind == UnderstandingCardKind::CodebaseExplainer)
        .expect("explainer card present");
    explainer.chips.confidence = UnderstandingConfidence::High;
    // Keep the export row aligned so the only finding is the authority one.
    let card_id = explainer.card_id.clone();
    for row in input.evidence_export.rows.iter_mut() {
        if row.card_id_ref == card_id {
            row.confidence = UnderstandingConfidence::High;
        }
    }
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::InferredCardLooksAuthoritative));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    // Ownership card is high-confidence + authoritative-live; flip its version.
    let card = input
        .cards
        .iter_mut()
        .find(|card| card.card_kind == UnderstandingCardKind::OwnershipSurface)
        .expect("ownership card present");
    card.chips.version_match = UnderstandingVersionMatch::IncompatibleDriftDetected;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::VersionTruthCollapsed));
}

#[test]
fn topology_card_without_edges_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.cards[0].topology_edges.clear();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::TopologyCardMissingEdges));
}

#[test]
fn ownership_card_without_owner_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    let card = input
        .cards
        .iter_mut()
        .find(|card| card.card_kind == UnderstandingCardKind::OwnershipSurface)
        .expect("ownership card present");
    card.owners.clear();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::OwnershipCardMissingOwner));
}

#[test]
fn heuristic_owner_at_high_confidence_is_unattributed() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    let card = input
        .cards
        .iter_mut()
        .find(|card| card.card_kind == UnderstandingCardKind::OwnershipSurface)
        .expect("ownership card present");
    card.owners[0].ownership_basis = OwnershipBasis::GitHistoryHeuristic;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::OwnershipBasisUnattributed));
}

#[test]
fn export_dropping_card_kind_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.evidence_export.preserves_card_kind = false;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::EvidenceExportDropsPreservation));
}

#[test]
fn export_card_kind_mismatch_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.evidence_export.rows[0].card_kind = UnderstandingCardKind::CodebaseExplainer;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::EvidenceExportCardKindMismatch));
}

#[test]
fn export_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.evidence_export.rows[0].source_class = UnderstandingSourceClass::DependencySource;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::EvidenceExportSourceClassMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.evidence_export.rows.pop();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::EvidenceExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.evidence_export.rows[0].card_id_ref = "card:does-not-exist".to_owned();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::EvidenceExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input
        .understanding_degradations
        .push(UnderstandingDegradation {
            degradation_class: UnderstandingDegradationClass::EmbedderUnavailableLexicalFallback,
            severity: UnderstandingFindingSeverity::Narrowing,
            summary: "embedder unavailable; explainers fell back to lexical signals only"
                .to_owned(),
            card_id_ref: None,
            evidence_ref: None,
        });
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        UnderstandingPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input
        .understanding_degradations
        .push(UnderstandingDegradation {
            degradation_class: UnderstandingDegradationClass::QuarantinedPack,
            severity: UnderstandingFindingSeverity::Blocking,
            summary: "a surfaced pack is quarantined and must not be presented as publishable"
                .to_owned(),
            card_id_ref: Some("card:topology:net_retry_region".to_owned()),
            evidence_ref: None,
        });
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        UnderstandingPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_card_is_orphan() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.understanding_degradations[0].card_id_ref = Some("card:does-not-exist".to_owned());
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_cards_drifts() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.consumer_projections[0].preserves_cards = false;
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input
        .consumer_projections
        .retain(|p| p.surface != UnderstandingConsumerSurface::GraphPanel);
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_card_id_is_flagged() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    let clone = input.cards[0].clone();
    input.cards.push(clone);
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::DuplicateCardId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_codebase_understanding_cards_input();
    input.cards[0].headline = "matched on bearer abc123 token in the source".to_owned();
    let packet = CodebaseUnderstandingCardsPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == UnderstandingFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_cards_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for card in &packet.cards {
        assert!(summary.contains(&card.card_id));
    }
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-09T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: CodebaseUnderstandingCardsSupportExport =
        serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        UNDERSTANDING_CARDS_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_codebase_understanding_cards_export()
        .expect("checked cards export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:understanding_cards:net_retry_region"
    );
    assert_eq!(
        export.packet.promotion_state,
        UnderstandingPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/graph_index_stale_narrowed.json"
            )),
            UnderstandingPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/uncited_topology_card_blocks_stable.json"
            )),
            UnderstandingPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/inferred_explainer_over_authoritative_blocks_stable.json"
            )),
            UnderstandingPromotionState::BlocksStable,
        ),
    ] {
        let fixture: CardsFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = CodebaseUnderstandingCardsPacket::materialize(fixture.input);
        assert_eq!(
            packet.promotion_state, expected,
            "fixture `{}` expected {:?}, findings: {:?}",
            fixture.case_name, expected, packet.validation_findings
        );
        for expected_kind in fixture.expect.expected_finding_kinds {
            assert!(
                packet
                    .validation_findings
                    .iter()
                    .any(|f| f.finding_kind.as_str() == expected_kind),
                "fixture `{}` expected finding `{}`",
                fixture.case_name,
                expected_kind
            );
        }
    }
}

#[derive(Debug, Deserialize)]
struct CardsFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: CodebaseUnderstandingCardsPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
