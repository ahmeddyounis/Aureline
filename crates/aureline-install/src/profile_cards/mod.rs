//! Install-profile cards, side-by-side import sheets, and rollout rows.
//!
//! This module consumes the release install-row schema as a Rust-facing beta
//! contract. It does not install, update, import, or roll back anything; it
//! validates that the product, diagnostics, support, and fleet rows can all
//! point at one explicit install-profile truth model.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::topology::{
    ArchitectureClass, BinaryRootClass, ChannelClass, InstallModeClass, PlatformClass,
    SideBySideRelationClass, UpdaterOwnerClass,
};

/// Schema version for install-profile beta packets.
pub const INSTALL_PROFILE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`InstallProfileBetaPacket`].
pub const INSTALL_PROFILE_BETA_PACKET_RECORD_KIND: &str = "install_profile_beta_packet";

/// Stable record-kind tag for [`InstallProfileBetaSupportExport`].
pub const INSTALL_PROFILE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "install_profile_beta_support_export";

/// Durable state-root class carried by install-profile cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableStateRootClass {
    /// User-authored workspace files outside the install tree.
    UserAuthoredWorkspaceFilesOutsideInstallTree,
    /// Per-user configuration root.
    PerUserConfigurationRoot,
    /// Per-user recovery root.
    PerUserRecoveryRoot,
    /// Per-user derived cache root.
    PerUserDerivedCacheRoot,
    /// Per-user keychain or secret-store handle root.
    PerUserKeychainOrSecretStore,
    /// Per-machine administrator policy root.
    PerMachineAdminPolicyRoot,
    /// Per-machine shared data root.
    PerMachineSharedDataRoot,
    /// Durable state colocated with a portable bundle.
    PortableColocatedRoot,
    /// Mirror metadata root for offline or air-gapped installs.
    OfflineBundleMirrorMetadataRoot,
    /// Shared platform resource that is read-only across channels.
    SharedPlatformOsRootReadOnly,
}

/// Authority class for a durable state-root row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateAuthorityClass {
    /// User-authored durable truth.
    UserAuthoredDurableTruth,
    /// User-owned recovery state.
    UserOwnedRecoveryState,
    /// Administrator or control-plane artifact.
    AdminOrControlArtifact,
    /// Disposable derived cache.
    DisposableDerivedCache,
    /// Platform shared read-only state.
    PlatformSharedReadOnly,
}

/// Diagnostics visibility class for state roots and cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsVisibilityClass {
    /// Visible in product About surfaces.
    ExposedInAbout,
    /// Visible in support bundles.
    ExposedInSupportBundle,
    /// Visible only in an admin console.
    ExposedInAdminConsoleOnly,
    /// Not exposed.
    NotExposed,
}

/// Collision policy for a state-root row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionPolicyClass {
    /// Channels do not share mutable durable state.
    NoSharedDurableStateAcrossChannels,
    /// Channels may share only a read-only platform resource.
    SharedAcrossChannelsReadOnlyPlatformResource,
}

/// Uninstall or disable path class shown on install cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UninstallPathClass {
    /// User-owned uninstall path.
    UserUninstall,
    /// Administrator-owned uninstall path.
    AdminUninstall,
    /// Managed fleet deprovision path.
    ManagedDeprovision,
    /// External package-manager remove path.
    ExternalPackageManagerRemove,
    /// Portable directory removal path.
    PortableDirectoryRemoval,
    /// Disable-only path.
    DisableOnly,
}

/// Diagnostics or export action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsExportActionClass {
    /// Open install diagnostics.
    OpenDiagnostics,
    /// Export a support bundle.
    ExportSupportBundle,
    /// Copy a human-readable install summary.
    CopyInstallSummary,
    /// Open the state-root audit.
    OpenStateRootAudit,
    /// Open rollback evidence.
    OpenRollbackEvidence,
}

/// File-association registration class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FileAssociationRegistrationClass {
    /// User or admin can select this channel as a candidate handler.
    UserOrAdminSelectableCandidateHandler,
    /// Administrator policy owns the default handler.
    AdminOnlyDefaultHandler,
    /// The install does not register file associations.
    NotRegistered,
}

/// Default handler selection rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultHandlerSelectionRule {
    /// User or admin selection is required and last-writer-wins is forbidden.
    UserOrAdminSelectableNeverLastWriterWins,
    /// Administrator policy alone selects the handler.
    AdminOnly,
    /// Handler selection is not applicable.
    NotApplicable,
}

/// Protocol-handler ownership class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolHandlerOwnershipClass {
    /// Protocol schemes are suffixed per channel.
    PerChannelSuffixedScheme,
    /// A shared scheme exists but user or admin default selection owns it.
    SharedSchemeWithUserOrAdminDefault,
    /// The install does not register protocol handlers.
    NotRegistered,
}

/// Shared-scheme resolution rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedSchemeResolutionRule {
    /// User or admin selected default owns shared-scheme resolution.
    UserOrAdminSelectedDefault,
    /// Shared-scheme resolution is not applicable.
    NotApplicable,
}

/// Portable integration posture for machine-global requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableIntegrationPosture {
    /// The integration is suppressed in portable mode.
    Suppressed,
    /// The integration is labeled and requires user opt-in review.
    LabeledUserOptIn,
    /// The integration is not applicable.
    NotApplicable,
}

/// Surface that consumes install-profile truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallSurfaceClass {
    /// Launcher surface.
    Launcher,
    /// About surface.
    About,
    /// Update center surface.
    UpdateCenter,
    /// Diagnostics center surface.
    DiagnosticsCenter,
    /// Side-by-side import sheet.
    ImportSheet,
    /// Rollback panel.
    RollbackPanel,
    /// Fleet console.
    FleetConsole,
    /// Installer summary.
    InstallerSummary,
    /// Silent deployment summary.
    SilentDeploymentSummary,
    /// Support bundle projection.
    SupportBundle,
}

