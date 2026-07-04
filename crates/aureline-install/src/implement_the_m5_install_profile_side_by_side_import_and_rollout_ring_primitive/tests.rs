//! Tests for the M5 deployment-profile primitive: the resolver, the parity matrix,
//! and the checked-in support export.

use super::*;

// --- resolver: AC1 identity + running-app owner / rollback disclosed ---

#[test]
fn resolver_preserves_deployment_identity_across_surfaces() {
    let input = desktop_per_user_running_input();
    let resolved = resolve_deployment_profile(&input).expect("resolves");
    assert_eq!(resolved.deployment_id, input.deployment_id);
    assert_eq!(resolved.install_card.deployment_id, input.deployment_id);
    assert_eq!(resolved.import_sheet.deployment_id, input.deployment_id);
    assert_eq!(resolved.rollout_row.deployment_id, input.deployment_id);
    assert!(resolved.identity_consistent());
    assert!(resolved.truth_class_consistent());
}

#[test]
fn resolver_discloses_running_owner_and_rollback_target() {
    let resolved = resolve_deployment_profile(&desktop_per_user_running_input()).expect("resolves");
    assert!(resolved.running_owner_disclosed());
    assert!(resolved.install_card.owns_running_app_disclosed);
    assert_eq!(resolved.install_card.build_ref, "build:stable:2026.7.0");
    assert_eq!(
        resolved.install_card.rollback_target,
        M5RollbackTargetState::CheckpointAvailable
    );
    assert_eq!(resolved.install_card.install_scope, M5InstallScope::PerUser);
    assert!(resolved.install_card.discloses_state_roots);
}

#[test]
fn resolver_marks_unknown_rollback_as_not_yet_disclosed() {
    let resolved =
        resolve_deployment_profile(&diagnostics_unknown_rollback_input()).expect("resolves");
    // The rollback target is kept explicit (Unknown), but that is not a disclosed
    // recoverable target, so running-owner disclosure is honestly false.
    assert_eq!(
        resolved.install_card.rollback_target,
        M5RollbackTargetState::Unknown
    );
    assert!(!resolved.running_owner_disclosed());
    assert!(resolved.degraded.is_some());
}

// --- resolver: AC2 shared-vs-isolated state explicit before handoff ---

#[test]
fn resolver_keeps_side_by_side_isolation_explicit_and_checkpointed() {
    let resolved = resolve_deployment_profile(&side_by_side_isolated_input()).expect("resolves");
    assert!(resolved.import_sheet.has_sibling);
    assert_eq!(
        resolved.import_sheet.state_sharing,
        M5StateSharingModel::OneTimeCopy
    );
    assert!(resolved.import_sheet.isolation_preserved);
    assert!(resolved.import_sheet.rollback_checkpoint_preserved);
    assert!(!resolved.import_sheet.handler_capture);
    assert!(resolved.state_sharing_explicit());
}

#[test]
fn resolver_discloses_shared_readonly_without_pretending_isolation() {
    let resolved =
        resolve_deployment_profile(&side_by_side_shared_readonly_input()).expect("resolves");
    assert_eq!(
        resolved.import_sheet.state_sharing,
        M5StateSharingModel::SharedReadOnly
    );
    // Shared read-only does not preserve isolation, but it is disclosed explicitly.
    assert!(!resolved.import_sheet.isolation_preserved);
    assert!(resolved.state_sharing_explicit());
}

#[test]
fn resolver_rejects_last_writer_wins_capture() {
    let input = M5DeploymentProfileInput {
        handler_capture: true,
        ..side_by_side_isolated_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::LastWriterWinsCapture)
    );
}

#[test]
fn resolver_rejects_sharing_without_sibling() {
    let input = M5DeploymentProfileInput {
        sibling_install_ref: None,
        ..side_by_side_isolated_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::SharingWithoutSibling)
    );
}

#[test]
fn resolver_rejects_state_move_without_checkpoint() {
    let input = M5DeploymentProfileInput {
        rollback_target: M5RollbackTargetState::NoRollback,
        ..side_by_side_isolated_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::StateMoveWithoutCheckpoint)
    );
}

