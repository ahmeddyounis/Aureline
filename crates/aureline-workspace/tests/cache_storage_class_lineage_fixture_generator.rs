//! Fixture generator helper for the cache / storage-class lineage
//! replay gate.
//!
//! Only runs when `CACHE_STORAGE_CLASS_LINEAGE_GEN_FIXTURES=1` is set
//! in the environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/cache_storage_class_lineage/` so the replay
//! gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_cache_storage_inspection_hooks, project_cache_storage_class_lineage_with_hooks,
    CacheStorageClassInputs, CacheStorageClassLineageRecord, CacheStorageInspectionHook,
    CacheStorageInspectionHookClass, CacheStorageObservation, CacheSupportExportInputs,
    CacheSupportExportPosture, ClaimedDurabilityTier, CleanupSurfaceKind, EvictionPolicyClass,
    StorageClassKind, UserStateClass,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/cache_storage_class_lineage")
}

#[allow(clippy::too_many_arguments)]
fn make_storage(
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
    captured_at: &str,
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
        support_export: CacheSupportExportInputs::metadata_safe_baseline(
            CacheSupportExportPosture::MetadataSafeExport,
        ),
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_storages(captured_at: &str) -> Vec<CacheStorageObservation> {
    vec![
        make_storage(
            "memory.editor_session",
            "Ephemeral editor session cache",
            StorageClassKind::EphemeralMemoryCache,
            UserStateClass::NoUserState,
            EvictionPolicyClass::RestartDrop,
            ClaimedDurabilityTier::StableEphemeralCache,
            vec![
                CleanupSurfaceKind::CommandPalette,
                CleanupSurfaceKind::HelpAbout,
            ],
            true,
            false,
            false,
            captured_at,
        ),
        make_storage(
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
            captured_at,
        ),
        make_storage(
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
            captured_at,
        ),
        make_storage(
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
            captured_at,
        ),
        make_storage(
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
            captured_at,
        ),
        make_storage(
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
            captured_at,
        ),
    ]
}

fn extended_storages(captured_at: &str) -> Vec<CacheStorageObservation> {
    let mut storages = baseline_storages(captured_at);
    storages.push(make_storage(
        "prebuild.artifact_cache",
        "Prebuild artifact cache",
        StorageClassKind::PrebuildArtifactCache,
        UserStateClass::RegenerableWithCost,
        EvictionPolicyClass::Lru,
        ClaimedDurabilityTier::StableRegenerableCache,
        vec![
            CleanupSurfaceKind::SettingsPanel,
            CleanupSurfaceKind::HeadlessCli,
        ],
        true,
        false,
        false,
        captured_at,
    ));
    storages.push(make_storage(
        "support.export_staging",
        "Support export staging area",
        StorageClassKind::SupportExportStaging,
        UserStateClass::NoUserState,
        EvictionPolicyClass::ManualAfterExport,
        ClaimedDurabilityTier::StableRegenerableCache,
        vec![
            CleanupSurfaceKind::SupportCleanupTool,
            CleanupSurfaceKind::HeadlessCli,
        ],
        true,
        false,
        false,
        captured_at,
    ));
    storages
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    storages: Vec<CacheStorageObservation>,
) -> CacheStorageClassInputs {
    CacheStorageClassInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        storages,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a CacheStorageClassInputs,
    inspection_hooks: &'a Vec<CacheStorageInspectionHook>,
    expected: &'a CacheStorageClassLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: CacheStorageClassInputs,
    inspection_hooks: Vec<CacheStorageInspectionHook>,
) {
    let record = project_cache_storage_class_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("CACHE_STORAGE_CLASS_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline: every required storage class is governed, every
    // user-state-bearing row is locked behind safe eviction + cleanup
    // guardrails, and every required cleanup surface is reachable.
    write_fixture(
        "baseline_user_state_safe_stable",
        "posture:baseline_user_state_safe",
        base_inputs(
            "workspace-rust-service-0001",
            "cache-storage-corpus-baseline-0001",
            "mono:1700000400",
            baseline_storages("mono:1700000400"),
        ),
        default_cache_storage_inspection_hooks(),
    );

    // Extended: adds the optional prebuild artifact cache and
    // support-export staging classes. Still Stable.
    write_fixture(
        "extended_with_prebuild_and_support_staging_stable",
        "posture:extended_with_prebuild_and_support_staging",
        base_inputs(
            "workspace-rust-service-0001",
            "cache-storage-corpus-extended-0001",
            "mono:1700000410",
            extended_storages("mono:1700000410"),
        ),
        default_cache_storage_inspection_hooks(),
    );

    // Narrowed: local history was downgraded to an `lru` eviction
    // policy. The derived tier must downgrade and the record must
    // narrow with `user_state_with_unsafe_eviction` plus
    // `durability_tier_mismatch_derived`.
    let mut narrowed_storages = baseline_storages("mono:1700000420");
    let history = narrowed_storages
        .iter_mut()
        .find(|s| s.storage_class == StorageClassKind::LocalHistory)
        .expect("local history seeded");
    history.eviction_policy = EvictionPolicyClass::Lru;
    write_fixture(
        "local_history_lru_eviction_narrowed",
        "posture:local_history_lru_eviction",
        base_inputs(
            "workspace-rust-service-0001",
            "cache-storage-corpus-narrowed-eviction-0001",
            "mono:1700000420",
            narrowed_storages,
        ),
        default_cache_storage_inspection_hooks(),
    );

    // Narrowed: required `export_before_clear` hook is unavailable on
    // this posture (e.g. degraded headless runner).
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "cache-storage-corpus-narrowed-hook-0001",
        "mono:1700000430",
        baseline_storages("mono:1700000430"),
    );
    let mut narrowed_hooks = default_cache_storage_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == CacheStorageInspectionHookClass::ExportBeforeClear {
            hook.available = false;
            hook.disclosure = "Export-before-clear unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_export_before_clear_hook_narrowed",
        "posture:missing_export_before_clear_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
