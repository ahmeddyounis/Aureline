use super::*;

fn live_assertion_failure() -> M5TriagePanelResolutionInput {
    M5TriagePanelResolutionInput {
        failure_category: M5FailureCategory::AssertionFailure,
        triage_disposition: M5TriageDisposition::ProductBug,
        result_origin: M5TestResultOrigin::LiveLocal,
        classifier_confidence: M5ClassifierConfidence::HighConfidence,
        recent_attempts: vec![
            M5AttemptLineageKind::FirstAttempt,
            M5AttemptLineageKind::RetriedFail,
        ],
        has_assertion_or_diff_summary: true,
        has_env_build_runtime_delta: true,
        assertion_summary_label: "assert eq: expected 200 got 500".to_owned(),
        panel_identity_ref: "triage:explorer::auth-assert".to_owned(),
    }
}

fn blocking_quarantine() -> M5QuarantineReviewResolutionInput {
    M5QuarantineReviewResolutionInput {
        suppression_kind: M5SuppressionKind::Quarantined,
        suppression_scope: M5SuppressionScope::SingleCase,
        ownership: M5QuarantineOwnership::TeamOwned,
        release_impact: M5TestReleaseImpact::BlocksRelease,
        expiry_state: M5QuarantineExpiry::ExpiresScheduled,
        has_linked_artifacts: true,
        reason_label: "flaky auth callback under investigation".to_owned(),
        owner_label: "team: payments-quality".to_owned(),
        sheet_identity_ref: "quarantine:explorer::auth-blocking".to_owned(),
    }
}

fn compatible_card() -> M5EnvironmentCardResolutionInput {
    M5EnvironmentCardResolutionInput {
        target_class: M5TestTargetClass::UnitTest,
        primary_environment_lane: M5TestEnvironmentLane::LocalHost,
        legs: vec![
            M5EnvironmentCompatibilityLeg {
                environment_lane: M5TestEnvironmentLane::LocalHost,
                target_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                runtime_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                toolchain_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                build_compatibility: M5EnvCompatibilityClass::NotApplicable,
                leg_label: "local host".to_owned(),
            },
            M5EnvironmentCompatibilityLeg {
                environment_lane: M5TestEnvironmentLane::Container,
                target_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                runtime_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                toolchain_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                build_compatibility: M5EnvCompatibilityClass::FullyCompatible,
                leg_label: "container".to_owned(),
            },
        ],
        card_identity_ref: "environment:explorer::unit-compatible".to_owned(),
    }
}

// ---- failure-triage-panel resolver --------------------------------------

#[test]
fn triage_posture_is_one_to_one_with_failure_category() {
    let cases = [
        (
            M5FailureCategory::AssertionFailure,
            M5TriagePanelPosture::AssertionEvidencePanel,
        ),
        (
            M5FailureCategory::RuntimeError,
            M5TriagePanelPosture::RuntimeEvidencePanel,
        ),
        (
            M5FailureCategory::Timeout,
            M5TriagePanelPosture::TimeoutEvidencePanel,
        ),
        (
            M5FailureCategory::EnvironmentError,
            M5TriagePanelPosture::EnvironmentEvidencePanel,
        ),
        (
            M5FailureCategory::FlakyUnderReview,
            M5TriagePanelPosture::FlakyReviewPanel,
        ),
        (
            M5FailureCategory::UnknownFailure,
            M5TriagePanelPosture::UnclassifiedEvidencePanel,
        ),
    ];
    let mut postures = std::collections::BTreeSet::new();
    for (category, expected) in cases {
        let resolved = resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
            failure_category: category,
            ..live_assertion_failure()
        })
        .expect("resolves");
        assert_eq!(resolved.triage_posture, expected);
        postures.insert(resolved.triage_posture);
    }
    assert_eq!(postures.len(), M5TriagePanelPosture::ALL.len());
}

#[test]
fn live_local_failure_can_debug_and_open_review_with_evidence() {
    let resolved = resolve_failure_triage_panel(&live_assertion_failure()).expect("resolves");
    assert!(resolved.provides_evidence_context);
    assert!(resolved.can_rerun);
    assert!(resolved.can_debug);
    assert!(resolved.can_open_review);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5TriagePanelAction::RevealTriageEvidence,
            M5TriagePanelAction::RerunExactSelection,
            M5TriagePanelAction::OpenDebugSession,
            M5TriagePanelAction::OpenQuarantineReview,
            M5TriagePanelAction::ExportTriage,
        ]
    );
    assert_eq!(resolved.recent_attempt_count, 2);
}

