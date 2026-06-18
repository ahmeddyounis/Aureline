use super::*;

fn packet() -> DocsSuggestionPanelPacket {
    DocsSuggestionPanelPacket::materialize(seeded_stable_docs_suggestion_panel_input())
}

#[test]
fn seeded_panel_is_clean_stable() {
    let packet = packet();
    assert!(
        packet.is_clean_stable(),
        "expected clean stable, findings: {:?}",
        packet.validation_findings
    );
    assert_eq!(packet.record_kind, DOCS_SUGGESTION_PANEL_RECORD_KIND);
    assert_eq!(packet.schema_version, DOCS_SUGGESTION_PANEL_SCHEMA_VERSION);
}

#[test]
fn panel_covers_required_target_kinds() {
    let kinds: BTreeSet<PanelTargetKind> = packet()
        .suggestions
        .iter()
        .map(|s| s.target.target_kind)
        .collect();
    for required in PanelTargetKind::REQUIRED {
        assert!(
            kinds.contains(&required),
            "missing target kind {required:?}"
        );
    }
}

#[test]
fn every_suggestion_names_target_trigger_proposal_and_actions() {
    for suggestion in packet().suggestions {
        // Concrete target.
        assert!(suggestion.target.names_concrete_target());
        // Concrete trigger source.
        assert!(suggestion.trigger.names_concrete_source());
        assert!(!suggestion.title.trim().is_empty());
        assert!(!suggestion.detail.trim().is_empty());
        assert!(!suggestion.provenance_disclosure_note.trim().is_empty());
        // Diff-first proposal.
        assert!(suggestion.proposal.is_diff_based());
        assert!(!suggestion.proposal.summary.trim().is_empty());
        assert!(!suggestion.proposal.preview_ref.trim().is_empty());
        // Action parity.
        assert!(suggestion.actions.parity_complete());
        // No unverified one-click apply.
        if suggestion.actions.apply_posture.offers_one_click_apply() {
            assert!(suggestion.provenance.is_authoritative());
        }
        // Resolved dispositions stay attributable and reopenable.
        assert!(suggestion.disposition.is_attributable());
        assert!(suggestion.disposition.is_reopenable());
        // Touch each token so it stays stable across refactors.
        let _ = (
            suggestion.target.target_kind.as_str(),
            suggestion.trigger.source.as_str(),
            suggestion.proposal.proposal_kind.as_str(),
            suggestion.actions.apply_posture.as_str(),
            suggestion.provenance.as_str(),
            suggestion.disposition.state.as_str(),
            suggestion.chips.confidence.as_str(),
            suggestion.chips.freshness.as_str(),
            suggestion.chips.version_match.as_str(),
            suggestion.chips.locality.as_str(),
        );
    }
}

#[test]
fn panel_demonstrates_imported_evidence_visibility() {
    // At least one suggestion carries non-authoritative provenance, stays cited,
    // and is not presented as high-confidence live truth.
    let suggestion = packet()
        .suggestions
        .into_iter()
        .find(|s| !s.provenance.is_authoritative())
        .expect("a non-authoritative suggestion is present");
    assert!(suggestion.cited);
    assert!(
        suggestion.chips.confidence != PanelConfidence::High
            || !suggestion.chips.freshness.is_authoritative_live()
    );
    assert!(!suggestion.actions.apply_posture.offers_one_click_apply());
}

#[test]
fn missing_required_target_kind_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let dropped = input
        .suggestions
        .iter()
        .position(|s| s.target.target_kind == PanelTargetKind::Tutorial)
        .expect("tutorial suggestion present");
    let dropped_id = input.suggestions.remove(dropped).suggestion_id;
    input
        .export
        .rows
        .retain(|r| r.suggestion_id_ref != dropped_id);
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert_eq!(packet.promotion_state, PanelPromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::RequiredTargetKindMissing));
}

#[test]
fn missing_target_identity_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].target.file_ref = "  ".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::TargetIdentityMissing));
}

#[test]
fn empty_section_anchor_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].target.section_anchor = Some("   ".to_owned());
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::TargetIdentityMissing));
}

#[test]
fn missing_trigger_detail_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].trigger.detail = "  ".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::TriggerSourceDetailMissing));
}

#[test]
fn missing_trigger_evidence_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].trigger.evidence_ref = "  ".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::TriggerSourceDetailMissing));
}

#[test]
fn missing_title_or_detail_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].detail = "  ".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::TitleOrDetailMissing));
}

#[test]
fn missing_provenance_disclosure_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].provenance_disclosure_note = "  ".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ProvenanceDisclosureMissing));
}

