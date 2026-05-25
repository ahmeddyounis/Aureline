//! Deterministic claimed-stable matrix for sync / device-registry
//! certifications.
//!
//! Every record here is a genuine projection of the **live** settings runtime.
//! The corpus builds an [`EffectiveSettingsResolver`] over the seeded
//! [`SchemaRegistry`] (plus a small set of sync-lane asset definitions),
//! installs overlays representing the synced user profile, machine-local state,
//! the workspace, and the policy-owned ceiling, then projects each conflicting
//! setting through the beta [`crate::sync::build_review_row`] path — which is
//! itself a projection of [`crate::inspector::conflict`] and the resolver. The
//! stable lane adds the field-aware outcome class, the REL-SYNC-009 merge class,
//! the device-participation truth, the snapshot provenance, the secret boundary,
//! and the profile-roaming summary on top of that real resolver state, so a
//! certification record can never drift from what the resolver actually
//! resolves.
//!
//! Six postures pin the matrix:
//!
//! - `nominal` — devices participate truthfully, conflict review is field-aware,
//!   every snapshot class carries provenance, overwrites are protected, the
//!   secret boundary holds, REL-SYNC-009 merge rules are enforced, profile
//!   roaming is complete, and every surface shares one truth. Qualifies
//!   **Stable**.
//! - `stale_remote_local_authoritative` — a remote device's bundle is stale; the
//!   device stays inspectable with a retained rollback checkpoint and local
//!   stays authoritative. Still qualifies **Stable**.
//! - `managed_sync_unavailable` — managed sync is unavailable; local launch /
//!   edit authority is retained and disclosed. Still qualifies **Stable**.
//! - `secret_boundary_drill` — an adversarial managed sync snapshot lists a
//!   dirty-buffer journal among its included state classes; the lane detects the
//!   boundary violation and narrows below Stable with a named reason.
//! - `unprotected_overwrite_drill` — a conflict row would overwrite a local
//!   scope with no change preview and no rollback checkpoint; the lane refuses to
//!   certify it Stable and narrows with a named reason.
//! - `device_registry_view_in_preview` — the admin device-registry surface
//!   treatment is still Preview, so the posture is narrowed below Stable by its
//!   lowest surface marker rather than inheriting an adjacent green row.

use crate::inspector::{SettingsInspectError, SettingsInspectionContext};
use crate::resolver::{CapabilityState, EffectiveSettingsResolver, PolicyConstraint, ScopeOverlay};
use crate::schema::{
    LifecycleLabel, PreviewClass, RedactionClass, RestartPosture, SchemaRegistry, SensitivityClass,
    SettingAlias, SettingDefinition, SettingScope, SettingValue, SettingValueType,
};
use crate::sync::{
    build_review_row, DeviceParticipationState, IdentityModeClass, SyncBetaDeviceRecord,
    SyncConflictReviewBetaRequest, SyncConflictReviewBetaRow,
};

use super::model::{
    is_canonical_object_ref, required_recovery_routes, AccessibilityDisclosure,
    CertificationClaimCeiling, CertificationInput, CertificationUpstream, ConflictOutcomeClass,
    ConflictReviewRow, DeviceParticipationRow, EntryRouteRecord, LayoutMode, LayoutModeDisclosure,
    LifecycleMarker, MergeClass, ProfileDurabilityClass, ProfileRoamingSummary, RouteSurface,
    SecretBoundaryRow, SettingCategory, SnapshotClass, SnapshotRow, StableClaimClass, StateClass,
    SurfaceClass, SurfaceParityRow, SyncDeviceRegistryCertification,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const REGISTRY_REF: &str = "settings:schema_registry_seed:v1";
const SYNC_CONTRACT_REF: &str = "settings:sync_beta:v1";
const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/sync-device-registry";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/sync-device-registry";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-sync-device-registry";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-sync-device-registry";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-sync-device-registry";

const EGRESS_SETTING: &str = "security.ai.egress_policy";
const RULER_SETTING: &str = "editor.ruler_column";
const RULER_ALIAS: &str = "editor.rulerColumn";
const TASKS_SETTING: &str = "tasks.runner_profile";
const TRUSTED_FOLDERS_SETTING: &str = "security.trusted_folders";

const LOCAL_DEVICE_ID: &str = "dev-laptop-primary-0001";
const REMOTE_DESKTOP_ID: &str = "dev-desktop-home-0002";
const REMOTE_LAPTOP_ID: &str = "dev-laptop-travel-0003";

const PRODUCER_AURELINE_VERSION: &str = "0.0.0";
const PRODUCER_SCHEMA_VERSION: &str = "settings-schema:v1";

/// One scenario in the claimed-stable certification matrix.
#[derive(Debug, Clone)]
pub struct SyncDeviceRegistryScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest surface).
    pub expected_surface_marker: LifecycleMarker,
    record: SyncDeviceRegistryCertification,
}

