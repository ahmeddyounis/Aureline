use super::*;
use crate::quality::{
    QualityApplyPostureClass, QualityRollbackBoundaryClass, QualitySessionOutcomeClass,
    QualitySessionTriggerClass, QualitySurfaceClass,
};

fn loaded() -> QualitySessionLedgerPacket {
    current_m5_quality_session_ledger_export().expect("checked export loads and validates")
}

#[test]
fn checked_export_validates() {
    let packet = loaded();
    assert!(
        packet.validate().is_empty(),
        "checked export must validate: {:?}",
        packet.validate()
    );
}

#[test]
fn checked_export_round_trips() {
    let packet = loaded();
    let json = packet.export_safe_json();
    let parsed: QualitySessionLedgerPacket =
        serde_json::from_str(&json).expect("round-trips through JSON");
    assert_eq!(packet, parsed);
}

#[test]
fn checked_export_covers_required_trigger_paths() {
    let packet = loaded();
    assert!(packet.coverage.covers_required_trigger_paths());
    for required in REQUIRED_TRIGGER_PATHS {
        assert!(
            packet.coverage.trigger_paths.contains(&required),
            "missing trigger path {required:?}"
        );
    }
}

#[test]
fn checked_export_covers_required_action_classes() {
    let packet = loaded();
    assert!(packet.coverage.covers_required_action_classes());
    for required in REQUIRED_ACTION_CLASSES {
        assert!(
            packet.coverage.action_classes.contains(&required),
            "missing action class {required:?}"
        );
    }
}

#[test]
fn checked_export_covers_every_safety_class() {
    let packet = loaded();
    for safety in [
        QualitySafetyClass::TriviaSafe,
        QualitySafetyClass::LocalSyntaxSafe,
        QualitySafetyClass::SemanticLocal,
        QualitySafetyClass::CrossFileSemantic,
        QualitySafetyClass::GeneratedOrProtected,
        QualitySafetyClass::UnknownOrUnstable,
    ] {
        assert!(
            packet.coverage.safety_classes.contains(&safety),
            "missing safety class {safety:?}"
        );
    }
}

#[test]
fn checked_export_spreads_outcomes() {
    let packet = loaded();
    let outcomes = packet.represented_outcomes();
    for outcome in [
        QualitySessionOutcomeClass::Applied,
        QualitySessionOutcomeClass::PreviewRequired,
        QualitySessionOutcomeClass::BlockedByPolicy,
        QualitySessionOutcomeClass::Failed,
    ] {
        assert!(outcomes.contains(&outcome), "missing outcome {outcome:?}");
    }
}

#[test]
fn one_result_vocabulary_across_paths() {
    let packet = loaded();
    for session in &packet.sessions {
        assert_eq!(session.trigger_token, session.trigger_class.as_str());
        assert_eq!(session.outcome_token, session.outcome_class.as_str());
        for proposal in &session.proposals {
            assert_eq!(proposal.action_token, proposal.action_class.as_str());
            assert_eq!(proposal.safety_token, proposal.safety_class.as_str());
            assert_eq!(
                proposal.rollback_boundary_token,
                proposal.rollback_boundary_class.as_str()
            );
        }
    }
}

#[test]
fn on_type_format_auto_applies_as_typed_proposal() {
    let packet = loaded();
    let session = packet
        .sessions
        .iter()
        .find(|s| s.trigger_class == QualitySessionTriggerClass::OnType)
        .expect("on-type session present");
    assert_eq!(session.outcome_class, QualitySessionOutcomeClass::Applied);
    assert!(!session.any_preview_first_required);
    assert!(!session.proposals.is_empty());
    assert!(session.proposals.iter().all(|p| p.is_mutating()));
}

#[test]
fn review_governance_is_blocked_by_policy() {
    let packet = loaded();
    let session = packet
        .sessions
        .iter()
        .find(|s| s.trigger_class == QualitySessionTriggerClass::Review)
        .expect("review session present");
    assert_eq!(
        session.outcome_class,
        QualitySessionOutcomeClass::BlockedByPolicy
    );
    assert!(session.any_apply_blocked);
}

#[test]
fn generated_and_protected_reuse_lifecycle() {
    let packet = loaded();
    for session in &packet.sessions {
        for proposal in &session.proposals {
            if super::touches_generated_or_protected(proposal) {
                assert_ne!(
                    proposal.apply_posture_class,
                    QualityApplyPostureClass::AutoApplyAllowed,
                    "generated/protected proposal must not auto-apply"
                );
                assert!(proposal.preview_first_required || proposal.apply_blocked);
            }
        }
    }
}

#[test]
fn import_comparison_stays_read_only() {
    let packet = loaded();
    let session = packet
        .sessions
        .iter()
        .find(|s| s.trigger_class == QualitySessionTriggerClass::ImportComparison)
        .expect("import-comparison session present");
    assert!(session.proposals.iter().all(|p| !p.is_mutating()));
    assert!(!session.any_apply_blocked);
}

#[test]
fn support_export_preserves_sessions() {
    let packet = loaded();
    assert!(packet.support_export.preserves(&packet.sessions));
    assert!(!packet.support_export.raw_source_content_included);
    assert!(!packet.support_export.raw_payload_included);
}

