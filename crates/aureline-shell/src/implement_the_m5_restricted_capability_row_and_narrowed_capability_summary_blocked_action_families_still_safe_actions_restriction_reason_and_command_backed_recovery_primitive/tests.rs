use super::*;

fn clean_row_input() -> M5RestrictedCapabilityRowResolutionInput {
    M5RestrictedCapabilityRowResolutionInput {
        row_id: "restricted-row:test".to_owned(),
        object_identity: "workspace: test-app".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        root_trust: M5RootTrustState::RootRestricted,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![
            M5RestrictedActionFamily::CodeExecution,
            M5RestrictedActionFamily::TaskAutomation,
        ],
        still_safe_actions: vec![M5RestrictedActionFamily::ReadOnlyNavigation],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        reads_as_uniform_trust: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_summary_input() -> M5NarrowedCapabilitySummaryResolutionInput {
    M5NarrowedCapabilitySummaryResolutionInput {
        summary_id: "narrowed-summary:test".to_owned(),
        object_identity: "workspace: test-app".to_owned(),
        trust_scope: M5TrustScopeState::RestrictedWorkspace,
        grant_source: M5TrustGrantSourceClass::WorkspaceConfig,
        restriction_reason_stated: true,
        capability_narrow: M5CapabilityNarrowState::ExecutionBlocked,
        capability_narrow_stated: true,
        blocked_action_families: vec![M5RestrictedActionFamily::CodeExecution],
        still_safe_actions: vec![M5RestrictedActionFamily::ReadOnlyNavigation],
        approval_allowed: true,
        reads_as_generic_unavailable: false,
        collapses_blocked_families: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_restricted_capability_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_RESTRICTED_CAPABILITY_CONTROLS_PACKET_ID
    );
}

#[test]
fn row_clean_names_restriction_and_is_legible() {
    let resolved = resolve_restricted_capability_row(clean_row_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.restricted_posture_legible);
    assert!(!resolved.collapses_into_generic_unavailable);
    assert_eq!(resolved.restriction_scope, "restricted_workspace");
    assert_eq!(
        resolved.trust_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Restricted)
    );
    assert!(!resolved.blocked_action_families.is_empty());
    assert!(!resolved.still_safe_actions.is_empty());
    // Command-backed recovery is always anchored on inspect-trust.
    assert_eq!(
        resolved.recovery_actions.first(),
        Some(&M5RestrictedRecoveryAction::InspectTrust)
    );
    // A narrowed capability plus allowed approval adds continue-limited and request-approval.
    assert!(resolved
        .recovery_actions
        .contains(&M5RestrictedRecoveryAction::ContinueLimited));
    assert!(resolved
        .recovery_actions
        .contains(&M5RestrictedRecoveryAction::RequestApproval));
    assert_eq!(
        resolved.next_action,
        M5RestrictedRecoveryAction::InspectTrust
    );
}

#[test]
fn row_without_approval_omits_request_approval() {
    let mut input = clean_row_input();
    input.approval_allowed = false;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert!(!resolved
        .recovery_actions
        .contains(&M5RestrictedRecoveryAction::RequestApproval));
    assert!(!resolved.approval_available);
}

#[test]
fn row_object_unstated_degrades() {
    let mut input = clean_row_input();
    input.object_identity = "  ".to_owned();
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::ObjectIdentityUnstated)
    );
}

#[test]
fn row_scope_unknown_degrades_and_has_no_disposition() {
    let mut input = clean_row_input();
    input.trust_scope = M5TrustScopeState::ScopeUnknown;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::RestrictionScopeUnresolved)
    );
    assert_eq!(resolved.trust_disposition, None);
}

#[test]
fn row_source_unstated_degrades() {
    let mut input = clean_row_input();
    input.grant_source = M5TrustGrantSourceClass::GrantSourceUnknown;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::RestrictionSourceUnstated)
    );
}

#[test]
fn row_reason_unstated_degrades() {
    let mut input = clean_row_input();
    input.restriction_reason_stated = false;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::RestrictionReasonUnstated)
    );
}