#[test]
fn imported_failure_cannot_debug_but_still_reviews() {
    let resolved = resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
        result_origin: M5TestResultOrigin::ImportedCi,
        ..live_assertion_failure()
    })
    .expect("resolves");
    assert!(!resolved.can_debug);
    assert!(!resolved
        .available_actions
        .contains(&M5TriagePanelAction::OpenDebugSession));
    // Evidence is present, so open-review is still offered.
    assert!(resolved
        .available_actions
        .contains(&M5TriagePanelAction::OpenQuarantineReview));
}

#[test]
fn triage_panel_always_provides_evidence_context_via_recent_attempts() {
    // Even with no assertion summary and no deltas, the recent attempt sequence is evidence.
    let resolved = resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
        has_assertion_or_diff_summary: false,
        has_env_build_runtime_delta: false,
        recent_attempts: vec![M5AttemptLineageKind::FirstAttempt],
        ..live_assertion_failure()
    })
    .expect("resolves");
    assert!(resolved.provides_evidence_context);
    assert!(resolved.can_open_review);
}

#[test]
fn low_confidence_disposition_is_provisional() {
    let resolved = resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
        classifier_confidence: M5ClassifierConfidence::LowConfidence,
        ..live_assertion_failure()
    })
    .expect("resolves");
    assert!(resolved.disposition_is_provisional);
}

#[test]
fn triage_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
            recent_attempts: vec![],
            ..live_assertion_failure()
        }),
        Err(M5TriagePanelResolutionError::EmptyRecentAttempts)
    );
    assert_eq!(
        resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
            assertion_summary_label: "  ".to_owned(),
            ..live_assertion_failure()
        }),
        Err(M5TriagePanelResolutionError::EmptyAssertionSummary)
    );
    assert_eq!(
        resolve_failure_triage_panel(&M5TriagePanelResolutionInput {
            panel_identity_ref: "triage:https://ci.example/run".to_owned(),
            ..live_assertion_failure()
        }),
        Err(M5TriagePanelResolutionError::ForbiddenTriageMaterial)
    );
}

// ---- quarantine-review-sheet resolver -----------------------------------

#[test]
fn quarantine_posture_is_honesty_first() {
    // Expired wins over everything.
    let resolved = resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
        expiry_state: M5QuarantineExpiry::Expired,
        ownership: M5QuarantineOwnership::Unowned,
        release_impact: M5TestReleaseImpact::HiddenFromRelease,
        ..blocking_quarantine()
    })
    .expect("resolves");
    assert_eq!(
        resolved.review_posture,
        M5QuarantineReviewPosture::ExpiredSuppression
    );
    // Unowned wins over hidden-release when not expired.
    let resolved = resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
        expiry_state: M5QuarantineExpiry::NoExpiry,
        ownership: M5QuarantineOwnership::Unowned,
        release_impact: M5TestReleaseImpact::HiddenFromRelease,
        ..blocking_quarantine()
    })
    .expect("resolves");
    assert_eq!(
        resolved.review_posture,
        M5QuarantineReviewPosture::UnownedSuppression
    );
}

#[test]
fn quarantine_always_stays_visible_preserves_reason_and_restores() {
    for expiry in M5QuarantineExpiry::ALL {
        let resolved = resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
            expiry_state: expiry,
            ..blocking_quarantine()
        })
        .expect("resolves");
        assert!(resolved.stays_visible);
        assert!(resolved.preserves_reason);
        assert!(resolved.can_restore);
        assert!(resolved
            .available_actions
            .contains(&M5QuarantineReviewAction::RestoreTest));
    }
}

#[test]
fn governed_quarantine_needs_no_attention() {
    let resolved = resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
        ownership: M5QuarantineOwnership::SelfOwned,
        release_impact: M5TestReleaseImpact::NoImpact,
        expiry_state: M5QuarantineExpiry::PermanentPolicy,
        ..blocking_quarantine()
    })
    .expect("resolves");
    assert_eq!(
        resolved.review_posture,
        M5QuarantineReviewPosture::GovernedSuppression
    );
    assert!(!resolved.needs_attention);
}

