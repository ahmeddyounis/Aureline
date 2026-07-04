//! Tests for the M5 run/attempt-header primitive: the resolver, the parity matrix,
//! and the checked-in support export.

use super::*;

// --- resolver: AC1 run/attempt identity distinct + attempt selector ---

#[test]
fn resolver_preserves_header_identity_across_surfaces() {
    let input = task_running_multi_attempt_input();
    let resolved = resolve_run_attempt_header(&input).expect("resolves");
    assert_eq!(resolved.header_id, input.header_id);
    assert_eq!(resolved.header.header_id, input.header_id);
    assert_eq!(resolved.selector.header_id, input.header_id);
    assert_eq!(resolved.cli_line.header_id, input.header_id);
    assert_eq!(resolved.export.header_id, input.header_id);
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_keeps_run_and_attempt_distinct() {
    let resolved = resolve_run_attempt_header(&task_running_multi_attempt_input()).expect("resolves");
    assert!(resolved.run_and_attempt_distinct());
    assert_ne!(resolved.header.run_ref, resolved.header.attempt_ref);
    assert!(resolved.header.run_and_attempt_distinct);
}

#[test]
fn resolver_selector_distinguishes_multi_attempt_run_from_separate_runs() {
    let resolved = resolve_run_attempt_header(&task_running_multi_attempt_input()).expect("resolves");
    assert_eq!(resolved.selector.attempt_count, 2);
    assert!(resolved.selector.all_attempts_share_run);
    assert!(resolved.distinguishes_attempts_from_runs());
    // The current attempt is present and flagged, and every attempt belongs to the
    // same run.
    assert_eq!(
        resolved.selector.current_attempt_ref,
        resolved.header.attempt_ref
    );
    assert!(resolved
        .selector
        .attempts
        .iter()
        .any(|a| a.is_current && a.attempt_ref == resolved.header.attempt_ref));
    // Ordinals are ordered for determinism.
    assert_eq!(resolved.selector.attempts[0].attempt_ordinal, 1);
    assert_eq!(resolved.selector.attempts[1].attempt_ordinal, 2);
}

#[test]
fn resolver_rejects_collapsed_run_and_attempt_identity() {
    let input = M5RunAttemptHeaderInput {
        attempt_ref: "run:build-and-test:0001".to_owned(),
        run_ref: "run:build-and-test:0001".to_owned(),
        sibling_attempts: vec![],
        ..task_running_multi_attempt_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::RunAttemptIdentityCollapsed)
    );
}

#[test]
fn resolver_rejects_sibling_collapsed_with_current() {
    let input = M5RunAttemptHeaderInput {
        sibling_attempts: vec![M5SiblingAttempt {
            attempt_ref: "attempt:build-and-test:0001#2".to_owned(),
            attempt_ordinal: 1,
            outcome: M5RunOutcome::Failed,
            is_current: false,
        }],
        ..task_running_multi_attempt_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::SiblingAttemptCollapsed)
    );
}

#[test]
fn resolver_rejects_duplicate_sibling_attempt() {
    let input = M5RunAttemptHeaderInput {
        sibling_attempts: vec![
            M5SiblingAttempt {
                attempt_ref: "attempt:build-and-test:0001#1".to_owned(),
                attempt_ordinal: 1,
                outcome: M5RunOutcome::Failed,
                is_current: false,
            },
            M5SiblingAttempt {
                attempt_ref: "attempt:build-and-test:0001#1b".to_owned(),
                attempt_ordinal: 1,
                outcome: M5RunOutcome::Failed,
                is_current: false,
            },
        ],
        ..task_running_multi_attempt_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::DuplicateSiblingAttempt)
    );
}

// --- resolver: AC2 state-label parity ---

#[test]
fn resolver_state_labels_consistent_across_projections() {
    let resolved = resolve_run_attempt_header(&test_running_input()).expect("resolves");
    assert_eq!(resolved.header.state_label, "Running");
    assert_eq!(resolved.export.state_label, "Running");
    assert_eq!(resolved.header.outcome, resolved.cli_line.outcome);
    assert!(resolved.state_labels_consistent());
}

#[test]
fn same_outcome_yields_same_label_across_surfaces() {
    let publish = resolve_run_attempt_header(&publish_passed_input()).expect("resolves");
    let preview = resolve_run_attempt_header(&preview_passed_input()).expect("resolves");
    assert_eq!(publish.header.outcome, M5RunOutcome::Passed);
    assert_eq!(preview.header.outcome, M5RunOutcome::Passed);
    assert_eq!(publish.header.state_label, preview.header.state_label);
    assert_eq!(publish.header.state_label, "Passed");
}

// --- resolver: AC3 export preserves IDs and states ---

