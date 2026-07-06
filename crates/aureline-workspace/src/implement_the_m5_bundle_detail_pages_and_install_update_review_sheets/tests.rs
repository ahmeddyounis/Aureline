//! Tests for the M5 bundle detail / review primitive: the resolver, the parity matrix, and the
//! checked-in support export.

use super::*;

// --- resolver: AC1 change fully disclosed ---

#[test]
fn resolver_preserves_review_identity_across_surfaces() {
    let input = detail_certified_input();
    let resolved = resolve_bundle_review(&input).expect("resolves");
    assert_eq!(resolved.review_id, input.review_id);
    assert_eq!(resolved.detail_page.review_id, input.review_id);
    assert_eq!(resolved.review_sheet.review_id, input.review_id);
    assert!(resolved.identity_consistent());
    assert!(resolved.dependency_markers_consistent());
}

#[test]
fn resolver_lists_full_inventory_and_enumerates_every_change() {
    let resolved = resolve_bundle_review(&detail_certified_input()).expect("resolves");
    assert!(resolved.detail_page.lists_full_inventory);
    assert_eq!(resolved.detail_page.component_inventory.len(), 5);
    assert!(resolved.review_sheet.enumerates_every_change);
    assert!(resolved.review_sheet.discloses_diff_scope);
    assert!(resolved.change_fully_disclosed());
    assert!(resolved.has_decision_requiring_change());
}

#[test]
fn resolver_rejects_hidden_change() {
    let input = M5BundleReviewInput {
        claims_no_change_despite_diff: true,
        ..detail_certified_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::HiddenChange)
    );
}

#[test]
fn resolver_rejects_empty_component_inventory() {
    let input = M5BundleReviewInput {
        component_inventory: vec![],
        ..detail_certified_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::EmptyComponentInventory)
    );
}

// --- resolver: AC2 intelligible under constraint + posture ---

#[test]
fn resolver_marks_ready_to_apply_for_unblocked_mutating_review() {
    let resolved = resolve_bundle_review(&install_web_input()).expect("resolves");
    assert_eq!(
        resolved.review_sheet.review_posture,
        M5BundleReviewPosture::ReadyToApply
    );
    assert!(resolved.review_sheet.creates_rollback_checkpoint);
    assert!(resolved.intelligible_under_constraint());
}

#[test]
fn resolver_marks_constrained_by_policy_when_asset_blocked() {
    let resolved = resolve_bundle_review(&policy_constrained_update_input()).expect("resolves");
    assert_eq!(
        resolved.review_sheet.review_posture,
        M5BundleReviewPosture::ConstrainedByPolicy
    );
    assert!(resolved.review_sheet.review_posture.is_mutating());
    assert!(resolved.intelligible_under_constraint());
    assert!(resolved.degraded.is_some());
}

#[test]
fn resolver_marks_read_only_comparison_for_drift_review() {
    let resolved = resolve_bundle_review(&drift_review_input()).expect("resolves");
    assert_eq!(
        resolved.review_sheet.review_posture,
        M5BundleReviewPosture::ReadOnlyComparison
    );
    assert!(!resolved.review_sheet.creates_rollback_checkpoint);
    assert!(resolved.review_sheet.rollback_checkpoint.is_none());
}

#[test]
fn resolver_stays_intelligible_under_offline_truth_mode() {
    let resolved = resolve_bundle_review(&offline_update_input()).expect("resolves");
    assert_eq!(
        resolved.review_sheet.truth_mode,
        M5BundleTruthMode::CachedOffline
    );
    assert!(!resolved.review_sheet.truth_mode.is_current_source());
    assert!(resolved.intelligible_under_constraint());
}

// --- resolver: rollback checkpoint + safety rules ---

#[test]
fn resolver_requires_checkpoint_for_mutating_op() {
    let input = M5BundleReviewInput {
        rollback_checkpoint: None,
        ..install_web_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::MutatingOpWithoutCheckpoint)
    );
}