#[test]
fn row_capability_unstated_degrades() {
    let mut input = clean_row_input();
    input.capability_narrow = M5CapabilityNarrowState::ExtensionBlocked;
    input.capability_narrow_stated = false;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert!(resolved.capability_narrowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::NarrowedCapabilityUnstated)
    );
}

#[test]
fn row_blocked_families_unstated_degrades() {
    let mut input = clean_row_input();
    input.blocked_action_families = Vec::new();
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::BlockedActionFamiliesUnstated)
    );
}

#[test]
fn row_still_safe_unstated_degrades() {
    let mut input = clean_row_input();
    input.still_safe_actions = Vec::new();
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::StillSafeActionsUnstated)
    );
}

#[test]
fn row_generic_unavailable_degrades() {
    let mut input = clean_row_input();
    input.reads_as_generic_unavailable = true;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.collapses_into_generic_unavailable);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::CollapsedIntoGenericUnavailable)
    );
}

#[test]
fn row_mixed_root_collapsed_degrades() {
    let mut input = clean_row_input();
    input.object_identity = "workspace: multi-root".to_owned();
    input.trust_scope = M5TrustScopeState::MixedRoot;
    input.root_trust = M5RootTrustState::RootMixedChildren;
    input.reads_as_uniform_trust = true;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert!(resolved.collapses_per_root_into_uniform);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::MixedRootCollapsedIntoUniform)
    );
}

#[test]
fn row_recovery_missing_degrades() {
    let mut input = clean_row_input();
    input.detail_command_available = false;
    let resolved = resolve_restricted_capability_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestrictedCapabilityRowDegradeReason::RecoveryPathMissing)
    );
}

#[test]
fn row_empty_id_and_forbidden_material_error() {
    let mut input = clean_row_input();
    input.row_id = "".to_owned();
    assert_eq!(
        resolve_restricted_capability_row(input).unwrap_err(),
        M5RestrictedCapabilityResolutionError::EmptyRowId
    );

    let mut input = clean_row_input();
    input.object_identity = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_restricted_capability_row(input).unwrap_err(),
        M5RestrictedCapabilityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn summary_clean_names_posture_and_counts() {
    let resolved = resolve_narrowed_capability_summary(clean_summary_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.posture_legible);
    assert_eq!(resolved.blocked_family_count, 1);
    assert_eq!(resolved.safe_action_count, 1);
    assert_eq!(
        resolved.recovery_actions.first(),
        Some(&M5RestrictedRecoveryAction::InspectTrust)
    );
}

#[test]
fn summary_blocked_families_collapsed_degrades() {
    let mut input = clean_summary_input();
    input.collapses_blocked_families = true;
    let resolved = resolve_narrowed_capability_summary(input).unwrap();
    assert!(resolved.collapses_blocked_families);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5NarrowedCapabilitySummaryDegradeReason::BlockedFamiliesCollapsedIntoGenericCount)
    );
}

#[test]
fn summary_generic_unavailable_degrades() {
    let mut input = clean_summary_input();
    input.reads_as_generic_unavailable = true;
    let resolved = resolve_narrowed_capability_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5NarrowedCapabilitySummaryDegradeReason::CollapsedIntoGenericUnavailable)
    );
}

#[test]
fn summary_recovery_missing_degrades() {
    let mut input = clean_summary_input();
    input.detail_command_available = false;
    let resolved = resolve_narrowed_capability_summary(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5NarrowedCapabilitySummaryDegradeReason::RecoveryPathMissing)
    );
}

