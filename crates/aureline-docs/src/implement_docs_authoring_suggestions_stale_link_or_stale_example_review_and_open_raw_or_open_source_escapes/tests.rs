use super::*;

fn packet() -> DocsAuthoringReviewPacket {
    DocsAuthoringReviewPacket::materialize(seeded_stable_docs_authoring_review_input())
}

#[test]
fn seeded_set_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, DOCS_AUTHORING_REVIEW_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_AUTHORING_REVIEW_SCHEMA_VERSION);
}

#[test]
fn packet_covers_suggestion_link_and_example_kinds() {
    let kinds: BTreeSet<DocsReviewItemKind> = packet().items.iter().map(|i| i.item_kind).collect();
    for required in DocsReviewItemKind::REQUIRED {
        assert!(kinds.contains(&required), "missing kind {required:?}");
    }
}

#[test]
fn every_item_carries_suggestion_review_and_escapes() {
    for item in packet().items {
        assert!(!item.title.trim().is_empty());
        assert!(!item.detail.trim().is_empty());
        assert!(!item.trust_disclosure_note.trim().is_empty());
        assert!(!item.suggestion.note.trim().is_empty());
        assert!(!item.review.note.trim().is_empty());
        assert!(!item.open_raw_escape_ref.trim().is_empty());
        assert!(!item.open_source_escape_ref.trim().is_empty());
        // No unverified item offers a one-click apply in the seeded set.
        if item.suggestion.apply_posture.offers_one_click_apply() {
            assert!(item.trust_class.may_be_authoritative());
            assert!(item.review.severity != DocsReviewFindingSeverity::Blocking);
        }
        // No stale verdict claims live-authoritative freshness.
        if item.review.finding_class.is_stale() {
            assert!(!item.chips.freshness.is_authoritative_live());
        }
        // Touch each token so it stays stable across refactors.
        let _ = (
            item.item_kind.as_str(),
            item.trust_class.as_str(),
            item.suggestion.apply_posture.as_str(),
            item.suggestion.trigger.as_str(),
            item.review.finding_class.as_str(),
            item.review.severity.as_str(),
            item.captured_vs_live.as_str(),
            item.chips.source_class.as_str(),
            item.chips.version_match.as_str(),
            item.chips.freshness.as_str(),
            item.chips.locality.as_str(),
            item.chips.confidence.as_str(),
        );
    }
}

#[test]
fn missing_required_kind_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let dropped = input
        .items
        .iter()
        .position(|i| i.item_kind == DocsReviewItemKind::StaleLinkReview)
        .expect("stale-link review item present");
    let dropped_id = input.items.remove(dropped).item_id;
    input.export.rows.retain(|r| r.item_id_ref != dropped_id);
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsReviewPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::RequiredItemKindMissing));
}

#[test]
fn missing_title_or_detail_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.items[0].detail = "  ".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ItemTitleOrDetailMissing));
}

#[test]
fn missing_trust_disclosure_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.items[0].trust_disclosure_note = "   ".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::TrustClassDisclosureMissing));
}

#[test]
fn untrusted_origin_at_high_confidence_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = input
        .items
        .iter_mut()
        .find(|i| i.trust_class == DocsReviewTrustClass::SignedDocsPack)
        .expect("signed docs pack item present");
    // Signed pack is authoritative; force an unverified trust class first.
    item.trust_class = DocsReviewTrustClass::LiveMirrorSuggestion;
    item.chips.confidence = DocsReviewConfidence::High;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.trust_class = DocsReviewTrustClass::LiveMirrorSuggestion;
            row.confidence = DocsReviewConfidence::High;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::TrustClassDisclosureCollapsed));
}

#[test]
fn uncited_untrusted_item_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = input
        .items
        .iter_mut()
        .find(|i| i.trust_class.needs_citation())
        .expect("untrusted item present");
    item.cited = false;
    item.citation_ref = None;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.cited = false;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ItemNotCited));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = input
        .items
        .iter_mut()
        .find(|i| i.chips.confidence == DocsReviewConfidence::High)
        .expect("high-confidence item present");
    item.chips.version_match = DocsReviewVersionMatch::IncompatibleDriftDetected;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::VersionTruthCollapsed));
}

