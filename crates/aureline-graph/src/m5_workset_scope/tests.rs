use super::*;

fn packet() -> M5WorksetScopePacket {
    current_m5_workset_scope_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_WORKSET_SCOPE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_WORKSET_SCOPE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_body() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_consumer_surface_has_exactly_one_binding() {
    let packet = packet();
    assert_eq!(
        packet.consumer_bindings.len(),
        WorksetScopeConsumerSurface::ALL.len()
    );
    for surface in WorksetScopeConsumerSurface::ALL {
        assert!(
            packet.binding(surface).is_some(),
            "missing binding for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_binding_is_stamped_with_the_active_snapshot() {
    let packet = packet();
    assert!(packet.all_bindings_snapshot_bound());
    for binding in &packet.consumer_bindings {
        assert_eq!(binding.snapshot_id, packet.active_snapshot.snapshot_id);
        assert_eq!(binding.scope_id, packet.active_snapshot.descriptor.scope_id);
    }
}

#[test]
fn no_slice_implies_full_workspace() {
    // The guardrail: a sparse-slice binding never claims whole-workspace knowledge.
    let packet = packet();
    assert!(!packet.active_snapshot.is_full_workspace());
    assert!(!packet.any_slice_implies_full_workspace());
    for binding in &packet.consumer_bindings {
        assert!(
            !binding.implies_full_workspace,
            "binding {} implies full workspace over a sparse slice",
            binding.binding_id
        );
    }
}

#[test]
fn every_widen_action_and_suggestion_is_reviewable() {
    // No silent broadening: a widen action or a suggestion must be reviewable.
    let packet = packet();
    assert!(packet.all_widening_reviewable());
    for action in &packet.scope_change_actions {
        if action.direction.is_widen() || action.actuation.is_suggestion() {
            assert!(
                action.requires_review,
                "action {} widens or suggests scope but is not reviewable",
                action.action_id
            );
        }
    }
}

#[test]
fn an_explicit_widen_action_is_offered() {
    let packet = packet();
    assert!(packet.widen_actions().count() >= 1);
    assert!(packet.scope_change_actions.iter().any(|a| {
        a.direction == ScopeChangeDirection::Widen && a.actuation == ScopeChangeActuation::Explicit
    }));
}

#[test]
fn active_snapshot_discloses_hidden_and_not_loaded_counts() {
    // A sparse slice keeps hidden-result and not-loaded counts visible so the user can tell
    // how much of the workspace remains out of scope.
    let packet = packet();
    let descriptor = &packet.active_snapshot.descriptor;
    assert_eq!(descriptor.scope_mode, WorksetScopeMode::Sparse);
    assert!(descriptor.hidden_result_count > 0);
    assert!(descriptor.index_coverage.not_loaded_count > 0);
    assert!(!descriptor.included_roots_or_repos.is_empty());
}

#[test]
fn export_projection_reflects_body_and_guardrails() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.snapshot_id, packet.active_snapshot.snapshot_id);
    assert_eq!(
        projection.scope_id,
        packet.active_snapshot.descriptor.scope_id
    );
    assert_eq!(projection.bindings.len(), packet.consumer_bindings.len());
    assert!(projection.all_bindings_snapshot_bound);
    assert!(!projection.any_slice_implies_full_workspace);
    assert!(projection.all_widening_reviewable);
    for (binding, export) in packet
        .consumer_bindings
        .iter()
        .zip(projection.bindings.iter())
    {
        assert_eq!(export.surface, binding.surface.as_str());
        assert_eq!(export.snapshot_id, binding.snapshot_id);
        assert_eq!(
            export.implies_full_workspace,
            binding.implies_full_workspace
        );
    }
}

#[test]
fn packet_binds_to_canonical_upstream_packets() {
    let packet = packet();
    assert_eq!(
        packet.governance_matrix_ref,
        M5_WORKSET_SCOPE_GOVERNANCE_MATRIX_REF
    );
    assert_eq!(packet.source_packet_ref, M5_WORKSET_SCOPE_SOURCE_PACKET_REF);
}

#[test]
fn validate_flags_silent_broadening() {
    let mut packet = packet();
    if let Some(action) = packet
        .scope_change_actions
        .iter_mut()
        .find(|a| a.direction == ScopeChangeDirection::Widen)
    {
        action.requires_review = false;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5WorksetScopeViolation::SilentBroadening { .. })));
    }
}

#[test]
fn validate_flags_full_workspace_claim_over_slice() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.implies_full_workspace = true;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5WorksetScopeViolation::FullWorkspaceClaimOverSlice { .. }
        )));
    }
}

#[test]
fn validate_flags_snapshot_binding_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.snapshot_id = "workset-scope:snapshot:stale".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5WorksetScopeViolation::SnapshotBindingMismatch { .. })));
    }
}

#[test]
fn validate_flags_scope_id_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.scope_id = "workset:other".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5WorksetScopeViolation::ScopeIdMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_surface_binding() {
    let mut packet = packet();
    packet.consumer_bindings.pop();
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5WorksetScopeViolation::MissingSurfaceBinding { .. })));
}

#[test]
fn validate_flags_missing_explicit_widen_action() {
    let mut packet = packet();
    packet
        .scope_change_actions
        .retain(|a| a.direction != ScopeChangeDirection::Widen);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.contains(&M5WorksetScopeViolation::MissingExplicitWidenAction));
}

#[test]
fn validate_flags_full_scope_hiding_results() {
    let mut packet = packet();
    packet.active_snapshot.descriptor.scope_mode = WorksetScopeMode::Full;
    // The descriptor still reports a non-zero hidden count, which a full scope may not.
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5WorksetScopeViolation::FullScopeHidesResults { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_bindings = packet.summary.total_bindings.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5WorksetScopeViolation::SummaryMismatch));
}

#[test]
fn validate_flags_governance_ref_mismatch() {
    let mut packet = packet();
    packet.governance_matrix_ref = "artifacts/graph/m5/not-the-matrix.json".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&M5WorksetScopeViolation::GovernanceMatrixRefMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        WorksetScopeConsumerSurface::DocsRecall.as_str(),
        "docs_recall"
    );
    assert_eq!(
        WorksetScopeConsumerSurface::AiContextAssembly.as_str(),
        "ai_context_assembly"
    );
    assert_eq!(ScopeChangeDirection::Widen.as_str(), "widen");
    assert_eq!(ScopeChangeDirection::Narrow.as_str(), "narrow");
    assert_eq!(ScopeChangeActuation::Explicit.as_str(), "explicit");
    assert_eq!(ScopeChangeActuation::Suggested.as_str(), "suggested");
}

#[test]
fn consumer_surfaces_are_exhaustive_in_packet() {
    let packet = packet();
    let present: BTreeSet<WorksetScopeConsumerSurface> =
        packet.consumer_bindings.iter().map(|b| b.surface).collect();
    for surface in WorksetScopeConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "no binding exercises surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn change_vocabulary_is_exercised() {
    let packet = packet();
    let directions: BTreeSet<ScopeChangeDirection> = packet
        .scope_change_actions
        .iter()
        .map(|a| a.direction)
        .collect();
    let actuations: BTreeSet<ScopeChangeActuation> = packet
        .scope_change_actions
        .iter()
        .map(|a| a.actuation)
        .collect();
    for direction in ScopeChangeDirection::ALL {
        assert!(directions.contains(&direction));
    }
    for actuation in ScopeChangeActuation::ALL {
        assert!(actuations.contains(&actuation));
    }
}
