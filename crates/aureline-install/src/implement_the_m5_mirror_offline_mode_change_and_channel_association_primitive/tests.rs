//! Tests for the M5 mirror-transition primitive: the resolver, the parity matrix, and
//! the checked-in support export.

use super::*;

// --- resolver: AC1 offline / mirror transitions read explicitly ---

#[test]
fn resolver_preserves_transition_identity_across_surfaces() {
    let input = admin_disconnect_input();
    let resolved = resolve_mirror_transition(&input).expect("resolves");
    assert_eq!(resolved.transition_id, input.transition_id);
    assert_eq!(
        resolved.mode_change_sheet.transition_id,
        input.transition_id
    );
    assert_eq!(resolved.channel_row.transition_id, input.transition_id);
    assert!(resolved
        .artifact_rows
        .iter()
        .all(|row| row.transition_id == input.transition_id));
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_marks_offline_cache_only_state() {
    let resolved = resolve_mirror_transition(&admin_disconnect_input()).expect("resolves");
    let policy_row = resolved
        .artifact_rows
        .iter()
        .find(|row| row.artifact_class == M5MirrorArtifactClass::PolicyBundles)
        .expect("policy bundle row exists");
    assert_eq!(
        policy_row.continuity_state,
        M5MirrorContinuityState::OfflineCacheOnly
    );
    assert!(policy_row.continuity_state.is_stale_or_blocked());
    assert!(resolved.has_stale_or_blocked_artifact());
    assert!(resolved.mode_change_sheet.discloses_stale_and_usable);
    assert!(resolved.transition_explicit_not_generic());
    // The mode-change sheet's overall posture is the worst continuity across artifacts.
    assert_eq!(
        resolved.mode_change_sheet.artifact_posture,
        M5MirrorContinuityState::OfflineCacheOnly
    );
}

#[test]
fn resolver_marks_needs_refresh_for_stale_mirror() {
    let resolved = resolve_mirror_transition(&mirror_manager_input()).expect("resolves");
    let row = &resolved.artifact_rows[0];
    assert_eq!(row.continuity_state, M5MirrorContinuityState::NeedsRefresh);
    assert!(row.actions.contains(&M5MirrorArtifactAction::RefreshNow));
}

#[test]
fn resolver_marks_verification_failed() {
    let resolved = resolve_mirror_transition(&diagnostics_verify_input()).expect("resolves");
    let row = &resolved.artifact_rows[0];
    assert_eq!(
        row.continuity_state,
        M5MirrorContinuityState::VerificationFailed
    );
    assert!(row.continuity_state.is_blocked());
}

#[test]
fn resolver_marks_current_verified_for_live_source() {
    let resolved = resolve_mirror_transition(&docs_reference_input()).expect("resolves");
    let row = &resolved.artifact_rows[0];
    assert_eq!(
        row.continuity_state,
        M5MirrorContinuityState::CurrentVerified
    );
    assert!(!row.continuity_state.is_stale_or_blocked());
}

#[test]
fn resolver_rejects_stale_shown_as_current() {
    let mut input = mirror_manager_input();
    // A mirrored (non-current) artifact that claims to be shown as current is rejected.
    input.artifacts[0].stale_not_shown_as_current = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::StaleShownAsCurrent)
    );
}

// --- resolver: AC2 verification / manifests accessible ---

#[test]
fn resolver_keeps_verification_and_manifest_accessible() {
    let resolved = resolve_mirror_transition(&update_center_input()).expect("resolves");
    assert!(resolved.verification_accessible_across_profiles());
    for row in &resolved.artifact_rows {
        assert!(row.verification_accessible);
        assert!(row
            .actions
            .contains(&M5MirrorArtifactAction::VerifySignature));
        assert!(row.actions.contains(&M5MirrorArtifactAction::OpenManifest));
        assert!(!row.manifest_ref.trim().is_empty());
    }
}

#[test]
fn resolver_rejects_hidden_verification() {
    let mut input = update_center_input();
    input.artifacts[0].verify_available = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::VerificationNotAccessible)
    );
}

#[test]
fn resolver_rejects_hidden_manifest() {
    let mut input = update_center_input();
    input.artifacts[0].open_manifest_available = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::ManifestNotAccessible)
    );
}

// --- resolver: AC3 export-before-change and rollback preserved ---

