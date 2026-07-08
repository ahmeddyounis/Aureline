use super::*;

fn loading_job_row() -> M5DegradedStateResolutionInput {
    M5DegradedStateResolutionInput {
        block_kind: M5DegradedStateBlockKind::JobRow,
        degraded_state: M5SharedComponentStateClass::Loading,
        severity: M5DegradedStateSeverity::Informational,
        recovery_class: M5RecoveryDisclosureClass::NamesFreshness,
        state_cause: M5StateCauseClass::UnknownCause,
        recovery_available: false,
        retains_partial_capability: true,
        high_contrast_active: false,
        block_identity_ref: "block:activity.index-rebuild-run".to_owned(),
        state_style_ref: "token:state.job_row.loading".to_owned(),
        submission_lineage_ref: String::new(),
        disclosure_ref: String::new(),
    }
}

fn pending_form() -> M5DegradedStateResolutionInput {
    M5DegradedStateResolutionInput {
        block_kind: M5DegradedStateBlockKind::Form,
        degraded_state: M5SharedComponentStateClass::Pending,
        severity: M5DegradedStateSeverity::Informational,
        recovery_class: M5RecoveryDisclosureClass::NamesRecoveryAction,
        state_cause: M5StateCauseClass::PreconditionCause,
        recovery_available: true,
        retains_partial_capability: true,
        high_contrast_active: false,
        block_identity_ref: "block:settings-form.save-workspace".to_owned(),
        state_style_ref: "token:state.form.pending".to_owned(),
        submission_lineage_ref: "submission:settings-form.save-workspace#req-1".to_owned(),
        disclosure_ref: String::new(),
    }
}

fn error_review_sheet() -> M5DegradedStateResolutionInput {
    M5DegradedStateResolutionInput {
        block_kind: M5DegradedStateBlockKind::ReviewSheet,
        degraded_state: M5SharedComponentStateClass::WarningError,
        severity: M5DegradedStateSeverity::Error,
        recovery_class: M5RecoveryDisclosureClass::NamesRecoveryAction,
        state_cause: M5StateCauseClass::PolicyCause,
        recovery_available: true,
        retains_partial_capability: true,
        high_contrast_active: false,
        block_identity_ref: "block:review-sheet.approve-change".to_owned(),
        state_style_ref: "token:state.review_sheet.error".to_owned(),
        submission_lineage_ref: "submission:review-sheet.approve-change#req-3".to_owned(),
        disclosure_ref: "error:review-sheet.policy-requires-second-reviewer".to_owned(),
    }
}

// ---- degraded-state resolver --------------------------------------------

#[test]
fn loading_state_is_loading_treatment_with_progress_indicator() {
    let resolved =
        resolve_degraded_state_application_contract(&loading_job_row()).expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5DegradedStatePresentation::LoadingTreatment
    );
    assert!(!resolved.explainable);
    assert!(!resolved.user_submitted);
    assert_eq!(
        resolved.required_non_color_cues,
        vec![M5DegradedStateCue::LoadingProgressIndicator]
    );
    assert!(resolved.loading_and_pending_stay_distinct);
    assert!(resolved.warning_and_error_stay_distinct);
    assert!(resolved.error_and_degraded_stay_distinct);
    assert!(resolved.pending_never_masquerades_as_loading);
    assert!(resolved.names_consequence_and_recovery_when_explainable);
    assert!(resolved.preserves_submission_lineage_and_capability);
    assert!(resolved.no_color_only_signaling);
    assert!(resolved.keyboard_and_screen_reader_explainable);
    assert!(resolved.driven_by_shared_state_contract);
}

#[test]
fn pending_state_is_distinct_from_loading_and_attributed_to_user_action() {
    let loading =
        resolve_degraded_state_application_contract(&loading_job_row()).expect("resolves");
    let pending = resolve_degraded_state_application_contract(&pending_form()).expect("resolves");
    assert_ne!(loading.presentation, pending.presentation);
    assert_eq!(
        pending.presentation,
        M5DegradedStatePresentation::PendingTreatment
    );
    assert!(pending.user_submitted);
    assert_eq!(
        pending.required_non_color_cues,
        vec![M5DegradedStateCue::PendingSubmissionAttribution]
    );
    // Pending and loading never share a cue, so a pending action can never masquerade as loading.
    assert!(!pending
        .required_non_color_cues
        .contains(&M5DegradedStateCue::LoadingProgressIndicator));
    assert!(!pending.submission_lineage_ref.is_empty());
}