#[test]
fn missing_apply_note_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.items[0].suggestion.note = "  ".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ApplyPostureNoteMissing));
}

#[test]
fn unverified_suggestion_one_click_apply_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = &mut input.items[0];
    item.trust_class = DocsReviewTrustClass::LiveMirrorSuggestion;
    item.suggestion.apply_posture = SuggestionApplyPosture::ApplyAvailable;
    item.chips.confidence = DocsReviewConfidence::Medium;
    item.cited = true;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.trust_class = DocsReviewTrustClass::LiveMirrorSuggestion;
            row.apply_posture = SuggestionApplyPosture::ApplyAvailable;
            row.confidence = DocsReviewConfidence::Medium;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::UnverifiedSuggestionApplyOffered));
}

#[test]
fn one_click_apply_on_blocking_review_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    // The first item is first-party with apply_available; make its review block.
    let item = &mut input.items[0];
    item.review.finding_class = ReviewFindingClass::StaleExampleUncompilable;
    item.review.severity = DocsReviewFindingSeverity::Blocking;
    item.chips.freshness = DocsReviewFreshness::Stale;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.finding_class = ReviewFindingClass::StaleExampleUncompilable;
            row.review_severity = DocsReviewFindingSeverity::Blocking;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ApplyOfferedOnBlockingFinding));
    assert_eq!(
        packet.promotion_state,
        DocsReviewPromotionState::BlocksStable
    );
}

#[test]
fn missing_review_note_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.items[0].review.note = "  ".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ReviewVerdictNoteMissing));
}

#[test]
fn stale_verdict_claiming_live_freshness_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = input
        .items
        .iter_mut()
        .find(|i| i.item_kind == DocsReviewItemKind::StaleLinkReview)
        .expect("stale-link review item present");
    item.review.finding_class = ReviewFindingClass::StaleLinkBroken;
    item.chips.freshness = DocsReviewFreshness::AuthoritativeLive;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.finding_class = ReviewFindingClass::StaleLinkBroken;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::StaleVerdictFreshnessMismatch));
}

#[test]
fn missing_escapes_block_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.items[0].open_raw_escape_ref = "  ".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::OpenRawOpenSourceEscapeMissing));
}

#[test]
fn export_dropping_apply_posture_preservation_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.preserves_apply_posture = false;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportDropsPreservation));
}

#[test]
fn export_apply_posture_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].apply_posture = SuggestionApplyPosture::ApplyBlockedByPolicy;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportApplyPostureMismatch));
}

#[test]
fn export_finding_class_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].finding_class = ReviewFindingClass::StaleExampleDrifted;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportFindingClassMismatch));
}

#[test]
fn export_item_kind_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].item_kind = DocsReviewItemKind::FreshnessReview;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportItemKindMismatch));
}

#[test]
fn export_trust_class_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].trust_class = DocsReviewTrustClass::DerivedHeuristicOnly;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportTrustClassMismatch));
}

#[test]
fn export_source_class_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].source_class = DocsReviewSourceClass::MirroredVendorDoc;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportSourceClassMismatch));
}

#[test]
fn export_confidence_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].confidence = DocsReviewConfidence::Low;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportConfidenceMismatch));
}