/// Import domain shown on a side-by-side import sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportDomain {
    /// Profile values.
    Profile,
    /// Settings values.
    Settings,
    /// Keybinding values.
    Keybindings,
    /// Snippets.
    Snippets,
    /// Recent work list.
    RecentWork,
    /// Extension state.
    Extensions,
    /// Layout state.
    Layout,
    /// Tasks and launch configurations.
    TasksAndLaunchConfigs,
    /// Credential metadata without raw secrets.
    CredentialsMetadata,
    /// Documentation and tour state.
    DocsAndTours,
    /// Workspace metadata.
    WorkspaceMetadata,
}

/// Import action for a side-by-side domain row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportDomainAction {
    /// Import selected values.
    ImportSelected,
    /// Keep source and target separate.
    KeepSeparate,
    /// Skip the domain.
    Skip,
    /// Require manual review.
    ManualReview,
    /// Block apply until the collision is resolved.
    Block,
}

/// Collision risk class for an import domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionRiskClass {
    /// No collision risk.
    None,
    /// Shared-state behavior is disclosed.
    DisclosedSharedState,
    /// A collision is disclosed and resolvable.
    DisclosedCollision,
    /// A collision blocks apply.
    BlockedCollision,
}

/// Collision class disclosed by an import sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionClass {
    /// Durable state-root collision.
    StateRootCollision,
    /// File-association collision.
    FileAssociationCollision,
    /// Protocol-handler collision.
    ProtocolHandlerCollision,
    /// Recent-items collision.
    RecentItemsCollision,
    /// Keychain or secret-store overlap.
    KeychainOrSecretStoreOverlap,
    /// Update-marker collision.
    UpdateMarkerCollision,
    /// Hidden shared-state assumption.
    HiddenSharedStateAssumption,
}

/// Resolution required for a collision disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollisionResolutionClass {
    /// Block apply.
    Block,
    /// Import as a copy.
    ImportAsCopy,
    /// Keep source and target separate.
    KeepSeparate,
    /// User or admin must select the owner.
    UserOrAdminSelectsOwner,
    /// No resolution is required.
    NotApplicable,
}

/// Checkpoint availability state for an import sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointAvailabilityState {
    /// Required checkpoint is already created.
    RequiredCreated,
    /// Required checkpoint is pending.
    RequiredPending,
    /// Checkpoint does not apply because the import is blocked.
    NotApplicableBlocked,
}

/// Rollback expectation for an import checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackExpectationClass {
    /// Exact checkpoint restore is expected.
    ExactCheckpointRestore,
    /// Compensating restore is expected.
    CompensatingRestore,
    /// Rollback is unsupported and blocked.
    UnsupportedBlocked,
    /// Portable delete restore is expected.
    PortableDeleteRestore,
}

/// Reason class for a side-by-side import sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportReasonClass {
    /// First-run migration.
    FirstRunMigration,
    /// Channel switch.
    ChannelSwitch,
    /// Side-by-side copy.
    SideBySideCopy,
    /// Rollback recovery.
    RollbackRecovery,
}

/// Rollback target class shown on install cards and rollout rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackTargetClass {
    /// Previous build on the same channel.
    PreviousBuildSameChannel,
    /// Last broad cut on the same channel.
    LastBroadCutSameChannel,
    /// Long-term support floor for the channel.
    ChannelLtsFloor,
    /// Administrator-pinned rollback target.
    AdminPinnedTarget,
    /// Rollback moves the install out of the ring.
    OutOfRing,
    /// Rollback is unsupported.
    Unsupported,
}

/// Rollout lane class for fleet-facing rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutLaneClass {
    /// Canary deployment-exposure ring.
    Canary,
    /// Pilot deployment-exposure ring.
    Pilot,
    /// Broad deployment-exposure ring.
    Broad,
    /// Stable channel population row.
    Stable,
    /// Preview channel population row.
    Preview,
    /// Beta channel population row.
    Beta,
    /// Long-term support population row.
    Lts,
}

/// Lane scope class for rollout rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneScopeClass {
    /// Deployment exposure ring.
    DeploymentExposure,
    /// Release-channel population row.
    ReleaseChannelPopulation,
    /// Long-term support population row.
    LongTermSupportPopulation,
}

/// Promotion state class for a rollout row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutPromotionStateClass {
    /// Seeded baseline state.
    Seeded,
    /// Promotion has not started.
    NotStarted,
    /// Candidate is ready.
    CandidateReady,
    /// Promotion is in progress.
    InProgress,
    /// Promotion is paused.
    Paused,
    /// Candidate is promoted.
    Promoted,
    /// Promotion is held.
    Held,
    /// Promotion is blocked.
    Blocked,
    /// Candidate was rolled back.
    RolledBack,
}

/// Rollback state class for a rollout row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRollbackStateClass {
    /// Seeded baseline state.
    Seeded,
    /// Rollback does not apply.
    NotApplicable,
    /// Rollback target is ready.
    RollbackReady,
    /// Rollback is required.
    RollbackRequired,
    /// Rollback is in progress.
    RollbackInProgress,
    /// Rollback completed.
    RolledBack,
    /// Rollback is blocked.
    Blocked,
}

/// Evidence type attached to rollout rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutEvidenceTypeClass {
    /// Release evidence packet.
    ReleaseEvidencePacket,
    /// Ring-history packet.
    RingHistoryPacket,
    /// Install-topology card.
    InstallTopologyCard,
    /// State-root map row.
    StateRootMapRow,
    /// Support bundle.
    SupportBundle,
    /// Deployment drill.
    DeploymentDrill,
    /// Silent deployment summary.
    SilentDeploymentSummary,
    /// Compatibility row.
    CompatibilityRow,
    /// Rollout decision.
    RolloutDecision,
}

/// Freshness state for rollout evidence links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessStateClass {
    /// Evidence is current.
    Current,
    /// Evidence is reserved.
    Reserved,
    /// Evidence is stale.
    Stale,
    /// Evidence is a seeded baseline.
    Seeded,
}