#[test]
fn warning_and_error_are_distinct_treatments() {
    let warning = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
        severity: M5DegradedStateSeverity::Warning,
        disclosure_ref: "warning:review-sheet.review-recommended".to_owned(),
        ..error_review_sheet()
    })
    .expect("resolves");
    let error =
        resolve_degraded_state_application_contract(&error_review_sheet()).expect("resolves");
    assert_eq!(warning.presentation, error.presentation);
    assert_ne!(warning.severity, error.severity);
    assert!(warning
        .required_non_color_cues
        .contains(&M5DegradedStateCue::WarningConsequenceGlyph));
    assert!(error
        .required_non_color_cues
        .contains(&M5DegradedStateCue::ErrorConsequenceGlyph));
    // A warning and an error never share their consequence glyph.
    assert!(!warning
        .required_non_color_cues
        .contains(&M5DegradedStateCue::ErrorConsequenceGlyph));
    assert!(!error
        .required_non_color_cues
        .contains(&M5DegradedStateCue::WarningConsequenceGlyph));
}

#[test]
fn warning_error_names_owner_block_reason_and_recovery_disclosures() {
    let resolved =
        resolve_degraded_state_application_contract(&error_review_sheet()).expect("resolves");
    assert!(resolved.explainable);
    for trigger in [
        M5StateDisclosureTrigger::StateCauseRequired,
        M5StateDisclosureTrigger::OwnerRequired,
        M5StateDisclosureTrigger::BlockReasonRequired,
        M5StateDisclosureTrigger::RecoveryActionRequired,
        M5StateDisclosureTrigger::SilentStyleOnlyForbidden,
    ] {
        assert!(
            resolved.required_disclosures.contains(&trigger),
            "warning/error state missing disclosure {}",
            trigger.as_str()
        );
    }
    assert!(resolved
        .required_non_color_cues
        .contains(&M5DegradedStateCue::RecoveryAffordance));
}

#[test]
fn degraded_state_names_what_still_works_and_recovery() {
    let resolved = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
        block_kind: M5DegradedStateBlockKind::Banner,
        degraded_state: M5SharedComponentStateClass::Degraded,
        severity: M5DegradedStateSeverity::Reduced,
        recovery_class: M5RecoveryDisclosureClass::NamesFallbackScope,
        state_cause: M5StateCauseClass::ConnectivityCause,
        retains_partial_capability: true,
        block_identity_ref: "block:shell-banner.offline-mode".to_owned(),
        state_style_ref: "token:state.banner.degraded".to_owned(),
        submission_lineage_ref: String::new(),
        disclosure_ref: "degraded:shell-banner.offline-read-only-cache".to_owned(),
        ..error_review_sheet()
    })
    .expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5DegradedStatePresentation::DegradedTreatment
    );
    assert!(resolved.explainable);
    assert!(resolved.retains_partial_capability);
    assert!(resolved
        .required_non_color_cues
        .contains(&M5DegradedStateCue::DegradedReducedCapabilityGlyph));
    assert!(resolved
        .required_disclosures
        .contains(&M5StateDisclosureTrigger::RecoveryActionRequired));
}

#[test]
fn pending_without_submission_lineage_is_rejected() {
    // The acceptance criterion: a pending action must be attributable to the user action that
    // triggered it.
    let err = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
        submission_lineage_ref: "   ".to_owned(),
        ..pending_form()
    });
    assert_eq!(
        err,
        Err(M5DegradedStateResolutionError::PendingWithoutSubmissionLineage)
    );
}

#[test]
fn loading_claiming_submission_lineage_is_rejected() {
    // The acceptance criterion: a pending action never masquerades as generic background loading —
    // and the inverse, background loading never claims a user submission.
    let err = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
        submission_lineage_ref: "submission:activity.index-rebuild#run-9".to_owned(),
        ..loading_job_row()
    });
    assert_eq!(
        err,
        Err(M5DegradedStateResolutionError::LoadingWithSubmissionLineage)
    );
}

#[test]
fn warning_error_without_decided_severity_is_rejected() {
    for severity in [
        M5DegradedStateSeverity::Informational,
        M5DegradedStateSeverity::Reduced,
    ] {
        let err = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
            severity,
            ..error_review_sheet()
        });
        assert_eq!(
            err,
            Err(M5DegradedStateResolutionError::WarningErrorSeverityUnset),
            "severity {} was not rejected for a warning/error state",
            severity.as_str()
        );
    }
}