impl SyncDeviceRegistryScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> SyncDeviceRegistryCertification {
        self.record.clone()
    }
}

/// One whole-posture spec.
struct ScenarioSpec {
    scenario_id: &'static str,
    posture_id: &'static str,
    posture_label: &'static str,
    title: &'static str,
    summary: &'static str,
    /// Whether managed sync is currently available.
    managed_sync_available: bool,
    /// Whether the travel laptop device is stale.
    stale_remote_device: bool,
    /// Whether the managed sync snapshot leaks a dirty-buffer journal.
    secret_boundary_drill: bool,
    /// Whether the additive overwrite row drops its change preview / checkpoint.
    unprotected_overwrite_drill: bool,
    /// Whether the admin device-registry surface treatment is still Preview.
    device_registry_preview: bool,
    claim_ceiling: CertificationClaimCeiling,
}

fn full_claim_ceiling() -> CertificationClaimCeiling {
    CertificationClaimCeiling {
        asserts_device_participation_truth: true,
        asserts_conflict_review_field_aware: true,
        asserts_snapshot_provenance_complete: true,
        asserts_local_fallback_proven: true,
        asserts_secret_boundary_held: true,
        asserts_merge_rules_enforced: true,
        asserts_profile_roaming_truth: true,
        asserts_surfaces_share_one_truth: true,
    }
}

// ---------------------------------------------------------------------------
// Live resolver
// ---------------------------------------------------------------------------

fn sync_lane_asset(
    setting_id: &str,
    value_type: SettingValueType,
    default_value: SettingValue,
    aliases: Vec<SettingAlias>,
) -> SettingDefinition {
    SettingDefinition {
        setting_id: setting_id.to_owned(),
        value_type,
        default_value,
        allowed_scopes: vec![
            SettingScope::BuiltInDefault,
            SettingScope::UserGlobal,
            SettingScope::Workspace,
        ],
        restart_posture: RestartPosture::NoRestart,
        lifecycle_label: LifecycleLabel::Stable,
        preview_class: PreviewClass::SafeApply,
        redaction_class: RedactionClass::None,
        sensitivity_class: SensitivityClass::GeneralPreference,
        alias_set: aliases,
        migration_table: Vec::new(),
        capability_dependencies: Vec::new(),
        help_doc_ref: Some(format!("docs:settings:{setting_id}")),
        evidence_refs: Vec::new(),
        decision_row_ref: None,
        since_version: None,
        description: None,
        change_guidance: None,
        is_machine_specific: false,
        is_synced_by_default: true,
        is_policy_narrowable: false,
        summary: format!("Sync-lane asset {setting_id}."),
    }
}

/// Builds the configured resolver: the synced user profile, machine-local state,
/// the workspace, the policy-owned ceiling, and the sync-lane asset definitions.
fn configured_resolver() -> EffectiveSettingsResolver {
    let mut registry = SchemaRegistry::with_seed_catalog();
    registry
        .register(sync_lane_asset(
            RULER_SETTING,
            SettingValueType::Integer {
                min: Some(0),
                max: Some(300),
            },
            SettingValue::Integer(80),
            vec![SettingAlias::active(RULER_ALIAS, "0.0.0-alpha")],
        ))
        .expect("register ruler asset");
    registry
        .register(sync_lane_asset(
            TASKS_SETTING,
            SettingValueType::String,
            SettingValue::String("default".into()),
            Vec::new(),
        ))
        .expect("register tasks asset");
    registry
        .register(sync_lane_asset(
            TRUSTED_FOLDERS_SETTING,
            SettingValueType::String,
            SettingValue::String("[]".into()),
            Vec::new(),
        ))
        .expect("register trusted-folders asset");

    let mut resolver = EffectiveSettingsResolver::new(registry);

    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "Synced user profile");
    user.set_value("editor.tab_size", SettingValue::Integer(4));
    user.set_value("editor.format_on_save", SettingValue::Boolean(true));
    user.set_value("ui.density", SettingValue::String("comfortable".into()));
    user.set_value(
        EGRESS_SETTING,
        SettingValue::String("any_hosted_provider".into()),
    );
    user.set_value(RULER_SETTING, SettingValue::Integer(80));
    user.set_value(TASKS_SETTING, SettingValue::String("local".into()));
    user.set_value(
        TRUSTED_FOLDERS_SETTING,
        SettingValue::String("[\"~/work\"]".into()),
    );
    resolver.set_overlay(user).expect("synced overlay");

    let mut machine = ScopeOverlay::new(SettingScope::MachineSpecific, "This machine");
    machine.set_value("ui.motion", SettingValue::String("reduced".into()));
    resolver.set_overlay(machine).expect("machine overlay");

    let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace settings");
    workspace.set_value("editor.tab_size", SettingValue::Integer(8));
    resolver.set_overlay(workspace).expect("workspace overlay");

    let mut policy =
        ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
    policy.set_policy_constraint(
        EGRESS_SETTING,
        PolicyConstraint::SingleValue {
            value: SettingValue::String("approved_hosted_providers_only".into()),
        },
    );
    resolver.set_overlay(policy).expect("policy overlay");

    let egress_dep = resolver
        .registry()
        .definition(EGRESS_SETTING)
        .expect("egress definition")
        .capability_dependencies[0]
        .clone();
    resolver.set_capability_state(
        &egress_dep,
        CapabilityState::satisfied("identity_mode=managed_convenience"),
    );

    resolver
}

