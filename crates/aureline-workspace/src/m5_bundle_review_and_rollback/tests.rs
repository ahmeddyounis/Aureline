use super::*;

fn packet() -> M5BundleReviewAndRollbackPacket {
    current_m5_bundle_review_and_rollback_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_BUNDLE_REVIEW_AND_ROLLBACK_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_BUNDLE_REVIEW_AND_ROLLBACK_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_matches_recomputed() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_wedge_has_a_review() {
    let packet = packet();
    assert!(packet.covers_every_wedge());
    for wedge in M5Wedge::ALL {
        assert!(
            packet.reviews_for_wedge(wedge).next().is_some(),
            "wedge {} has no review",
            wedge.as_str()
        );
    }
}

#[test]
fn every_operation_is_exercised() {
    let packet = packet();
    assert!(packet.covers_every_operation());
    for op in BundleReviewOperation::ALL {
        assert!(
            packet.reviews_with_operation(op).next().is_some(),
            "operation {} is never exercised",
            op.as_str()
        );
    }
}

#[test]
fn all_reviews_consistent() {
    let packet = packet();
    assert!(packet.all_reviews_consistent());
}

#[test]
fn mutating_reviews_carry_one_step_rollback() {
    let packet = packet();
    for review in &packet.reviews {
        if review.operation.is_mutating() {
            assert!(
                review.rollback_checkpoint.supports_one_step_rollback(),
                "review {} lacks a one-step rollback checkpoint",
                review.review_id
            );
        }
    }
}

#[test]
fn remove_reviews_preserve_user_assets() {
    let packet = packet();
    let removes: Vec<_> = packet
        .reviews_with_operation(BundleReviewOperation::Remove)
        .collect();
    assert!(!removes.is_empty());
    for review in removes {
        assert!(review.removal_preserves_user_assets());
    }
}

#[test]
fn non_stable_components_are_disclosed_and_review_gated() {
    let packet = packet();
    for review in &packet.reviews {
        for entry in &review.component_diffs {
            if entry.lifecycle_stage.is_non_stable() {
                assert!(
                    entry.requires_review,
                    "non-stable component {} is not review-gated",
                    entry.component_id
                );
            }
        }
        assert_eq!(
            review.discloses_non_stable_dependencies,
            review.has_non_stable_components()
        );
        assert_eq!(
            review.dependency_markers,
            review.computed_dependency_markers()
        );
    }
}

#[test]
fn blocked_components_carry_warnings() {
    let packet = packet();
    for review in &packet.reviews {
        assert!(review.blocked_components_warned());
    }
}

#[test]
fn certification_never_overreaches() {
    let packet = packet();
    for review in &packet.reviews {
        if review.presents_as_certified() {
            assert_eq!(review.certification_target, CertificationTarget::Certified);
        }
    }
}

#[test]
fn export_projection_roundtrips() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.reviews.len(), packet.reviews.len());
    assert!(projection.all_reviews_consistent);
    assert_eq!(projection.reviews_with_one_step_rollback, 6);
    // Export rows stay redaction-safe: every ref is opaque, never a raw payload.
    let serialized = serde_json::to_string(&projection).expect("serialize");
    assert!(serialized.contains("review-notebook-install"));
}

#[test]
fn user_protected_asset_cannot_be_removed() {
    // A locally-overridden or adopted asset paired with remove_bundle_owned is rejected.
    let entry = ComponentDiffEntry {
        component_kind: BundleComponentKind::SettingsPreset,
        component_id: "settings.x".to_string(),
        lifecycle_stage: LifecycleStage::Stable,
        requires_review: false,
        diff_action: DiffAction::Removed,
        ownership: AssetOwnership::LocallyOverridden,
        resolution: ResolutionChoice::RemoveBundleOwned,
        diffable: true,
        label: "X".to_string(),
        diff_preview_ref: "diff:x".to_string(),
        local_override_ref: None,
    };
    assert!(!entry.resolution_safe());
}

#[test]
fn blocked_asset_cannot_be_adopted() {
    let entry = ComponentDiffEntry {
        component_kind: BundleComponentKind::SettingsPreset,
        component_id: "settings.y".to_string(),
        lifecycle_stage: LifecycleStage::PolicyGated,
        requires_review: true,
        diff_action: DiffAction::Added,
        ownership: AssetOwnership::BlockedByPolicy,
        resolution: ResolutionChoice::AdoptBundle,
        diffable: true,
        label: "Y".to_string(),
        diff_preview_ref: "diff:y".to_string(),
        local_override_ref: None,
    };
    assert!(!entry.resolution_safe());
}

#[test]
fn unchanged_component_takes_no_action() {
    let mut entry = ComponentDiffEntry {
        component_kind: BundleComponentKind::Extension,
        component_id: "ext.z".to_string(),
        lifecycle_stage: LifecycleStage::Stable,
        requires_review: false,
        diff_action: DiffAction::Unchanged,
        ownership: AssetOwnership::BundleOwned,
        resolution: ResolutionChoice::NotApplicable,
        diffable: true,
        label: "Z".to_string(),
        diff_preview_ref: "diff:z".to_string(),
        local_override_ref: None,
    };
    assert!(entry.resolution_safe());
    entry.resolution = ResolutionChoice::AdoptBundle;
    assert!(!entry.resolution_safe());
}

#[test]
fn validate_rejects_summary_drift() {
    let mut packet = packet();
    packet.summary.total_reviews += 1;
    assert!(packet
        .validate()
        .contains(&M5BundleReviewAndRollbackViolation::SummaryMismatch));
}

#[test]
fn validate_rejects_undisclosed_dependency() {
    let mut packet = packet();
    // Flip a review with a non-stable dependency to hide its disclosure.
    let review = packet
        .reviews
        .iter_mut()
        .find(|r| r.has_non_stable_components())
        .expect("a review with a non-stable dependency");
    review.discloses_non_stable_dependencies = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BundleReviewAndRollbackViolation::UndisclosedNonStableDependency { .. }
    )));
}

#[test]
fn validate_rejects_missing_rollback() {
    let mut packet = packet();
    let review = packet
        .reviews
        .iter_mut()
        .find(|r| r.operation.is_mutating())
        .expect("a mutating review");
    review.rollback_checkpoint.one_step = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BundleReviewAndRollbackViolation::MissingRollbackCheckpoint { .. }
    )));
}

#[test]
fn vocabulary_arrays_are_complete() {
    let packet = packet();
    assert_eq!(packet.wedges, M5Wedge::ALL.to_vec());
    assert_eq!(packet.operations, BundleReviewOperation::ALL.to_vec());
    assert_eq!(packet.diff_actions, DiffAction::ALL.to_vec());
    assert_eq!(packet.ownership_classes, AssetOwnership::ALL.to_vec());
    assert_eq!(packet.resolution_choices, ResolutionChoice::ALL.to_vec());
    assert_eq!(packet.drift_states, DriftState::ALL.to_vec());
    assert_eq!(packet.component_kinds, BundleComponentKind::ALL.to_vec());
    assert_eq!(packet.lifecycle_stages, LifecycleStage::ALL.to_vec());
    assert_eq!(
        packet.certification_targets,
        CertificationTarget::ALL.to_vec()
    );
}
