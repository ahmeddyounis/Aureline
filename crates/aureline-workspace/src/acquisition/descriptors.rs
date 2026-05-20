//! Boundary records for the repository-acquisition beta lane.
//!
//! Every record mirrors the matching seed schema in `schemas/workspace/`:
//!
//! - [`SourceLocatorRecord`] — `source_locator.schema.json`
//! - [`CheckoutPlanRecord`] — `checkout_plan.schema.json`
//! - [`BootstrapQueueItemRecord`] — `bootstrap_queue_item.schema.json`
//!
//! The records carry only opaque refs and typed labels. Raw absolute
//! paths, raw credentials, raw remote URLs with embedded credentials, raw
//! archive bytes, and raw policy-bundle bytes never appear. The closed
//! vocabulary is frozen by
//! `docs/workspace/source_acquisition_and_bootstrap_seed.md`; adding a new
//! enum value is additive-minor, repurposing one is breaking.

use serde::{Deserialize, Serialize};

use super::shared::FixtureMetadata;

pub const SOURCE_LOCATOR_SCHEMA_VERSION: u32 = 1;
pub const CHECKOUT_PLAN_SCHEMA_VERSION: u32 = 1;
pub const BOOTSTRAP_QUEUE_ITEM_SCHEMA_VERSION: u32 = 1;

pub const SOURCE_LOCATOR_RECORD_KIND: &str = "source_locator_record";
pub const CHECKOUT_PLAN_RECORD_KIND: &str = "checkout_plan_record";
pub const BOOTSTRAP_QUEUE_ITEM_RECORD_KIND: &str = "bootstrap_queue_item_record";

// ----------------------------------------------------------------------
// SourceLocatorRecord
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLocatorRecordKind {
    SourceLocatorRecord,
}

/// Closed locator-class set. Names the kind of source being acquired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocatorClass {
    LocalFolder,
    LocalFile,
    WorkspaceFileManifest,
    WorksetManifest,
    RepoUrl,
    MirrorOrProxyRepo,
    SnapshotArchive,
    Template,
    PrebuildSnapshot,
    LiveResumeTarget,
    HandoffPacket,
    PortableStatePackage,
    RecoveryCheckpoint,
    ReviewOrWorkItemDeepLink,
}

impl LocatorClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFolder => "local_folder",
            Self::LocalFile => "local_file",
            Self::WorkspaceFileManifest => "workspace_file_manifest",
            Self::WorksetManifest => "workset_manifest",
            Self::RepoUrl => "repo_url",
            Self::MirrorOrProxyRepo => "mirror_or_proxy_repo",
            Self::SnapshotArchive => "snapshot_archive",
            Self::Template => "template",
            Self::PrebuildSnapshot => "prebuild_snapshot",
            Self::LiveResumeTarget => "live_resume_target",
            Self::HandoffPacket => "handoff_packet",
            Self::PortableStatePackage => "portable_state_package",
            Self::RecoveryCheckpoint => "recovery_checkpoint",
            Self::ReviewOrWorkItemDeepLink => "review_or_work_item_deep_link",
        }
    }

    /// True when the source is materialized purely from the local
    /// filesystem and never rides through a remote / mirror / archive
    /// transport on acquisition.
    pub const fn is_local_only(self) -> bool {
        matches!(
            self,
            Self::LocalFolder
                | Self::LocalFile
                | Self::WorkspaceFileManifest
                | Self::WorksetManifest
                | Self::RecoveryCheckpoint
        )
    }
}

/// Closed transport-class set. Names how the source is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportClass {
    LocalFilesystem,
    Https,
    Ssh,
    GitProtocol,
    Sftp,
    Mirror,
    Proxy,
    AirGappedMedia,
    AurelineRemote,
    ManagedCloud,
    DevcontainerRuntime,
    ContainerRuntime,
    FileUpload,
    DeepLinkHandoff,
    Other,
}

impl TransportClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFilesystem => "local_filesystem",
            Self::Https => "https",
            Self::Ssh => "ssh",
            Self::GitProtocol => "git_protocol",
            Self::Sftp => "sftp",
            Self::Mirror => "mirror",
            Self::Proxy => "proxy",
            Self::AirGappedMedia => "air_gapped_media",
            Self::AurelineRemote => "aureline_remote",
            Self::ManagedCloud => "managed_cloud",
            Self::DevcontainerRuntime => "devcontainer_runtime",
            Self::ContainerRuntime => "container_runtime",
            Self::FileUpload => "file_upload",
            Self::DeepLinkHandoff => "deep_link_handoff",
            Self::Other => "other",
        }
    }

    /// True when the transport pulls bytes over a network on acquisition.
    pub const fn is_network_bearing(self) -> bool {
        matches!(
            self,
            Self::Https
                | Self::Ssh
                | Self::GitProtocol
                | Self::Sftp
                | Self::Mirror
                | Self::Proxy
                | Self::AurelineRemote
                | Self::ManagedCloud
        )
    }

    /// True when the transport rides through a mirror or proxy and must
    /// not masquerade as a live upstream fetch.
    pub const fn is_mirror_or_proxy(self) -> bool {
        matches!(self, Self::Mirror | Self::Proxy)
    }
}

/// Closed auth-mode set. Typed handle class only; raw credentials never
/// cross this boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthModeClass {
    SshAgent,
    PatHandle,
    OauthHandle,
    DeviceCodeHandle,
    ManagedSessionTicket,
    ConnectedProviderTicket,
    Anonymous,
    InheritLocalIdentity,
    None,
    Other,
}

