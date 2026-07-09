use super::*;

const PACKET_ID: &str = LEARNING_EDUCATIONAL_AI_CONTINUITY_PACKET_ID;

type Violation = LearningEducationalAiContinuityViolation;

fn packet() -> LearningEducationalAiContinuityPacket {
    seeded_learning_educational_ai_continuity_controls()
}

fn find_state(
    packet: &mut LearningEducationalAiContinuityPacket,
    state: LearningContinuityState,
) -> &mut LearningDegradedComponentRow {
    packet
        .components
        .iter_mut()
        .find(|component| component.continuity_state == state)
        .expect("component with requested state present")
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
        LEARNING_EDUCATIONAL_AI_CONTINUITY_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        LEARNING_EDUCATIONAL_AI_CONTINUITY_SCHEMA_VERSION
    );
}

#[test]
fn continuity_is_derived_not_asserted() {
    use LearningContinuityState as State;
    use LearningNextSafeAction as Next;
    use LearningTrustClass as Trust;

    // Live → live-enriched, live, no explanation, no fallback, source reachable.
    let d = resolve_continuity(State::Live);
    assert_eq!(d.trust_class, Trust::LiveEnriched);
    assert_eq!(d.next_safe_action, Next::ProceedInLearning);
    assert!(d.is_live);
    assert!(!d.needs_continuity_explanation);
    assert!(!d.needs_source_fallback);
    assert!(!d.source_unavailable);

    // Cached → cached-pack, never live, explanation, source fallback reachable.
    let d = resolve_continuity(State::Cached);
    assert_eq!(d.trust_class, Trust::CachedPack);
    assert_eq!(d.next_safe_action, Next::RefreshEnrichment);
    assert!(!d.is_live);
    assert!(d.needs_continuity_explanation);
    assert!(d.needs_source_fallback);

    // Local-only, offline, stale-pack → not live, source fallback reachable.
    for state in [State::LocalOnly, State::Offline, State::StalePack] {
        let d = resolve_continuity(state);
        assert!(!d.is_live);
        assert!(d.needs_source_fallback);
        assert!(!d.source_unavailable);
    }
    assert_eq!(
        resolve_continuity(State::LocalOnly).next_safe_action,
        Next::ContinueLocalOnly
    );
    assert_eq!(
        resolve_continuity(State::Offline).next_safe_action,
        Next::RetryWhenOnline
    );
    assert_eq!(
        resolve_continuity(State::StalePack).next_safe_action,
        Next::UpdateDocsPack
    );

    // Citation-unavailable and not-installed → source unavailable, stops cited-source routing.
    for state in [State::CitationUnavailable, State::NotInstalled] {
        let d = resolve_continuity(state);
        assert!(!d.is_live);
        assert!(!d.needs_source_fallback);
        assert!(d.source_unavailable);
        assert!(d.needs_continuity_explanation);
    }
    assert_eq!(
        resolve_continuity(State::CitationUnavailable).trust_class,
        Trust::UncitedWithheld
    );
    assert_eq!(
        resolve_continuity(State::NotInstalled).next_safe_action,
        Next::InstallToEnable
    );
}

#[test]
fn apply_posture_is_derived_not_asserted() {
    use EducationalApplyDisposition as Disp;
    use EducationalApplyPosture as Posture;

    let a = resolve_apply(Posture::ExplainOnly);
    assert_eq!(a.apply_disposition, Disp::NoMutation);
    assert!(!a.offers_live_mutation);

    let a = resolve_apply(Posture::SandboxedPractice);
    assert_eq!(a.apply_disposition, Disp::SandboxMutationOnly);
    assert!(!a.offers_live_mutation);
    assert!(a.practice_is_sandboxed);

    // The only posture that offers a live mutation always requires the preview / approval path.
    let a = resolve_apply(Posture::PreviewThenApprove);
    assert_eq!(a.apply_disposition, Disp::PreviewApprovalRequired);
    assert!(a.offers_live_mutation);
    assert!(a.requires_preview_approval);

    let a = resolve_apply(Posture::ApplyBlocked);
    assert_eq!(a.apply_disposition, Disp::MutationUnavailable);
    assert!(!a.offers_live_mutation);
}

#[test]
fn continuity_state_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .components
        .iter()
        .map(|component| component.continuity_state)
        .collect();
    for state in LearningContinuityState::ALL {
        assert!(
            covered.contains(&state),
            "missing continuity state {state:?}"
        );
    }
}

