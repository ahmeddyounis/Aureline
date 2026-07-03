use super::*;

fn seeded() -> DependencyRow {
    current_m5_dependency_row().expect("canonical dependency row loads and validates")
}

fn cloned() -> DependencyRow {
    seeded()
}

#[test]
fn checked_in_dependency_row_validates_clean() {
    let row = seeded();
    assert_eq!(row.record_kind, M5_DEPENDENCY_ROW_RECORD_KIND);
    assert_eq!(row.schema_version, M5_DEPENDENCY_ROW_SCHEMA_VERSION);
    assert!(row.validate().is_empty(), "{:?}", row.validate());
}

#[test]
fn row_discloses_package_ecosystem_relation_versions_and_scope() {
    let row = seeded();
    assert_eq!(row.package_name, "openssl-sys");
    assert_eq!(row.manifest_identity.ecosystem, "cargo");
    assert_eq!(row.dependency_relation, "transitive");
    assert_eq!(row.version_delta.current_version_repr, "0.9.98");
    assert_eq!(row.version_delta.target_version_repr, "0.9.104");
    assert_eq!(row.manifest_identity.scope_kind, "selected_manifest");
}

#[test]
fn lockfile_impact_and_update_state_remain_visible() {
    let row = seeded();
    assert_eq!(row.lockfile_impact.impact_state, "manifest_and_lockfile");
    assert_eq!(
        row.lockfile_impact.lockfile_ref,
        "lockfile:cargo:Cargo.lock"
    );
    assert_eq!(row.lockfile_impact.affected_entries, 1);
    assert!(row.lockfile_impact.review_required);
    assert_eq!(row.update_state, "limited");
    assert_ne!(row.degraded_state, "none");
}

#[test]
fn advisory_changelog_and_license_actions_are_available_from_row() {
    let row = seeded();
    assert_eq!(row.advisory_summary.advisory_count, 2);
    assert_eq!(row.advisory_summary.highest_severity, "high");
    assert_eq!(row.license_action.action_state, "available");
    assert!(row.license_action.available);
    assert_eq!(row.changelog_action.action_state, "available");
    assert!(row.changelog_action.available);
}

#[test]
fn projections_reuse_one_contract_for_package_review_health_companion_and_support() {
    let row = seeded();
    let package = row
        .projection_for("package_manager")
        .expect("package projection exists");
    let review = row
        .projection_for("review_pane")
        .expect("review projection exists");
    let health = row
        .projection_for("project_health_center")
        .expect("health projection exists");
    let companion = row
        .projection_for("companion_client")
        .expect("companion projection exists");
    let support = row
        .projection_for("support_export")
        .expect("support projection exists");

    for projection in [&package, &review, &health, &companion, &support] {
        assert_eq!(projection.row_id, row.row_id);
        assert_eq!(projection.package_name, row.package_name);
        assert_eq!(projection.ecosystem, row.manifest_identity.ecosystem);
        assert_eq!(projection.dependency_relation, row.dependency_relation);
        assert_eq!(projection.manifest_scope, row.manifest_identity.scope_kind);
        assert_eq!(
            projection.current_version,
            row.version_delta.current_version_repr
        );
        assert_eq!(
            projection.target_version,
            row.version_delta.target_version_repr
        );
        assert_eq!(projection.delta_class, row.version_delta.delta_class);
        assert_eq!(projection.lockfile_impact, row.lockfile_impact.impact_state);
        assert_eq!(
            projection.advisory_count,
            row.advisory_summary.advisory_count
        );
        assert_eq!(
            projection.license_action_state,
            row.license_action.action_state
        );
        assert_eq!(
            projection.changelog_action_state,
            row.changelog_action.action_state
        );
        assert_eq!(projection.update_state, row.update_state);
        assert_eq!(projection.freshness_state, row.freshness_state);
        assert_eq!(projection.degraded_state, row.degraded_state);
    }
}

#[test]
fn export_copy_preserves_dependency_truth() {
    let row = seeded();
    let exported = row.export_safe_json();
    for required in [
        "openssl-sys",
        "cargo",
        "transitive",
        "selected_manifest",
        "0.9.98",
        "0.9.104",
        "manifest_and_lockfile",
        "advisory_summary",
        "license_action",
        "changelog_action",
        "limited",
    ] {
        assert!(exported.contains(required), "export dropped {required}");
    }
    for forbidden in ["api_key", "password", "bearer ", "raw advisory"] {
        assert!(!exported.to_lowercase().contains(forbidden));
    }
}

#[test]
fn constrained_state_hidden_as_none_fails_validation() {
    let mut row = cloned();
    row.update_state = "policy_constrained".to_owned();
    row.degraded_state = "none".to_owned();
    assert!(
        row.validate()
            .contains(&DependencyRowViolation::ConstrainedUpdateStateHidden),
        "{:?}",
        row.validate()
    );
}

#[test]
fn missing_lockfile_impact_fails_validation() {
    let mut row = cloned();
    row.lockfile_impact.impact_state = "manifest_and_lockfile".to_owned();
    row.lockfile_impact.lockfile_ref = "lockfile:none".to_owned();
    assert!(
        row.validate()
            .contains(&DependencyRowViolation::MissingLockfileImpact),
        "{:?}",
        row.validate()
    );
}

#[test]
fn copy_export_that_drops_actions_fails_validation() {
    let mut row = cloned();
    row.copy_export
        .export_fields
        .retain(|f| f != "changelog_action");
    assert!(
        row.validate()
            .contains(&DependencyRowViolation::CopyExportDropsDependencyTruth),
        "{:?}",
        row.validate()
    );
}
