use super::*;

#[test]
fn seeded_freeze_validates() {
    let freeze = seeded_m5_learnability_lane_freeze();
    validate_m5_learnability_lane(&freeze)
        .expect("seeded M5 learnability-lane freeze must pass validation");
}

#[test]
fn vocabulary_freezes_all_nine_terms() {
    let freeze = seeded_m5_learnability_lane_freeze();
    assert_eq!(freeze.vocabulary.len(), LearnabilityTerm::ALL.len());
    for term in LearnabilityTerm::ALL {
        let entry = freeze
            .vocabulary
            .iter()
            .find(|e| e.term == term)
            .unwrap_or_else(|| panic!("missing vocabulary entry for {}", term.as_str()));
        assert_eq!(entry.token, term.as_str());
        assert!(!entry.authority_boundary_change_allowed);
        assert_eq!(
            entry.data_ownership_class,
            DataOwnershipClass::UserOwnedLocalFirst
        );
        assert!(entry.command_backed_required);
        assert!(!entry.definition.is_empty());
    }
}

#[test]
fn matrix_covers_every_family_and_term() {
    let freeze = seeded_m5_learnability_lane_freeze();
    assert_eq!(
        freeze.lane_rows.len(),
        M5LearningSurfaceFamily::ALL.len() * LearnabilityTerm::ALL.len()
    );
    for family in M5LearningSurfaceFamily::ALL {
        for term in LearnabilityTerm::ALL {
            assert!(
                freeze.row(family, term).is_some(),
                "missing lane row for {}:{}",
                family.as_str(),
                term.as_str()
            );
        }
    }
}

#[test]
fn cross_cutting_terms_share_one_canonical_lane() {
    let freeze = seeded_m5_learnability_lane_freeze();
    for term in LearnabilityTerm::ALL
        .into_iter()
        .filter(|t| t.is_cross_cutting())
    {
        let refs: std::collections::BTreeSet<&str> = freeze
            .lane_rows
            .iter()
            .filter(|r| r.term == term)
            .map(|r| r.canonical_lane_ref.as_str())
            .collect();
        assert_eq!(
            refs.len(),
            1,
            "cross-cutting term {} must route through one canonical lane",
            term.as_str()
        );
    }
}

#[test]
fn no_row_has_hidden_coachmark_or_private_mutation() {
    let freeze = seeded_m5_learnability_lane_freeze();
    for row in &freeze.lane_rows {
        assert!(
            !row.hidden_feature_local_coachmark,
            "{} hides behind a coachmark",
            row.surface_token
        );
        assert!(
            !row.private_mutation_path,
            "{} uses a private mutation path",
            row.surface_token
        );
        assert!(
            row.command_backed,
            "{} is not command-backed",
            row.surface_token
        );
        assert_ne!(
            row.mutation_path_class,
            MutationPathClass::HiddenDirectMutation
        );
    }
}

#[test]
fn educational_ai_keeps_explain_separate_from_do() {
    let freeze = seeded_m5_learnability_lane_freeze();
    let boundary = &freeze.educational_ai_boundary;
    assert!(boundary.explain_and_do_separate);
    assert!(boundary.do_requires_same_preview_approval);
    assert!(!boundary.can_mutate_live_state_directly);
    // Every educational-AI row stays preview/approval-gated, never read-write
    // direct.
    for family in M5LearningSurfaceFamily::ALL {
        let row = freeze
            .row(family, LearnabilityTerm::EducationalAi)
            .expect("educational_ai row");
        assert_eq!(
            row.explain_apply_class,
            ExplainApplyClass::ApplyRequiresApproval
        );
        assert_eq!(
            row.mutation_path_class,
            MutationPathClass::PreviewApprovalRequired
        );
    }
}

#[test]
fn preview_pack_backed_rows_narrow_for_missing_mirror_parity() {
    let freeze = seeded_m5_learnability_lane_freeze();
    let tour = freeze
        .row(
            M5LearningSurfaceFamily::Preview,
            LearnabilityTerm::TourPackage,
        )
        .expect("preview tour row");
    assert!(!tour.mirror_parity.available_on_mirror);
    assert_eq!(tour.verdict, QualificationVerdict::NarrowedBeta);
    assert!(tour
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("mirror_parity")));
}