#[test]
fn unverified_evidence_at_high_confidence_live_collapses_provenance_truth() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| s.provenance == PanelEvidenceProvenance::Imported)
        .expect("imported suggestion present");
    suggestion.chips.confidence = PanelConfidence::High;
    suggestion.chips.freshness = PanelFreshness::AuthoritativeLive;
    let id = suggestion.suggestion_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == id {
            row.confidence = PanelConfidence::High;
            row.freshness = PanelFreshness::AuthoritativeLive;
        }
    }
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ProvenanceTruthCollapsed));
}

#[test]
fn uncited_unverified_suggestion_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| s.provenance.needs_citation())
        .expect("unverified suggestion present");
    suggestion.cited = false;
    suggestion.citation_ref = None;
    let id = suggestion.suggestion_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == id {
            row.cited = false;
        }
    }
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::SuggestionNotCited));
}

#[test]
fn drifted_version_presented_as_confident_live_collapses_version_truth() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| {
            s.chips.confidence == PanelConfidence::High
                && s.chips.freshness == PanelFreshness::AuthoritativeLive
        })
        .expect("confident live suggestion present");
    suggestion.chips.version_match = PanelVersionMatch::IncompatibleDriftDetected;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::VersionTruthCollapsed));
}

#[test]
fn prose_only_proposal_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].proposal.proposal_kind = PanelProposalKind::ProseOnlyCard;
    input.suggestions[0].proposal.hunk_count = 0;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert_eq!(packet.promotion_state, PanelPromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ProposalNotDiffBased));
}

#[test]
fn zero_hunk_diff_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].proposal.hunk_count = 0;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ProposalNotDiffBased));
}

#[test]
fn missing_proposal_summary_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].proposal.preview_ref = "  ".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ProposalSummaryMissing));
}

#[test]
fn incomplete_action_parity_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].actions.save_for_later_available = false;
    // Keep the export row consistent so the parity finding is the row-level one.
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == input.suggestions[0].suggestion_id {
            row.action_parity_complete = false;
        }
    }
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ActionParityIncomplete));
}

#[test]
fn missing_open_evidence_ref_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].actions.open_evidence_ref = "  ".to_owned();
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == input.suggestions[0].suggestion_id {
            row.action_parity_complete = false;
        }
    }
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ActionParityIncomplete));
}

#[test]
fn unverified_one_click_apply_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| s.provenance == PanelEvidenceProvenance::Imported)
        .expect("imported suggestion present");
    suggestion.actions.apply_posture = PanelApplyPosture::ApplyAvailable;
    let id = suggestion.suggestion_id.clone();
    for row in input.export.rows.iter_mut() {
        if row.suggestion_id_ref == id {
            row.apply_posture = PanelApplyPosture::ApplyAvailable;
        }
    }
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert_eq!(packet.promotion_state, PanelPromotionState::BlocksStable);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::UnverifiedApplyOffered));
}

#[test]
fn resolved_disposition_without_attribution_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| s.disposition.state == PanelDispositionState::Applied)
        .expect("applied suggestion present");
    suggestion.disposition.attributed_to_ref = None;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::DispositionNotAttributable));
}

#[test]
fn resolved_disposition_not_reopenable_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let suggestion = input
        .suggestions
        .iter_mut()
        .find(|s| s.disposition.state == PanelDispositionState::Applied)
        .expect("applied suggestion present");
    suggestion.disposition.reopenable = false;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::DispositionNotReopenable));
}

#[test]
fn export_dropping_action_parity_preservation_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.preserves_action_parity = false;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportDropsPreservation));
}

#[test]
fn export_apply_posture_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].apply_posture = PanelApplyPosture::ApplyBlockedByPolicy;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportApplyPostureMismatch));
}

#[test]
fn export_target_kind_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].target_kind = PanelTargetKind::Guide;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportTargetKindMismatch));
}

#[test]
fn export_trigger_source_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].trigger_source = PanelTriggerSource::ManualAuthoring;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportTriggerSourceMismatch));
}

#[test]
fn export_provenance_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].provenance = PanelEvidenceProvenance::DerivedHeuristic;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportProvenanceMismatch));
}

#[test]
fn export_disposition_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].disposition_state = PanelDispositionState::Dismissed;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportDispositionMismatch));
}

#[test]
fn export_confidence_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].confidence = PanelConfidence::Low;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportConfidenceMismatch));
}

#[test]
fn export_freshness_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].freshness = PanelFreshness::Stale;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportFreshnessMismatch));
}

#[test]
fn export_cited_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].cited = false;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportCitedMismatch));
}

#[test]
fn export_missing_coverage_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows.pop();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportCoverageMissing));
}

