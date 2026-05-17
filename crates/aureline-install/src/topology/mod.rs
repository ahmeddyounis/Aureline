//! Install topology state model, validation, and surface projections.
//!
//! The packet in this module is the alpha implementation contract for install
//! review and supportability. Callers provide rows derived from the release
//! topology artifacts, then render [`InstallTopologyAlphaPacket`] through
//! [`InstallTopologyAlphaPacket::surface_projection`] or
//! [`InstallTopologyAlphaPacket::support_export_projection`].

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

mod diagnostics;

pub use diagnostics::*;

/// Schema version for install-topology alpha packets.
pub const INSTALL_TOPOLOGY_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`InstallTopologyAlphaPacket`].
pub const INSTALL_TOPOLOGY_ALPHA_PACKET_RECORD_KIND: &str = "install_topology_alpha_packet";

/// Stable record-kind tag for [`InstallTopologySupportExport`].
pub const INSTALL_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND: &str = "install_topology_support_export";

/// Platform class for an install topology row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformClass {
    /// Microsoft Windows desktop packaging target.
    Windows,
    /// Apple macOS desktop packaging target.
    Macos,
    /// Linux desktop packaging target.
    Linux,
    /// Detached air-gap bundle target rather than a live desktop OS.
    AirGapBundleTarget,
}

/// CPU or artifact architecture class for an install topology row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchitectureClass {
    /// x86-64 native artifact.
    X86_64,
    /// AArch64 native artifact.
    Aarch64,
    /// Universal binary artifact containing multiple architecture slices.
    UniversalBinary,
    /// Platform-native artifact where the exact architecture is selected by the package manager.
    PlatformNativeAny,
}

/// Install mode exposed by product and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallModeClass {
    /// User-owned install rooted under a user-writable program area.
    PerUserInstalled,
    /// Machine install controlled by an administrator or package manager.
    PerMachineInstalled,
    /// Portable or unpacked bundle with no ordinary host-global mutation.
    Portable,
    /// Offline or air-gapped bundle installation path.
    OfflineBundle,
    /// Managed fleet deployment controlled by a rollout owner.
    ManagedDeployed,
    /// Preview channel installed beside a stable channel.
    SideBySidePreview,
}

/// Release channel class exposed by install surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelClass {
    /// Stable channel.
    Stable,
    /// Preview channel.
    Preview,
    /// Beta channel.
    Beta,
    /// Long-term support channel.
    Lts,
    /// Portable stable channel.
    PortableStable,
    /// Portable preview channel.
    PortablePreview,
}

/// Owner of update decisions for an install row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdaterOwnerClass {
    /// The current user controls updates.
    User,
    /// An administrator controls updates.
    Admin,
    /// An external package manager controls updates.
    ExternalPackageManager,
    /// A managed fleet rollout service controls updates.
    ManagedFleet,
    /// No updater owns the row.
    None,
}

/// Binary root placement class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BinaryRootClass {
    /// Binary root under a per-user profile program area.
    PerUserProfileProgramArea,
    /// Binary root under a per-machine program area.
    PerMachineProgramArea,
    /// Binary root colocated with a portable directory.
    PortableDirectory,
    /// Binary root extracted from an offline bundle.
    OfflineBundleExtractedProgramArea,
    /// Binary root pinned by admin policy.
    AdminPinnedProgramArea,
    /// Binary root owned by an external package manager.
    ExternalPackageManagerOwnedArea,
}

/// Side-by-side relation between channels or deployment styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideBySideRelationClass {
    /// No side-by-side relation is claimed.
    None,
    /// Stable and Preview are expected to coexist.
    StableAndPreview,
    /// Stable and Beta are expected to coexist.
    StableAndBeta,
    /// Stable and LTS are expected to coexist.
    StableAndLts,
    /// Preview and Beta are expected to coexist.
    PreviewAndBeta,
    /// Installed and portable rows can sit beside each other without sharing durable roots.
    InstalledAndPortable,
    /// Three or more channels are modeled as a channel matrix.
    ThreeChannelMatrix,
    /// Managed and portable rows can sit beside each other without sharing durable roots.
    ManagedAndPortable,
}

/// Rollout ring surfaced for managed or deployment-aware rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutRingClass {
    /// Canary ring.
    Canary,
    /// Pilot ring.
    Pilot,
    /// Broad rollout ring.
    Broad,
    /// Long-term support ring.
    Lts,
}

/// Silent install support class for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SilentInstallSupportClass {
    /// Full silent install, update, rollback, and uninstall support is claimed.
    Full,
    /// Partial silent support is claimed with named limits.
    Partial,
    /// Silent support is not claimed.
    Unsupported,
    /// Silent support is only claimed through a managed deployment lane.
    ManagedOnly,
}

/// Managed package report availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedPackageReportClass {
    /// Managed package report is available.
    Available,
    /// Managed package report is reserved but not yet available.
    Reserved,
    /// Managed package report does not apply to this row.
    NotApplicable,
}

/// Publication or distribution posture for an install row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationPostureClass {
    /// Online vendor-hosted publication.
    OnlineVendor,
    /// Offline signed bundle publication.
    OfflineSignedBundle,
    /// Customer-managed mirror publication.
    CustomerManagedMirror,
    /// Third-party package index publication.
    ThirdPartyPackageIndex,
}

