use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_mutation_guardrails_packet();
    validate_mutation_guardrails_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_covers_every_route_outcome_and_unmet_requirement() {
    let packet = seeded_mutation_guardrails_packet();
    let routes: BTreeSet<_> = packet.cases.iter().map(|c| c.decision.route).collect();
    for required in MutationRoute::ALL {
        assert!(routes.contains(&required), "missing route {required:?}");
    }
    let outcomes: BTreeSet<_> = packet
        .cases
        .iter()
        .map(|c| c.decision.guardrail_outcome)
        .collect();
    for required in GuardrailOutcome::ALL {
        assert!(outcomes.contains(&required), "missing outcome {required:?}");
    }
    let unmet: BTreeSet<_> = packet
        .cases
        .iter()
        .flat_map(|c| c.decision.unmet_safety_requirements.iter().copied())
        .collect();
    for required in SafetyRequirement::ALL {
        assert!(
            unmet.contains(&required),
            "missing unmet requirement {required:?}"
        );
    }
}

#[test]
fn canonical_in_sync_admits_ai_apply_directly() {
    let attempt = make_attempt(
        "t.admit",
        MutationRoute::AiApply,
        MutationSourceClass::AiHostedProvider,
        "ai/scoped-composer@1.0.0",
        MutationClass::SemanticTooling,
        BoundaryDataState::Present,
        ArtifactClass::AiAssistedEdit,
        BoundaryState::InSync,
        complete_envelope().build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(decision.guardrail_outcome, GuardrailOutcome::AdmittedDirect);
    assert!(decision.mutation_admitted);
    assert!(!decision.crosses_canonical_boundary);
    assert!(decision.why_blocked_tokens.is_empty());
    assert!(decision.required_safety.is_empty());
}

#[test]
fn complete_envelope_with_override_admits_cross_boundary_with_divergence() {
    let attempt = make_attempt(
        "t.cross_admit",
        MutationRoute::Refactor,
        MutationSourceClass::MachineLocal,
        "refactor/extract-function",
        MutationClass::SemanticTooling,
        BoundaryDataState::Present,
        ArtifactClass::FrameworkCodegen,
        BoundaryState::InSync,
        complete_envelope().build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::AdmittedWithPreviewAndOverride
    );
    assert!(decision.mutation_admitted);
    assert!(decision.crosses_canonical_boundary);
    assert!(decision.safety_envelope_complete);
    assert!(decision.why_blocked_tokens.is_empty());
    assert!(decision.boundary_decision.diverged_from_generator.is_some());
}

#[test]
fn missing_preview_holds_cross_boundary_mutation() {
    let attempt = make_attempt(
        "t.no_preview",
        MutationRoute::AiApply,
        MutationSourceClass::AiHostedProvider,
        "ai/scoped-composer@1.0.0",
        MutationClass::SemanticTooling,
        BoundaryDataState::Present,
        ArtifactClass::FrameworkCodegen,
        BoundaryState::InSync,
        EnvelopeSpec {
            preview_ref: None,
            ..complete_envelope()
        }
        .build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::BlockedPendingReview
    );
    assert!(!decision.mutation_admitted);
    assert_eq!(
        decision.unmet_safety_requirements,
        vec![SafetyRequirement::Preview]
    );
    assert!(decision
        .why_blocked_tokens
        .contains(&"missing_preview".to_owned()));
}

#[test]
fn undeclared_side_effects_are_never_run_silently() {
    let attempt = make_attempt(
        "t.undeclared",
        MutationRoute::Automation,
        MutationSourceClass::PolicyDriven,
        "automation/codegen-runner",
        MutationClass::GeneratedState,
        BoundaryDataState::Present,
        ArtifactClass::FrameworkCodegen,
        BoundaryState::InSync,
        EnvelopeSpec {
            side_effects: vec![
                SideEffectClass::LocalCompute,
                SideEffectClass::NetworkInstall,
            ],
            side_effect_disclosure: SideEffectDisclosure::Undeclared,
            reversal_class: ReversalClass::RegenerateOrRecompute,
            ..complete_envelope()
        }
        .build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::BlockedPendingReview
    );
    assert!(decision
        .unmet_safety_requirements
        .contains(&SafetyRequirement::SideEffectSummary));
    // A networked install escapes the checkpoint, so the rollback is partial.
    assert_eq!(
        decision.rollback_coverage,
        RollbackCoverage::PartiallyReversible
    );
}

#[test]
fn audit_only_reversal_class_is_not_rollback_safe() {
    let attempt = make_attempt(
        "t.audit_only",
        MutationRoute::Automation,
        MutationSourceClass::PolicyDriven,
        "automation/codegen-runner",
        MutationClass::GeneratedState,
        BoundaryDataState::Present,
        ArtifactClass::FrameworkCodegen,
        BoundaryState::InSync,
        EnvelopeSpec {
            reversal_class: ReversalClass::AuditOnly,
            ..complete_envelope()
        }
        .build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::BlockedPendingReview
    );
    assert!(decision
        .unmet_safety_requirements
        .contains(&SafetyRequirement::RollbackClass));
}

#[test]
fn derived_readonly_target_is_blocked_regenerate_first() {
    let attempt = make_attempt(
        "t.regen",
        MutationRoute::Automation,
        MutationSourceClass::MachineRemoteAgent,
        "automation/notebook-runner",
        MutationClass::GeneratedState,
        BoundaryDataState::Present,
        ArtifactClass::NotebookOutput,
        BoundaryState::InSync,
        complete_envelope().build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::BlockedRegenerateFirst
    );
    assert!(!decision.mutation_admitted);
    assert!(decision.crosses_canonical_boundary);
}

#[test]
fn missing_boundary_data_blocks_even_with_complete_envelope() {
    let attempt = make_attempt(
        "t.missing_data",
        MutationRoute::AiApply,
        MutationSourceClass::AiHostedProvider,
        "ai/scoped-composer@1.0.0",
        MutationClass::SemanticTooling,
        BoundaryDataState::Missing,
        ArtifactClass::FrameworkCodegen,
        BoundaryState::SourceMissing,
        complete_envelope().build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::BlockedMissingBoundaryData
    );
    assert!(!decision.mutation_admitted);
    assert_eq!(decision.effective_edit_gate, EditPosture::RegenerateOnly);
    assert!(decision
        .why_blocked_tokens
        .contains(&"missing_canonical_source_boundary_data".to_owned()));
}

#[test]
fn no_override_holds_derived_editable_target() {
    let attempt = make_attempt(
        "t.no_override",
        MutationRoute::QuickFix,
        MutationSourceClass::MachineLocal,
        "quick-fix/apply-suggestion",
        MutationClass::SemanticTooling,
        BoundaryDataState::Present,
        ArtifactClass::RequestArtifact,
        BoundaryState::InSync,
        EnvelopeSpec {
            override_recorded: false,
            ..complete_envelope()
        }
        .build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(
        decision.guardrail_outcome,
        GuardrailOutcome::BlockedPendingReview
    );
    // The boundary itself holds it pending review; the envelope is otherwise
    // complete, so no unmet requirement is named.
    assert!(decision.unmet_safety_requirements.is_empty());
    assert!(decision
        .why_blocked_tokens
        .contains(&"declared_reviewed_override_required".to_owned()));
}

#[test]
fn actor_lineage_records_route_and_reversal_class() {
    let attempt = make_attempt(
        "t.lineage",
        MutationRoute::Refactor,
        MutationSourceClass::MachineLocal,
        "refactor/rename-symbol",
        MutationClass::SemanticTooling,
        BoundaryDataState::Present,
        ArtifactClass::ScaffoldedProject,
        BoundaryState::InSync,
        complete_envelope().build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    assert_eq!(decision.actor_lineage.actor_class, "refactor_engine");
    assert_eq!(
        decision.actor_lineage.reversal_class,
        ReversalClass::RestoreFromCheckpoint
    );
    assert_eq!(
        decision.actor_lineage.mutation_class,
        MutationClass::SemanticTooling
    );
}

#[test]
fn copy_line_is_stable_and_self_consistent() {
    let attempt = make_attempt(
        "t.copy",
        MutationRoute::Refactor,
        MutationSourceClass::MachineLocal,
        "refactor/extract-function",
        MutationClass::SemanticTooling,
        BoundaryDataState::Present,
        ArtifactClass::FrameworkCodegen,
        BoundaryState::InSync,
        complete_envelope().build(),
    );
    let decision = decide_mutation_guardrail(&attempt);
    let expected = "mutation-guardrails route=refactor mutation_class=semantic_tooling boundary_data=present outcome=admitted_with_preview_and_override admitted=true crosses_boundary=true gate=reviewed_override_required rollback=fully_reversible reversal=restore_from_checkpoint";
    assert_eq!(decision.copy_line, expected);
    assert_eq!(mutation_guardrails_copy_line(&decision), expected);
}

#[test]
fn seeded_fixtures_validate_and_cover_outcomes() {
    let fixtures = seeded_mutation_guardrails_fixtures();
    assert!(!fixtures.is_empty());
    let mut outcomes = BTreeSet::new();
    let mut saw_divergence = false;
    let mut saw_block = false;
    for fixture in &fixtures {
        validate_mutation_guardrails_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        outcomes.insert(fixture.expected_guardrail_outcome);
        if fixture.expected_leaves_divergence {
            saw_divergence = true;
        }
        if !fixture.expected_why_blocked_tokens.is_empty() {
            saw_block = true;
        }
    }
    for required in GuardrailOutcome::ALL {
        assert!(
            outcomes.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(saw_divergence, "fixtures must cover a divergence");
    assert!(saw_block, "fixtures must cover a blocked mutation");
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_mutation_guardrails_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: MutationGuardrailPacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}

#[test]
fn fixtures_round_trip_through_json() {
    for fixture in seeded_mutation_guardrails_fixtures() {
        let json = serde_json::to_string(&fixture).expect("fixture serializes");
        let back: MutationGuardrailFixture =
            serde_json::from_str(&json).expect("fixture deserializes");
        assert_eq!(fixture, back);
    }
}
