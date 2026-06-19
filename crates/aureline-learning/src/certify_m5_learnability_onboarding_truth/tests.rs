use super::*;

fn packet() -> LearnabilityCertificationPacket {
    seeded_m5_learnability_certification()
}

#[test]
fn certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_family_is_present() {
    let families = packet().represented_families();
    for family in M5LearningSurfaceFamily::ALL {
        assert!(
            families.contains(&family),
            "missing family: {}",
            family.as_str()
        );
    }
}

#[test]
fn every_dimension_is_certified() {
    let dimensions = packet().represented_dimensions();
    for dimension in LearnabilityEvidenceDimension::ALL {
        assert!(
            dimensions.contains(&dimension),
            "missing dimension: {}",
            dimension.as_str()
        );
    }
}

#[test]
fn missing_family_fails_validation() {
    let mut packet = packet();
    packet
        .rows
        .retain(|row| row.subject.family != M5LearningSurfaceFamily::Companion);
    let violations = packet.validate();
    assert!(violations.contains(&LearnabilityCertificationViolation::RequiredFamilyMissing));
    // Companion was the only mirror-served row.
    assert!(violations.contains(&LearnabilityCertificationViolation::MirrorRowCaseMissing));
}

#[test]
fn missing_dimension_fails_validation() {
    let mut packet = packet();
    // Drop every learning-mode-profile certification so the dimension is unrepresented.
    for row in &mut packet.rows {
        row.certifications
            .retain(|c| c.dimension != LearnabilityEvidenceDimension::LearningModeProfile);
    }
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::DimensionCoverageMissing));
}

#[test]
fn auto_narrow_case_is_present() {
    assert_eq!(packet().narrowed_row_count(), 1);
}

#[test]
fn missing_narrowed_case_fails_validation() {
    let mut packet = packet();
    let narrowed = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "learn-cert:profiler_trace:stale-offline-mirror:0001")
        .expect("narrowed row");
    // Re-back the stale offline-mirror proof so no row demonstrates auto-narrowing.
    for c in &mut narrowed.certifications {
        if c.dimension == LearnabilityEvidenceDimension::OfflineMirror {
            c.proof_currency = LearnabilityProofCurrency::VerifiedCurrent;
        }
    }
    narrowed.effective_grade = narrowed.claimed_grade;
    narrowed.narrow_trigger = None;
    narrowed.narrowed_label = None;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::NarrowedRowCaseMissing));
}

#[test]
fn claimed_row_losing_current_proof_must_narrow() {
    let mut packet = packet();
    let notebook = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "learn-cert:notebook:0001")
        .expect("notebook row");
    for c in &mut notebook.certifications {
        if c.dimension == LearnabilityEvidenceDimension::EducationalAi {
            c.proof_currency = LearnabilityProofCurrency::StaleExpired;
        }
    }
    assert!(notebook.needs_narrow());
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::RowNotNarrowedOnUncurrentProof));
}

#[test]
fn missing_core_dimension_forces_narrow() {
    let mut packet = packet();
    let notebook = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "learn-cert:notebook:0001")
        .expect("notebook row");
    notebook
        .certifications
        .retain(|c| c.dimension != LearnabilityEvidenceDimension::GuidedExercise);
    assert!(notebook.needs_narrow());
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::RowNotNarrowedOnUncurrentProof));
}

#[test]
fn mirror_proof_on_live_row_narrows() {
    let mut packet = packet();
    let preview = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "learn-cert:preview:0001")
        .expect("preview row");
    // Mirror proof can never back a live row's claim.
    for c in &mut preview.certifications {
        if c.dimension == LearnabilityEvidenceDimension::GuidedTour {
            c.proof_currency = LearnabilityProofCurrency::MirrorCurrent;
        }
    }
    assert!(preview.needs_narrow());
}

#[test]
fn mirror_row_marker_mismatch_fails() {
    let mut packet = packet();
    let companion = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "learn-cert:companion:0001")
        .expect("companion row");
    // Drop the subject mirror flag while keeping the row mirror_served flag.
    companion.subject.mirror_served = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::MirrorReadsAsLive));
}

#[test]
fn offline_mirror_continuity_undisclosed_fails() {
    let mut packet = packet();
    packet.rows[0].offline_mirror_continuity_disclosed = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::OfflineMirrorReadsAsLive));
}

#[test]
fn generic_narrowed_label_fails() {
    let mut packet = packet();
    let narrowed = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "learn-cert:profiler_trace:stale-offline-mirror:0001")
        .expect("narrowed row");
    narrowed.narrowed_label = Some("uncertified".to_owned());
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::NarrowedRowMissingLabelOrTrigger));
}

#[test]
fn fingerprint_substituting_identity_fails() {
    let mut packet = packet();
    packet.rows[0].subject.subject_fingerprint_token = packet.rows[0].subject.subject_id.clone();
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn tour_step_not_command_backed_fails() {
    let mut packet = packet();
    packet.rows[0].tour_steps_command_backed = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::TourStepNotCommandBacked));
}

#[test]
fn progress_widened_to_collaborators_fails() {
    let mut packet = packet();
    packet.rows[1].progress_private_to_user = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::ProgressNotUserOwned));
}

#[test]
fn uncited_educational_ai_fails() {
    let mut packet = packet();
    packet.rows[2].educational_ai_cites_repository_truth = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::EducationalAiUncited));
}

#[test]
fn explain_do_conflation_fails() {
    let mut packet = packet();
    packet.rows[3].explain_separate_from_do = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::ExplainDoConflated));
}

#[test]
fn trapped_expert_fails() {
    let mut packet = packet();
    packet.rows[4].experts_not_trapped_in_tutorials = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::ExpertTrappedInTutorial));
}

#[test]
fn dimension_proof_without_fingerprint_fails() {
    let mut packet = packet();
    // A present proof ref with a fingerprint equal to the ref is not reopenable.
    let cert = &mut packet.rows[0].certifications[0];
    cert.proof_fingerprint_token = cert.proof_ref.clone();
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::DimensionProofNotReopenable));
}

#[test]
fn missing_proof_with_ref_fails() {
    let mut packet = packet();
    let cert = &mut packet.rows[0].certifications[0];
    cert.proof_currency = LearnabilityProofCurrency::MissingProof;
    // A missing proof must carry no ref; keeping one is malformed.
    assert!(!cert.is_well_formed());
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::DimensionProofNotReopenable));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.rows[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != LEARNABILITY_CERT_DOC_REF);
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.rows_auto_narrow_without_current_proof = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .start_center_ingests_certification = false;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn incomplete_evidence_freshness_fails() {
    let mut packet = packet();
    packet.evidence_freshness.evidence_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::EvidenceFreshnessIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&LearnabilityCertificationViolation::WrongRecordKind));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: LearnabilityCertificationPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Learnability Certification"));
    assert!(summary.contains("companion"));
    assert!(summary.contains("Narrowed:"));
}

#[test]
fn waiver_log_names_narrowed_rows() {
    let log = packet().render_waiver_and_downgrade_log();
    assert!(log.contains("Waiver and Downgrade Log"));
    assert!(log.contains("No manual waivers"));
    assert!(log.contains("learn-cert:profiler_trace:stale-offline-mirror:0001"));
    assert!(log.contains("offline_mirror_continuity_lost"));
    assert!(log.contains("offline_mirror"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_learnability_certification_export()
        .expect("checked learnability certification export validates");
    assert_eq!(checked, seeded_m5_learnability_certification());
}