/// Policy injection mechanism available to an install row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyInjectionClass {
    /// No policy injection is available.
    None,
    /// Group Policy Object injection.
    Gpo,
    /// Mobile device management injection.
    Mdm,
    /// macOS configuration-profile injection.
    ConfigProfile,
    /// Command-line flag injection.
    CliFlag,
    /// Environment-variable injection.
    EnvVar,
}

/// Channel pinning posture surfaced by install and update rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelPinningPosture {
    /// User can select or pin the channel.
    UserSelectable,
    /// Admin policy pins the channel.
    AdminPinned,
    /// Managed rollout ring pins the channel.
    ManagedRingPinned,
    /// Channel pinning is not applicable to the row.
    NotApplicable,
}

/// CLI scriptability class for an install row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CliScriptabilityClass {
    /// CLI is present on PATH.
    CliPresentInPath,
    /// CLI is present inside the install tree.
    CliPresentInInstallTree,
    /// CLI is available only from the portable bundle.
    CliPortableOnly,
    /// CLI is not exposed by the row.
    CliNotExposed,
}

/// Repair or verification capability surfaced by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairVerifySupport {
    /// Installer repair is available.
    RepairInstall,
    /// Install signature verification is available.
    VerifyInstallSignature,
    /// Durable state-root integrity verification is available.
    VerifyStateRootIntegrity,
    /// Rollback to a previous build is available.
    RollbackToPreviousBuild,
    /// Uninstall can preserve declared user state while cleaning install state.
    UninstallCleanState,
}

/// Mirror or offline verification state surfaced by install rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOfflineVerificationState {
    /// Mirror and offline verification do not apply to this row.
    NotApplicable,
    /// Online vendor metadata was verified.
    OnlineVendorVerified,
    /// Customer mirror metadata was verified.
    MirrorMetadataVerified,
    /// Offline bundle metadata was verified.
    OfflineBundleVerified,
    /// Metadata is stale and blocks promotion, install, or rollback.
    StaleMetadataBlocked,
}

/// Actor class responsible for rollback or uninstall action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackOwnerClass {
    /// User owns rollback or uninstall.
    User,
    /// Admin owns rollback or uninstall.
    Admin,
    /// External package manager owns rollback or uninstall.
    ExternalPackageManager,
    /// Managed fleet owner controls rollback.
    ManagedFleet,
    /// No rollback owner exists.
    None,
}

/// Product or support surface that consumes topology truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologySurfaceClass {
    /// About surface.
    About,
    /// Update surface.
    Update,
    /// Diagnostics surface.
    Diagnostics,
    /// Install-review surface.
    InstallReview,
    /// CLI inspection surface.
    Cli,
    /// Support-export surface.
    SupportExport,
}

/// OS integration handler kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandlerKind {
    /// File association handler.
    FileAssociation,
    /// Protocol or deep-link handler.
    ProtocolHandler,
    /// Recent-item registration handler.
    RecentItemRegistration,
    /// Default-open behavior.
    DefaultOpenBehavior,
}

/// Import and handoff posture before cross-channel or portable state movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportHandoffPosture {
    /// Import requires compare-before-commit review.
    ExplicitCompareBeforeImport,
    /// No import or handoff is needed for this row.
    NoImportNeeded,
    /// Portable imports do not claim OS entry-point ownership.
    PortableImportOnlyNoOsOwnership,
    /// Admin-managed row does not expose user-owned import handoff.
    AdminManagedNoUserImport,
    /// Mirror or offline import requires verification review.
    MirrorImportReviewRequired,
}

/// Hidden global-state guarantee surfaced for portable and side-by-side rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenGlobalStateGuarantee {
    /// No hidden host-global durable state is claimed.
    NoHiddenGlobalDurableState,
    /// OS integration requires explicit review before host-global mutation.
    OsIntegrationRequiresReview,
    /// Host-global behavior is owned by admin policy.
    AdminPolicyOwned,
}

/// References to upstream contracts consumed by the topology packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractRefs {
    /// Install profile matrix reference.
    pub install_topology_matrix_ref: String,
    /// Durable state-root map reference.
    pub state_root_map_ref: String,
    /// Silent deployment seed reference.
    pub silent_deployment_seed_ref: String,
    /// Boundary manifest reference consumed from the managed-truth lane.
    pub boundary_manifest_ref: String,
    /// Proof artifact index reference consumed from the alpha proof lane.
    pub proof_artifact_index_ref: String,
}

/// Handler ownership and default-open truth for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlerOwnership {
    /// File-association registration class.
    pub file_association_registration_class: String,
    /// Protocol-handler ownership class.
    pub protocol_handler_ownership_class: String,
    /// Recent-item registration class.
    pub recent_item_registration_class: String,
    /// Default-open owner class.
    pub default_open_behavior_owner: String,
    /// Update-marker owner class.
    pub update_marker_owner: String,
    /// Shared scheme default resolution rule.
    pub shared_scheme_default_resolution: String,
    /// Channel currently selected as owner when an OS entry point has a selected owner.
    pub selected_owner_channel: Option<ChannelClass>,
}

