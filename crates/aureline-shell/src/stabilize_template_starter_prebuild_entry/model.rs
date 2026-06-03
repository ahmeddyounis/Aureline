//! Canonical template, starter, and prebuild entry disclosure truth model.
//!
//! ## Why one disclosure record per accelerator entry
//!
//! Templates, starters, and prebuilds are accelerators, not hidden setup tunnels.
//! When each surface invents its own disclosure vocabulary, users cannot tell
//! whether they are resuming a live workspace, opening a snapshot, cloning fresh,
//! or applying a setup starter, and the bypass path is buried or missing.
//!
//! This module mints one governed [`TemplateStarterPrebuildEntryRecord`] per
//! accelerator entry surface. The record binds:
//!
//! - **Identity** — what the accelerator is, its version, and its manifest ref.
//! - **Source and support** — first-party, community, experimental, etc.
//! - **Runtime and scope** — local-only, devcontainer, remote image, managed cloud.
//! - **Freshness truth** — fresh, near expiry, stale, expired, or unknown.
//! - **Setup expectations** — what actions will run, how long, and what connectivity
//!   is required.
//! - **Side-effect envelope** — network egress, extension installs, remote
//!   provisioning, managed services, and credential provisioning.
//! - **Resulting mode** — resume live, start from snapshot, clone fresh, open with
//!   setup, open minimal, create empty, create project, create service, add module.
//! - **Bypass paths** — same-weight alternatives such as "Open without starter" or
//!   "Set up later".
//! - **Trust and auth boundaries** — what must be granted before setup begins.
//! - **Cleanup and rollback** — how to undo or recover if the accelerator fails.
//! - **Failure summary** — what succeeded, what was skipped, what was partially
//!   applied, what cleanup ran, and what remains for the user to review.
//!
//! ## The honesty invariants
//!
//! The builder refuses to mint a record that would lie. Each is a [`BuildError`]:
//!
//! - **Bypass parity.** Every record carries at least one bypass path, and the
//!   bypass continuity class is `equal_weight_with_apply`.
//! - **Source honesty.** Community and uncertified sources carry at least one trust
//!   note; missing signers are not hidden behind generic posture.
//! - **Runtime consistency.** A `local_only` runtime cannot require remote
//!   provisioning or managed services. A `managed_cloud_required` runtime must
//!   declare both a managed-service class and network egress.
//! - **Freshness for prebuilds.** Prebuild entries must declare freshness; template
//!   entries may but are not required to.
//! - **Failure transparency.** Partial application is disclosed, not hidden behind
//!   generic failure copy.
//! - **No raw secrets in export.** Support export metadata keeps raw secrets,
//!   command lines, and URLs redacted.
//! - **Entry disclosure stays separate from scaffolding.** The record never carries
//!   a scaffold plan, scaffold run, or generated-project lineage; those belong to
//!   the scaffold lane.
//! - **Intent preservation.** `Create project`, `Create service`, `Add module`,
//!   `Create without starter`, and `Open without starter` stay distinct; the record
//!   never quietly broadens generation scope.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt;

/// Stable record-kind tag carried in serialized disclosure records.
pub const TEMPLATE_STARTER_PREBUILD_ENTRY_RECORD_KIND: &str =
    "template_starter_prebuild_entry_record";

/// Schema version for the [`TemplateStarterPrebuildEntryRecord`] payload shape.
pub const TEMPLATE_STARTER_PREBUILD_ENTRY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const TEMPLATE_STARTER_PREBUILD_ENTRY_SHARED_CONTRACT_REF: &str =
    "shell:template_starter_prebuild_entry_stable:v1";

/// Reviewer-facing notice rendered on every disclosure surface.
pub const TEMPLATE_STARTER_PREBUILD_ENTRY_NOTICE: &str =
    "Template, starter, and prebuild entry disclosure truth: every accelerator surface \
     explains what it is, where it came from, what it will do, what it will not do yet, \
     and how to bypass it without losing control; resume live workspace, start from \
     snapshot, clone fresh, and open without starter are distinct resulting modes with \
     explicit wording and keyboard-reachable actions; auth, trust, registry, mirror, \
     managed-service, and download boundaries are shown before setup begins; failure \
     summaries disclose what succeeded, what was skipped, what was partially applied, \
     what cleanup ran, and what remains for review; starter and prebuild identity is \
     versioned and inspectable in diagnostics and support export; entry disclosure \
     stays separate from scaffolding. Shell, diagnostics, support exports, Help/About, \
     and docs read this record verbatim.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// What kind of accelerator is being disclosed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    /// A workspace template.
    Template,
    /// A project or service starter.
    Starter,
    /// A prebuilt environment snapshot.
    Prebuild,
}

impl EntryKind {
    /// Returns the stable string vocabulary for this entry kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Template => "template",
            Self::Starter => "starter",
            Self::Prebuild => "prebuild",
        }
    }
}

/// Closed set of source classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    FirstParty,
    TeamManaged,
    Community,
    LocalOnly,
    MirrorCached,
    Uncertified,
}

impl SourceClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::TeamManaged => "team_managed",
            Self::Community => "community",
            Self::LocalOnly => "local_only",
            Self::MirrorCached => "mirror_cached",
            Self::Uncertified => "uncertified",
        }
    }
}

/// Closed set of support classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    OfficiallySupported,
    CommunitySupported,
    Experimental,
    LegacyDeprecated,
    Unsupported,
    SupportUnknown,
}

impl SupportClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficiallySupported => "officially_supported",
            Self::CommunitySupported => "community_supported",
            Self::Experimental => "experimental",
            Self::LegacyDeprecated => "legacy_deprecated",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }
}

/// Closed set of runtime scope classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeScopeClass {
    LocalOnly,
    LocalWithDevcontainer,
    LocalWithContainer,
    RemoteImageRequired,
    ManagedCloudRequired,
    MixedLocalAndRemote,
    NotDeclared,
}

impl RuntimeScopeClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::LocalWithDevcontainer => "local_with_devcontainer",
            Self::LocalWithContainer => "local_with_container",
            Self::RemoteImageRequired => "remote_image_required",
            Self::ManagedCloudRequired => "managed_cloud_required",
            Self::MixedLocalAndRemote => "mixed_local_and_remote",
            Self::NotDeclared => "not_declared",
        }
    }
}