impl AuthModeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SshAgent => "ssh_agent",
            Self::PatHandle => "pat_handle",
            Self::OauthHandle => "oauth_handle",
            Self::DeviceCodeHandle => "device_code_handle",
            Self::ManagedSessionTicket => "managed_session_ticket",
            Self::ConnectedProviderTicket => "connected_provider_ticket",
            Self::Anonymous => "anonymous",
            Self::InheritLocalIdentity => "inherit_local_identity",
            Self::None => "none",
            Self::Other => "other",
        }
    }
}

/// Closed acquisition-posture set. Names how materialized the source is at
/// locator-mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionPosture {
    AlreadyOnDisk,
    NotYetAcquired,
    PartiallyAcquired,
    FilteredOrSparse,
    HydratedStale,
    LiveSessionAttached,
    Unreachable,
    PolicyBlocked,
    Unknown,
}

impl AcquisitionPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlreadyOnDisk => "already_on_disk",
            Self::NotYetAcquired => "not_yet_acquired",
            Self::PartiallyAcquired => "partially_acquired",
            Self::FilteredOrSparse => "filtered_or_sparse",
            Self::HydratedStale => "hydrated_stale",
            Self::LiveSessionAttached => "live_session_attached",
            Self::Unreachable => "unreachable",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed declared-freshness set the locator carries forward for
/// downstream checkout and bootstrap review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeclaredFreshnessClass {
    LiveOrigin,
    MirrorFresh,
    MirrorLagged,
    MirrorStale,
    OfflineSnapshot,
    SignedOfflineBundle,
    UnknownFreshness,
}

impl DeclaredFreshnessClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveOrigin => "live_origin",
            Self::MirrorFresh => "mirror_fresh",
            Self::MirrorLagged => "mirror_lagged",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineSnapshot => "offline_snapshot",
            Self::SignedOfflineBundle => "signed_offline_bundle",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }

    /// True when the source is not a live upstream and a surface that
    /// renders it as live origin would be lying.
    pub const fn is_not_live_origin(self) -> bool {
        !matches!(self, Self::LiveOrigin)
    }

    /// True when the freshness class is a lagged or stale mirror that MUST
    /// be rendered distinctly from a fresh mirror.
    pub const fn is_lagged_or_stale(self) -> bool {
        matches!(self, Self::MirrorLagged | Self::MirrorStale)
    }
}

/// Closed signer-continuity set. Names whether the producer / mirror /
/// bundle signer identity is continuous with what was previously accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerContinuityClass {
    ContinuousWithPreviousAcquisition,
    NewSignerFirstSeen,
    SignerChangedTrustOnFirstUse,
    SignerChangedReviewRequired,
    SignerRotationPreauthorized,
    Unsigned,
    SignatureMissing,
    SignatureMismatch,
    NotApplicable,
}

impl SignerContinuityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuousWithPreviousAcquisition => "continuous_with_previous_acquisition",
            Self::NewSignerFirstSeen => "new_signer_first_seen",
            Self::SignerChangedTrustOnFirstUse => "signer_changed_trust_on_first_use",
            Self::SignerChangedReviewRequired => "signer_changed_review_required",
            Self::SignerRotationPreauthorized => "signer_rotation_preauthorized",
            Self::Unsigned => "unsigned",
            Self::SignatureMissing => "signature_missing",
            Self::SignatureMismatch => "signature_mismatch",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the signer change MUST route to a typed review hook before
    /// trust admission.
    pub const fn requires_signer_review(self) -> bool {
        matches!(
            self,
            Self::SignerChangedReviewRequired | Self::SignatureMismatch
        )
    }
}

/// Entry-verb hint re-exported from the entry-and-restore vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocatorEntryVerbHint {
    Open,
    Clone,
    Import,
    AddRoot,
    Restore,
    Resume,
    StartFromSnapshot,
}

/// Target-kind hint re-exported from the entry-and-restore vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocatorTargetKindHint {
    LocalFile,
    LocalFolder,
    LocalRepoRoot,
    WorkspaceManifest,
    WorksetManifest,
    RemoteRepository,
    SshWorkspace,
    ContainerWorkspace,
    DevcontainerWorkspace,
    ManagedCloudWorkspace,
    PortableStatePackage,
    HandoffPacket,
    CompetitorConfigRoot,
    TemplateOrPrebuildSnapshot,
    ReviewOrWorkItemDeepLink,
    RecoveryCheckpoint,
}

/// Typed descriptor for a remote / mirror / proxy / air-gapped endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostEndpointDescriptor {
    pub host_label: String,
    pub transport_class: TransportClass,
    pub auth_mode: AuthModeClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_handle_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_or_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_origin_label: Option<String>,
}

/// Closed artifact-class set for a transferred / portable / template /
/// snapshot locator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocatorArtifactClass {
    SnapshotArchive,
    Template,
    PrebuildSnapshot,
    HandoffPacket,
    PortableStatePackage,
    CompetitorConfigRoot,
    RecoveryCheckpoint,
    SupportBundleReplay,
}

/// Closed artifact signature-state set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactSignatureState {
    SignedVerified,
    SignedUnverified,
    Unsigned,
    SignatureMissing,
    SignatureMismatch,
}

/// Typed artifact descriptor. Raw bytes never appear.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDescriptor {
    pub artifact_class: LocatorArtifactClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_schema_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_identity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_build: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_state: Option<ArtifactSignatureState>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub machine_local_exclusions: Vec<String>,
}