#[test]
fn unowned_and_expired_quarantines_offer_reassign_and_renew() {
    let resolved = resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
        ownership: M5QuarantineOwnership::OwnerExpired,
        expiry_state: M5QuarantineExpiry::Expired,
        has_linked_artifacts: true,
        ..blocking_quarantine()
    })
    .expect("resolves");
    assert!(resolved
        .available_actions
        .contains(&M5QuarantineReviewAction::ReassignOwner));
    assert!(resolved
        .available_actions
        .contains(&M5QuarantineReviewAction::RenewSuppression));
    assert!(resolved
        .available_actions
        .contains(&M5QuarantineReviewAction::OpenLinkedArtifacts));
}

#[test]
fn quarantine_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
            reason_label: "  ".to_owned(),
            ..blocking_quarantine()
        }),
        Err(M5QuarantineReviewResolutionError::EmptyReason)
    );
    assert_eq!(
        resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
            owner_label: "".to_owned(),
            ..blocking_quarantine()
        }),
        Err(M5QuarantineReviewResolutionError::EmptyOwnerLabel)
    );
    assert_eq!(
        resolve_quarantine_review_sheet(&M5QuarantineReviewResolutionInput {
            reason_label: "owner password leaked".to_owned(),
            ..blocking_quarantine()
        }),
        Err(M5QuarantineReviewResolutionError::ForbiddenQuarantineMaterial)
    );
}

// ---- environment-matrix-card resolver -----------------------------------

#[test]
fn compatible_matrix_never_asserts_safe_equivalence() {
    let resolved = resolve_environment_matrix_card(&compatible_card()).expect("resolves");
    assert_eq!(
        resolved.card_posture,
        M5EnvironmentCardPosture::CompatibleMatrix
    );
    assert_eq!(
        resolved.overall_compatibility,
        M5EnvCompatibilityClass::FullyCompatible
    );
    assert!(!resolved.asserts_safe_equivalence);
    assert!(!resolved.has_incompatible_leg);
    assert!(resolved.can_rerun_on_leg);
    assert!(!resolved.warns_on_incompatibility);
}

#[test]
fn incompatible_axis_dominates_and_warns() {
    let mut input = compatible_card();
    input.legs[0].target_compatibility = M5EnvCompatibilityClass::Incompatible;
    let resolved = resolve_environment_matrix_card(&input).expect("resolves");
    assert_eq!(
        resolved.card_posture,
        M5EnvironmentCardPosture::IncompatibleMatrix
    );
    assert_eq!(
        resolved.overall_compatibility,
        M5EnvCompatibilityClass::Incompatible
    );
    assert!(resolved.has_incompatible_leg);
    assert!(resolved.warns_on_incompatibility);
    assert!(!resolved.asserts_safe_equivalence);
}

#[test]
fn unverified_axis_reads_unverified_not_compatible() {
    let mut input = compatible_card();
    input.legs[1].runtime_compatibility = M5EnvCompatibilityClass::Unverified;
    let resolved = resolve_environment_matrix_card(&input).expect("resolves");
    assert_eq!(
        resolved.card_posture,
        M5EnvironmentCardPosture::UnverifiedMatrix
    );
    assert!(resolved.has_unverified_axis);
    assert!(resolved.warns_on_incompatibility);
}

#[test]
fn partial_axis_reads_mixed() {
    let mut input = compatible_card();
    input.legs[0].runtime_compatibility = M5EnvCompatibilityClass::PartiallyCompatible;
    let resolved = resolve_environment_matrix_card(&input).expect("resolves");
    assert_eq!(resolved.card_posture, M5EnvironmentCardPosture::MixedMatrix);
}