/// Shell integration limits surfaced before a row claims OS entry points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellIntegrationLimits {
    /// True when global file associations may be created.
    pub file_associations_may_register: bool,
    /// True when protocol handlers may be created.
    pub protocol_handlers_may_register: bool,
    /// True when host-global services may be created.
    pub global_services_may_register: bool,
    /// Hidden-global-state guarantee.
    pub hidden_global_state_guarantee: HiddenGlobalStateGuarantee,
    /// Import or handoff posture for the row.
    pub import_handoff_posture: ImportHandoffPosture,
    /// Reviewer-facing summary of the integration limits.
    pub limits_summary: String,
}

/// Silent deployment posture for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SilentDeploymentPosture {
    /// Silent-install support class.
    pub support_class: SilentInstallSupportClass,
    /// Return-code families this row resolves into for unattended outcomes.
    pub return_code_family_refs: Vec<String>,
    /// Limits that must be shown on silent or managed deployment surfaces.
    pub disclosed_limits: Vec<String>,
}

/// Rollback and uninstall posture for one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackPosture {
    /// Actor class that owns rollback.
    pub rollback_owner: RollbackOwnerClass,
    /// Actor class that owns uninstall.
    pub uninstall_owner: RollbackOwnerClass,
    /// Rollback target class from the release topology matrix.
    pub rollback_target_class: String,
    /// True when rollback is available for the row.
    pub rollback_available: bool,
    /// Reviewer-facing guidance naming the owner and action.
    pub guidance: String,
}

/// One install-topology alpha row consumed by product and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyRow {
    /// Stable row id.
    pub topology_row_id: String,
    /// Upstream install profile card ref.
    pub install_profile_card_ref: String,
    /// Platform class.
    pub platform_class: PlatformClass,
    /// Architecture class.
    pub architecture_class: ArchitectureClass,
    /// Install mode class.
    pub install_mode_class: InstallModeClass,
    /// Channel class.
    pub channel_class: ChannelClass,
    /// Paired channel class when side-by-side behavior is claimed.
    pub paired_channel_class: Option<ChannelClass>,
    /// Updater owner class.
    pub updater_owner_class: UpdaterOwnerClass,
    /// Binary root class.
    pub binary_root_class: BinaryRootClass,
    /// Durable state-root refs surfaced by diagnostics and support export.
    pub durable_state_root_refs: Vec<String>,
    /// Human-readable state-root topology summary.
    pub state_root_topology_summary: String,
    /// Side-by-side relation class.
    pub side_by_side_relation_class: SideBySideRelationClass,
    /// Rollout ring class.
    pub rollout_ring_class: RolloutRingClass,
    /// Managed package report availability.
    pub managed_package_report_class: ManagedPackageReportClass,
    /// Publication posture.
    pub publication_posture_class: PublicationPostureClass,
    /// Policy injection class.
    pub policy_injection_class: PolicyInjectionClass,
    /// Channel pinning posture.
    pub channel_pinning_posture: ChannelPinningPosture,
    /// CLI scriptability class.
    pub cli_scriptability_class: CliScriptabilityClass,
    /// True when policy bootstrap injection is available for the row.
    pub policy_bootstrap_injection_available: bool,
    /// True when inventory hooks are available for the row.
    pub inventory_hooks_available: bool,
    /// Repair and verification capabilities.
    pub repair_verify_support: Vec<RepairVerifySupport>,
    /// Mirror or offline verification state.
    pub mirror_offline_verification_state: MirrorOfflineVerificationState,
    /// Shell integration limits.
    pub shell_integration_limits: ShellIntegrationLimits,
    /// Handler ownership truth.
    pub handler_ownership: HandlerOwnership,
    /// Silent deployment posture.
    pub silent_deployment_posture: SilentDeploymentPosture,
    /// Rollback and uninstall posture.
    pub rollback_posture: RollbackPosture,
    /// Surfaces that must render the row.
    pub surface_claims: Vec<TopologySurfaceClass>,
}

impl InstallTopologyRow {
    /// Returns true when the row claims side-by-side behavior.
    pub fn is_side_by_side(&self) -> bool {
        self.side_by_side_relation_class != SideBySideRelationClass::None
            || self.install_mode_class == InstallModeClass::SideBySidePreview
    }

    /// Returns true when the row claims portable or unpacked behavior.
    pub fn is_portable(&self) -> bool {
        self.install_mode_class == InstallModeClass::Portable
    }

    /// Builds the cross-surface truth fingerprint for this row.
    pub fn truth_fingerprint(&self) -> InstallTopologyTruthFingerprint {
        InstallTopologyTruthFingerprint {
            install_mode_class: self.install_mode_class,
            channel_class: self.channel_class,
            updater_owner_class: self.updater_owner_class,
            binary_root_class: self.binary_root_class,
            durable_state_root_refs: self.durable_state_root_refs.clone(),
            side_by_side_relation_class: self.side_by_side_relation_class,
            repair_verify_support: self.repair_verify_support.clone(),
            mirror_offline_verification_state: self.mirror_offline_verification_state,
            policy_injection_class: self.policy_injection_class,
            policy_bootstrap_injection_available: self.policy_bootstrap_injection_available,
            inventory_hooks_available: self.inventory_hooks_available,
            cli_scriptability_class: self.cli_scriptability_class,
            file_association_registration_class: self
                .handler_ownership
                .file_association_registration_class
                .clone(),
            protocol_handler_ownership_class: self
                .handler_ownership
                .protocol_handler_ownership_class
                .clone(),
            default_open_behavior_owner: self.handler_ownership.default_open_behavior_owner.clone(),
            silent_install_support_class: self.silent_deployment_posture.support_class,
            rollback_owner: self.rollback_posture.rollback_owner,
            rollback_target_class: self.rollback_posture.rollback_target_class.clone(),
        }
    }
}

