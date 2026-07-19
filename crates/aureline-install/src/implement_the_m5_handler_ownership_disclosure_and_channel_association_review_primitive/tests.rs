//! Tests for the M5 handler-ownership primitive: the resolver, the parity matrix, and the
//! checked-in support export.

use super::*;

// --- resolver: AC1 side-by-side installs disclose which build owns and why ---

#[test]
fn resolver_preserves_ownership_identity_across_surfaces() {
    let input = diagnostics_handlers_input();
    let resolved = resolve_handler_ownership(&input).expect("resolves");
    assert_eq!(resolved.ownership_id, input.ownership_id);
    assert_eq!(resolved.disclosure_card.ownership_id, input.ownership_id);
    assert_eq!(resolved.recovery_alignment.ownership_id, input.ownership_id);
    assert!(resolved
        .association_rows
        .iter()
        .all(|row| row.ownership_id == input.ownership_id));
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_discloses_owner_and_precedence() {
    let resolved = resolve_handler_ownership(&about_integration_input()).expect("resolves");
    assert!(resolved.disclosure_card.discloses_current_owner);
    assert!(resolved.disclosure_card.discloses_precedence);
    assert_eq!(
        resolved.disclosure_card.owner_class,
        M5HandlerOwnerClass::PrimaryStableInstall
    );
    assert_eq!(
        resolved.disclosure_card.precedence_state,
        M5HandlerPrecedenceState::PrimaryAmongInstalls
    );
    assert!(resolved.has_contested_or_multi_install());
    assert!(!resolved.disclosure_card.ownership_reason.trim().is_empty());
    assert!(resolved.owner_and_precedence_disclosed());
}

#[test]
fn resolver_stays_inspectable_without_installer() {
    let resolved = resolve_handler_ownership(&about_integration_input()).expect("resolves");
    assert!(resolved.disclosure_card.inspectable_without_installer);
}

#[test]
fn resolver_rejects_hidden_owner() {
    let mut input = about_integration_input();
    input.discloses_current_owner = false;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::OwnerNotDisclosed)
    );
}

#[test]
fn resolver_rejects_manual_installer_inspection() {
    let mut input = about_integration_input();
    input.inspectable_without_installer = false;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::RequiresManualInstallerInspection)
    );
}

#[test]
fn resolver_rejects_missing_ownership_reason() {
    let mut input = about_integration_input();
    input.ownership_reason = "   ".to_owned();
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::OwnershipReasonMissing)
    );
}

#[test]
fn resolver_rejects_hidden_channel_owner() {
    let mut input = about_integration_input();
    input.channels[0].discloses_current_owner = false;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::ChannelOwnerHidden)
    );
}

// --- resolver: AC2 handler changes previewable and reversible ---

#[test]
fn resolver_keeps_bounded_actions_and_preview_for_changes() {
    let resolved = resolve_handler_ownership(&install_review_input()).expect("resolves");
    let reassign_row = resolved
        .association_rows
        .iter()
        .find(|row| row.change_state == M5HandlerChangeState::ReassignToThisInstall)
        .expect("reassign row exists");
    assert!(reassign_row
        .actions
        .contains(&M5ChannelAssociationAction::Keep));
    assert!(reassign_row
        .actions
        .contains(&M5ChannelAssociationAction::Reassign));
    assert!(reassign_row
        .actions
        .contains(&M5ChannelAssociationAction::Cancel));
    assert!(reassign_row
        .actions
        .contains(&M5ChannelAssociationAction::PreviewChange));
    assert!(reassign_row.previewable);
    assert!(reassign_row.reversible);
    assert!(resolved.has_proposed_change());
    assert!(resolved.changes_previewable_and_reversible());
}

#[test]
fn resolver_no_change_row_omits_preview_action() {
    let resolved = resolve_handler_ownership(&docs_reference_input()).expect("resolves");
    let row = &resolved.association_rows[0];
    assert_eq!(row.change_state, M5HandlerChangeState::NoChange);
    assert!(row.actions.contains(&M5ChannelAssociationAction::Keep));
    assert!(!row
        .actions
        .contains(&M5ChannelAssociationAction::PreviewChange));
}

#[test]
fn resolver_rejects_silent_takeover() {
    let mut input = install_review_input();
    input.channels[0].last_writer_wins_capture = true;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::SilentTakeover)
    );
}

#[test]
fn resolver_rejects_unreviewed_change() {
    let mut input = install_review_input();
    input.channels[0].reviewed_before_apply = false;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::ChannelChangeNotReviewed)
    );
}

#[test]
fn resolver_rejects_non_previewable_change() {
    let mut input = install_review_input();
    input.channels[0].previewable = false;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::ChangeNotPreviewable)
    );
}

#[test]
fn resolver_rejects_irreversible_change() {
    let mut input = install_review_input();
    input.channels[0].reversible = false;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::ChangeNotReversible)
    );
}

// --- resolver: AC3 ownership / precedence preserved with aligned recovery ---

