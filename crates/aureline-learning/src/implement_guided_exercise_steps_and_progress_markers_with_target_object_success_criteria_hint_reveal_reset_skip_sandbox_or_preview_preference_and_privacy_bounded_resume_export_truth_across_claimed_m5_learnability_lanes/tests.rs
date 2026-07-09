use super::*;

const PACKET_ID: &str = GUIDED_EXERCISE_STEP_PROGRESS_MARKER_PACKET_ID;

fn packet() -> GuidedExerciseStepProgressMarkerControlsPacket {
    seeded_guided_exercise_step_progress_marker_controls()
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
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_VERSION
    );
}

#[test]
fn exercise_progress_is_derived_not_asserted() {
    use ExerciseStepProgressClass as Progress;
    use M5ExerciseStepState as State;

    let d = resolve_exercise_progress(State::NotStarted);
    assert_eq!(d.progress_class, Progress::Pending);
    assert!(!d.is_completed);

    let d = resolve_exercise_progress(State::Active);
    assert_eq!(d.progress_class, Progress::InProgress);
    assert!(!d.is_completed);

    // Passed and replayable both count as completed.
    for state in [State::Passed, State::Replayable] {
        let d = resolve_exercise_progress(state);
        assert_eq!(d.progress_class, Progress::Completed);
        assert!(d.is_completed);
    }

    // Failed retryable never counts as passed and needs a retry note.
    let d = resolve_exercise_progress(State::FailedRetryable);
    assert_eq!(d.progress_class, Progress::Retryable);
    assert!(!d.is_completed);
    assert!(d.needs_retry_note);

    // Sandboxed is sandbox practice and needs a sandbox note.
    let d = resolve_exercise_progress(State::Sandboxed);
    assert_eq!(d.progress_class, Progress::SandboxPractice);
    assert!(!d.is_completed);
    assert!(d.needs_sandbox_note);
}

#[test]
fn progress_standing_is_derived_not_asserted() {
    use M5ProgressState as State;
    use ProgressStanding as Standing;

    let d = resolve_progress_standing(State::NotStarted);
    assert_eq!(d.standing_class, Standing::Unstarted);
    assert!(!d.is_complete);

    let d = resolve_progress_standing(State::InProgress);
    assert_eq!(d.standing_class, Standing::Underway);
    assert!(!d.is_complete);

    let d = resolve_progress_standing(State::Completed);
    assert_eq!(d.standing_class, Standing::Complete);
    assert!(d.is_complete);

    // Paused and reset are both interrupted, never complete, and need an interrupted note.
    for state in [State::Paused, State::Reset] {
        let d = resolve_progress_standing(state);
        assert_eq!(d.standing_class, Standing::Interrupted);
        assert!(!d.is_complete);
        assert!(d.needs_interrupted_note);
    }

    // Offline local is offline-cached and needs an offline note.
    let d = resolve_progress_standing(State::OfflineLocal);
    assert_eq!(d.standing_class, Standing::OfflineCached);
    assert!(d.needs_offline_note);
}

#[test]
fn exercise_coverage_is_complete() {
    let packet = packet();
    let classes: std::collections::BTreeSet<_> = packet
        .exercise_steps
        .iter()
        .map(|s| s.progress_disclosure().progress_class)
        .collect();
    for class in ExerciseStepProgressClass::ALL {
        assert!(classes.contains(&class), "missing progress class {class:?}");
    }
    let states: std::collections::BTreeSet<_> =
        packet.exercise_steps.iter().map(|s| s.step_state).collect();
    for state in M5ExerciseStepState::ALL {
        assert!(states.contains(&state), "missing step state {state:?}");
    }
    let modes: std::collections::BTreeSet<_> = packet
        .exercise_steps
        .iter()
        .map(|s| s.validation_mode)
        .collect();
    for mode in M5ExerciseValidationMode::ALL {
        assert!(modes.contains(&mode), "missing validation mode {mode:?}");
    }
}

