use super::*;

fn packet() -> M5MutationReviewMatrix {
    current_m5_mutation_review_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_MUTATION_REVIEW_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_MUTATION_REVIEW_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_path_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.paths.len(), packet.mutation_paths.len());
    for &path in &packet.mutation_paths {
        assert!(
            packet.path(path).is_some(),
            "missing row for path {}",
            path.as_str()
        );
    }
}

#[test]
fn every_path_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_paths_gate_consistent());
    for row in &packet.paths {
        assert_eq!(
            row.published_readiness,
            row.effective_readiness(),
            "path {} publishes beyond the gate",
            row.path_id
        );
        assert_eq!(
            row.review_decision,
            row.required_decision(),
            "path {} decision diverges from the gate",
            row.path_id
        );
        assert_eq!(
            row.narrowing_reasons,
            row.computed_narrowing_reasons(),
            "path {} narrowing reasons diverge from the gate",
            row.path_id
        );
    }
}

#[test]
fn every_path_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.paths {
        assert!(
            row.has_required_evidence(),
            "path {} is missing required evidence refs",
            row.path_id
        );
        assert!(
            !row.execution_ref.trim().is_empty(),
            "path {} has no execution ref to join in support exports",
            row.path_id
        );
        assert!(
            !row.mutation_receipt_ref.trim().is_empty(),
            "path {} has no mutation receipt for audit reconstruction",
            row.path_id
        );
    }
}

#[test]
fn conditional_refs_are_present_when_required() {
    let packet = packet();
    for row in &packet.paths {
        if row.approval_state.requires_ticket() {
            assert!(
                !row.approval_ticket_ref.trim().is_empty(),
                "path {} requires an approval ticket ref",
                row.path_id
            );
        }
        if row.rollback_class.requires_plan() {
            assert!(
                !row.rollback_plan_ref.trim().is_empty(),
                "path {} requires a rollback plan ref",
                row.path_id
            );
        }
        if row.route_effect.requires_expiry() {
            assert!(
                !row.route_expiry_ref.trim().is_empty(),
                "path {} requires a route expiry ref",
                row.path_id
            );
        }
    }
}

#[test]
fn handoff_paths_declare_a_handoff_continuity() {
    let packet = packet();
    for row in &packet.paths {
        if row.mutation_path.requires_handoff() {
            assert_ne!(
                row.handoff_continuity,
                HandoffContinuity::NotHandoff,
                "handoff path {} must declare a handoff continuity",
                row.path_id
            );
        }
    }
}

#[test]
fn narrowed_paths_offer_a_fallback() {
    let packet = packet();
    for row in &packet.paths {
        if row.review_decision.is_narrowed() {
            assert!(
                row.fallback_path.is_offered(),
                "narrowed path {} must offer a fallback",
                row.path_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.paths.len(), packet.paths.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_paths_gate_consistent,
        packet.all_paths_gate_consistent()
    );
    assert_eq!(
        projection.committable_count,
        packet.committable_paths().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_paths().count());
    assert_eq!(projection.withheld_count, packet.withheld_paths().count());
    for (row, export) in packet.paths.iter().zip(projection.paths.iter()) {
        assert_eq!(export.execution_ref, row.execution_ref);
        assert_eq!(export.committable, row.is_committable());
        assert_eq!(export.crosses_handoff, row.mutation_path.requires_handoff());
        assert_eq!(export.published_readiness, row.published_readiness.as_str());
    }
}

#[test]
fn published_readinesses_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CommitReadiness> =
        packet.paths.iter().map(|p| p.published_readiness).collect();
    for readiness in CommitReadiness::ALL {
        assert!(
            present.contains(&readiness),
            "no path publishes readiness {}",
            readiness.as_str()
        );
    }
}

#[test]
fn review_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ReviewDecision> =
        packet.paths.iter().map(|p| p.review_decision).collect();
    for decision in ReviewDecision::ALL {
        assert!(
            present.contains(&decision),
            "no path exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn actor_authority_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ActorAuthorityState> =
        packet.paths.iter().map(|p| p.actor_authority).collect();
    for state in ActorAuthorityState::ALL {
        assert!(
            present.contains(&state),
            "no path exercises actor-authority state {}",
            state.as_str()
        );
    }
}

#[test]
fn approval_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ApprovalState> = packet.paths.iter().map(|p| p.approval_state).collect();
    for state in ApprovalState::ALL {
        assert!(
            present.contains(&state),
            "no path exercises approval state {}",
            state.as_str()
        );
    }
}

#[test]
fn rollback_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RollbackClass> = packet.paths.iter().map(|p| p.rollback_class).collect();
    for class in RollbackClass::ALL {
        assert!(
            present.contains(&class),
            "no path exercises rollback class {}",
            class.as_str()
        );
    }
}

#[test]
fn route_effects_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RouteEffect> = packet.paths.iter().map(|p| p.route_effect).collect();
    for effect in RouteEffect::ALL {
        assert!(
            present.contains(&effect),
            "no path exercises route effect {}",
            effect.as_str()
        );
    }
}

