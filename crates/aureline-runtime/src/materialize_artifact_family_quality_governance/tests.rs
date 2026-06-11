use super::*;

fn packet() -> ArtifactFamilyQualityGovernance {
    current_artifact_family_quality_governance().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        ARTIFACT_FAMILY_QUALITY_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        ARTIFACT_FAMILY_QUALITY_GOVERNANCE_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn every_artifact_family_is_materialized_exactly_once() {
    let packet = packet();
    for family in ArtifactFamilyClass::ALL {
        let count = packet
            .families
            .iter()
            .filter(|f| f.family_class == family)
            .count();
        assert_eq!(count, 1, "family {} not materialized once", family.as_str());
    }
}

#[test]
fn summary_counts_match_materializations() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn export_projection_preserves_family_and_record_refs() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.families.len(), packet.families.len());
    assert_eq!(projection.suppression_refs.len(), packet.suppressions.len());
    assert_eq!(projection.baseline_refs.len(), packet.baselines.len());
    assert!(projection.redaction_safe);
}

#[test]
fn debt_states_keep_suppressed_distinct_from_baselined() {
    let packet = packet();
    // The notebook family suppresses; the request-file family baselines. The
    // two must never collapse into the same governed record class.
    let notebook = packet
        .family(ArtifactFamilyClass::Notebook)
        .expect("notebook");
    let suppressed: Vec<_> = notebook
        .debt_rows_in_state(QualityReleaseDebtStateClass::Suppressed)
        .collect();
    assert_eq!(suppressed.len(), 1);
    assert!(suppressed[0].suppression_ref.is_some());
    assert!(suppressed[0].baseline_ref.is_none());

    let request = packet
        .family(ArtifactFamilyClass::RequestFile)
        .expect("request_file");
    let baselined: Vec<_> = request
        .debt_rows_in_state(QualityReleaseDebtStateClass::Baselined)
        .collect();
    assert_eq!(baselined.len(), 1);
    assert!(baselined[0].baseline_ref.is_some());
    assert!(baselined[0].suppression_ref.is_none());
}

#[test]
fn newly_introduced_debt_carries_no_governed_ref() {
    let packet = packet();
    let scaffold = packet
        .family(ArtifactFamilyClass::ScaffoldedOutput)
        .expect("scaffold");
    let new: Vec<_> = scaffold
        .debt_rows_in_state(QualityReleaseDebtStateClass::New)
        .collect();
    assert_eq!(new.len(), 1);
    assert!(new[0].suppression_ref.is_none());
    assert!(new[0].baseline_ref.is_none());
}

#[test]
fn policy_overrides_are_surfaced_not_masked() {
    let packet = packet();
    let generator = packet
        .family(ArtifactFamilyClass::FrameworkGenerator)
        .expect("generator");
    assert!(generator.profile.has_policy_overrides());
    assert_eq!(
        generator.profile.winning_source_state,
        QualityProfileSourceStateClass::PolicyOverridden
    );
}

#[test]
fn imported_config_unmapped_keys_are_disclosed() {
    let packet = packet();
    let data = packet
        .family(ArtifactFamilyClass::DataAdjacentArtifact)
        .expect("data_adjacent");
    assert!(data.profile.imported_config_mapped);
    assert!(data.profile.has_unmapped_imported_config());
}

#[test]
fn validate_flags_invisible_broad_write() {
    let mut packet = packet();
    // Force the generated-companion participant to auto-apply: an invisible
    // broad write that must be rejected.
    let participant = packet.families[0].save_participants[1].clone();
    assert!(!participant.fix_safety_class.allows_auto_apply());
    packet.families[0].save_participants[1].auto_apply_allowed = true;
    packet.families[0].save_participants[1].preview_first_required = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::InvisibleBroadWrite { .. }
    )));
}

#[test]
fn validate_flags_unsafe_write_without_preview() {
    let mut packet = packet();
    // Scaffold normalization is a whole-file rewrite; dropping its preview gate
    // (without auto-apply) must still flag a silent unsafe write.
    let scaffold = packet
        .families
        .iter_mut()
        .find(|f| f.family_class == ArtifactFamilyClass::ScaffoldedOutput)
        .expect("scaffold");
    scaffold.save_participants[0].preview_first_required = false;
    scaffold.save_participants[0].apply_blocked = false;
    scaffold.save_participants[0].preview_requirement_class =
        QualityPreviewRequirementClass::InlineSummary;
    scaffold.save_participants[0].apply_posture_class = QualityApplyPostureClass::AutoApplyAllowed;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::UnsafeWriteWithoutPreview { .. }
    )));
}

#[test]
fn validate_flags_debt_row_carrying_both_refs() {
    let mut packet = packet();
    packet.families[1].debt_rows[0].suppression_ref =
        Some("suppress:notebook:flaky-rule".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::DebtRowCarriesBothRefs { .. }
    )));
}

#[test]
fn validate_flags_dangling_debt_ref() {
    let mut packet = packet();
    packet.families[0].debt_rows[0].suppression_ref = Some("suppress:does-not-exist".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::DanglingDebtRef { .. }
    )));
}

#[test]
fn validate_flags_hidden_permanent_suppression() {
    let mut packet = packet();
    let suppression = packet
        .suppressions
        .iter_mut()
        .find(|s| s.suppression_id == "suppress:notebook:flaky-rule")
        .expect("suppression");
    suppression.expires_at = None;
    suppression.policy_lock_state_class = QualityPolicyLockStateClass::EditableLocal;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::HiddenPermanentSuppression { .. }
    )));
}

#[test]
fn validate_flags_generated_phase_mismatch() {
    let mut packet = packet();
    // Move the notebook generated-companion participant into the format phase.
    packet.families[0].save_participants[1].phase_class = SaveParticipantPhaseClass::FormatFix;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::GeneratedPhaseMismatch { .. }
    )));
}

#[test]
fn validate_flags_missing_family() {
    let mut packet = packet();
    packet
        .families
        .retain(|f| f.family_class != ArtifactFamilyClass::Notebook);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        ArtifactFamilyQualityGovernanceViolation::MissingFamily { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_save_participants = packet.summary.total_save_participants.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ArtifactFamilyQualityGovernanceViolation::SummaryMismatch));
}

#[test]
fn artifact_family_tokens_are_stable() {
    assert_eq!(ArtifactFamilyClass::Notebook.as_str(), "notebook");
    assert_eq!(ArtifactFamilyClass::RequestFile.as_str(), "request_file");
    assert_eq!(
        ArtifactFamilyClass::ScaffoldedOutput.as_str(),
        "scaffolded_output"
    );
    assert_eq!(
        ArtifactFamilyClass::FrameworkGenerator.as_str(),
        "framework_generator"
    );
    assert_eq!(
        ArtifactFamilyClass::DataAdjacentArtifact.as_str(),
        "data_adjacent_artifact"
    );
}

#[test]
fn save_participants_are_listed_in_execution_order() {
    let packet = packet();
    let notebook = packet
        .family(ArtifactFamilyClass::Notebook)
        .expect("notebook");
    // format_fix must precede generated_artifact_update.
    assert_eq!(
        notebook.save_participants[0].phase_class,
        SaveParticipantPhaseClass::FormatFix
    );
    assert_eq!(
        notebook.save_participants[1].phase_class,
        SaveParticipantPhaseClass::GeneratedArtifactUpdate
    );
}
