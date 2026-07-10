use super::*;

fn reproduced_flaky_badge() -> M5FlakyBadgeResolutionInput {
    M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::ReproducedFlaky,
        confidence_class: M5FlakyConfidenceClass::HighConfidence,
        classifier_source: M5FlakyClassifierSource::StatisticalModel,
        provenance_class: M5TestIntelligenceProvenanceClass::ReproducedFlaky,
        mute_state: M5FlakyMuteState::NotMuted,
        retry_window_size: 8,
        observed_failures: 5,
        last_outcome: M5RetryAttemptOutcome::PassedOnRetry,
        badge_identity_ref: "flaky-badge:dashboard::reproduced-checkout".to_owned(),
        test_identity_ref: "test:dashboard::checkout-flow".to_owned(),
    }
}

fn divergent_retry_row() -> M5RetryRowResolutionInput {
    M5RetryRowResolutionInput {
        last_outcome: M5RetryAttemptOutcome::PassedOnRetry,
        recent_outcomes: vec![
            M5RetryAttemptOutcome::FailedAllRetries,
            M5RetryAttemptOutcome::PassedOnRetry,
        ],
        scope_class: M5RetryScopeClass::SameSelection,
        attempt_origin: M5RetryAttemptOrigin::LocalAttempt,
        confidence_class: M5FlakyConfidenceClass::ModerateConfidence,
        provenance_class: M5TestIntelligenceProvenanceClass::VerifiedCurrentRun,
        has_env_delta: true,
        has_build_delta: false,
        has_runtime_delta: false,
        test_identity_ref: "test:dashboard::checkout-flow".to_owned(),
        attempt_log_ref: "attempt-log:dashboard::checkout-flow-2".to_owned(),
    }
}

// ---- flaky-state-badge resolver -----------------------------------------

#[test]
fn reproduced_flaky_is_confirmed_with_evidence_window() {
    let resolved = resolve_flaky_state_badge(&reproduced_flaky_badge()).expect("resolves");
    assert_eq!(
        resolved.flaky_posture,
        M5FlakyBadgePosture::ReproducedFlakyBadge
    );
    assert!(resolved.has_sufficient_evidence_window);
    assert!(resolved.claims_reproduced_flaky);
    assert!(resolved.reproduced_claim_supported);
    assert!(resolved.is_confirmed_flaky);
    assert!(!resolved.is_muted_or_quarantined);
    assert_eq!(
        resolved.badge_identity_ref,
        "flaky-badge:dashboard::reproduced-checkout"
    );
}

#[test]
fn every_classification_has_a_distinct_posture() {
    // The acceptance-criterion axis: a suspected verdict never borrows a reproduced posture.
    let cases = [
        (
            M5FlakyClassification::Stable,
            M5FlakyBadgePosture::StableBadge,
        ),
        (
            M5FlakyClassification::SuspectedFlaky,
            M5FlakyBadgePosture::SuspectedFlakyBadge,
        ),
        (
            M5FlakyClassification::ReproducedFlaky,
            M5FlakyBadgePosture::ReproducedFlakyBadge,
        ),
        (
            M5FlakyClassification::StableAgain,
            M5FlakyBadgePosture::StableAgainBadge,
        ),
        (
            M5FlakyClassification::ManuallyMuted,
            M5FlakyBadgePosture::ManuallyMutedBadge,
        ),
        (
            M5FlakyClassification::UnknownFlaky,
            M5FlakyBadgePosture::UnknownFlakyBadge,
        ),
    ];
    let mut postures = std::collections::BTreeSet::new();
    for (classification, expected) in cases {
        let resolved = resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            classification,
            ..reproduced_flaky_badge()
        })
        .expect("resolves");
        assert_eq!(resolved.flaky_posture, expected);
        assert_eq!(resolved.flaky_posture.classification(), classification);
        postures.insert(resolved.flaky_posture);
    }
    assert_eq!(postures.len(), M5FlakyBadgePosture::ALL.len());
}