#[test]
fn resolver_rejects_checkpoint_that_does_not_support_one_step_rollback() {
    let input = M5BundleReviewInput {
        rollback_checkpoint: Some(RollbackCheckpoint {
            checkpoint_ref: "checkpoint:bad".to_owned(),
            one_step: false,
            reversible: true,
            captured_before_mutation: true,
            captured_component_count: 1,
        }),
        ..install_web_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::MutatingOpWithoutCheckpoint)
    );
}

#[test]
fn resolver_rejects_unsafe_resolution() {
    let mut input = install_web_input();
    // Remove a bundle-owned asset that is actually user-protected (locally overridden).
    input.diff_rows[0].ownership = AssetOwnership::LocallyOverridden;
    input.diff_rows[0].resolution = ResolutionChoice::RemoveBundleOwned;
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::UnsafeResolution)
    );
}

#[test]
fn resolver_rejects_dependency_marker_hidden() {
    let mut input = install_web_input();
    // The inventory carries a policy-gated component; drop the policy marker.
    input
        .dependency_markers
        .retain(|marker| *marker != M5BundleDependencyMarker::PolicyGated);
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::DependencyMarkerHidden)
    );
}

// --- resolver: stale claim + structural rules ---

#[test]
fn resolver_rejects_stale_claim_shown_as_current() {
    let input = M5BundleReviewInput {
        claims_current_despite_stale: true,
        ..imported_migration_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::StaleClaimShownAsCurrent)
    );
}

#[test]
fn resolver_rejects_empty_review_id() {
    let input = M5BundleReviewInput {
        review_id: "  ".to_owned(),
        ..detail_certified_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::EmptyReviewId)
    );
}

#[test]
fn resolver_rejects_empty_changelog_ref() {
    let input = M5BundleReviewInput {
        changelog_ref: String::new(),
        ..detail_certified_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::EmptyChangelogRef)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    let input = M5BundleReviewInput {
        changelog_ref: "https://mirror.example/changelog".to_owned(),
        ..detail_certified_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_rejects_generic_degraded_label() {
    let input = M5BundleReviewInput {
        degraded: Some(DegradedState {
            trigger: M5BundleComponentDowngradeTrigger::StaleCertification,
            degraded_label: "unsupported".to_owned(),
        }),
        ..detail_certified_input()
    };
    assert_eq!(
        resolve_bundle_review(&input),
        Err(M5BundleReviewResolutionError::DegradedLabelGeneric)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_bundle_detail_review_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_bundle_detail_review_packet();
    let present: BTreeSet<M5BundleReviewSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5BundleReviewSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_cases_are_self_consistent() {
    let packet = seeded_m5_bundle_detail_review_packet();
    for row in &packet.surface_rows {
        for case in &row.example_reviews {
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
    assert!(M5BundleReviewVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_bundle_detail_review_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_bundle_detail_review_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleReviewViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_bundle_detail_review_packet();
    packet.surface_rows[0].hides_diff_scope = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleReviewViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_case_is_flagged() {
    let mut packet = seeded_m5_bundle_detail_review_packet();
    packet.surface_rows[0].example_reviews[0]
        .resolved
        .change_fully_disclosed = !packet.surface_rows[0].example_reviews[0]
        .resolved
        .change_fully_disclosed;
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleReviewViolation::ExampleReviewDrift));
}

#[test]
fn vocabulary_drift_is_flagged() {
    let mut packet = seeded_m5_bundle_detail_review_packet();
    packet.vocabulary_set.diff_actions.push("bogus".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleReviewViolation::VocabularySetDrift));
}

#[test]
fn mandatory_export_field_missing_is_flagged() {
    let mut packet = seeded_m5_bundle_detail_review_packet();
    packet.surface_rows[0]
        .export_fields
        .retain(|f| *f != M5BundleReviewExportField::DiffScope);
    let violations = packet.validate();
    assert!(violations.contains(&M5BundleReviewViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_bundle_detail_review_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_bundle_detail_review_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_bundle_detail_review_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-bundle-detail-review-primitive-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_bundle_detail_review_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_bundle_detail_review_packet();
    assert_eq!(packet.record_kind, M5_BUNDLE_REVIEW_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_BUNDLE_REVIEW_SCHEMA_VERSION);
}