/// One durable state-root row on an install-profile card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableStateRootRow {
    /// Stable state-root ref.
    pub state_root_ref: String,
    /// Durable state-root class.
    pub durable_state_root_class: DurableStateRootClass,
    /// Authority class for the root.
    pub authority_class: StateAuthorityClass,
    /// Channels that own the root.
    pub owning_channel_classes: Vec<ChannelClass>,
    /// Diagnostics visibility class.
    pub diagnostics_visibility_class: DiagnosticsVisibilityClass,
    /// True when the root is shared across channels.
    pub shared_across_channels: bool,
    /// Optional reason when the root is shared.
    pub shared_reason: Option<String>,
    /// Collision policy for the root.
    pub collision_policy: CollisionPolicyClass,
    /// Path class placeholder instead of a host-specific path.
    pub path_class_placeholder: String,
}

/// Uninstall or disable path shown by an install-profile card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UninstallOrDisablePath {
    /// Path class.
    pub path_class: UninstallPathClass,
    /// Owner class.
    pub owner_class: UpdaterOwnerClass,
    /// Stable action ref.
    pub action_ref: String,
    /// True when human-readable summary output is required.
    pub human_readable_summary_required: bool,
}

/// Diagnostics or export action shown by an install-profile card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsExportAction {
    /// Action class.
    pub action_class: DiagnosticsExportActionClass,
    /// Stable action ref.
    pub action_ref: String,
    /// Surfaces where the action is available.
    pub available_in_surfaces: Vec<InstallSurfaceClass>,
}

/// File-association ownership truth for a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileAssociationOwnership {
    /// Registration class.
    pub registration_class: FileAssociationRegistrationClass,
    /// Default handler selection rule.
    pub default_handler_selection_rule: DefaultHandlerSelectionRule,
    /// Owner channel when one channel owns the selected handler.
    pub owner_channel_class: Option<ChannelClass>,
    /// Human-readable collision disclosure.
    pub collision_disclosure: String,
}

/// Protocol-handler ownership truth for a card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolHandlerOwnership {
    /// Ownership class.
    pub ownership_class: ProtocolHandlerOwnershipClass,
    /// Scheme placeholder instead of a host-specific scheme registration.
    pub scheme_placeholder: String,
    /// Shared scheme resolution rule.
    pub shared_scheme_resolution_rule: SharedSchemeResolutionRule,
}

/// Portable-mode restrictions shown by an install-profile card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableModeRestrictions {
    /// True when the install is in portable mode.
    pub active: bool,
    /// True when durable roots are colocated with the portable bundle.
    pub durable_roots_colocated: bool,
    /// File-association posture.
    pub file_associations: PortableIntegrationPosture,
    /// Protocol-handler posture.
    pub protocol_handlers: PortableIntegrationPosture,
    /// Service registration posture.
    pub services: PortableIntegrationPosture,
    /// Shell-hook posture.
    pub shell_hooks: PortableIntegrationPosture,
    /// Credential-store state posture.
    pub credential_store_state: PortableIntegrationPosture,
    /// Human-readable state-root collision disclosure.
    pub state_root_collision_disclosure: String,
}

/// Surface projection metadata for install-profile records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallSurfaceProjectionRow {
    /// Surface class.
    pub surface: InstallSurfaceClass,
    /// Required field refs rendered by this surface.
    pub required_field_refs: Vec<String>,
    /// Wording source ref.
    pub wording_source_ref: String,
    /// True when the row is exportable.
    pub exportable: bool,
}

/// Human-readable summary requirements for install rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HumanReadableSummaryRequirement {
    /// True when the install id must be copyable.
    pub copyable_install_id: bool,
    /// True when timestamps are required.
    pub timestamps: bool,
    /// True when state-root information is required.
    pub state_root_information: bool,
    /// True when a human-readable summary is required.
    pub human_summary: bool,
}

impl HumanReadableSummaryRequirement {
    /// Returns true when every human-summary field required by the UX contract is present.
    pub fn complete(&self) -> bool {
        self.copyable_install_id
            && self.timestamps
            && self.state_root_information
            && self.human_summary
    }
}

/// Install-profile card record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileCardRecord {
    /// Optional JSON Schema ref.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Schema version.
    pub schema_version: u32,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable install-profile card id.
    pub install_profile_card_id: String,
    /// Copyable install id ref.
    pub install_id_ref: String,
    /// Display label.
    pub display_label: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Architecture class.
    pub architecture_class: ArchitectureClass,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Binary root ref.
    pub binary_root_ref: String,
    /// Durable state roots.
    pub durable_state_roots: Vec<DurableStateRootRow>,
    /// Side-by-side relation class.
    pub side_by_side_relation_class: SideBySideRelationClass,
    /// Rollback target class.
    pub rollback_target_class: RollbackTargetClass,
    /// Optional rollback target ref.
    pub rollback_target_ref: Option<String>,
    /// Uninstall or disable path.
    pub uninstall_or_disable_path: UninstallOrDisablePath,
    /// Diagnostics and export actions.
    pub diagnostics_export_actions: Vec<DiagnosticsExportAction>,
    /// File-association ownership truth.
    pub file_association_ownership: FileAssociationOwnership,
    /// Protocol-handler ownership truth.
    pub protocol_handler_ownership: ProtocolHandlerOwnership,
    /// Portable-mode restrictions.
    pub portable_mode: PortableModeRestrictions,
    /// Surface projections.
    pub surface_projection: Vec<InstallSurfaceProjectionRow>,
    /// Human-readable summary requirement.
    pub human_readable_summary_requirement: HumanReadableSummaryRequirement,
    /// Source refs consumed by the card.
    pub source_refs: Vec<String>,
    /// Reviewer-facing notes.
    pub notes: String,
}

impl InstallProfileCardRecord {
    /// Returns true when this card describes a portable install.
    pub fn is_portable(&self) -> bool {
        self.install_mode_class == InstallModeClass::Portable
    }

    /// Returns the durable state-root refs carried by the card.
    pub fn state_root_refs(&self) -> Vec<String> {
        self.durable_state_roots
            .iter()
            .map(|root| root.state_root_ref.clone())
            .collect()
    }
}

/// Compare semantics for a side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareSemantics {
    /// Domains included in comparison.
    pub comparison_scope: Vec<ImportDomain>,
    /// Diff materialization ref.
    pub diff_materialization_ref: String,
    /// Collision scan ref.
    pub collision_scan_ref: String,
    /// True when the user can compare before apply.
    pub can_compare_before_apply: bool,
}

