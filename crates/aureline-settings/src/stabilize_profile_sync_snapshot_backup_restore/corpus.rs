//! Deterministic profile sync, backup, restore, and offboarding corpus.
//!
//! The corpus pins the stable contract and three failure drills so restore,
//! export, support, and offboarding surfaces can replay the same evidence.

use super::model::{
    ConflictClass, MergeRuleClass, MergeRuleRow, MergeSubjectClass, OffboardingRetentionSummary,
    ProfileSyncRestoreCertification, ProfileSyncRestoreInput, RestorePreviewRow,
    SecretBoundaryAuditRow, SnapshotClass, SnapshotManifestRow, StableClaimClass, StateClass,
    SurfaceClass, SurfaceTruthRow,
};

/// Timestamp pinned for every record in this corpus.
pub const CORPUS_AS_OF: &str = "2026-06-06T22:45:00Z";

/// One deterministic scenario in the profile portability corpus.
#[derive(Debug, Clone)]
pub struct ProfileSyncRestoreScenario {
    /// Stable scenario id.
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable verdict.
    pub expected_qualifies_stable: bool,
    record: ProfileSyncRestoreCertification,
}

impl ProfileSyncRestoreScenario {
    /// Returns the canonical record for this scenario.
    pub fn record(&self) -> ProfileSyncRestoreCertification {
        self.record.clone()
    }
}

struct ScenarioSpec {
    scenario_id: &'static str,
    summary: &'static str,
    leak_secret_material: bool,
    omit_restore_checkpoint: bool,
    managed_sync_required: bool,
}

/// Returns the deterministic corpus for the profile portability contract.
pub fn profile_sync_restore_corpus() -> Vec<ProfileSyncRestoreScenario> {
    [
        ScenarioSpec {
            scenario_id: "stable_profile_portability",
            summary: "Profile sync, backup, restore, and offboarding are previewable, checkpointed, local-authoritative, and secret-safe.",
            leak_secret_material: false,
            omit_restore_checkpoint: false,
            managed_sync_required: false,
        },
        ScenarioSpec {
            scenario_id: "secret_boundary_violation",
            summary: "A managed sync snapshot attempts to include secret material and is narrowed below stable.",
            leak_secret_material: true,
            omit_restore_checkpoint: false,
            managed_sync_required: false,
        },
        ScenarioSpec {
            scenario_id: "restore_without_checkpoint",
            summary: "A restore preview would overwrite local state without a rollback checkpoint and is narrowed below stable.",
            leak_secret_material: false,
            omit_restore_checkpoint: true,
            managed_sync_required: false,
        },
        ScenarioSpec {
            scenario_id: "managed_sync_required_for_local_work",
            summary: "Managed sync is incorrectly required for local launch and editing and is narrowed below stable.",
            leak_secret_material: false,
            omit_restore_checkpoint: false,
            managed_sync_required: true,
        },
    ]
    .into_iter()
    .map(build_scenario)
    .collect()
}

fn build_scenario(spec: ScenarioSpec) -> ProfileSyncRestoreScenario {
    let record = ProfileSyncRestoreCertification::build(ProfileSyncRestoreInput {
        record_id: format!("profile_sync_restore:{id}", id = spec.scenario_id),
        as_of: CORPUS_AS_OF.to_owned(),
        summary: spec.summary.to_owned(),
        snapshots: snapshots(spec.leak_secret_material),
        merge_rules: merge_rules(spec.omit_restore_checkpoint),
        restore_previews: restore_previews(spec.omit_restore_checkpoint),
        secret_boundary_audit: secret_boundary_audit(),
        surface_truth: surface_truth(),
        offboarding_retention: offboarding_retention(spec.managed_sync_required),
    })
    .expect("scenario builds");

    ProfileSyncRestoreScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id.replace('_', "-")),
        expected_claim_class: record.stable_qualification.claim_class,
        expected_qualifies_stable: record.stable_qualification.qualifies_stable,
        record,
    }
}

