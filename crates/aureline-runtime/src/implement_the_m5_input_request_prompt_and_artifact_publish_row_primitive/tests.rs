//! Tests for the M5 input-request / artifact-publish primitive: the resolver, the
//! parity matrix, and the checked-in support export.

use super::*;

// --- resolver: AC1 dismissed / timed-out requests are visible, attributable ---

#[test]
fn resolver_awaiting_prompt_resolves_to_awaiting_posture() {
    let resolved =
        resolve_execution_interaction(&request_awaiting_approval_input()).expect("resolves");
    let prompt = resolved.input_prompt.as_ref().expect("prompt present");
    assert_eq!(prompt.disposition, M5InputRequestDisposition::AwaitingResponse);
    assert_eq!(prompt.result_posture, M5InputResultPosture::AwaitingResponse);
    assert!(prompt.is_attributable);
    assert!(resolved.discloses_input_consequence());
}

#[test]
fn resolver_dismissed_prompt_leaves_run_blocked_not_silent() {
    let resolved = resolve_execution_interaction(&ai_dismissed_input()).expect("resolves");
    let prompt = resolved.input_prompt.as_ref().expect("prompt present");
    assert_eq!(prompt.disposition, M5InputRequestDisposition::Dismissed);
    assert_eq!(prompt.result_posture, M5InputResultPosture::RunBlockedWaiting);
    assert!(prompt.disposition.is_negative());
    assert!(prompt.result_posture.is_dismissal_or_timeout_consequence());
    assert!(resolved.discloses_input_consequence());
}

#[test]
fn resolver_timed_out_prompt_cancels_run() {
    let resolved = resolve_execution_interaction(&companion_timed_out_input()).expect("resolves");
    let prompt = resolved.input_prompt.as_ref().expect("prompt present");
    assert_eq!(prompt.disposition, M5InputRequestDisposition::TimedOut);
    assert_eq!(prompt.result_posture, M5InputResultPosture::RunCancelled);
    assert_eq!(resolved.export.run_outcome, M5RunOutcome::Cancelled);
}

#[test]
fn resolver_timeout_applies_default_posture() {
    let resolved = resolve_execution_interaction(&notebook_partial_input()).expect("resolves");
    let prompt = resolved.input_prompt.as_ref().expect("prompt present");
    // The notebook answered before the deadline, so it proceeds; a timeout would have
    // applied the declared default.
    assert_eq!(prompt.disposition, M5InputRequestDisposition::Continued);
    assert_eq!(prompt.result_posture, M5InputResultPosture::RunProceeds);
    assert_eq!(
        input_result_posture(
            M5InputRequestDisposition::TimedOut,
            M5InputConsequence::TimeoutAppliesDefault
        ),
        M5InputResultPosture::RunProceedsWithDefault
    );
}

#[test]
fn resolver_rejects_awaiting_response_when_run_not_waiting() {
    let mut input = ai_dismissed_input();
    input.run_outcome = M5RunOutcome::Running;
    input.input_request.as_mut().unwrap().disposition = M5InputRequestDisposition::AwaitingResponse;
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::AwaitingResponseButRunNotWaiting)
    );
}

#[test]
fn resolver_rejects_waiting_run_without_prompt() {
    let mut input = test_running_input();
    input.run_outcome = M5RunOutcome::WaitingInput;
    input.input_request = None;
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::WaitingWithoutInputPrompt)
    );
}

#[test]
fn resolver_rejects_timeout_governed_prompt_without_deadline() {
    let mut input = companion_timed_out_input();
    let request = input.input_request.as_mut().unwrap();
    request.has_deadline = false;
    request.deadline_label = None;
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::InputDeadlineMissing)
    );
}

#[test]
fn resolver_rejects_timeout_default_without_default_label() {
    let mut input = notebook_partial_input();
    input.input_request.as_mut().unwrap().default_label = None;
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::TimeoutDefaultMissing)
    );
}

// --- resolver: AC2 produced artifacts stay attributable via lineage ---

#[test]
fn resolver_preserves_artifact_lineage() {
    let resolved = resolve_execution_interaction(&task_running_input()).expect("resolves");
    assert_eq!(resolved.artifact_rows.len(), 2);
    for row in &resolved.artifact_rows {
        assert_eq!(row.producing_run_ref, resolved.run_ref);
        assert_eq!(row.producing_attempt_ref, resolved.attempt_ref);
        assert!(row.lineage_preserved);
        assert!(row.is_attributable);
    }
    assert!(resolved.preserves_artifact_lineage());
}

#[test]
fn resolver_evicted_artifact_stays_attributable() {
    let resolved = resolve_execution_interaction(&history_evicted_input()).expect("resolves");
    let row = &resolved.artifact_rows[0];
    assert!(row.retention.is_evicted());
    assert!(row.is_attributable);
    assert!(!row.is_openable, "evicted-recoverable report offers no open action");
    assert!(row.export_action_ref.is_some());
    assert!(resolved.preserves_artifact_lineage());
}