/// One domain row on a side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportDomainRow {
    /// Import domain.
    pub domain: ImportDomain,
    /// Domain action.
    pub action: ImportDomainAction,
    /// Source state-root ref.
    pub source_state_root_ref: String,
    /// Target state-root ref.
    pub target_state_root_ref: String,
    /// Collision risk class.
    pub collision_risk_class: CollisionRiskClass,
    /// Expected outcome.
    pub expected_outcome: String,
}

/// Skip semantics for a side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkipSemantics {
    /// True when skip preserves the source.
    pub skip_preserves_source: bool,
    /// True when skip writes target state.
    pub skip_writes_target: bool,
    /// Human-readable skip summary.
    pub skip_summary: String,
}

/// Checkpoint expectation for a side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointExpectation {
    /// Checkpoint availability state.
    pub availability_state: CheckpointAvailabilityState,
    /// Checkpoint ref, when available.
    pub checkpoint_ref: Option<String>,
    /// True when checkpoint is created before apply.
    pub created_before_apply: bool,
    /// Rollback expectation class.
    pub rollback_expectation_class: RollbackExpectationClass,
    /// Human-readable rollback summary.
    pub rollback_summary: String,
}

/// Collision disclosure for a side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedStateCollisionDisclosure {
    /// Collision class.
    pub collision_class: CollisionClass,
    /// Source ref.
    pub source_ref: String,
    /// Target ref.
    pub target_ref: String,
    /// True when disclosure is required.
    pub disclosure_required: bool,
    /// Resolution required before apply.
    pub resolution_required: CollisionResolutionClass,
    /// Optional notes.
    pub notes: Option<String>,
}

/// Side-by-side import sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideBySideImportSheetRecord {
    /// Optional JSON Schema ref.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Schema version.
    pub schema_version: u32,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable import sheet id.
    pub import_sheet_id: String,
    /// Source install ref.
    pub source_install_ref: String,
    /// Target install ref.
    pub target_install_ref: String,
    /// Source channel class.
    pub source_channel_class: ChannelClass,
    /// Target channel class.
    pub target_channel_class: ChannelClass,
    /// Import reason class.
    pub import_reason_class: ImportReasonClass,
    /// Compare semantics.
    pub compare_semantics: CompareSemantics,
    /// Domain rows.
    pub domain_rows: Vec<ImportDomainRow>,
    /// Skip semantics.
    pub skip_semantics: SkipSemantics,
    /// Checkpoint expectation.
    pub checkpoint: CheckpointExpectation,
    /// Shared-state collision disclosures.
    pub shared_state_collision_disclosures: Vec<SharedStateCollisionDisclosure>,
    /// Surface projections.
    pub surface_projection: Vec<InstallSurfaceProjectionRow>,
    /// Human-readable summary requirement.
    pub human_readable_summary_requirement: HumanReadableSummaryRequirement,
    /// Support refs.
    pub support_refs: Vec<String>,
    /// Reviewer-facing notes.
    pub notes: String,
}

/// Promotion state for a rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionState {
    /// Promotion state.
    pub state: RolloutPromotionStateClass,
    /// Candidate build ref.
    pub candidate_build_ref: String,
    /// Current ring ref.
    pub current_ring_ref: String,
    /// Promotion decision ref.
    pub promotion_decision_ref: String,
    /// Lanes this row was promoted from.
    pub promoted_from_lane_refs: Vec<String>,
    /// Lanes this row may promote to.
    pub promoted_to_lane_refs: Vec<String>,
    /// Evidence required before promotion.
    pub required_evidence_refs: Vec<String>,
    /// Evidence preserved with the row.
    pub preserved_evidence_refs: Vec<String>,
    /// UTC update timestamp.
    pub updated_at: String,
}

/// Rollback state for a rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutRollbackState {
    /// Rollback state.
    pub state: RolloutRollbackStateClass,
    /// Rollback target class.
    pub rollback_target_class: RollbackTargetClass,
    /// Rollback target ref.
    pub rollback_target_ref: String,
    /// Stop conditions that bound rollback.
    pub rollback_stop_condition_refs: Vec<String>,
    /// Last rollback ref, when present.
    pub last_rollback_ref: Option<String>,
    /// Evidence preserved with rollback.
    pub preserved_evidence_refs: Vec<String>,
    /// Human-readable rollback summary.
    pub summary: String,
}

/// Preserved evidence link on a rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutEvidenceLink {
    /// Stable evidence link ref.
    pub link_ref: String,
    /// Evidence type.
    pub evidence_type: RolloutEvidenceTypeClass,
    /// Freshness state.
    pub freshness_state: EvidenceFreshnessStateClass,
    /// True when evidence is preserved.
    pub preserved: bool,
    /// Optional notes.
    pub notes: Option<String>,
}

/// Rollout ring row record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutRingRowRecord {
    /// Optional JSON Schema ref.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,
    /// Schema version.
    pub schema_version: u32,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable rollout lane id.
    pub rollout_lane_id: String,
    /// Display name.
    pub display_name: String,
    /// Lane class.
    pub lane_class: RolloutLaneClass,
    /// Lane scope class.
    pub lane_scope_class: LaneScopeClass,
    /// Channels admitted by this lane.
    pub admitted_channel_classes: Vec<ChannelClass>,
    /// Accountable owner.
    pub owner: String,
    /// Promotion state.
    pub promotion_state: PromotionState,
    /// Rollback state.
    pub rollback_state: RolloutRollbackState,
    /// Preserved evidence links.
    pub preserved_evidence_links: Vec<RolloutEvidenceLink>,
    /// Install-profile cards this lane covers.
    pub install_profile_card_refs: Vec<String>,
    /// Surface projections.
    pub surface_projection: Vec<InstallSurfaceProjectionRow>,
    /// Human-readable summary requirement.
    pub human_readable_summary_requirement: HumanReadableSummaryRequirement,
    /// Reviewer-facing notes.
    pub notes: String,
}

