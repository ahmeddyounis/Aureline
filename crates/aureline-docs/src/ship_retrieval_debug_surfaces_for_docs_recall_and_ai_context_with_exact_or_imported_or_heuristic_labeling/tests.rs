use super::*;
use crate::DocsInfraTruthLayer;

fn packet() -> RetrievalDebugPacket {
    RetrievalDebugPacket::materialize(seeded_stable_retrieval_debug_input())
}

#[test]
fn seeded_set_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, RETRIEVAL_DEBUG_RECORD_KIND);
    assert_eq!(packet.schema_version, RETRIEVAL_DEBUG_SCHEMA_VERSION);
}

#[test]
fn packet_carries_all_three_lanes() {
    let lanes: BTreeSet<RetrievalLane> = packet().entries.iter().map(|e| e.lane).collect();
    for required in RetrievalLane::ALL {
        assert!(lanes.contains(&required), "missing lane {required:?}");
    }
}

#[test]
fn seeded_set_uses_each_derivation_label() {
    let labels: BTreeSet<RetrievalDerivationLabel> = packet()
        .entries
        .iter()
        .map(|e| e.derivation_label)
        .collect();
    for required in RetrievalDerivationLabel::ALL {
        assert!(labels.contains(&required), "missing label {required:?}");
    }
}

#[test]
fn every_entry_carries_chips_reason_signals_and_escapes() {
    for entry in packet().entries {
        assert!(!entry.title.trim().is_empty());
        assert!(!entry.headline.trim().is_empty());
        assert!(!entry.derivation_reason.trim().is_empty());
        assert!(!entry.ranking_signals.is_empty());
        assert!(!entry.open_raw_escape_ref.trim().is_empty());
        assert!(!entry.open_source_escape_ref.trim().is_empty());
        // Touch each token so it stays stable across refactors.
        let _ = (
            entry.lane.as_str(),
            entry.derivation_label.as_str(),
            entry.subject_kind.as_str(),
            entry.chips.source_class.as_str(),
            entry.chips.version_match.as_str(),
            entry.chips.freshness.as_str(),
            entry.chips.locality.as_str(),
            entry.chips.confidence.as_str(),
        );
        for signal in entry.ranking_signals {
            assert!(!signal.note.trim().is_empty());
            let _ = (signal.signal_kind.as_str(), signal.contribution.as_str());
        }
    }
}

#[test]
fn seeded_set_preserves_infra_truth_layers_and_visible_limits() {
    let packet = packet();
    let docs_entry = packet
        .entries
        .iter()
        .find(|entry| entry.entry_id == "entry:docs:checkout-rendered-manifest")
        .expect("infra docs entry present");
    let docs_lineage = docs_entry
        .infrastructure_lineage
        .as_ref()
        .expect("infra lineage present");
    assert_eq!(
        docs_lineage.truth_layer_tokens().join(","),
        "authored_desired,rendered_expanded,planned_validated"
    );
    assert_eq!(
        docs_lineage.visible_limit_summary.as_deref(),
        Some(
            "Provider overlay was unavailable, so the answer degraded to authored, rendered, and planned checkout intelligence."
        )
    );

    let ai_entry = packet
        .entries
        .iter()
        .find(|entry| entry.entry_id == "entry:ai_context:checkout-drift-fragment")
        .expect("infra ai-context entry present");
    assert_eq!(
        ai_entry
            .infrastructure_lineage
            .as_ref()
            .expect("infra lineage present")
            .unavailable_truth_layer_tokens(),
        vec!["observed_live".to_owned(), "provider_overlay".to_owned()]
    );
}

#[test]
fn missing_required_lane_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    let dropped = input
        .entries
        .iter()
        .position(|e| e.lane == RetrievalLane::AiContext)
        .expect("ai-context entry present");
    let dropped_id = input.entries.remove(dropped).entry_id;
    input.export.rows.retain(|r| r.entry_id_ref != dropped_id);
    let packet = RetrievalDebugPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        RetrievalPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::RequiredLaneMissing));
}

#[test]
fn missing_derivation_reason_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.entries[0].derivation_reason = "  ".to_owned();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::DerivationReasonMissing));
}

#[test]
fn entry_without_ranking_signals_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.entries[0].ranking_signals.clear();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::RankingSignalsMissing));
}

#[test]
fn ranking_signal_without_note_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.entries[0].ranking_signals[0].note = "   ".to_owned();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::RankingSignalNoteMissing));
}

#[test]
fn uncited_imported_entry_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.derivation_label == RetrievalDerivationLabel::Imported)
        .expect("imported entry present");
    entry.cited = false;
    entry.citation_ref = None;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.cited = false;
        }
    }
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::EntryNotCited));
}