#[test]
fn handoff_continuities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HandoffContinuity> =
        packet.paths.iter().map(|p| p.handoff_continuity).collect();
    for state in HandoffContinuity::ALL {
        assert!(
            present.contains(&state),
            "no path exercises handoff continuity {}",
            state.as_str()
        );
    }
}

#[test]
fn duration_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DurationClass> =
        packet.paths.iter().map(|p| p.expected_duration).collect();
    for class in DurationClass::ALL {
        assert!(
            present.contains(&class),
            "no path exercises duration class {}",
            class.as_str()
        );
    }
}

#[test]
fn fallback_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FallbackPath> = packet.paths.iter().map(|p| p.fallback_path).collect();
    for fallback in FallbackPath::ALL {
        assert!(
            present.contains(&fallback),
            "no path exercises fallback path {}",
            fallback.as_str()
        );
    }
}

#[test]
fn narrowing_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<MutationNarrowingReason> = packet
        .paths
        .iter()
        .flat_map(|p| p.narrowing_reasons.iter().copied())
        .collect();
    for reason in MutationNarrowingReason::ALL {
        assert!(
            present.contains(&reason),
            "no path exercises narrowing reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn committable_paths_are_clean() {
    let packet = packet();
    assert!(
        packet.committable_paths().count() > 0,
        "fixture needs a committable path"
    );
    for row in packet.committable_paths() {
        assert_eq!(row.capability_floor(), CommitReadiness::ReviewedApply);
        assert!(row.narrowing_reasons.is_empty());
        assert!(!row.handoff_continuity.is_flaggable());
        assert_eq!(row.published_readiness, CommitReadiness::ReviewedApply);
        assert_eq!(row.review_decision, ReviewDecision::Apply);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        ActorAuthorityState::Inherited.readiness_ceiling(),
        CommitReadiness::PreviewOnly
    );
    assert_eq!(
        ActorAuthorityState::Unestablished.readiness_ceiling(),
        CommitReadiness::Blocked
    );
    assert_eq!(
        ApprovalState::Required.readiness_ceiling(),
        CommitReadiness::ApprovalRequired
    );
    assert_eq!(
        ApprovalState::Bypassed.readiness_ceiling(),
        CommitReadiness::Blocked
    );
    assert_eq!(
        RollbackClass::Compensable.readiness_ceiling(),
        CommitReadiness::ApprovalRequired
    );
    assert_eq!(
        RollbackClass::ProviderManaged.readiness_ceiling(),
        CommitReadiness::PreviewOnly
    );
    assert_eq!(
        RollbackClass::Unknown.readiness_ceiling(),
        CommitReadiness::Blocked
    );
    assert_eq!(
        RouteEffect::Unbounded.readiness_ceiling(),
        CommitReadiness::PreviewOnly
    );
    assert_eq!(
        HandoffContinuity::Severed.readiness_ceiling(),
        CommitReadiness::Blocked
    );
}

#[test]
fn inherited_authority_never_commits() {
    // The guardrail: a path that only inherits authority from a trusted local shell can
    // never publish a reviewed apply.
    let packet = packet();
    let row = packet
        .path(MutationPath::RemoteMutation)
        .expect("remote-mutation row");
    assert_eq!(row.actor_authority, ActorAuthorityState::Inherited);
    assert_eq!(row.published_readiness, CommitReadiness::PreviewOnly);
    assert_ne!(row.review_decision, ReviewDecision::Apply);
    assert!(row
        .narrowing_reasons
        .contains(&MutationNarrowingReason::InheritedAuthority));
}

#[test]
fn preserved_handoff_can_apply() {
    // A browser/companion handoff that preserves authority and rollback semantics is
    // allowed to commit a reviewed apply — handoff is not a blanket block.
    let packet = packet();
    let row = packet
        .path(MutationPath::BrowserRuntimeAction)
        .expect("browser-runtime-action row");
    assert_eq!(row.handoff_continuity, HandoffContinuity::Preserved);
    assert_eq!(row.published_readiness, CommitReadiness::ReviewedApply);
    assert_eq!(row.review_decision, ReviewDecision::Apply);
    assert!(row.mutation_path.requires_handoff());
}

#[test]
fn renegotiated_handoff_is_flagged_for_review() {
    // A handoff that re-established authority must be flagged so it is not treated as an
    // approval bypass.
    let packet = packet();
    let row = packet
        .path(MutationPath::CompanionHandoff)
        .expect("companion-handoff row");
    assert_eq!(row.handoff_continuity, HandoffContinuity::Renegotiated);
    assert_eq!(row.review_decision, ReviewDecision::FlagForReview);
}

#[test]
fn severed_handoff_is_withheld() {
    // A handoff that would drop authority or rollback semantics blocks any native commit.
    let packet = packet();
    let row = packet
        .path(MutationPath::ProviderConsoleHandoff)
        .expect("provider-console-handoff row");
    assert_eq!(row.handoff_continuity, HandoffContinuity::Severed);
    assert_eq!(row.published_readiness, CommitReadiness::Blocked);
    assert_eq!(row.review_decision, ReviewDecision::Withhold);
    assert!(row
        .narrowing_reasons
        .contains(&MutationNarrowingReason::HandoffSevered));
    assert!(row.fallback_path.is_offered());
}

#[test]
fn bypassed_approval_is_withheld() {
    let packet = packet();
    let row = packet
        .path(MutationPath::TunnelRouteAction)
        .expect("tunnel-route-action row");
    assert!(row.approval_state.is_bypassed_trigger());
    assert_eq!(row.published_readiness, CommitReadiness::Blocked);
    assert_eq!(row.review_decision, ReviewDecision::Withhold);
    assert!(row
        .narrowing_reasons
        .contains(&MutationNarrowingReason::ApprovalBypassed));
    assert!(row
        .narrowing_reasons
        .contains(&MutationNarrowingReason::RollbackUnknown));
}

#[test]
fn validate_flags_overstated_readiness() {
    let mut packet = packet();
    if let Some(row) = packet
        .paths
        .iter_mut()
        .find(|p| p.effective_readiness() != CommitReadiness::ReviewedApply)
    {
        row.published_readiness = CommitReadiness::ReviewedApply;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MutationReviewViolation::OverstatedReadiness { .. })));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .paths
        .iter_mut()
        .find(|p| p.review_decision != ReviewDecision::Withhold)
    {
        row.review_decision = ReviewDecision::Withhold;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MutationReviewViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_narrowing_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.paths.iter_mut().find(|p| {
        !p.narrowing_reasons
            .contains(&MutationNarrowingReason::HandoffSevered)
    }) {
        row.narrowing_reasons
            .push(MutationNarrowingReason::HandoffSevered);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5MutationReviewViolation::NarrowingReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_handoff_context() {
    let mut packet = packet();
    if let Some(row) = packet
        .paths
        .iter_mut()
        .find(|p| p.mutation_path.requires_handoff())
    {
        row.handoff_continuity = HandoffContinuity::NotHandoff;
        // Re-align the gate so only the handoff-context violation remains in scope.
        row.published_readiness = row.effective_readiness();
        row.review_decision = row.required_decision();
        row.narrowing_reasons = row.computed_narrowing_reasons();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MutationReviewViolation::HandoffContextMissing { .. })));
    }
}

