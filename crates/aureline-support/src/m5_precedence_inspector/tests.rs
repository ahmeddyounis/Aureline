use super::*;

fn packet() -> M5PrecedenceInspectors {
    current_m5_precedence_inspectors().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_PRECEDENCE_INSPECTOR_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_PRECEDENCE_INSPECTOR_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_inspectors() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_family_has_at_least_one_inspector() {
    // Route, credential, setting, policy, and toolchain precedence all share this one grammar.
    let packet = packet();
    for family in PrecedenceFamily::ALL {
        assert!(
            packet.inspectors_for(family).next().is_some(),
            "missing inspector for family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.summary.families_covered, PrecedenceFamily::ALL.len());
}

#[test]
fn every_inspector_carries_one_step_explainability() {
    // Precedence is answerable from the active surface, Support Center, and CLI/headless in one step.
    let packet = packet();
    for inspector in &packet.inspectors {
        assert!(
            inspector.has_one_step_explainability(),
            "{} lacks one-step explainability",
            inspector.inspector_id
        );
        assert!(
            !inspector.explain_entrypoint_ref.trim().is_empty(),
            "{} has no explain entrypoint",
            inspector.inspector_id
        );
        assert!(
            !inspector.cli_object_ref.trim().is_empty(),
            "{} has no CLI-equivalent object",
            inspector.inspector_id
        );
    }
}

#[test]
fn every_inspector_shows_winner_and_overshadowed_and_lineage() {
    let packet = packet();
    for inspector in &packet.inspectors {
        assert!(
            inspector.candidates.len() >= 2,
            "{} shows fewer than two candidates",
            inspector.inspector_id
        );
        assert!(
            inspector.overshadowed_candidates().next().is_some(),
            "{} shows no overshadowed candidate",
            inspector.inspector_id
        );
        assert!(
            !inspector.affected_surfaces.is_empty(),
            "{} names no affected surface",
            inspector.inspector_id
        );
        assert!(
            inspector.has_required_evidence(),
            "{}",
            inspector.inspector_id
        );
        for candidate in &inspector.candidates {
            assert!(
                !candidate.descriptor_ref.trim().is_empty(),
                "{} candidate has no lineage ref",
                inspector.inspector_id
            );
        }
    }
}

#[test]
fn every_inspector_excludes_raw_material() {
    // No raw secret bodies or hidden policy payloads are ever carried.
    let packet = packet();
    for inspector in &packet.inspectors {
        assert!(
            inspector.raw_material_excluded,
            "{} does not exclude raw material",
            inspector.inspector_id
        );
    }
}

#[test]
fn every_inspector_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_inspectors_gate_consistent());
    for inspector in &packet.inspectors {
        assert_eq!(
            inspector.presentation,
            inspector.effective_presentation(),
            "{}",
            inspector.inspector_id
        );
        assert_eq!(
            inspector.downgrade_reasons,
            inspector.computed_downgrade_reasons(),
            "{}",
            inspector.inspector_id
        );
        assert_eq!(
            inspector.resolution_path,
            inspector.computed_resolution_path(),
            "{}",
            inspector.inspector_id
        );
        assert_eq!(
            inspector.blocked_before_use,
            inspector.effective_presentation().warns_before_use(),
            "{}",
            inspector.inspector_id
        );
    }
}

#[test]
fn transparent_inspectors_are_whole() {
    let packet = packet();
    let transparent = packet.transparent_inspectors().count();
    assert!(
        transparent >= 1,
        "fixture needs at least one transparent inspector to prove the gate is not a blanket flag"
    );
    for inspector in packet.transparent_inspectors() {
        assert_eq!(inspector.resolution_class, ResolutionClass::Resolved);
        assert_eq!(inspector.value_disclosure, ValueDisclosure::PlainValues);
        assert!(inspector.downgrade_reasons.is_empty());
        assert!(inspector.caveats.is_empty());
        assert!(inspector.unmet_or_blocked_sources.is_empty());
        assert!(!inspector.resolution_path.is_offered());
        assert_eq!(
            inspector.restart_reauth_posture,
            RestartReauthPosture::NoneNeeded
        );
        assert!(!inspector.policy_lock_state.is_locked());
        assert!(!inspector.blocked_before_use);
        // Even a clean resolution still shows what lost.
        assert!(inspector.overshadowed_candidates().next().is_some());
    }
}

#[test]
fn narrowed_and_blocked_inspectors_name_resolution_and_caveats() {
    let packet = packet();
    for inspector in &packet.inspectors {
        if inspector.effective_presentation().requires_attention() {
            assert!(!inspector.caveats.is_empty(), "{}", inspector.inspector_id);
        }
        if inspector.resolution_class.requires_resolution() {
            assert!(
                inspector.resolution_path.is_offered(),
                "{}",
                inspector.inspector_id
            );
            assert!(
                !inspector.unmet_or_blocked_sources.is_empty(),
                "{}",
                inspector.inspector_id
            );
        }
    }
}

#[test]
fn blocked_inspectors_warn_before_use() {
    let packet = packet();
    for inspector in packet.blocked_inspectors() {
        assert_eq!(inspector.presentation, InspectorPresentation::Blocked);
        assert!(
            inspector.blocked_before_use,
            "{} is blocked but does not warn before use",
            inspector.inspector_id
        );
    }
    for inspector in &packet.inspectors {
        if inspector.effective_presentation() != InspectorPresentation::Blocked {
            assert!(
                !inspector.blocked_before_use,
                "{} warns before use without being blocked",
                inspector.inspector_id
            );
        }
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in InspectorConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_inspectors_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.inspectors.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_inspectors_gate_consistent,
        packet.all_inspectors_gate_consistent()
    );
    assert_eq!(
        projection.transparent_count,
        packet.transparent_inspectors().count()
    );
    assert_eq!(
        projection.narrowed_count,
        packet.narrowed_inspectors().count()
    );
    assert_eq!(
        projection.blocked_count,
        packet.blocked_inspectors().count()
    );
    for (inspector, row) in packet.inspectors.iter().zip(projection.rows.iter()) {
        assert_eq!(row.presentation, inspector.presentation.as_str());
        assert_eq!(row.transparent, inspector.is_transparent());
        assert_eq!(row.winning_value_label, inspector.winning_value_label);
        assert_eq!(row.source_of_truth_ref, inspector.source_of_truth_ref);
        assert_eq!(
            row.affected_surface_count,
            inspector.affected_surface_count()
        );
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("support:m5:precedence", "2026-06-16T13:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    assert!(export.raw_material_excluded);
}

#[test]
fn presentations_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<InspectorPresentation> =
        packet.inspectors.iter().map(|i| i.presentation).collect();
    for decision in InspectorPresentation::ALL {
        assert!(
            present.contains(&decision),
            "no inspector exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn resolution_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ResolutionClass> = packet
        .inspectors
        .iter()
        .map(|i| i.resolution_class)
        .collect();
    for class in ResolutionClass::ALL {
        assert!(
            present.contains(&class),
            "no inspector exercises resolution {}",
            class.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<InspectorDowngradeReason> = packet
        .inspectors
        .iter()
        .flat_map(|i| i.downgrade_reasons.iter().copied())
        .collect();
    for reason in InspectorDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no inspector exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn resolution_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PrecedenceResolutionPath> = packet
        .inspectors
        .iter()
        .map(|i| i.resolution_path)
        .collect();
    for path in PrecedenceResolutionPath::ALL {
        assert!(
            present.contains(&path),
            "no inspector exercises {}",
            path.as_str()
        );
    }
}

#[test]
fn postures_and_sources_and_dispositions_are_exhaustive() {
    let packet = packet();
    let postures: BTreeSet<RestartReauthPosture> = packet
        .inspectors
        .iter()
        .map(|i| i.restart_reauth_posture)
        .collect();
    for posture in RestartReauthPosture::ALL {
        assert!(
            postures.contains(&posture),
            "no inspector exercises posture {}",
            posture.as_str()
        );
    }
    let sources: BTreeSet<PrecedenceSource> = packet
        .inspectors
        .iter()
        .flat_map(|i| i.candidates.iter().map(|c| c.source_class))
        .collect();
    for source in PrecedenceSource::ALL {
        assert!(
            sources.contains(&source),
            "no candidate exercises source {}",
            source.as_str()
        );
    }
    let dispositions: BTreeSet<CandidateDisposition> = packet
        .inspectors
        .iter()
        .flat_map(|i| i.candidates.iter().map(|c| c.disposition))
        .collect();
    for disposition in CandidateDisposition::ALL {
        assert!(
            dispositions.contains(&disposition),
            "no candidate exercises disposition {}",
            disposition.as_str()
        );
    }
}

#[test]
fn workspace_over_user_override_is_surfaced() {
    let packet = packet();
    let inspector = packet
        .inspector("m5-precedence-inspector:setting-workspace-over-user")
        .expect("setting inspector");
    assert_eq!(inspector.resolution_class, ResolutionClass::Override);
    assert_eq!(inspector.presentation, InspectorPresentation::Narrowed);
    assert_eq!(
        inspector.resolution_path,
        PrecedenceResolutionPath::ReviewOverride
    );
    assert!(inspector
        .downgrade_reasons
        .contains(&InspectorDowngradeReason::HiddenOverride));
    let winner = inspector.winner().expect("winner");
    assert_eq!(winner.source_class, PrecedenceSource::ProjectScoped);
    // The suppressed user value stays visible.
    assert!(inspector
        .overshadowed_candidates()
        .any(|c| c.source_class == PrecedenceSource::UserScoped));
}

#[test]
fn policy_over_user_lock_blocks_before_use() {
    let packet = packet();
    let inspector = packet
        .inspector("m5-precedence-inspector:policy-lock-blocked")
        .expect("policy inspector");
    assert_eq!(inspector.resolution_class, ResolutionClass::Blocked);
    assert_eq!(inspector.presentation, InspectorPresentation::Blocked);
    assert!(inspector.policy_lock_state.is_locked());
    assert!(inspector.blocked_before_use);
    assert_eq!(
        inspector.resolution_path,
        PrecedenceResolutionPath::RequestPolicyChange
    );
    // The user value is shown as blocked, not silently dropped.
    assert!(inspector
        .candidates
        .iter()
        .any(|c| c.disposition == CandidateDisposition::Blocked));
}

#[test]
fn credential_class_change_stays_metadata_only() {
    let packet = packet();
    let inspector = packet
        .inspector("m5-precedence-inspector:credential-class-change")
        .expect("credential inspector");
    assert_eq!(inspector.family, PrecedenceFamily::Credential);
    assert_eq!(inspector.value_disclosure, ValueDisclosure::MetadataOnly);
    assert!(inspector
        .downgrade_reasons
        .contains(&InspectorDowngradeReason::RedactionBoundary));
    assert_eq!(
        inspector.restart_reauth_posture,
        RestartReauthPosture::ReauthRequired
    );
    assert!(inspector.raw_material_excluded);
}

#[test]
fn route_target_drift_is_flagged_and_reconnects() {
    let packet = packet();
    let inspector = packet
        .inspector("m5-precedence-inspector:route-target-drift")
        .expect("route drift inspector");
    assert_eq!(inspector.resolution_class, ResolutionClass::Drift);
    assert_eq!(inspector.presentation, InspectorPresentation::Narrowed);
    assert_eq!(
        inspector.resolution_path,
        PrecedenceResolutionPath::ReconnectSource
    );
    assert_eq!(
        inspector.restart_reauth_posture,
        RestartReauthPosture::ReconnectRequired
    );
}

#[test]
fn hidden_fallback_is_eliminated() {
    let packet = packet();
    let inspector = packet
        .inspector("m5-precedence-inspector:toolchain-fallback")
        .expect("fallback inspector");
    assert_eq!(inspector.resolution_class, ResolutionClass::Fallback);
    assert!(inspector
        .downgrade_reasons
        .contains(&InspectorDowngradeReason::SilentFallbackEliminated));
    // The preferred, higher-precedence source is shown as unavailable — the fallback is forced, not silent.
    let winner = inspector.winner().expect("winner");
    assert!(inspector.candidates.iter().any(|c| {
        c.disposition == CandidateDisposition::Unavailable
            && c.source_class.rank() > winner.source_class.rank()
    }));
}

#[test]
fn route_conflict_declares_no_winner() {
    let packet = packet();
    let inspector = packet
        .inspector("m5-precedence-inspector:route-conflict")
        .expect("route conflict inspector");
    assert_eq!(inspector.resolution_class, ResolutionClass::Conflict);
    assert!(inspector.winner().is_none());
    assert!(
        inspector
            .candidates
            .iter()
            .filter(|c| c.disposition == CandidateDisposition::Conflicting)
            .count()
            >= 2
    );
    assert_eq!(
        inspector.resolution_path,
        PrecedenceResolutionPath::ReconcileConflict
    );
}

#[test]
fn validate_flags_overstated_presentation() {
    let mut packet = packet();
    if let Some(inspector) = packet
        .inspectors
        .iter_mut()
        .find(|i| i.effective_presentation() != InspectorPresentation::Transparent)
    {
        inspector.presentation = InspectorPresentation::Transparent;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5PrecedenceInspectorViolation::OverstatedPresentation { .. }
        )));
    }
}

#[test]
fn validate_flags_lower_precedence_silent_win() {
    // A lower-precedence source that wins under a "resolved" class without a fallback explanation is
    // exactly the silent fallback this gate exists to catch.
    let mut packet = packet();
    let inspector = packet
        .inspectors
        .iter_mut()
        .find(|i| i.inspector_id == "m5-precedence-inspector:setting-workspace-over-user")
        .expect("setting inspector");
    inspector.resolution_class = ResolutionClass::Resolved;
    // Now the project winner over the user candidate is fine for rank, but flip the winner to a lower
    // source to trigger the out-precede check.
    if let Some(winner) = inspector.candidates.iter_mut().find(|c| c.is_winner()) {
        winner.source_class = PrecedenceSource::FallbackScoped;
    }
    inspector.source_class = PrecedenceSource::FallbackScoped;
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5PrecedenceInspectorViolation::WinnerDoesNotOutrank { .. }
    )));
}

