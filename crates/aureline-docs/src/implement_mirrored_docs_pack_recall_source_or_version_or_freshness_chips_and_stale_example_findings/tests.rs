use super::*;

fn packet() -> DocsPackRecallPacket {
    DocsPackRecallPacket::materialize(seeded_stable_docs_pack_recall_input())
}

#[test]
fn seeded_recall_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, DOCS_PACK_RECALL_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_PACK_RECALL_SCHEMA_VERSION);
}

#[test]
fn every_row_carries_a_full_chip_set_and_escapes() {
    for row in packet().result_rows {
        assert!(!row.ranking_reason.trim().is_empty());
        assert!(!row.open_raw_escape_ref.trim().is_empty());
        assert!(!row.open_source_escape_ref.trim().is_empty());
        // The chip set is a fixed five-tuple; touching each field keeps the
        // tokens stable across refactors.
        let _ = (
            row.chips.source_class.as_str(),
            row.chips.version_match.as_str(),
            row.chips.freshness.as_str(),
            row.chips.locality.as_str(),
            row.chips.confidence.as_str(),
        );
    }
}

#[test]
fn missing_ranking_reason_blocks_stable() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.result_rows[0].ranking_reason = "  ".to_owned();
    let packet = DocsPackRecallPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsPackRecallPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::RankingReasonMissing));
}

#[test]
fn missing_open_escape_blocks_stable() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.result_rows[1].open_source_escape_ref = String::new();
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn unpinned_live_mirror_is_flagged_as_over_authoritative() {
    let mut input = seeded_stable_docs_pack_recall_input();
    let row = &mut input.result_rows[1];
    assert!(row.chips.source_class.is_mirrored_upstream());
    row.chips.freshness = DocsPackRecallFreshness::AuthoritativeLive;
    row.pack_pinned = false;
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::LiveMirrorLooksMoreAuthoritative));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_docs_pack_recall_input();
    let row = &mut input.result_rows[2];
    row.chips.version_match = DocsPackRecallVersionMatch::IncompatibleDriftDetected;
    row.chips.confidence = DocsPackRecallConfidence::High;
    row.chips.freshness = DocsPackRecallFreshness::AuthoritativeLive;
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::VersionTruthCollapsed));
}

#[test]
fn dropping_all_mirrored_rows_loses_mirror_awareness() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input
        .result_rows
        .retain(|row| !row.chips.source_class.is_mirrored_upstream());
    for (index, row) in input.result_rows.iter_mut().enumerate() {
        row.rank = (index as u32) + 1;
    }
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::MirroredSourceCoverageMissing));
}

#[test]
fn nearby_version_finding_without_ref_is_incomplete() {
    let mut input = seeded_stable_docs_pack_recall_input();
    for finding in input.stale_example_findings.iter_mut() {
        if finding.finding_class == DocsPackRecallStaleFindingClass::NearbyVersion {
            finding.nearby_version_ref = None;
        }
    }
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::StaleFindingIncomplete));
}

#[test]
fn orphan_stale_finding_blocks_stable() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.stale_example_findings[0].result_id_ref = "result:does-not-exist".to_owned();
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::StaleFindingOrphan));
}

#[test]
fn projection_dropping_chips_drifts() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.consumer_projections[0].preserves_chips = false;
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input
        .consumer_projections
        .retain(|p| p.surface != DocsPackRecallConsumerSurface::RetrievalInspector);
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn non_monotonic_rank_is_flagged() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.result_rows[0].rank = 9;
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::ResultRankNotMonotonic));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_docs_pack_recall_input();
    input.result_rows[0].ranking_reason = "matched on bearer abc123 token in the query".to_owned();
    let packet = DocsPackRecallPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsPackRecallFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_rows_and_findings() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for row in &packet.result_rows {
        assert!(summary.contains(&row.result_id));
    }
    assert!(summary.contains("Stale-example findings"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-08T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsPackRecallSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        DOCS_PACK_RECALL_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_docs_pack_recall_export()
        .expect("checked recall export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:docs_pack_recall:async_runtime_setup"
    );
    assert_eq!(
        export.packet.promotion_state,
        DocsPackRecallPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/mirror_offline_recall_narrowed.json"
            )),
            DocsPackRecallPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_mirrored_docs_pack_recall_source_or_version_or_freshness_chips_and_stale_example_findings/live_mirror_over_authoritative_blocks_stable.json"
            )),
            DocsPackRecallPromotionState::BlocksStable,
        ),
    ] {
        let fixture: RecallFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = DocsPackRecallPacket::materialize(fixture.input);
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
    input: DocsPackRecallPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