#[test]
fn intermittent_failure_cannot_masquerade_as_reproduced() {
    // A reproduced classification with only a single occurrence fails resolution — the core
    // acceptance criterion.
    assert_eq!(
        resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            classification: M5FlakyClassification::ReproducedFlaky,
            confidence_class: M5FlakyConfidenceClass::SingleOccurrence,
            retry_window_size: 1,
            observed_failures: 1,
            ..reproduced_flaky_badge()
        }),
        Err(M5FlakyBadgeResolutionError::ReproducedWithoutEvidenceWindow)
    );
    // A reproduced classification measured over too small a window also fails.
    assert_eq!(
        resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            classification: M5FlakyClassification::ReproducedFlaky,
            confidence_class: M5FlakyConfidenceClass::HighConfidence,
            retry_window_size: 3,
            observed_failures: 1,
            ..reproduced_flaky_badge()
        }),
        Err(M5FlakyBadgeResolutionError::ReproducedWithoutEvidenceWindow)
    );
}

#[test]
fn suspected_flaky_stays_suspected_and_unconfirmed() {
    let resolved = resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::SuspectedFlaky,
        confidence_class: M5FlakyConfidenceClass::SingleOccurrence,
        retry_window_size: 1,
        observed_failures: 1,
        last_outcome: M5RetryAttemptOutcome::FailedAllRetries,
        ..reproduced_flaky_badge()
    })
    .expect("resolves");
    assert_eq!(
        resolved.flaky_posture,
        M5FlakyBadgePosture::SuspectedFlakyBadge
    );
    assert!(!resolved.is_confirmed_flaky);
    assert!(!resolved.claims_reproduced_flaky);
    assert!(resolved.needs_attention);
}

#[test]
fn muted_verdict_is_disclosed_and_drops_mute_action_when_policy_blocked() {
    let muted = resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::ManuallyMuted,
        mute_state: M5FlakyMuteState::QuarantineActive,
        ..reproduced_flaky_badge()
    })
    .expect("resolves");
    assert!(muted.is_muted_or_quarantined);
    assert!(muted
        .available_actions
        .contains(&M5FlakyBadgeAction::MuteOrQuarantine));

    let blocked = resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::Stable,
        mute_state: M5FlakyMuteState::PolicyBlocked,
        ..reproduced_flaky_badge()
    })
    .expect("resolves");
    assert!(!blocked
        .available_actions
        .contains(&M5FlakyBadgeAction::MuteOrQuarantine));
}

#[test]
fn flaky_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            badge_identity_ref: "  ".to_owned(),
            ..reproduced_flaky_badge()
        }),
        Err(M5FlakyBadgeResolutionError::EmptyBadgeIdentity)
    );
    assert_eq!(
        resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            test_identity_ref: "".to_owned(),
            ..reproduced_flaky_badge()
        }),
        Err(M5FlakyBadgeResolutionError::EmptyTestIdentity)
    );
    assert_eq!(
        resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            retry_window_size: 2,
            observed_failures: 5,
            ..reproduced_flaky_badge()
        }),
        Err(M5FlakyBadgeResolutionError::InvalidFailureCount)
    );
    assert_eq!(
        resolve_flaky_state_badge(&M5FlakyBadgeResolutionInput {
            test_identity_ref: "test:https://ci.example/flaky".to_owned(),
            ..reproduced_flaky_badge()
        }),
        Err(M5FlakyBadgeResolutionError::ForbiddenFlakyMaterial)
    );
}

// ---- retry-history-row resolver -----------------------------------------

#[test]
fn divergent_row_explains_outcomes_with_delta() {
    let resolved = resolve_retry_history_row(&divergent_retry_row()).expect("resolves");
    assert_eq!(resolved.row_posture, M5RetryRowPosture::PassedOnRetryRow);
    assert!(resolved.is_divergent);
    assert!(resolved.explains_divergent_outcomes);
    assert!(resolved.discloses_env_build_runtime_delta);
    assert!(resolved.preserves_outcome_order);
    assert!(resolved.has_log_continuity);
    assert!(resolved.needs_attention);
    assert!(resolved
        .available_actions
        .contains(&M5RetryRowAction::OpenLogs));
    assert!(resolved
        .available_actions
        .contains(&M5RetryRowAction::RerunTest));
}