#[test]
fn progress_coverage_is_complete() {
    let packet = packet();
    let standings: std::collections::BTreeSet<_> = packet
        .progress_markers
        .iter()
        .map(|m| m.standing_disclosure().standing_class)
        .collect();
    for standing in ProgressStanding::ALL {
        assert!(
            standings.contains(&standing),
            "missing standing {standing:?}"
        );
    }
    let states: std::collections::BTreeSet<_> = packet
        .progress_markers
        .iter()
        .map(|m| m.progress_state)
        .collect();
    for state in M5ProgressState::ALL {
        assert!(states.contains(&state), "missing progress state {state:?}");
    }
    let owners: std::collections::BTreeSet<_> = packet
        .progress_markers
        .iter()
        .map(|m| m.ownership)
        .collect();
    for owner in M5ProgressOwnershipClass::ALL {
        assert!(owners.contains(&owner), "missing ownership {owner:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::MissingSourceContracts));
}

#[test]
fn empty_exercise_steps_fails() {
    let mut packet = packet();
    packet.exercise_steps.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ExerciseStepsMissing));
}

#[test]
fn empty_progress_markers_fails() {
    let mut packet = packet();
    packet.progress_markers.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ProgressMarkersMissing));
}

#[test]
fn step_wrong_component_class_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].component = M5LearningComponentFamily::ProgressMarker;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ExerciseStepWrongComponentClass));
}

#[test]
fn marker_wrong_component_class_fails() {
    let mut packet = packet();
    packet.progress_markers[0].component = M5LearningComponentFamily::GuidedExerciseStep;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ProgressMarkerWrongComponentClass));
}

#[test]
fn retryable_step_claiming_completed_fails() {
    let mut packet = packet();
    let step = packet
        .exercise_steps
        .iter_mut()
        .find(|s| s.progress_class == ExerciseStepProgressClass::Retryable)
        .expect("retryable step present");
    step.claims_completed = true;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ExerciseProgressMisrepresented));
}

#[test]
fn interrupted_marker_claiming_complete_fails() {
    let mut packet = packet();
    let marker = packet
        .progress_markers
        .iter_mut()
        .find(|m| m.standing_class == ProgressStanding::Interrupted)
        .expect("interrupted marker present");
    marker.claims_complete = true;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::CompletionMisrepresented));
}

#[test]
fn missing_retry_note_fails() {
    let mut packet = packet();
    let step = packet
        .exercise_steps
        .iter_mut()
        .find(|s| s.progress_class == ExerciseStepProgressClass::Retryable)
        .expect("retryable step present");
    step.retry_note.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::RetryNoteMissing));
}

#[test]
fn missing_interrupted_note_fails() {
    let mut packet = packet();
    let marker = packet
        .progress_markers
        .iter_mut()
        .find(|m| m.standing_class == ProgressStanding::Interrupted)
        .expect("interrupted marker present");
    marker.interrupted_note.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::InterruptedNoteMissing));
}

#[test]
fn missing_success_criteria_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].success_criteria.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::SuccessCriteriaMissing));
}

#[test]
fn missing_target_object_label_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].target_object_label.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::TargetObjectLabelMissing));
}

#[test]
fn mutating_lesson_without_sandbox_or_preview_fails() {
    let mut packet = packet();
    let step = packet
        .exercise_steps
        .iter_mut()
        .find(|s| s.mutates_state)
        .expect("mutating step present");
    step.mutation_preference = LessonMutationPreference::ReadOnlyWalkthrough;
    assert!(packet.validate().contains(
        &GuidedExerciseStepProgressMarkerViolation::MutatingLessonWithoutSandboxOrPreview
    ));
}

#[test]
fn step_missing_reset_action_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].step_actions = vec![ExerciseStepAction::ShowHint];
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ExerciseStepActionsIncomplete));
}

#[test]
fn marker_missing_resume_reset_export_fails() {
    let mut packet = packet();
    packet.progress_markers[0].marker_actions = vec![ProgressMarkerAction::ResumeProgress];
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ProgressMarkerActionsIncomplete));
}

#[test]
fn target_action_without_target_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].target_kind = DeepLinkKind::NoDeepLink;
    packet.exercise_steps[0].target_ref.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::TargetObjectUnresolved));
}

