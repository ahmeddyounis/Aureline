use super::*;

const PACKET_ID: &str = GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_PACKET_ID;

fn packet() -> GlossaryChipCardSafeExplanationBannerControlsPacket {
    seeded_glossary_chip_card_safe_explanation_banner_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_VERSION
    );
}

#[test]
fn glossary_citation_is_derived_not_asserted() {
    use GlossaryCitationClass as Class;
    use M5GlossaryCitationState as State;

    // Current and versioned both count as cited-current.
    for state in [State::CitationCurrent, State::CitationVersioned] {
        let d = resolve_glossary_citation(state);
        assert_eq!(d.citation_class, Class::CitedCurrent);
        assert!(d.is_cited_current);
    }

    let d = resolve_glossary_citation(State::CitationStale);
    assert_eq!(d.citation_class, Class::CitedStale);
    assert!(!d.is_cited_current);
    assert!(d.needs_stale_note);

    let d = resolve_glossary_citation(State::CitationCached);
    assert_eq!(d.citation_class, Class::CitedCached);
    assert!(!d.is_cited_current);

    let d = resolve_glossary_citation(State::CitationOfflineUnavailable);
    assert_eq!(d.citation_class, Class::OfflineUnavailable);
    assert!(d.needs_offline_note);

    let d = resolve_glossary_citation(State::CitationMissing);
    assert_eq!(d.citation_class, Class::Uncited);
    assert!(!d.is_cited_current);
    assert!(d.needs_uncited_note);
}

#[test]
fn explanation_apply_is_derived_not_asserted() {
    use ExplanationApplyDisposition as Disp;
    use M5ExplanationApplyState as State;

    let d = resolve_explanation_apply(State::NoApply);
    assert_eq!(d.apply_disposition, Disp::ExplainOnly);
    assert!(d.is_explain_only);

    let d = resolve_explanation_apply(State::PreviewAvailable);
    assert_eq!(d.apply_disposition, Disp::PreviewOffered);
    assert!(!d.is_explain_only);

    let d = resolve_explanation_apply(State::ApprovalPending);
    assert_eq!(d.apply_disposition, Disp::ApprovalPending);

    let d = resolve_explanation_apply(State::AppliedWithUndo);
    assert_eq!(d.apply_disposition, Disp::AppliedReversible);
    assert!(d.needs_undo_note);

    // Blocked and declined both count as withheld and need a withheld note.
    for state in [State::BlockedApply, State::MutationDeclined] {
        let d = resolve_explanation_apply(state);
        assert_eq!(d.apply_disposition, Disp::ApplyWithheld);
        assert!(!d.is_explain_only);
        assert!(d.needs_withheld_note);
    }
}

#[test]
fn glossary_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .glossary_entries
        .iter()
        .map(|e| e.citation_disclosure().citation_class)
        .collect();
    for class in GlossaryCitationClass::ALL {
        assert!(classes.contains(&class), "missing citation class {class:?}");
    }
    let sources: std::collections::BTreeSet<_> = packet
        .glossary_entries
        .iter()
        .map(|e| e.source_class)
        .collect();
    for source in M5GlossarySourceClass::ALL {
        assert!(sources.contains(&source), "missing source class {source:?}");
    }
    let states: std::collections::BTreeSet<_> = packet
        .glossary_entries
        .iter()
        .map(|e| e.citation_state)
        .collect();
    for state in M5GlossaryCitationState::ALL {
        assert!(states.contains(&state), "missing citation state {state:?}");
    }
}

#[test]
fn explanation_coverage_is_complete() {
    let packet = packet();
    let dispositions: std::collections::BTreeSet<_> = packet
        .explanation_banners
        .iter()
        .map(|b| b.apply_disclosure().apply_disposition)
        .collect();
    for disposition in ExplanationApplyDisposition::ALL {
        assert!(
            dispositions.contains(&disposition),
            "missing apply disposition {disposition:?}"
        );
    }
    let boundaries: std::collections::BTreeSet<_> = packet
        .explanation_banners
        .iter()
        .map(|b| b.boundary_class)
        .collect();
    for boundary in M5ExplanationBoundaryClass::ALL {
        assert!(
            boundaries.contains(&boundary),
            "missing boundary class {boundary:?}"
        );
    }
    let states: std::collections::BTreeSet<_> = packet
        .explanation_banners
        .iter()
        .map(|b| b.apply_state)
        .collect();
    for state in M5ExplanationApplyState::ALL {
        assert!(states.contains(&state), "missing apply state {state:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::MissingSourceContracts));
}

#[test]
fn empty_glossary_entries_fails() {
    let mut packet = packet();
    packet.glossary_entries.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryEntriesMissing));
}