fn snapshots(leak_secret_material: bool) -> Vec<SnapshotManifestRow> {
    let ordinary_exclusions = vec![
        StateClass::DirtyBufferJournals,
        StateClass::SessionRestoreState,
        StateClass::Caches,
        StateClass::Indexes,
        StateClass::SecretMaterial,
    ];
    let portable_includes = vec![
        StateClass::ScalarSettings,
        StateClass::Keybindings,
        StateClass::Snippets,
        StateClass::ThemesAndUiPresets,
        StateClass::ExtensionInventoryRefs,
        StateClass::ReferenceOnlySecretMetadata,
    ];
    let mut managed_includes = vec![
        StateClass::ScalarSettings,
        StateClass::Keybindings,
        StateClass::Snippets,
        StateClass::ExtensionInventoryRefs,
        StateClass::ReferenceOnlySecretMetadata,
    ];
    if leak_secret_material {
        managed_includes.push(StateClass::SecretMaterial);
    }

    vec![
        SnapshotManifestRow {
            snapshot_class: SnapshotClass::LocalRollbackCheckpoint,
            snapshot_ref: "aureline://snapshot/local-rollback-before-profile-import".to_owned(),
            snapshot_schema_version: "profile-sync-restore:v1".to_owned(),
            aureline_version: "0.0.0".to_owned(),
            platform_traits: vec!["macos".to_owned(), "aarch64".to_owned()],
            included_state_classes: vec![
                StateClass::ScalarSettings,
                StateClass::Keybindings,
                StateClass::SessionRestoreState,
            ],
            excluded_state_classes: vec![StateClass::SecretMaterial],
            integrity_hash:
                "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_owned(),
            source_provenance: "device:local-primary profile:dev-profile rev:41".to_owned(),
            local_only: true,
            waiver_ref: None,
        },
        SnapshotManifestRow {
            snapshot_class: SnapshotClass::PortableProfileExport,
            snapshot_ref: "aureline://snapshot/portable-profile-export-current".to_owned(),
            snapshot_schema_version: "profile-sync-restore:v1".to_owned(),
            aureline_version: "0.0.0".to_owned(),
            platform_traits: vec!["macos".to_owned(), "linux-compatible".to_owned()],
            included_state_classes: portable_includes.clone(),
            excluded_state_classes: ordinary_exclusions.clone(),
            integrity_hash:
                "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_owned(),
            source_provenance: "package:portable-profile-export profile:dev-profile rev:41"
                .to_owned(),
            local_only: false,
            waiver_ref: None,
        },
        SnapshotManifestRow {
            snapshot_class: SnapshotClass::ManagedSyncSnapshot,
            snapshot_ref: "aureline://snapshot/managed-sync-latest-success".to_owned(),
            snapshot_schema_version: "profile-sync-restore:v1".to_owned(),
            aureline_version: "0.0.0".to_owned(),
            platform_traits: vec!["macos".to_owned(), "linux-compatible".to_owned()],
            included_state_classes: managed_includes,
            excluded_state_classes: ordinary_exclusions.clone(),
            integrity_hash:
                "sha256:3333333333333333333333333333333333333333333333333333333333333333".to_owned(),
            source_provenance: "device:travel-laptop sync-rev:78 profile:dev-profile".to_owned(),
            local_only: false,
            waiver_ref: None,
        },
        SnapshotManifestRow {
            snapshot_class: SnapshotClass::SupportRecoveryManifest,
            snapshot_ref: "aureline://snapshot/support-recovery-manifest-current".to_owned(),
            snapshot_schema_version: "profile-sync-restore:v1".to_owned(),
            aureline_version: "0.0.0".to_owned(),
            platform_traits: vec!["redacted-platform-family".to_owned()],
            included_state_classes: vec![
                StateClass::ExtensionInventoryRefs,
                StateClass::ReferenceOnlySecretMetadata,
            ],
            excluded_state_classes: ordinary_exclusions,
            integrity_hash:
                "sha256:4444444444444444444444444444444444444444444444444444444444444444".to_owned(),
            source_provenance: "support-export:profile-roaming-manifest profile:dev-profile"
                .to_owned(),
            local_only: false,
            waiver_ref: None,
        },
    ]
}

