//! Effective-settings schema registry, precedence engine, and
//! locked-write flow.
//!
//! This crate is the canonical truth source for settings shape and
//! resolution. Surfaces (settings UI, CLI inspect, support export,
//! shell readiness consumers) read effective values, shadow chains,
//! and lock reasons through this crate; they do not invent private
//! "configuration" reads against the filesystem or environment.
//!
//! Two layers:
//!
//! - [`schema`] — stable [`SettingDefinition`](schema::SettingDefinition)
//!   rows: `setting_id`, value type, default, allowed scopes,
//!   aliases, migrations, restart posture, lifecycle, redaction, and
//!   capability dependencies. The
//!   [`SchemaRegistry`](schema::SchemaRegistry) is the catalog of
//!   record; [`SchemaRegistry::with_seed_catalog`](schema::SchemaRegistry::with_seed_catalog)
//!   ships the small seed used by protected dogfood walks.
//! - [`resolver`] — the precedence engine and the locked-write
//!   flow. Given the registry plus a stack of per-scope overlays,
//!   [`EffectiveSettingsResolver::resolve`](resolver::EffectiveSettingsResolver::resolve)
//!   returns the [`EffectiveValue`](resolver::EffectiveValue) with
//!   the shadow chain, the lock state, and any active policy
//!   ceiling. [`EffectiveSettingsResolver::attempt_write`](resolver::EffectiveSettingsResolver::attempt_write)
//!   returns a typed [`WriteIntent`](resolver::WriteIntent) /
//!   [`WriteDenialReason`](resolver::WriteDenialReason) outcome
//!   without ever silently dropping a denied write.
//!
//! The reviewer-facing landing page is
//! `docs/settings/effective_settings_contract.md`.

#![doc(html_root_url = "https://docs.rs/aureline-settings/0.0.0")]

pub mod component_state_registry_stable;
pub mod design_token_runtime_stable;
pub mod experiments;
pub mod finalize_appearance_session_theme_packages_token_overlays;
pub mod finalize_settings_definition_registry;
pub mod inspector;
pub mod keybindings;
pub mod locale_beta;
pub mod m5_effective_settings;
pub mod m5_portable_state_and_restore;
pub mod m5_sync_and_device_review;
pub mod repair_review;
pub mod resolver;
pub mod schema;
pub mod settings_ui_stable;
pub mod stabilize_profile_switch_review_and_temporary_profile_lifecycle;
pub mod stabilize_profile_sync_snapshot_backup_restore;
pub mod sync;
pub mod sync_device_registry_stable;
pub mod ui;

