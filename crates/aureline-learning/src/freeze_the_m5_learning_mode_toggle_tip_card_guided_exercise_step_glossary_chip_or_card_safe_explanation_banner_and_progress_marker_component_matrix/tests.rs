use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_learning_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_LEARNING_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_learning_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5LearningComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5LearningComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_dispositions_and_deployment_lines() {
    let packet = seeded_m5_learning_component_matrix();
    for row in &packet.component_rows {
        for label in M5LearningRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.dispositions.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5LearningAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_learning_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.learning_mode_states.is_empty(),
            family.is_learning_mode_toggle(),
            "learning_mode_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.learning_mode_scopes.is_empty(),
            family.is_learning_mode_toggle(),
            "learning_mode_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.tip_trigger_classes.is_empty(),
            family.is_tip_card(),
            "tip_trigger_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.tip_dismissal_states.is_empty(),
            family.is_tip_card(),
            "tip_dismissal_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.exercise_step_states.is_empty(),
            family.is_guided_exercise_step(),
            "exercise_step_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.exercise_validation_modes.is_empty(),
            family.is_guided_exercise_step(),
            "exercise_validation_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.glossary_source_classes.is_empty(),
            family.is_glossary_chip_or_card(),
            "glossary_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.glossary_citation_states.is_empty(),
            family.is_glossary_chip_or_card(),
            "glossary_citation_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.explanation_boundary_classes.is_empty(),
            family.is_safe_explanation_banner(),
            "explanation_boundary_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.explanation_apply_states.is_empty(),
            family.is_safe_explanation_banner(),
            "explanation_apply_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.progress_ownership_classes.is_empty(),
            family.is_progress_marker(),
            "progress_ownership_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.progress_states.is_empty(),
            family.is_progress_marker(),
            "progress_states presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_learning_component_matrix();
    for disposition in M5LearningDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for state in M5LearningModeState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.learning_mode_states.contains(&state)),
            "no component declares learning mode state {}",
            state.as_str()
        );
    }
    for scope in M5LearningModeScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.learning_mode_scopes.contains(&scope)),
            "no component declares learning mode scope {}",
            scope.as_str()
        );
    }
    for trigger in M5TipTriggerClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tip_trigger_classes.contains(&trigger)),
            "no component declares tip trigger class {}",
            trigger.as_str()
        );
    }
    for state in M5TipDismissalState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tip_dismissal_states.contains(&state)),
            "no component declares tip dismissal state {}",
            state.as_str()
        );
    }
    for state in M5ExerciseStepState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.exercise_step_states.contains(&state)),
            "no component declares exercise step state {}",
            state.as_str()
        );
    }
    for mode in M5ExerciseValidationMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.exercise_validation_modes.contains(&mode)),
            "no component declares exercise validation mode {}",
            mode.as_str()
        );
    }
    for class in M5GlossarySourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.glossary_source_classes.contains(&class)),
            "no component declares glossary source class {}",
            class.as_str()
        );
    }
    for state in M5GlossaryCitationState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.glossary_citation_states.contains(&state)),
            "no component declares glossary citation state {}",
            state.as_str()
        );
    }
    for class in M5ExplanationBoundaryClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.explanation_boundary_classes.contains(&class)),
            "no component declares explanation boundary class {}",
            class.as_str()
        );
    }
    for state in M5ExplanationApplyState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.explanation_apply_states.contains(&state)),
            "no component declares explanation apply state {}",
            state.as_str()
        );
    }
    for class in M5ProgressOwnershipClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.progress_ownership_classes.contains(&class)),
            "no component declares progress ownership class {}",
            class.as_str()
        );
    }
    for state in M5ProgressState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.progress_states.contains(&state)),
            "no component declares progress state {}",
            state.as_str()
        );
    }
}

#[test]
fn ac_disposition_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin one controlled vocabulary; assert the exact tokens.
    let tokens: Vec<&str> = M5LearningDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "learning_on",
            "paused",
            "replayable",
            "sandboxed",
            "cached",
            "local_only",
            "not_installed",
            "no_hidden_apply",
        ]
    );
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5LearningComponentFamily::GuidedExerciseStep);
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5LearningRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn dispositions_missing_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::DispositionsMissing));
}

