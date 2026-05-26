//! Unit tests for the cache / storage-class governance lineage
//! projection.

use super::*;

#[allow(clippy::too_many_arguments)]
fn storage(
    storage_id: &str,
    title: &str,
    class: StorageClassKind,
    user_state: UserStateClass,
    eviction: EvictionPolicyClass,
    tier: ClaimedDurabilityTier,
    cleanup_surfaces: Vec<CleanupSurfaceKind>,
    silent_clear_blocked: bool,
    requires_inspection: bool,
    preserve_on_quota: bool,
    posture: CacheSupportExportPosture,
) -> CacheStorageObservation {
    CacheStorageObservation {
        storage_id: storage_id.to_owned(),
        title: title.to_owned(),
        storage_class: class,
        user_state_class: user_state,
        eviction_policy: eviction,
        claimed_durability_tier: tier,
        cleanup_surfaces,
        silent_clear_blocked,
        requires_pre_action_inspection: requires_inspection,
        preserve_on_quota_pressure: preserve_on_quota,
        support_export: CacheSupportExportInputs::metadata_safe_baseline(posture),
        captured_at: "mono:1700000400".to_owned(),
    }
}

fn baseline_inputs() -> CacheStorageClassInputs {
    CacheStorageClassInputs {
        workspace_ref: "workspace-rust-service-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "cache-storage-corpus-0001".to_owned(),
        captured_at: "mono:1700000400".to_owned(),
        storages: vec![
            storage(
                "memory.editor_session",
                "Ephemeral editor session cache",
                StorageClassKind::EphemeralMemoryCache,
                UserStateClass::NoUserState,
                EvictionPolicyClass::RestartDrop,
                ClaimedDurabilityTier::StableEphemeralCache,
                vec![CleanupSurfaceKind::CommandPalette, CleanupSurfaceKind::HelpAbout],
                true,
                false,
                false,
                CacheSupportExportPosture::MetadataSafeExport,
            ),
            storage(
                "disk.git_object_cache",
                "Local disk git object cache",
                StorageClassKind::LocalDiskCache,
                UserStateClass::NoUserState,
                EvictionPolicyClass::Lru,
                ClaimedDurabilityTier::StableRegenerableCache,
                vec![
                    CleanupSurfaceKind::SettingsPanel,
                    CleanupSurfaceKind::HeadlessCli,
                ],
                true,
                false,
                false,
                CacheSupportExportPosture::MetadataSafeExport,
            ),
            storage(
                "index.search",
                "Search index",
                StorageClassKind::DerivedIndex,
                UserStateClass::RegenerableWithCost,
                EvictionPolicyClass::QuotaPressure,
                ClaimedDurabilityTier::StableRegenerableCache,
                vec![
                    CleanupSurfaceKind::SettingsPanel,
                    CleanupSurfaceKind::SupportCleanupTool,
                ],
                true,
                false,
                false,
                CacheSupportExportPosture::MetadataSafeExport,
            ),
            storage(
                "workspace.durable_layout",
                "Durable workspace layout state",
                StorageClassKind::DurableWorkspaceState,
                UserStateClass::UserAuthored,
                EvictionPolicyClass::ManualOnly,
                ClaimedDurabilityTier::StableDurableUserState,
                vec![
                    CleanupSurfaceKind::SettingsPanel,
                    CleanupSurfaceKind::CommandPalette,
                    CleanupSurfaceKind::HeadlessCli,
                ],
                true,
                true,
                true,
                CacheSupportExportPosture::MetadataSafeExport,
            ),
            storage(
                "recovery.checkpoint",
                "Recovery checkpoint snapshot",
                StorageClassKind::RecoveryCheckpoint,
                UserStateClass::UserDerived,
                EvictionPolicyClass::ManualAfterExport,
                ClaimedDurabilityTier::StableDurableUserState,
                vec![
                    CleanupSurfaceKind::SupportCleanupTool,
                    CleanupSurfaceKind::CrashRecoveryPanel,
                    CleanupSurfaceKind::HeadlessCli,
                ],
                true,
                true,
                true,
                CacheSupportExportPosture::MetadataSafeExport,
            ),
            storage(
                "history.local",
                "Local edit history",
                StorageClassKind::LocalHistory,
                UserStateClass::UserDerived,
                EvictionPolicyClass::ManualOnly,
                ClaimedDurabilityTier::StableDurableUserState,
                vec![
                    CleanupSurfaceKind::CommandPalette,
                    CleanupSurfaceKind::HelpAbout,
                ],
                true,
                true,
                true,
                CacheSupportExportPosture::MetadataSafeExport,
            ),
        ],
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record = project_cache_storage_class_lineage("posture.clean", &inputs);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF);
    assert!(
        record
            .storage_class_coverage
            .all_required_storage_classes_present
    );
    assert_eq!(record.storage_class_coverage.storage_rows.len(), 6);
    assert!(record.eviction_policy_truth.all_tiers_match_derived);
    assert_eq!(record.user_state_governance.user_state_row_count, 3);
    assert!(
        record
            .user_state_governance
            .all_user_state_rows_have_safe_eviction
    );
    assert!(
        record
            .cleanup_surface_coverage
            .all_required_cleanup_surfaces_present
    );
    assert!(
        record
            .support_export_honesty
            .all_user_state_rows_have_safe_posture
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record.producer_attribution.integrity_hash.starts_with("csl:"));
}