/// Closed live-session class set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveSessionClass {
    ManagedCloudWorkspaceSession,
    RemoteAgentSession,
    DevcontainerSession,
    ContainerSession,
    SharedControlSession,
    NotebookKernelSession,
    CompanionHandoffSession,
}

/// Closed attach-authority class set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachAuthorityClass {
    AuthorityLive,
    AuthorityExpired,
    AuthorityRevoked,
    AuthorityPendingReauth,
    AuthorityNeverEstablished,
}

impl AttachAuthorityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityLive => "authority_live",
            Self::AuthorityExpired => "authority_expired",
            Self::AuthorityRevoked => "authority_revoked",
            Self::AuthorityPendingReauth => "authority_pending_reauth",
            Self::AuthorityNeverEstablished => "authority_never_established",
        }
    }

    /// True when the attach authority is not live and a surface that
    /// claims a live session would be lying.
    pub const fn requires_reauth(self) -> bool {
        matches!(
            self,
            Self::AuthorityExpired | Self::AuthorityRevoked | Self::AuthorityPendingReauth
        )
    }
}

/// Typed descriptor for a live-resume locator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveSessionDescriptor {
    pub session_class: LiveSessionClass,
    pub attach_authority_class: AttachAuthorityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_handle_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_endpoint: Option<HostEndpointDescriptor>,
}

/// Closed deep-link class set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkClass {
    ReviewDeepLink,
    WorkItemDeepLink,
    IncidentDeepLink,
    NotificationReopen,
    AuthCallbackReopen,
    Other,
}

/// Typed descriptor for a deep link that resolves to a concrete locator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkDescriptor {
    pub origin: String,
    pub link_class: DeepLinkClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_object_id: Option<String>,
}

/// One source-locator record. Mirrors `source_locator.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocatorRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,

    pub record_kind: SourceLocatorRecordKind,
    pub source_locator_schema_version: u32,
    pub source_locator_id: String,

    pub locator_class: LocatorClass,
    pub acquisition_posture: AcquisitionPosture,
    pub declared_freshness_class: DeclaredFreshnessClass,
    pub signer_continuity_class: SignerContinuityClass,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_identity_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_endpoint: Option<HostEndpointDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact_descriptor: Option<ArtifactDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_session_descriptor: Option<LiveSessionDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link_descriptor: Option<DeepLinkDescriptor>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_verb_hint: Option<LocatorEntryVerbHint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_kind_hint: Option<LocatorTargetKindHint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_acquisition_ref: Option<String>,

    pub observed_at: String,
}

// ----------------------------------------------------------------------
// CheckoutPlanRecord
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutPlanRecordKind {
    CheckoutPlanRecord,
}

/// Trust-state vocabulary re-exported unchanged from the entry-and-restore
/// schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutTrustState {
    Trusted,
    Restricted,
    PendingEvaluation,
}

impl CheckoutTrustState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
            Self::PendingEvaluation => "pending_evaluation",
        }
    }
}

/// Closed trust-stage set the checkout plan moves through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutTrustStage {
    NotEvaluated,
    PreFetchInspection,
    PostFetchContentReview,
    AdmittedRestricted,
    AdmittedTrusted,
    Quarantined,
    PolicyBlocked,
    SignerReviewRequired,
    ReauthRequired,
    ReconnectRequired,
}

impl CheckoutTrustStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotEvaluated => "not_evaluated",
            Self::PreFetchInspection => "pre_fetch_inspection",
            Self::PostFetchContentReview => "post_fetch_content_review",
            Self::AdmittedRestricted => "admitted_restricted",
            Self::AdmittedTrusted => "admitted_trusted",
            Self::Quarantined => "quarantined",
            Self::PolicyBlocked => "policy_blocked",
            Self::SignerReviewRequired => "signer_review_required",
            Self::ReauthRequired => "reauth_required",
            Self::ReconnectRequired => "reconnect_required",
        }
    }

    /// True once trust is admitted (restricted or trusted) and the plan may
    /// begin narrowing the blocked-execution-path list.
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::AdmittedRestricted | Self::AdmittedTrusted)
    }

    /// True while the plan permits only browse-safe inspection and every
    /// execution path remains blocked.
    pub const fn is_pre_admission_inspection(self) -> bool {
        matches!(
            self,
            Self::NotEvaluated | Self::PreFetchInspection | Self::PostFetchContentReview
        )
    }
}

/// Closed browse-safe action set the plan advertises during trust review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowseSafeActionClass {
    InspectManifest,
    InspectHistory,
    InspectContributors,
    InspectReadme,
    InspectLicense,
    InspectWorkflowsCiConfig,
    InspectDependencyManifest,
    InspectScriptsCatalog,
    InspectEntryPoints,
    DiffBeforeCommit,
    RenderDiffReadonly,
    ShowSignerIdentity,
    ShowMirrorMetadata,
    ShowUpstreamDelta,
    ShowPartialOrSparseScope,
    ExportEvidenceOnly,
}

/// Closed blocked-execution-path set. Each entry names a concrete
/// execution path the plan refuses until trust is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedExecutionPathClass {
    PostCheckoutHook,
    PreCommitHook,
    GitFilterDriver,
    LfsSmudgeFilter,
    SubmoduleRecursiveInit,
    SubmoduleUpdateCommand,
    GeneratorInstall,
    GeneratorRun,
    PackageManagerRestore,
    PackageManagerPostinstallScript,
    ToolchainBootstrap,
    DevcontainerBootstrap,
    PrebuildAttachSideEffects,
    RemoteAttachHandshake,
    IndexWarmupWithWorkers,
    DocsImportFetch,
    AiContextWarmup,
    ExtensionActivation,
    WorkspaceAutoTask,
    DeepLinkSideEffect,
}