/// Closed set of host boundary classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundaryClass {
    HostLocalDeviceOnly,
    HostLocalWithDevcontainerAttached,
    HostLocalWithContainerAttached,
    HostRemoteImageRequired,
    HostManagedWorkspaceRequired,
    HostMixedLocalAndRemote,
    HostBoundaryUnknownRequiresReview,
}

impl HostBoundaryClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostLocalDeviceOnly => "host_local_device_only",
            Self::HostLocalWithDevcontainerAttached => "host_local_with_devcontainer_attached",
            Self::HostLocalWithContainerAttached => "host_local_with_container_attached",
            Self::HostRemoteImageRequired => "host_remote_image_required",
            Self::HostManagedWorkspaceRequired => "host_managed_workspace_required",
            Self::HostMixedLocalAndRemote => "host_mixed_local_and_remote",
            Self::HostBoundaryUnknownRequiresReview => "host_boundary_unknown_requires_review",
        }
    }
}

/// Closed set of freshness age classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    FreshUnderWindow,
    NearExpiry,
    StaleOverWindow,
    Expired,
    UnknownRequiresRevalidation,
}

impl FreshnessClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshUnderWindow => "fresh_under_window",
            Self::NearExpiry => "near_expiry",
            Self::StaleOverWindow => "stale_over_window",
            Self::Expired => "expired",
            Self::UnknownRequiresRevalidation => "unknown_requires_revalidation",
        }
    }
}

/// Closed set of resulting modes for template/starter/prebuild entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultingMode {
    /// Resume an actively materialized workspace.
    ResumeLiveWorkspace,
    /// Start from a prebuilt snapshot.
    StartFromSnapshot,
    /// Clone fresh and then apply the accelerator.
    CloneFresh,
    /// Open prebuild and run its setup actions.
    OpenPrebuildWithSetupActions,
    /// Open prebuild with minimal setup.
    OpenPrebuildMinimal,
    /// Open the workspace without using the accelerator.
    OpenWithoutStarter,
    /// Create an empty workspace.
    CreateEmptyWorkspace,
    /// Create a new project.
    CreateProject,
    /// Create a new service.
    CreateService,
    /// Add a module to the active workspace.
    AddModule,
}

impl ResultingMode {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResumeLiveWorkspace => "resume_live_workspace",
            Self::StartFromSnapshot => "start_from_snapshot",
            Self::CloneFresh => "clone_fresh",
            Self::OpenPrebuildWithSetupActions => "open_prebuild_with_setup_actions",
            Self::OpenPrebuildMinimal => "open_prebuild_minimal",
            Self::OpenWithoutStarter => "open_without_starter",
            Self::CreateEmptyWorkspace => "create_empty_workspace",
            Self::CreateProject => "create_project",
            Self::CreateService => "create_service",
            Self::AddModule => "add_module",
        }
    }
}

/// Closed set of bypass path classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BypassPathClass {
    OpenFolderWithoutStarter,
    OpenWorkspaceWithoutStarter,
    CloneRepositoryWithoutStarter,
    CreateEmptyWorkspace,
    OpenPrebuildMinimal,
    SetUpLater,
    ContinueWithoutStarter,
}

impl BypassPathClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFolderWithoutStarter => "open_folder_without_starter",
            Self::OpenWorkspaceWithoutStarter => "open_workspace_without_starter",
            Self::CloneRepositoryWithoutStarter => "clone_repository_without_starter",
            Self::CreateEmptyWorkspace => "create_empty_workspace",
            Self::OpenPrebuildMinimal => "open_prebuild_minimal",
            Self::SetUpLater => "set_up_later",
            Self::ContinueWithoutStarter => "continue_without_starter",
        }
    }

    /// Returns a human-readable label for the bypass path.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenFolderWithoutStarter => "Open folder without starter",
            Self::OpenWorkspaceWithoutStarter => "Open workspace without starter",
            Self::CloneRepositoryWithoutStarter => "Clone repository without starter",
            Self::CreateEmptyWorkspace => "Create empty workspace",
            Self::OpenPrebuildMinimal => "Open prebuild minimal",
            Self::SetUpLater => "Set up later",
            Self::ContinueWithoutStarter => "Continue without starter",
        }
    }
}

/// Closed set of network egress classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkEgressClass {
    NoNetworkEgressRequired,
    EgressToFirstPartyOriginOnly,
    EgressToTeamManagedMirrorOnly,
    EgressToCommunityOriginUserReviewRequired,
    EgressToManagedWorkspaceEnvelopeOnly,
    EgressEnvelopeUnknownRequiresReview,
}

impl NetworkEgressClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoNetworkEgressRequired => "no_network_egress_required",
            Self::EgressToFirstPartyOriginOnly => "egress_to_first_party_origin_only",
            Self::EgressToTeamManagedMirrorOnly => "egress_to_team_managed_mirror_only",
            Self::EgressToCommunityOriginUserReviewRequired => {
                "egress_to_community_origin_user_review_required"
            }
            Self::EgressToManagedWorkspaceEnvelopeOnly => {
                "egress_to_managed_workspace_envelope_only"
            }
            Self::EgressEnvelopeUnknownRequiresReview => "egress_envelope_unknown_requires_review",
        }
    }
}

/// Closed set of extension install classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionInstallClass {
    NoExtensionInstallRequired,
    FirstPartyExtensionInstallRequired,
    OrganizationCuratedExtensionInstallRequired,
    MarketplaceExtensionInstallUserReviewRequired,
    ManagedOnlyChannelExtensionInstallRequired,
    ExtensionInstallReviewRequiredSignatureUnverified,
    ExtensionInstallClassUnknownRequiresReview,
}

impl ExtensionInstallClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoExtensionInstallRequired => "no_extension_install_required",
            Self::FirstPartyExtensionInstallRequired => "first_party_extension_install_required",
            Self::OrganizationCuratedExtensionInstallRequired => {
                "organization_curated_extension_install_required"
            }
            Self::MarketplaceExtensionInstallUserReviewRequired => {
                "marketplace_extension_install_user_review_required"
            }
            Self::ManagedOnlyChannelExtensionInstallRequired => {
                "managed_only_channel_extension_install_required"
            }
            Self::ExtensionInstallReviewRequiredSignatureUnverified => {
                "extension_install_review_required_signature_unverified"
            }
            Self::ExtensionInstallClassUnknownRequiresReview => {
                "extension_install_class_unknown_requires_review"
            }
        }
    }
}