/// Preview packet for a file-association or protocol-owner change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandlerOwnershipChangePreview {
    /// Stable preview id.
    pub preview_id: String,
    /// Platform where the preview applies.
    pub platform_class: PlatformClass,
    /// Channel that owns the handler before commit.
    pub before_owner_channel: ChannelClass,
    /// Channel that would own the handler after commit.
    pub after_owner_channel: ChannelClass,
    /// Handler kinds affected by the change.
    pub affected_handlers: Vec<HandlerKind>,
    /// True when the change is reviewed before commit.
    pub previewed_before_commit: bool,
    /// True when commit requires explicit acknowledgement.
    pub commit_requires_acknowledgement: bool,
    /// Actor that owns rollback if the change is committed.
    pub rollback_owner: RollbackOwnerClass,
    /// Diagnostics ref that can explain the pending change.
    pub diagnostics_ref: String,
}

/// Diagnostic row for a stale or displaced handler owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleHandlerOwnerDiagnostic {
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Platform where the stale owner was observed.
    pub platform_class: PlatformClass,
    /// Expected channel owner.
    pub expected_owner_channel: ChannelClass,
    /// Observed channel owner.
    pub observed_owner_channel: ChannelClass,
    /// Channel that displaced the expected owner, when known.
    pub displaced_by_channel: Option<ChannelClass>,
    /// Handler kinds diagnosed.
    pub affected_handlers: Vec<HandlerKind>,
    /// True when diagnostics can explain the state without installer logs.
    pub diagnosed_without_installer_logs: bool,
    /// Actor that owns the next corrective action.
    pub next_action_owner: RollbackOwnerClass,
    /// Support-export row ref carrying this diagnostic.
    pub support_export_ref: String,
}

/// Install-topology alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyAlphaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Upstream contract refs.
    pub contract_refs: ContractRefs,
    /// Claimed install topology rows.
    pub rows: Vec<InstallTopologyRow>,
    /// Handler-owner change previews.
    pub handler_ownership_change_previews: Vec<HandlerOwnershipChangePreview>,
    /// Stale or displaced handler diagnostics.
    pub stale_handler_owner_diagnostics: Vec<StaleHandlerOwnerDiagnostic>,
}

impl InstallTopologyAlphaPacket {
    /// Validates row coverage, non-contradiction, handler ownership, portable
    /// guarantees, silent-deployment truth, and support-export posture.
    pub fn validate(&self) -> InstallTopologyValidationReport {
        let mut validator = InstallTopologyValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Returns one product or support surface projection from the packet.
    pub fn surface_projection(
        &self,
        surface_class: TopologySurfaceClass,
    ) -> InstallTopologySurfaceProjection {
        let rows = self
            .rows
            .iter()
            .filter(|row| row.surface_claims.contains(&surface_class))
            .map(InstallTopologySurfaceRow::from)
            .collect();
        InstallTopologySurfaceProjection {
            surface_class,
            packet_id: self.packet_id.clone(),
            rows,
        }
    }

    /// Returns a metadata-safe support-export projection.
    pub fn support_export_projection(&self) -> InstallTopologySupportExport {
        InstallTopologySupportExport {
            record_kind: INSTALL_TOPOLOGY_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: INSTALL_TOPOLOGY_ALPHA_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            projection: self.surface_projection(TopologySurfaceClass::SupportExport),
            handler_diagnostics: self.stale_handler_owner_diagnostics.clone(),
            redaction_class: "metadata_only_no_paths_or_secrets".to_string(),
        }
    }

    /// Finds a row by id.
    pub fn row_by_id(&self, topology_row_id: &str) -> Option<&InstallTopologyRow> {
        self.rows
            .iter()
            .find(|row| row.topology_row_id == topology_row_id)
    }

    /// Returns true when roots for two channels are disjoint.
    pub fn state_roots_disjoint_for_channels(
        &self,
        left: ChannelClass,
        right: ChannelClass,
    ) -> bool {
        let left_roots: BTreeSet<&String> = self
            .rows
            .iter()
            .filter(|row| row.channel_class == left)
            .flat_map(|row| row.durable_state_root_refs.iter())
            .collect();
        let right_roots: BTreeSet<&String> = self
            .rows
            .iter()
            .filter(|row| row.channel_class == right)
            .flat_map(|row| row.durable_state_root_refs.iter())
            .collect();
        !left_roots.is_empty() && !right_roots.is_empty() && left_roots.is_disjoint(&right_roots)
    }
}

/// One row rendered on a product or support surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologySurfaceRow {
    /// Stable row id.
    pub topology_row_id: String,
    /// Upstream install profile card ref.
    pub install_profile_card_ref: String,
    /// Platform class.
    pub platform_class: PlatformClass,
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
    /// Repair and verification capabilities.
    pub repair_verify_support: Vec<RepairVerifySupport>,
    /// Mirror or offline verification state.
    pub mirror_offline_verification_state: MirrorOfflineVerificationState,
    /// Policy injection class.
    pub policy_injection_class: PolicyInjectionClass,
    /// Channel pinning posture.
    pub channel_pinning_posture: ChannelPinningPosture,
    /// CLI scriptability class.
    pub cli_scriptability_class: CliScriptabilityClass,
    /// Handler ownership truth.
    pub handler_ownership: HandlerOwnership,
    /// Silent deployment posture.
    pub silent_deployment_posture: SilentDeploymentPosture,
    /// Rollback and uninstall posture.
    pub rollback_posture: RollbackPosture,
    /// Surface-stable truth fingerprint.
    pub truth_fingerprint: InstallTopologyTruthFingerprint,
}