#[test]
fn empty_explanation_banners_fails() {
    let mut packet = packet();
    packet.explanation_banners.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannersMissing));
}

#[test]
fn glossary_wrong_component_class_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].component = M5LearningComponentFamily::SafeExplanationBanner;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::GlossaryEntryWrongComponentClass
    ));
}

#[test]
fn banner_wrong_component_class_fails() {
    let mut packet = packet();
    packet.explanation_banners[0].component = M5LearningComponentFamily::GlossaryChipOrCard;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannerWrongComponentClass
    ));
}

#[test]
fn stale_glossary_claiming_current_fails() {
    let mut packet = packet();
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| e.citation_class == GlossaryCitationClass::CitedStale)
        .expect("stale entry present");
    entry.claims_citation_current = true;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationClassMisrepresented
    ));
}

#[test]
fn preview_banner_claiming_explain_only_fails() {
    let mut packet = packet();
    let banner = packet
        .explanation_banners
        .iter_mut()
        .find(|b| b.apply_disposition == ExplanationApplyDisposition::PreviewOffered)
        .expect("preview banner present");
    banner.claims_explain_only = true;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ApplyDispositionMisrepresented));
}

#[test]
fn missing_stale_note_fails() {
    let mut packet = packet();
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| e.citation_class == GlossaryCitationClass::CitedStale)
        .expect("stale entry present");
    entry.stale_note.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryStaleNoteMissing));
}

#[test]
fn missing_undo_note_fails() {
    let mut packet = packet();
    let banner = packet
        .explanation_banners
        .iter_mut()
        .find(|b| b.apply_disposition == ExplanationApplyDisposition::AppliedReversible)
        .expect("applied banner present");
    banner.undo_note.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::UndoNoteMissing));
}

#[test]
fn missing_term_meaning_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].term_meaning.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryTermMeaningMissing));
}

#[test]
fn missing_citation_label_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].citation_label.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationLabelMissing));
}

#[test]
fn source_backed_claim_unsupported_fails() {
    let mut packet = packet();
    // A community / uncited entry cannot claim to rest on cited source truth.
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| !source_is_cited(e.source_class))
        .expect("uncited entry present");
    entry.claims_source_backed = true;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::GlossarySourceBackedClaimUnsupported
    ));
}

#[test]
fn source_backing_note_missing_fails() {
    let mut packet = packet();
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| !source_is_cited(e.source_class))
        .expect("uncited entry present");
    entry.source_backing_note.clear();
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::GlossarySourceBackingNoteMissing
    ));
}

#[test]
fn cited_glossary_missing_open_citation_fails() {
    let mut packet = packet();
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| source_is_cited(e.source_class))
        .expect("cited entry present");
    entry
        .entry_actions
        .retain(|a| *a != GlossaryEntryAction::OpenCitation);
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitedWithoutOpenCitation
    ));
}

#[test]
fn glossary_citation_unresolved_fails() {
    let mut packet = packet();
    // A cited entry offers open-citation but its kind resolves nowhere.
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| e.offers_citation_action())
        .expect("citation-offering entry present");
    entry.citation_kind = DeepLinkKind::NoDeepLink;
    entry.citation_ref.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationUnresolved));
}

#[test]
fn glossary_resolvable_citation_without_ref_fails() {
    let mut packet = packet();
    let entry = packet
        .glossary_entries
        .iter_mut()
        .find(|e| e.citation_kind.is_resolvable())
        .expect("resolvable entry present");
    entry.citation_ref.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryCitationRefMissing));
}

#[test]
fn glossary_missing_related_concept_action_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].entry_actions = vec![GlossaryEntryAction::ShowDefinition];
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::GlossaryActionsIncomplete));
}

#[test]
fn banner_missing_show_explanation_action_fails() {
    let mut packet = packet();
    packet.explanation_banners[0].banner_actions = vec![ExplanationBannerAction::DismissBanner];
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::ExplanationBannerActionsIncomplete
    ));
}