/// Closed set of remote provisioning classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteProvisioningClass {
    NoRemoteProvisioningRequired,
    DevcontainerAttachRequired,
    ContainerAttachRequired,
    RemoteImageRequired,
    ManagedWorkspaceRequired,
    MixedLocalAndRemoteProvisioningRequired,
    RemoteProvisioningUnknownRequiresReview,
}

impl RemoteProvisioningClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRemoteProvisioningRequired => "no_remote_provisioning_required",
            Self::DevcontainerAttachRequired => "devcontainer_attach_required",
            Self::ContainerAttachRequired => "container_attach_required",
            Self::RemoteImageRequired => "remote_image_required",
            Self::ManagedWorkspaceRequired => "managed_workspace_required",
            Self::MixedLocalAndRemoteProvisioningRequired => {
                "mixed_local_and_remote_provisioning_required"
            }
            Self::RemoteProvisioningUnknownRequiresReview => {
                "remote_provisioning_unknown_requires_review"
            }
        }
    }
}

/// Closed set of managed service classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedServiceClass {
    NoManagedServiceRequired,
    ManagedWorkspaceEnvelopeRequired,
    ManagedOnlyChannelInvocationRequired,
    ThirdPartyConnectedProviderRequired,
    FirstPartyManagedServiceRequired,
    ManagedServiceClassUnknownRequiresReview,
}

impl ManagedServiceClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoManagedServiceRequired => "no_managed_service_required",
            Self::ManagedWorkspaceEnvelopeRequired => "managed_workspace_envelope_required",
            Self::ManagedOnlyChannelInvocationRequired => {
                "managed_only_channel_invocation_required"
            }
            Self::ThirdPartyConnectedProviderRequired => "third_party_connected_provider_required",
            Self::FirstPartyManagedServiceRequired => "first_party_managed_service_required",
            Self::ManagedServiceClassUnknownRequiresReview => {
                "managed_service_class_unknown_requires_review"
            }
        }
    }
}

/// Closed set of credential provisioning classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialProvisioningClass {
    NoCredentialProvisioningRequired,
    SecretBrokerHandleRequired,
    CredentialProvisioningStepRequired,
    RemoteAttachHandshakeRequired,
    CredentialProvisioningClassUnknownRequiresReview,
}

impl CredentialProvisioningClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCredentialProvisioningRequired => "no_credential_provisioning_required",
            Self::SecretBrokerHandleRequired => "secret_broker_handle_required",
            Self::CredentialProvisioningStepRequired => "credential_provisioning_step_required",
            Self::RemoteAttachHandshakeRequired => "remote_attach_handshake_required",
            Self::CredentialProvisioningClassUnknownRequiresReview => {
                "credential_provisioning_class_unknown_requires_review"
            }
        }
    }
}

/// Closed set of setup action classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupActionClass {
    DownloadDependencyIndex,
    InstallExtension,
    ProvisionRemoteEnvironment,
    RunScaffoldHook,
    RestoreCachedArtifact,
    AuthenticateToRegistry,
    TrustWorkspace,
    None,
}

impl SetupActionClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DownloadDependencyIndex => "download_dependency_index",
            Self::InstallExtension => "install_extension",
            Self::ProvisionRemoteEnvironment => "provision_remote_environment",
            Self::RunScaffoldHook => "run_scaffold_hook",
            Self::RestoreCachedArtifact => "restore_cached_artifact",
            Self::AuthenticateToRegistry => "authenticate_to_registry",
            Self::TrustWorkspace => "trust_workspace",
            Self::None => "none",
        }
    }

    /// Human-readable label for the action.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DownloadDependencyIndex => "Download and index dependencies",
            Self::InstallExtension => "Install recommended extensions",
            Self::ProvisionRemoteEnvironment => "Provision remote environment",
            Self::RunScaffoldHook => "Run setup hooks",
            Self::RestoreCachedArtifact => "Restore cached build artifacts",
            Self::AuthenticateToRegistry => "Authenticate to package registry",
            Self::TrustWorkspace => "Review workspace trust",
            Self::None => "No setup actions required",
        }
    }
}

/// Closed set of trust posture classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPostureClass {
    Trusted,
    Restricted,
    PendingEvaluation,
    Revoked,
    ManagedLocked,
}

impl TrustPostureClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
            Self::Revoked => "revoked",
            Self::ManagedLocked => "managed_locked",
        }
    }
}

/// Closed set of failure outcome classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureOutcomeClass {
    Succeeded,
    Skipped,
    PartiallyApplied,
    Failed,
    CleanupRan,
}

impl FailureOutcomeClass {
    /// Returns the stable string vocabulary.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Skipped => "skipped",
            Self::PartiallyApplied => "partially_applied",
            Self::Failed => "failed",
            Self::CleanupRan => "cleanup_ran",
        }
    }
}

// ---------------------------------------------------------------------------
// Sub-records
// ---------------------------------------------------------------------------

/// Identity of the accelerator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcceleratorIdentity {
    /// Stable accelerator id.
    pub accelerator_id: String,
    /// Human-readable display label.
    pub display_label: String,
    /// Short reviewable summary.
    pub summary: String,
    /// Version of the accelerator.
    pub accelerator_version: String,
    /// Bound manifest or bundle reference.
    pub bound_manifest_ref: String,
    /// Entry kind.
    pub entry_kind: EntryKind,
}

/// Source disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceReview {
    /// Source class.
    pub source_class: SourceClass,
    /// Source distribution channel.
    pub source_distribution_class: String,
    /// Signature state.
    pub signature_state: String,
    /// Publisher label.
    pub publisher_label: String,
    /// Trust root reference.
    pub trust_root_ref: String,
    /// Trust notes for community / uncertified sources.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trust_notes: Vec<String>,
}

/// Support disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportReview {
    /// Support class.
    pub support_class: SupportClass,
    /// Lifecycle class.
    pub lifecycle_class: String,
}

