use super::*;

fn packet() -> SemanticRecallLedgerPacket {
    SemanticRecallLedgerPacket::materialize(seeded_stable_semantic_recall_ledger_input())
}

#[test]
fn seeded_recall_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, SEMANTIC_RECALL_LEDGER_RECORD_KIND);
    assert_eq!(packet.schema_version, SEMANTIC_RECALL_LEDGER_SCHEMA_VERSION);
}

#[test]
fn every_row_carries_chips_reason_signals_and_escapes() {
    for row in packet().result_rows {
        assert!(!row.ranking_reason.trim().is_empty());
        assert!(!row.ranking_signals.is_empty());
        assert!(!row.open_raw_escape_ref.trim().is_empty());
        assert!(!row.open_source_escape_ref.trim().is_empty());
        // Touch each chip so the tokens stay stable across refactors.
        let _ = (
            row.chips.source_class.as_str(),
            row.chips.version_match.as_str(),
            row.chips.freshness.as_str(),
            row.chips.locality.as_str(),
            row.chips.confidence.as_str(),
            row.subject_kind.as_str(),
        );
    }
}

#[test]
fn session_covers_both_docs_and_code_subjects() {
    let packet = packet();
    assert!(packet
        .result_rows
        .iter()
        .any(|row| row.subject_kind == SemanticRecallSubjectKind::DocsNode));
    assert!(packet
        .result_rows
        .iter()
        .any(|row| row.subject_kind.is_code()));
}

#[test]
fn missing_ranking_reason_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.result_rows[0].ranking_reason = "  ".to_owned();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SemanticRecallPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::RankingReasonMissing));
}

#[test]
fn missing_ranking_signals_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.result_rows[0].ranking_signals.clear();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::RankingSignalsMissing));
}

#[test]
fn missing_open_escape_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.result_rows[1].open_source_escape_ref = String::new();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn uncited_derived_result_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    // Row 4 is a derived summary; dropping its citation must block.
    input.result_rows[3].provenance.cited = false;
    input.provenance_export.rows[3].cited = false;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::CodeResultNotCited));
}

#[test]
fn inferred_result_presented_as_high_confidence_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    let row = &mut input.result_rows[3];
    row.provenance.derivation = DerivationClass::InferredExplanation;
    row.chips.confidence = SemanticRecallConfidence::High;
    input.provenance_export.rows[3].derivation = DerivationClass::InferredExplanation;
    input.provenance_export.rows[3].confidence = SemanticRecallConfidence::High;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::InferredResultLooksAuthoritative));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    let row = &mut input.result_rows[2];
    row.chips.version_match = SemanticRecallVersionMatch::IncompatibleDriftDetected;
    row.chips.confidence = SemanticRecallConfidence::High;
    row.chips.freshness = SemanticRecallFreshness::AuthoritativeLive;
    input.provenance_export.rows[2].confidence = SemanticRecallConfidence::High;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::VersionTruthCollapsed));
}

#[test]
fn empty_ledger_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.query_session_ledger.entries.clear();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::LedgerEmpty));
}

#[test]
fn non_initial_first_entry_is_inconsistent() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.query_session_ledger.entries[0].refinement = QueryRefinementRelation::Narrowed;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::LedgerRefinementInconsistent));
}

#[test]
fn ledger_surfacing_unknown_result_is_orphan() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.query_session_ledger.entries[0]
        .surfaced_result_ids
        .push("result:does-not-exist".to_owned());
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::LedgerSurfacedResultOrphan));
}

#[test]
fn result_origin_outside_ledger_is_orphan() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.result_rows[0].origin_query_sequence = 99;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ResultOriginQueryOrphan));
}

#[test]
fn provenance_dropping_source_class_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.provenance_export.preserves_source_class = false;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ProvenanceExportDropsPreservation));
}

#[test]
fn provenance_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.provenance_export.rows[0].source_class = SemanticRecallSourceClass::DependencySource;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ProvenanceSourceClassMismatch));
}

#[test]
fn provenance_missing_coverage_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.provenance_export.rows.pop();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ProvenanceExportCoverageMissing));
}

#[test]
fn provenance_orphan_row_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.provenance_export.rows[0].result_id_ref = "result:does-not-exist".to_owned();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ProvenanceExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.recall_degradations.push(RecallDegradation {
        degradation_class: RecallDegradationClass::EmbedderUnavailableLexicalFallback,
        severity: SemanticRecallFindingSeverity::Narrowing,
        summary: "embedder unavailable; ranking fell back to lexical signals only".to_owned(),
        result_id_ref: None,
        evidence_ref: None,
    });
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SemanticRecallPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.recall_degradations.push(RecallDegradation {
        degradation_class: RecallDegradationClass::QuarantinedPack,
        severity: SemanticRecallFindingSeverity::Blocking,
        summary: "a surfaced pack is quarantined and must not be presented as publishable"
            .to_owned(),
        result_id_ref: Some("result:docs:mirrored_backoff_reference".to_owned()),
        evidence_ref: None,
    });
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        SemanticRecallPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_result_is_orphan() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.recall_degradations[0].result_id_ref = Some("result:does-not-exist".to_owned());
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_ledger_drifts() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.consumer_projections[0].preserves_query_session_ledger = false;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input
        .consumer_projections
        .retain(|p| p.surface != SemanticRecallConsumerSurface::CodeExplainer);
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn non_monotonic_rank_is_flagged() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.result_rows[0].rank = 9;
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::ResultRankNotMonotonic));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_semantic_recall_ledger_input();
    input.result_rows[0].ranking_reason = "matched on bearer abc123 token in the query".to_owned();
    let packet = SemanticRecallLedgerPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == SemanticRecallFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_ledger_rows_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for entry in &packet.query_session_ledger.entries {
        assert!(summary.contains(&entry.query_label));
    }
    for row in &packet.result_rows {
        assert!(summary.contains(&row.result_id));
    }
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-08T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: SemanticRecallLedgerSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        SEMANTIC_RECALL_LEDGER_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_semantic_recall_ledger_export()
        .expect("checked recall export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:semantic_recall:retry_backoff_session"
    );
    assert_eq!(
        export.packet.promotion_state,
        SemanticRecallPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/embedder_unavailable_lexical_fallback_narrowed.json"
            )),
            SemanticRecallPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/uncited_code_explainer_blocks_stable.json"
            )),
            SemanticRecallPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_docs_and_code_semantic_recall_with_query_session_ledger_ranking_reasons_and_provenance_export/inferred_explanation_over_authoritative_blocks_stable.json"
            )),
            SemanticRecallPromotionState::BlocksStable,
        ),
    ] {
        let fixture: RecallFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = SemanticRecallLedgerPacket::materialize(fixture.input);
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
struct RecallFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: SemanticRecallLedgerPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
