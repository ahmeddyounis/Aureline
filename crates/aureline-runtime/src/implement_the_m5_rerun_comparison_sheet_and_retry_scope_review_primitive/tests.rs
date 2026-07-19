//! Tests for the M5 rerun-comparison-sheet primitive: the resolver, the parity matrix,
//! and the checked-in support export.

use super::*;

// --- resolver: AC1 distinct reviewed actions never collapse ---

#[test]
fn resolver_keeps_actions_distinct_when_context_changed() {
    let resolved = resolve_rerun_review(&task_current_context_input()).expect("resolves");
    assert!(resolved.sheet.context_has_changes);
    assert!(!resolved.sheet.modes_semantically_equivalent);
    assert!(resolved
        .sheet
        .available_modes
        .contains(&M5RerunMode::RerunExactly));
    assert!(resolved
        .sheet
        .available_modes
        .contains(&M5RerunMode::RerunWithCurrentContext));
    assert!(resolved.distinguishes_rerun_actions());
}

#[test]
fn resolver_allows_single_action_for_equivalent_exact_replay() {
    let resolved = resolve_rerun_review(&publish_exact_replay_input()).expect("resolves");
    assert!(!resolved.sheet.context_has_changes);
    assert!(resolved.sheet.modes_semantically_equivalent);
    assert_eq!(
        resolved.sheet.available_modes,
        vec![M5RerunMode::RerunExactly]
    );
    assert!(resolved.distinguishes_rerun_actions());
}

#[test]
fn resolver_rejects_collapsed_distinct_actions_when_context_changed() {
    let mut input = task_current_context_input();
    // Collapse the offered actions down to one generic action despite a changed input.
    input.available_modes = vec![M5RerunMode::RerunWithCurrentContext];
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::DistinctRerunActionsCollapsed)
    );
}

#[test]
fn resolver_retry_failed_step_is_distinct_action() {
    let resolved = resolve_rerun_review(&test_retry_failed_step_input()).expect("resolves");
    assert_eq!(resolved.sheet.rerun_mode, M5RerunMode::RetryFailedStepOnly);
    assert_eq!(resolved.sheet.retry_scope, M5RetryScope::FailedStepOnly);
    assert!(resolved
        .sheet
        .available_modes
        .contains(&M5RerunMode::RetryFailedStepOnly));
}

#[test]
fn resolver_rejects_retry_failed_step_for_passed_run() {
    let mut input = test_retry_failed_step_input();
    input.prior_run_outcome = M5RunOutcome::Passed;
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::RetryFailedStepNotApplicable)
    );
}

#[test]
fn resolver_rejects_mode_context_mismatch() {
    let mut input = task_current_context_input();
    input.rerun_context = M5RerunContext::ExactReplay; // but mode is current-context
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::RerunModeContextMismatch)
    );
}

#[test]
fn resolver_rejects_scope_inconsistent_with_mode() {
    let mut input = test_retry_failed_step_input();
    input.retry_scope = M5RetryScope::WholeRun; // retry-failed-step must be failed-step-only
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::RetryScopeInconsistentWithMode)
    );
}

#[test]
fn resolver_rejects_chosen_mode_not_offered() {
    let mut input = task_current_context_input();
    input.available_modes = vec![M5RerunMode::RerunExactly, M5RerunMode::RetryFailedStepOnly];
    // chosen mode is RerunWithCurrentContext, which is no longer offered
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::ChosenModeNotOffered)
    );
}

#[test]
fn resolver_rejects_duplicate_available_mode() {
    let mut input = task_current_context_input();
    input.available_modes = vec![
        M5RerunMode::RerunExactly,
        M5RerunMode::RerunWithCurrentContext,
        M5RerunMode::RerunWithCurrentContext,
    ];
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::DuplicateAvailableMode)
    );
}

// --- resolver: AC2 changed context reviewable before dispatch ---