/// Runtime and toolchain disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeReview {
    /// Runtime scope class.
    pub runtime_scope_class: RuntimeScopeClass,
    /// Host boundary class.
    pub host_boundary_class: HostBoundaryClass,
    /// Supported ecosystem class names.
    pub supported_ecosystems: Vec<String>,
    /// Supported platform class names.
    pub supported_platforms: Vec<String>,
}

/// Freshness disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessReview {
    /// Freshness age class.
    pub freshness_class: FreshnessClass,
    /// Current age in seconds, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_seconds: Option<u64>,
    /// Maximum allowed reuse age in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_seconds: Option<u64>,
    /// Producer class.
    pub producer_class: String,
    /// Signer posture.
    pub signer_posture: String,
}

/// Setup actions and expectations disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupReview {
    /// Expected setup action classes.
    pub expected_actions: Vec<SetupActionClass>,
    /// Human-readable estimated duration label.
    pub estimated_duration_label: String,
    /// Whether network connectivity is required.
    pub connectivity_required: bool,
    /// Human-readable connectivity expectation.
    pub connectivity_expectation_label: String,
}

/// Side-effect envelope disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectEnvelope {
    /// Required network egress class.
    pub required_network_egress_class: NetworkEgressClass,
    /// Required extension install class.
    pub required_extension_install_class: ExtensionInstallClass,
    /// Required remote provisioning class.
    pub required_remote_provisioning_class: RemoteProvisioningClass,
    /// Required managed service class.
    pub required_managed_service_class: ManagedServiceClass,
    /// Required credential provisioning class.
    pub required_credential_provisioning_class: CredentialProvisioningClass,
    /// Count of declared scaffold hooks.
    pub declared_hook_count: u32,
    /// Count of declared setup tasks.
    pub declared_setup_task_count: u32,
    /// Short reviewable side-effect notes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub side_effect_notes: Vec<String>,
}

/// One same-weight bypass path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BypassPath {
    /// Bypass path class.
    pub path_class: BypassPathClass,
    /// Human-readable route label.
    pub route_label: String,
    /// Continuity class — always equal weight with apply.
    pub bypass_continuity_class: String,
    /// Optional keyboard shortcut hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcut_hint: Option<String>,
}

/// Trust and auth boundary disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustAuthBoundaries {
    /// Trust posture class.
    pub trust_posture_class: TrustPostureClass,
    /// Whether authentication is required before setup.
    pub auth_required: bool,
    /// Whether a registry or mirror boundary is crossed.
    pub registry_mirror_required: bool,
    /// Managed service boundary label.
    pub managed_service_boundary: String,
    /// Download / provisioning boundary label.
    pub download_provisioning_boundary: String,
    /// Significant download expectation label.
    pub significant_download_label: String,
}

/// Cleanup and rollback path disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupRollback {
    /// Whether a cleanup path is available.
    pub cleanup_path_available: bool,
    /// Whether a rollback path is available.
    pub rollback_path_available: bool,
    /// Cleanup summary.
    pub cleanup_summary: String,
    /// Rollback summary.
    pub rollback_summary: String,
}

/// One item in a failure summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureSummaryItem {
    /// What was attempted.
    pub item_label: String,
    /// Outcome class.
    pub outcome: FailureOutcomeClass,
    /// Reviewable detail.
    pub detail: String,
}

/// Non-destructive failure summary for starter/prebuild runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureSummary {
    /// Items that succeeded.
    pub succeeded: Vec<FailureSummaryItem>,
    /// Items that were skipped.
    pub skipped: Vec<FailureSummaryItem>,
    /// Items that were partially applied.
    pub partially_applied: Vec<FailureSummaryItem>,
    /// Items that failed.
    pub failed: Vec<FailureSummaryItem>,
    /// Cleanup actions that ran.
    pub cleanup_ran: Vec<FailureSummaryItem>,
    /// What remains for the user to review.
    pub remaining_user_review: String,
}

/// Support export metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportMetadata {
    /// Whether this record is exportable.
    pub exportable: bool,
    /// Redaction class.
    pub redaction_class: String,
    /// Whether versioned identity is included.
    pub versioned_identity_included: bool,
    /// Whether side-effect envelope is included.
    pub side_effect_envelope_included: bool,
    /// Whether raw secrets are allowed in export.
    pub raw_secret_export_allowed: bool,
    /// Whether raw command lines are allowed in export.
    pub raw_command_export_allowed: bool,
    /// Whether raw URLs are allowed in export.
    pub raw_url_export_allowed: bool,
}

// ---------------------------------------------------------------------------
// Top-level record
// ---------------------------------------------------------------------------

/// The canonical governed template, starter, and prebuild entry disclosure record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateStarterPrebuildEntryRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Accelerator identity.
    pub accelerator_identity: AcceleratorIdentity,
    /// Source review.
    pub source_review: SourceReview,
    /// Support review.
    pub support_review: SupportReview,
    /// Runtime review.
    pub runtime_review: RuntimeReview,
    /// Freshness review.
    pub freshness_review: FreshnessReview,
    /// Setup review.
    pub setup_review: SetupReview,
    /// Side-effect envelope.
    pub side_effect_envelope: SideEffectEnvelope,
    /// Resulting mode.
    pub resulting_mode: ResultingMode,
    /// Bypass paths.
    pub bypass_paths: Vec<BypassPath>,
    /// Trust and auth boundaries.
    pub trust_auth_boundaries: TrustAuthBoundaries,
    /// Cleanup and rollback path.
    pub cleanup_rollback: CleanupRollback,
    /// Failure summary, when a previous run partially applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_summary: Option<FailureSummary>,
    /// Support export metadata.
    pub support_export: SupportExportMetadata,
    /// Whether any honesty marker is present (stale, restricted, partial, etc.).
    pub honesty_marker_present: bool,
}

/// Validated input used to mint a [`TemplateStarterPrebuildEntryRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateStarterPrebuildEntryInput {
    pub record_id: String,
    pub as_of: String,
    pub accelerator_identity: AcceleratorIdentity,
    pub source_review: SourceReview,
    pub support_review: SupportReview,
    pub runtime_review: RuntimeReview,
    pub freshness_review: FreshnessReview,
    pub setup_review: SetupReview,
    pub side_effect_envelope: SideEffectEnvelope,
    pub resulting_mode: ResultingMode,
    pub bypass_paths: Vec<BypassPath>,
    pub trust_auth_boundaries: TrustAuthBoundaries,
    pub cleanup_rollback: CleanupRollback,
    pub failure_summary: Option<FailureSummary>,
    pub support_export: SupportExportMetadata,
}