impl BlockedExecutionPathClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostCheckoutHook => "post_checkout_hook",
            Self::PreCommitHook => "pre_commit_hook",
            Self::GitFilterDriver => "git_filter_driver",
            Self::LfsSmudgeFilter => "lfs_smudge_filter",
            Self::SubmoduleRecursiveInit => "submodule_recursive_init",
            Self::SubmoduleUpdateCommand => "submodule_update_command",
            Self::GeneratorInstall => "generator_install",
            Self::GeneratorRun => "generator_run",
            Self::PackageManagerRestore => "package_manager_restore",
            Self::PackageManagerPostinstallScript => "package_manager_postinstall_script",
            Self::ToolchainBootstrap => "toolchain_bootstrap",
            Self::DevcontainerBootstrap => "devcontainer_bootstrap",
            Self::PrebuildAttachSideEffects => "prebuild_attach_side_effects",
            Self::RemoteAttachHandshake => "remote_attach_handshake",
            Self::IndexWarmupWithWorkers => "index_warmup_with_workers",
            Self::DocsImportFetch => "docs_import_fetch",
            Self::AiContextWarmup => "ai_context_warmup",
            Self::ExtensionActivation => "extension_activation",
            Self::WorkspaceAutoTask => "workspace_auto_task",
            Self::DeepLinkSideEffect => "deep_link_side_effect",
        }
    }

    /// True when the path runs repository-owned code (hooks, filters,
    /// generators, package scripts) that MUST never run implicitly during
    /// acquisition.
    pub const fn is_repo_owned_code(self) -> bool {
        matches!(
            self,
            Self::PostCheckoutHook
                | Self::PreCommitHook
                | Self::GitFilterDriver
                | Self::LfsSmudgeFilter
                | Self::GeneratorInstall
                | Self::GeneratorRun
                | Self::PackageManagerPostinstallScript
                | Self::WorkspaceAutoTask
        )
    }
}

/// Closed acquisition-resume-state set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionResumeState {
    NeverStarted,
    InProgress,
    InterruptedResumable,
    InterruptedDiscardRequired,
    InterruptedOpenReadOnlyAvailable,
    Completed,
    Aborted,
}

impl AcquisitionResumeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverStarted => "never_started",
            Self::InProgress => "in_progress",
            Self::InterruptedResumable => "interrupted_resumable",
            Self::InterruptedDiscardRequired => "interrupted_discard_required",
            Self::InterruptedOpenReadOnlyAvailable => "interrupted_open_read_only_available",
            Self::Completed => "completed",
            Self::Aborted => "aborted",
        }
    }

    /// True for any of the explicit interrupted branches.
    pub const fn is_interrupted(self) -> bool {
        matches!(
            self,
            Self::InterruptedResumable
                | Self::InterruptedDiscardRequired
                | Self::InterruptedOpenReadOnlyAvailable
        )
    }
}

/// Closed discard-posture set. Names what `discard` costs for an
/// interrupted acquisition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscardPosture {
    NoDiscardRequired,
    DiscardStagingOnly,
    DiscardWithCompensation,
    DiscardBlockedRequireAdmin,
    DiscardUnavailableManualOnly,
}

impl DiscardPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDiscardRequired => "no_discard_required",
            Self::DiscardStagingOnly => "discard_staging_only",
            Self::DiscardWithCompensation => "discard_with_compensation",
            Self::DiscardBlockedRequireAdmin => "discard_blocked_require_admin",
            Self::DiscardUnavailableManualOnly => "discard_unavailable_manual_only",
        }
    }
}

/// Closed acquisition failure-reason set carried by an interrupted plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionFailureReasonClass {
    NetworkInterruption,
    AuthExpired,
    AuthorityRevoked,
    SignerMismatch,
    MirrorUnreachable,
    DiskPressure,
    PolicyBlocked,
    UserCancelled,
    SchemaVersionUnsupported,
    Unknown,
}

impl AcquisitionFailureReasonClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkInterruption => "network_interruption",
            Self::AuthExpired => "auth_expired",
            Self::AuthorityRevoked => "authority_revoked",
            Self::SignerMismatch => "signer_mismatch",
            Self::MirrorUnreachable => "mirror_unreachable",
            Self::DiskPressure => "disk_pressure",
            Self::PolicyBlocked => "policy_blocked",
            Self::UserCancelled => "user_cancelled",
            Self::SchemaVersionUnsupported => "schema_version_unsupported",
            Self::Unknown => "unknown",
        }
    }
}

/// Typed resumable-acquisition block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumableAcquisitionState {
    pub resume_state: AcquisitionResumeState,
    pub discard_posture: DiscardPosture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_checkpoint_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_read_only_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason_class: Option<AcquisitionFailureReasonClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_visible_reason: Option<String>,
}

/// Closed mirror-freshness class. Re-exported from the source-locator
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorFreshnessClass {
    LiveOrigin,
    MirrorFresh,
    MirrorLagged,
    MirrorStale,
    OfflineSnapshot,
    SignedOfflineBundle,
    UnknownFreshness,
}

/// Closed upstream-delta class. Names how far the mirror is from upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpstreamDeltaClass {
    NoDelta,
    DeltaWithinDeclaredSkew,
    DeltaOutsideDeclaredSkew,
    DeltaUnmeasured,
    DeltaUnknown,
}