fn inspection_context(resolver: &EffectiveSettingsResolver) -> SettingsInspectionContext {
    let egress_dep = resolver
        .registry()
        .definition(EGRESS_SETTING)
        .expect("egress definition")
        .capability_dependencies[0]
        .clone();
    SettingsInspectionContext::new()
        .with_last_applied_revision(EGRESS_SETTING, "settings-rev:00042")
        .with_capability_state(&egress_dep, true, "identity_mode=managed_convenience")
}

// ---------------------------------------------------------------------------
// Sync device adapters
// ---------------------------------------------------------------------------

fn sync_local_device() -> SyncBetaDeviceRecord {
    SyncBetaDeviceRecord {
        device_id: LOCAL_DEVICE_ID.to_owned(),
        device_label: Some("Primary laptop".to_owned()),
        device_class: "personal_laptop".to_owned(),
        os_family_class: "macos".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Active,
        revocation_reason: None,
        lineage_cursor: Some("lc-0001-000000000517".to_owned()),
        last_seen_at: Some("2026-05-25T09:00:00Z".to_owned()),
        last_seen_source: Some("local_heartbeat".to_owned()),
        trust_state: Some("trusted".to_owned()),
    }
}

fn sync_remote_desktop() -> SyncBetaDeviceRecord {
    SyncBetaDeviceRecord {
        device_id: REMOTE_DESKTOP_ID.to_owned(),
        device_label: Some("Home desktop".to_owned()),
        device_class: "personal_workstation".to_owned(),
        os_family_class: "linux".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Active,
        revocation_reason: None,
        lineage_cursor: Some("lc-0002-000000000142".to_owned()),
        last_seen_at: Some("2026-05-25T08:30:00Z".to_owned()),
        last_seen_source: Some("push".to_owned()),
        trust_state: Some("trusted".to_owned()),
    }
}

// ---------------------------------------------------------------------------
// Conflict-review rows (projected from the live sync path)
// ---------------------------------------------------------------------------

struct ConflictSpec {
    setting_id: &'static str,
    conflicting_scope: SettingScope,
    conflicting_value: SettingValue,
    category: SettingCategory,
    outcome: ConflictOutcomeClass,
    merge: MergeClass,
    overwrites_local: bool,
    local_authoritative: bool,
    provide_protection: bool,
    remote_bundle_epoch: Option<u64>,
    local_bundle_epoch_floor: Option<u64>,
}

fn project_sync_row(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    spec: &ConflictSpec,
) -> Result<SyncConflictReviewBetaRow, SettingsInspectError> {
    build_review_row(
        resolver,
        context,
        SyncConflictReviewBetaRequest {
            setting_id: spec.setting_id.to_owned(),
            local_device: sync_local_device(),
            remote_device: sync_remote_desktop(),
            conflicting_scope: spec.conflicting_scope,
            conflicting_value: spec.conflicting_value.clone(),
            import_continuity: false,
            remote_bundle_epoch: spec.remote_bundle_epoch,
            local_bundle_epoch_floor: spec.local_bundle_epoch_floor,
            last_writer: None,
            rollback_checkpoint_ref: None,
            approval_ticket_ref: None,
        },
    )
}

fn conflict_row(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    spec: &ConflictSpec,
) -> ConflictReviewRow {
    let sync_row = project_sync_row(resolver, context, spec)
        .unwrap_or_else(|err| panic!("project sync row {}: {err}", spec.setting_id));
    let change_preview_ref = spec
        .provide_protection
        .then(|| format!("aureline://change-preview/sync/{}", spec.setting_id));
    let rollback_checkpoint_ref = spec
        .provide_protection
        .then(|| format!("aureline://checkpoint/sync/{}", spec.setting_id));

    ConflictReviewRow {
        setting_id: sync_row.setting_id.clone(),
        conflicting_scope: spec.conflicting_scope.as_str().to_owned(),
        setting_category: spec.category,
        outcome_class: spec.outcome,
        merge_class: spec.merge,
        remote_device_id: REMOTE_DESKTOP_ID.to_owned(),
        remote_participation_state: DeviceParticipationState::Active,
        recommended_path: sync_row.recommended_resolution_path.clone(),
        overwrites_local: spec.overwrites_local,
        local_authoritative: spec.local_authoritative,
        change_preview_ref,
        rollback_checkpoint_ref,
        inspectable_before_apply: true,
        widens_authority: false,
        redaction_class: sync_row.redaction_class.clone(),
        lock_state: sync_row.lock_state.clone(),
        source_packet_ref: sync_row.source_packet_ref.clone(),
        diagnostics_entry_ref: format!(
            "aureline://diagnostics/sync-conflict/{}",
            sync_row.setting_id
        ),
        merge_rule_satisfied: false,
        protected_before_overwrite: false,
        waiver_ref: None,
        conforms: false,
    }
}