#[test]
fn heuristic_entry_at_high_confidence_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|e| e.derivation_label == RetrievalDerivationLabel::Heuristic)
        .expect("heuristic entry present");
    entry.chips.confidence = RetrievalConfidence::High;
    let id = entry.entry_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.entry_id_ref == id {
            row.confidence = RetrievalConfidence::High;
        }
    }
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::HeuristicLabelLooksAuthoritative));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_retrieval_debug_input();
    // The docs-search entry is high-confidence; drift its version and freshness into a fake live claim.
    input.entries[0].chips.version_match = RetrievalVersionMatch::IncompatibleDriftDetected;
    input.entries[0].chips.freshness = RetrievalFreshness::AuthoritativeLive;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::VersionTruthCollapsed));
}

#[test]
fn missing_infra_limit_summary_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    let entry = input
        .entries
        .iter_mut()
        .find(|entry| entry.entry_id == "entry:ai_context:checkout-drift-fragment")
        .expect("infra ai-context entry present");
    entry
        .infrastructure_lineage
        .as_mut()
        .expect("infra lineage present")
        .visible_limit_summary = None;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::InfrastructureLineageInvalid));
}

#[test]
fn export_must_preserve_infra_truth_layers() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.rows[0].infrastructure_truth_layers = vec![DocsInfraTruthLayer::AuthoredDesired];
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| { f.finding_kind == RetrievalFindingKind::ExportInfrastructureLineageMismatch }));
}

#[test]
fn missing_escapes_block_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.entries[0].open_raw_escape_ref = "  ".to_owned();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn export_dropping_derivation_label_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.preserves_derivation_label = false;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ExportDropsPreservation));
}

#[test]
fn export_lane_mismatch_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.rows[0].lane = RetrievalLane::AiContext;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ExportLaneMismatch));
}

#[test]
fn export_derivation_label_mismatch_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.rows[0].derivation_label = RetrievalDerivationLabel::Heuristic;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ExportDerivationLabelMismatch));
}

#[test]
fn export_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.rows[0].source_class = RetrievalSourceClass::ImportedPack;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ExportSourceClassMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.rows.pop();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.export.rows[0].entry_id_ref = "entry:does-not-exist".to_owned();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.retrieval_degradations.push(RetrievalDegradation {
        degradation_class: RetrievalDegradationClass::EmbedderUnavailableLexicalFallback,
        severity: RetrievalFindingSeverity::Narrowing,
        summary: "embedder unavailable; recall fell back to lexical signals only".to_owned(),
        entry_id_ref: None,
        evidence_ref: None,
    });
    let packet = RetrievalDebugPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        RetrievalPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.retrieval_degradations.push(RetrievalDegradation {
        degradation_class: RetrievalDegradationClass::QuarantinedPack,
        severity: RetrievalFindingSeverity::Blocking,
        summary: "a surfaced pack is quarantined and must not be presented as publishable"
            .to_owned(),
        entry_id_ref: Some("entry:recall:backoff_policy_guide".to_owned()),
        evidence_ref: None,
    });
    let packet = RetrievalDebugPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        RetrievalPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_entry_is_orphan() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.retrieval_degradations[0].entry_id_ref = Some("entry:does-not-exist".to_owned());
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_lanes_drifts() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.consumer_projections[0].preserves_lanes = false;
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_retrieval_debug_input();
    input
        .consumer_projections
        .retain(|p| p.surface != RetrievalConsumerSurface::SemanticRecallPanel);
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_entry_id_is_flagged() {
    let mut input = seeded_stable_retrieval_debug_input();
    let clone = input.entries[0].clone();
    input.entries.push(clone);
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::DuplicateEntryId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_retrieval_debug_input();
    input.entries[0].headline = "matched on bearer abc123 token in the source".to_owned();
    let packet = RetrievalDebugPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == RetrievalFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_entries_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for entry in &packet.entries {
        assert!(summary.contains(&entry.entry_id));
    }
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-09T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: RetrievalDebugSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        RETRIEVAL_DEBUG_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_retrieval_debug_export()
        .expect("checked retrieval-debug export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:retrieval_debug:checkout_infra_query"
    );
    assert_eq!(
        export.packet.promotion_state,
        RetrievalPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/index_stale_narrowed.json"
            )),
            RetrievalPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/uncited_imported_entry_blocks_stable.json"
            )),
            RetrievalPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_retrieval_debug_surfaces_for_docs_recall_and_ai_context_with_exact_or_imported_or_heuristic_labeling/heuristic_entry_high_confidence_blocks_stable.json"
            )),
            RetrievalPromotionState::BlocksStable,
        ),
    ] {
        let fixture: RetrievalFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = RetrievalDebugPacket::materialize(fixture.input);
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
struct RetrievalFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: RetrievalDebugPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