#[test]
fn environment_resolver_rejects_malformed_input() {
    let mut single = compatible_card();
    single.legs.truncate(1);
    assert_eq!(
        resolve_environment_matrix_card(&single),
        Err(M5EnvironmentCardResolutionError::InsufficientComparisonLegs)
    );
    let mut empty_label = compatible_card();
    empty_label.legs[0].leg_label = "  ".to_owned();
    assert_eq!(
        resolve_environment_matrix_card(&empty_label),
        Err(M5EnvironmentCardResolutionError::EmptyLegLabel)
    );
    let mut forbidden = compatible_card();
    forbidden.card_identity_ref = "env://ci.example/leg".to_owned();
    assert_eq!(
        resolve_environment_matrix_card(&forbidden),
        Err(M5EnvironmentCardResolutionError::ForbiddenEnvironmentMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_quality_triage_status_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_QUALITY_TRIAGE_STATUS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_quality_triage_status_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5QualityTriageConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5QualityTriageConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_quality_triage_status_packet();
    for row in &packet.rows {
        for part in M5TriagePanelAnatomyPart::MANDATORY {
            assert!(row.triage_anatomy_parts.contains(&part));
        }
        for part in M5QuarantineReviewAnatomyPart::MANDATORY {
            assert!(row.quarantine_anatomy_parts.contains(&part));
        }
        for part in M5EnvironmentCardAnatomyPart::MANDATORY {
            assert!(row.environment_anatomy_parts.contains(&part));
        }
        for field in M5TriagePanelExportField::MANDATORY {
            assert!(row.triage_export_fields.contains(&field));
        }
        for field in M5QuarantineReviewExportField::MANDATORY {
            assert!(row.quarantine_export_fields.contains(&field));
        }
        for field in M5EnvironmentCardExportField::MANDATORY {
            assert!(row.environment_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestAccessibilityRoute::KeyboardFocusable));
        assert!(!row.triage_examples.is_empty());
        assert!(!row.quarantine_examples.is_empty());
        assert!(!row.environment_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_quality_triage_status_packet();
    let triage: Vec<&M5TriagePanelResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.triage_examples.iter())
        .collect();
    let quarantine: Vec<&M5QuarantineReviewResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.quarantine_examples.iter())
        .collect();
    let environment: Vec<&M5EnvironmentCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.environment_examples.iter())
        .collect();

    for posture in M5TriagePanelPosture::ALL {
        assert!(
            triage.iter().any(|c| c.resolved.triage_posture == posture),
            "no example exercises triage posture {}",
            posture.as_str()
        );
    }
    for action in M5TriagePanelAction::ALL {
        assert!(
            triage
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises triage action {}",
            action.as_str()
        );
    }
    for posture in M5QuarantineReviewPosture::ALL {
        assert!(
            quarantine
                .iter()
                .any(|c| c.resolved.review_posture == posture),
            "no example exercises quarantine posture {}",
            posture.as_str()
        );
    }
    for action in M5QuarantineReviewAction::ALL {
        assert!(
            quarantine
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises quarantine action {}",
            action.as_str()
        );
    }
    for posture in M5EnvironmentCardPosture::ALL {
        assert!(
            environment
                .iter()
                .any(|c| c.resolved.card_posture == posture),
            "no example exercises environment posture {}",
            posture.as_str()
        );
    }
    for action in M5EnvironmentCardAction::ALL {
        assert!(
            environment
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises environment action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_quality_triage_status_packet();
    for row in &packet.rows {
        for case in &row.triage_examples {
            assert!(case.is_self_consistent(), "triage case drifted");
            assert!(case.preserves_identity(), "triage case lost identity");
        }
        for case in &row.quarantine_examples {
            assert!(case.is_self_consistent(), "quarantine case drifted");
            assert!(case.preserves_identity(), "quarantine case lost identity");
        }
        for case in &row.environment_examples {
            assert!(case.is_self_consistent(), "environment case drifted");
            assert!(case.preserves_identity(), "environment case lost identity");
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5QualityTriageConsumerSurface::NotebookTriageView);
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.vocabulary_set.quarantine_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::VocabularySetDrift));
}

#[test]
fn mandatory_triage_anatomy_missing_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0]
        .triage_anatomy_parts
        .retain(|p| *p != M5TriagePanelAnatomyPart::RecentAttemptSequenceCue);
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::MandatoryTriageAnatomyMissing));
}

#[test]
fn mandatory_quarantine_anatomy_missing_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0]
        .quarantine_anatomy_parts
        .retain(|p| *p != M5QuarantineReviewAnatomyPart::ReleaseImpactCue);
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::MandatoryQuarantineAnatomyMissing));
}