/// Upstream source refs consumed by an install-profile beta packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileBetaSourceRefs {
    /// Install-row schema ref.
    pub install_row_schema_ref: String,
    /// Install-topology matrix ref.
    pub install_topology_matrix_ref: String,
    /// Install diagnostics packet ref.
    pub install_diagnostics_packet_ref: String,
    /// State-root map ref.
    pub state_root_map_ref: String,
    /// Ring rollout packet ref.
    pub ring_rollout_packet_ref: String,
    /// Silent deployment results ref.
    pub silent_deployment_results_ref: String,
}

/// Beta packet containing profile cards, import sheets, and rollout rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileBetaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Source refs consumed by the packet.
    pub source_refs: InstallProfileBetaSourceRefs,
    /// Install-profile cards.
    pub install_profile_cards: Vec<InstallProfileCardRecord>,
    /// Side-by-side import sheets.
    pub side_by_side_import_sheets: Vec<SideBySideImportSheetRecord>,
    /// Rollout ring rows.
    pub rollout_ring_rows: Vec<RolloutRingRowRecord>,
}

impl InstallProfileBetaPacket {
    /// Validates profile-card, import-sheet, portable, side-by-side, and rollout truth.
    pub fn validate(&self) -> InstallProfileBetaValidationReport {
        let mut validator = InstallProfileBetaValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Finds an install-profile card by id.
    pub fn card_by_id(&self, install_profile_card_id: &str) -> Option<&InstallProfileCardRecord> {
        self.install_profile_cards
            .iter()
            .find(|card| card.install_profile_card_id == install_profile_card_id)
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> InstallProfileBetaSupportExport {
        InstallProfileBetaSupportExport {
            record_kind: INSTALL_PROFILE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: INSTALL_PROFILE_BETA_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            source_packet_ref:
                "fixtures/install/m3/profile_cards_and_repair/profile_cards_packet.json".to_string(),
            cards: self
                .install_profile_cards
                .iter()
                .map(InstallProfileCardSupportRow::from)
                .collect(),
            import_sheets: self
                .side_by_side_import_sheets
                .iter()
                .map(ImportSheetSupportRow::from)
                .collect(),
            rollout_rows: self
                .rollout_ring_rows
                .iter()
                .map(RolloutRingSupportRow::from)
                .collect(),
            redaction_class: "metadata_only_no_paths_or_secrets".to_string(),
        }
    }
}

/// Support-export projection for install-profile beta packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileBetaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source packet id.
    pub packet_id: String,
    /// Source packet ref.
    pub source_packet_ref: String,
    /// Install-profile card rows.
    pub cards: Vec<InstallProfileCardSupportRow>,
    /// Side-by-side import sheet rows.
    pub import_sheets: Vec<ImportSheetSupportRow>,
    /// Rollout ring rows.
    pub rollout_rows: Vec<RolloutRingSupportRow>,
    /// Redaction class.
    pub redaction_class: String,
}

/// Support-export row for an install-profile card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileCardSupportRow {
    /// Install-profile card id.
    pub install_profile_card_id: String,
    /// Copyable install id ref.
    pub install_id_ref: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Architecture class.
    pub architecture_class: ArchitectureClass,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Durable state-root refs.
    pub durable_state_root_refs: Vec<String>,
    /// Side-by-side relation class.
    pub side_by_side_relation_class: SideBySideRelationClass,
    /// Rollback target class.
    pub rollback_target_class: RollbackTargetClass,
    /// Uninstall action ref.
    pub uninstall_action_ref: String,
    /// Portable mode is active.
    pub portable_mode_active: bool,
}

impl From<&InstallProfileCardRecord> for InstallProfileCardSupportRow {
    fn from(card: &InstallProfileCardRecord) -> Self {
        Self {
            install_profile_card_id: card.install_profile_card_id.clone(),
            install_id_ref: card.install_id_ref.clone(),
            platform_class: card.platform_class,
            architecture_class: card.architecture_class,
            install_mode_class: card.install_mode_class,
            channel_class: card.channel_class,
            updater_owner_class: card.updater_owner_class,
            binary_root_class: card.binary_root_class,
            durable_state_root_refs: card.state_root_refs(),
            side_by_side_relation_class: card.side_by_side_relation_class,
            rollback_target_class: card.rollback_target_class,
            uninstall_action_ref: card.uninstall_or_disable_path.action_ref.clone(),
            portable_mode_active: card.portable_mode.active,
        }
    }
}

/// Support-export row for a side-by-side import sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportSheetSupportRow {
    /// Import sheet id.
    pub import_sheet_id: String,
    /// Source install ref.
    pub source_install_ref: String,
    /// Target install ref.
    pub target_install_ref: String,
    /// Source channel class.
    pub source_channel_class: ChannelClass,
    /// Target channel class.
    pub target_channel_class: ChannelClass,
    /// True when compare-before-apply is available.
    pub can_compare_before_apply: bool,
    /// Checkpoint ref.
    pub checkpoint_ref: Option<String>,
    /// Domain actions keyed by import domain.
    pub domain_actions: BTreeMap<ImportDomain, ImportDomainAction>,
}

impl From<&SideBySideImportSheetRecord> for ImportSheetSupportRow {
    fn from(sheet: &SideBySideImportSheetRecord) -> Self {
        Self {
            import_sheet_id: sheet.import_sheet_id.clone(),
            source_install_ref: sheet.source_install_ref.clone(),
            target_install_ref: sheet.target_install_ref.clone(),
            source_channel_class: sheet.source_channel_class,
            target_channel_class: sheet.target_channel_class,
            can_compare_before_apply: sheet.compare_semantics.can_compare_before_apply,
            checkpoint_ref: sheet.checkpoint.checkpoint_ref.clone(),
            domain_actions: sheet
                .domain_rows
                .iter()
                .map(|row| (row.domain, row.action))
                .collect(),
        }
    }
}

/// Support-export row for a rollout ring row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RolloutRingSupportRow {
    /// Rollout lane id.
    pub rollout_lane_id: String,
    /// Lane class.
    pub lane_class: RolloutLaneClass,
    /// Accountable owner.
    pub owner: String,
    /// Promotion state.
    pub promotion_state: RolloutPromotionStateClass,
    /// Rollback state.
    pub rollback_state: RolloutRollbackStateClass,
    /// Rollback target class.
    pub rollback_target_class: RollbackTargetClass,
    /// Install-profile cards covered by the lane.
    pub install_profile_card_refs: Vec<String>,
}