#[test]
fn explain_only_banner_offering_do_action_fails() {
    let mut packet = packet();
    let banner = packet
        .explanation_banners
        .iter_mut()
        .find(|b| b.boundary_class == M5ExplanationBoundaryClass::ExplainOnly)
        .expect("explain-only banner present");
    banner
        .banner_actions
        .push(ExplanationBannerAction::PreviewChange);
    banner.offers_do_action = true;
    banner.do_disclosure_note = "offers do".to_owned();
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::ExplainOnlyBannerOffersDoAction
    ));
}

#[test]
fn apply_state_beyond_boundary_fails() {
    let mut packet = packet();
    // Force an explain-only boundary onto a banner whose apply disposition is not explain-only.
    let banner = packet
        .explanation_banners
        .iter_mut()
        .find(|b| b.apply_disposition == ExplanationApplyDisposition::PreviewOffered)
        .expect("preview banner present");
    banner.boundary_class = M5ExplanationBoundaryClass::ExplainOnly;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ApplyStateBeyondBoundary));
}

#[test]
fn do_disclosure_note_missing_fails() {
    let mut packet = packet();
    let banner = packet
        .explanation_banners
        .iter_mut()
        .find(|b| b.offers_do_action)
        .expect("do-offering banner present");
    banner.do_disclosure_note.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::DoDisclosureNoteMissing));
}

#[test]
fn apply_action_without_do_disclosure_fails() {
    let mut packet = packet();
    let banner = packet
        .explanation_banners
        .iter_mut()
        .find(|b| b.offers_apply_action())
        .expect("apply-action banner present");
    banner.offers_do_action = false;
    banner.do_disclosure_note.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ApplyActionWithoutDoDisclosure));
}

#[test]
fn missing_explain_versus_do_note_fails() {
    let mut packet = packet();
    packet.explanation_banners[0].explain_versus_do_note.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ExplainVersusDoNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::DispositionsMissing));
}

#[test]
fn glossary_masking_privacy_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].masks_privacy_or_offline_state = true;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::PrivacyOrOfflineStateMasked));
}

#[test]
fn glossary_hiding_citation_source_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].hides_citation_source_or_freshness = true;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::CitationSourceOrFreshnessHidden
    ));
}

#[test]
fn banner_implying_apply_capable_action_fails() {
    let mut packet = packet();
    packet.explanation_banners[0].implies_apply_capable_action_or_hidden_authority = true;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::ApplyCapableActionOrHiddenAuthorityImplied
    ));
}

#[test]
fn banner_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.explanation_banners[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::AlternateStateLabelInvented));
}

#[test]
fn control_drifting_prose_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].drifts_prose_from_cited_source_truth = true;
    assert!(packet.validate().contains(
        &GlossaryChipCardSafeExplanationBannerViolation::ProseDriftsFromCitedSourceTruth
    ));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].required_labels = vec![M5LearningRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.explanation_banners[0].accessibility_routes =
        vec![M5LearningAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::AccessibilityRouteMissing));
}

#[test]
fn learnability_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .learnability_review
        .explanation_never_implies_apply_capable_action = false;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::LearnabilityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .explain_versus_do_boundary_visible_before_tap = false;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.glossary_entries[0].citation_ref = "see https://internal.example/term".to_owned();
    assert!(packet
        .validate()
        .contains(&GlossaryChipCardSafeExplanationBannerViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Glossary chips and cards"));
    assert!(summary.contains("## Safe explanation banners"));
    assert!(summary.contains("uncited"));
    assert!(summary.contains("explain_only"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 glossary entries + 6 explanation banners
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("glossary_chip_or_card"));
    assert!(csv.contains("safe_explanation_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_glossary_chip_card_safe_explanation_banner_export()
        .expect("checked glossary chip card safe explanation banner export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-glossary-chip-card-safe-explanation-banner-controls/glossary_chip_card_uncited.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-glossary-chip-card-safe-explanation-banner-controls/safe_explanation_banner_explain_only.json"
        )),
    ] {
        let packet: GlossaryChipCardSafeExplanationBannerControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as glossary chip card safe explanation banner packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_glossary_chip_card_safe_explanation_banner_controls_glossary_chip_card_uncited(),
        seeded_glossary_chip_card_safe_explanation_banner_controls_safe_explanation_banner_explain_only(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