impl UpstreamDeltaClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDelta => "no_delta",
            Self::DeltaWithinDeclaredSkew => "delta_within_declared_skew",
            Self::DeltaOutsideDeclaredSkew => "delta_outside_declared_skew",
            Self::DeltaUnmeasured => "delta_unmeasured",
            Self::DeltaUnknown => "delta_unknown",
        }
    }
}

/// Typed mirror / proxy freshness evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorFreshnessEvidence {
    pub freshness_class: MirrorFreshnessClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_delta_class: Option<UpstreamDeltaClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_delta_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_attestation_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_origin_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_at: Option<String>,
}

/// Typed signer-continuity evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignerContinuityEvidence {
    pub continuity_class: SignerContinuityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_identity_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_signer_identity_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_acquisition_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_policy_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_ticket_ref: Option<String>,
}

/// Closed read-only partial-root class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadOnlyPartialRootClass {
    SparseCheckoutRoot,
    PartialCloneFilterRoot,
    PromisorPackRoot,
    ShallowHistoryRoot,
    LfsPointerOnlyRoot,
    SubmodulePlaceholderRoot,
    ArchiveExtractedReadOnly,
    TemplateMaterializedReadOnly,
    PrebuildAttachedReadOnly,
    RemoteVirtualRoot,
    PortableBundleExtractedReadOnly,
}

impl ReadOnlyPartialRootClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SparseCheckoutRoot => "sparse_checkout_root",
            Self::PartialCloneFilterRoot => "partial_clone_filter_root",
            Self::PromisorPackRoot => "promisor_pack_root",
            Self::ShallowHistoryRoot => "shallow_history_root",
            Self::LfsPointerOnlyRoot => "lfs_pointer_only_root",
            Self::SubmodulePlaceholderRoot => "submodule_placeholder_root",
            Self::ArchiveExtractedReadOnly => "archive_extracted_read_only",
            Self::TemplateMaterializedReadOnly => "template_materialized_read_only",
            Self::PrebuildAttachedReadOnly => "prebuild_attached_read_only",
            Self::RemoteVirtualRoot => "remote_virtual_root",
            Self::PortableBundleExtractedReadOnly => "portable_bundle_extracted_read_only",
        }
    }
}

/// One read-only partial root the plan has already made safe to inspect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadOnlyPartialRoot {
    pub root_class: ReadOnlyPartialRootClass,
    pub root_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_identity_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_glob_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_glob_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browse_safe_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Closed seed-level topology-marker class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyMarkerClass {
    SparseWorksetPresent,
    PartialCloneFilterPresent,
    PromisorRemoteRequired,
    ShallowHistoryPresent,
    SubmoduleInitPending,
    SubmoduleInitPartial,
    SubmoduleInitComplete,
    SubmoduleInitFailed,
    LfsPointerOnly,
    LfsHydratePending,
    LfsHydratePartial,
    LfsHydrateComplete,
    LfsHydrateFailed,
    None,
}

impl TopologyMarkerClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SparseWorksetPresent => "sparse_workset_present",
            Self::PartialCloneFilterPresent => "partial_clone_filter_present",
            Self::PromisorRemoteRequired => "promisor_remote_required",
            Self::ShallowHistoryPresent => "shallow_history_present",
            Self::SubmoduleInitPending => "submodule_init_pending",
            Self::SubmoduleInitPartial => "submodule_init_partial",
            Self::SubmoduleInitComplete => "submodule_init_complete",
            Self::SubmoduleInitFailed => "submodule_init_failed",
            Self::LfsPointerOnly => "lfs_pointer_only",
            Self::LfsHydratePending => "lfs_hydrate_pending",
            Self::LfsHydratePartial => "lfs_hydrate_partial",
            Self::LfsHydrateComplete => "lfs_hydrate_complete",
            Self::LfsHydrateFailed => "lfs_hydrate_failed",
            Self::None => "none",
        }
    }
}

/// One seed-level topology-marker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyMarker {
    pub marker_class: TopologyMarkerClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// Closed policy-source class for a policy-narrowing reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySourceClass {
    WorkspaceTrustPolicy,
    AdminPolicy,
    FleetPolicy,
    ConnectedProviderPolicy,
    ExtensionEffectivePermission,
    SignaturePolicy,
    MirrorOrAirgapPolicy,
}

impl PolicySourceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTrustPolicy => "workspace_trust_policy",
            Self::AdminPolicy => "admin_policy",
            Self::FleetPolicy => "fleet_policy",
            Self::ConnectedProviderPolicy => "connected_provider_policy",
            Self::ExtensionEffectivePermission => "extension_effective_permission",
            Self::SignaturePolicy => "signature_policy",
            Self::MirrorOrAirgapPolicy => "mirror_or_airgap_policy",
        }
    }
}

/// Typed policy-narrowing reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyNarrowingRef {
    pub policy_source_class: PolicySourceClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_bundle_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub narrowed_execution_paths: Vec<BlockedExecutionPathClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale_label: Option<String>,
}

/// Closed next-step decision hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NextStepDecisionHook {
    ContinueInRestrictedMode,
    ReviewTrustAndOpen,
    OpenMinimal,
    SetUpLater,
    ReviewArchetypeMatch,
    CompareBeforeRestore,
    OpenWithoutRestore,
    SafeMode,
    ReconnectRequired,
    ReauthRequired,
    LocateMissingTarget,
    RemoveFromRecents,
    ReviewMigrationReport,
    RollBackImport,
    KeepImportedState,
    AdoptRecommendedBundle,
    ReviewUnsupportedItems,
    ResumeAcquisition,
    DiscardAndRestart,
    OpenReadOnlyPartial,
    ReviewSignerChange,
    RefreshMirror,
    SwitchToLiveOrigin,
}