impl From<&InstallTopologyRow> for InstallTopologySurfaceRow {
    fn from(row: &InstallTopologyRow) -> Self {
        Self {
            topology_row_id: row.topology_row_id.clone(),
            install_profile_card_ref: row.install_profile_card_ref.clone(),
            platform_class: row.platform_class,
            install_mode_class: row.install_mode_class,
            channel_class: row.channel_class,
            updater_owner_class: row.updater_owner_class,
            binary_root_class: row.binary_root_class,
            durable_state_root_refs: row.durable_state_root_refs.clone(),
            repair_verify_support: row.repair_verify_support.clone(),
            mirror_offline_verification_state: row.mirror_offline_verification_state,
            policy_injection_class: row.policy_injection_class,
            channel_pinning_posture: row.channel_pinning_posture,
            cli_scriptability_class: row.cli_scriptability_class,
            handler_ownership: row.handler_ownership.clone(),
            silent_deployment_posture: row.silent_deployment_posture.clone(),
            rollback_posture: row.rollback_posture.clone(),
            truth_fingerprint: row.truth_fingerprint(),
        }
    }
}

/// Projection for one product or support surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologySurfaceProjection {
    /// Surface class.
    pub surface_class: TopologySurfaceClass,
    /// Source packet id.
    pub packet_id: String,
    /// Rows rendered on the surface.
    pub rows: Vec<InstallTopologySurfaceRow>,
}

impl InstallTopologySurfaceProjection {
    /// Returns row truth fingerprints keyed by topology row id.
    pub fn truth_fingerprints(&self) -> BTreeMap<String, InstallTopologyTruthFingerprint> {
        self.rows
            .iter()
            .map(|row| (row.topology_row_id.clone(), row.truth_fingerprint.clone()))
            .collect()
    }
}

/// Metadata-safe support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologySupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source packet id.
    pub packet_id: String,
    /// Support-export surface projection.
    pub projection: InstallTopologySurfaceProjection,
    /// Handler diagnostics included in the support export.
    pub handler_diagnostics: Vec<StaleHandlerOwnerDiagnostic>,
    /// Redaction class for the projection.
    pub redaction_class: String,
}

/// Cross-surface fingerprint for fields that must never contradict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyTruthFingerprint {
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
    /// Repair and verification capabilities.
    pub repair_verify_support: Vec<RepairVerifySupport>,
    /// Mirror or offline verification state.
    pub mirror_offline_verification_state: MirrorOfflineVerificationState,
    /// Policy injection class.
    pub policy_injection_class: PolicyInjectionClass,
    /// True when policy bootstrap injection is available.
    pub policy_bootstrap_injection_available: bool,
    /// True when inventory hooks are available.
    pub inventory_hooks_available: bool,
    /// CLI scriptability class.
    pub cli_scriptability_class: CliScriptabilityClass,
    /// File-association registration class.
    pub file_association_registration_class: String,
    /// Protocol-handler ownership class.
    pub protocol_handler_ownership_class: String,
    /// Default-open owner class.
    pub default_open_behavior_owner: String,
    /// Silent install support class.
    pub silent_install_support_class: SilentInstallSupportClass,
    /// Rollback owner.
    pub rollback_owner: RollbackOwnerClass,
    /// Rollback target class.
    pub rollback_target_class: String,
}

/// Validation coverage collected from an install-topology packet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyCoverage {
    /// Install modes covered by rows.
    pub install_modes: BTreeSet<InstallModeClass>,
    /// Channels covered by rows.
    pub channels: BTreeSet<ChannelClass>,
    /// Platforms covered by rows.
    pub platforms: BTreeSet<PlatformClass>,
    /// Product and support surfaces covered by rows.
    pub surfaces: BTreeSet<TopologySurfaceClass>,
    /// Publication postures covered by rows.
    pub publication_postures: BTreeSet<PublicationPostureClass>,
    /// Mirror or offline verification states covered by rows.
    pub mirror_offline_states: BTreeSet<MirrorOfflineVerificationState>,
    /// Handler kinds covered by preview or diagnostic rows.
    pub handler_kinds: BTreeSet<HandlerKind>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyValidationFinding {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable finding message.
    pub message: String,
    /// Row or packet ref associated with the finding.
    pub ref_id: String,
}