#[test]
fn retry_posture_is_one_to_one_with_outcome() {
    for outcome in M5RetryAttemptOutcome::ALL {
        let resolved = resolve_retry_history_row(&M5RetryRowResolutionInput {
            last_outcome: outcome,
            // A two-element sequence so a passed-on-retry (divergent) posture still carries the
            // ordered evidence its resolver requires.
            recent_outcomes: vec![outcome, outcome],
            ..divergent_retry_row()
        })
        .expect("resolves");
        assert_eq!(resolved.row_posture.outcome(), outcome);
        assert!(resolved.preserves_outcome_order);
    }
}

#[test]
fn divergence_without_a_sequence_fails() {
    assert_eq!(
        resolve_retry_history_row(&M5RetryRowResolutionInput {
            last_outcome: M5RetryAttemptOutcome::PassedOnRetry,
            recent_outcomes: vec![M5RetryAttemptOutcome::PassedOnRetry],
            ..divergent_retry_row()
        }),
        Err(M5RetryRowResolutionError::DivergenceWithoutSequence)
    );
}

#[test]
fn widened_and_imported_rows_stay_disclosed() {
    let resolved = resolve_retry_history_row(&M5RetryRowResolutionInput {
        last_outcome: M5RetryAttemptOutcome::SkippedAttempt,
        recent_outcomes: vec![M5RetryAttemptOutcome::SkippedAttempt],
        scope_class: M5RetryScopeClass::ImportedAttempt,
        attempt_origin: M5RetryAttemptOrigin::ImportedCiAttempt,
        provenance_class: M5TestIntelligenceProvenanceClass::ImportedCiArtifact,
        ..divergent_retry_row()
    })
    .expect("resolves");
    assert!(resolved.is_imported);

    let widened = resolve_retry_history_row(&M5RetryRowResolutionInput {
        last_outcome: M5RetryAttemptOutcome::FailedAllRetries,
        recent_outcomes: vec![M5RetryAttemptOutcome::FailedAllRetries],
        scope_class: M5RetryScopeClass::WidenedSelection,
        ..divergent_retry_row()
    })
    .expect("resolves");
    assert!(widened.widened_scope);
    assert!(widened.needs_attention);
}