impl NextStepDecisionHook {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueInRestrictedMode => "continue_in_restricted_mode",
            Self::ReviewTrustAndOpen => "review_trust_and_open",
            Self::OpenMinimal => "open_minimal",
            Self::SetUpLater => "set_up_later",
            Self::ReviewArchetypeMatch => "review_archetype_match",
            Self::CompareBeforeRestore => "compare_before_restore",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::SafeMode => "safe_mode",
            Self::ReconnectRequired => "reconnect_required",
            Self::ReauthRequired => "reauth_required",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::RemoveFromRecents => "remove_from_recents",
            Self::ReviewMigrationReport => "review_migration_report",
            Self::RollBackImport => "roll_back_import",
            Self::KeepImportedState => "keep_imported_state",
            Self::AdoptRecommendedBundle => "adopt_recommended_bundle",
            Self::ReviewUnsupportedItems => "review_unsupported_items",
            Self::ResumeAcquisition => "resume_acquisition",
            Self::DiscardAndRestart => "discard_and_restart",
            Self::OpenReadOnlyPartial => "open_read_only_partial",
            Self::ReviewSignerChange => "review_signer_change",
            Self::RefreshMirror => "refresh_mirror",
            Self::SwitchToLiveOrigin => "switch_to_live_origin",
        }
    }
}

/// One checkout-plan record. Mirrors `checkout_plan.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckoutPlanRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,

    pub record_kind: CheckoutPlanRecordKind,
    pub checkout_plan_schema_version: u32,
    pub checkout_plan_id: String,

    pub source_locator_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_action_ref: Option<String>,

    pub trust_state: CheckoutTrustState,
    pub trust_stage: CheckoutTrustStage,

    pub browse_safe_actions: Vec<BrowseSafeActionClass>,
    pub blocked_execution_paths: Vec<BlockedExecutionPathClass>,

    pub resumable_acquisition: ResumableAcquisitionState,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_freshness: Option<MirrorFreshnessEvidence>,
    pub signer_continuity: SignerContinuityEvidence,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub read_only_partial_roots: Vec<ReadOnlyPartialRoot>,

    pub topology_markers: Vec<TopologyMarker>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_narrowing_refs: Vec<PolicyNarrowingRef>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bootstrap_queue_ref: Option<String>,

    pub next_step_decision_hooks: Vec<NextStepDecisionHook>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_subtitle: Option<String>,

    pub emitted_at: String,
}

// ----------------------------------------------------------------------
// BootstrapQueueItemRecord
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapQueueItemRecordKind {
    BootstrapQueueItemRecord,
}

/// Closed bootstrap-item class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapItemClass {
    SubmoduleInit,
    LfsHydrate,
    PartialCloneHydrate,
    ShallowHistoryDeepen,
    PackageRestore,
    PackageAudit,
    GeneratorInstall,
    GeneratorRun,
    ToolchainInstall,
    ToolchainDetect,
    DevcontainerAttach,
    PrebuildAttach,
    ExtensionRestore,
    ExtensionActivation,
    IndexWarmUp,
    DocsImport,
    AiContextWarmUp,
    SecretHandleRequest,
    CacheWarm,
    SettingsMaterialize,
    ProfileMaterialize,
    CredentialProvisioning,
    MirrorRefresh,
    Other,
}

impl BootstrapItemClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmoduleInit => "submodule_init",
            Self::LfsHydrate => "lfs_hydrate",
            Self::PartialCloneHydrate => "partial_clone_hydrate",
            Self::ShallowHistoryDeepen => "shallow_history_deepen",
            Self::PackageRestore => "package_restore",
            Self::PackageAudit => "package_audit",
            Self::GeneratorInstall => "generator_install",
            Self::GeneratorRun => "generator_run",
            Self::ToolchainInstall => "toolchain_install",
            Self::ToolchainDetect => "toolchain_detect",
            Self::DevcontainerAttach => "devcontainer_attach",
            Self::PrebuildAttach => "prebuild_attach",
            Self::ExtensionRestore => "extension_restore",
            Self::ExtensionActivation => "extension_activation",
            Self::IndexWarmUp => "index_warm_up",
            Self::DocsImport => "docs_import",
            Self::AiContextWarmUp => "ai_context_warm_up",
            Self::SecretHandleRequest => "secret_handle_request",
            Self::CacheWarm => "cache_warm",
            Self::SettingsMaterialize => "settings_materialize",
            Self::ProfileMaterialize => "profile_materialize",
            Self::CredentialProvisioning => "credential_provisioning",
            Self::MirrorRefresh => "mirror_refresh",
            Self::Other => "other",
        }
    }
}

/// Closed bootstrap execution-class set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapExecutionClass {
    BrowseSafe,
    SideEffectDeclared,
    NetworkRequired,
    Privileged,
    Blocked,
    DeferredUntilTrustAdmitted,
    ManualUserActionRequired,
}

impl BootstrapExecutionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowseSafe => "browse_safe",
            Self::SideEffectDeclared => "side_effect_declared",
            Self::NetworkRequired => "network_required",
            Self::Privileged => "privileged",
            Self::Blocked => "blocked",
            Self::DeferredUntilTrustAdmitted => "deferred_until_trust_admitted",
            Self::ManualUserActionRequired => "manual_user_action_required",
        }
    }

    /// True when running the item before trust admission would execute a
    /// side effect, touch the network, or require elevation.
    pub const fn runs_side_effects(self) -> bool {
        matches!(
            self,
            Self::SideEffectDeclared | Self::NetworkRequired | Self::Privileged
        )
    }
}

