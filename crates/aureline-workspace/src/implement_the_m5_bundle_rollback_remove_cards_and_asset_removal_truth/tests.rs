//! Tests for the M5 bundle rollback / remove primitive: the resolver, the parity matrix, and the
//! checked-in support export.

use super::*;

// --- resolver: AC1 non-destructive of user work ---

#[test]
fn resolver_preserves_removal_identity_across_surfaces() {
    let input = workspace_rollback_input();
    let resolved = resolve_bundle_removal(&input).expect("resolves");
    assert_eq!(resolved.removal_id, input.removal_id);
    assert_eq!(resolved.card.removal_id, input.removal_id);
    assert_eq!(resolved.inventory.removal_id, input.removal_id);
    assert_eq!(resolved.restore_path.removal_id, input.removal_id);
    assert!(resolved.identity_consistent());
}

#[test]
fn resolver_separates_created_from_user_owned() {
    let resolved = resolve_bundle_removal(&workspace_rollback_input()).expect("resolves");
    assert!(resolved.non_destructive_of_user_work());
    assert!(!resolved.card.implies_destructive_cleanup);
    assert!(resolved.inventory.separates_created_from_user_owned());
    assert_eq!(resolved.inventory.bundle_created_count, 1);
    assert_eq!(resolved.inventory.user_owned_count, 2);
    // A user profile and a user-authored file both survive.
    assert_eq!(resolved.card.kept_local.len(), 2);
    assert_eq!(resolved.card.reverted.len(), 1);
}

#[test]
fn resolver_attributes_created_versus_adopted_origins() {
    let resolved = resolve_bundle_removal(&workspace_rollback_input()).expect("resolves");
    assert!(!resolved.inventory.collapses_to_opaque_removal);
    assert!(resolved
        .inventory
        .origins_present
        .contains(&M5RemovalAssetOrigin::BundleCreated));
    assert!(resolved
        .inventory
        .origins_present
        .contains(&M5RemovalAssetOrigin::UserProfile));
    assert!(resolved
        .inventory
        .origins_present
        .contains(&M5RemovalAssetOrigin::UserCreatedFile));
}

#[test]
fn resolver_rejects_card_that_reads_like_destructive_cleanup() {
    let input = M5BundleRemovalInput {
        reads_like_destructive_cleanup: true,
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::ReadsLikeDestructiveCleanup)
    );
}

#[test]
fn resolver_rejects_reverting_user_asset_without_explicit_selection() {
    let mut input = workspace_rollback_input();
    // Force the user's profile to be reverted without an explicit selection.
    input.assets[1].disposition = M5RemovalDisposition::Reverted;
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::UserAssetNotPreserved)
    );
}

#[test]
fn resolver_allows_reverting_user_asset_when_explicitly_selected() {
    let resolved = resolve_bundle_removal(&diagnostics_explicit_remove_input()).expect("resolves");
    // The adopted package the user explicitly selected is reverted; the bundle-owned extension too.
    assert_eq!(resolved.card.reverted.len(), 2);
    assert!(resolved.card.kept_local.is_empty());
    assert!(resolved.non_destructive_of_user_work());
}

#[test]
fn resolver_rejects_empty_asset_inventory() {
    let input = M5BundleRemovalInput {
        assets: vec![],
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::EmptyAssetInventory)
    );
}

#[test]
fn resolver_rejects_dishonest_safe_to_remove_class() {
    let mut input = workspace_rollback_input();
    // Paint the user's profile as safe-to-remove.
    input.assets[1].safe_to_remove_class = M5SafeToRemoveClass::SafeToRemove;
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::AssetRowIncomplete)
    );
}

// --- resolver: AC2 remains / reverted / manual partition ---

#[test]
fn resolver_states_remains_reverted_and_manual() {
    let resolved = resolve_bundle_removal(&detail_remove_input()).expect("resolves");
    assert!(resolved.states_remains_reverted_manual());
    assert!(resolved.card.discloses_what_remains);
    assert!(resolved.card.discloses_what_is_reverted);
    assert!(resolved.card.discloses_manual_follow_up);
    assert_eq!(resolved.card.reverted.len(), 1);
    assert_eq!(resolved.card.manual_follow_up.len(), 1);
    assert!(resolved.card.kept_local.is_empty());
}

#[test]
fn manual_follow_up_asset_is_flagged_for_manual_handling() {
    let resolved = resolve_bundle_removal(&migration_rollback_preview_input()).expect("resolves");
    assert_eq!(resolved.card.manual_follow_up.len(), 1);
    assert_eq!(resolved.card.kept_local.len(), 1);
    assert!(resolved
        .inventory
        .safe_to_remove_classes_present
        .contains(&M5SafeToRemoveClass::RequiresManualHandling));
}