#[test]
fn every_required_surface_projection_is_present_and_honest() {
    let packet = loaded();
    for session in &packet.sessions {
        for surface in QUALITY_ACTION_EXPOSURE_SURFACES {
            let projection = packet
                .projection_for(&session.session_id, surface)
                .expect("projection present");
            assert!(projection.is_honest(session));
        }
    }
}

#[test]
fn markdown_summary_names_sessions() {
    let packet = loaded();
    let markdown = packet.render_markdown_summary();
    for session in &packet.sessions {
        assert!(markdown.contains(&session.session_id));
    }
}

#[test]
fn wrong_record_kind_is_flagged() {
    let mut packet = loaded();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::WrongRecordKind));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = loaded();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::MissingSourceContracts));
}

#[test]
fn divergent_result_token_is_flagged() {
    let mut packet = loaded();
    packet.sessions[0].outcome_token = "made_up_status".to_owned();
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::ResultVocabularyDivergent));
}

#[test]
fn proposal_token_divergence_is_flagged() {
    let mut packet = loaded();
    packet.sessions[0].proposals[0].action_token = "made_up_action".to_owned();
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::ResultVocabularyDivergent));
}

#[test]
fn generated_weaker_bar_is_flagged() {
    let mut packet = loaded();
    let (s, p) = generated_proposal_index(&packet);
    packet.sessions[s].proposals[p].apply_posture_class =
        QualityApplyPostureClass::AutoApplyAllowed;
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::GeneratedOrProtectedWeakerBar));
}

#[test]
fn dropped_rollback_boundary_is_flagged() {
    let mut packet = loaded();
    let (s, p) = mutating_proposal_index(&packet);
    packet.sessions[s].proposals[p].rollback_boundary_class =
        QualityRollbackBoundaryClass::NoMutation;
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::RollbackNoteMissing));
}

#[test]
fn import_comparison_mutation_is_flagged() {
    let mut packet = loaded();
    let s = packet
        .sessions
        .iter()
        .position(|s| s.trigger_class == QualitySessionTriggerClass::ImportComparison)
        .expect("import-comparison session present");
    packet.sessions[s].proposals[0].action_class = QualityActionClass::QuickFixSingle;
    packet.sessions[s].proposals[0].action_token = "quick_fix_single".to_owned();
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::ImportComparisonMutated));
}

#[test]
fn missing_surface_projection_is_flagged() {
    let mut packet = loaded();
    packet.surface_projections.retain(|projection| {
        !(projection.session_id == packet.sessions[0].session_id
            && projection.surface_class == QualitySurfaceClass::Review)
    });
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::SurfaceProjectionMissing));
}

#[test]
fn projection_dropping_truth_is_flagged() {
    let mut packet = loaded();
    packet.surface_projections[0].exposes_rollback_note = false;
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::SurfaceProjectionDropsTruth));
}

#[test]
fn inconsistent_coverage_is_flagged() {
    let mut packet = loaded();
    packet.coverage.trigger_paths.clear();
    let violations = packet.validate();
    assert!(violations.contains(&QualityActionViolation::CoverageInconsistent));
    assert!(violations.contains(&QualityActionViolation::RequiredTriggerPathMissing));
}

#[test]
fn lossy_support_export_is_flagged() {
    let mut packet = loaded();
    packet.support_export.session_trails.clear();
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::SupportExportLossy));
}

#[test]
fn raw_content_support_export_is_flagged() {
    let mut packet = loaded();
    packet.support_export.raw_payload_included = true;
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::SupportExportIncludesRawContent));
}

#[test]
fn incomplete_guardrails_are_flagged() {
    let mut packet = loaded();
    packet.guardrails.one_result_vocabulary_across_paths = false;
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_is_flagged() {
    let mut packet = loaded();
    packet.consumer_projection.cli_shows_proposal_and_session = false;
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::ConsumerProjectionIncomplete));
}

#[test]
fn inconsistent_proposal_refs_are_flagged() {
    let mut packet = loaded();
    packet.sessions[0].proposal_refs = vec!["mismatched-ref".to_owned()];
    assert!(packet
        .validate()
        .contains(&QualityActionViolation::ProposalRefsInconsistent));
}

fn mutating_proposal_index(packet: &QualitySessionLedgerPacket) -> (usize, usize) {
    for (s, session) in packet.sessions.iter().enumerate() {
        for (p, proposal) in session.proposals.iter().enumerate() {
            if proposal.is_mutating()
                && proposal.rollback_boundary_class != QualityRollbackBoundaryClass::NoMutation
            {
                return (s, p);
            }
        }
    }
    panic!("a mutating proposal with a rollback boundary is present");
}

fn generated_proposal_index(packet: &QualitySessionLedgerPacket) -> (usize, usize) {
    for (s, session) in packet.sessions.iter().enumerate() {
        for (p, proposal) in session.proposals.iter().enumerate() {
            if super::touches_generated_or_protected(proposal) {
                return (s, p);
            }
        }
    }
    panic!("a generated/protected proposal is present");
}