// --- resolver: AC3 rollout ring identity preserved ---

#[test]
fn resolver_preserves_managed_ring_identity() {
    let resolved = resolve_deployment_profile(&managed_canary_rollout_input()).expect("resolves");
    assert!(resolved.rollout_row.managed);
    assert_eq!(resolved.rollout_row.ring, M5RolloutRing::Canary);
    assert_eq!(resolved.rollout_row.promotion_state, M5PromotionState::Held);
    assert!(!resolved.rollout_row.ring_owner_ref.trim().is_empty());
    assert!(!resolved.rollout_row.platform_scope_ref.trim().is_empty());
    assert!(resolved.ring_identity_preserved());
}

#[test]
fn resolver_rejects_flattened_managed_rollout() {
    let input = M5DeploymentProfileInput {
        ring_owner_ref: String::new(),
        ..managed_canary_rollout_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::RolloutIdentityFlattened)
    );
}

#[test]
fn resolver_allows_unmanaged_install_without_ring_owner() {
    let input = M5DeploymentProfileInput {
        managed_rollout: false,
        ring_owner_ref: String::new(),
        platform_scope_ref: String::new(),
        ..desktop_per_user_running_input()
    };
    let resolved = resolve_deployment_profile(&input).expect("resolves");
    assert!(!resolved.rollout_row.managed);
    // Nothing to flatten for an unmanaged install, so ring identity is preserved.
    assert!(resolved.ring_identity_preserved());
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_deployment_id() {
    let input = M5DeploymentProfileInput {
        deployment_id: "  ".to_owned(),
        ..desktop_per_user_running_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::EmptyDeploymentId)
    );
}

#[test]
fn resolver_rejects_empty_build_ref() {
    let input = M5DeploymentProfileInput {
        build_ref: String::new(),
        ..desktop_per_user_running_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::EmptyBuildRef)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5DeploymentProfileInput {
        channel_ref: "channel://stable".to_owned(),
        ..desktop_per_user_running_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5DeploymentProfileInput {
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::StateRootUnavailable,
            degraded_label: "unavailable".to_owned(),
        }),
        ..desktop_per_user_running_input()
    };
    assert_eq!(
        resolve_deployment_profile(&input),
        Err(M5DeploymentProfileResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_standalone_install_has_no_sibling() {
    let resolved = resolve_deployment_profile(&desktop_per_user_running_input()).expect("resolves");
    assert!(!resolved.import_sheet.has_sibling);
    assert_eq!(resolved.import_sheet.import_choice, M5ImportChoice::Skip);
    assert!(resolved.import_sheet.isolation_preserved);
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_deployment_profile_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_deployment_profile_packet();
    let present: BTreeSet<M5DeploymentSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5DeploymentSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_deployment_profile_packet();
    for row in &packet.surface_rows {
        for case in &row.example_profiles {
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
    assert!(M5DeploymentProfileVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_deployment_profile_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_deployment_profile_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentProfileViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_deployment_profile_packet();
    packet.surface_rows[0].flattens_rollout_identity = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentProfileViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_deployment_profile_packet();
    packet.surface_rows[0].example_profiles[0]
        .resolved
        .running_owner_disclosed = !packet.surface_rows[0].example_profiles[0]
        .resolved
        .running_owner_disclosed;
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentProfileViolation::ExampleProfileDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_deployment_profile_packet();
    packet
        .vocabulary_set
        .install_scopes
        .push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentProfileViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_deployment_profile_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5DeploymentProfileExportField::RollbackTarget);
    let violations = packet.validate();
    assert!(violations.contains(&M5DeploymentProfileViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_deployment_profile_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_deployment_profile_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_deployment_profile_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-profile-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_deployment_profile_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_deployment_profile_packet();
    assert_eq!(packet.record_kind, M5_DEPLOYMENT_PROFILE_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_DEPLOYMENT_PROFILE_SCHEMA_VERSION);
}