#[test]
fn learning_mode_toggle_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_learning_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5LearningComponentFamily::LearningModeToggle)
            .expect("learning-mode-toggle row present");
        let expected = if clear == 0 {
            row.learning_mode_states.clear();
            M5LearningComponentMatrixViolation::LearningModeStateMissing
        } else {
            row.learning_mode_scopes.clear();
            M5LearningComponentMatrixViolation::LearningModeScopeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn tip_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_learning_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5LearningComponentFamily::TipCard)
            .expect("tip-card row present");
        let expected = if clear == 0 {
            row.tip_trigger_classes.clear();
            M5LearningComponentMatrixViolation::TipTriggerClassMissing
        } else {
            row.tip_dismissal_states.clear();
            M5LearningComponentMatrixViolation::TipDismissalStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn guided_exercise_step_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_learning_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5LearningComponentFamily::GuidedExerciseStep)
            .expect("guided-exercise-step row present");
        let expected = if clear == 0 {
            row.exercise_step_states.clear();
            M5LearningComponentMatrixViolation::ExerciseStepStateMissing
        } else {
            row.exercise_validation_modes.clear();
            M5LearningComponentMatrixViolation::ExerciseValidationModeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn glossary_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_learning_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5LearningComponentFamily::GlossaryChipOrCard)
            .expect("glossary-chip-or-card row present");
        let expected = if clear == 0 {
            row.glossary_source_classes.clear();
            M5LearningComponentMatrixViolation::GlossarySourceClassMissing
        } else {
            row.glossary_citation_states.clear();
            M5LearningComponentMatrixViolation::GlossaryCitationStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn safe_explanation_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_learning_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5LearningComponentFamily::SafeExplanationBanner)
            .expect("safe-explanation-banner row present");
        let expected = if clear == 0 {
            row.explanation_boundary_classes.clear();
            M5LearningComponentMatrixViolation::ExplanationBoundaryClassMissing
        } else {
            row.explanation_apply_states.clear();
            M5LearningComponentMatrixViolation::ExplanationApplyStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn progress_marker_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_learning_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5LearningComponentFamily::ProgressMarker)
            .expect("progress-marker row present");
        let expected = if clear == 0 {
            row.progress_ownership_classes.clear();
            M5LearningComponentMatrixViolation::ProgressOwnershipClassMissing
        } else {
            row.progress_states.clear();
            M5LearningComponentMatrixViolation::ProgressStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[0].masks_privacy_or_offline_state = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[3].hides_citation_source = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[4].implies_hidden_apply_or_mutation = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5LearningComponentFamily::TipCard)
        .expect("tip-card row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_learning_component_matrix().render_markdown_summary();
    for family in M5LearningComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_learning_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5LearningComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,dispositions,"));
    for family in M5LearningComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_learning_component_matrix_export()
        .expect("checked M5 learning component matrix export validates");
    assert_eq!(packet.packet_id, M5_LEARNING_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_learning_component_matrix_export()
        .expect("checked M5 learning component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_learning_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed(),
        seeded_m5_learning_component_matrix_progress_marker_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5LearningComponentFamily::ALL.len()
        );
    }

    let toggle = seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed();
    let row = toggle
        .component_rows
        .iter()
        .find(|r| r.component_family == M5LearningComponentFamily::LearningModeToggle)
        .expect("learning-mode-toggle row present");
    assert_eq!(row.qualification, M5LearningQualificationClass::Beta);

    let progress = seeded_m5_learning_component_matrix_progress_marker_preview_narrowed();
    let row = progress
        .component_rows
        .iter()
        .find(|r| r.component_family == M5LearningComponentFamily::ProgressMarker)
        .expect("progress-marker row present");
    assert_eq!(row.qualification, M5LearningQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let toggle: M5LearningComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-learning-components/learning_mode_toggle_beta_narrowed.json"
    )))
    .expect("learning-mode-toggle fixture parses");
    assert!(toggle.validate().is_empty());
    assert_eq!(
        toggle,
        seeded_m5_learning_component_matrix_learning_mode_toggle_beta_narrowed()
    );

    let progress: M5LearningComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-learning-components/progress_marker_preview_narrowed.json"
    )))
    .expect("progress-marker fixture parses");
    assert!(progress.validate().is_empty());
    assert_eq!(
        progress,
        seeded_m5_learning_component_matrix_progress_marker_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_learning_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