#[test]
fn resolver_aligns_recovery_paths_with_rollback_identity() {
    let resolved = resolve_handler_ownership(&support_replay_input()).expect("resolves");
    assert!(resolved.has_recovery_path());
    assert_eq!(resolved.recovery_alignment.recovery_paths.len(), 4);
    assert!(resolved.recovery_alignment.all_paths_aligned_with_owner);
    assert!(
        resolved
            .recovery_alignment
            .all_paths_carry_rollback_identity
    );
    for path in &resolved.recovery_alignment.recovery_paths {
        assert!(path.aligned_with_channel_owner);
        assert!(path.carries_rollback_identity);
        assert!(path.channel_class.is_recovery_path());
    }
    assert!(resolved.ownership_precedence_preserved_in_export());
}

#[test]
fn resolver_rejects_misaligned_recovery_path() {
    let mut input = support_replay_input();
    // A recovery-class channel whose current owner differs from the disclosed install owner is
    // rejected as a misaligned recovery route.
    input.channels[0].current_owner_class = M5HandlerOwnerClass::ExternalNonAureline;
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::RecoveryPathMisaligned)
    );
}

#[test]
fn resolver_rejects_empty_rollback_identity() {
    let mut input = support_replay_input();
    input.rollback_identity_ref = "  ".to_owned();
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::EmptyRollbackIdentityRef)
    );
}

#[test]
fn resolver_non_recovery_channel_can_have_other_owner() {
    // A file-association (non-recovery) channel may legitimately name a different owner than the
    // disclosed install (a contested handover) without being rejected.
    let resolved = resolve_handler_ownership(&install_review_input()).expect("resolves");
    let protocol_row = resolved
        .association_rows
        .iter()
        .find(|row| row.channel_class == M5HandlerChannelClass::ProtocolHandler)
        .expect("protocol row exists");
    assert!(!protocol_row.is_recovery_path);
    assert_eq!(
        protocol_row.current_owner_class,
        M5HandlerOwnerClass::PrimaryStableInstall
    );
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_empty_ownership_id() {
    let input = M5HandlerOwnershipInput {
        ownership_id: "  ".to_owned(),
        ..about_integration_input()
    };
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::EmptyOwnershipId)
    );
}

#[test]
fn resolver_rejects_empty_install_identity_ref() {
    let input = M5HandlerOwnershipInput {
        install_identity_ref: String::new(),
        ..about_integration_input()
    };
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::EmptyInstallIdentityRef)
    );
}

#[test]
fn resolver_rejects_no_channels() {
    let input = M5HandlerOwnershipInput {
        channels: Vec::new(),
        ..about_integration_input()
    };
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::NoChannels)
    );
}

#[test]
fn resolver_rejects_empty_channel_ref() {
    let mut input = about_integration_input();
    input.channels[0].channel_ref = "   ".to_owned();
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::EmptyChannelRef)
    );
}

#[test]
fn resolver_rejects_empty_owner_ref() {
    let mut input = about_integration_input();
    input.channels[0].current_owner_ref = String::new();
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::EmptyOwnerRef)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5HandlerOwnershipInput {
        install_identity_ref: "install://desktop".to_owned(),
        ..about_integration_input()
    };
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5HandlerOwnershipInput {
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
            degraded_label: "unavailable".to_owned(),
        }),
        ..about_integration_input()
    };
    assert_eq!(
        resolve_handler_ownership(&input),
        Err(M5HandlerOwnershipResolutionError::DegradedLabelGeneric)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_handler_ownership_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_handler_ownership_packet();
    let present: BTreeSet<M5HandlerSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5HandlerSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_handler_ownership_packet();
    for row in &packet.surface_rows {
        for case in &row.example_cases {
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
    assert!(M5HandlerOwnershipVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_handler_ownership_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_handler_ownership_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5HandlerOwnershipViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_handler_ownership_packet();
    packet.surface_rows[0].shows_silent_takeover = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5HandlerOwnershipViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_handler_ownership_packet();
    packet.surface_rows[0].example_cases[0]
        .resolved
        .ownership_precedence_preserved_in_export = !packet.surface_rows[0].example_cases[0]
        .resolved
        .ownership_precedence_preserved_in_export;
    let violations = packet.validate();
    assert!(violations.contains(&M5HandlerOwnershipViolation::ExampleCaseDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_handler_ownership_packet();
    packet
        .vocabulary_set
        .precedence_states
        .push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5HandlerOwnershipViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_handler_ownership_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5HandlerOwnershipExportField::CurrentOwner);
    let violations = packet.validate();
    assert!(violations.contains(&M5HandlerOwnershipViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_handler_ownership_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_handler_ownership_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_handler_ownership_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-handler-ownership-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_handler_ownership_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_handler_ownership_packet();
    assert_eq!(packet.record_kind, M5_HANDLER_OWNERSHIP_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_HANDLER_OWNERSHIP_SCHEMA_VERSION);
}
