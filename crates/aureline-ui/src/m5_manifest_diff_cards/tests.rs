use super::*;

fn seeded() -> ManifestDiffCard {
    current_m5_manifest_diff_card().expect("canonical manifest diff card loads and validates")
}

fn cloned() -> ManifestDiffCard {
    seeded()
}

#[test]
fn checked_in_manifest_diff_card_validates_clean() {
    let card = seeded();
    assert_eq!(card.record_kind, M5_MANIFEST_DIFF_CARD_RECORD_KIND);
    assert_eq!(card.schema_version, M5_MANIFEST_DIFF_CARD_SCHEMA_VERSION);
    assert!(card.validate().is_empty(), "{:?}", card.validate());
}

#[test]
fn card_discloses_manifest_scope_hooks_constraints_checkpoint_and_rollback() {
    let card = seeded();
    assert_eq!(card.manifest_identity.ecosystem, "node_pnpm");
    assert_eq!(card.manifest_identity.scope_kind, "selected_manifest");
    assert_eq!(card.change_summary.scripts_hooks_change_count, 1);
    assert_eq!(card.change_summary.constraint_change_count, 2);
    assert_eq!(card.scripts_hooks_preview[0].name, "postinstall");
    assert_eq!(
        card.scripts_hooks_preview[0].policy_label,
        "review_required"
    );
    assert_eq!(card.constraint_changes[1].compatibility_posture, "breaking");
    assert_eq!(card.checkpoint_state.state, "available");
    assert!(card.checkpoint_state.created_before_apply);
    assert_eq!(card.rollback_state.state, "compensating_only");
}

#[test]
fn apply_boundary_stages_instead_of_implying_direct_mutation() {
    let card = seeded();
    assert_eq!(card.apply_boundary.write_authority, "stages");
    assert_eq!(card.apply_boundary.mutation_posture, "stage_for_review");
    assert!(card.apply_boundary.review_required);
    assert_eq!(card.apply_boundary.disabled_reason, "not_applicable");
}

#[test]
fn projections_reuse_one_contract_for_package_review_health_companion_and_support() {
    let card = seeded();
    let package = card
        .projection_for("package_manager")
        .expect("package projection exists");
    let review = card
        .projection_for("review_pane")
        .expect("review projection exists");
    let health = card
        .projection_for("project_health_center")
        .expect("health projection exists");
    let companion = card
        .projection_for("companion_client")
        .expect("companion projection exists");
    let support = card
        .projection_for("support_export")
        .expect("support projection exists");

    for projection in [&package, &review, &health, &companion, &support] {
        assert_eq!(projection.card_id, card.card_id);
        assert_eq!(projection.manifest_diff_id, card.manifest_diff_id);
        assert_eq!(projection.manifest_scope, card.manifest_identity.scope_kind);
        assert_eq!(projection.ecosystem, card.manifest_identity.ecosystem);
        assert_eq!(
            projection.scripts_hooks_change_count,
            card.change_summary.scripts_hooks_change_count
        );
        assert_eq!(
            projection.constraint_change_count,
            card.change_summary.constraint_change_count
        );
        assert_eq!(projection.checkpoint_state, card.checkpoint_state.state);
        assert_eq!(projection.rollback_state, card.rollback_state.state);
        assert_eq!(
            projection.write_authority,
            card.apply_boundary.write_authority
        );
        assert_eq!(
            projection.mutation_posture,
            card.apply_boundary.mutation_posture
        );
        assert_eq!(projection.freshness_state, card.freshness_state);
        assert_eq!(projection.degraded_state, card.degraded_state);
    }
}

#[test]
fn export_copy_preserves_manifest_diff_truth() {
    let card = seeded();
    let exported = card.export_safe_json();
    for required in [
        "manifest-diff:pnpm-security-refresh",
        "package_operation:grouped-security-refresh:2026-07-02",
        "manifest:node:ui-package-json",
        "postinstall",
        "runtime:node",
        "available",
        "compensating_only",
        "stages",
    ] {
        assert!(exported.contains(required), "export dropped {required}");
    }
    for forbidden in ["api_key", "password", "bearer ", "raw manifest"] {
        assert!(!exported.to_lowercase().contains(forbidden));
    }
}

#[test]
fn changed_summary_that_drops_hook_rows_fails_validation() {
    let mut card = cloned();
    card.scripts_hooks_preview.clear();
    assert!(
        card.validate()
            .contains(&ManifestDiffCardViolation::ChangeSummaryMismatch),
        "{:?}",
        card.validate()
    );
}

#[test]
fn unsafe_direct_apply_without_mutating_authority_fails_validation() {
    let mut card = cloned();
    card.apply_boundary.mutation_posture = "direct_apply".to_owned();
    assert!(
        card.validate()
            .contains(&ManifestDiffCardViolation::ApplyBoundaryUnsafe),
        "{:?}",
        card.validate()
    );
}

#[test]
fn copy_export_that_drops_rollback_fails_validation() {
    let mut card = cloned();
    card.copy_export
        .export_fields
        .retain(|field| field != "rollback_state");
    assert!(
        card.validate()
            .contains(&ManifestDiffCardViolation::CopyExportDropsDiffTruth),
        "{:?}",
        card.validate()
    );
}