fn conflict_specs() -> Vec<ConflictSpec> {
    vec![
        // ExactMatch: local and remote agree; nothing changes.
        ConflictSpec {
            setting_id: "editor.format_on_save",
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Boolean(true),
            category: SettingCategory::Scalar,
            outcome: ConflictOutcomeClass::ExactMatch,
            merge: MergeClass::FieldwiseMerge,
            overwrites_local: false,
            local_authoritative: true,
            provide_protection: false,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
        },
        // Translated: the remote arrived under the migration alias and maps onto
        // the local value, so nothing overwrites.
        ConflictSpec {
            setting_id: RULER_SETTING,
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Integer(80),
            category: SettingCategory::Scalar,
            outcome: ConflictOutcomeClass::Translated,
            merge: MergeClass::FieldwiseMerge,
            overwrites_local: false,
            local_authoritative: true,
            provide_protection: false,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
        },
        // StaleRemote: the remote bundle is older than the local lineage.
        ConflictSpec {
            setting_id: "editor.tab_size",
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::Integer(2),
            category: SettingCategory::Scalar,
            outcome: ConflictOutcomeClass::StaleRemote,
            merge: MergeClass::LocalPrecedence,
            overwrites_local: false,
            local_authoritative: true,
            provide_protection: false,
            remote_bundle_epoch: Some(100),
            local_bundle_epoch_floor: Some(200),
        },
        // PolicyLocked: an admin policy ceiling owns the value.
        ConflictSpec {
            setting_id: EGRESS_SETTING,
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::String("any_hosted_provider".into()),
            category: SettingCategory::Scalar,
            outcome: ConflictOutcomeClass::PolicyLocked,
            merge: MergeClass::FieldwiseMerge,
            overwrites_local: false,
            local_authoritative: true,
            provide_protection: false,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
        },
        // LocalAuthoritative: a scalar divergence where the local explicit edit
        // wins until the user merges.
        ConflictSpec {
            setting_id: "ui.density",
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::String("compact".into()),
            category: SettingCategory::Scalar,
            outcome: ConflictOutcomeClass::LocalAuthoritative,
            merge: MergeClass::FieldwiseMerge,
            overwrites_local: false,
            local_authoritative: true,
            provide_protection: false,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
        },
        // Partial (structured): a task definition that requires explicit review
        // before any overwrite.
        ConflictSpec {
            setting_id: TASKS_SETTING,
            conflicting_scope: SettingScope::Workspace,
            conflicting_value: SettingValue::String("ci".into()),
            category: SettingCategory::StructuredDefinition,
            outcome: ConflictOutcomeClass::Partial,
            merge: MergeClass::ExplicitConflictReview,
            overwrites_local: false,
            local_authoritative: true,
            provide_protection: false,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
        },
        // Partial (additive): trusted folders merge additively, overwriting the
        // local set, protected by a change preview and a rollback checkpoint.
        ConflictSpec {
            setting_id: TRUSTED_FOLDERS_SETTING,
            conflicting_scope: SettingScope::UserGlobal,
            conflicting_value: SettingValue::String("[\"~/work\",\"~/oss\"]".into()),
            category: SettingCategory::AdditiveAsset,
            outcome: ConflictOutcomeClass::Partial,
            merge: MergeClass::AdditiveMerge,
            overwrites_local: true,
            local_authoritative: false,
            provide_protection: true,
            remote_bundle_epoch: Some(142),
            local_bundle_epoch_floor: Some(140),
        },
    ]
}

