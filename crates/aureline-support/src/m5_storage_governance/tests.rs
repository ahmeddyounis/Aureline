use super::*;

fn matrix() -> M5ArtifactFamilyStorageMatrix {
    current_m5_artifact_family_storage_matrix().expect("matrix parses")
}

fn profiles() -> RuntimeStorageClassProfiles {
    current_runtime_storage_class_profiles().expect("runtime profiles parse")
}

#[test]
fn matrix_parses_and_covers_every_family() {
    let matrix = matrix();
    assert_eq!(
        matrix.rows.len(),
        REQUIRED_FAMILIES.len(),
        "matrix must list every required family exactly once"
    );
    for required in REQUIRED_FAMILIES {
        assert!(
            matrix.family(*required).is_some(),
            "missing family {}",
            required.as_str()
        );
    }
}

#[test]
fn matrix_validates_against_runtime_profiles() {
    let matrix = matrix();
    let profiles = profiles();
    let violations = matrix.validate(&profiles);
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn runtime_profiles_cover_all_six_classes() {
    let profiles = profiles();
    for class in [
        StorageClassId::InteractiveHotCache,
        StorageClassId::KnowledgeCache,
        StorageClassId::ArtifactCache,
        StorageClassId::PrebuildEnvironmentCache,
        StorageClassId::EvidenceSupportCache,
        StorageClassId::UserOwnedRecoveryState,
    ] {
        assert!(
            profiles.get(class).is_some(),
            "missing profile for {}",
            class.as_str()
        );
    }
}

#[test]
fn low_disk_eviction_order_is_early_to_late() {
    let matrix = matrix();
    let order = matrix.low_disk_eviction_order();
    assert_eq!(order.len(), matrix.rows.len());

    // The order is monotonically non-decreasing by ladder position, the hot
    // cache is trimmed before user-owned recovery state, and user-owned
    // recovery state is dead last.
    let mut prev = 0;
    for row in &order {
        let pos = row.low_disk_ladder_step.ladder_order();
        assert!(pos >= prev, "ladder order must be non-decreasing");
        prev = pos;
    }
    assert_eq!(
        order.first().unwrap().family_id,
        ArtifactFamilyId::GeneratedPreview
    );
    assert_eq!(
        order.last().unwrap().family_id,
        ArtifactFamilyId::UserOwnedRecoveryState
    );
}

#[test]
fn protected_families_never_admit_a_generic_clear() {
    let matrix = matrix();
    for row in &matrix.rows {
        if !row.protected_continuity {
            continue;
        }
        assert!(
            row.export_before_delete_required,
            "{} must require export-before-delete",
            row.family_id.as_str()
        );
        let plan = matrix
            .clear_data_plan_for(row.family_id)
            .expect("plan exists");
        for action in &plan.allowed_clear_data_actions {
            assert!(
                !matches!(
                    action,
                    ClearDataActionClass::GenericClearInBulk
                        | ClearDataActionClass::GenericClearExcludingPins
                        | ClearDataActionClass::ClassSelectiveClear
                ),
                "{} must not admit a generic clear action",
                row.family_id.as_str()
            );
        }
    }
}

#[test]
fn offboarding_reset_never_silently_disposes_protected_state() {
    let matrix = matrix();
    let plan = matrix.offboarding_reset_plan();
    // User-owned recovery state and every evidence family are export-gated.
    assert!(plan
        .export_before_delete
        .contains(&ArtifactFamilyId::UserOwnedRecoveryState));
    assert!(plan
        .export_before_delete
        .contains(&ArtifactFamilyId::ReviewIncidentEvidence));
    assert!(plan
        .export_before_delete
        .contains(&ArtifactFamilyId::ProfilerTrace));
    // Disposed-without-review never contains a protected family.
    for family in &plan.disposed_without_review {
        let row = matrix.family(*family).expect("row exists");
        assert!(
            !row.protected_continuity,
            "{} must not be silently disposed",
            family.as_str()
        );
    }
    // Every family appears in exactly one bucket.
    assert_eq!(
        plan.disposed_without_review.len() + plan.export_before_delete.len(),
        matrix.rows.len()
    );
}

#[test]
fn support_export_is_metadata_safe_and_family_complete() {
    let matrix = matrix();
    let export = matrix.support_export(
        "support_export.m5_storage_governance.v1",
        "2026-06-14T00:00:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.rows.len(), REQUIRED_FAMILIES.len());
    assert!(!export.raw_content_exported);
    // The envelope round-trips through serde without losing fields.
    let json = serde_json::to_string(&export).expect("serialize");
    let back: M5StorageGovernanceSupportExport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(export, back);
}

#[test]
fn support_export_matches_checked_in_golden() {
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_artifact_family_storage_matrix/support_export.golden.json"
    ));
    let matrix = matrix();
    let export = matrix.support_export(
        "support_export.m5_storage_governance.v1",
        "2026-06-14T00:00:00Z",
    );
    let golden: M5StorageGovernanceSupportExport =
        serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support \
         --example dump_m5_storage_governance_support_export`"
    );
}

#[test]
fn mutating_a_protected_family_to_generic_clear_is_rejected() {
    let mut matrix = matrix();
    let profiles = profiles();
    // Force the user-owned recovery row to admit a bulk generic clear.
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.family_id == ArtifactFamilyId::UserOwnedRecoveryState)
        .expect("row exists");
    row.allowed_clear_data_actions = vec![ClearDataActionClass::GenericClearInBulk];
    let violations = matrix.validate(&profiles);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.clear_actions.never_generic"),
        "expected a never_generic violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_row_to_an_inadmissible_authority_is_rejected() {
    let mut matrix = matrix();
    let profiles = profiles();
    // Generated previews live in interactive_hot_cache, which only admits
    // disposable_derived_cache authority.
    let row = matrix
        .rows
        .iter_mut()
        .find(|row| row.family_id == ArtifactFamilyId::GeneratedPreview)
        .expect("row exists");
    row.authority_class = AuthorityClass::UserOwnedRecoveryState;
    let violations = matrix.validate(&profiles);
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.authority_not_admissible"),
        "expected an authority violation, got {violations:#?}"
    );
}