impl From<&RolloutRingRowRecord> for RolloutRingSupportRow {
    fn from(row: &RolloutRingRowRecord) -> Self {
        Self {
            rollout_lane_id: row.rollout_lane_id.clone(),
            lane_class: row.lane_class,
            owner: row.owner.clone(),
            promotion_state: row.promotion_state.state,
            rollback_state: row.rollback_state.state,
            rollback_target_class: row.rollback_state.rollback_target_class,
            install_profile_card_refs: row.install_profile_card_refs.clone(),
        }
    }
}

/// Validation coverage for an install-profile beta packet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileBetaCoverage {
    /// Install modes covered by cards.
    pub install_modes: BTreeSet<InstallModeClass>,
    /// Channels covered by cards.
    pub channels: BTreeSet<ChannelClass>,
    /// Surfaces covered by rows.
    pub surfaces: BTreeSet<InstallSurfaceClass>,
    /// Rollout lane classes covered by rows.
    pub rollout_lanes: BTreeSet<RolloutLaneClass>,
    /// Import actions covered by sheets.
    pub import_actions: BTreeSet<ImportDomainAction>,
}

/// One install-profile beta validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileBetaValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable message.
    pub message: String,
    /// Row or packet ref associated with the finding.
    pub ref_id: String,
}

/// Validation report for an install-profile beta packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallProfileBetaValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Coverage collected during validation.
    pub coverage: InstallProfileBetaCoverage,
    /// Validation findings.
    pub findings: Vec<InstallProfileBetaValidationFinding>,
}

struct InstallProfileBetaValidator<'a> {
    packet: &'a InstallProfileBetaPacket,
    coverage: InstallProfileBetaCoverage,
    findings: Vec<InstallProfileBetaValidationFinding>,
    seen_card_ids: BTreeSet<String>,
    seen_install_ids: BTreeSet<String>,
}