#[test]
fn summary_empty_id_and_forbidden_material_error() {
    let mut input = clean_summary_input();
    input.summary_id = "   ".to_owned();
    assert_eq!(
        resolve_narrowed_capability_summary(input).unwrap_err(),
        M5RestrictedCapabilityResolutionError::EmptySummaryId
    );

    let mut input = clean_summary_input();
    input.object_identity = "bearer abc".to_owned();
    assert_eq!(
        resolve_narrowed_capability_summary(input).unwrap_err(),
        M5RestrictedCapabilityResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_restricted_capability_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.vocabulary_set.action_families.pop();
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RestrictedCapabilityAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5RestrictedCapabilityExportField::TrustDispositions);
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.controls_rows[0]
        .narrowed_capability_summary_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_row_example_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    // Force a clean row to also read as generic unavailable — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.restricted_capability_row_examples[0].degrade_reason = None;
    row.restricted_capability_row_examples[0].collapses_into_generic_unavailable = true;
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_restricted_capability_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.collapses_restricted_into_generic_unavailable = true,
            1 => row.hides_blocked_families_or_still_safe_actions = true,
            2 => row.routes_recovery_through_docs_or_logs_only = true,
            _ => row.implies_blanket_restriction_across_roots_or_routes = true,
        }
        assert!(packet
            .validate()
            .contains(&M5RestrictedCapabilityControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn no_generic_unavailable_not_proven_when_generic_example_removed() {
    let mut packet = seeded_m5_restricted_capability_controls();
    for row in &mut packet.controls_rows {
        row.restricted_capability_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5RestrictedCapabilityRowDegradeReason::CollapsedIntoGenericUnavailable)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::NoGenericUnavailableNotProven));
}

#[test]
fn no_generic_unavailable_not_proven_when_policy_blocked_scope_uncovered() {
    let mut packet = seeded_m5_restricted_capability_controls();
    // Drop every clean policy-blocked row so the required scope coverage breaks.
    for row in &mut packet.controls_rows {
        row.restricted_capability_row_examples
            .retain(|ex| !(ex.is_clean() && ex.restriction_scope == "policy_blocked"));
    }
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::NoGenericUnavailableNotProven));
}

#[test]
fn still_safe_and_recovery_not_proven_when_still_safe_example_removed() {
    let mut packet = seeded_m5_restricted_capability_controls();
    for row in &mut packet.controls_rows {
        row.restricted_capability_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5RestrictedCapabilityRowDegradeReason::StillSafeActionsUnstated)
        });
        row.narrowed_capability_summary_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5NarrowedCapabilitySummaryDegradeReason::StillSafeActionsUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::StillSafeAndRecoveryNotProven));
}

#[test]
fn still_safe_and_recovery_not_proven_when_recovery_example_removed() {
    let mut packet = seeded_m5_restricted_capability_controls();
    for row in &mut packet.controls_rows {
        row.restricted_capability_row_examples.retain(|ex| {
            ex.degrade_reason != Some(M5RestrictedCapabilityRowDegradeReason::RecoveryPathMissing)
        });
        row.narrowed_capability_summary_examples.retain(|ex| {
            ex.degrade_reason != Some(M5NarrowedCapabilitySummaryDegradeReason::RecoveryPathMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::StillSafeAndRecoveryNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet
        .governance_review
        .no_surface_collapses_into_generic_unavailable = false;
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet
        .consumer_projection
        .still_safe_actions_legible_without_docs = false;
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_restricted_capability_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RestrictedCapabilityControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_restricted_capability_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_restricted_capability_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_restricted_capability_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_restricted_capability_controls_export()
        .expect("checked M5 restricted-capability controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_RESTRICTED_CAPABILITY_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_restricted_capability_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::WorkspaceTrustUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Beta
    );

    let preview = seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5RestrictedCapabilityControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-restricted-capability-row-narrowed-capability-summary-controls/workspace_trust_ui_beta_narrowed.json"
    )))
    .expect("workspace-trust-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed()
    );

    let preview: M5RestrictedCapabilityControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-restricted-capability-row-narrowed-capability-summary-controls/safe_mode_ui_preview_narrowed.json"
    )))
    .expect("safe-mode-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_the_restricted_capability_row() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5WorkspaceTrustRepairComponentFamily::RestrictedCapabilityRow]
    );
}