/// Closed bootstrap-item state set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapItemState {
    Pending,
    Running,
    Succeeded,
    FailedRecoverable,
    FailedBlocking,
    Skipped,
    Cancelled,
    AwaitingAdmission,
    AwaitingNetwork,
    AwaitingUserAction,
    AwaitingPolicyDecision,
}

impl BootstrapItemState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::FailedRecoverable => "failed_recoverable",
            Self::FailedBlocking => "failed_blocking",
            Self::Skipped => "skipped",
            Self::Cancelled => "cancelled",
            Self::AwaitingAdmission => "awaiting_admission",
            Self::AwaitingNetwork => "awaiting_network",
            Self::AwaitingUserAction => "awaiting_user_action",
            Self::AwaitingPolicyDecision => "awaiting_policy_decision",
        }
    }

    /// True for the states that require a typed blocker and at least one
    /// repair hook.
    pub const fn requires_blocker_and_repair(self) -> bool {
        matches!(
            self,
            Self::FailedRecoverable
                | Self::FailedBlocking
                | Self::AwaitingNetwork
                | Self::AwaitingUserAction
                | Self::AwaitingPolicyDecision
        )
    }

    /// True when the item still needs to run before the workspace is fully
    /// set up (it has not succeeded, been skipped, or cancelled).
    pub const fn is_remaining(self) -> bool {
        matches!(
            self,
            Self::Pending
                | Self::FailedRecoverable
                | Self::FailedBlocking
                | Self::AwaitingAdmission
                | Self::AwaitingNetwork
                | Self::AwaitingUserAction
                | Self::AwaitingPolicyDecision
        )
    }
}

/// Closed absence-class set. Distinguishes not-yet-fetched / not-yet-hydrated
/// content from genuine absence or user misconfiguration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AbsenceClass {
    Present,
    NotYetFetched,
    NotYetHydrated,
    PartiallyHydrated,
    GenuinelyAbsent,
    UserMisconfigured,
    PolicyBlocked,
    TrustBlocked,
    MirrorUnreachable,
    AuthorityExpired,
    SchemaVersionUnsupported,
    Unknown,
}

impl AbsenceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::NotYetFetched => "not_yet_fetched",
            Self::NotYetHydrated => "not_yet_hydrated",
            Self::PartiallyHydrated => "partially_hydrated",
            Self::GenuinelyAbsent => "genuinely_absent",
            Self::UserMisconfigured => "user_misconfigured",
            Self::PolicyBlocked => "policy_blocked",
            Self::TrustBlocked => "trust_blocked",
            Self::MirrorUnreachable => "mirror_unreachable",
            Self::AuthorityExpired => "authority_expired",
            Self::SchemaVersionUnsupported => "schema_version_unsupported",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed skip-reason class. Required when state is `skipped`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkipReasonClass {
    UserDeselected,
    PolicyExcludes,
    TrustExcludes,
    UnsupportedOnTarget,
    RedundantWithExisting,
    SourceMissing,
    OfflineBundleExcludes,
    LossyStepRefused,
    Other,
}

impl SkipReasonClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserDeselected => "user_deselected",
            Self::PolicyExcludes => "policy_excludes",
            Self::TrustExcludes => "trust_excludes",
            Self::UnsupportedOnTarget => "unsupported_on_target",
            Self::RedundantWithExisting => "redundant_with_existing",
            Self::SourceMissing => "source_missing",
            Self::OfflineBundleExcludes => "offline_bundle_excludes",
            Self::LossyStepRefused => "lossy_step_refused",
            Self::Other => "other",
        }
    }
}

/// Closed blocker class. Required on failure / awaiting states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerClass {
    NetworkInterruption,
    AuthExpired,
    AuthorityRevoked,
    SignerMismatch,
    MirrorUnreachable,
    DiskPressure,
    QuotaExceeded,
    PolicyBlocked,
    TrustBlocked,
    SchemaVersionUnsupported,
    DependencyItemFailed,
    UserCancelled,
    Unknown,
}

impl BlockerClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkInterruption => "network_interruption",
            Self::AuthExpired => "auth_expired",
            Self::AuthorityRevoked => "authority_revoked",
            Self::SignerMismatch => "signer_mismatch",
            Self::MirrorUnreachable => "mirror_unreachable",
            Self::DiskPressure => "disk_pressure",
            Self::QuotaExceeded => "quota_exceeded",
            Self::PolicyBlocked => "policy_blocked",
            Self::TrustBlocked => "trust_blocked",
            Self::SchemaVersionUnsupported => "schema_version_unsupported",
            Self::DependencyItemFailed => "dependency_item_failed",
            Self::UserCancelled => "user_cancelled",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed attributable-evidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributableEvidenceClass {
    SourceProducerIdentity,
    CheckoutPlanReference,
    SourceLocatorReference,
    PackageManifestRef,
    LockfileDigestRef,
    GeneratorManifestRef,
    ToolchainManifestRef,
    ExtensionManifestRef,
    DocsPackManifestRef,
    PolicyBundleRef,
    TrustReviewTicketRef,
    SignerContinuityRef,
    MirrorFreshnessRef,
    PreviousBootstrapRunRef,
    RecoveryCheckpointRef,
    UserActionRef,
}