#[test]
fn resolver_export_preserves_ids_and_states() {
    let resolved = resolve_run_attempt_header(&support_replay_input()).expect("resolves");
    assert_eq!(resolved.export.run_ref, resolved.header.run_ref);
    assert_eq!(resolved.export.attempt_ref, resolved.header.attempt_ref);
    assert_eq!(resolved.export.attempt_ordinal, resolved.header.attempt_ordinal);
    assert_eq!(resolved.export.outcome, resolved.header.outcome);
    assert_eq!(resolved.export.truth_mode, resolved.header.truth_mode);
    assert!(resolved.export_preserves_ids_and_states());
    assert!(declares_mandatory_export_fields(&resolved.export.export_fields));
}

// --- resolver: queue reason + admission-control disclosure ---

#[test]
fn resolver_discloses_queue_reason_and_admission_class() {
    let resolved = resolve_run_attempt_header(&ai_queued_input()).expect("resolves");
    assert_eq!(resolved.header.outcome, M5RunOutcome::Queued);
    assert_eq!(
        resolved.header.admission_control,
        M5AdmissionControlClass::DependencyQueued
    );
    assert!(resolved.header.queue_reason.is_some());
    assert_eq!(resolved.selector.relative_ordering, Some(3));
    // The CLI line renders the admission class in the shared vocabulary.
    assert!(resolved.cli_line.line.contains("admission=dependency_queued"));
}

#[test]
fn resolver_rejects_queued_without_admission_reason() {
    let input = M5RunAttemptHeaderInput {
        admission_control: M5AdmissionControlClass::Immediate,
        queue_reason: None,
        ..ai_queued_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::QueuedWithoutAdmissionReason)
    );
}

#[test]
fn resolver_rejects_admission_queued_without_queue_reason() {
    let input = M5RunAttemptHeaderInput {
        outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Live,
        admission_control: M5AdmissionControlClass::CapacityQueued,
        queue_reason: None,
        ..ai_queued_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::QueueReasonMissing)
    );
}

// --- resolver: captured-versus-live honesty ---

#[test]
fn resolver_rejects_active_outcome_not_live() {
    let input = M5RunAttemptHeaderInput {
        outcome: M5RunOutcome::Running,
        truth_mode: M5ExecutionTruthMode::Captured,
        ..test_running_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::ActiveOutcomeNotLive)
    );
}

#[test]
fn resolver_rejects_stale_output_claiming_live() {
    let input = M5RunAttemptHeaderInput {
        outcome: M5RunOutcome::StaleOutput,
        truth_mode: M5ExecutionTruthMode::Live,
        degraded: None,
        ..history_stale_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::StaleOutputClaimsLive)
    );
}

#[test]
fn resolver_stale_output_is_captured_evidence() {
    let resolved = resolve_run_attempt_header(&history_stale_input()).expect("resolves");
    assert_eq!(resolved.header.outcome, M5RunOutcome::StaleOutput);
    assert_eq!(resolved.header.truth_mode, M5ExecutionTruthMode::Captured);
    assert!(resolved.degraded.is_some());
    assert_eq!(resolved.header.state_label, "Stale output");
}

// --- resolver: structural rejections ---

#[test]
fn resolver_rejects_empty_header_id() {
    let input = M5RunAttemptHeaderInput {
        header_id: "   ".to_owned(),
        ..test_running_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::EmptyHeaderId)
    );
}

#[test]
fn resolver_rejects_zero_attempt_ordinal() {
    let input = M5RunAttemptHeaderInput {
        attempt_ordinal: 0,
        ..test_running_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::InvalidAttemptOrdinal)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5RunAttemptHeaderInput {
        context_summary: "see https://example.com/run".to_owned(),
        ..test_running_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5RunAttemptHeaderInput {
        degraded: Some(DegradedState {
            trigger: M5ExecutionDowngradeTrigger::CapturedEvidenceOnly,
            degraded_label: "stale".to_owned(),
        }),
        ..history_stale_input()
    };
    assert_eq!(
        resolve_run_attempt_header(&input),
        Err(M5RunAttemptResolutionError::DegradedLabelGeneric)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_run_attempt_header_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_run_attempt_header_packet();
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
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_run_attempt_header_packet();
    for row in &packet.surface_rows {
        for case in &row.example_headers {
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
    assert!(M5RunAttemptVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_run_attempt_header_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_run_attempt_header_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5RunAttemptViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_run_attempt_header_packet();
    packet.surface_rows[0].blurs_run_and_attempt = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5RunAttemptViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_run_attempt_header_packet();
    packet.surface_rows[0].example_headers[0]
        .resolved
        .header
        .state_label = "Bogus".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&M5RunAttemptViolation::ExampleHeaderDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_run_attempt_header_packet();
    packet.vocabulary_set.initiator_classes.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5RunAttemptViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_run_attempt_header_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_run_attempt_header_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_run_attempt_header_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_run_attempt_header_packet();
    assert_eq!(packet.record_kind, M5_RUN_ATTEMPT_HEADER_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_RUN_ATTEMPT_HEADER_SCHEMA_VERSION);
}