pub use experiments::labs_governance_beta::{
    build_default_labs_governance_beta_page, build_labs_governance_beta_page_from_records,
    project_labs_governance_beta_cli, project_labs_governance_beta_support_export,
    validate_labs_governance_beta_page, validate_labs_governance_beta_support_export,
    HostSurfaceAssignment, HostSurfaceClass, KillSwitchPathProjection, LabsGovernanceBetaBadge,
    LabsGovernanceBetaCliProjection, LabsGovernanceBetaCliRow, LabsGovernanceBetaPage,
    LabsGovernanceBetaRow, LabsGovernanceBetaSupportExport, LabsGovernanceBetaSupportExportRow,
    LabsGovernanceBetaValidationError, VisibleMarkerToken, LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
    LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF,
};
pub use experiments::{
    inspect_default_inventory, load_default_inventory, project_cli_inventory,
    project_support_export as project_experiments_support_export, ArtifactDependencyWarning,
    CapabilityDependencyMarker, CapabilityLifecycleState, DependencyEffectOnParent, DisableSource,
    ExperimentsInventory, ExperimentsInventoryCliProjection, ExperimentsInventoryError,
    ExperimentsInventoryInspectionRecord, ExperimentsInventorySupportExportProjection,
    KillSwitchSourceClass, DEFAULT_EXPERIMENTS_INVENTORY_SOURCE_REF,
    EXPERIMENTS_INVENTORY_SCHEMA_VERSION,
};
pub use finalize_appearance_session_theme_packages_token_overlays::{
    appearance_session_finalization_corpus, required_recovery_routes, AccessibilityDisclosure,
    AppearanceSessionBinding, AppearanceSessionFinalizationCertification,
    AppearanceSessionFinalizationScenario, AppearanceSessionSummaryRow, BuildError,
    CertificationClaimCeiling, CertificationInput, CertificationNarrowingReason,
    CertificationPillars, CertificationQualification, CertificationRecoveryAction,
    CertificationUpstream, EntryRouteRecord, ExtensionAppearanceDescriptorRow,
    ExtensionInheritanceState, ImportedThemeMappingReportRow, LayoutMode, LayoutModeDisclosure,
    LiveAppearanceAxisClass, LiveAppearanceChangeRow, LiveApplyClass, OverlayScopeClass,
    ProvenanceDimensionClass, ProvenancePreservationRow, RecoveryActionRole, RecoveryRouteRecord,
    RouteSurface, StableClaimClass, ThemePackageManifestRow, TokenOverlayValidationRow,
    APPEARANCE_SESSION_FINALIZATION_NOTICE, APPEARANCE_SESSION_FINALIZATION_RECORD_KIND,
    APPEARANCE_SESSION_FINALIZATION_SCHEMA_VERSION,
    APPEARANCE_SESSION_FINALIZATION_SHARED_CONTRACT_REF,
};
pub use finalize_settings_definition_registry::{
    audit_finalize_settings_definition_registry_page,
    seeded_finalize_settings_definition_registry_page,
    validate_finalize_settings_definition_registry_page,
    FinalizeSettingsDefinitionRegistryCliProjection, FinalizeSettingsDefinitionRegistryCliRow,
    FinalizeSettingsDefinitionRegistryDefect, FinalizeSettingsDefinitionRegistryError,
    FinalizeSettingsDefinitionRegistryPage, FinalizeSettingsDefinitionRegistryRow,
    FinalizeSettingsDefinitionRegistrySummary, FinalizeSettingsDefinitionRegistrySupportExport,
    InspectSurfaceClass, LifecycleDependencyMarker, OfflineEntitlementGraceRow,
    RegistryNarrowReasonClass, RegistryQualificationClass, SurfaceParityRow,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_ARTIFACT_REF,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_DEFECT_RECORD_KIND,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_DOC_REF,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_PAGE_RECORD_KIND,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_ROW_RECORD_KIND,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_SCHEMA_VERSION,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_SHARED_CONTRACT_REF,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_SUMMARY_RECORD_KIND,
    FINALIZE_SETTINGS_DEFINITION_REGISTRY_SUPPORT_EXPORT_RECORD_KIND,
};
pub use keybindings::mode_state::{
    ModeStateOrientationSettingsSummary, ModeStateSettingsInspectionRecord,
    ModeStateSettingsMacroRow, ModeStateSettingsRouteRow, MODE_STATE_SETTINGS_SCHEMA_VERSION,
};
pub use keybindings::{
    KeybindingNarrowingRecord, KeybindingSettingInspectionRecord, KeybindingSettingSourceLayer,
    KeybindingSettingSourceRecord, KeybindingSettingsConflictRecord,
    KEYBINDING_SETTINGS_SCHEMA_VERSION,
};
pub use locale_beta::project_locale_beta_settings_panel;
pub use m5_effective_settings::{
    m5_effective_settings_corpus, EffectiveSettingsClaim, EffectiveSettingsPillars,
    EffectiveSettingsQualification, HighImpactClass, LifecycleDependencyKind,
    M5EffectiveSettingRow, M5EffectiveSettingsCertification, M5EffectiveSettingsInput,
    M5EffectiveSettingsScenario, M5SettingFamily, PolicyLockState, RowTrust,
    ScopeExplicitWritePreview, ShadowReason, ShadowedCandidate, WinningValue, WriteEffect,
    M5_EFFECTIVE_SETTINGS_RECORD_KIND, M5_EFFECTIVE_SETTINGS_SCHEMA_VERSION,
    M5_EFFECTIVE_SETTINGS_SHARED_CONTRACT_REF,
};
pub use m5_portable_state_and_restore::{
    portable_state_restore_corpus, ExclusionReason, M5PortableStateRestoreCertification,
    M5PortableStateRestoreInput, MigrationLabel, MissingDependencyKind,
    MissingDependencyPlaceholder, PortabilityDisposition, PortableArtifactClass,
    PortablePackageClassRow, PortableRestoreClaim, PortableRestorePillars,
    PortableRestoreQualification, PortableStateRestoreScenario, RestoreProvenanceCard,
    M5_PORTABLE_STATE_RESTORE_RECORD_KIND, M5_PORTABLE_STATE_RESTORE_SCHEMA_VERSION,
    M5_PORTABLE_STATE_RESTORE_SHARED_CONTRACT_REF,
};
pub use m5_sync_and_device_review::{
    m5_sync_and_device_review_corpus, BundleSyncTrust, ConflictClass, ConflictDisposition,
    DeviceAction, DeviceActionRecord, DeviceClass,
    DeviceParticipationState as SyncDeviceParticipationState, DrillKind, FieldConflict,
    M5SyncAndDeviceReview, M5SyncAndDeviceReviewInput, M5SyncAndDeviceReviewScenario,
    RedactionMode, ScopeCapabilityDependency, ScopeRevisionSets, SyncDrill, SyncReviewClaim,
    SyncReviewPillars, SyncReviewQualification, SyncScopeBundle, SyncScopeFamily,
    SyncTransportState, TrustWideningClass, M5_SYNC_AND_DEVICE_REVIEW_RECORD_KIND,
    M5_SYNC_AND_DEVICE_REVIEW_SCHEMA_VERSION, M5_SYNC_AND_DEVICE_REVIEW_SHARED_CONTRACT_REF,
};
pub use repair_review::{
    build_repair_plan, project_review_sheet,
    project_support_export as project_repair_support_export, HiddenResetGuard,
    ImportedProfileFragmentRef, MigrationStepRef, RepairActionClass, RepairBlockedWriteReason,
    RepairPlanVerdict, RepairTargetScopeClass, RepairUserDecision, RepairWriteIntentRow,
    SettingsRepairPlan, SettingsRepairPlanRequest, SettingsRepairReviewSheet,
    SettingsRepairSupportExport, SETTINGS_REPAIR_PLAN_SCHEMA_VERSION,
    SETTINGS_REPAIR_PLAN_SHARED_CONTRACT_REF,
};
pub use resolver::{
    CapabilityState, EffectiveCapabilityDependency, EffectiveControlStack, EffectiveLastWritten,
    EffectiveSettingRecord, EffectiveSettingsResolver, EffectiveValue, LockReason, LockState,
    PolicyConstraint, ResolveError, ScopeOverlay, ShadowChainEntry, ShadowRelation,
    WriteAttemptOutcome, WriteDenialReason, WriteIntent,
};
pub use schema::{
    AliasDirection, CapabilityDependency, CapabilityDependencyKind, LifecycleLabel, MigrationRule,
    MigrationTransformClass, PreviewClass, RedactionClass, RestartPosture, SchemaRegistry,
    SchemaRegistryError, SensitivityClass, SettingAlias, SettingDefinition, SettingScope,
    SettingValue, SettingValueType, ValueValidationError,
};
pub use stabilize_profile_switch_review_and_temporary_profile_lifecycle::{
    profile_switch_lifecycle_corpus, validate_profile_switch_lifecycle_record,
    ApplyAuditRow as ProfileSwitchApplyAuditRow, ApplyTimingClass as ProfileSwitchApplyTimingClass,
    ArtifactExclusionClass as ProfileSwitchArtifactExclusionClass,
    ConflictSourceClass as ProfileSwitchConflictSourceClass,
    ExcludedMachineStateRow as ProfileSwitchExcludedMachineStateRow,
    ImportConflictReviewRow as ProfileSwitchImportConflictReviewRow,
    LocalAuthoritativeReason as ProfileSwitchLocalAuthoritativeReason,
    NarrowingEffectRow as ProfileSwitchNarrowingEffectRow,
    ProfileArtifactBoundaryRow as ProfileSwitchArtifactBoundaryRow,
    ProfileCardRow as ProfileSwitchCardRow, ProfileDurabilityClass as ProfileSwitchDurabilityClass,
    ProfileScopeClass as ProfileSwitchScopeClass, ProfileSourceClass as ProfileSwitchSourceClass,
    ProfileSwitchDeltaRow, ProfileSwitchLifecycleCertification, ProfileSwitchLifecyclePillars,
    ProfileSwitchLifecycleQualification, ProfileSwitchLifecycleScenario,
    ProfileSwitchLifecycleValidationError, ProfileSwitchNarrowingReason, ProfileSwitchReviewSheet,
    StableClaimClass as ProfileSwitchStableClaimClass, SurfaceClass as ProfileSwitchSurfaceClass,
    SurfaceTruthRow as ProfileSwitchSurfaceTruthRow,
    SyncFallbackRow as ProfileSwitchSyncFallbackRow,
    TemporaryProfileActionClass as ProfileSwitchTemporaryProfileActionClass,
    TemporaryProfileActionRow as ProfileSwitchTemporaryProfileActionRow, TemporaryProfileLifecycle,
    PROFILE_SWITCH_REVIEW_RECORD_KIND, PROFILE_SWITCH_REVIEW_SCHEMA_VERSION,
    PROFILE_SWITCH_REVIEW_SHARED_CONTRACT_REF,
};
pub use stabilize_profile_sync_snapshot_backup_restore::{
    profile_sync_restore_corpus, ConflictClass as ProfileSyncConflictClass,
    MergeRuleClass as ProfileSyncMergeRuleClass, MergeRuleRow as ProfileSyncMergeRuleRow,
    MergeSubjectClass as ProfileSyncMergeSubjectClass,
    NarrowingReason as ProfileSyncNarrowingReason, OffboardingRetentionSummary,
    ProfileSyncRestoreCertification, ProfileSyncRestoreInput, ProfileSyncRestorePillars,
    ProfileSyncRestoreQualification, ProfileSyncRestoreScenario, RestorePreviewRow,
    SecretBoundaryAuditRow, SnapshotClass as ProfileSyncSnapshotClass, SnapshotManifestRow,
    StableClaimClass as ProfileSyncStableClaimClass, StateClass as ProfileSyncStateClass,
    SurfaceClass as ProfileSyncSurfaceClass, SurfaceTruthRow, PROFILE_SYNC_RESTORE_SCHEMA_VERSION,
    PROFILE_SYNC_RESTORE_SHARED_CONTRACT_REF,
};
pub use sync::{
    build_review_row as build_sync_review_row, project_review_page as project_sync_review_page,
    project_support_export as project_sync_support_export, DeviceParticipationState,
    IdentityModeClass, LastWriterBreadcrumb, RollbackDecision, SyncBetaDeviceRecord,
    SyncConflictReviewBetaPage, SyncConflictReviewBetaRequest, SyncConflictReviewBetaRow,
    SyncConflictReviewBetaSupportExport, SyncStateClass, SyncStateSummary,
    SETTINGS_SYNC_BETA_SCHEMA_VERSION, SETTINGS_SYNC_BETA_SHARED_CONTRACT_REF,
};
pub use sync_device_registry_stable::{
    sync_device_registry_corpus, ConflictOutcomeClass, ConflictReviewRow, DeviceParticipationRow,
    MergeClass, ProfileDurabilityClass, ProfileRoamingSummary, SecretBoundaryRow, SettingCategory,
    SnapshotClass, SnapshotRow, SurfaceParityRow as SyncSurfaceParityRow,
    SyncDeviceRegistryCertification, SyncDeviceRegistryScenario,
    SYNC_DEVICE_REGISTRY_SCHEMA_VERSION, SYNC_DEVICE_REGISTRY_SHARED_CONTRACT_REF,
};
pub use ui::{
    inspect_setting_pane, project_inspector_pane, project_page_from_records,
    project_settings_ui_beta_page, project_support_export as project_ui_beta_support_export,
    project_write_composer, write_composer_from_preview, DefinitionSummary, DenialExplanation,
    LifecycleBadge, LockBadge, LockExplanation, PolicyLockSummary, RedactionBadge,
    RedactionSummary, RestartBadge, RestartPostureSummary, SensitivityBadge, SettingsUiBetaGroup,
    SettingsUiBetaInspectorPane, SettingsUiBetaPage, SettingsUiBetaRow,
    SettingsUiBetaSupportExport, SettingsUiBetaWriteComposer, SourceChainRow, WriteAffordance,
    SETTINGS_UI_BETA_SCHEMA_VERSION,
};