impl AttributableEvidenceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceProducerIdentity => "source_producer_identity",
            Self::CheckoutPlanReference => "checkout_plan_reference",
            Self::SourceLocatorReference => "source_locator_reference",
            Self::PackageManifestRef => "package_manifest_ref",
            Self::LockfileDigestRef => "lockfile_digest_ref",
            Self::GeneratorManifestRef => "generator_manifest_ref",
            Self::ToolchainManifestRef => "toolchain_manifest_ref",
            Self::ExtensionManifestRef => "extension_manifest_ref",
            Self::DocsPackManifestRef => "docs_pack_manifest_ref",
            Self::PolicyBundleRef => "policy_bundle_ref",
            Self::TrustReviewTicketRef => "trust_review_ticket_ref",
            Self::SignerContinuityRef => "signer_continuity_ref",
            Self::MirrorFreshnessRef => "mirror_freshness_ref",
            Self::PreviousBootstrapRunRef => "previous_bootstrap_run_ref",
            Self::RecoveryCheckpointRef => "recovery_checkpoint_ref",
            Self::UserActionRef => "user_action_ref",
        }
    }
}

/// One typed attributable-evidence entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributableEvidence {
    pub evidence_class: AttributableEvidenceClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary_label: Option<String>,
}

/// Closed setup-actions class for a side-effect envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupActionsClass {
    NoSetupActions,
    ToolchainDetectOnly,
    DependencyInstall,
    ContainerizedBootstrap,
    DevcontainerBootstrap,
    RemoteAttach,
    BrowserAuthHandshake,
    SecretHandleRequest,
    MixedSetup,
}

/// Closed time class for a side-effect envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectTimeClass {
    Instant,
    Fast,
    Moderate,
    LongRunning,
    NetworkDependent,
}

/// Closed connectivity class for a side-effect envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectConnectivityClass {
    LocalOnly,
    LocalWithRegistry,
    RemoteRequired,
    MirrorOrAirGapped,
    Unknown,
}

/// Closed cleanup class for a side-effect envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectCleanupClass {
    NoCleanupRequired,
    CleanupOnFailure,
    RollbackCheckpointRetained,
    ManualCleanupRequired,
}

/// Closed bypass-path class for a side-effect envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectBypassPath {
    OpenPrebuildMinimal,
    OpenPlainFolder,
    SetUpLater,
    NoBypassAvailable,
}

/// Typed side-effect envelope for side-effect-declared items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectEnvelope {
    pub setup_actions_class: SetupActionsClass,
    pub time_class: SideEffectTimeClass,
    pub connectivity_class: SideEffectConnectivityClass,
    pub cleanup_class: SideEffectCleanupClass,
    pub bypass_path: SideEffectBypassPath,
}

/// Closed repair-hook class. Required on failure / awaiting states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairHookClass {
    RetryWithBackoff,
    RetryAfterNetworkRestored,
    RetryAfterReauth,
    RetryAfterPolicyRefresh,
    RefreshMirrorThenRetry,
    SwitchToLiveOriginThenRetry,
    DeepenHistoryThenRetry,
    SkipAndContinue,
    SkipAndOpenReadOnly,
    DiscardAndRestart,
    OpenReadOnlyPartial,
    RequestAdminHelp,
    OpenMinimal,
    SetUpLater,
}

impl RepairHookClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryWithBackoff => "retry_with_backoff",
            Self::RetryAfterNetworkRestored => "retry_after_network_restored",
            Self::RetryAfterReauth => "retry_after_reauth",
            Self::RetryAfterPolicyRefresh => "retry_after_policy_refresh",
            Self::RefreshMirrorThenRetry => "refresh_mirror_then_retry",
            Self::SwitchToLiveOriginThenRetry => "switch_to_live_origin_then_retry",
            Self::DeepenHistoryThenRetry => "deepen_history_then_retry",
            Self::SkipAndContinue => "skip_and_continue",
            Self::SkipAndOpenReadOnly => "skip_and_open_read_only",
            Self::DiscardAndRestart => "discard_and_restart",
            Self::OpenReadOnlyPartial => "open_read_only_partial",
            Self::RequestAdminHelp => "request_admin_help",
            Self::OpenMinimal => "open_minimal",
            Self::SetUpLater => "set_up_later",
        }
    }
}

/// One typed bootstrap-queue-item record. Mirrors
/// `bootstrap_queue_item.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapQueueItemRecord {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "__fixture__", default, skip_serializing_if = "Option::is_none")]
    pub fixture: Option<FixtureMetadata>,

    pub record_kind: BootstrapQueueItemRecordKind,
    pub bootstrap_queue_item_schema_version: u32,
    pub bootstrap_item_id: String,

    pub checkout_plan_ref: String,
    pub source_locator_ref: String,

    pub queue_position: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on_item_ids: Vec<String>,

    pub item_class: BootstrapItemClass,
    pub execution_class: BootstrapExecutionClass,
    pub state: BootstrapItemState,
    pub absence_class: AbsenceClass,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_reason: Option<SkipReasonClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocker: Option<BlockerClass>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side_effect_envelope: Option<SideEffectEnvelope>,

    pub attributable_evidence: Vec<AttributableEvidence>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topology_marker_refs: Vec<TopologyMarkerClass>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repair_hooks: Vec<RepairHookClass>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_visible_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation_label: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    pub emitted_at: String,
}

impl BootstrapQueueItemRecord {
    /// True when the item is one of the explicit interrupted / blocked
    /// states whose contract requires a typed blocker and at least one
    /// typed repair hook.
    pub fn is_well_formed_blocked_item(&self) -> bool {
        if !self.state.requires_blocker_and_repair() {
            return true;
        }
        self.blocker.is_some() && !self.repair_hooks.is_empty()
    }
}