#[test]
fn resolver_enumerates_changed_dimensions_before_dispatch() {
    let resolved = resolve_rerun_review(&request_authority_changed_input()).expect("resolves");
    assert_eq!(resolved.change_rows.len(), 2);
    for row in &resolved.change_rows {
        assert!(row.shown_before_dispatch);
        assert!(row.requires_review);
        assert!(!row.change_summary.trim().is_empty());
    }
    assert!(resolved.discloses_context_delta_before_dispatch());
    // Authority change is enumerated distinctly.
    assert!(resolved
        .change_rows
        .iter()
        .any(|r| r.dimension == M5RerunChangeDimension::ApprovalAuthority));
}

#[test]
fn resolver_marks_unconfirmed_input_as_unknown() {
    let resolved = resolve_rerun_review(&history_unknown_input()).expect("resolves");
    let row = &resolved.change_rows[0];
    assert_eq!(row.state, M5RerunChangeState::Unknown);
    assert!(row.requires_review);
    assert!(resolved.sheet.context_has_changes);
}

#[test]
fn resolver_rejects_changed_dimension_without_delta() {
    let mut input = request_authority_changed_input();
    input.changed_dimensions[0].after_label = None;
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::ChangedDimensionMissingDelta)
    );
}

#[test]
fn resolver_rejects_change_row_without_detail() {
    let mut input = request_authority_changed_input();
    input.changed_dimensions[0].detail = "  ".to_owned();
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::ChangeRowIncomplete)
    );
}

#[test]
fn resolver_rejects_duplicate_change_dimension() {
    let mut input = request_authority_changed_input();
    input.changed_dimensions[1].dimension = M5RerunChangeDimension::ApprovalAuthority;
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::DuplicateChangeDimension)
    );
}

// --- resolver: side-effect escalation ---

#[test]
fn resolver_discloses_side_effect_escalation() {
    let resolved = resolve_rerun_review(&ai_side_effect_escalates_input()).expect("resolves");
    assert!(resolved.sheet.side_effect_escalates);
    assert!(resolved
        .sheet
        .change_summary
        .contains("side effects escalate"));
    assert!(resolved
        .change_rows
        .iter()
        .any(|r| r.dimension == M5RerunChangeDimension::SideEffectClass && r.requires_review));
}

#[test]
fn resolver_rejects_undisclosed_side_effect_escalation() {
    let mut input = ai_side_effect_escalates_input();
    // Escalate side effects but drop the disclosing change row.
    input.changed_dimensions.clear();
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::SideEffectEscalationNotDisclosed)
    );
}

// --- resolver: AC3 export preserves reviewed mode + summary + lineage ---

#[test]
fn resolver_export_preserves_mode_and_summary() {
    let resolved = resolve_rerun_review(&notebook_environment_changed_input()).expect("resolves");
    assert_eq!(
        resolved.export.rerun_mode,
        M5RerunMode::RerunWithCurrentContext
    );
    assert!(!resolved.export.change_summary.trim().is_empty());
    assert!(resolved
        .export
        .export_fields
        .contains(&M5RerunExportField::RerunMode));
    assert!(declares_mandatory_export_fields(
        &resolved.export.export_fields
    ));
    assert_eq!(
        resolved.export.changed_dimensions,
        vec![
            M5RerunChangeDimension::Runtime,
            M5RerunChangeDimension::Profile
        ]
    );
}

#[test]
fn resolver_preserves_prior_lineage() {
    let resolved = resolve_rerun_review(&test_retry_failed_step_input()).expect("resolves");
    assert!(resolved.sheet.cites_prior_attempt);
    assert_eq!(
        resolved.export.prior_attempt_ref,
        resolved.prior_attempt_ref
    );
    assert!(resolved.export.new_attempt_ordinal > resolved.export.prior_attempt_ordinal);
    assert!(!resolved.export.difference_reason.trim().is_empty());
    assert!(resolved.preserves_prior_lineage());
}

// --- resolver: shared identity + structural rejections ---