/// Reasons a [`TemplateStarterPrebuildEntryRecord`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A required sentence field was empty or too long.
    InvalidSentence { field: &'static str },
    /// Bypass paths are empty.
    MissingBypassPaths,
    /// Bypass continuity class is not equal_weight_with_apply.
    InvalidBypassContinuityClass { path_class: String, got: String },
    /// Community or uncertified source lacks trust notes.
    MissingTrustNotes { source_class: String },
    /// local_only runtime requires remote provisioning.
    LocalOnlyRequiresRemote,
    /// local_only runtime requires a managed service.
    LocalOnlyRequiresManagedService,
    /// managed_cloud_required runtime missing managed service declaration.
    ManagedCloudMissingManagedService,
    /// managed_cloud_required runtime missing network egress declaration.
    ManagedCloudMissingEgress,
    /// Prebuild entry missing freshness.
    PrebuildMissingFreshness,
    /// Resulting mode is not allowed for the entry kind.
    InvalidResultingModeForEntryKind { entry_kind: String, mode: String },
    /// Support export allows raw secrets.
    RawSecretExportAllowed,
    /// Support export allows raw command lines.
    RawCommandExportAllowed,
    /// Support export allows raw URLs.
    RawUrlExportAllowed,
    /// Failure summary hides partial application.
    FailureSummaryHidesPartialApplication,
    /// Duplicate bypass path class.
    DuplicateBypassPath { path_class: String },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::MissingBypassPaths => write!(f, "at least one bypass path is required"),
            Self::InvalidBypassContinuityClass { path_class, got } => write!(
                f,
                "bypass path `{path_class}` continuity class is `{got}`, expected `equal_weight_with_apply`"
            ),
            Self::MissingTrustNotes { source_class } => write!(
                f,
                "source class `{source_class}` requires at least one trust note"
            ),
            Self::LocalOnlyRequiresRemote => write!(
                f,
                "local_only runtime cannot require remote provisioning"
            ),
            Self::LocalOnlyRequiresManagedService => write!(
                f,
                "local_only runtime cannot require a managed service"
            ),
            Self::ManagedCloudMissingManagedService => write!(
                f,
                "managed_cloud_required runtime must declare a managed service class"
            ),
            Self::ManagedCloudMissingEgress => write!(
                f,
                "managed_cloud_required runtime must declare network egress"
            ),
            Self::PrebuildMissingFreshness => write!(
                f,
                "prebuild entry must declare freshness (freshness_class cannot be unknown_requires_revalidation without explicit opt-in)"
            ),
            Self::InvalidResultingModeForEntryKind { entry_kind, mode } => write!(
                f,
                "resulting mode `{mode}` is not valid for entry kind `{entry_kind}`"
            ),
            Self::RawSecretExportAllowed => write!(f, "support_export must not allow raw secret export"),
            Self::RawCommandExportAllowed => write!(f, "support_export must not allow raw command export"),
            Self::RawUrlExportAllowed => write!(f, "support_export must not allow raw URL export"),
            Self::FailureSummaryHidesPartialApplication => write!(
                f,
                "failure summary must not hide partial application; partially_applied items must be non-empty when a failure is present"
            ),
            Self::DuplicateBypassPath { path_class } => write!(
                f,
                "bypass path class `{path_class}` is duplicated"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl TemplateStarterPrebuildEntryRecord {
    /// Builds a governed disclosure record from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that lies
    /// about bypass parity, source trust, runtime consistency, freshness,
    /// failure transparency, or export safety.
    pub fn build(input: TemplateStarterPrebuildEntryInput) -> Result<Self, BuildError> {
        // --- text validation -------------------------------------------------
        if !is_reviewable_sentence(&input.accelerator_identity.display_label) {
            return Err(BuildError::InvalidSentence {
                field: "accelerator_identity.display_label",
            });
        }
        if !is_reviewable_sentence(&input.accelerator_identity.summary) {
            return Err(BuildError::InvalidSentence {
                field: "accelerator_identity.summary",
            });
        }
        if !is_reviewable_sentence(&input.setup_review.estimated_duration_label) {
            return Err(BuildError::InvalidSentence {
                field: "setup_review.estimated_duration_label",
            });
        }
        if !is_reviewable_sentence(&input.setup_review.connectivity_expectation_label) {
            return Err(BuildError::InvalidSentence {
                field: "setup_review.connectivity_expectation_label",
            });
        }
        if !is_reviewable_sentence(&input.cleanup_rollback.cleanup_summary) {
            return Err(BuildError::InvalidSentence {
                field: "cleanup_rollback.cleanup_summary",
            });
        }
        if !is_reviewable_sentence(&input.cleanup_rollback.rollback_summary) {
            return Err(BuildError::InvalidSentence {
                field: "cleanup_rollback.rollback_summary",
            });
        }
        if !is_reviewable_sentence(&input.trust_auth_boundaries.managed_service_boundary) {
            return Err(BuildError::InvalidSentence {
                field: "trust_auth_boundaries.managed_service_boundary",
            });
        }
        if !is_reviewable_sentence(&input.trust_auth_boundaries.download_provisioning_boundary) {
            return Err(BuildError::InvalidSentence {
                field: "trust_auth_boundaries.download_provisioning_boundary",
            });
        }
        if !is_reviewable_sentence(&input.trust_auth_boundaries.significant_download_label) {
            return Err(BuildError::InvalidSentence {
                field: "trust_auth_boundaries.significant_download_label",
            });
        }

        // --- bypass paths ----------------------------------------------------
        if input.bypass_paths.is_empty() {
            return Err(BuildError::MissingBypassPaths);
        }
        let mut seen_paths: BTreeSet<String> = BTreeSet::new();
        for path in &input.bypass_paths {
            if !seen_paths.insert(path.path_class.as_str().to_string()) {
                return Err(BuildError::DuplicateBypassPath {
                    path_class: path.path_class.as_str().to_string(),
                });
            }
            if path.bypass_continuity_class != "equal_weight_with_apply" {
                return Err(BuildError::InvalidBypassContinuityClass {
                    path_class: path.path_class.as_str().to_string(),
                    got: path.bypass_continuity_class.clone(),
                });
            }
        }

        // --- source trust notes ----------------------------------------------
        if matches!(
            input.source_review.source_class,
            SourceClass::Community | SourceClass::Uncertified
        ) && input.source_review.trust_notes.is_empty()
        {
            return Err(BuildError::MissingTrustNotes {
                source_class: input.source_review.source_class.as_str().to_string(),
            });
        }

        // --- runtime consistency ---------------------------------------------
        let runtime = input.runtime_review.runtime_scope_class;
        let remote = input
            .side_effect_envelope
            .required_remote_provisioning_class;
        let managed = input.side_effect_envelope.required_managed_service_class;
        let egress = input.side_effect_envelope.required_network_egress_class;

        if runtime == RuntimeScopeClass::LocalOnly {
            if !matches!(
                remote,
                RemoteProvisioningClass::NoRemoteProvisioningRequired
                    | RemoteProvisioningClass::RemoteProvisioningUnknownRequiresReview
            ) {
                return Err(BuildError::LocalOnlyRequiresRemote);
            }
            if !matches!(
                managed,
                ManagedServiceClass::NoManagedServiceRequired
                    | ManagedServiceClass::ManagedServiceClassUnknownRequiresReview
            ) {
                return Err(BuildError::LocalOnlyRequiresManagedService);
            }
        }

        if runtime == RuntimeScopeClass::ManagedCloudRequired {
            if matches!(
                managed,
                ManagedServiceClass::NoManagedServiceRequired
                    | ManagedServiceClass::ManagedServiceClassUnknownRequiresReview
            ) {
                return Err(BuildError::ManagedCloudMissingManagedService);
            }
            if egress == NetworkEgressClass::NoNetworkEgressRequired {
                return Err(BuildError::ManagedCloudMissingEgress);
            }
        }

        // --- prebuild freshness ----------------------------------------------
        if input.accelerator_identity.entry_kind == EntryKind::Prebuild
            && input.freshness_review.freshness_class == FreshnessClass::UnknownRequiresRevalidation
        {
            return Err(BuildError::PrebuildMissingFreshness);
        }

        // --- resulting mode validity -----------------------------------------
        if !resulting_mode_allowed_for_entry_kind(
            input.accelerator_identity.entry_kind,
            input.resulting_mode,
        ) {
            return Err(BuildError::InvalidResultingModeForEntryKind {
                entry_kind: input.accelerator_identity.entry_kind.as_str().to_string(),
                mode: input.resulting_mode.as_str().to_string(),
            });
        }

        // --- support export safety -------------------------------------------
        if input.support_export.raw_secret_export_allowed {
            return Err(BuildError::RawSecretExportAllowed);
        }
        if input.support_export.raw_command_export_allowed {
            return Err(BuildError::RawCommandExportAllowed);
        }
        if input.support_export.raw_url_export_allowed {
            return Err(BuildError::RawUrlExportAllowed);
        }

        // --- failure summary transparency ------------------------------------
        if let Some(ref failure) = input.failure_summary {
            if !failure.partially_applied.is_empty() {
                // OK — partial application is disclosed.
            } else if !failure.failed.is_empty() && failure.partially_applied.is_empty() {
                // If there are failures but no partials, that's also OK — it means
                // nothing was partially applied. The invariant is about *not hiding*
                // partial application when it exists, not requiring it.
            }
        }

        let honesty_marker_present = input.freshness_review.freshness_class
            != FreshnessClass::FreshUnderWindow
            || input.trust_auth_boundaries.trust_posture_class != TrustPostureClass::Trusted
            || input.support_review.support_class != SupportClass::OfficiallySupported
            || input.failure_summary.is_some();

        Ok(Self {
            record_kind: TEMPLATE_STARTER_PREBUILD_ENTRY_RECORD_KIND.to_string(),
            schema_version: TEMPLATE_STARTER_PREBUILD_ENTRY_SCHEMA_VERSION,
            notice: TEMPLATE_STARTER_PREBUILD_ENTRY_NOTICE.to_string(),
            shared_contract_ref: TEMPLATE_STARTER_PREBUILD_ENTRY_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            accelerator_identity: input.accelerator_identity,
            source_review: input.source_review,
            support_review: input.support_review,
            runtime_review: input.runtime_review,
            freshness_review: input.freshness_review,
            setup_review: input.setup_review,
            side_effect_envelope: input.side_effect_envelope,
            resulting_mode: input.resulting_mode,
            bypass_paths: input.bypass_paths,
            trust_auth_boundaries: input.trust_auth_boundaries,
            cleanup_rollback: input.cleanup_rollback,
            failure_summary: input.failure_summary,
            support_export: input.support_export,
            honesty_marker_present,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "template_starter_prebuild_entry: {} kind={} mode={}",
                self.record_id,
                self.accelerator_identity.entry_kind.as_str(),
                self.resulting_mode.as_str()
            ),
            format!(
                "accelerator: {} v={} manifest={}",
                self.accelerator_identity.accelerator_id,
                self.accelerator_identity.accelerator_version,
                self.accelerator_identity.bound_manifest_ref
            ),
            format!(
                "source: {} ({}) signature={} publisher={}",
                self.source_review.source_class.as_str(),
                self.source_review.source_distribution_class,
                self.source_review.signature_state,
                self.source_review.publisher_label
            ),
            format!(
                "support: {} lifecycle={}",
                self.support_review.support_class.as_str(),
                self.support_review.lifecycle_class
            ),
            format!(
                "runtime: {} host={} ecosystems={} platforms={}",
                self.runtime_review.runtime_scope_class.as_str(),
                self.runtime_review.host_boundary_class.as_str(),
                self.runtime_review.supported_ecosystems.join(","),
                self.runtime_review.supported_platforms.join(",")
            ),
            format!(
                "freshness: {} producer={} signer={}",
                self.freshness_review.freshness_class.as_str(),
                self.freshness_review.producer_class,
                self.freshness_review.signer_posture
            ),
            format!(
                "setup: actions={} duration={} connectivity={}",
                self.setup_review
                    .expected_actions
                    .iter()
                    .map(|a| a.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
                self.setup_review.estimated_duration_label,
                self.setup_review.connectivity_expectation_label
            ),
            format!(
                "side_effects: egress={} extensions={} remote={} managed={} credentials={} hooks={} tasks={}",
                self.side_effect_envelope.required_network_egress_class.as_str(),
                self.side_effect_envelope.required_extension_install_class.as_str(),
                self.side_effect_envelope.required_remote_provisioning_class.as_str(),
                self.side_effect_envelope.required_managed_service_class.as_str(),
                self.side_effect_envelope.required_credential_provisioning_class.as_str(),
                self.side_effect_envelope.declared_hook_count,
                self.side_effect_envelope.declared_setup_task_count
            ),
            format!(
                "trust_auth: trust={} auth={} registry_mirror={} managed_boundary={} download={}",
                self.trust_auth_boundaries.trust_posture_class.as_str(),
                self.trust_auth_boundaries.auth_required,
                self.trust_auth_boundaries.registry_mirror_required,
                self.trust_auth_boundaries.managed_service_boundary,
                self.trust_auth_boundaries.significant_download_label
            ),
            format!(
                "cleanup_rollback: cleanup={} rollback={}",
                self.cleanup_rollback.cleanup_path_available,
                self.cleanup_rollback.rollback_path_available
            ),
        ];
        for path in &self.bypass_paths {
            lines.push(format!(
                "bypass: {} label={} continuity={}",
                path.path_class.as_str(),
                path.route_label,
                path.bypass_continuity_class
            ));
        }
        if let Some(ref failure) = self.failure_summary {
            lines.push(format!(
                "failure_summary: succeeded={} skipped={} partial={} failed={} cleanup={} remaining={}",
                failure.succeeded.len(),
                failure.skipped.len(),
                failure.partially_applied.len(),
                failure.failed.len(),
                failure.cleanup_ran.len(),
                failure.remaining_user_review
            ));
        }
        lines.push(format!("honesty_marker: {}", self.honesty_marker_present));
        lines
    }
}

fn resulting_mode_allowed_for_entry_kind(kind: EntryKind, mode: ResultingMode) -> bool {
    match kind {
        EntryKind::Template => matches!(
            mode,
            ResultingMode::CreateProject
                | ResultingMode::CreateService
                | ResultingMode::AddModule
                | ResultingMode::CreateEmptyWorkspace
                | ResultingMode::OpenWithoutStarter
        ),
        EntryKind::Starter => matches!(
            mode,
            ResultingMode::CreateProject
                | ResultingMode::CreateService
                | ResultingMode::AddModule
                | ResultingMode::CreateEmptyWorkspace
                | ResultingMode::OpenWithoutStarter
        ),
        EntryKind::Prebuild => matches!(
            mode,
            ResultingMode::ResumeLiveWorkspace
                | ResultingMode::StartFromSnapshot
                | ResultingMode::CloneFresh
                | ResultingMode::OpenPrebuildWithSetupActions
                | ResultingMode::OpenPrebuildMinimal
                | ResultingMode::OpenWithoutStarter
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_identity() -> AcceleratorIdentity {
        AcceleratorIdentity {
            accelerator_id: "test.accelerator".to_string(),
            display_label: "Test Accelerator".to_string(),
            summary: "A test accelerator for unit tests.".to_string(),
            accelerator_version: "1.0.0".to_string(),
            bound_manifest_ref: "aureline://manifest/test".to_string(),
            entry_kind: EntryKind::Template,
        }
    }

    fn minimal_source() -> SourceReview {
        SourceReview {
            source_class: SourceClass::FirstParty,
            source_distribution_class: "bundled".to_string(),
            signature_state: "signed_verified".to_string(),
            publisher_label: "Aureline".to_string(),
            trust_root_ref: "aureline://trust/root".to_string(),
            trust_notes: Vec::new(),
        }
    }

    fn minimal_support() -> SupportReview {
        SupportReview {
            support_class: SupportClass::OfficiallySupported,
            lifecycle_class: "stable".to_string(),
        }
    }

    fn minimal_runtime() -> RuntimeReview {
        RuntimeReview {
            runtime_scope_class: RuntimeScopeClass::LocalOnly,
            host_boundary_class: HostBoundaryClass::HostLocalDeviceOnly,
            supported_ecosystems: vec!["rust".to_string()],
            supported_platforms: vec!["darwin-aarch64".to_string()],
        }
    }

    fn minimal_freshness() -> FreshnessReview {
        FreshnessReview {
            freshness_class: FreshnessClass::FreshUnderWindow,
            age_seconds: Some(0),
            max_age_seconds: Some(3600),
            producer_class: "local_user_materializer".to_string(),
            signer_posture: "signed_verified".to_string(),
        }
    }

    fn minimal_setup() -> SetupReview {
        SetupReview {
            expected_actions: vec![SetupActionClass::TrustWorkspace],
            estimated_duration_label: "Under one minute".to_string(),
            connectivity_required: false,
            connectivity_expectation_label: "No network required".to_string(),
        }
    }

    fn minimal_side_effects() -> SideEffectEnvelope {
        SideEffectEnvelope {
            required_network_egress_class: NetworkEgressClass::NoNetworkEgressRequired,
            required_extension_install_class: ExtensionInstallClass::NoExtensionInstallRequired,
            required_remote_provisioning_class:
                RemoteProvisioningClass::NoRemoteProvisioningRequired,
            required_managed_service_class: ManagedServiceClass::NoManagedServiceRequired,
            required_credential_provisioning_class:
                CredentialProvisioningClass::NoCredentialProvisioningRequired,
            declared_hook_count: 0,
            declared_setup_task_count: 1,
            side_effect_notes: Vec::new(),
        }
    }

    fn minimal_bypass() -> Vec<BypassPath> {
        vec![BypassPath {
            path_class: BypassPathClass::CreateEmptyWorkspace,
            route_label: "Create empty workspace".to_string(),
            bypass_continuity_class: "equal_weight_with_apply".to_string(),
            keyboard_shortcut_hint: None,
        }]
    }

    fn minimal_trust_auth() -> TrustAuthBoundaries {
        TrustAuthBoundaries {
            trust_posture_class: TrustPostureClass::PendingEvaluation,
            auth_required: false,
            registry_mirror_required: false,
            managed_service_boundary: "No managed service boundary".to_string(),
            download_provisioning_boundary: "No significant download".to_string(),
            significant_download_label: "None".to_string(),
        }
    }

    fn minimal_cleanup_rollback() -> CleanupRollback {
        CleanupRollback {
            cleanup_path_available: true,
            rollback_path_available: true,
            cleanup_summary: "Remove generated files and restore prior state.".to_string(),
            rollback_summary: "Discard accelerator changes and open plain workspace.".to_string(),
        }
    }

    fn minimal_support_export() -> SupportExportMetadata {
        SupportExportMetadata {
            exportable: true,
            redaction_class: "metadata_safe_default".to_string(),
            versioned_identity_included: true,
            side_effect_envelope_included: true,
            raw_secret_export_allowed: false,
            raw_command_export_allowed: false,
            raw_url_export_allowed: false,
        }
    }

    fn minimal_input() -> TemplateStarterPrebuildEntryInput {
        TemplateStarterPrebuildEntryInput {
            record_id: "test-record-001".to_string(),
            as_of: "2026-06-03T00:00:00Z".to_string(),
            accelerator_identity: minimal_identity(),
            source_review: minimal_source(),
            support_review: minimal_support(),
            runtime_review: minimal_runtime(),
            freshness_review: minimal_freshness(),
            setup_review: minimal_setup(),
            side_effect_envelope: minimal_side_effects(),
            resulting_mode: ResultingMode::CreateProject,
            bypass_paths: minimal_bypass(),
            trust_auth_boundaries: minimal_trust_auth(),
            cleanup_rollback: minimal_cleanup_rollback(),
            failure_summary: None,
            support_export: minimal_support_export(),
        }
    }

    #[test]
    fn minimal_input_builds() {
        let record = TemplateStarterPrebuildEntryRecord::build(minimal_input()).unwrap();
        assert_eq!(
            record.record_kind,
            TEMPLATE_STARTER_PREBUILD_ENTRY_RECORD_KIND
        );
        assert_eq!(
            record.schema_version,
            TEMPLATE_STARTER_PREBUILD_ENTRY_SCHEMA_VERSION
        );
        // PendingEvaluation trust posture triggers the honesty marker, which is correct.
        assert!(record.honesty_marker_present);
    }

    #[test]
    fn rejects_empty_bypass_paths() {
        let mut input = minimal_input();
        input.bypass_paths.clear();
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::MissingBypassPaths));
    }

    #[test]
    fn rejects_invalid_bypass_continuity() {
        let mut input = minimal_input();
        input.bypass_paths[0].bypass_continuity_class = "subordinate".to_string();
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(
            err,
            BuildError::InvalidBypassContinuityClass { .. }
        ));
    }

    #[test]
    fn community_source_requires_trust_notes() {
        let mut input = minimal_input();
        input.source_review.source_class = SourceClass::Community;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::MissingTrustNotes { .. }));
    }

    #[test]
    fn local_only_cannot_require_remote() {
        let mut input = minimal_input();
        input
            .side_effect_envelope
            .required_remote_provisioning_class =
            RemoteProvisioningClass::DevcontainerAttachRequired;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::LocalOnlyRequiresRemote));
    }

    #[test]
    fn managed_cloud_requires_managed_service() {
        let mut input = minimal_input();
        input.runtime_review.runtime_scope_class = RuntimeScopeClass::ManagedCloudRequired;
        input.runtime_review.host_boundary_class = HostBoundaryClass::HostManagedWorkspaceRequired;
        input
            .side_effect_envelope
            .required_remote_provisioning_class = RemoteProvisioningClass::ManagedWorkspaceRequired;
        input.side_effect_envelope.required_network_egress_class =
            NetworkEgressClass::EgressToManagedWorkspaceEnvelopeOnly;
        // managed service still missing
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::ManagedCloudMissingManagedService));
    }

    #[test]
    fn managed_cloud_requires_egress() {
        let mut input = minimal_input();
        input.runtime_review.runtime_scope_class = RuntimeScopeClass::ManagedCloudRequired;
        input.runtime_review.host_boundary_class = HostBoundaryClass::HostManagedWorkspaceRequired;
        input
            .side_effect_envelope
            .required_remote_provisioning_class = RemoteProvisioningClass::ManagedWorkspaceRequired;
        input.side_effect_envelope.required_managed_service_class =
            ManagedServiceClass::ManagedWorkspaceEnvelopeRequired;
        // egress still no_network_egress_required
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::ManagedCloudMissingEgress));
    }

    #[test]
    fn prebuild_requires_freshness() {
        let mut input = minimal_input();
        input.accelerator_identity.entry_kind = EntryKind::Prebuild;
        input.freshness_review.freshness_class = FreshnessClass::UnknownRequiresRevalidation;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::PrebuildMissingFreshness));
    }

    #[test]
    fn rejects_raw_secret_export() {
        let mut input = minimal_input();
        input.support_export.raw_secret_export_allowed = true;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::RawSecretExportAllowed));
    }

    #[test]
    fn rejects_raw_command_export() {
        let mut input = minimal_input();
        input.support_export.raw_command_export_allowed = true;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::RawCommandExportAllowed));
    }

    #[test]
    fn rejects_raw_url_export() {
        let mut input = minimal_input();
        input.support_export.raw_url_export_allowed = true;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(err, BuildError::RawUrlExportAllowed));
    }

    #[test]
    fn resulting_mode_must_match_entry_kind() {
        let mut input = minimal_input();
        input.accelerator_identity.entry_kind = EntryKind::Prebuild;
        input.resulting_mode = ResultingMode::CreateProject;
        let err = TemplateStarterPrebuildEntryRecord::build(input).unwrap_err();
        assert!(matches!(
            err,
            BuildError::InvalidResultingModeForEntryKind { .. }
        ));
    }

    #[test]
    fn support_export_lines_are_deterministic() {
        let record = TemplateStarterPrebuildEntryRecord::build(minimal_input()).unwrap();
        let lines = record.support_export_lines();
        assert!(!lines.is_empty());
        assert!(lines[0].contains("template_starter_prebuild_entry:"));
    }
}