#[test]
fn mismatched_severity_for_loading_pending_or_degraded_is_rejected() {
    let loading_err =
        resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
            severity: M5DegradedStateSeverity::Error,
            ..loading_job_row()
        });
    assert_eq!(
        loading_err,
        Err(M5DegradedStateResolutionError::SeverityStateMismatch)
    );
    let degraded_err =
        resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
            block_kind: M5DegradedStateBlockKind::Banner,
            degraded_state: M5SharedComponentStateClass::Degraded,
            severity: M5DegradedStateSeverity::Warning,
            disclosure_ref: "degraded:x.y".to_owned(),
            submission_lineage_ref: String::new(),
            ..error_review_sheet()
        });
    assert_eq!(
        degraded_err,
        Err(M5DegradedStateResolutionError::SeverityStateMismatch)
    );
}

#[test]
fn degraded_without_partial_capability_is_rejected() {
    let err = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
        degraded_state: M5SharedComponentStateClass::Degraded,
        severity: M5DegradedStateSeverity::Reduced,
        retains_partial_capability: false,
        disclosure_ref: "degraded:x.y".to_owned(),
        submission_lineage_ref: String::new(),
        ..error_review_sheet()
    });
    assert_eq!(
        err,
        Err(M5DegradedStateResolutionError::DegradedWithoutPartialCapability)
    );
}

#[test]
fn explainable_state_without_disclosure_detail_is_rejected() {
    let err = resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
        disclosure_ref: "   ".to_owned(),
        ..error_review_sheet()
    });
    assert_eq!(
        err,
        Err(M5DegradedStateResolutionError::MissingDisclosureDetail)
    );
}

#[test]
fn resolver_rejects_non_degraded_state() {
    for state in [
        M5SharedComponentStateClass::Default,
        M5SharedComponentStateClass::Hover,
        M5SharedComponentStateClass::FocusVisible,
        M5SharedComponentStateClass::PressedActive,
        M5SharedComponentStateClass::Selected,
        M5SharedComponentStateClass::Current,
        M5SharedComponentStateClass::Disabled,
        M5SharedComponentStateClass::ReadOnly,
        M5SharedComponentStateClass::Locked,
    ] {
        assert_eq!(
            resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
                degraded_state: state,
                severity: M5DegradedStateSeverity::Informational,
                ..loading_job_row()
            }),
            Err(M5DegradedStateResolutionError::NonDegradedState),
            "state {} was not rejected as non-degraded",
            state.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
            block_identity_ref: " ".to_owned(),
            ..loading_job_row()
        }),
        Err(M5DegradedStateResolutionError::EmptyBlockIdentity)
    );
    assert_eq!(
        resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
            state_style_ref: "".to_owned(),
            ..loading_job_row()
        }),
        Err(M5DegradedStateResolutionError::EmptyStateStyleRef)
    );
    assert_eq!(
        resolve_degraded_state_application_contract(&M5DegradedStateResolutionInput {
            disclosure_ref: "error:https://evil.example/x".to_owned(),
            ..error_review_sheet()
        }),
        Err(M5DegradedStateResolutionError::ForbiddenStateMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_degraded_state_contract_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DEGRADED_STATE_CONTRACT_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_block_kind() {
    let packet = seeded_m5_degraded_state_contract_packet();
    let present: std::collections::BTreeSet<_> = packet.rows.iter().map(|r| r.block_kind).collect();
    for block in M5DegradedStateBlockKind::ALL {
        assert!(
            present.contains(&block),
            "missing block kind {}",
            block.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5DegradedStateBlockKind::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_labels() {
    let packet = seeded_m5_degraded_state_contract_packet();
    for row in &packet.rows {
        for part in M5DegradedStateAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5DegradedStateExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for label in M5ComponentStateRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded));
        assert!(!row.state_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_degraded_state_contract_packet();
    let cases: Vec<&M5DegradedStateResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .collect();

    for state in degraded_states() {
        assert!(
            cases.iter().any(|c| c.resolved.degraded_state == state),
            "no example exercises degraded state {}",
            state.as_str()
        );
    }
    for posture in M5DegradedStatePresentation::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.presentation == posture),
            "no example exercises presentation {}",
            posture.as_str()
        );
    }
    for severity in M5DegradedStateSeverity::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.severity == severity),
            "no example exercises severity {}",
            severity.as_str()
        );
    }
    for cue in M5DegradedStateCue::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_non_color_cues.contains(&cue)),
            "no example exercises non-color cue {}",
            cue.as_str()
        );
    }
    for trigger in M5StateDisclosureTrigger::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_disclosures.contains(&trigger)),
            "no example exercises disclosure {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_guarantees() {
    let packet = seeded_m5_degraded_state_contract_packet();
    for row in &packet.rows {
        for case in &row.state_examples {
            assert!(
                case.is_self_consistent(),
                "state case for {} drifted",
                row.block_kind.as_str()
            );
            assert!(
                case.preserves_identity(),
                "state case for {} lost identity",
                row.block_kind.as_str()
            );
            assert!(
                case.preserves_guarantees(),
                "state case for {} lost a guarantee",
                row.block_kind.as_str()
            );
        }
    }
}

