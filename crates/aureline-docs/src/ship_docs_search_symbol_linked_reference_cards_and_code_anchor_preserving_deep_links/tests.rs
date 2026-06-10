use super::*;

fn packet() -> DocsSearchLinkPacket {
    DocsSearchLinkPacket::materialize(seeded_stable_docs_search_link_input())
}

#[test]
fn seeded_packet_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, DOCS_SEARCH_LINK_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_SEARCH_LINK_SCHEMA_VERSION);
}

#[test]
fn every_row_carries_chips_reason_and_escapes() {
    for row in packet().search_results {
        assert!(!row.ranking_reason.trim().is_empty());
        assert!(!row.open_raw_escape_ref.trim().is_empty());
        assert!(!row.open_source_escape_ref.trim().is_empty());
        let _ = (
            row.result_kind.as_str(),
            row.chips.source_class.as_str(),
            row.chips.version_match.as_str(),
            row.chips.freshness.as_str(),
        );
    }
}

#[test]
fn every_resolved_card_is_cited() {
    for card in packet().symbol_cards {
        if card.resolution_class.is_resolved() {
            assert!(
                !card.citation_anchor_refs.is_empty(),
                "resolved card `{}` must be cited",
                card.card_id
            );
        }
    }
}

#[test]
fn every_deep_link_preserves_its_code_anchor_and_return_path() {
    for link in packet().code_anchor_deep_links {
        assert!(link.preserves_anchor_across_export);
        assert!(!link.return_path_ref.trim().is_empty());
        assert!(!link.code_anchor.file_ref.trim().is_empty());
        assert!(!link.code_anchor.revision_ref.trim().is_empty());
        let _ = link.anchor_kind.as_str();
    }
}

#[test]
fn missing_ranking_reason_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.search_results[0].ranking_reason = "  ".to_owned();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsSearchLinkPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::RankingReasonMissing));
}

#[test]
fn missing_open_escape_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.search_results[1].open_source_escape_ref = String::new();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn row_linking_unknown_card_is_orphan() {
    let mut input = seeded_stable_docs_search_link_input();
    input.search_results[0].symbol_card_id_ref = Some("symref:does-not-exist".to_owned());
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::SymbolCardOrphan));
}

#[test]
fn row_linking_unknown_deep_link_is_orphan() {
    let mut input = seeded_stable_docs_search_link_input();
    input.search_results[0].deep_link_id_ref = Some("deeplink:does-not-exist".to_owned());
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DeepLinkRefOrphan));
}

#[test]
fn uncited_resolved_card_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.symbol_cards[0].citation_anchor_refs.clear();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::SymbolCardUncited));
}

#[test]
fn inconsistent_fallback_chain_is_flagged() {
    let mut input = seeded_stable_docs_search_link_input();
    // A package-level guide fallback with an empty chain is inconsistent.
    input.symbol_cards[1].resolution_fallback_chain.clear();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::ResolutionFallbackInconsistent));
}

#[test]
fn unresolved_card_without_repair_hook_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    let card = &mut input.symbol_cards[1];
    card.resolution_class = DocsSearchLinkResolutionClass::UnresolvedRequiresRefresh;
    card.resolution_fallback_chain = vec![
        DocsSearchLinkResolutionClass::ExactSymbolMatch,
        DocsSearchLinkResolutionClass::UnresolvedRequiresRefresh,
    ];
    card.citation_anchor_refs.clear();
    card.repair_hook = None;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::UnresolvedMissingRepairHook));
}

#[test]
fn refused_reuse_without_repair_hook_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.symbol_cards[0].reuse_state = DocsSearchLinkReuseState::RefusedSignatureUnverified;
    input.symbol_cards[0].repair_hook = None;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::RefusedReuseMissingRepairHook));
}

#[test]
fn vendor_overlay_without_handoff_is_flagged() {
    let mut input = seeded_stable_docs_search_link_input();
    let card = &mut input.symbol_cards[0];
    card.project_vs_vendor_cue = DocsSearchLinkProjectVendorCue::VendorProviderOverlayInspectOnly;
    card.source_class = DocsSearchLinkSourceClass::VendorProviderDocs;
    card.browser_handoff_reason = None;
    card.destination_descriptor_ref = None;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::VendorOverlayMissingHandoff));
}