fn merge_rules(omit_restore_checkpoint: bool) -> Vec<MergeRuleRow> {
    let checkpoint = if omit_restore_checkpoint {
        ""
    } else {
        "aureline://snapshot/local-rollback-before-profile-import"
    };
    vec![
        MergeRuleRow {
            subject_id: "editor.font_size".to_owned(),
            subject_class: MergeSubjectClass::ScalarSetting,
            conflict_class: ConflictClass::FieldwiseMerge,
            merge_rule: MergeRuleClass::FieldwiseMerge,
            source_provenance: "device:travel-laptop sync-rev:78".to_owned(),
            stale_remote: false,
            local_explicit_edit_wins: false,
            change_set_ref: "aureline://change-set/fieldwise-font-size".to_owned(),
            rollback_checkpoint_ref: checkpoint.to_owned(),
            overwrites_local_state: true,
            explicit_review_required: false,
            previewable_before_apply: true,
        },
        MergeRuleRow {
            subject_id: "snippets.rust".to_owned(),
            subject_class: MergeSubjectClass::AdditiveAsset,
            conflict_class: ConflictClass::AdditiveMerge,
            merge_rule: MergeRuleClass::AdditiveMerge,
            source_provenance: "package:portable-profile-export".to_owned(),
            stale_remote: false,
            local_explicit_edit_wins: false,
            change_set_ref: "aureline://change-set/additive-rust-snippets".to_owned(),
            rollback_checkpoint_ref: "aureline://snapshot/local-rollback-before-profile-import"
                .to_owned(),
            overwrites_local_state: false,
            explicit_review_required: false,
            previewable_before_apply: true,
        },
        MergeRuleRow {
            subject_id: "tasks.build".to_owned(),
            subject_class: MergeSubjectClass::StructuredDefinition,
            conflict_class: ConflictClass::ExplicitConflictReview,
            merge_rule: MergeRuleClass::ExplicitConflictReview,
            source_provenance: "package:portable-profile-export".to_owned(),
            stale_remote: false,
            local_explicit_edit_wins: false,
            change_set_ref: "aureline://change-set/task-build-review".to_owned(),
            rollback_checkpoint_ref: "aureline://snapshot/local-rollback-before-profile-import"
                .to_owned(),
            overwrites_local_state: true,
            explicit_review_required: true,
            previewable_before_apply: true,
        },
        MergeRuleRow {
            subject_id: "keybindings.save_all".to_owned(),
            subject_class: MergeSubjectClass::StructuredDefinition,
            conflict_class: ConflictClass::StaleRemoteLocalPrecedence,
            merge_rule: MergeRuleClass::LocalPrecedence,
            source_provenance: "device:old-desktop sync-rev:19".to_owned(),
            stale_remote: true,
            local_explicit_edit_wins: true,
            change_set_ref: "aureline://change-set/stale-keybinding-kept-local".to_owned(),
            rollback_checkpoint_ref: "aureline://snapshot/local-rollback-before-profile-import"
                .to_owned(),
            overwrites_local_state: false,
            explicit_review_required: true,
            previewable_before_apply: true,
        },
        MergeRuleRow {
            subject_id: "theme.active".to_owned(),
            subject_class: MergeSubjectClass::ScalarSetting,
            conflict_class: ConflictClass::ExactMatch,
            merge_rule: MergeRuleClass::FieldwiseMerge,
            source_provenance: "device:travel-laptop sync-rev:78".to_owned(),
            stale_remote: false,
            local_explicit_edit_wins: false,
            change_set_ref: "aureline://change-set/theme-no-op".to_owned(),
            rollback_checkpoint_ref: "aureline://snapshot/local-rollback-before-profile-import"
                .to_owned(),
            overwrites_local_state: false,
            explicit_review_required: false,
            previewable_before_apply: true,
        },
    ]
}