/// Validation report for an install-topology packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallTopologyValidationReport {
    /// True when validation found no errors.
    pub passed: bool,
    /// Coverage collected while validating.
    pub coverage: InstallTopologyCoverage,
    /// Validation findings.
    pub findings: Vec<InstallTopologyValidationFinding>,
}

struct InstallTopologyValidator<'a> {
    packet: &'a InstallTopologyAlphaPacket,
    coverage: InstallTopologyCoverage,
    findings: Vec<InstallTopologyValidationFinding>,
    seen_row_ids: BTreeSet<String>,
    seen_card_refs: BTreeSet<String>,
}

impl<'a> InstallTopologyValidator<'a> {
    fn new(packet: &'a InstallTopologyAlphaPacket) -> Self {
        Self {
            packet,
            coverage: InstallTopologyCoverage::default(),
            findings: Vec::new(),
            seen_row_ids: BTreeSet::new(),
            seen_card_refs: BTreeSet::new(),
        }
    }

    fn validate(&mut self) {
        self.validate_header();
        for row in &self.packet.rows {
            self.validate_row(row);
        }
        self.validate_required_coverage();
        self.validate_side_by_side_roots();
        self.validate_handler_change_previews();
        self.validate_stale_handler_diagnostics();
    }

    fn finish(self) -> InstallTopologyValidationReport {
        InstallTopologyValidationReport {
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn push(&mut self, check_id: &str, message: impl Into<String>, ref_id: impl Into<String>) {
        self.findings.push(InstallTopologyValidationFinding {
            check_id: check_id.to_string(),
            message: message.into(),
            ref_id: ref_id.into(),
        });
    }

    fn validate_header(&mut self) {
        if self.packet.record_kind != INSTALL_TOPOLOGY_ALPHA_PACKET_RECORD_KIND {
            self.push(
                "install_topology.packet.record_kind",
                "packet record_kind is not install_topology_alpha_packet",
                &self.packet.packet_id,
            );
        }
        if self.packet.schema_version != INSTALL_TOPOLOGY_ALPHA_SCHEMA_VERSION {
            self.push(
                "install_topology.packet.schema_version",
                "packet schema_version is unsupported",
                &self.packet.packet_id,
            );
        }
        if self.packet.rows.is_empty() {
            self.push(
                "install_topology.packet.rows_empty",
                "packet must contain at least one topology row",
                &self.packet.packet_id,
            );
        }
    }

    fn validate_row(&mut self, row: &InstallTopologyRow) {
        if row.topology_row_id.trim().is_empty() {
            self.push(
                "install_topology.row.id_missing",
                "topology row id must not be empty",
                row.install_profile_card_ref.clone(),
            );
        }
        if !self.seen_row_ids.insert(row.topology_row_id.clone()) {
            self.push(
                "install_topology.row.id_duplicate",
                "topology row id must be unique",
                row.topology_row_id.clone(),
            );
        }
        if !self
            .seen_card_refs
            .insert(row.install_profile_card_ref.clone())
        {
            self.push(
                "install_topology.row.install_profile_duplicate",
                "install profile card ref must be unique in the alpha packet",
                row.topology_row_id.clone(),
            );
        }

        self.coverage.install_modes.insert(row.install_mode_class);
        self.coverage.channels.insert(row.channel_class);
        self.coverage.platforms.insert(row.platform_class);
        self.coverage
            .publication_postures
            .insert(row.publication_posture_class);
        self.coverage
            .mirror_offline_states
            .insert(row.mirror_offline_verification_state);
        for surface in &row.surface_claims {
            self.coverage.surfaces.insert(*surface);
        }

        if row.durable_state_root_refs.is_empty() {
            self.push(
                "install_topology.row.state_roots_missing",
                "row must disclose at least one durable state root",
                row.topology_row_id.clone(),
            );
        }
        if row.state_root_topology_summary.trim().is_empty() {
            self.push(
                "install_topology.row.state_root_summary_missing",
                "row must include a reviewer-facing state-root summary",
                row.topology_row_id.clone(),
            );
        }
        if row.is_side_by_side() && row.side_by_side_relation_class == SideBySideRelationClass::None
        {
            self.push(
                "install_topology.row.side_by_side_relation_missing",
                "side-by-side row must name a non-none relation",
                row.topology_row_id.clone(),
            );
        }
        if row.is_side_by_side() && row.paired_channel_class.is_none() {
            self.push(
                "install_topology.row.paired_channel_missing",
                "side-by-side row must name its paired channel",
                row.topology_row_id.clone(),
            );
        }
        if !row.is_side_by_side() && row.paired_channel_class.is_some() {
            self.push(
                "install_topology.row.paired_channel_unexpected",
                "non-side-by-side row must not name a paired channel",
                row.topology_row_id.clone(),
            );
        }
        self.validate_row_surfaces(row);
        self.validate_repair_verify(row);
        self.validate_portable_limits(row);
        self.validate_silent_and_managed_truth(row);
        self.validate_mirror_offline_truth(row);
    }

    fn validate_row_surfaces(&mut self, row: &InstallTopologyRow) {
        let required = required_surfaces();
        let actual: BTreeSet<TopologySurfaceClass> = row.surface_claims.iter().copied().collect();
        let missing: Vec<_> = required.difference(&actual).copied().collect();
        if !missing.is_empty() {
            self.push(
                "install_topology.row.surface_coverage_missing",
                format!("row is missing required surface claims: {missing:?}"),
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_repair_verify(&mut self, row: &InstallTopologyRow) {
        let support: BTreeSet<RepairVerifySupport> =
            row.repair_verify_support.iter().copied().collect();
        for required in [
            RepairVerifySupport::VerifyInstallSignature,
            RepairVerifySupport::VerifyStateRootIntegrity,
        ] {
            if !support.contains(&required) {
                self.push(
                    "install_topology.row.verify_support_missing",
                    format!("row must disclose {required:?} support"),
                    row.topology_row_id.clone(),
                );
            }
        }
        if row.rollback_posture.rollback_available
            && !support.contains(&RepairVerifySupport::RollbackToPreviousBuild)
            && !row.is_portable()
        {
            self.push(
                "install_topology.row.rollback_support_missing",
                "rollback-available rows must disclose rollback support",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_portable_limits(&mut self, row: &InstallTopologyRow) {
        if !row.is_portable() {
            return;
        }
        if row.shell_integration_limits.file_associations_may_register
            || row.shell_integration_limits.protocol_handlers_may_register
            || row.shell_integration_limits.global_services_may_register
        {
            self.push(
                "install_topology.portable.global_integration_claimed",
                "portable rows must not claim host-global handlers or services",
                row.topology_row_id.clone(),
            );
        }
        if row.shell_integration_limits.hidden_global_state_guarantee
            != HiddenGlobalStateGuarantee::NoHiddenGlobalDurableState
        {
            self.push(
                "install_topology.portable.hidden_global_state_not_closed",
                "portable rows must guarantee no hidden host-global durable state",
                row.topology_row_id.clone(),
            );
        }
        if row.handler_ownership.file_association_registration_class != "not_registered"
            || row.handler_ownership.protocol_handler_ownership_class != "not_registered"
        {
            self.push(
                "install_topology.portable.handler_ownership_not_closed",
                "portable rows must disclose file and protocol handlers as not registered",
                row.topology_row_id.clone(),
            );
        }
        if row.rollback_posture.rollback_owner != RollbackOwnerClass::User {
            self.push(
                "install_topology.portable.rollback_owner_not_user",
                "portable rollback must name the user as owner",
                row.topology_row_id.clone(),
            );
        }
        if row.silent_deployment_posture.disclosed_limits.is_empty() {
            self.push(
                "install_topology.portable.silent_limits_missing",
                "portable rows must disclose silent-deployment limits",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_silent_and_managed_truth(&mut self, row: &InstallTopologyRow) {
        if row.silent_deployment_posture.support_class != SilentInstallSupportClass::Unsupported
            && row
                .silent_deployment_posture
                .return_code_family_refs
                .is_empty()
        {
            self.push(
                "install_topology.row.silent_return_codes_missing",
                "silent-capable rows must disclose return-code families",
                row.topology_row_id.clone(),
            );
        }
        if matches!(
            row.updater_owner_class,
            UpdaterOwnerClass::Admin | UpdaterOwnerClass::ManagedFleet
        ) && row.rollback_posture.rollback_owner == RollbackOwnerClass::None
        {
            self.push(
                "install_topology.row.rollback_owner_missing",
                "admin or managed rows must name who owns rollback",
                row.topology_row_id.clone(),
            );
        }
        if matches!(
            row.install_mode_class,
            InstallModeClass::PerMachineInstalled | InstallModeClass::ManagedDeployed
        ) && row.channel_pinning_posture == ChannelPinningPosture::NotApplicable
        {
            self.push(
                "install_topology.row.channel_pinning_missing",
                "managed or per-machine rows must disclose channel pinning posture",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_mirror_offline_truth(&mut self, row: &InstallTopologyRow) {
        if matches!(
            row.publication_posture_class,
            PublicationPostureClass::CustomerManagedMirror
                | PublicationPostureClass::OfflineSignedBundle
        ) && row.mirror_offline_verification_state
            == MirrorOfflineVerificationState::NotApplicable
        {
            self.push(
                "install_topology.row.mirror_offline_state_missing",
                "mirror or offline rows must disclose verification posture",
                row.topology_row_id.clone(),
            );
        }
        if row.install_mode_class == InstallModeClass::OfflineBundle
            && row.policy_injection_class == PolicyInjectionClass::None
        {
            self.push(
                "install_topology.row.offline_policy_root_missing",
                "offline bundle rows must disclose a policy or bootstrap root",
                row.topology_row_id.clone(),
            );
        }
    }

    fn validate_required_coverage(&mut self) {
        for mode in [
            InstallModeClass::PerUserInstalled,
            InstallModeClass::PerMachineInstalled,
            InstallModeClass::Portable,
            InstallModeClass::ManagedDeployed,
            InstallModeClass::SideBySidePreview,
            InstallModeClass::OfflineBundle,
        ] {
            if !self.coverage.install_modes.contains(&mode) {
                self.push(
                    "install_topology.coverage.install_mode_missing",
                    format!("required install mode is not covered: {mode:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for channel in [
            ChannelClass::Stable,
            ChannelClass::Preview,
            ChannelClass::PortableStable,
        ] {
            if !self.coverage.channels.contains(&channel) {
                self.push(
                    "install_topology.coverage.channel_missing",
                    format!("required channel is not covered: {channel:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        for surface in required_surfaces() {
            if !self.coverage.surfaces.contains(&surface) {
                self.push(
                    "install_topology.coverage.surface_missing",
                    format!("required surface is not covered: {surface:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
        if !self
            .coverage
            .publication_postures
            .contains(&PublicationPostureClass::CustomerManagedMirror)
            && !self
                .coverage
                .publication_postures
                .contains(&PublicationPostureClass::OfflineSignedBundle)
        {
            self.push(
                "install_topology.coverage.mirror_or_airgap_missing",
                "packet must cover mirror or air-gapped delivery",
                self.packet.packet_id.clone(),
            );
        }
    }

    fn validate_side_by_side_roots(&mut self) {
        if !self
            .packet
            .state_roots_disjoint_for_channels(ChannelClass::Stable, ChannelClass::Preview)
        {
            self.push(
                "install_topology.side_by_side.state_roots_overlap",
                "stable and preview rows must expose distinct durable state roots",
                self.packet.packet_id.clone(),
            );
        }
    }

    fn validate_handler_change_previews(&mut self) {
        if self.packet.handler_ownership_change_previews.is_empty() {
            self.push(
                "install_topology.handler_preview.missing",
                "at least one handler-owner change preview is required",
                self.packet.packet_id.clone(),
            );
        }
        for preview in &self.packet.handler_ownership_change_previews {
            for handler in &preview.affected_handlers {
                self.coverage.handler_kinds.insert(*handler);
            }
            if preview.before_owner_channel == preview.after_owner_channel {
                self.push(
                    "install_topology.handler_preview.owner_unchanged",
                    "handler preview must show an owner change",
                    preview.preview_id.clone(),
                );
            }
            if preview.affected_handlers.is_empty() {
                self.push(
                    "install_topology.handler_preview.handlers_missing",
                    "handler preview must name affected handlers",
                    preview.preview_id.clone(),
                );
            }
            if !preview.previewed_before_commit || !preview.commit_requires_acknowledgement {
                self.push(
                    "install_topology.handler_preview.not_reviewed_before_commit",
                    "handler owner changes must be previewed before commit and require acknowledgement",
                    preview.preview_id.clone(),
                );
            }
            if preview.diagnostics_ref.trim().is_empty() {
                self.push(
                    "install_topology.handler_preview.diagnostics_ref_missing",
                    "handler preview must name diagnostics ref",
                    preview.preview_id.clone(),
                );
            }
        }
        for required in [HandlerKind::FileAssociation, HandlerKind::ProtocolHandler] {
            if !self.coverage.handler_kinds.contains(&required) {
                self.push(
                    "install_topology.handler_preview.required_handler_missing",
                    format!("handler preview or diagnostic must cover {required:?}"),
                    self.packet.packet_id.clone(),
                );
            }
        }
    }

    fn validate_stale_handler_diagnostics(&mut self) {
        if self.packet.stale_handler_owner_diagnostics.is_empty() {
            self.push(
                "install_topology.handler_diagnostic.missing",
                "at least one stale or displaced handler diagnostic is required",
                self.packet.packet_id.clone(),
            );
        }
        for diagnostic in &self.packet.stale_handler_owner_diagnostics {
            for handler in &diagnostic.affected_handlers {
                self.coverage.handler_kinds.insert(*handler);
            }
            if diagnostic.expected_owner_channel == diagnostic.observed_owner_channel {
                self.push(
                    "install_topology.handler_diagnostic.owner_not_stale",
                    "diagnostic must show an observed owner different from expected owner",
                    diagnostic.diagnostic_id.clone(),
                );
            }
            if diagnostic.affected_handlers.is_empty() {
                self.push(
                    "install_topology.handler_diagnostic.handlers_missing",
                    "diagnostic must name affected handlers",
                    diagnostic.diagnostic_id.clone(),
                );
            }
            if !diagnostic.diagnosed_without_installer_logs {
                self.push(
                    "install_topology.handler_diagnostic.requires_installer_logs",
                    "stale handler owner must be diagnosable without reading installer logs",
                    diagnostic.diagnostic_id.clone(),
                );
            }
            if diagnostic.support_export_ref.trim().is_empty() {
                self.push(
                    "install_topology.handler_diagnostic.support_ref_missing",
                    "diagnostic must carry a support-export ref",
                    diagnostic.diagnostic_id.clone(),
                );
            }
        }
    }
}

fn required_surfaces() -> BTreeSet<TopologySurfaceClass> {
    [
        TopologySurfaceClass::About,
        TopologySurfaceClass::Update,
        TopologySurfaceClass::Diagnostics,
        TopologySurfaceClass::InstallReview,
        TopologySurfaceClass::Cli,
        TopologySurfaceClass::SupportExport,
    ]
    .into_iter()
    .collect()
}