#[test]
fn resolver_identity_consistent_across_projections() {
    let resolved = resolve_rerun_review(&task_current_context_input()).expect("resolves");
    assert!(resolved.identity_consistent());
    assert_eq!(resolved.cli_line.sheet_id, resolved.sheet_id);
    assert_eq!(resolved.export.sheet_id, resolved.sheet_id);
    assert_eq!(resolved.sheet.prior_run_ref, resolved.prior_run_ref);
}

#[test]
fn resolver_rejects_collapsed_run_and_attempt_identity() {
    let mut input = task_current_context_input();
    input.prior_attempt_ref = input.prior_run_ref.clone();
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::RunAttemptIdentityCollapsed)
    );
}

#[test]
fn resolver_rejects_new_attempt_not_after_prior() {
    let mut input = task_current_context_input();
    input.new_attempt_ordinal = input.prior_attempt_ordinal;
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::NewAttemptNotAfterPrior)
    );
}

#[test]
fn resolver_rejects_empty_difference_reason() {
    let mut input = task_current_context_input();
    input.difference_reason = "   ".to_owned();
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::EmptyDifferenceReason)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let mut input = task_current_context_input();
    input.context_summary = "see https://example.com/run".to_owned();
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let mut input = history_unknown_input();
    input.degraded = Some(DegradedState {
        trigger: M5ExecutionDowngradeTrigger::RerunContextDrift,
        degraded_label: "degraded".to_owned(),
    });
    assert_eq!(
        resolve_rerun_review(&input),
        Err(M5RerunReviewError::DegradedLabelGeneric)
    );
}

#[test]
fn cli_line_renders_mode_and_change_tokens() {
    let resolved = resolve_rerun_review(&ai_side_effect_escalates_input()).expect("resolves");
    assert!(resolved
        .cli_line
        .line
        .contains("mode=rerun_with_current_context"));
    assert!(resolved.cli_line.line.contains("escalates=true"));
    assert!(resolved.cli_line.line.contains("changed=1"));
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_rerun_review_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_rerun_review_packet();
    let present: BTreeSet<M5RunAttemptSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5RunAttemptSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_matrix_covers_every_rerun_mode() {
    let packet = seeded_m5_rerun_review_packet();
    let seen: BTreeSet<M5RerunMode> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_reruns.iter())
        .map(|case| case.resolved.sheet.rerun_mode)
        .collect();
    for mode in M5RerunMode::ALL {
        assert!(seen.contains(&mode), "missing rerun mode {mode:?}");
    }
}

#[test]
fn seeded_matrix_covers_every_change_dimension() {
    let packet = seeded_m5_rerun_review_packet();
    let seen: BTreeSet<M5RerunChangeDimension> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_reruns.iter())
        .flat_map(|case| case.resolved.change_rows.iter())
        .map(|row| row.dimension)
        .collect();
    for dimension in M5RerunChangeDimension::ALL {
        assert!(seen.contains(&dimension), "missing dimension {dimension:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_rerun_review_packet();
    for row in &packet.surface_rows {
        for case in &row.example_reruns {
            assert!(
                case.is_self_consistent(),
                "case drifted on {:?}",
                row.surface_family
            );
        }
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5RerunVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_rerun_review_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_rerun_review_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5RerunViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_rerun_review_packet();
    packet.surface_rows[0].collapses_distinct_actions = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5RerunViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_rerun_review_packet();
    packet.surface_rows[0].example_reruns[0]
        .resolved
        .sheet
        .rerun_mode = M5RerunMode::RerunExactly;
    let violations = packet.validate();
    assert!(violations.contains(&M5RerunViolation::ExampleRerunDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_rerun_review_packet();
    packet.vocabulary_set.rerun_modes.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5RerunViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_rerun_review_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_rerun_review_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_rerun_review_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_rerun_review_packet();
    assert_eq!(packet.record_kind, M5_RERUN_REVIEW_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_RERUN_REVIEW_SCHEMA_VERSION);
}
