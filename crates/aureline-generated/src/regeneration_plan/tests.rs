use super::*;

fn healthy_request(plan_id: &str, class: ArtifactClass) -> RegenerationRequest {
    request(
        plan_id,
        "Regenerate the artifact",
        vec![TargetSpec::healthy(class)],
    )
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_regeneration_plan_packet();
    validate_regeneration_plan_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_covers_every_readiness_and_outcome() {
    let packet = seeded_regeneration_plan_packet();
    let readiness: BTreeSet<_> = packet.cases.iter().map(|c| c.plan.readiness).collect();
    for required in PlanReadiness::ALL {
        assert!(
            readiness.contains(&required),
            "missing readiness {required:?}"
        );
    }
    let outcomes: BTreeSet<_> = packet
        .cases
        .iter()
        .flat_map(|c| c.plan.targets.iter().map(|t| t.outcome))
        .collect();
    for required in TargetOutcome::ALL {
        assert!(outcomes.contains(&required), "missing outcome {required:?}");
    }
}

#[test]
fn healthy_plan_is_ready_and_fully_reversible() {
    let plan = plan_regeneration(&healthy_request(
        "plan.ready",
        ArtifactClass::ScaffoldedProject,
    ));
    assert_eq!(plan.readiness, PlanReadiness::Ready);
    assert!(plan.runs_in_full);
    assert!(!plan.partial);
    assert!(!plan.stale_inputs);
    assert!(plan.why_blocked_tokens.is_empty());
    assert!(plan.recovery.is_empty());
    assert_eq!(plan.rollback_coverage, RollbackCoverage::FullyReversible);
    assert_eq!(plan.runnable_target_count, 1);
    assert_eq!(plan.blocked_target_count, 0);
}

#[test]
fn stale_inputs_are_labeled_not_hidden() {
    let req = request(
        "plan.stale",
        "Re-run the cell",
        vec![TargetSpec {
            input_freshness: PreconditionState::Stale,
            ..TargetSpec::healthy(ArtifactClass::NotebookOutput)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::ReadyStaleInputs);
    assert!(plan.runs_in_full);
    assert!(plan.stale_inputs);
    // Stale is surfaced via a refresh recovery, not buried.
    assert_eq!(
        plan.recovery.first().map(|s| s.class),
        Some(RecoveryClass::RefreshInputs)
    );
    // Staleness is not a block reason.
    assert!(plan.why_blocked_tokens.is_empty());
}

#[test]
fn mixed_targets_yield_a_partial_plan() {
    let req = request(
        "plan.partial",
        "Regenerate both",
        vec![
            TargetSpec::healthy(ArtifactClass::FrameworkCodegen),
            TargetSpec {
                source_state: PreconditionState::Missing,
                ..TargetSpec::healthy(ArtifactClass::RequestArtifact)
            },
        ],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::Partial);
    assert!(plan.partial);
    assert!(!plan.runs_in_full);
    assert_eq!(plan.runnable_target_count, 1);
    assert_eq!(plan.blocked_target_count, 1);
    assert!(plan
        .why_blocked_tokens
        .contains(&"source_missing".to_owned()));
    // Partial offers a regenerate-ready-targets step plus the unblock step.
    let classes: BTreeSet<_> = plan.recovery.iter().map(|s| s.class).collect();
    assert!(classes.contains(&RecoveryClass::RegenerateReadyTargets));
    assert!(classes.contains(&RecoveryClass::RestoreCanonicalSource));
}

#[test]
fn missing_runtime_blocks_the_plan() {
    let req = request(
        "plan.blocked",
        "Rebuild the bundle",
        vec![TargetSpec {
            runtime_state: PreconditionState::Missing,
            ..TargetSpec::healthy(ArtifactClass::PreviewDerivative)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::Blocked);
    assert!(!plan.runs_in_full);
    assert_eq!(plan.runnable_target_count, 0);
    assert!(plan
        .why_blocked_tokens
        .contains(&"runtime_unavailable".to_owned()));
    assert_eq!(
        plan.recovery.first().map(|s| s.class),
        Some(RecoveryClass::ProvisionRuntime)
    );
}

#[test]
fn policy_block_is_policy_limited_not_blocked() {
    let req = request(
        "plan.policy",
        "Replay the request",
        vec![TargetSpec {
            policy_state: PreconditionState::BlockedByPolicy,
            ..TargetSpec::healthy(ArtifactClass::RequestArtifact)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::PolicyLimited);
    assert!(plan
        .why_blocked_tokens
        .contains(&"regeneration_blocked_by_policy".to_owned()));
    assert_eq!(
        plan.recovery.last().map(|s| s.class),
        Some(RecoveryClass::ResolveRegenerationPolicy)
    );
}

#[test]
fn undeclared_side_effect_holds_for_disclosure_and_never_runs_silently() {
    let req = request(
        "plan.undeclared",
        "Regenerate",
        vec![TargetSpec {
            side_effects: vec![
                local_compute(),
                side_effect(
                    SideEffectClass::NetworkInstall,
                    SideEffectDisclosure::Undeclared,
                ),
            ],
            ..TargetSpec::healthy(ArtifactClass::ScaffoldedProject)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::PolicyLimited);
    assert_eq!(plan.runnable_target_count, 0);
    assert_eq!(plan.targets[0].outcome, TargetOutcome::HeldForDisclosure);
    assert!(!plan.side_effect_boundary.all_sensitive_declared);
    assert!(plan
        .side_effect_boundary
        .undeclared_sensitive_classes
        .contains(&SideEffectClass::NetworkInstall));
    assert!(plan
        .why_blocked_tokens
        .iter()
        .any(|t| t == "undeclared_side_effect_network_install"));
    let classes: BTreeSet<_> = plan.recovery.iter().map(|s| s.class).collect();
    assert!(classes.contains(&RecoveryClass::DeclareAndReviewSideEffect));
}

#[test]
fn declared_global_side_effect_is_ready_but_only_partially_reversible() {
    let req = request(
        "plan.declared",
        "Regenerate",
        vec![TargetSpec {
            side_effects: vec![
                local_compute(),
                side_effect(
                    SideEffectClass::NetworkInstall,
                    SideEffectDisclosure::DeclaredReviewed,
                ),
            ],
            ..TargetSpec::healthy(ArtifactClass::FrameworkCodegen)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::Ready);
    assert!(plan.side_effect_boundary.all_sensitive_declared);
    // A declared install still escapes the checkpoint: honest rollback.
    assert_eq!(
        plan.rollback_coverage,
        RollbackCoverage::PartiallyReversible
    );
}

#[test]
fn secret_access_is_sensitive_but_does_not_reduce_reversibility() {
    let req = request(
        "plan.secret",
        "Regenerate",
        vec![TargetSpec {
            side_effects: vec![
                local_compute(),
                side_effect(
                    SideEffectClass::SecretAccess,
                    SideEffectDisclosure::DeclaredReviewed,
                ),
            ],
            ..TargetSpec::healthy(ArtifactClass::RequestArtifact)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.readiness, PlanReadiness::Ready);
    // Secret access writes nothing persistent, so reversibility is unaffected.
    assert_eq!(plan.rollback_coverage, RollbackCoverage::FullyReversible);
}

#[test]
fn hard_block_outranks_policy_in_target_outcome() {
    // A target that is both missing its source and policy-blocked is a hard
    // block: missing material is the more fundamental reason.
    let req = request(
        "plan.both",
        "Regenerate",
        vec![TargetSpec {
            source_state: PreconditionState::Missing,
            policy_state: PreconditionState::BlockedByPolicy,
            ..TargetSpec::healthy(ArtifactClass::NotebookOutput)
        }],
    );
    let plan = plan_regeneration(&req);
    assert_eq!(plan.targets[0].outcome, TargetOutcome::Blocked);
    assert_eq!(plan.readiness, PlanReadiness::Blocked);
    // Both reasons are still carried.
    assert!(plan
        .why_blocked_tokens
        .contains(&"source_missing".to_owned()));
    assert!(plan
        .why_blocked_tokens
        .contains(&"regeneration_blocked_by_policy".to_owned()));
}

#[test]
fn copy_line_is_stable_and_self_consistent() {
    let plan = plan_regeneration(&healthy_request(
        "regeneration-plan.scaffolded_project.ready",
        ArtifactClass::ScaffoldedProject,
    ));
    let expected = "regeneration-plan id=regeneration-plan.scaffolded_project.ready readiness=ready targets=1 runnable=1 blocked=0 side_effects=local_compute undeclared=false rollback=fully_reversible stale_inputs=false";
    assert_eq!(plan.copy_line, expected);
    assert_eq!(regeneration_plan_copy_line(&plan), expected);
}

#[test]
fn seeded_fixtures_validate_and_cover_every_readiness() {
    let fixtures = seeded_regeneration_plan_fixtures();
    assert!(!fixtures.is_empty());
    let mut readiness = BTreeSet::new();
    let mut saw_partial_rollback = false;
    let mut saw_undeclared = false;
    for fixture in &fixtures {
        validate_regeneration_plan_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        readiness.insert(fixture.expected_readiness);
        if fixture.expected_rollback_coverage == RollbackCoverage::PartiallyReversible {
            saw_partial_rollback = true;
        }
        if !fixture.expected_all_sensitive_declared {
            saw_undeclared = true;
        }
    }
    for required in PlanReadiness::ALL {
        assert!(
            readiness.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(
        saw_partial_rollback,
        "fixtures must cover a partial rollback"
    );
    assert!(
        saw_undeclared,
        "fixtures must cover an undeclared side effect"
    );
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_regeneration_plan_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: RegenerationPlanPacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}

#[test]
fn fixtures_round_trip_through_json() {
    for fixture in seeded_regeneration_plan_fixtures() {
        let json = serde_json::to_string(&fixture).expect("fixture serializes");
        let back: RegenerationPlanFixture =
            serde_json::from_str(&json).expect("fixture deserializes");
        assert_eq!(fixture, back);
    }
}
