//! Frozen M5 setting-definition, write-intent, sync-conflict, and capability-lifecycle execution matrix.
//!
//! This module locks Aureline's concrete settings-resolver, sync-conflict, and capability-lifecycle
//! runtime behavior into one export-safe packet. Every claimed M5 configuration-bearing verb — resolve an
//! effective setting, land a write intent, sync a scope bundle across devices, migrate a settings schema,
//! and roll out a capability lifecycle — is named once here and constrained by the same shared
//! settings-governance-role taxonomy (setting_definition, effective_resolution, write_intent,
//! policy_constraint, sync_conflict, schema_migration, capability_lifecycle), the same
//! stable-setting-ids-are-never-recycled rule, the same
//! winning-scope-shadowed-values-restart-posture-and-lock-source-stay-inspectable rule, the same
//! writes-land-only-in-the-chosen-artifact-and-scope-with-preview-checkpoint-and-rollback rule, the same
//! sync-never-silently-overwrites-local-authoritative-state rule, and the same
//! lifecycle-and-kill-switch-state-preserves-user-data-and-explains-itself rule regardless of the surface
//! that renders it.
//!
//! The matrix does not redesign settings-row chrome or generic admin dashboards — it is the shared
//! reusable settings-resolver, sync-conflict, and capability-lifecycle engine contract those
//! already-governed surfaces consume, and it binds back to the already-landed effective-setting and
//! capability-lifecycle packets instead of leaving configuration truth split across scattered settings
//! copy and hand-copied admin prose. The controlled vocabularies are frozen in one self-describing
//! [`M5SettingsGovernanceVocabularySet`] rather than minted per surface. The single controlled
//! settings-governance-role vocabulary consumers bind to — setting_definition, effective_resolution,
//! write_intent, policy_constraint, sync_conflict, schema_migration, and capability_lifecycle — keeps the
//! setting definition and the effective resolution separately inspectable; keeps stable setting IDs from
//! being recycled; keeps winning scope, shadowed values, restart posture, and lock source inspectable;
//! keeps writes landing only in the chosen artifact and scope with preview / checkpoint / rollback
//! evidence; keeps sync from silently overwriting local authoritative state during outages; keeps
//! machine-only state from masquerading as portable; and keeps lifecycle, experiment, and kill-switch
//! state visible and self-explaining. Raw secret values and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_settings_governance_matrix,
    seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed,
    seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed,
    M5_SETTINGS_GOVERNANCE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SettingsGovernanceMatrixPacket`].
pub const M5_SETTINGS_GOVERNANCE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_setting_definition_write_intent_sync_conflict_and_capability_lifecycle_matrix";

/// Schema version for M5 settings-governance matrix records.
pub const M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined settings-governance matrix schema.
pub const M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF: &str =
    "schemas/config/m5-settings-resolver-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF: &str =
    "docs/settings/m5_settings_resolver_contract.md";

/// Repo-relative path of the canonical setting-definition domain schema (resolve-setting and
/// migrate-schema families: how a setting is defined, its stable ID preserved, and its effective value
/// resolved from the winning scope).
pub const M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF: &str =
    "schemas/config/m5-setting-definition.schema.json";

/// Repo-relative path of the canonical setting-write-intent domain schema (write-setting family: the
/// chosen artifact and scope, and the preview / checkpoint / rollback evidence a write materializes).
pub const M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF: &str =
    "schemas/config/m5-setting-write-intent.schema.json";

/// Repo-relative path of the canonical sync-conflict-packet domain schema (sync-scope family: sync scope
/// bundles, sessions, conflict packets, and device actions that never silently overwrite local state).
pub const M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF: &str =
    "schemas/config/m5-sync-conflict-packet.schema.json";

/// Repo-relative path of the canonical capability-lifecycle domain schema (rollout-capability family:
/// capability records, Labs enrollment, rollout plans, dependency markers, and kill-switch records).
pub const M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF: &str =
    "schemas/config/m5-capability-lifecycle.schema.json";

/// Repo-relative path of the already-landed effective-setting schema the matrix binds back to.
pub const M5_EFFECTIVE_SETTING_SCHEMA_REF: &str = "schemas/config/effective_setting.schema.json";

/// Repo-relative path of the already-landed capability-lifecycle schema the settings-governance matrix
/// binds back to.
pub const M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF: &str =
    "schemas/governance/capability_lifecycle.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SETTINGS_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/config/m5-settings-runtime";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SETTINGS_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/release/m5-settings-governance-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SETTINGS_GOVERNANCE_CSV_REF: &str =
    "artifacts/release/m5-settings-governance-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SETTINGS_GOVERNANCE_REPORT_REF: &str =
    "artifacts/config/m5-settings-resolver-matrix.md";

/// One of the five governed configuration-runtime families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceFamily {
    /// Resolve an effective setting from the winning scope, never recycling a stable setting ID.
    ResolveSetting,
    /// Land a write intent in the chosen artifact and scope with preview / checkpoint / rollback evidence.
    WriteSetting,
    /// Sync a scope bundle across devices, never silently overwriting local authoritative state.
    SyncScope,
    /// Migrate a settings schema across versions, preserving setting-ID continuity with a checkpoint.
    MigrateSchema,
    /// Roll out a capability lifecycle, keeping kill-switch and policy-disable causes self-explaining.
    RolloutCapability,
}

impl M5SettingsGovernanceFamily {
    /// Every governed configuration-runtime family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ResolveSetting,
        Self::WriteSetting,
        Self::SyncScope,
        Self::MigrateSchema,
        Self::RolloutCapability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveSetting => "resolve_setting",
            Self::WriteSetting => "write_setting",
            Self::SyncScope => "sync_scope",
            Self::MigrateSchema => "migrate_schema",
            Self::RolloutCapability => "rollout_capability",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's setting-definition, write-intent, sync-conflict, or capability-lifecycle meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ResolveSetting | Self::MigrateSchema => M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
            Self::WriteSetting => M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
            Self::SyncScope => M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
            Self::RolloutCapability => M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled resolve-setting role.
    pub const fn declares_resolve_setting_roles(self) -> bool {
        matches!(self, Self::ResolveSetting)
    }

    /// `true` when this family must name a controlled write-setting role.
    pub const fn declares_write_setting_roles(self) -> bool {
        matches!(self, Self::WriteSetting)
    }

    /// `true` when this family must name a controlled sync-scope role.
    pub const fn declares_sync_scope_roles(self) -> bool {
        matches!(self, Self::SyncScope)
    }

    /// `true` when this family must name a controlled migrate-schema role.
    pub const fn declares_migrate_schema_roles(self) -> bool {
        matches!(self, Self::MigrateSchema)
    }

    /// `true` when this family must name a controlled rollout-capability role.
    pub const fn declares_rollout_capability_roles(self) -> bool {
        matches!(self, Self::RolloutCapability)
    }
}

/// The single controlled settings-governance-role vocabulary every settings, shell, diagnostics, admin,
/// docs, or support consumer binds to. These are the exact acceptance-criteria tokens that keep
/// `setting_definition`, `effective_resolution`, `write_intent`, `policy_constraint`, `sync_conflict`,
/// `schema_migration`, and `capability_lifecycle` meaning the same thing everywhere the settings-governance
/// grammar ships. No surface invents a parallel word for any of these roles, and the write-intent /
/// policy-constraint / sync-conflict / capability-lifecycle roles may never widen a scope, silently
/// overwrite local authoritative state, hide a lifecycle dependency behind unpublished markers, or hide a
/// kill-switch or policy-disable cause behind generic unavailable copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceRole {
    /// Setting-definition role (how a setting is defined and its stable ID preserved).
    SettingDefinition,
    /// Effective-resolution role (the winning scope, shadowed values, restart posture, and lock source).
    EffectiveResolution,
    /// Write-intent role (the chosen artifact and scope, and the preview / checkpoint / rollback evidence).
    WriteIntent,
    /// Policy-constraint role (the lock, constraint, or DisabledByPolicy posture a setting carries).
    PolicyConstraint,
    /// Sync-conflict role (the sync scope bundle, session, conflict packet, and device action).
    SyncConflict,
    /// Schema-migration role (the schema-migration record and setting-ID continuity across versions).
    SchemaMigration,
    /// Capability-lifecycle role (the capability record, Labs enrollment, rollout plan, and kill switch).
    CapabilityLifecycle,
}

impl M5SettingsGovernanceRole {
    /// Every settings-governance role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SettingDefinition,
        Self::EffectiveResolution,
        Self::WriteIntent,
        Self::PolicyConstraint,
        Self::SyncConflict,
        Self::SchemaMigration,
        Self::CapabilityLifecycle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingDefinition => "setting_definition",
            Self::EffectiveResolution => "effective_resolution",
            Self::WriteIntent => "write_intent",
            Self::PolicyConstraint => "policy_constraint",
            Self::SyncConflict => "sync_conflict",
            Self::SchemaMigration => "schema_migration",
            Self::CapabilityLifecycle => "capability_lifecycle",
        }
    }

    /// Whether this role carries write-intent, policy-constraint, sync-conflict, or capability-lifecycle
    /// truth whose per-family behavior must never widen a scope, silently overwrite local authoritative
    /// state, hide a lifecycle dependency behind unpublished markers, or hide a kill-switch or
    /// policy-disable cause behind generic unavailable copy (`write_intent`, `policy_constraint`,
    /// `sync_conflict`, `capability_lifecycle`). The descriptive structure roles (`setting_definition`,
    /// `effective_resolution`, `schema_migration`) are inspectable descriptors rather than trust-carrying
    /// truth and so do not carry this requirement.
    pub const fn must_preserve_evidence_and_disclose_cause_before_applying(self) -> bool {
        matches!(
            self,
            Self::WriteIntent
                | Self::PolicyConstraint
                | Self::SyncConflict
                | Self::CapabilityLifecycle
        )
    }
}

/// Controlled resolve-setting role — how resolving an effective setting is named, so the effective value
/// resolved from the winning scope, the shadowed values and scope chain kept inspectable, the restart
/// posture and lock source disclosed, and the stable setting ID preserved follow one settings-governance
/// registry rather than recycling a retired setting ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResolveSettingRole {
    /// Effective value resolved from the winning scope.
    EffectiveValueResolvedFromWinningScope,
    /// Shadowed values and the scope chain kept inspectable.
    ShadowedValuesAndScopeChainInspectable,
    /// Restart posture and lock source disclosed.
    RestartPostureAndLockSourceDisclosed,
    /// Stable setting ID preserved, never recycled.
    StableSettingIdPreservedNeverRecycled,
    /// A role bound to the single settings-governance registry.
    BoundToSettingsGovernanceRegistry,
    /// A recycled retired setting ID, which is disallowed.
    RecycledRetiredSettingIdDisallowed,
}

impl M5ResolveSettingRole {
    /// Every resolve-setting role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EffectiveValueResolvedFromWinningScope,
        Self::ShadowedValuesAndScopeChainInspectable,
        Self::RestartPostureAndLockSourceDisclosed,
        Self::StableSettingIdPreservedNeverRecycled,
        Self::BoundToSettingsGovernanceRegistry,
        Self::RecycledRetiredSettingIdDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveValueResolvedFromWinningScope => {
                "effective_value_resolved_from_winning_scope"
            }
            Self::ShadowedValuesAndScopeChainInspectable => {
                "shadowed_values_and_scope_chain_inspectable"
            }
            Self::RestartPostureAndLockSourceDisclosed => {
                "restart_posture_and_lock_source_disclosed"
            }
            Self::StableSettingIdPreservedNeverRecycled => {
                "stable_setting_id_preserved_never_recycled"
            }
            Self::BoundToSettingsGovernanceRegistry => "bound_to_settings_governance_registry",
            Self::RecycledRetiredSettingIdDisallowed => "recycled_retired_setting_id_disallowed",
        }
    }
}

/// Controlled write-setting role — how landing a write intent is named, so the write intent targeting the
/// chosen artifact and scope, the preview / checkpoint / rollback evidence created, the material behavior
/// change disclosed before apply, and the chosen scope preserved follow one settings-governance registry
/// rather than widening a scoped write into a broader scope because it is easier downstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WriteSettingRole {
    /// Write intent targets the chosen artifact and scope.
    WriteIntentTargetsChosenArtifactAndScope,
    /// Preview, checkpoint, and rollback evidence created.
    PreviewCheckpointAndRollbackEvidenceCreated,
    /// Material behavior change disclosed before apply.
    MaterialBehaviorChangeDisclosedBeforeApply,
    /// Chosen scope preserved, never widened.
    ChosenScopePreservedNeverWidened,
    /// A role bound to the single settings-governance registry.
    BoundToSettingsGovernanceRegistry,
    /// A scope widened for convenience, which is disallowed.
    ScopeWidenedForConvenienceDisallowed,
}

impl M5WriteSettingRole {
    /// Every write-setting role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WriteIntentTargetsChosenArtifactAndScope,
        Self::PreviewCheckpointAndRollbackEvidenceCreated,
        Self::MaterialBehaviorChangeDisclosedBeforeApply,
        Self::ChosenScopePreservedNeverWidened,
        Self::BoundToSettingsGovernanceRegistry,
        Self::ScopeWidenedForConvenienceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WriteIntentTargetsChosenArtifactAndScope => {
                "write_intent_targets_chosen_artifact_and_scope"
            }
            Self::PreviewCheckpointAndRollbackEvidenceCreated => {
                "preview_checkpoint_and_rollback_evidence_created"
            }
            Self::MaterialBehaviorChangeDisclosedBeforeApply => {
                "material_behavior_change_disclosed_before_apply"
            }
            Self::ChosenScopePreservedNeverWidened => "chosen_scope_preserved_never_widened",
            Self::BoundToSettingsGovernanceRegistry => "bound_to_settings_governance_registry",
            Self::ScopeWidenedForConvenienceDisallowed => {
                "scope_widened_for_convenience_disallowed"
            }
        }
    }
}

/// Controlled sync-scope role — how syncing a scope bundle across devices is named, so the sync scope
/// bundle and session resolved, the conflict packet surfaced rather than auto-overwritten, the local
/// authoritative state preserved during an outage, and machine-only state never marked portable follow one
/// settings-governance registry rather than silently overwriting local authoritative state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncScopeRole {
    /// Sync scope bundle and session resolved.
    SyncScopeBundleAndSessionResolved,
    /// Conflict packet surfaced, never auto-overwritten.
    ConflictPacketSurfacedNeverAutoOverwritten,
    /// Local authoritative state preserved during an outage.
    LocalAuthoritativeStatePreservedDuringOutage,
    /// Machine-only state never marked portable.
    MachineOnlyStateNeverMarkedPortable,
    /// A role bound to the single settings-governance registry.
    BoundToSettingsGovernanceRegistry,
    /// A silent overwrite of local authoritative state, which is disallowed.
    SilentOverwriteOfLocalStateDisallowed,
}

impl M5SyncScopeRole {
    /// Every sync-scope role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SyncScopeBundleAndSessionResolved,
        Self::ConflictPacketSurfacedNeverAutoOverwritten,
        Self::LocalAuthoritativeStatePreservedDuringOutage,
        Self::MachineOnlyStateNeverMarkedPortable,
        Self::BoundToSettingsGovernanceRegistry,
        Self::SilentOverwriteOfLocalStateDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncScopeBundleAndSessionResolved => "sync_scope_bundle_and_session_resolved",
            Self::ConflictPacketSurfacedNeverAutoOverwritten => {
                "conflict_packet_surfaced_never_auto_overwritten"
            }
            Self::LocalAuthoritativeStatePreservedDuringOutage => {
                "local_authoritative_state_preserved_during_outage"
            }
            Self::MachineOnlyStateNeverMarkedPortable => "machine_only_state_never_marked_portable",
            Self::BoundToSettingsGovernanceRegistry => "bound_to_settings_governance_registry",
            Self::SilentOverwriteOfLocalStateDisallowed => {
                "silent_overwrite_of_local_state_disallowed"
            }
        }
    }
}

/// Controlled migrate-schema role — how migrating a settings schema across versions is named, so the
/// schema-migration record resolved, the setting-ID continuity preserved across versions, the migration
/// preview shown before rewrite, and the reversible migration checkpoint recorded follow one
/// settings-governance registry rather than silently rewriting a schema without a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrateSchemaRole {
    /// Schema-migration record resolved.
    SchemaMigrationRecordResolved,
    /// Setting-ID continuity preserved across versions.
    SettingIdContinuityPreservedAcrossVersions,
    /// Migration preview shown before rewrite.
    MigrationPreviewShownBeforeRewrite,
    /// Reversible migration checkpoint recorded.
    ReversibleMigrationCheckpointRecorded,
    /// A role bound to the single settings-governance registry.
    BoundToSettingsGovernanceRegistry,
    /// A silent schema rewrite without a checkpoint, which is disallowed.
    SilentSchemaRewriteWithoutCheckpointDisallowed,
}

impl M5MigrateSchemaRole {
    /// Every migrate-schema role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SchemaMigrationRecordResolved,
        Self::SettingIdContinuityPreservedAcrossVersions,
        Self::MigrationPreviewShownBeforeRewrite,
        Self::ReversibleMigrationCheckpointRecorded,
        Self::BoundToSettingsGovernanceRegistry,
        Self::SilentSchemaRewriteWithoutCheckpointDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaMigrationRecordResolved => "schema_migration_record_resolved",
            Self::SettingIdContinuityPreservedAcrossVersions => {
                "setting_id_continuity_preserved_across_versions"
            }
            Self::MigrationPreviewShownBeforeRewrite => "migration_preview_shown_before_rewrite",
            Self::ReversibleMigrationCheckpointRecorded => {
                "reversible_migration_checkpoint_recorded"
            }
            Self::BoundToSettingsGovernanceRegistry => "bound_to_settings_governance_registry",
            Self::SilentSchemaRewriteWithoutCheckpointDisallowed => {
                "silent_schema_rewrite_without_checkpoint_disallowed"
            }
        }
    }
}

/// Controlled rollout-capability role — how rolling out a capability lifecycle is named, so the capability
/// lifecycle state resolved, the Labs and rollout dependency markers published, the kill-switch and
/// policy-disable cause explained, and the disabled state preserving user data follow one
/// settings-governance registry rather than hiding a lifecycle dependency on unpublished flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RolloutCapabilityRole {
    /// Capability lifecycle state resolved.
    CapabilityLifecycleStateResolved,
    /// Labs and rollout dependency markers published.
    LabsAndRolloutDependencyMarkersPublished,
    /// Kill-switch and policy-disable cause explained.
    KillSwitchAndPolicyDisableCauseExplained,
    /// Disabled state preserves user data.
    DisabledStatePreservesUserData,
    /// A role bound to the single settings-governance registry.
    BoundToSettingsGovernanceRegistry,
    /// A hidden lifecycle dependency on unpublished flags, which is disallowed.
    HiddenLifecycleDependencyOnUnpublishedFlagsDisallowed,
}

impl M5RolloutCapabilityRole {
    /// Every rollout-capability role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CapabilityLifecycleStateResolved,
        Self::LabsAndRolloutDependencyMarkersPublished,
        Self::KillSwitchAndPolicyDisableCauseExplained,
        Self::DisabledStatePreservesUserData,
        Self::BoundToSettingsGovernanceRegistry,
        Self::HiddenLifecycleDependencyOnUnpublishedFlagsDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityLifecycleStateResolved => "capability_lifecycle_state_resolved",
            Self::LabsAndRolloutDependencyMarkersPublished => {
                "labs_and_rollout_dependency_markers_published"
            }
            Self::KillSwitchAndPolicyDisableCauseExplained => {
                "kill_switch_and_policy_disable_cause_explained"
            }
            Self::DisabledStatePreservesUserData => "disabled_state_preserves_user_data",
            Self::BoundToSettingsGovernanceRegistry => "bound_to_settings_governance_registry",
            Self::HiddenLifecycleDependencyOnUnpublishedFlagsDisallowed => {
                "hidden_lifecycle_dependency_on_unpublished_flags_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a settings-governance family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceSurfaceFamily {
    /// The settings surface.
    Settings,
    /// The shell surface.
    Shell,
    /// The diagnostics surface.
    Diagnostics,
    /// The admin surface.
    Admin,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5SettingsGovernanceSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Settings,
        Self::Shell,
        Self::Diagnostics,
        Self::Admin,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Shell => "shell",
            Self::Diagnostics => "diagnostics",
            Self::Admin => "admin",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Configuration context a family must survive with the same truth, so a family's setting-definition,
/// effective-resolution, write-intent, policy-constraint, sync-conflict, schema-migration, or
/// capability-lifecycle meaning never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceDeploymentLine {
    /// A fresh-install configuration on a new machine.
    FreshInstall,
    /// A returning-profile configuration.
    ReturningProfile,
    /// An offline or sync-outage configuration.
    OfflineOrOutage,
    /// A policy-managed-fleet configuration.
    PolicyManagedFleet,
    /// A configuration resumed after a sync conflict.
    ResumedAfterSyncConflict,
}

impl M5SettingsGovernanceDeploymentLine {
    /// Every configuration context, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FreshInstall,
        Self::ReturningProfile,
        Self::OfflineOrOutage,
        Self::PolicyManagedFleet,
        Self::ResumedAfterSyncConflict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshInstall => "fresh_install",
            Self::ReturningProfile => "returning_profile",
            Self::OfflineOrOutage => "offline_or_outage",
            Self::PolicyManagedFleet => "policy_managed_fleet",
            Self::ResumedAfterSyncConflict => "resumed_after_sync_conflict",
        }
    }
}

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceConsumerSurface {
    /// The settings resolver.
    SettingsResolver,
    /// The shell UI.
    ShellUi,
    /// The sync service.
    SyncService,
    /// The policy service.
    PolicyService,
    /// The capability service.
    CapabilityService,
    /// The diagnostics surface.
    Diagnostics,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
}

impl M5SettingsGovernanceConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SettingsResolver,
        Self::ShellUi,
        Self::SyncService,
        Self::PolicyService,
        Self::CapabilityService,
        Self::Diagnostics,
        Self::DocsHelp,
        Self::CliExport,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsResolver => "settings_resolver",
            Self::ShellUi => "shell_ui",
            Self::SyncService => "sync_service",
            Self::PolicyService => "policy_service",
            Self::CapabilityService => "capability_service",
            Self::Diagnostics => "diagnostics",
            Self::DocsHelp => "docs_help",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no settings-governance meaning disappears
/// under zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5SettingsGovernanceAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a settings-governance family has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The setting-definition registry source is unavailable.
    SettingDefinitionSourceUnavailable,
    /// The write-intent source is unavailable.
    WriteIntentSourceUnavailable,
    /// Policy-constraint evidence is unverified.
    PolicyConstraintEvidenceUnverified,
    /// Sync-conflict evidence is unverified.
    SyncConflictEvidenceUnverified,
    /// Capability-lifecycle evidence is unavailable.
    CapabilityLifecycleEvidenceUnavailable,
}

impl M5SettingsGovernanceDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::SettingDefinitionSourceUnavailable,
        Self::WriteIntentSourceUnavailable,
        Self::PolicyConstraintEvidenceUnverified,
        Self::SyncConflictEvidenceUnverified,
        Self::CapabilityLifecycleEvidenceUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::SettingDefinitionSourceUnavailable => "setting_definition_source_unavailable",
            Self::WriteIntentSourceUnavailable => "write_intent_source_unavailable",
            Self::PolicyConstraintEvidenceUnverified => "policy_constraint_evidence_unverified",
            Self::SyncConflictEvidenceUnverified => "sync_conflict_evidence_unverified",
            Self::CapabilityLifecycleEvidenceUnavailable => {
                "capability_lifecycle_evidence_unavailable"
            }
        }
    }
}

/// Mandatory label a claimed settings-governance family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// winning scope, the write intent, and the lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's settings-governance role.
    SemanticRole,
    /// The canonical registry reference the family points at.
    RegistryReference,
    /// The winning scope the family resolves.
    WinningScope,
    /// The write intent the family lands.
    WriteIntent,
    /// The lifecycle state the family discloses.
    LifecycleState,
}

impl M5SettingsGovernanceRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::WinningScope,
        Self::WriteIntent,
        Self::LifecycleState,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::WinningScope => "winning_scope",
            Self::WriteIntent => "write_intent",
            Self::LifecycleState => "lifecycle_state",
        }
    }
}

/// Qualification class for an M5 settings-governance row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5SettingsGovernanceQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a settings-governance family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SettingsGovernanceDowngradeTrigger {
    /// A retired setting ID was recycled.
    RecycledARetiredSettingId,
    /// A scoped write was rewritten into a broader scope.
    RewroteAScopedWriteIntoABroaderScope,
    /// Sync silently overwrote locked or machine-only state during an outage.
    SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
    /// A lifecycle or experiment dependency was hidden behind unpublished markers.
    HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
    /// A kill-switch or policy-disable cause was hidden behind generic unavailable copy.
    HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
    /// A scope boundary drifted by surface instead of following one registry.
    ScopeBoundaryDriftedBySurface,
    /// A family left its winning scope unstated.
    WinningScopeUnstated,
    /// A family left its write intent unstated.
    WriteIntentUnstated,
    /// A family left its lifecycle state unstated.
    LifecycleStateUnstated,
    /// A family left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A family left its sync-conflict rule unstated.
    SyncConflictRuleUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5SettingsGovernanceDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RecycledARetiredSettingId,
        Self::RewroteAScopedWriteIntoABroaderScope,
        Self::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync,
        Self::HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers,
        Self::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy,
        Self::ScopeBoundaryDriftedBySurface,
        Self::WinningScopeUnstated,
        Self::WriteIntentUnstated,
        Self::LifecycleStateUnstated,
        Self::RegistryReferenceUnstated,
        Self::SyncConflictRuleUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecycledARetiredSettingId => "recycled_a_retired_setting_id",
            Self::RewroteAScopedWriteIntoABroaderScope => {
                "rewrote_a_scoped_write_into_a_broader_scope"
            }
            Self::SilentlyOverwroteLockedOrMachineOnlyStateDuringSync => {
                "silently_overwrote_locked_or_machine_only_state_during_sync"
            }
            Self::HidLifecycleOrExperimentDependencyBehindUnpublishedMarkers => {
                "hid_lifecycle_or_experiment_dependency_behind_unpublished_markers"
            }
            Self::HidKillSwitchOrPolicyDisableCauseBehindGenericUnavailableCopy => {
                "hid_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy"
            }
            Self::ScopeBoundaryDriftedBySurface => "scope_boundary_drifted_by_surface",
            Self::WinningScopeUnstated => "winning_scope_unstated",
            Self::WriteIntentUnstated => "write_intent_unstated",
            Self::LifecycleStateUnstated => "lifecycle_state_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::SyncConflictRuleUnstated => "sync_conflict_rule_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed settings-governance family bound to the surface-specific truth it
/// must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceRow {
    /// Governed settings-governance family.
    pub settings_governance_family: M5SettingsGovernanceFamily,
    /// Qualification class earned by this family.
    pub qualification: M5SettingsGovernanceQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5SettingsGovernanceSurfaceFamily>,
    /// Configuration contexts this family keeps the same truth across.
    pub deployment_lines: Vec<M5SettingsGovernanceDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5SettingsGovernanceRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5SettingsGovernanceRequiredLabel>,
    /// Settings-governance roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5SettingsGovernanceRole>,
    /// Resolve-setting roles this family names (resolve-setting family only).
    pub resolve_setting_roles: Vec<M5ResolveSettingRole>,
    /// Write-setting roles this family names (write-setting family only).
    pub write_setting_roles: Vec<M5WriteSettingRole>,
    /// Sync-scope roles this family names (sync-scope family only).
    pub sync_scope_roles: Vec<M5SyncScopeRole>,
    /// Migrate-schema roles this family names (migrate-schema family only).
    pub migrate_schema_roles: Vec<M5MigrateSchemaRole>,
    /// Rollout-capability roles this family names (rollout-capability family only).
    pub rollout_capability_roles: Vec<M5RolloutCapabilityRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5SettingsGovernanceDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5SettingsGovernanceAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5SettingsGovernanceConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5SettingsGovernanceDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never recycles a retired setting ID. MUST be `false`.
    pub recycles_a_retired_setting_id: bool,
    /// Hard invariant: this family never rewrites a scoped (Workspace/Profile) write into a broader
    /// (User/Machine) scope because it is easier downstream. MUST be `false`.
    pub rewrites_a_scoped_write_into_a_broader_scope: bool,
    /// Hard invariant: this family never silently overwrites locked or machine-only state during sync.
    /// MUST be `false`.
    pub silently_overwrites_locked_or_machine_only_state_during_sync: bool,
    /// Hard invariant: this family never hides a lifecycle or experiment dependency behind unpublished
    /// markers. MUST be `false`.
    pub hides_lifecycle_or_experiment_dependency_behind_unpublished_markers: bool,
    /// Hard invariant: this family never hides a kill-switch or policy-disable cause behind generic
    /// unavailable copy. MUST be `false`.
    pub hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy: bool,
}

impl M5SettingsGovernanceRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5SettingsGovernanceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5SettingsGovernanceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.recycles_a_retired_setting_id
            && !self.rewrites_a_scoped_write_into_a_broader_scope
            && !self.silently_overwrites_locked_or_machine_only_state_during_sync
            && !self.hides_lifecycle_or_experiment_dependency_behind_unpublished_markers
            && !self.hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceVocabularySet {
    /// Settings-governance-family tokens.
    pub settings_governance_families: Vec<String>,
    /// Settings-governance-role tokens.
    pub semantic_roles: Vec<String>,
    /// Resolve-setting-role tokens.
    pub resolve_setting_roles: Vec<String>,
    /// Write-setting-role tokens.
    pub write_setting_roles: Vec<String>,
    /// Sync-scope-role tokens.
    pub sync_scope_roles: Vec<String>,
    /// Migrate-schema-role tokens.
    pub migrate_schema_roles: Vec<String>,
    /// Rollout-capability-role tokens.
    pub rollout_capability_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Configuration-context tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5SettingsGovernanceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            settings_governance_families: tokens(&M5SettingsGovernanceFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5SettingsGovernanceRole::ALL, |v| v.as_str()),
            resolve_setting_roles: tokens(&M5ResolveSettingRole::ALL, |v| v.as_str()),
            write_setting_roles: tokens(&M5WriteSettingRole::ALL, |v| v.as_str()),
            sync_scope_roles: tokens(&M5SyncScopeRole::ALL, |v| v.as_str()),
            migrate_schema_roles: tokens(&M5MigrateSchemaRole::ALL, |v| v.as_str()),
            rollout_capability_roles: tokens(&M5RolloutCapabilityRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5SettingsGovernanceSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5SettingsGovernanceDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5SettingsGovernanceConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5SettingsGovernanceAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5SettingsGovernanceDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5SettingsGovernanceRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5SettingsGovernanceDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceGovernanceReview {
    /// Setting definition and effective resolution stay separately inspectable.
    pub setting_definition_and_effective_resolution_stay_separately_inspectable: bool,
    /// Stable setting IDs are never recycled.
    pub setting_ids_are_never_recycled: bool,
    /// Scoped writes are never widened into a broader scope.
    pub scoped_writes_are_never_widened_into_broader_scope: bool,
    /// Winning scope, shadowed values, restart posture, and lock source stay inspectable.
    pub winning_scope_shadowed_values_restart_posture_and_lock_source_stay_inspectable: bool,
    /// Writes land only in the chosen artifact and scope with preview / checkpoint / rollback evidence.
    pub writes_land_only_in_chosen_artifact_and_scope_with_preview_checkpoint_rollback: bool,
    /// Sync never silently overwrites local authoritative state during outages.
    pub sync_never_silently_overwrites_local_authoritative_state_during_outages: bool,
    /// Machine-only state never masquerades as portable.
    pub machine_only_state_never_masquerades_as_portable: bool,
    /// Lifecycle and experiment dependencies stay visible across surfaces.
    pub lifecycle_and_experiment_dependencies_stay_visible_across_surfaces: bool,
    /// Kill-switch and DisabledByPolicy states preserve user data and explain themselves.
    pub kill_switch_and_disabled_by_policy_states_preserve_user_data_and_explain_themselves: bool,
    /// Every family keeps the same truth across every configuration context.
    pub every_family_declares_deployment_contexts: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical settings-governance source.
    pub support_export_reads_single_settings_governance_source: bool,
    /// Settings, shell, diagnostics, and admin bind to a single canonical settings-governance source.
    pub settings_shell_diagnostics_admin_bind_to_single_settings_governance_source: bool,
    /// Later M5 rows cannot invent parallel settings-governance vocabulary.
    pub later_rows_cannot_invent_parallel_settings_governance_vocabulary: bool,
    /// Configuration truth survives zoom and high contrast.
    pub configuration_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceConsumerProjection {
    /// Settings and shell consume the shared settings-governance truth.
    pub settings_and_shell_consume_shared_settings_governance_truth: bool,
    /// Diagnostics and admin consume the shared policy and lifecycle boundaries.
    pub diagnostics_and_admin_consume_shared_policy_and_lifecycle_boundaries: bool,
    /// Sync and capability services consume the shared write-intent and conflict classes.
    pub sync_and_capability_services_consume_shared_write_intent_and_conflict_classes: bool,
    /// Docs, help, and screenshots read a single settings-governance source.
    pub docs_help_and_screenshots_read_single_settings_governance_source: bool,
    /// Labs, experiments, and kill switches bind to the shared capability-lifecycle rule.
    pub labs_experiments_and_kill_switches_bind_to_shared_capability_lifecycle_rule: bool,
    /// Support / export reads a single canonical settings-governance source.
    pub support_export_reads_single_settings_governance_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the settings-governance lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting settings-governance audit for the lane.
    pub settings_governance_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SettingsGovernanceMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SettingsGovernanceMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Settings-governance rows.
    pub settings_governance_rows: Vec<M5SettingsGovernanceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingsGovernanceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingsGovernanceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingsGovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingsGovernanceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingsGovernanceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 settings-governance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SettingsGovernanceMatrixPacket {
    /// Record kind; must equal [`M5_SETTINGS_GOVERNANCE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Settings-governance rows.
    pub settings_governance_rows: Vec<M5SettingsGovernanceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SettingsGovernanceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SettingsGovernanceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SettingsGovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SettingsGovernanceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SettingsGovernanceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SettingsGovernanceMatrixPacket {
    /// Builds an M5 settings-governance matrix packet from stable-lane input.
    pub fn new(input: M5SettingsGovernanceMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SETTINGS_GOVERNANCE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            settings_governance_rows: input.settings_governance_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 settings-governance matrix invariants.
    pub fn validate(&self) -> Vec<M5SettingsGovernanceMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_SETTINGS_GOVERNANCE_MATRIX_RECORD_KIND {
            violations.push(M5SettingsGovernanceMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_VERSION {
            violations.push(M5SettingsGovernanceMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SettingsGovernanceMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_settings_governance_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 settings-governance matrix serializes"),
        ) {
            violations.push(M5SettingsGovernanceMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 settings-governance matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "settings_governance_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.settings_governance_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.settings_governance_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.settings_governance_family.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .settings_governance_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Setting-Definition, Write-Intent, Sync-Conflict, and Capability-Lifecycle Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Settings-governance families: {} ({} stable)\n",
            self.settings_governance_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Settings-governance roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Resolve-setting roles: {}\n",
            self.vocabulary_set.resolve_setting_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Settings-governance families\n\n");
        for row in &self.settings_governance_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.settings_governance_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.settings_governance_family.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 settings-governance matrix export.
#[derive(Debug)]
pub enum M5SettingsGovernanceMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SettingsGovernanceMatrixViolation>),
}

impl fmt::Display for M5SettingsGovernanceMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 settings-governance matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 settings-governance matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SettingsGovernanceMatrixArtifactError {}

/// Validation failures emitted by [`M5SettingsGovernanceMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SettingsGovernanceMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed settings-governance family is missing from the matrix.
    RequiredFamilyMissing,
    /// A settings-governance row is incomplete.
    SettingsGovernanceRowIncomplete,
    /// A settings-governance row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A settings-governance row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no settings-governance roles.
    SemanticRoleMissing,
    /// The resolve-setting family declares no resolve-setting roles.
    ResolveSettingRoleMissing,
    /// The write-setting family declares no write-setting roles.
    WriteSettingRoleMissing,
    /// The sync-scope family declares no sync-scope roles.
    SyncScopeRoleMissing,
    /// The migrate-schema family declares no migrate-schema roles.
    MigrateSchemaRoleMissing,
    /// The rollout-capability family declares no rollout-capability roles.
    RolloutCapabilityRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no configuration contexts.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (recycling a retired setting ID, rewriting a scoped write into a
    /// broader scope, silently overwriting locked or machine-only state during sync, hiding a lifecycle or
    /// experiment dependency behind unpublished markers, or hiding a kill-switch or policy-disable cause
    /// behind generic unavailable copy).
    SettingsGovernanceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SettingsGovernanceMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::SettingsGovernanceRowIncomplete => "settings_governance_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ResolveSettingRoleMissing => "resolve_setting_role_missing",
            Self::WriteSettingRoleMissing => "write_setting_role_missing",
            Self::SyncScopeRoleMissing => "sync_scope_role_missing",
            Self::MigrateSchemaRoleMissing => "migrate_schema_role_missing",
            Self::RolloutCapabilityRoleMissing => "rollout_capability_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::SettingsGovernanceInvariantViolated => "settings_governance_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 settings-governance matrix export.
pub fn current_stable_m5_settings_governance_matrix_export(
) -> Result<M5SettingsGovernanceMatrixPacket, M5SettingsGovernanceMatrixArtifactError> {
    let packet: M5SettingsGovernanceMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-settings-governance-proof/support_export.json"
    )))
    .map_err(M5SettingsGovernanceMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SettingsGovernanceMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SETTINGS_GOVERNANCE_MATRIX_SCHEMA_REF,
        M5_SETTINGS_GOVERNANCE_MATRIX_DOC_REF,
        M5_SETTING_DEFINITION_DOMAIN_SCHEMA_REF,
        M5_SETTING_WRITE_INTENT_DOMAIN_SCHEMA_REF,
        M5_SYNC_CONFLICT_PACKET_DOMAIN_SCHEMA_REF,
        M5_CAPABILITY_LIFECYCLE_DOMAIN_SCHEMA_REF,
        M5_EFFECTIVE_SETTING_SCHEMA_REF,
        M5_CAPABILITY_LIFECYCLE_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SettingsGovernanceMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SettingsGovernanceMatrixViolation::VocabularySetDrift);
    }
}

fn validate_settings_governance_rows(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    let present: BTreeSet<M5SettingsGovernanceFamily> = packet
        .settings_governance_rows
        .iter()
        .map(|row| row.settings_governance_family)
        .collect();
    for required in M5SettingsGovernanceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5SettingsGovernanceMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.settings_governance_rows {
        let family = row.settings_governance_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5SettingsGovernanceMatrixViolation::SettingsGovernanceRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5SettingsGovernanceMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5SettingsGovernanceMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_resolve_setting_roles() && row.resolve_setting_roles.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::ResolveSettingRoleMissing);
        }
        if family.declares_write_setting_roles() && row.write_setting_roles.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::WriteSettingRoleMissing);
        }
        if family.declares_sync_scope_roles() && row.sync_scope_roles.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::SyncScopeRoleMissing);
        }
        if family.declares_migrate_schema_roles() && row.migrate_schema_roles.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::MigrateSchemaRoleMissing);
        }
        if family.declares_rollout_capability_roles() && row.rollout_capability_roles.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::RolloutCapabilityRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SettingsGovernanceMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5SettingsGovernanceMatrixViolation::SettingsGovernanceInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.setting_definition_and_effective_resolution_stay_separately_inspectable,
        review.setting_ids_are_never_recycled,
        review.scoped_writes_are_never_widened_into_broader_scope,
        review.winning_scope_shadowed_values_restart_posture_and_lock_source_stay_inspectable,
        review.writes_land_only_in_chosen_artifact_and_scope_with_preview_checkpoint_rollback,
        review.sync_never_silently_overwrites_local_authoritative_state_during_outages,
        review.machine_only_state_never_masquerades_as_portable,
        review.lifecycle_and_experiment_dependencies_stay_visible_across_surfaces,
        review.kill_switch_and_disabled_by_policy_states_preserve_user_data_and_explain_themselves,
        review.every_family_declares_deployment_contexts,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_settings_governance_source,
        review.settings_shell_diagnostics_admin_bind_to_single_settings_governance_source,
        review.later_rows_cannot_invent_parallel_settings_governance_vocabulary,
        review.configuration_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5SettingsGovernanceMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.settings_and_shell_consume_shared_settings_governance_truth,
        projection.diagnostics_and_admin_consume_shared_policy_and_lifecycle_boundaries,
        projection.sync_and_capability_services_consume_shared_write_intent_and_conflict_classes,
        projection.docs_help_and_screenshots_read_single_settings_governance_source,
        projection.labs_experiments_and_kill_switches_bind_to_shared_capability_lifecycle_rule,
        projection.support_export_reads_single_settings_governance_source,
    ] {
        if !ok {
            violations.push(M5SettingsGovernanceMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SettingsGovernanceMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SettingsGovernanceMatrixPacket,
    violations: &mut Vec<M5SettingsGovernanceMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.settings_governance_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SettingsGovernanceMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses setting / scope / policy / sync / capability / lifecycle words; what is rejected is a
/// raw secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