#[test]
fn vendor_source_presented_as_project_authoritative_collapses_truth() {
    let mut input = seeded_stable_docs_search_link_input();
    input.symbol_cards[0].source_class = DocsSearchLinkSourceClass::VendorProviderDocs;
    // cue stays project_authoritative_only — a vendor source must not claim it.
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::ProjectVsVendorTruthCollapsed));
}

#[test]
fn deep_link_dropping_anchor_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.code_anchor_deep_links[0].preserves_anchor_across_export = false;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DeepLinkAnchorNotPreserved));
}

#[test]
fn deep_link_missing_return_path_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.code_anchor_deep_links[1].return_path_ref = String::new();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DeepLinkReturnPathMissing));
}

#[test]
fn deep_link_incomplete_anchor_is_flagged() {
    let mut input = seeded_stable_docs_search_link_input();
    input.code_anchor_deep_links[0].code_anchor.revision_ref = String::new();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DeepLinkCodeAnchorIncomplete));
}

#[test]
fn deep_link_handoff_without_descriptor_is_flagged() {
    let mut input = seeded_stable_docs_search_link_input();
    input.code_anchor_deep_links[0].browser_handoff_reason =
        Some(DocsSearchLinkBrowserHandoffReason::ExternalDocsOrRunbook);
    input.code_anchor_deep_links[0].destination_descriptor_ref = None;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DeepLinkHandoffMissingDescriptor));
}

#[test]
fn orphan_disclosure_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.resolution_disclosures[0].subject_id_ref = "does-not-exist".to_owned();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DisclosureOrphan));
}

#[test]
fn unresolved_disclosure_without_repair_hook_is_incomplete() {
    let mut input = seeded_stable_docs_search_link_input();
    input.resolution_disclosures[0].disclosure_class =
        DocsSearchLinkDisclosureClass::UnresolvedRequiresRefresh;
    input.resolution_disclosures[0].repair_hook = None;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::DisclosureIncomplete));
}

#[test]
fn narrowing_disclosure_narrows_below_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input.resolution_disclosures[0].severity = DocsSearchLinkFindingSeverity::Narrowing;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsSearchLinkPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn projection_dropping_resolution_drifts() {
    let mut input = seeded_stable_docs_search_link_input();
    input.consumer_projections[0].preserves_symbol_resolution = false;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_docs_search_link_input();
    input
        .consumer_projections
        .retain(|p| p.surface != DocsSearchLinkConsumerSurface::RetrievalInspector);
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_search_link_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn non_monotonic_rank_is_flagged() {
    let mut input = seeded_stable_docs_search_link_input();
    input.search_results[0].rank = 9;
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::ResultRankNotMonotonic));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_docs_search_link_input();
    input.search_results[0].ranking_reason = "matched on bearer abc123 token".to_owned();
    let packet = DocsSearchLinkPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsSearchLinkFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_rows_cards_and_links() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for row in &packet.search_results {
        assert!(summary.contains(&row.result_id));
    }
    assert!(summary.contains("Symbol reference cards"));
    assert!(summary.contains("Code-anchor deep links"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-10T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsSearchLinkSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        DOCS_SEARCH_LINK_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_docs_search_link_export()
        .expect("checked search-link export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:docs_search_link:http_client_publish"
    );
    assert_eq!(
        export.packet.promotion_state,
        DocsSearchLinkPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/symbol_link_unresolved_narrowed.json"
            )),
            DocsSearchLinkPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/deep_link_drops_anchor_blocks_stable.json"
            )),
            DocsSearchLinkPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/ship_docs_search_symbol_linked_reference_cards_and_code_anchor_preserving_deep_links/vendor_overlay_uncited_blocks_stable.json"
            )),
            DocsSearchLinkPromotionState::BlocksStable,
        ),
    ] {
        let fixture: SearchLinkFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = DocsSearchLinkPacket::materialize(fixture.input);
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
struct SearchLinkFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: DocsSearchLinkPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