#[test]
fn export_orphan_row_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.export.rows[0].suggestion_id_ref = "suggestion:does-not-exist".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ExportRowOrphan));
}

#[test]
fn narrowing_degradation_narrows_below_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.panel_degradations.push(PanelDegradation {
        degradation_class: PanelDegradationClass::PanelNarrowed,
        severity: PanelFindingSeverity::Narrowing,
        summary: "the panel was narrowed to the qualified release docs after a scope change"
            .to_owned(),
        suggestion_id_ref: None,
        evidence_ref: None,
    });
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        PanelPromotionState::NarrowedBelowStable
    );
    assert!(packet.validation_findings.is_empty());
}

#[test]
fn blocking_degradation_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.panel_degradations.push(PanelDegradation {
        degradation_class: PanelDegradationClass::QuarantinedSource,
        severity: PanelFindingSeverity::Blocking,
        summary: "a docs source is quarantined and must not present as available".to_owned(),
        suggestion_id_ref: Some("suggestion:readme:retry_backoff_api_contract".to_owned()),
        evidence_ref: None,
    });
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert_eq!(packet.promotion_state, PanelPromotionState::BlocksStable);
}

#[test]
fn degradation_referencing_unknown_suggestion_is_orphan() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.panel_degradations[0].suggestion_id_ref = Some("suggestion:does-not-exist".to_owned());
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::DegradationOrphan));
}

#[test]
fn projection_dropping_action_parity_drifts() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.consumer_projections[0].preserves_action_parity = false;
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ConsumerProjectionDrift));
}

#[test]
fn missing_required_surface_blocks_stable() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input
        .consumer_projections
        .retain(|p| p.surface != PanelConsumerSurface::DocsReviewPanel);
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::RequiredSurfaceCoverageMissing));
}

#[test]
fn projection_packet_id_mismatch_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.consumer_projections[0].packet_id_ref = "packet:other".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::ConsumerProjectionPacketIdMismatch));
}

#[test]
fn duplicate_suggestion_id_is_flagged() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    let clone = input.suggestions[0].clone();
    input.suggestions.push(clone);
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::DuplicateSuggestionId));
}

#[test]
fn secrets_in_export_are_blocked() {
    let mut input = seeded_stable_docs_suggestion_panel_input();
    input.suggestions[0].detail = "matched on bearer abc123 token in the source".to_owned();
    let packet = DocsSuggestionPanelPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == PanelFindingKind::RawBoundaryMaterialPresent));
}

#[test]
fn markdown_summary_lists_suggestions_and_degradations() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    for suggestion in &packet.suggestions {
        assert!(summary.contains(&suggestion.suggestion_id));
    }
    assert!(summary.contains("Proposal"));
    assert!(summary.contains("Actions"));
    assert!(summary.contains("Provenance"));
    assert!(summary.contains("Degradations"));
}

#[test]
fn support_export_round_trips() {
    let packet = packet();
    let export = packet.support_export("export:test:001", "2026-06-12T01:00:00Z");
    let json = serde_json::to_string(&export).expect("serializes");
    let parsed: DocsSuggestionPanelSupportExport = serde_json::from_str(&json).expect("parses");
    assert_eq!(parsed, export);
    assert_eq!(
        parsed.record_kind,
        DOCS_SUGGESTION_PANEL_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn checked_support_export_revalidates() {
    let export = current_stable_docs_suggestion_panel_export()
        .expect("checked docs-suggestion-panel export re-validates as clean stable");
    assert_eq!(
        export.packet.packet_id,
        "packet:m5:docs_suggestion_panel:retry_backoff_release"
    );
    assert_eq!(export.packet.promotion_state, PanelPromotionState::Stable);
}

#[test]
fn checked_narrowed_and_blocked_fixtures_match_expected_state() {
    for (raw, expected) in [
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/docs-suggestion-triggers/mirror_offline_narrows.json"
            )),
            PanelPromotionState::NarrowedBelowStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/docs-suggestion-triggers/prose_only_card_blocks_stable.json"
            )),
            PanelPromotionState::BlocksStable,
        ),
        (
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../fixtures/docs/m5/docs-suggestion-triggers/unverified_apply_blocks_stable.json"
            )),
            PanelPromotionState::BlocksStable,
        ),
    ] {
        let fixture: DocsSuggestionPanelFixture =
            serde_json::from_str(raw).expect("fixture parses");
        let packet = DocsSuggestionPanelPacket::materialize(fixture.input);
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
struct DocsSuggestionPanelFixture {
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: DocsSuggestionPanelPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    #[allow(dead_code)]
    promotion_state: String,
    expected_finding_kinds: Vec<String>,
}