#[test]
fn notebook_rows_qualify_stable() {
    let freeze = seeded_m5_learnability_lane_freeze();
    for term in LearnabilityTerm::ALL {
        let row = freeze
            .row(M5LearningSurfaceFamily::Notebook, term)
            .expect("notebook row");
        assert_eq!(
            row.verdict,
            QualificationVerdict::QualifiedStable,
            "notebook:{} should qualify stable",
            term.as_str()
        );
    }
}

#[test]
fn overall_verdict_reflects_narrowed_preview_rows() {
    let freeze = seeded_m5_learnability_lane_freeze();
    assert_eq!(freeze.overall_verdict, QualificationVerdict::NarrowedBeta);
    assert!(!freeze.overall_narrowing_reasons.is_empty());
}

#[test]
fn progress_and_digest_are_user_owned_local_first() {
    let freeze = seeded_m5_learnability_lane_freeze();
    for family in M5LearningSurfaceFamily::ALL {
        for term in [
            LearnabilityTerm::ProgressSnapshot,
            LearnabilityTerm::LearningDigest,
        ] {
            let row = freeze.row(family, term).expect("row");
            assert_eq!(
                row.data_ownership_class,
                DataOwnershipClass::UserOwnedLocalFirst
            );
            assert!(row.support_export_parity.qualifies_stable());
        }
    }
}

#[test]
fn validation_catches_hidden_coachmark() {
    let mut freeze = seeded_m5_learnability_lane_freeze();
    freeze.lane_rows[0].hidden_feature_local_coachmark = true;
    freeze.lane_rows[0].sync_verdict();
    let result = validate_m5_learnability_lane(&freeze);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("coachmark")));
}

#[test]
fn validation_catches_private_mutation_path() {
    let mut freeze = seeded_m5_learnability_lane_freeze();
    freeze.lane_rows[1].private_mutation_path = true;
    freeze.lane_rows[1].sync_verdict();
    let result = validate_m5_learnability_lane(&freeze);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("mutation path")));
}

#[test]
fn validation_catches_educational_ai_live_mutation() {
    let mut freeze = seeded_m5_learnability_lane_freeze();
    freeze
        .educational_ai_boundary
        .can_mutate_live_state_directly = true;
    let result = validate_m5_learnability_lane(&freeze);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("mutate live state")));
}

#[test]
fn validation_catches_missing_vocabulary_term() {
    let mut freeze = seeded_m5_learnability_lane_freeze();
    freeze
        .vocabulary
        .retain(|e| e.term != LearnabilityTerm::EducationalAi);
    let result = validate_m5_learnability_lane(&freeze);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("missing")));
}

#[test]
fn validation_catches_forked_cross_cutting_lane() {
    let mut freeze = seeded_m5_learnability_lane_freeze();
    // Fork the learning-mode lane for one family.
    for row in &mut freeze.lane_rows {
        if row.family == M5LearningSurfaceFamily::Companion
            && row.term == LearnabilityTerm::LearningMode
        {
            row.canonical_lane_ref = "learning:m5:companion_local_fork".to_string();
        }
    }
    let result = validate_m5_learnability_lane(&freeze);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("forks")));
}

#[test]
fn validation_catches_telemetry_grade_ownership() {
    let mut freeze = seeded_m5_learnability_lane_freeze();
    freeze.lane_rows[2].data_ownership_class = DataOwnershipClass::TelemetryGradeShared;
    freeze.lane_rows[2].sync_verdict();
    let result = validate_m5_learnability_lane(&freeze);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("local-first")));
}

#[test]
fn freeze_serializes_and_roundtrips() {
    let freeze = seeded_m5_learnability_lane_freeze();
    let json = serde_json::to_string_pretty(&freeze).expect("serialize");
    let back: M5LearnabilityLaneFreeze = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(freeze, back);
}