#[test]
fn validate_flags_metadata_only_wrong_family() {
    let mut packet = packet();
    if let Some(inspector) = packet
        .inspectors
        .iter_mut()
        .find(|i| i.family != PrecedenceFamily::Credential)
    {
        inspector.value_disclosure = ValueDisclosure::MetadataOnly;
        inspector
            .downgrade_reasons
            .push(InspectorDowngradeReason::RedactionBoundary);
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5PrecedenceInspectorViolation::MetadataOnlyWrongFamily { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_overshadowed_candidate() {
    let mut packet = packet();
    if let Some(inspector) = packet
        .inspectors
        .iter_mut()
        .find(|i| i.resolution_class == ResolutionClass::Resolved)
    {
        inspector.candidates.retain(|c| c.is_winner());
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5PrecedenceInspectorViolation::NoOvershadowedCandidate { .. }
                | M5PrecedenceInspectorViolation::TooFewCandidates { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_one_step_explainability() {
    let mut packet = packet();
    if let Some(inspector) = packet.inspectors.first_mut() {
        inspector.cli_object_ref = "  ".to_owned();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5PrecedenceInspectorViolation::MissingOneStepExplainability { .. }
        )));
    }
}

#[test]
fn validate_flags_blocked_inspector_that_does_not_warn() {
    let mut packet = packet();
    if let Some(inspector) = packet
        .inspectors
        .iter_mut()
        .find(|i| i.effective_presentation() == InspectorPresentation::Blocked)
    {
        inspector.blocked_before_use = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5PrecedenceInspectorViolation::BlockedBeforeUseMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_policy_lock_mismatch() {
    let mut packet = packet();
    if let Some(inspector) = packet
        .inspectors
        .iter_mut()
        .find(|i| i.resolution_class != ResolutionClass::Blocked)
    {
        inspector.policy_lock_state = PolicyLockState::Locked;
        assert!(packet
            .validate()
            .iter()
            .any(|v| matches!(v, M5PrecedenceInspectorViolation::PolicyLockMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != InspectorConsumerSurface::SupportCenter);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5PrecedenceInspectorViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_stops_preserving_object_ids() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.preserves_object_ids = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5PrecedenceInspectorViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_family() {
    let mut packet = packet();
    packet
        .inspectors
        .retain(|i| i.family != PrecedenceFamily::Policy);
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5PrecedenceInspectorViolation::MissingFamily { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_inspectors = packet.summary.total_inspectors.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5PrecedenceInspectorViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(PrecedenceFamily::Credential.as_str(), "credential");
    assert_eq!(PrecedenceSource::FallbackScoped.as_str(), "fallback_scoped");
    assert_eq!(ValueDisclosure::MetadataOnly.as_str(), "metadata_only");
    assert_eq!(ResolutionClass::Conflict.as_str(), "conflict");
    assert_eq!(InspectorPresentation::Narrowed.as_str(), "narrowed");
    assert_eq!(
        InspectorDowngradeReason::SilentFallbackEliminated.as_str(),
        "silent_fallback_eliminated"
    );
    assert_eq!(PrecedenceResolutionPath::NoneNeeded.as_str(), "none");
    assert_eq!(
        RestartReauthPosture::ReauthRequired.as_str(),
        "reauth_required"
    );
    assert_eq!(PolicyLockState::Locked.as_str(), "locked");
    assert_eq!(CandidateDisposition::Conflicting.as_str(), "conflicting");
    assert_eq!(
        InspectorConsumerSurface::IssueReportPacket.as_str(),
        "issue_report_packet"
    );
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        ResolutionClass::Resolved.presentation_ceiling(),
        InspectorPresentation::Transparent
    );
    assert_eq!(
        ResolutionClass::Override.presentation_ceiling(),
        InspectorPresentation::Narrowed
    );
    assert_eq!(
        ResolutionClass::Blocked.presentation_ceiling(),
        InspectorPresentation::Blocked
    );
    assert_eq!(
        ValueDisclosure::PlainValues.presentation_ceiling(),
        InspectorPresentation::Transparent
    );
    assert_eq!(
        ValueDisclosure::MetadataOnly.presentation_ceiling(),
        InspectorPresentation::Narrowed
    );
}