#[test]
fn missing_block_kind_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet
        .rows
        .retain(|row| row.block_kind != M5DegradedStateBlockKind::Banner);
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::RequiredBlockMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.vocabulary_set.presentations.pop();
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DegradedStateAnatomyPart::StateCauseCue);
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5DegradedStateExportField::StateCause);
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::MandatoryExportMissing));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0]
        .required_labels
        .retain(|l| *l != M5ComponentStateRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::MandatoryLabelMissing));
}

#[test]
fn accessibility_route_missing_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0]
        .accessibility_routes
        .retain(|r| *r != M5ComponentStateAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0].state_examples[0].resolved.user_submitted = false;
    // The first example is the form's pending case, which is user-submitted; flipping it drifts.
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::ExampleResolutionDrift));
}

#[test]
fn state_example_missing_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[1].state_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::StateExampleMissing));
}

#[test]
fn degraded_state_coverage_unproven_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    for row in &mut packet.rows {
        row.state_examples = vec![M5DegradedStateResolutionCase::resolved(loading_job_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::DegradedStateCoverageUnproven));
}

#[test]
fn presentation_severity_cue_and_disclosure_coverage_unproven_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    // Every example a loading job row → no pending/warning-error/degraded posture, no warning/error/
    // reduced severity, no pending/warning/error/degraded/recovery cue, no cause/owner/block/
    // recovery disclosure.
    for row in &mut packet.rows {
        row.state_examples = vec![M5DegradedStateResolutionCase::resolved(loading_job_row())];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5DegradedStateContractViolation::PresentationCoverageUnproven));
    assert!(violations.contains(&M5DegradedStateContractViolation::SeverityCoverageUnproven));
    assert!(violations.contains(&M5DegradedStateContractViolation::CueCoverageUnproven));
    assert!(violations.contains(&M5DegradedStateContractViolation::DisclosureCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0].presents_pending_as_generic_loading = true;
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::RowInvariantViolated));
}

#[test]
fn stable_block_missing_proof_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::StableBlockMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.governance_review.warning_and_error_never_collapse = false;
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet
        .consumer_projection
        .disclosure_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_degraded_state_contract_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DegradedStateContractViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_block_kind() {
    let summary = seeded_m5_degraded_state_contract_packet().render_markdown_summary();
    for block in M5DegradedStateBlockKind::ALL {
        assert!(
            summary.contains(block.label()),
            "summary missing block {}",
            block.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_block() {
    let csv = seeded_m5_degraded_state_contract_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DegradedStateBlockKind::ALL.len());
    assert!(lines[0].starts_with("block_kind,qualification,owner,"));
    for block in M5DegradedStateBlockKind::ALL {
        assert!(
            csv.contains(block.as_str()),
            "csv missing block {}",
            block.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_degraded_state_contract_export()
        .expect("checked M5 degraded state contract primitive export validates");
    assert_eq!(from_disk.packet_id, M5_DEGRADED_STATE_CONTRACT_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_degraded_state_contract_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_blocks_visible() {
    for packet in [
        seeded_m5_degraded_state_contract_banner_beta_narrowed(),
        seeded_m5_degraded_state_contract_review_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5DegradedStateBlockKind::ALL.len());
    }

    let banner = seeded_m5_degraded_state_contract_banner_beta_narrowed();
    let row = banner
        .rows
        .iter()
        .find(|r| r.block_kind == M5DegradedStateBlockKind::Banner)
        .expect("banner row present");
    assert_eq!(row.qualification, M5ComponentStateQualificationClass::Beta);

    let review = seeded_m5_degraded_state_contract_review_sheet_preview_narrowed();
    let row = review
        .rows
        .iter()
        .find(|r| r.block_kind == M5DegradedStateBlockKind::ReviewSheet)
        .expect("review-sheet row present");
    assert_eq!(
        row.qualification,
        M5ComponentStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let banner: M5DegradedStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-loading-pending-degraded-state-contract-primitive/banner_beta_narrowed.json"
    )))
    .expect("banner fixture parses");
    assert!(banner.validate().is_empty());
    assert_eq!(
        banner,
        seeded_m5_degraded_state_contract_banner_beta_narrowed()
    );

    let review: M5DegradedStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-loading-pending-degraded-state-contract-primitive/review_sheet_preview_narrowed.json"
    )))
    .expect("review-sheet fixture parses");
    assert!(review.validate().is_empty());
    assert_eq!(
        review,
        seeded_m5_degraded_state_contract_review_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_degraded_state_contract_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