#[test]
fn resolvable_target_without_ref_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].target_ref.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::TargetObjectRefMissing));
}

#[test]
fn resolvable_resume_export_without_ref_fails() {
    let mut packet = packet();
    packet.progress_markers[0].resume_export_ref.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ResumeExportRefMissing));
}

#[test]
fn progress_count_misrepresented_fails() {
    let mut packet = packet();
    // An interrupted (not complete) marker that claims all units done misrepresents progress.
    let marker = packet
        .progress_markers
        .iter_mut()
        .find(|m| m.standing_class == ProgressStanding::Interrupted)
        .expect("interrupted marker present");
    marker.completed_units = marker.total_units;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ProgressCountMisrepresented));
}

#[test]
fn progress_shared_beyond_scope_fails() {
    let mut packet = packet();
    // A local-only marker cannot claim to share progress beyond local scope.
    let marker = packet
        .progress_markers
        .iter_mut()
        .find(|m| m.ownership == M5ProgressOwnershipClass::LocalOnly)
        .expect("local-only marker present");
    marker.shares_beyond_local_scope = true;
    marker.sharing_disclosure_note = "shared".to_owned();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ProgressSharedBeyondScope));
}

#[test]
fn sharing_disclosure_missing_fails() {
    let mut packet = packet();
    let marker = packet
        .progress_markers
        .iter_mut()
        .find(|m| m.shares_beyond_local_scope)
        .expect("sharing marker present");
    marker.sharing_disclosure_note.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::SharingDisclosureMissing));
}

#[test]
fn missing_ownership_note_fails() {
    let mut packet = packet();
    packet.progress_markers[0]
        .ownership_and_privacy_note
        .clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::OwnershipAndPrivacyNoteMissing));
}

#[test]
fn missing_dispositions_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::DispositionsMissing));
}

#[test]
fn step_masking_privacy_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].masks_privacy_or_offline_state = true;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::PrivacyOrOfflineStateMasked));
}

#[test]
fn step_hiding_success_or_target_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].hides_success_criteria_or_target_identity = true;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::SuccessCriteriaOrTargetHidden));
}

#[test]
fn marker_implying_hidden_apply_fails() {
    let mut packet = packet();
    packet.progress_markers[0].implies_hidden_apply_or_mutation = true;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::HiddenApplyOrMutationImplied));
}

#[test]
fn marker_inventing_alternate_state_label_fails() {
    let mut packet = packet();
    packet.progress_markers[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::AlternateStateLabelInvented));
}

#[test]
fn control_trapping_progress_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].traps_progress_without_resume_reset_export = true;
    assert!(packet.validate().contains(
        &GuidedExerciseStepProgressMarkerViolation::ProgressTrappedWithoutResumeResetExport
    ));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].required_labels = vec![M5LearningRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.progress_markers[0].accessibility_routes =
        vec![M5LearningAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::AccessibilityRouteMissing));
}

#[test]
fn learnability_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .learnability_review
        .incomplete_never_shown_as_complete = false;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::LearnabilityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .target_and_success_visible_before_start = false;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.exercise_steps[0].target_ref = "see https://internal.example/target".to_owned();
    assert!(packet
        .validate()
        .contains(&GuidedExerciseStepProgressMarkerViolation::RawMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Guided exercise steps"));
    assert!(summary.contains("## Progress markers"));
    assert!(summary.contains("retryable"));
    assert!(summary.contains("interrupted"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 exercise steps + 6 progress markers
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("guided_exercise_step"));
    assert!(csv.contains("progress_marker"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_guided_exercise_step_progress_marker_export()
        .expect("checked guided exercise step progress marker export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-guided-exercise-step-progress-marker-controls/guided_exercise_step_retryable.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-guided-exercise-step-progress-marker-controls/progress_marker_reset.json"
        )),
    ] {
        let packet: GuidedExerciseStepProgressMarkerControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as guided exercise step progress marker packet");
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
        seeded_guided_exercise_step_progress_marker_controls_guided_exercise_step_retryable(),
        seeded_guided_exercise_step_progress_marker_controls_progress_marker_reset(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