#[test]
fn component_family_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .components
        .iter()
        .map(|component| component.component_family)
        .collect();
    for family in M5LearningComponentFamily::ALL {
        assert!(
            covered.contains(&family),
            "missing component family {family:?}"
        );
    }
}

#[test]
fn apply_posture_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .components
        .iter()
        .map(|component| component.apply_posture)
        .collect();
    for posture in EducationalApplyPosture::ALL {
        assert!(
            covered.contains(&posture),
            "missing apply posture {posture:?}"
        );
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet.validate().contains(&Violation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&Violation::MissingSourceContracts));
}

#[test]
fn empty_components_fails() {
    let mut packet = packet();
    packet.components.clear();
    assert!(packet.validate().contains(&Violation::ComponentsMissing));
}

#[test]
fn cached_component_claiming_live_fails() {
    let mut packet = packet();
    find_state(&mut packet, LearningContinuityState::Cached).claims_live_enrichment = true;
    assert!(packet
        .validate()
        .contains(&Violation::ContinuityStateMisrepresented));
}

#[test]
fn misdeclared_trust_class_fails() {
    let mut packet = packet();
    packet.components[0].trust_class = LearningTrustClass::NotInstalledUnavailable;
    assert!(packet
        .validate()
        .contains(&Violation::ContinuityStateMisrepresented));
}

#[test]
fn misdeclared_next_safe_action_fails() {
    let mut packet = packet();
    packet.components[0].next_safe_action = LearningNextSafeAction::InstallToEnable;
    assert!(packet
        .validate()
        .contains(&Violation::NextSafeActionMisrepresented));
}

#[test]
fn misdeclared_apply_disposition_fails() {
    let mut packet = packet();
    packet.components[0].apply_disposition = EducationalApplyDisposition::PreviewApprovalRequired;
    assert!(packet
        .validate()
        .contains(&Violation::ApplyPostureMisrepresented));
}

#[test]
fn live_mutation_without_preview_approval_fails() {
    let mut packet = packet();
    packet.components[0].mutates_live_without_preview_approval = true;
    assert!(packet
        .validate()
        .contains(&Violation::LiveMutationWithoutPreviewApproval));
}

#[test]
fn missing_apply_boundary_note_fails() {
    let mut packet = packet();
    packet.components[0].apply_boundary_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::ApplyBoundaryNoteMissing));
}

#[test]
fn missing_next_safe_action_note_fails() {
    let mut packet = packet();
    packet.components[0].next_safe_action_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::NextSafeActionNoteMissing));
}

#[test]
fn missing_subject_summary_fails() {
    let mut packet = packet();
    packet.components[0].subject_summary_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::SubjectSummaryMissing));
}

#[test]
fn missing_stable_component_ref_fails() {
    let mut packet = packet();
    packet.components[0].stable_component_ref.clear();
    assert!(packet
        .validate()
        .contains(&Violation::StableComponentRefMissing));
}

#[test]
fn reachable_state_missing_cited_source_ref_fails() {
    let mut packet = packet();
    find_state(&mut packet, LearningContinuityState::Offline)
        .cited_source_ref
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::CitedSourceRefMissing));
}

#[test]
fn uncited_state_claiming_source_fails() {
    let mut packet = packet();
    // A citation-unavailable component must not claim a cited source.
    find_state(&mut packet, LearningContinuityState::CitationUnavailable).cited_source_ref =
        "file:crates/aureline-learning/src/lib.rs".to_owned();
    assert!(packet
        .validate()
        .contains(&Violation::UncitedStateClaimsSource));
}

#[test]
fn degraded_component_missing_state_explanation_fails() {
    let mut packet = packet();
    find_state(&mut packet, LearningContinuityState::Offline)
        .state_explanation_note
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::StateExplanationMissing));
}

#[test]
fn reachable_state_missing_source_fallback_fails() {
    let mut packet = packet();
    find_state(&mut packet, LearningContinuityState::Cached)
        .source_fallback_note
        .clear();
    assert!(packet
        .validate()
        .contains(&Violation::SourceFallbackMissing));
}

#[test]
fn reachable_state_without_resolvable_source_fails() {
    let mut packet = packet();
    // A cached component must offer a resolvable cited source — dropping the open-source verb
    // leaves it without a fallback route.
    let component = find_state(&mut packet, LearningContinuityState::Cached);
    component.safe_verbs = vec![LearningSafeVerb::Explain, LearningSafeVerb::Dismiss];
    assert!(packet
        .validate()
        .contains(&Violation::SourceFallbackRouteMissing));
}

