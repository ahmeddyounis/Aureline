use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_write_boundary_packet();
    validate_write_boundary_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_covers_every_state_and_outcome() {
    let packet = seeded_write_boundary_packet();
    let states: BTreeSet<_> = packet
        .cases
        .iter()
        .map(|c| c.decision.boundary_state)
        .collect();
    for required in BoundaryState::ALL {
        assert!(
            states.contains(&required),
            "missing boundary state {required:?}"
        );
    }
    let outcomes: BTreeSet<_> = packet
        .cases
        .iter()
        .map(|c| c.decision.attempt_outcome)
        .collect();
    for required in AttemptOutcome::ALL {
        assert!(outcomes.contains(&required), "missing outcome {required:?}");
    }
}

#[test]
fn canonical_authoritative_in_sync_admits_direct_edit() {
    let subject = subject_for(ArtifactClass::AiAssistedEdit, BoundaryState::InSync, false);
    let decision = decide_write_boundary(&subject);
    assert_eq!(decision.attempt_outcome, AttemptOutcome::DirectEditAdmitted);
    assert!(decision.direct_edit_admitted);
    assert_eq!(decision.effective_edit_gate, EditPosture::DirectEditAllowed);
    assert!(decision.why_blocked_tokens.is_empty());
    assert!(decision.recovery.is_empty());
    assert!(decision.diverged_from_generator.is_none());
    // The compare is still offered so a user can diff against the basis.
    assert_eq!(decision.three_way_compare.legs.len(), 3);
}

#[test]
fn derived_readonly_in_sync_is_blocked_regenerate_first() {
    let subject = subject_for(ArtifactClass::NotebookOutput, BoundaryState::InSync, false);
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.attempt_outcome,
        AttemptOutcome::BlockedRegenerateFirst
    );
    assert!(!decision.direct_edit_admitted);
    assert_eq!(decision.effective_edit_gate, EditPosture::RegenerateOnly);
    assert!(!decision.why_blocked_tokens.is_empty());
    assert_eq!(
        decision.recovery.first().map(|s| s.class),
        Some(RecoveryClass::RegenerateFromSource)
    );
}

#[test]
fn derived_editable_in_sync_is_held_pending_review() {
    let subject = subject_for(ArtifactClass::RequestArtifact, BoundaryState::InSync, false);
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.attempt_outcome,
        AttemptOutcome::BlockedPendingReview
    );
    assert_eq!(
        decision.effective_edit_gate,
        EditPosture::ReviewedOverrideRequired
    );
    assert!(decision.diverged_from_generator.is_none());
    assert_eq!(
        decision.recovery.first().map(|s| s.class),
        Some(RecoveryClass::ReviewedOverride)
    );
}

#[test]
fn recorded_override_admits_and_leaves_divergence() {
    let subject = subject_for(ArtifactClass::FrameworkCodegen, BoundaryState::InSync, true);
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.attempt_outcome,
        AttemptOutcome::OverrideAdmittedWithDivergence
    );
    assert!(decision.attempt_outcome.admits_write());
    assert!(!decision.direct_edit_admitted);
    let divergence = decision
        .diverged_from_generator
        .as_ref()
        .expect("an admitted override must leave a durable divergence");
    assert!(divergence.diverged);
    assert!(!divergence.recovery.is_empty());
    // Recovery offers both discard-via-regenerate and reconcile-into-source.
    let classes: BTreeSet<_> = divergence.recovery.iter().map(|s| s.class).collect();
    assert!(classes.contains(&RecoveryClass::RegenerateFromSource));
    assert!(classes.contains(&RecoveryClass::ReconcileIntoSource));
}

#[test]
fn drift_detected_forces_reviewed_override_on_direct_editable_source() {
    // A canonical-authoritative artifact is directly editable in sync, but
    // drift narrows the gate to a reviewed override.
    let subject = subject_for(
        ArtifactClass::AiAssistedEdit,
        BoundaryState::DriftDetected,
        false,
    );
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.effective_edit_gate,
        EditPosture::ReviewedOverrideRequired
    );
    assert!(decision.edit_gate_downgraded);
    assert_eq!(
        decision.attempt_outcome,
        AttemptOutcome::BlockedPendingReview
    );
    assert!(decision
        .why_blocked_tokens
        .contains(&"boundary_drift_detected".to_owned()));
}