#[test]
fn retry_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_retry_history_row(&M5RetryRowResolutionInput {
            test_identity_ref: "  ".to_owned(),
            ..divergent_retry_row()
        }),
        Err(M5RetryRowResolutionError::EmptyTestIdentity)
    );
    assert_eq!(
        resolve_retry_history_row(&M5RetryRowResolutionInput {
            attempt_log_ref: "".to_owned(),
            ..divergent_retry_row()
        }),
        Err(M5RetryRowResolutionError::EmptyLogReference)
    );
    assert_eq!(
        resolve_retry_history_row(&M5RetryRowResolutionInput {
            recent_outcomes: vec![],
            ..divergent_retry_row()
        }),
        Err(M5RetryRowResolutionError::EmptyOutcomeSequence)
    );
    assert_eq!(
        resolve_retry_history_row(&M5RetryRowResolutionInput {
            attempt_log_ref: "attempt-log bearer token".to_owned(),
            ..divergent_retry_row()
        }),
        Err(M5RetryRowResolutionError::ForbiddenRetryMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_flaky_retry_components_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_FLAKY_RETRY_COMPONENTS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_flaky_retry_components_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5FlakyRetryComponentConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5FlakyRetryComponentConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_flaky_retry_components_packet();
    for row in &packet.rows {
        for part in M5FlakyBadgeAnatomyPart::MANDATORY {
            assert!(row.flaky_anatomy_parts.contains(&part));
        }
        for part in M5RetryRowAnatomyPart::MANDATORY {
            assert!(row.retry_anatomy_parts.contains(&part));
        }
        for field in M5FlakyBadgeExportField::MANDATORY {
            assert!(row.flaky_export_fields.contains(&field));
        }
        for field in M5RetryRowExportField::MANDATORY {
            assert!(row.retry_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.flaky_examples.is_empty());
        assert!(!row.retry_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_flaky_retry_components_packet();
    let flakies: Vec<&M5FlakyBadgeResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.flaky_examples.iter())
        .collect();
    let retries: Vec<&M5RetryRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.retry_examples.iter())
        .collect();

    for posture in M5FlakyBadgePosture::ALL {
        assert!(
            flakies.iter().any(|c| c.resolved.flaky_posture == posture),
            "no example exercises flaky posture {}",
            posture.as_str()
        );
    }
    for action in M5FlakyBadgeAction::ALL {
        assert!(
            flakies
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises flaky action {}",
            action.as_str()
        );
    }
    for posture in M5RetryRowPosture::ALL {
        assert!(
            retries.iter().any(|c| c.resolved.row_posture == posture),
            "no example exercises retry posture {}",
            posture.as_str()
        );
    }
    for action in M5RetryRowAction::ALL {
        assert!(
            retries
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises retry action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_flaky_retry_components_packet();
    for row in &packet.rows {
        for case in &row.flaky_examples {
            assert!(
                case.is_self_consistent(),
                "flaky case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "flaky case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.retry_examples {
            assert!(
                case.is_self_consistent(),
                "retry case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "retry case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5FlakyRetryComponentConsumerSurface::RetryHistoryPanel
    });
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.vocabulary_set.flaky_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::VocabularySetDrift));
}

#[test]
fn mandatory_flaky_anatomy_missing_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0]
        .flaky_anatomy_parts
        .retain(|p| *p != M5FlakyBadgeAnatomyPart::RetryWindowCue);
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::MandatoryFlakyAnatomyMissing));
}

#[test]
fn mandatory_retry_anatomy_missing_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0]
        .retry_anatomy_parts
        .retain(|p| *p != M5RetryRowAnatomyPart::EnvBuildRuntimeDeltaCue);
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::MandatoryRetryAnatomyMissing));
}

#[test]
fn mandatory_flaky_export_missing_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0]
        .flaky_export_fields
        .retain(|f| *f != M5FlakyBadgeExportField::ClassifierSource);
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::MandatoryFlakyExportMissing));
}

#[test]
fn mandatory_retry_export_missing_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0]
        .retry_export_fields
        .retain(|f| *f != M5RetryRowExportField::AttemptOrigin);
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::MandatoryRetryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0].flaky_examples[0].resolved.is_confirmed_flaky = false;
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::ExampleResolutionDrift));
}

#[test]
fn example_missing_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[1].retry_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::ExampleMissing));
}

#[test]
fn flaky_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    let stable = M5FlakyBadgeResolutionCase::resolved(M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::Stable,
        ..reproduced_flaky_badge()
    });
    for row in &mut packet.rows {
        row.flaky_examples = vec![stable.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::FlakyPostureCoverageUnproven));
}

#[test]
fn retry_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    let first_try = M5RetryRowResolutionCase::resolved(M5RetryRowResolutionInput {
        last_outcome: M5RetryAttemptOutcome::PassedFirstTry,
        recent_outcomes: vec![M5RetryAttemptOutcome::PassedFirstTry],
        ..divergent_retry_row()
    });
    for row in &mut packet.rows {
        row.retry_examples = vec![first_try.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::RetryPostureCoverageUnproven));
}

#[test]
fn evidence_window_disclosure_unproven_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    // Replace every flaky example with a stable one so neither the confirmed nor the suspected
    // half is proven.
    let stable = M5FlakyBadgeResolutionCase::resolved(M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::Stable,
        ..reproduced_flaky_badge()
    });
    for row in &mut packet.rows {
        row.flaky_examples = vec![stable.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::EvidenceWindowDisclosureUnproven));
}