#[test]
fn reachable_state_with_no_source_kind_fails() {
    let mut packet = packet();
    // Local-only needs a fallback; a no-source kind makes the offered source unresolvable.
    let component = find_state(&mut packet, LearningContinuityState::LocalOnly);
    component.source_kind = LearningSourceKind::NoSource;
    assert!(packet
        .validate()
        .contains(&Violation::SourceFallbackRouteMissing));
}

#[test]
fn unavailable_source_still_opening_fails() {
    let mut packet = packet();
    // A not-installed component must stop routing; giving it a resolvable source must fail.
    let component = find_state(&mut packet, LearningContinuityState::NotInstalled);
    component.source_kind = LearningSourceKind::FileLocation;
    component.safe_verbs.push(LearningSafeVerb::OpenSource);
    assert!(packet
        .validate()
        .contains(&Violation::UnavailableSourceStillOpens));
}

#[test]
fn missing_safe_explain_verb_fails() {
    let mut packet = packet();
    packet.components[0].safe_verbs = vec![LearningSafeVerb::Dismiss];
    assert!(packet.validate().contains(&Violation::SafeVerbsIncomplete));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.components[0].dispositions.clear();
    assert!(packet.validate().contains(&Violation::DispositionsMissing));
}

#[test]
fn missing_continuity_note_fails() {
    let mut packet = packet();
    packet.components[0].continuity_note.clear();
    assert!(packet
        .validate()
        .contains(&Violation::ContinuityNoteMissing));
}

#[test]
fn missing_source_label_fails() {
    let mut packet = packet();
    packet.components[0].source_label.clear();
    assert!(packet.validate().contains(&Violation::SourceLabelMissing));
}

#[test]
fn missing_scope_label_fails() {
    let mut packet = packet();
    packet.components[0].scope_label.clear();
    assert!(packet.validate().contains(&Violation::ScopeLabelMissing));
}

#[test]
fn component_masking_privacy_fails() {
    let mut packet = packet();
    packet.components[0].masks_privacy_or_offline_state = true;
    assert!(packet
        .validate()
        .contains(&Violation::PrivacyOrOfflineStateMasked));
}

#[test]
fn component_hiding_citation_source_fails() {
    let mut packet = packet();
    packet.components[0].hides_citation_source = true;
    assert!(packet.validate().contains(&Violation::CitationSourceHidden));
}

#[test]
fn component_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.components[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&Violation::AlternateStateLabelInvented));
}

#[test]
fn component_implying_hidden_apply_fails() {
    let mut packet = packet();
    packet.components[0].implies_hidden_apply_or_mutation = true;
    assert!(packet.validate().contains(&Violation::HiddenApplyImplied));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.components[0].required_labels = vec![M5LearningRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&Violation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.components[0].accessibility_routes =
        vec![M5LearningAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&Violation::AccessibilityRouteMissing));
}

#[test]
fn glance_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .glance_review
        .educational_ai_never_mutates_live_without_preview_approval = false;
    assert!(packet
        .validate()
        .contains(&Violation::GlanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .next_safe_action_visible_before_action = false;
    assert!(packet
        .validate()
        .contains(&Violation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&Violation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.components[0].stable_component_ref = "see https://internal.example/obj".to_owned();
    assert!(packet
        .validate()
        .contains(&Violation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_components() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Degraded components"));
    assert!(summary.contains("live_enriched"));
    assert!(summary.contains("uncited_withheld"));
    assert!(summary.contains("not_installed_unavailable"));
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 7 degraded components
    assert_eq!(lines, 1 + 7);
    assert!(csv.contains("learning_degraded_component"));
    assert!(csv.contains("citation_unavailable"));
    assert!(csv.contains("preview_approval_required"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_learning_educational_ai_continuity_export()
        .expect("checked learning educational-AI continuity export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-learning-educational-ai-continuity-controls/citation_unavailable_glossary.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-learning-educational-ai-continuity-controls/not_installed_progress_marker.json"
        )),
    ] {
        let packet: LearningEducationalAiContinuityPacket = serde_json::from_str(raw)
            .expect("fixture parses as learning educational-AI continuity packet");
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
        seeded_learning_educational_ai_continuity_controls_citation_unavailable_glossary(),
        seeded_learning_educational_ai_continuity_controls_not_installed_progress_marker(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