fn conflict_review(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    spec: &ScenarioSpec,
) -> Vec<ConflictReviewRow> {
    conflict_specs()
        .iter()
        .map(|conflict| {
            let mut row = conflict_row(resolver, context, conflict);
            if spec.unprotected_overwrite_drill && row.setting_id == TRUSTED_FOLDERS_SETTING {
                // Drop the protection so the lane catches an unprotected overwrite.
                row.change_preview_ref = None;
                row.rollback_checkpoint_ref = None;
                row.waiver_ref = Some(format!(
                    "aureline://waiver/sync-{}-unprotected-overwrite",
                    row.setting_id
                ));
            }
            row
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Device participation
// ---------------------------------------------------------------------------

fn device_participation(spec: &ScenarioSpec) -> Vec<DeviceParticipationRow> {
    let scalar_scopes = vec![
        SettingScope::UserGlobal.as_str().to_owned(),
        SettingScope::Workspace.as_str().to_owned(),
    ];

    let local = DeviceParticipationRow {
        device_id: LOCAL_DEVICE_ID.to_owned(),
        device_label: Some("Primary laptop".to_owned()),
        device_class: "personal_laptop".to_owned(),
        os_family_class: "macos".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Active,
        is_local_device: true,
        profile_durability: ProfileDurabilityClass::Durable,
        last_successful_sync: Some("2026-05-25T09:00:00Z".to_owned()),
        sync_freshness: if spec.managed_sync_available {
            "fresh".to_owned()
        } else {
            "blocked".to_owned()
        },
        selected_scope_set: scalar_scopes.clone(),
        conflict_class: "scalar_divergence".to_owned(),
        rollback_checkpoint_ref: Some("aureline://checkpoint/device/laptop-primary".to_owned()),
        local_authoritative_fallback: true,
        inspectable_without_mutation: true,
        revocation_reason: None,
        waiver_ref: None,
        conforms: false,
    };

    let desktop = DeviceParticipationRow {
        device_id: REMOTE_DESKTOP_ID.to_owned(),
        device_label: Some("Home desktop".to_owned()),
        device_class: "personal_workstation".to_owned(),
        os_family_class: "linux".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Active,
        is_local_device: false,
        profile_durability: ProfileDurabilityClass::Durable,
        last_successful_sync: Some("2026-05-25T08:30:00Z".to_owned()),
        sync_freshness: if spec.managed_sync_available {
            "fresh".to_owned()
        } else {
            "blocked".to_owned()
        },
        selected_scope_set: scalar_scopes.clone(),
        conflict_class: "none".to_owned(),
        rollback_checkpoint_ref: Some("aureline://checkpoint/device/desktop-home".to_owned()),
        local_authoritative_fallback: true,
        inspectable_without_mutation: true,
        revocation_reason: None,
        waiver_ref: None,
        conforms: false,
    };

    let laptop = DeviceParticipationRow {
        device_id: REMOTE_LAPTOP_ID.to_owned(),
        device_label: Some("Travel laptop".to_owned()),
        device_class: "personal_laptop".to_owned(),
        os_family_class: "windows".to_owned(),
        identity_mode: IdentityModeClass::AccountFreeLocal,
        participation_state: DeviceParticipationState::Paused,
        is_local_device: false,
        profile_durability: ProfileDurabilityClass::Durable,
        last_successful_sync: Some("2026-05-18T22:11:00Z".to_owned()),
        sync_freshness: if spec.stale_remote_device {
            "stale".to_owned()
        } else {
            "paused".to_owned()
        },
        selected_scope_set: scalar_scopes,
        conflict_class: "none".to_owned(),
        rollback_checkpoint_ref: Some("aureline://checkpoint/device/laptop-travel".to_owned()),
        local_authoritative_fallback: true,
        inspectable_without_mutation: true,
        revocation_reason: Some("user_paused".to_owned()),
        waiver_ref: None,
        conforms: false,
    };

    vec![local, desktop, laptop]
}

// ---------------------------------------------------------------------------
// Snapshots
// ---------------------------------------------------------------------------

fn portable_state_classes() -> Vec<StateClass> {
    vec![
        StateClass::ScalarSettings,
        StateClass::Keybindings,
        StateClass::Tasks,
        StateClass::LaunchConfigs,
        StateClass::WorksetDefinitions,
        StateClass::ExtensionInventoryRefs,
        StateClass::ReferenceOnlyMetadata,
    ]
}

fn excluded_from_lane() -> Vec<StateClass> {
    vec![
        StateClass::MachineLocalTopology,
        StateClass::DirtyBufferJournals,
        StateClass::SecretMaterial,
    ]
}

fn snapshots(spec: &ScenarioSpec) -> Vec<SnapshotRow> {
    let mut managed_included = portable_state_classes();
    let mut managed_excluded = excluded_from_lane();
    let mut managed_waiver = None;
    if spec.secret_boundary_drill {
        // Adversarial: the managed snapshot leaks a dirty-buffer journal.
        managed_included.push(StateClass::DirtyBufferJournals);
        managed_excluded.retain(|class| *class != StateClass::DirtyBufferJournals);
        managed_waiver = Some(
            "aureline://waiver/sync-managed-snapshot-dirty-buffer-leak".to_owned(),
        );
    }

    vec![
        SnapshotRow {
            snapshot_class: SnapshotClass::LocalRollbackCheckpoint,
            snapshot_ref: "aureline://snapshot/local-rollback-checkpoint".to_owned(),
            producer_aureline_version: PRODUCER_AURELINE_VERSION.to_owned(),
            producer_schema_version: PRODUCER_SCHEMA_VERSION.to_owned(),
            integrity_hash: "sha256:checkpoint-0001".to_owned(),
            source_provenance: format!("{LOCAL_DEVICE_ID}@rev-00517"),
            // The local checkpoint may retain machine-local topology: it never
            // crosses a sync or export lane.
            included_state_classes: {
                let mut classes = portable_state_classes();
                classes.push(StateClass::MachineLocalTopology);
                classes
            },
            excluded_state_classes: vec![
                StateClass::DirtyBufferJournals,
                StateClass::SecretMaterial,
            ],
            local_authoritative_fallback: true,
            carries_forbidden_state_class: false,
            waiver_ref: None,
            conforms: false,
        },
        SnapshotRow {
            snapshot_class: SnapshotClass::PortableProfileExport,
            snapshot_ref: "aureline://snapshot/portable-profile-export".to_owned(),
            producer_aureline_version: PRODUCER_AURELINE_VERSION.to_owned(),
            producer_schema_version: PRODUCER_SCHEMA_VERSION.to_owned(),
            integrity_hash: "sha256:portable-export-0001".to_owned(),
            source_provenance: format!("{LOCAL_DEVICE_ID}@rev-00517"),
            included_state_classes: portable_state_classes(),
            excluded_state_classes: excluded_from_lane(),
            local_authoritative_fallback: true,
            carries_forbidden_state_class: false,
            waiver_ref: None,
            conforms: false,
        },
        SnapshotRow {
            snapshot_class: SnapshotClass::ManagedSyncSnapshot,
            snapshot_ref: "aureline://snapshot/managed-sync-snapshot".to_owned(),
            producer_aureline_version: PRODUCER_AURELINE_VERSION.to_owned(),
            producer_schema_version: PRODUCER_SCHEMA_VERSION.to_owned(),
            integrity_hash: "sha256:managed-sync-0001".to_owned(),
            source_provenance: format!("{REMOTE_DESKTOP_ID}@rev-00142"),
            included_state_classes: managed_included,
            excluded_state_classes: managed_excluded,
            local_authoritative_fallback: true,
            carries_forbidden_state_class: false,
            waiver_ref: managed_waiver,
            conforms: false,
        },
        SnapshotRow {
            snapshot_class: SnapshotClass::SupportRecoveryManifest,
            snapshot_ref: "aureline://snapshot/support-recovery-manifest".to_owned(),
            producer_aureline_version: PRODUCER_AURELINE_VERSION.to_owned(),
            producer_schema_version: PRODUCER_SCHEMA_VERSION.to_owned(),
            integrity_hash: "sha256:support-manifest-0001".to_owned(),
            source_provenance: format!("{LOCAL_DEVICE_ID}@rev-00517"),
            included_state_classes: vec![
                StateClass::ExtensionInventoryRefs,
                StateClass::ReferenceOnlyMetadata,
            ],
            excluded_state_classes: vec![
                StateClass::ScalarSettings,
                StateClass::MachineLocalTopology,
                StateClass::DirtyBufferJournals,
                StateClass::SecretMaterial,
            ],
            local_authoritative_fallback: true,
            carries_forbidden_state_class: false,
            waiver_ref: None,
            conforms: false,
        },
    ]
}

// ---------------------------------------------------------------------------
// Secret boundary
// ---------------------------------------------------------------------------

fn secret_boundary() -> Vec<SecretBoundaryRow> {
    let mut rows = Vec::new();
    for class in StateClass::FORBIDDEN_ON_LANE {
        for lane in ["sync", "export"] {
            rows.push(SecretBoundaryRow {
                state_class: class,
                lane: lane.to_owned(),
                excluded: true,
                reference_only_allowed: matches!(class, StateClass::SecretMaterial),
                reason: match class {
                    StateClass::DirtyBufferJournals => {
                        "Dirty-buffer journals are never auto-synced or exported cross-device."
                            .to_owned()
                    }
                    StateClass::SecretMaterial => {
                        "Raw tokens, passkeys, and private keys never cross the lane; only \
                         reference-only metadata is allowed."
                            .to_owned()
                    }
                    _ => "Excluded from the lane.".to_owned(),
                },
                conforms: false,
            });
        }
    }
    rows
}

// ---------------------------------------------------------------------------
// Surface parity + profile roaming
// ---------------------------------------------------------------------------

fn surface_parity(spec: &ScenarioSpec) -> Vec<SurfaceParityRow> {
    SurfaceClass::REQUIRED
        .into_iter()
        .map(|surface| {
            let preview = spec.device_registry_preview
                && surface == SurfaceClass::AdminDeviceRegistry;
            SurfaceParityRow {
                surface_class: surface,
                consumes_shared_record: true,
                clones_prose: false,
                surface_marker: if preview {
                    LifecycleMarker::Preview
                } else {
                    LifecycleMarker::Stable
                },
                shared_contract_ref: super::model::SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF
                    .to_owned(),
                record_ref: format!("aureline://sync-device-surface/{}", surface.as_str()),
                waiver_ref: preview.then(|| {
                    format!(
                        "aureline://waiver/sync-{}-surface-preview",
                        surface.as_str()
                    )
                }),
                conforms: false,
            }
        })
        .collect()
}

fn profile_roaming(spec: &ScenarioSpec) -> ProfileRoamingSummary {
    let summary = if spec.managed_sync_available {
        "Managed sync is available; the latest successful sync manifest, extension inventory \
         pointer, and remaining-retention timeline are current, temporary profiles are excluded \
         by default, and local launch and edit authority is retained."
            .to_owned()
    } else {
        "Managed sync is unavailable; the device-registry view discloses the narrowing while local \
         launch and edit authority is retained, the latest successful sync manifest stays the local \
         source of truth, and temporary profiles remain excluded by default."
            .to_owned()
    };
    ProfileRoamingSummary {
        latest_successful_sync_ref: Some(
            "aureline://snapshot/managed-sync-snapshot".to_owned(),
        ),
        extension_inventory_ref: Some(
            "aureline://inventory/extensions-reference-only".to_owned(),
        ),
        remaining_retention_days: Some(if spec.managed_sync_available { 90 } else { 45 }),
        managed_sync_available: spec.managed_sync_available,
        local_launch_edit_authority_retained: true,
        temporary_profiles_excluded: true,
        active_profile_durability: ProfileDurabilityClass::Durable,
        originating_profile_revision: format!("{LOCAL_DEVICE_ID}@rev-00517"),
        summary,
        conforms: false,
    }
}

// ---------------------------------------------------------------------------
// Routes, recovery, accessibility
// ---------------------------------------------------------------------------

fn accessibility() -> AccessibilityDisclosure {
    let action_labels: Vec<String> = required_recovery_routes()
        .into_iter()
        .map(|route| route.action_label)
        .collect();
    let layout_modes = LayoutMode::REQUIRED
        .into_iter()
        .map(|mode| LayoutModeDisclosure {
            mode,
            row_narration_available: true,
            recovery_affordances_reachable: true,
        })
        .collect();
    AccessibilityDisclosure {
        focus_order_index: 0,
        tab_stop_count: 6,
        row_narration: "Device & sync registry for the active sync posture".to_owned(),
        action_labels,
        layout_modes,
    }
}

fn routes() -> Vec<EntryRouteRecord> {
    RouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!("aureline://route/sync-device-registry/{}", surface.as_str()),
            keyboard_reachable: true,
            activates_same_record: true,
        })
        .collect()
}

fn upstream(
    scenario_id: &str,
    device_ids: Vec<String>,
    setting_ids: Vec<String>,
) -> CertificationUpstream {
    CertificationUpstream {
        registry_ref: REGISTRY_REF.to_owned(),
        resolver_state_ref: format!("aureline://resolver-state/sync-device-registry-{scenario_id}"),
        sync_contract_ref: SYNC_CONTRACT_REF.to_owned(),
        participating_device_ids: device_ids,
        reviewed_setting_ids: setting_ids,
    }
}

// ---------------------------------------------------------------------------
// Scenario assembly
// ---------------------------------------------------------------------------

fn build_scenario(spec: &ScenarioSpec) -> SyncDeviceRegistryScenario {
    let resolver = configured_resolver();
    let context = inspection_context(&resolver);

    let device_participation = device_participation(spec);
    let conflict_review = conflict_review(&resolver, &context, spec);
    let device_ids: Vec<String> = device_participation
        .iter()
        .map(|row| row.device_id.clone())
        .collect();
    let setting_ids: Vec<String> = conflict_review
        .iter()
        .map(|row| row.setting_id.clone())
        .collect();

    let input = CertificationInput {
        record_id: spec.scenario_id.to_owned(),
        as_of: CORPUS_AS_OF.to_owned(),
        posture_id: spec.posture_id.to_owned(),
        posture_label: spec.posture_label.to_owned(),
        title: spec.title.to_owned(),
        summary: spec.summary.to_owned(),
        device_participation,
        conflict_review,
        snapshots: snapshots(spec),
        secret_boundary: secret_boundary(),
        surface_parity: surface_parity(spec),
        profile_roaming: profile_roaming(spec),
        claim_ceiling: spec.claim_ceiling,
        recovery_routes: required_recovery_routes(),
        routes: routes(),
        accessibility: accessibility(),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(spec.scenario_id, device_ids, setting_ids),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_owned(),
        support_export_ref: SUPPORT_EXPORT_REF.to_owned(),
        evidence_refs: vec![
            EVIDENCE_ARTIFACT_REF.to_owned(),
            EVIDENCE_FIXTURE_REF.to_owned(),
        ],
        narrative_refs: vec![NARRATIVE_REF.to_owned()],
    };
    let record = SyncDeviceRegistryCertification::build(input)
        .unwrap_or_else(|err| panic!("scenario {} must build: {err}", spec.scenario_id));

    SyncDeviceRegistryScenario {
        scenario_id: spec.scenario_id,
        fixture_filename: format!("{}.json", spec.scenario_id),
        expected_posture: record.posture_id.clone(),
        expected_claim_class: record.stable_qualification.claim_class,
        expected_qualifies_stable: record.stable_qualification.qualifies_stable,
        expected_surface_marker: record.surface_lifecycle_marker,
        record,
    }
}

/// Returns the deterministic claimed-stable certification matrix.
pub fn sync_device_registry_corpus() -> Vec<SyncDeviceRegistryScenario> {
    let specs = [
        ScenarioSpec {
            scenario_id: "nominal",
            posture_id: "sync_device_registry_nominal",
            posture_label: "Nominal sync and device registry",
            title: "Sync, device registry, and profile portability are certified replacement-grade",
            summary: "Every device exposes its participation truth; conflict review is field-aware \
                      across exact-match, translated, partial, stale-remote, policy-locked, and \
                      local-authoritative outcomes; every snapshot class carries provenance; \
                      overwrites are protected; the secret boundary holds; merge rules are \
                      enforced; profile roaming is complete; and every surface shares one truth.",
            managed_sync_available: true,
            stale_remote_device: false,
            secret_boundary_drill: false,
            unprotected_overwrite_drill: false,
            device_registry_preview: false,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "stale_remote_local_authoritative",
            posture_id: "sync_device_registry_stale_remote",
            posture_label: "Stale remote device, local authoritative",
            title: "A stale remote device stays inspectable while local stays authoritative",
            summary: "A remote device's bundle is stale; the device stays inspectable without a \
                      mutating action, retains its rollback checkpoint, and the local profile stays \
                      authoritative, so the stale copy never masquerades as healthy continuity.",
            managed_sync_available: true,
            stale_remote_device: true,
            secret_boundary_drill: false,
            unprotected_overwrite_drill: false,
            device_registry_preview: false,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "managed_sync_unavailable",
            posture_id: "sync_device_registry_managed_unavailable",
            posture_label: "Managed sync unavailable, local authority retained",
            title: "Managed sync is unavailable without losing local launch or edit authority",
            summary: "Managed sync is unavailable; the device-registry view discloses the narrowing \
                      across every surface while local launch and edit authority is retained and \
                      temporary profiles stay excluded, so offline or blocked sync never overstates \
                      portability.",
            managed_sync_available: false,
            stale_remote_device: false,
            secret_boundary_drill: false,
            unprotected_overwrite_drill: false,
            device_registry_preview: false,
            claim_ceiling: full_claim_ceiling(),
        },
        ScenarioSpec {
            scenario_id: "secret_boundary_drill",
            posture_id: "sync_device_registry_secret_boundary_drill",
            posture_label: "Managed snapshot leaks a dirty-buffer journal",
            title: "A snapshot that leaks a forbidden state class narrows below Stable",
            summary: "An adversarial managed sync snapshot lists a dirty-buffer journal among its \
                      included state classes; the lane detects the secret-boundary violation and \
                      narrows the posture below Stable with a named reason rather than shipping the \
                      leak.",
            managed_sync_available: true,
            stale_remote_device: false,
            secret_boundary_drill: true,
            unprotected_overwrite_drill: false,
            device_registry_preview: false,
            claim_ceiling: CertificationClaimCeiling {
                asserts_secret_boundary_held: false,
                ..full_claim_ceiling()
            },
        },
        ScenarioSpec {
            scenario_id: "unprotected_overwrite_drill",
            posture_id: "sync_device_registry_unprotected_overwrite_drill",
            posture_label: "Overwrite without a checkpoint or preview",
            title: "An unprotected overwrite narrows below Stable instead of applying",
            summary: "A conflict row would overwrite a local scope with no structured change \
                      preview and no rollback checkpoint; the lane refuses to certify the posture \
                      Stable and narrows it with a named reason, keeping the local source of truth.",
            managed_sync_available: true,
            stale_remote_device: false,
            secret_boundary_drill: false,
            unprotected_overwrite_drill: true,
            device_registry_preview: false,
            claim_ceiling: CertificationClaimCeiling {
                asserts_local_fallback_proven: false,
                ..full_claim_ceiling()
            },
        },
        ScenarioSpec {
            scenario_id: "device_registry_view_in_preview",
            posture_id: "sync_device_registry_view_in_preview",
            posture_label: "Admin device-registry view still in Preview",
            title: "A below-Stable surface narrows the certification instead of inheriting green",
            summary: "Every pillar holds, but the admin device-registry view treatment still \
                      carries a Preview marker; the posture is narrowed below Stable by its lowest \
                      surface marker rather than inheriting an adjacent green row.",
            managed_sync_available: true,
            stale_remote_device: false,
            secret_boundary_drill: false,
            unprotected_overwrite_drill: false,
            device_registry_preview: true,
            claim_ceiling: full_claim_ceiling(),
        },
    ];
    let scenarios: Vec<SyncDeviceRegistryScenario> = specs.iter().map(build_scenario).collect();
    debug_assert!(scenarios.iter().all(|scenario| {
        is_canonical_object_ref(&scenario.record.diagnostics_export_ref)
            && is_canonical_object_ref(&scenario.record.support_export_ref)
    }));
    scenarios
}