#[test]
fn mandatory_environment_anatomy_missing_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0]
        .environment_anatomy_parts
        .retain(|p| *p != M5EnvironmentCardAnatomyPart::CompatibilityClassCue);
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::MandatoryEnvironmentAnatomyMissing));
}

#[test]
fn mandatory_environment_export_missing_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0]
        .environment_export_fields
        .retain(|f| *f != M5EnvironmentCardExportField::OverallCompatibility);
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::MandatoryEnvironmentExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0].triage_examples[0].resolved.can_debug = false;
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::ExampleResolutionDrift));
}

#[test]
fn example_missing_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[1].environment_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::ExampleMissing));
}

#[test]
fn triage_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    let only = M5TriagePanelResolutionCase::resolved(live_assertion_failure());
    for row in &mut packet.rows {
        row.triage_examples = vec![only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::TriagePostureCoverageUnproven));
}

#[test]
fn quarantine_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    let only = M5QuarantineReviewResolutionCase::resolved(blocking_quarantine());
    for row in &mut packet.rows {
        row.quarantine_examples = vec![only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::QuarantinePostureCoverageUnproven));
}

#[test]
fn environment_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    let only = M5EnvironmentCardResolutionCase::resolved(compatible_card());
    for row in &mut packet.rows {
        row.environment_examples = vec![only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::EnvironmentPostureCoverageUnproven));
}

#[test]
fn safe_equivalence_coverage_unproven_fails() {
    // Replace every environment example with a compatible one so the incompatible half fires.
    let mut packet = seeded_m5_quality_triage_status_packet();
    let only = M5EnvironmentCardResolutionCase::resolved(compatible_card());
    for row in &mut packet.rows {
        row.environment_examples = vec![only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::SafeEquivalenceCoverageUnproven));
}

#[test]
fn owner_expiry_release_coverage_unproven_fails() {
    // Replace every quarantine example with a governed one (no hidden-from-release), so the
    // hidden-still-visible half fires.
    let mut packet = seeded_m5_quality_triage_status_packet();
    let governed = M5QuarantineReviewResolutionCase::resolved(M5QuarantineReviewResolutionInput {
        ownership: M5QuarantineOwnership::SelfOwned,
        release_impact: M5TestReleaseImpact::NoImpact,
        expiry_state: M5QuarantineExpiry::PermanentPolicy,
        ..blocking_quarantine()
    });
    for row in &mut packet.rows {
        row.quarantine_examples = vec![governed.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::OwnerExpiryReleaseCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0].implies_safe_environment_equivalence = true;
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet
        .governance_review
        .no_suppression_without_evidence_context = false;
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet
        .consumer_projection
        .environment_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_quality_triage_status_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5QualityTriageViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_quality_triage_status_packet().render_markdown_summary();
    for surface in M5QualityTriageConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_quality_triage_status_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5QualityTriageConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5QualityTriageConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_quality_triage_status_export()
        .expect("checked M5 quality triage status export validates");
    assert_eq!(from_disk.packet_id, M5_QUALITY_TRIAGE_STATUS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_quality_triage_status_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_quality_triage_status_notebook_triage_preview_narrowed(),
        seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5QualityTriageConsumerSurface::ALL.len());
    }

    let notebook = seeded_m5_quality_triage_status_notebook_triage_preview_narrowed();
    let row = notebook
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5QualityTriageConsumerSurface::NotebookTriageView)
        .expect("notebook-triage row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Preview);

    let editor = seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed();
    let row = editor
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5QualityTriageConsumerSurface::EditorInlineTriage)
        .expect("editor-inline-triage row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let notebook: M5QualityTriageStatusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-failure-triage-quarantine-environment-primitive/notebook_triage_preview_narrowed.json"
    )))
    .expect("notebook fixture parses");
    assert!(notebook.validate().is_empty());
    assert_eq!(
        notebook,
        seeded_m5_quality_triage_status_notebook_triage_preview_narrowed()
    );

    let editor: M5QualityTriageStatusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-failure-triage-quarantine-environment-primitive/editor_inline_triage_beta_narrowed.json"
    )))
    .expect("editor fixture parses");
    assert!(editor.validate().is_empty());
    assert_eq!(
        editor,
        seeded_m5_quality_triage_status_editor_inline_triage_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_quality_triage_status_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