#[test]
fn export_cited_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].cited = false;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportCitedMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows.pop();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.export.rows[0].item_id_ref = "item:does-not-exist".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_review_verdict_narrows_below_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = input
        .items
        .iter_mut()
        .find(|i| i.item_kind == DocsReviewItemKind::StaleExampleReview)
        .expect("stale-example review item present");
    item.review.finding_class = ReviewFindingClass::StaleExampleDrifted;
    item.review.severity = DocsReviewFindingSeverity::Narrowing;
    item.chips.freshness = DocsReviewFreshness::Stale;
    // A drifted example is no longer a confident live match.
    item.chips.confidence = DocsReviewConfidence::Medium;
    item.suggestion.apply_posture = SuggestionApplyPosture::PreviewRequired;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.finding_class = ReviewFindingClass::StaleExampleDrifted;
            row.review_severity = DocsReviewFindingSeverity::Narrowing;
            row.confidence = DocsReviewConfidence::Medium;
            row.apply_posture = SuggestionApplyPosture::PreviewRequired;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsReviewPromotionState::NarrowedBelowStable,
        "findings: {:?}",
        packet.validation_findings
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_review_verdict_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let item = input
        .items
        .iter_mut()
        .find(|i| i.item_kind == DocsReviewItemKind::StaleLinkReview)
        .expect("stale-link review item present");
    item.review.finding_class = ReviewFindingClass::StaleLinkBroken;
    item.review.severity = DocsReviewFindingSeverity::Blocking;
    item.chips.freshness = DocsReviewFreshness::Stale;
    let id = item.item_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.item_id_ref == id {
            row.finding_class = ReviewFindingClass::StaleLinkBroken;
            row.review_severity = DocsReviewFindingSeverity::Blocking;
        }
    }
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsReviewPromotionState::BlocksStable
    );
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.review_degradations.push(DocsReviewDegradation {
        degradation_class: DocsReviewDegradationClass::ReviewNarrowed,
        severity: DocsReviewFindingSeverity::Narrowing,
        summary: "the review was narrowed to the qualified guide after a scope change".to_owned(),
        item_id_ref: None,
        evidence_ref: None,
    });
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsReviewPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.review_degradations.push(DocsReviewDegradation {
        degradation_class: DocsReviewDegradationClass::QuarantinedSource,
        severity: DocsReviewFindingSeverity::Blocking,
        summary: "a docs source is quarantined and must not present as available".to_owned(),
        item_id_ref: Some("item:authoring_suggestion:retry_backoff_guide_intro".to_owned()),
        evidence_ref: None,
    });
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DocsReviewPromotionState::BlocksStable
    );
}

#[test]
fn degradation_referencing_unknown_item_is_orphan() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.review_degradations[0].item_id_ref = Some("item:does-not-exist".to_owned());
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_apply_posture_drifts() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.consumer_projections[0].preserves_apply_posture = false;
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input
        .consumer_projections
        .retain(|p| p.surface != DocsReviewConsumerSurface::StaleExampleReviewQueue);
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_item_id_is_flagged() {
    let mut input = seeded_stable_docs_authoring_review_input();
    let clone = input.items[0].clone();
    input.items.push(clone);
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::DuplicateItemId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_docs_authoring_review_input();
    input.items[0].detail = "matched on bearer abc123 token in the source".to_owned();
    let packet = DocsAuthoringReviewPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == DocsReviewFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_items_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for item in &packet.items {
        assert!(summary.contains(&item.item_id));
    }
    assert!(summary.contains("Suggestion"));
    assert!(summary.contains("Review"));
    assert!(summary.contains("Escapes"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-10T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsAuthoringReviewSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        DOCS_AUTHORING_REVIEW_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_docs_authoring_review_export()
        .expect("checked docs-authoring-review export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:docs_authoring_review:retry_backoff_guide"
    );
    assert_eq!(
        export.packet.promotion_state,
        DocsReviewPromotionState::Stable
    );
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/stale_example_drift_narrows.json"
            )),
            DocsReviewPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/broken_link_blocks_stable.json"
            )),
            DocsReviewPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/unverified_suggestion_apply_blocks_stable.json"
            )),
            DocsReviewPromotionState::BlocksStable,
        ),
    ] {
        let fixture: DocsAuthoringReviewFixture = serde_json::from_str(raw).expect("fixture parses");
        let packet = DocsAuthoringReviewPacket::materialize(fixture.input);
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
struct DocsAuthoringReviewFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: DocsAuthoringReviewPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