#[test]
fn resolver_rejects_broken_artifact_lineage() {
    let mut input = publish_passed_input();
    input.artifacts[0].producing_run_ref = "run:some-other-run:9999".to_owned();
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::ArtifactLineageBroken)
    );
}

#[test]
fn resolver_rejects_duplicate_artifact() {
    let mut input = test_running_input();
    input.artifacts[1].artifact_ref = input.artifacts[0].artifact_ref.clone();
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::DuplicateArtifact)
    );
}

// --- resolver: AC3 artifact freshness disclosed before action ---

#[test]
fn resolver_discloses_artifact_freshness() {
    let resolved = resolve_execution_interaction(&support_imported_input()).expect("resolves");
    let row = &resolved.artifact_rows[0];
    assert_eq!(row.freshness, M5ArtifactFreshness::Imported);
    assert!(row.freshness_disclosed);
    assert!(row.open_action_ref.is_some() || row.export_action_ref.is_some());
    assert!(resolved.discloses_artifact_freshness());
}

#[test]
fn resolver_rejects_live_artifact_from_inactive_run() {
    let mut input = publish_passed_input();
    input.artifacts[0].freshness = M5ArtifactFreshness::Live;
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::LiveArtifactFromInactiveRun)
    );
}

#[test]
fn resolver_rejects_evicted_gone_artifact_offering_open() {
    let mut input = publish_passed_input();
    input.artifacts[0].retention = M5RetentionClass::EvictedGone;
    // still offers an open action → rejected
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::EvictedGoneArtifactOffersOpen)
    );
}

#[test]
fn resolver_rejects_artifact_without_any_action() {
    let mut input = publish_passed_input();
    input.artifacts[0].open_action_ref = None;
    input.artifacts[0].export_action_ref = None;
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::ArtifactMissingAction)
    );
}

// --- resolver: shared identity + structural rejections ---

#[test]
fn resolver_identity_consistent_across_projections() {
    let resolved = resolve_execution_interaction(&task_running_input()).expect("resolves");
    assert!(resolved.identity_consistent());
    assert_eq!(resolved.cli_line.interaction_id, resolved.interaction_id);
    assert_eq!(resolved.export.interaction_id, resolved.interaction_id);
    assert_eq!(
        resolved.export.artifact_refs.len(),
        resolved.artifact_rows.len()
    );
}

#[test]
fn resolver_rejects_collapsed_run_and_attempt_identity() {
    let mut input = task_running_input();
    input.attempt_ref = input.run_ref.clone();
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::RunAttemptIdentityCollapsed)
    );
}

#[test]
fn resolver_rejects_empty_interaction() {
    let mut input = test_running_input();
    input.input_request = None;
    input.artifacts = vec![];
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::EmptyInteraction)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let mut input = test_running_input();
    input.context_summary = "see https://example.com/run".to_owned();
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let mut input = history_evicted_input();
    input.degraded = Some(DegradedState {
        trigger: M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
        degraded_label: "degraded".to_owned(),
    });
    assert_eq!(
        resolve_execution_interaction(&input),
        Err(M5ExecutionInteractionError::DegradedLabelGeneric)
    );
}

#[test]
fn cli_line_renders_input_and_artifact_tokens() {
    let resolved = resolve_execution_interaction(&ai_dismissed_input()).expect("resolves");
    assert!(resolved
        .cli_line
        .line
        .contains("input=dismissed:run_blocked_waiting"));
    assert!(resolved.cli_line.line.contains("artifacts=1"));
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_execution_interaction_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_execution_interaction_packet();
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
fn seeded_matrix_covers_every_freshness_class() {
    let packet = seeded_m5_execution_interaction_packet();
    let seen: BTreeSet<M5ArtifactFreshness> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_interactions.iter())
        .flat_map(|case| case.resolved.artifact_rows.iter())
        .map(|row| row.freshness)
        .collect();
    for freshness in M5ArtifactFreshness::ALL {
        assert!(seen.contains(&freshness), "missing freshness {freshness:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_execution_interaction_packet();
    for row in &packet.surface_rows {
        for case in &row.example_interactions {
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
    assert!(M5InteractionVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_execution_interaction_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_execution_interaction_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5InteractionViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_execution_interaction_packet();
    packet.surface_rows[0].drops_artifact_lineage = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5InteractionViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_execution_interaction_packet();
    packet.surface_rows[0].example_interactions[0]
        .resolved
        .artifact_rows[0]
        .freshness = M5ArtifactFreshness::Imported;
    let violations = packet.validate();
    assert!(violations.contains(&M5InteractionViolation::ExampleInteractionDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_execution_interaction_packet();
    packet.vocabulary_set.input_kinds.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5InteractionViolation::VocabularySetDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_execution_interaction_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_execution_interaction_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_execution_interaction_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_execution_interaction_packet();
    assert_eq!(packet.record_kind, M5_EXECUTION_INTERACTION_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_EXECUTION_INTERACTION_SCHEMA_VERSION
    );
}