impl<'a> InstallProfileBetaValidator<'a> {
    fn new(packet: &'a InstallProfileBetaPacket) -> Self {
        Self {
            packet,
            coverage: InstallProfileBetaCoverage::default(),
            findings: Vec::new(),
            seen_card_ids: BTreeSet::new(),
            seen_install_ids: BTreeSet::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        for card in &self.packet.install_profile_cards {
            self.validate_card(card);
        }
        for sheet in &self.packet.side_by_side_import_sheets {
            self.validate_import_sheet(sheet);
        }
        for row in &self.packet.rollout_ring_rows {
            self.validate_rollout_row(row);
        }
        self.validate_required_coverage();
    }

    fn finish(self) -> InstallProfileBetaValidationReport {
        InstallProfileBetaValidationReport {
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn push(&mut self, check_id: &str, message: impl Into<String>, ref_id: impl Into<String>) {
        self.findings.push(InstallProfileBetaValidationFinding {
            check_id: check_id.to_string(),
            message: message.into(),
            ref_id: ref_id.into(),
        });
    }

    fn validate_header(&mut self) {
        if self.packet.record_kind != INSTALL_PROFILE_BETA_PACKET_RECORD_KIND {
            self.push(
                "install_profile.packet.record_kind",
                "packet record_kind must be install_profile_beta_packet",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.schema_version != INSTALL_PROFILE_BETA_SCHEMA_VERSION {
            self.push(
                "install_profile.packet.schema_version",
                "packet schema_version must be 1",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.install_profile_cards.is_empty() {
            self.push(
                "install_profile.packet.cards_empty",
                "packet must contain install-profile cards",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.side_by_side_import_sheets.is_empty() {
            self.push(
                "install_profile.packet.import_sheets_empty",
                "packet must contain at least one side-by-side import sheet",
                self.packet.packet_id.clone(),
            );
        }
        if self.packet.rollout_ring_rows.is_empty() {
            self.push(
                "install_profile.packet.rollout_rows_empty",
                "packet must contain rollout ring rows",
                self.packet.packet_id.clone(),
            );
        }
    }

    fn validate_card(&mut self, card: &InstallProfileCardRecord) {
        if card.record_kind != "install_profile_card_record" {
            self.push(
                "install_profile.card.record_kind",
                "card record_kind must be install_profile_card_record",
                card.install_profile_card_id.clone(),
            );
        }
        if card.schema_version != INSTALL_PROFILE_BETA_SCHEMA_VERSION {
            self.push(
                "install_profile.card.schema_version",
                "card schema_version must be 1",
                card.install_profile_card_id.clone(),
            );
        }
        if !self
            .seen_card_ids
            .insert(card.install_profile_card_id.clone())
        {
            self.push(
                "install_profile.card.duplicate",
                "install-profile card ids must be unique",
                card.install_profile_card_id.clone(),
            );
        }
        if !self.seen_install_ids.insert(card.install_id_ref.clone()) {
            self.push(
                "install_profile.card.install_id_duplicate",
                "copyable install ids must be unique",
                card.install_profile_card_id.clone(),
            );
        }
        self.coverage.install_modes.insert(card.install_mode_class);
        self.coverage.channels.insert(card.channel_class);
        for surface in &card.surface_projection {
            self.coverage.surfaces.insert(surface.surface);
        }

        if card.binary_root_ref.trim().is_empty() {
            self.push(
                "install_profile.card.binary_root_ref_missing",
                "card must carry a binary root ref",
                card.install_profile_card_id.clone(),
            );
        }
        if card.durable_state_roots.is_empty() {
            self.push(
                "install_profile.card.state_roots_missing",
                "card must disclose durable state roots",
                card.install_profile_card_id.clone(),
            );
        }
        if !card.human_readable_summary_requirement.complete()
            || !card
                .uninstall_or_disable_path
                .human_readable_summary_required
        {
            self.push(
                "install_profile.card.human_summary_incomplete",
                "cards must require copyable install ids, timestamps, state-root info, and human summary",
                card.install_profile_card_id.clone(),
            );
        }
        self.validate_card_surfaces(card);
        self.validate_card_actions(card);
        self.validate_card_state_roots(card);
        self.validate_portable_card(card);
        self.validate_side_by_side_card(card);
    }

    fn validate_card_surfaces(&mut self, card: &InstallProfileCardRecord) {
        let surfaces: BTreeSet<_> = card
            .surface_projection
            .iter()
            .map(|row| row.surface)
            .collect();
        for required in [
            InstallSurfaceClass::About,
            InstallSurfaceClass::UpdateCenter,
            InstallSurfaceClass::DiagnosticsCenter,
            InstallSurfaceClass::SupportBundle,
        ] {
            if !surfaces.contains(&required) {
                self.push(
                    "install_profile.card.surface_missing",
                    format!("card must project to {required:?}"),
                    card.install_profile_card_id.clone(),
                );
            }
        }
        for row in &card.surface_projection {
            if row.required_field_refs.is_empty() || row.wording_source_ref.trim().is_empty() {
                self.push(
                    "install_profile.card.surface_projection_incomplete",
                    "surface projection must name required fields and wording source",
                    card.install_profile_card_id.clone(),
                );
            }
            if !row.exportable {
                self.push(
                    "install_profile.card.surface_not_exportable",
                    "install-profile surface rows must be exportable",
                    card.install_profile_card_id.clone(),
                );
            }
        }
    }

    fn validate_card_actions(&mut self, card: &InstallProfileCardRecord) {
        let action_classes: BTreeSet<_> = card
            .diagnostics_export_actions
            .iter()
            .map(|action| action.action_class)
            .collect();
        for required in [
            DiagnosticsExportActionClass::OpenDiagnostics,
            DiagnosticsExportActionClass::CopyInstallSummary,
        ] {
            if !action_classes.contains(&required) {
                self.push(
                    "install_profile.card.diagnostics_action_missing",
                    format!("card must expose {required:?}"),
                    card.install_profile_card_id.clone(),
                );
            }
        }
    }

    fn validate_card_state_roots(&mut self, card: &InstallProfileCardRecord) {
        let mut seen = BTreeSet::new();
        for root in &card.durable_state_roots {
            if root.state_root_ref.trim().is_empty()
                || root.path_class_placeholder.trim().is_empty()
            {
                self.push(
                    "install_profile.card.state_root_ref_missing",
                    "state-root rows must carry stable refs and path placeholders",
                    card.install_profile_card_id.clone(),
                );
            }
            if !seen.insert(root.state_root_ref.clone()) {
                self.push(
                    "install_profile.card.state_root_duplicate",
                    "state-root refs must be unique within a card",
                    root.state_root_ref.clone(),
                );
            }
            if root.shared_across_channels
                && root.collision_policy
                    != CollisionPolicyClass::SharedAcrossChannelsReadOnlyPlatformResource
            {
                self.push(
                    "install_profile.card.shared_mutable_state",
                    "shared state roots must be read-only platform resources",
                    root.state_root_ref.clone(),
                );
            }
        }
    }

    fn validate_portable_card(&mut self, card: &InstallProfileCardRecord) {
        if !card.is_portable() {
            if card.portable_mode.active {
                self.push(
                    "install_profile.card.portable_active_on_nonportable",
                    "non-portable cards must not mark portable mode active",
                    card.install_profile_card_id.clone(),
                );
            }
            return;
        }
        if !card.portable_mode.active || !card.portable_mode.durable_roots_colocated {
            self.push(
                "install_profile.portable.not_colocated",
                "portable cards must mark active portable mode with colocated durable roots",
                card.install_profile_card_id.clone(),
            );
        }
        for posture in [
            card.portable_mode.file_associations,
            card.portable_mode.protocol_handlers,
            card.portable_mode.services,
            card.portable_mode.shell_hooks,
            card.portable_mode.credential_store_state,
        ] {
            if posture != PortableIntegrationPosture::Suppressed {
                self.push(
                    "install_profile.portable.global_integration_not_suppressed",
                    "portable cards must suppress machine-global integrations in the beta contract",
                    card.install_profile_card_id.clone(),
                );
            }
        }
        if card.file_association_ownership.registration_class
            != FileAssociationRegistrationClass::NotRegistered
            || card.protocol_handler_ownership.ownership_class
                != ProtocolHandlerOwnershipClass::NotRegistered
        {
            self.push(
                "install_profile.portable.handler_registered",
                "portable cards must not register file or protocol handlers",
                card.install_profile_card_id.clone(),
            );
        }
        for root in &card.durable_state_roots {
            if root.durable_state_root_class != DurableStateRootClass::PortableColocatedRoot
                || !root.state_root_ref.contains("portable_colocated_root")
            {
                self.push(
                    "install_profile.portable.root_not_portable",
                    "portable cards must disclose only portable-colocated durable roots",
                    root.state_root_ref.clone(),
                );
            }
        }
    }

    fn validate_side_by_side_card(&mut self, card: &InstallProfileCardRecord) {
        if !matches!(
            card.side_by_side_relation_class,
            SideBySideRelationClass::StableAndPreview
                | SideBySideRelationClass::StableAndBeta
                | SideBySideRelationClass::StableAndLts
                | SideBySideRelationClass::PreviewAndBeta
                | SideBySideRelationClass::InstalledAndPortable
                | SideBySideRelationClass::ThreeChannelMatrix
                | SideBySideRelationClass::ManagedAndPortable
        ) {
            return;
        }
        if card
            .durable_state_roots
            .iter()
            .any(|root| root.shared_across_channels)
        {
            self.push(
                "install_profile.side_by_side.shared_mutable_root",
                "side-by-side cards must not share mutable durable roots",
                card.install_profile_card_id.clone(),
            );
        }
        if card
            .file_association_ownership
            .default_handler_selection_rule
            != DefaultHandlerSelectionRule::UserOrAdminSelectableNeverLastWriterWins
            && !card.is_portable()
        {
            self.push(
                "install_profile.side_by_side.last_writer_wins_risk",
                "side-by-side handler ownership must use explicit user/admin selection",
                card.install_profile_card_id.clone(),
            );
        }
    }

    fn validate_import_sheet(&mut self, sheet: &SideBySideImportSheetRecord) {
        if sheet.record_kind != "side_by_side_import_sheet_record" {
            self.push(
                "install_profile.import.record_kind",
                "import sheet record_kind must be side_by_side_import_sheet_record",
                sheet.import_sheet_id.clone(),
            );
        }
        if sheet.source_channel_class == sheet.target_channel_class {
            self.push(
                "install_profile.import.same_channel",
                "side-by-side import sheets must cross channels",
                sheet.import_sheet_id.clone(),
            );
        }
        if self.packet.card_by_id(&sheet.source_install_ref).is_none()
            || self.packet.card_by_id(&sheet.target_install_ref).is_none()
        {
            self.push(
                "install_profile.import.card_ref_missing",
                "source and target install refs must resolve to packet cards",
                sheet.import_sheet_id.clone(),
            );
        }
        if !sheet.compare_semantics.can_compare_before_apply
            || sheet.compare_semantics.comparison_scope.is_empty()
        {
            self.push(
                "install_profile.import.compare_missing",
                "side-by-side import must offer compare before apply",
                sheet.import_sheet_id.clone(),
            );
        }
        if !sheet.skip_semantics.skip_preserves_source || sheet.skip_semantics.skip_writes_target {
            self.push(
                "install_profile.import.skip_semantics_unsafe",
                "skip must preserve source and write no target state",
                sheet.import_sheet_id.clone(),
            );
        }
        if sheet.checkpoint.availability_state != CheckpointAvailabilityState::RequiredCreated
            || !sheet.checkpoint.created_before_apply
            || sheet.checkpoint.checkpoint_ref.is_none()
        {
            self.push(
                "install_profile.import.checkpoint_missing",
                "side-by-side import must create a rollback checkpoint before apply",
                sheet.import_sheet_id.clone(),
            );
        }
        for row in &sheet.domain_rows {
            self.coverage.import_actions.insert(row.action);
            if row.collision_risk_class == CollisionRiskClass::BlockedCollision
                && row.action != ImportDomainAction::Block
            {
                self.push(
                    "install_profile.import.blocked_collision_not_blocked",
                    "blocked collisions must use the block action",
                    sheet.import_sheet_id.clone(),
                );
            }
        }
        let disclosures: BTreeSet<_> = sheet
            .shared_state_collision_disclosures
            .iter()
            .map(|disclosure| disclosure.collision_class)
            .collect();
        for required in [
            CollisionClass::StateRootCollision,
            CollisionClass::FileAssociationCollision,
            CollisionClass::HiddenSharedStateAssumption,
        ] {
            if !disclosures.contains(&required) {
                self.push(
                    "install_profile.import.disclosure_missing",
                    format!("import sheet must disclose {required:?}"),
                    sheet.import_sheet_id.clone(),
                );
            }
        }
        if !sheet.human_readable_summary_requirement.complete() {
            self.push(
                "install_profile.import.human_summary_incomplete",
                "import sheet must require copyable ids, timestamps, state roots, and summary",
                sheet.import_sheet_id.clone(),
            );
        }
    }

    fn validate_rollout_row(&mut self, row: &RolloutRingRowRecord) {
        if row.record_kind != "rollout_ring_row_record" {
            self.push(
                "install_profile.rollout.record_kind",
                "rollout row record_kind must be rollout_ring_row_record",
                row.rollout_lane_id.clone(),
            );
        }
        self.coverage.rollout_lanes.insert(row.lane_class);
        for surface in &row.surface_projection {
            self.coverage.surfaces.insert(surface.surface);
        }
        if row.owner.trim().is_empty() {
            self.push(
                "install_profile.rollout.owner_missing",
                "rollout rows must name an owner",
                row.rollout_lane_id.clone(),
            );
        }
        if row.promotion_state.required_evidence_refs.is_empty()
            || row.promotion_state.preserved_evidence_refs.is_empty()
        {
            self.push(
                "install_profile.rollout.evidence_missing",
                "rollout rows must carry required and preserved evidence refs",
                row.rollout_lane_id.clone(),
            );
        }
        if matches!(
            row.rollback_state.state,
            RolloutRollbackStateClass::NotApplicable | RolloutRollbackStateClass::Blocked
        ) || row.rollback_state.rollback_target_class == RollbackTargetClass::Unsupported
            || row.rollback_state.rollback_stop_condition_refs.is_empty()
            || row.rollback_state.summary.trim().is_empty()
        {
            self.push(
                "install_profile.rollout.rollback_unbounded",
                "rollout rows must carry bounded rollback posture",
                row.rollout_lane_id.clone(),
            );
        }
        for card_ref in &row.install_profile_card_refs {
            if self.packet.card_by_id(card_ref).is_none() {
                self.push(
                    "install_profile.rollout.card_ref_missing",
                    "rollout row card refs must resolve to packet cards",
                    format!("{} -> {card_ref}", row.rollout_lane_id),
                );
            }
        }
        if !row.human_readable_summary_requirement.complete() {
            self.push(
                "install_profile.rollout.human_summary_incomplete",
                "rollout rows must require copyable ids, timestamps, state roots, and summary",
                row.rollout_lane_id.clone(),
            );
        }
    }

    fn validate_required_coverage(&mut self) {
        for mode in [
            InstallModeClass::PerUserInstalled,
            InstallModeClass::SideBySidePreview,
            InstallModeClass::Portable,
            InstallModeClass::ManagedDeployed,
        ] {
            if !self.coverage.install_modes.contains(&mode) {
                self.push(
                    "install_profile.coverage.install_mode_missing",
                    format!("required install mode is not covered: {mode:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for lane in [
            RolloutLaneClass::Canary,
            RolloutLaneClass::Pilot,
            RolloutLaneClass::Broad,
            RolloutLaneClass::Lts,
        ] {
            if !self.coverage.rollout_lanes.contains(&lane) {
                self.push(
                    "install_profile.coverage.rollout_lane_missing",
                    format!("required rollout lane is not covered: {lane:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for action in [
            ImportDomainAction::ImportSelected,
            ImportDomainAction::KeepSeparate,
        ] {
            if !self.coverage.import_actions.contains(&action) {
                self.push(
                    "install_profile.coverage.import_action_missing",
                    format!("required import action is not covered: {action:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
    }
}