#[test]
fn validate_flags_missing_fallback() {
    let mut packet = packet();
    if let Some(row) = packet
        .paths
        .iter_mut()
        .find(|p| p.review_decision.is_narrowed())
    {
        row.fallback_path = FallbackPath::NoFallback;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5MutationReviewViolation::MissingFallback { .. })));
    }
}

#[test]
fn validate_flags_missing_path_row() {
    let mut packet = packet();
    let removed = packet.paths.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5MutationReviewViolation::MissingPathRow { .. })));
}

#[test]
fn validate_flags_unclaimed_path_row() {
    let mut packet = packet();
    packet
        .mutation_paths
        .retain(|p| *p != MutationPath::ProviderConsoleHandoff);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5MutationReviewViolation::UnclaimedPathRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5MutationReviewViolation::ClosedVocabularyMismatch {
            field: "mutation_paths"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_paths = packet.summary.total_paths.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5MutationReviewViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        MutationPath::RequestWorkspaceMutation.as_str(),
        "request_workspace_mutation"
    );
    assert_eq!(
        MutationPath::ProviderConsoleHandoff.as_str(),
        "provider_console_handoff"
    );
    assert_eq!(CommitReadiness::ReviewedApply.as_str(), "reviewed_apply");
    assert_eq!(CommitReadiness::Blocked.as_str(), "blocked");
    assert_eq!(ActorAuthorityState::Inherited.as_str(), "inherited");
    assert_eq!(ApprovalState::Bypassed.as_str(), "bypassed");
    assert_eq!(RollbackClass::ProviderManaged.as_str(), "provider_managed");
    assert_eq!(RouteEffect::Unbounded.as_str(), "unbounded");
    assert_eq!(HandoffContinuity::Severed.as_str(), "severed");
    assert_eq!(DurationClass::OpenEnded.as_str(), "open_ended");
    assert_eq!(FallbackPath::VendorConsole.as_str(), "vendor_console");
    assert_eq!(
        MutationNarrowingReason::InheritedAuthority.as_str(),
        "inherited_authority"
    );
    assert_eq!(ReviewDecision::FlagForReview.as_str(), "flag_for_review");
}

#[test]
fn readiness_rank_orders_low_to_high() {
    assert!(CommitReadiness::Blocked.rank() < CommitReadiness::PreviewOnly.rank());
    assert!(CommitReadiness::PreviewOnly.rank() < CommitReadiness::ApprovalRequired.rank());
    assert!(CommitReadiness::ApprovalRequired.rank() < CommitReadiness::ReviewedApply.rank());
    assert_eq!(
        CommitReadiness::ReviewedApply.min(CommitReadiness::PreviewOnly),
        CommitReadiness::PreviewOnly
    );
}