#[test]
fn resolver_rejects_manual_disposition_for_keep_local_class() {
    let mut input = extension_remove_input();
    // A keep-local class with a manual disposition is not honest.
    input.assets[1].disposition = M5RemovalDisposition::ManualFollowUp;
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::AssetRowIncomplete)
    );
}

// --- resolver: AC3 export-before-remove + checkpoint restore ---

#[test]
fn resolver_requires_checkpoint_for_mutating_remove() {
    let input = M5BundleRemovalInput {
        rollback_checkpoint: None,
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::MutatingOpWithoutCheckpoint)
    );
}

#[test]
fn resolver_creates_checkpoint_only_for_mutating_ops() {
    let mutating = resolve_bundle_removal(&workspace_rollback_input()).expect("resolves");
    assert!(mutating.restore_path.provides_checkpoint_restore);
    assert!(mutating.restore_path.rollback_checkpoint.is_some());

    let read_only = resolve_bundle_removal(&migration_rollback_preview_input()).expect("resolves");
    assert!(!read_only.restore_path.provides_checkpoint_restore);
    assert!(read_only.restore_path.rollback_checkpoint.is_none());
}

#[test]
fn resolver_requires_export_before_remove_when_narrowing() {
    // A read-only imported preview narrows portability truth; without an export it is rejected.
    let input = M5BundleRemovalInput {
        export_before_remove: None,
        ..migration_rollback_preview_input()
    };
    assert!(input.narrows_support_or_portability());
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::ExportBeforeRemoveMissing)
    );
}

#[test]
fn resolver_makes_export_available_on_read_only_preview() {
    let resolved = resolve_bundle_removal(&migration_rollback_preview_input()).expect("resolves");
    assert!(resolved.export_and_restore_available());
    assert!(resolved.restore_path.provides_export_before_remove);
    assert!(resolved.restore_path.narrows_support_or_portability);
    assert!(!resolved.restore_path.provides_checkpoint_restore);
}

#[test]
fn resolver_rejects_forced_reset_to_export() {
    let input = M5BundleRemovalInput {
        forces_reset_to_export: true,
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::ForcesResetToExport)
    );
}

// --- resolver: structural rules ---

#[test]
fn resolver_rejects_stale_claim_shown_as_current() {
    let input = M5BundleRemovalInput {
        claims_current_despite_stale: true,
        ..support_replay_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::StaleClaimShownAsCurrent)
    );
}

#[test]
fn resolver_rejects_empty_removal_id() {
    let input = M5BundleRemovalInput {
        removal_id: "  ".to_owned(),
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::EmptyRemovalId)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5BundleRemovalInput {
        surface_label: "https://mirror.example/removal".to_owned(),
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5BundleRemovalInput {
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::RollbackOnlyPath,
            degraded_label: "unsupported".to_owned(),
        }),
        ..workspace_rollback_input()
    };
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::DegradedLabelGeneric)
    );
}

#[test]
fn resolver_rejects_ownership_origin_mismatch() {
    let mut input = workspace_rollback_input();
    // A bundle-created asset owned as a user override is inconsistent.
    input.assets[0].ownership = AssetOwnership::LocallyOverridden;
    assert_eq!(
        resolve_bundle_removal(&input),
        Err(M5BundleRemovalResolutionError::AssetRowIncomplete)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_bundle_rollback_remove_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_bundle_rollback_remove_packet();
    let present: BTreeSet<M5BundleRemovalSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5BundleRemovalSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_bundle_rollback_remove_packet();
    for row in &packet.surface_rows {
        for case in &row.example_removals {
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
    assert!(M5BundleRemovalVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_bundle_rollback_remove_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_bundle_rollback_remove_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleRemovalViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_bundle_rollback_remove_packet();
    packet.surface_rows[0].collapses_to_opaque_removal = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleRemovalViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_bundle_rollback_remove_packet();
    packet.surface_rows[0].example_removals[0]
        .resolved
        .non_destructive_of_user_work = !packet.surface_rows[0].example_removals[0]
        .resolved
        .non_destructive_of_user_work;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleRemovalViolation::ExampleRemovalDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_bundle_rollback_remove_packet();
    packet.vocabulary_set.asset_origins.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleRemovalViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_bundle_rollback_remove_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5BundleRemovalExportField::ExportBeforeRemove);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleRemovalViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_bundle_rollback_remove_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_bundle_rollback_remove_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_bundle_rollback_remove_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-rollback-remove-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_bundle_rollback_remove_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_bundle_rollback_remove_packet();
    assert_eq!(packet.record_kind, M5_BUNDLE_REMOVAL_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_BUNDLE_REMOVAL_SCHEMA_VERSION);
}
