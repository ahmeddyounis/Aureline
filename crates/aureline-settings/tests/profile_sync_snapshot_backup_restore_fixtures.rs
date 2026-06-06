//! Fixture replay and invariant tests for profile portability certification.

use std::collections::BTreeSet;

use aureline_settings::stabilize_profile_sync_snapshot_backup_restore::{
    is_canonical_object_ref, profile_sync_restore_corpus, ConflictClass, MergeRuleClass,
    ProfileSyncRestoreCertification, SnapshotClass, StableClaimClass, StateClass, SurfaceClass,
    PROFILE_SYNC_RESTORE_RECORD_KIND, PROFILE_SYNC_RESTORE_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/m4/stabilize-profile-sync-snapshot-backup-restore",
);

fn load_record(filename: &str) -> ProfileSyncRestoreCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_match_in_code_projection() {
    for scenario in profile_sync_restore_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        assert_eq!(
            on_disk,
            scenario.record(),
            "{} drifted",
            scenario.scenario_id
        );
    }
}

#[test]
fn record_identity_and_rollups_are_stable() {
    for scenario in profile_sync_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, PROFILE_SYNC_RESTORE_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            PROFILE_SYNC_RESTORE_SHARED_CONTRACT_REF
        );
        assert_eq!(
            record.stable_qualification.claim_class,
            scenario.expected_claim_class
        );
        assert_eq!(
            record.stable_qualification.qualifies_stable,
            scenario.expected_qualifies_stable
        );
    }
}

#[test]
fn four_snapshot_classes_have_required_metadata() {
    for scenario in profile_sync_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let classes: BTreeSet<SnapshotClass> = record
            .snapshots
            .iter()
            .map(|row| row.snapshot_class)
            .collect();
        for class in SnapshotClass::REQUIRED {
            assert!(
                classes.contains(&class),
                "{} missing {class:?}",
                scenario.scenario_id
            );
        }
        for snapshot in &record.snapshots {
            assert!(is_canonical_object_ref(&snapshot.snapshot_ref));
            assert!(!snapshot.snapshot_schema_version.is_empty());
            assert!(!snapshot.aureline_version.is_empty());
            assert!(!snapshot.platform_traits.is_empty());
            assert!(!snapshot.included_state_classes.is_empty());
            assert!(!snapshot.excluded_state_classes.is_empty());
            assert!(snapshot.integrity_hash.starts_with("sha256:"));
            assert!(!snapshot.source_provenance.is_empty());
        }
    }
}

#[test]
fn ordinary_roaming_lanes_exclude_volatile_and_secret_classes_or_narrow() {
    for scenario in profile_sync_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let ordinary_lanes = record
            .snapshots
            .iter()
            .filter(|row| row.snapshot_class.crosses_ordinary_roaming_lane());
        let leaks = ordinary_lanes
            .flat_map(|row| row.included_state_classes.iter())
            .any(|class| class.forbidden_in_ordinary_roaming());

        if record.stable_qualification.qualifies_stable {
            assert!(!leaks, "{} leaked forbidden state", scenario.scenario_id);
            for row in &record.snapshots {
                if row.snapshot_class.crosses_ordinary_roaming_lane() {
                    for excluded in StateClass::ORDINARY_ROAMING_EXCLUSIONS {
                        assert!(
                            row.excluded_state_classes.contains(&excluded),
                            "{} {:?} does not exclude {:?}",
                            scenario.scenario_id,
                            row.snapshot_class,
                            excluded
                        );
                    }
                }
            }
            assert!(record.pillars.secret_boundary_held);
        } else if leaks {
            assert!(!record.pillars.secret_boundary_held);
        }
    }
}

#[test]
fn conflict_corpus_covers_merge_rules_and_local_precedence() {
    let record = load_record("stable-profile-portability.json");
    let conflicts: BTreeSet<ConflictClass> = record.conflict_coverage.iter().copied().collect();
    for class in ConflictClass::REQUIRED {
        assert!(conflicts.contains(&class), "missing {class:?}");
    }
    let rules: BTreeSet<MergeRuleClass> = record.merge_rule_coverage.iter().copied().collect();
    for rule in [
        MergeRuleClass::FieldwiseMerge,
        MergeRuleClass::AdditiveMerge,
        MergeRuleClass::ExplicitConflictReview,
        MergeRuleClass::LocalPrecedence,
    ] {
        assert!(rules.contains(&rule), "missing {rule:?}");
    }
    assert!(record.merge_rules.iter().any(|row| {
        row.stale_remote
            && row.local_explicit_edit_wins
            && row.merge_rule == MergeRuleClass::LocalPrecedence
    }));
    assert!(record.pillars.merge_rules_enforced);
}

#[test]
fn restores_are_previewable_checkpointed_and_preserve_unmappables() {
    for scenario in profile_sync_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for preview in &record.restore_previews {
            assert!(is_canonical_object_ref(&preview.source_ref));
            assert!(is_canonical_object_ref(&preview.structured_change_set_ref));
            assert!(is_canonical_object_ref(
                &preview.cross_platform_unmappable_sidecar_ref
            ));
            assert!(preview.previewable_before_apply);
            assert!(preview.retained_vs_overwritten_explicit);
        }
        if record.stable_qualification.qualifies_stable {
            assert!(record.pillars.restore_preview_checkpointed);
            assert!(record.pillars.cross_platform_sidecar_preserved);
            assert!(record
                .restore_previews
                .iter()
                .all(|row| !row.rollback_checkpoint_ref.is_empty()));
        }
    }
}

#[test]
fn surfaces_show_required_pre_mutation_truth() {
    let record = load_record("stable-profile-portability.json");
    let surfaces: BTreeSet<SurfaceClass> = record
        .surface_truth
        .iter()
        .map(|row| row.surface_class)
        .collect();
    for surface in SurfaceClass::REQUIRED {
        assert!(surfaces.contains(&surface), "missing {surface:?}");
    }
    assert!(record.surface_truth.iter().all(|row| {
        row.consumes_shared_record
            && row.shows_source
            && row.shows_snapshot_class
            && row.shows_state_classes
            && row.shows_conflict_class
            && row.shows_rollback_checkpoint
            && row.shows_local_authoritative_fallback
    }));
}

#[test]
fn offboarding_package_explains_exports_retention_and_local_authority() {
    let record = load_record("stable-profile-portability.json");
    let retention = &record.offboarding_retention;
    assert!(retention.retention_inspectable);
    assert!((1..=90).contains(&retention.local_checkpoint_retention_days));
    assert!(is_canonical_object_ref(&retention.final_export_package_ref));
    assert!(is_canonical_object_ref(
        &retention.latest_successful_sync_manifest_ref
    ));
    assert!(!retention.profile_export_pointers.is_empty());
    assert!(retention
        .profile_export_pointers
        .iter()
        .all(|pointer| { is_canonical_object_ref(pointer) }));
    assert!(is_canonical_object_ref(&retention.extension_inventory_ref));
    assert!(is_canonical_object_ref(
        &retention.remaining_retention_timeline_ref
    ));
    assert!(retention.explainable_without_internal_logs);
    assert!(record.pillars.local_authority_retained);
    assert_eq!(
        record.stable_qualification.claim_class,
        StableClaimClass::Stable
    );
}