#[test]
fn source_missing_blocks_and_drops_canonical_and_regenerated_legs() {
    let subject = subject_for(
        ArtifactClass::NotebookOutput,
        BoundaryState::SourceMissing,
        false,
    );
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.attempt_outcome,
        AttemptOutcome::BlockedRegenerateFirst
    );
    assert_eq!(
        decision.regeneration_availability,
        RegenerationAvailability::BlockedSourceMissing
    );
    // No canonical-source jump when the source is gone.
    assert!(decision.canonical_source_jump.is_none());
    // The canonical-source and regenerated-candidate legs are unavailable but
    // still preserve a provenance ref.
    for leg in &decision.three_way_compare.legs {
        assert!(!leg.provenance_ref.trim().is_empty());
        match leg.kind {
            CompareLegKind::CanonicalSource | CompareLegKind::RegeneratedCandidate => {
                assert_eq!(leg.availability, LegAvailability::Unavailable);
            }
            CompareLegKind::CurrentArtifact => {
                assert_eq!(leg.availability, LegAvailability::Available);
            }
        }
    }
    assert!(decision.three_way_compare.provenance_preserved);
    assert_eq!(
        decision.recovery.first().map(|s| s.class),
        Some(RecoveryClass::RestoreCanonicalSource)
    );
}

#[test]
fn generator_unavailable_blocks_with_restore_generator_recovery() {
    let subject = subject_for(
        ArtifactClass::PreviewDerivative,
        BoundaryState::GeneratorUnavailable,
        false,
    );
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.regeneration_availability,
        RegenerationAvailability::BlockedGeneratorUnavailable
    );
    assert_eq!(
        decision.recovery.first().map(|s| s.class),
        Some(RecoveryClass::RestoreGenerator)
    );
    // Source is still linked, so the jump action stays.
    assert!(decision.canonical_source_jump.is_some());
}

#[test]
fn regeneration_blocked_by_policy_surfaces_the_policy_block() {
    let subject = subject_for(
        ArtifactClass::FrameworkCodegen,
        BoundaryState::RegenerationBlockedByPolicy,
        false,
    );
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.regeneration_availability,
        RegenerationAvailability::BlockedByPolicy
    );
    assert_eq!(
        decision.recovery.first().map(|s| s.class),
        Some(RecoveryClass::ResolveRegenerationPolicy)
    );
    // The guidance names the policy block rather than a generic failure.
    assert!(decision.guidance_line.contains("policy"));
}

#[test]
fn override_never_forces_write_past_regenerate_only() {
    // Even with a recorded override, a regenerate-only gate is not bypassed.
    let subject = subject_for(
        ArtifactClass::NotebookOutput,
        BoundaryState::SourceMissing,
        true,
    );
    // The override ref is set but the gate is regenerate-only.
    assert!(subject.override_review_ref.is_some());
    let decision = decide_write_boundary(&subject);
    assert_eq!(
        decision.attempt_outcome,
        AttemptOutcome::BlockedRegenerateFirst
    );
    assert!(!decision.attempt_outcome.admits_write());
    assert!(decision.diverged_from_generator.is_none());
}

#[test]
fn gate_only_narrows_never_widens() {
    // A regenerate-only declared posture with an in-sync boundary stays
    // regenerate-only — the boundary state never widens the gate.
    let subject = subject_for(ArtifactClass::SupportPacket, BoundaryState::InSync, false);
    let decision = decide_write_boundary(&subject);
    assert_eq!(decision.effective_edit_gate, EditPosture::RegenerateOnly);
    assert!(!decision.edit_gate_downgraded);
}

#[test]
fn seeded_fixtures_validate_and_cover_every_outcome() {
    let fixtures = seeded_write_boundary_fixtures();
    assert!(!fixtures.is_empty());
    let mut outcomes = BTreeSet::new();
    let mut saw_divergence = false;
    let mut saw_block = false;
    for fixture in &fixtures {
        validate_write_boundary_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        outcomes.insert(fixture.expected_attempt_outcome);
        if fixture.expected_leaves_divergence {
            saw_divergence = true;
        }
        if !fixture.expected_why_blocked_tokens.is_empty() {
            saw_block = true;
        }
    }
    for required in AttemptOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(saw_divergence, "fixtures must cover a divergence");
    assert!(saw_block, "fixtures must cover a blocked edit");
}

#[test]
fn copy_line_is_stable_and_self_consistent() {
    let subject = subject_for(
        ArtifactClass::FrameworkCodegen,
        BoundaryState::InSync,
        false,
    );
    let decision = decide_write_boundary(&subject);
    let expected = "write-boundary class=framework_codegen authority=derived_editable boundary=in_sync gate=reviewed_override_required outcome=blocked_pending_review direct_edit=false regen=available";
    assert_eq!(decision.copy_line, expected);
    assert_eq!(write_boundary_copy_line(&decision, &subject), expected);
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_write_boundary_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: WriteBoundaryPacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}

#[test]
fn fixtures_round_trip_through_json() {
    for fixture in seeded_write_boundary_fixtures() {
        let json = serde_json::to_string(&fixture).expect("fixture serializes");
        let back: WriteBoundaryFixture = serde_json::from_str(&json).expect("fixture deserializes");
        assert_eq!(fixture, back);
    }
}