#[test]
fn missing_required_storage_class_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .storages
        .retain(|s| s.storage_class != StorageClassKind::LocalHistory);

    let record = project_cache_storage_class_lineage("posture.missing_local_history", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::RequiredStorageClassMissing));
}

#[test]
fn user_state_with_lru_eviction_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::LocalHistory)
        .expect("local history seeded");
    row.eviction_policy = EvictionPolicyClass::Lru;
    row.claimed_durability_tier = ClaimedDurabilityTier::StableDurableUserState;

    let record = project_cache_storage_class_lineage("posture.lru_history", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::UserStateWithUnsafeEviction));
    // The derived tier must downgrade to narrowed and surface the
    // mismatch.
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::DurabilityTierMismatchDerived));
}

#[test]
fn user_state_without_silent_clear_blocked_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::DurableWorkspaceState)
        .expect("durable workspace state seeded");
    row.silent_clear_blocked = false;

    let record = project_cache_storage_class_lineage("posture.silent_clear", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::UserStateCleanupNotBlocked));
}

#[test]
fn user_state_without_pre_action_inspection_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::LocalHistory)
        .expect("local history seeded");
    row.requires_pre_action_inspection = false;

    let record = project_cache_storage_class_lineage("posture.no_inspection", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::UserStateInspectionNotRequired));
}

#[test]
fn user_state_not_preserved_on_quota_pressure_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::RecoveryCheckpoint)
        .expect("recovery checkpoint seeded");
    row.preserve_on_quota_pressure = false;

    let record = project_cache_storage_class_lineage("posture.quota_pressure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::UserStateQuotaPressureUnsafe));
}

#[test]
fn user_state_without_cleanup_surface_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::DurableWorkspaceState)
        .expect("durable workspace state seeded");
    row.cleanup_surfaces.clear();

    let record = project_cache_storage_class_lineage("posture.no_cleanup", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::UserStateCleanupSurfaceMissing));
}

#[test]
fn missing_required_cleanup_surface_narrows_record() {
    let mut inputs = baseline_inputs();
    for s in &mut inputs.storages {
        s.cleanup_surfaces
            .retain(|surface| *surface != CleanupSurfaceKind::SupportCleanupTool);
    }

    let record = project_cache_storage_class_lineage("posture.no_support_cleanup", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::RequiredCleanupSurfaceMissing));
}

#[test]
fn declared_tier_mismatch_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::EphemeralMemoryCache)
        .expect("memory cache seeded");
    row.claimed_durability_tier = ClaimedDurabilityTier::StableDurableUserState;

    let record = project_cache_storage_class_lineage("posture.tier_mismatch", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::DurabilityTierMismatchDerived));
}

#[test]
fn support_export_dropping_fields_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.storages[0].support_export.includes_eviction_policy = false;

    let record = project_cache_storage_class_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn support_export_raising_raw_cache_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.storages[0].support_export.raw_cache_content_excluded = false;

    let record = project_cache_storage_class_lineage("posture.raw_cache", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn user_state_local_only_support_posture_narrows_record() {
    let mut inputs = baseline_inputs();
    let row = inputs
        .storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::LocalHistory)
        .expect("local history seeded");
    row.support_export.posture = CacheSupportExportPosture::LocalOnly;

    let record = project_cache_storage_class_lineage("posture.local_only", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::SupportExportPostureUnsafe));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = baseline_inputs();
    let mut hooks = default_cache_storage_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == CacheStorageInspectionHookClass::ExportBeforeClear {
            hook.available = false;
        }
    }

    let record = project_cache_storage_class_lineage_with_hooks(
        "posture.no_export_before_clear",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref = "".to_owned();

    let record = project_cache_storage_class_lineage("posture.empty_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.storages.clear();

    let record = project_cache_storage_class_lineage("posture.empty_corpus", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::CorpusEmpty));
}

#[test]
fn producer_attribution_incomplete_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref = "".to_owned();

    let record = project_cache_storage_class_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&CacheStorageClassLineageNarrowReason::ProducerAttributionIncomplete));
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = baseline_inputs();
    let record = project_cache_storage_class_lineage("posture.lines", &inputs);
    let lines = cache_storage_class_lineage_lines(&record);

    assert!(lines
        .iter()
        .any(|line| line.contains("Cache / storage-class lineage")));
    assert!(lines
        .iter()
        .any(|line| line.contains("storage_class_coverage")));
    assert!(lines.iter().any(|line| line == "Storage rows:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("Eviction policy truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("User-state governance")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Cleanup surface coverage")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = baseline_inputs();
    let record = project_cache_storage_class_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: CacheStorageClassLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