#[test]
fn mute_disclosure_unproven_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    // Replace every flaky example with an unmuted one so the muted half fires.
    let unmuted = M5FlakyBadgeResolutionCase::resolved(M5FlakyBadgeResolutionInput {
        classification: M5FlakyClassification::Stable,
        mute_state: M5FlakyMuteState::NotMuted,
        ..reproduced_flaky_badge()
    });
    for row in &mut packet.rows {
        row.flaky_examples = vec![unmuted.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::MuteDisclosureUnproven));
}

#[test]
fn divergence_context_unproven_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    // Replace every retry example with a non-divergent one so the divergence proof fires.
    let clean = M5RetryRowResolutionCase::resolved(M5RetryRowResolutionInput {
        last_outcome: M5RetryAttemptOutcome::PassedFirstTry,
        recent_outcomes: vec![M5RetryAttemptOutcome::PassedFirstTry],
        has_env_delta: false,
        has_build_delta: false,
        has_runtime_delta: false,
        ..divergent_retry_row()
    });
    for row in &mut packet.rows {
        row.retry_examples = vec![clean.clone()];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5FlakyRetryComponentViolation::DivergenceContextUnproven));
}

#[test]
fn attempt_origin_coverage_unproven_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    // Replace every retry example with a local-origin one so remote / notebook / imported CI go
    // uncovered.
    let local = M5RetryRowResolutionCase::resolved(M5RetryRowResolutionInput {
        last_outcome: M5RetryAttemptOutcome::FailedAllRetries,
        recent_outcomes: vec![M5RetryAttemptOutcome::FailedAllRetries],
        attempt_origin: M5RetryAttemptOrigin::LocalAttempt,
        has_env_delta: true,
        ..divergent_retry_row()
    });
    for row in &mut packet.rows {
        row.retry_examples = vec![local.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::AttemptOriginCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0].labels_intermittent_as_confirmed_flaky = true;
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet
        .governance_review
        .intermittent_never_confirmed_without_evidence_window = false;
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet
        .consumer_projection
        .ci_and_support_read_same_flaky_retry_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_flaky_retry_components_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5FlakyRetryComponentViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_flaky_retry_components_packet().render_markdown_summary();
    for surface in M5FlakyRetryComponentConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_flaky_retry_components_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5FlakyRetryComponentConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5FlakyRetryComponentConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_flaky_retry_components_export()
        .expect("checked M5 flaky retry components export validates");
    assert_eq!(from_disk.packet_id, M5_FLAKY_RETRY_COMPONENTS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_flaky_retry_components_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed(),
        seeded_m5_flaky_retry_components_editor_badge_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5FlakyRetryComponentConsumerSurface::ALL.len()
        );
    }

    let dashboard = seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed();
    let row = dashboard
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5FlakyRetryComponentConsumerSurface::FlakyDashboardPanel)
        .expect("flaky-dashboard-panel row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Preview
    );

    let editor = seeded_m5_flaky_retry_components_editor_badge_beta_narrowed();
    let row = editor
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5FlakyRetryComponentConsumerSurface::EditorTestTreeBadge)
        .expect("editor-test-tree-badge row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Beta
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let dashboard: M5FlakyRetryComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-flaky-retry-primitive/flaky_dashboard_preview_narrowed.json"
    )))
    .expect("flaky-dashboard fixture parses");
    assert!(dashboard.validate().is_empty());
    assert_eq!(
        dashboard,
        seeded_m5_flaky_retry_components_flaky_dashboard_preview_narrowed()
    );

    let editor: M5FlakyRetryComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-flaky-retry-primitive/editor_badge_beta_narrowed.json"
    )))
    .expect("editor-badge fixture parses");
    assert!(editor.validate().is_empty());
    assert_eq!(
        editor,
        seeded_m5_flaky_retry_components_editor_badge_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_flaky_retry_components_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