#[test]
fn resolver_preserves_export_and_rollback() {
    let resolved = resolve_mirror_transition(&support_replay_input()).expect("resolves");
    assert!(resolved.export_and_rollback_preserved());
    assert!(resolved.mode_change_sheet.export_before_change_available);
    assert!(resolved.mode_change_sheet.reviewed_before_change);
    assert!(resolved.mode_change_sheet.reversible);
    assert!(resolved
        .mode_change_sheet
        .rollback_path_state
        .is_recoverable());
}

#[test]
fn resolver_rejects_unreviewed_change() {
    let mut input = support_replay_input();
    input.reviewed_before_change = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::ChangeNotReviewed)
    );
}

#[test]
fn resolver_rejects_blind_switch_without_export() {
    let mut input = support_replay_input();
    input.export_before_change_available = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::BlindSwitchWithoutExport)
    );
}

#[test]
fn resolver_rejects_last_writer_wins_capture() {
    let mut input = update_center_input();
    input.last_writer_wins_capture = true;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::LastWriterWinsCapture)
    );
}

#[test]
fn resolver_rejects_unreviewed_channel_change() {
    let mut input = update_center_input();
    input.reviewed_before_apply = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::ChannelChangeNotReviewed)
    );
}

#[test]
fn resolver_rejects_hidden_current_owner() {
    let mut input = update_center_input();
    input.discloses_current_owner = false;
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::CurrentOwnerHidden)
    );
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_transition_id() {
    let input = M5MirrorTransitionInput {
        transition_id: "  ".to_owned(),
        ..update_center_input()
    };
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::EmptyTransitionId)
    );
}

#[test]
fn resolver_rejects_empty_preserved_state_ref() {
    let input = M5MirrorTransitionInput {
        preserved_local_state_ref: String::new(),
        ..update_center_input()
    };
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::EmptyPreservedStateRef)
    );
}

#[test]
fn resolver_rejects_empty_channel_ref() {
    let input = M5MirrorTransitionInput {
        channel_ref: "  ".to_owned(),
        ..update_center_input()
    };
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::EmptyChannelRef)
    );
}

#[test]
fn resolver_rejects_no_artifacts() {
    let input = M5MirrorTransitionInput {
        artifacts: Vec::new(),
        ..update_center_input()
    };
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::NoArtifacts)
    );
}

#[test]
fn resolver_rejects_empty_artifact_ref() {
    let mut input = update_center_input();
    input.artifacts[0].artifact_ref = "   ".to_owned();
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::EmptyArtifactRef)
    );
}

#[test]
fn resolver_rejects_empty_manifest_ref() {
    let mut input = update_center_input();
    input.artifacts[0].manifest_ref = String::new();
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::EmptyManifestRef)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5MirrorTransitionInput {
        channel_ref: "channel://protocol-handler".to_owned(),
        ..update_center_input()
    };
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5MirrorTransitionInput {
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::MirrorStale,
            degraded_label: "offline".to_owned(),
        }),
        ..update_center_input()
    };
    assert_eq!(
        resolve_mirror_transition(&input),
        Err(M5MirrorTransitionResolutionError::DegradedLabelGeneric)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_mirror_transition_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_mirror_transition_packet();
    let present: BTreeSet<M5MirrorSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5MirrorSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_mirror_transition_packet();
    for row in &packet.surface_rows {
        for case in &row.example_transitions {
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
    assert!(M5MirrorTransitionVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_mirror_transition_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_mirror_transition_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5MirrorTransitionViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_mirror_transition_packet();
    packet.surface_rows[0].shows_stale_as_current = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5MirrorTransitionViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_mirror_transition_packet();
    packet.surface_rows[0].example_transitions[0]
        .resolved
        .export_and_rollback_preserved = !packet.surface_rows[0].example_transitions[0]
        .resolved
        .export_and_rollback_preserved;
    let violations = packet.validate();
    assert!(violations.contains(&M5MirrorTransitionViolation::ExampleTransitionDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_mirror_transition_packet();
    packet
        .vocabulary_set
        .continuity_states
        .push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5MirrorTransitionViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_mirror_transition_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5MirrorTransitionExportField::ContinuityState);
    let violations = packet.validate();
    assert!(violations.contains(&M5MirrorTransitionViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_mirror_transition_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_mirror_transition_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_mirror_transition_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-mirror-transition-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_mirror_transition_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_mirror_transition_packet();
    assert_eq!(packet.record_kind, M5_MIRROR_TRANSITION_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_MIRROR_TRANSITION_SCHEMA_VERSION);
}