fn restore_previews(omit_restore_checkpoint: bool) -> Vec<RestorePreviewRow> {
    vec![
        RestorePreviewRow {
            preview_id: "portable-profile-import-preview".to_owned(),
            source_ref: "aureline://snapshot/portable-profile-export-current".to_owned(),
            snapshot_class: SnapshotClass::PortableProfileExport,
            structured_change_set_ref: "aureline://change-set/profile-import-preview".to_owned(),
            rollback_checkpoint_ref: if omit_restore_checkpoint {
                String::new()
            } else {
                "aureline://snapshot/local-rollback-before-profile-import".to_owned()
            },
            cross_platform_unmappable_sidecar_ref:
                "aureline://sidecar/profile-import-unmappable-values".to_owned(),
            preserves_unmappable_sidecar: true,
            retained_vs_overwritten_explicit: true,
            previewable_before_apply: true,
            overwrites_local_state: true,
        },
        RestorePreviewRow {
            preview_id: "support-recovery-manifest-preview".to_owned(),
            source_ref: "aureline://snapshot/support-recovery-manifest-current".to_owned(),
            snapshot_class: SnapshotClass::SupportRecoveryManifest,
            structured_change_set_ref: "aureline://change-set/support-recovery-explain-only"
                .to_owned(),
            rollback_checkpoint_ref: "aureline://snapshot/local-rollback-before-profile-import"
                .to_owned(),
            cross_platform_unmappable_sidecar_ref:
                "aureline://sidecar/support-recovery-unmappable-values".to_owned(),
            preserves_unmappable_sidecar: true,
            retained_vs_overwritten_explicit: true,
            previewable_before_apply: true,
            overwrites_local_state: false,
        },
    ]
}

fn secret_boundary_audit() -> Vec<SecretBoundaryAuditRow> {
    StateClass::ORDINARY_ROAMING_EXCLUSIONS
        .into_iter()
        .flat_map(|state_class| {
            [
                SnapshotClass::PortableProfileExport,
                SnapshotClass::ManagedSyncSnapshot,
                SnapshotClass::SupportRecoveryManifest,
            ]
            .into_iter()
            .map(move |lane| SecretBoundaryAuditRow {
                state_class,
                lane,
                raw_material_excluded: true,
                reference_only_metadata_allowed: true,
                evidence_ref: format!(
                    "aureline://audit/profile-sync-secret-boundary/{}-{}",
                    lane.as_str(),
                    state_class.as_str()
                ),
            })
        })
        .collect()
}

fn surface_truth() -> Vec<SurfaceTruthRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface_class| SurfaceTruthRow {
            surface_class,
            consumes_shared_record: true,
            shows_source: true,
            shows_snapshot_class: true,
            shows_state_classes: true,
            shows_conflict_class: true,
            shows_rollback_checkpoint: true,
            shows_local_authoritative_fallback: true,
        })
        .collect()
}

fn offboarding_retention(managed_sync_required: bool) -> OffboardingRetentionSummary {
    OffboardingRetentionSummary {
        local_checkpoint_retention_days: 30,
        retention_inspectable: true,
        final_export_package_ref: "aureline://offboarding/final-profile-export-package".to_owned(),
        latest_successful_sync_manifest_ref: "aureline://snapshot/managed-sync-latest-success"
            .to_owned(),
        profile_export_pointers: vec![
            "aureline://snapshot/portable-profile-export-current".to_owned(),
            "aureline://snapshot/support-recovery-manifest-current".to_owned(),
        ],
        extension_inventory_ref: "aureline://extension-inventory/profile-export-current".to_owned(),
        remaining_retention_timeline_ref: "aureline://retention/profile-checkpoint-timeline"
            .to_owned(),
        explainable_without_internal_logs: true,
        local_launch_edit_authority_retained: !managed_sync_required,
        managed_sync_required_for_local_work: managed_sync_required,
    }
}
