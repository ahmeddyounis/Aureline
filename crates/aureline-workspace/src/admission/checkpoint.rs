//! Post-entry admission checkpoints and first-useful-work routing.
//!
//! This module consumes the reviewed [`AdmissionReviewPacket`] emitted by the
//! entry surface and adds the post-entry truth that downstream shell, support,
//! CLI, and project-doctor surfaces need: trust state, archetype confidence,
//! source-labeled detection signals, readiness buckets, boundary choices, and
//! a reversible first-useful-work route.

use serde::{Deserialize, Serialize};

use super::{AdmissionAction, AdmissionReviewPacket};
use crate::{ResultingMode, TargetKind, TrustState};

macro_rules! impl_as_str {
    ($ty:ty { $($variant:ident => $value:literal),+ $(,)? }) => {
        impl $ty {
            /// Returns the stable snake_case token for this vocabulary value.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }
    };
}

/// Schema version for [`AdmissionCheckpointRouteRecord`].
pub const ADMISSION_CHECKPOINT_ROUTE_SCHEMA_VERSION: u32 = 1;

/// Schema version for [`WorkspaceAdmissionCheckpoint`].
pub const WORKSPACE_ADMISSION_CHECKPOINT_SCHEMA_VERSION: u32 = 2;

/// Identifies an `admission_checkpoint_route_record`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionCheckpointRouteRecordKind {
    /// `admission_checkpoint_route_record`.
    AdmissionCheckpointRouteRecord,
}

/// Entry source used by first-useful-work routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstUsefulEntrySource {
    /// A single file was opened from the OS, CLI, or shell.
    SingleFileOpen,
    /// A folder or repository root was opened.
    FolderOrRepoOpen,
    /// A remote repository was cloned.
    RepositoryClone,
    /// A review, incident, or work-item link opened the product.
    ReviewOrIncidentDeepLink,
    /// The user restored the previous session.
    RestoreLastSession,
    /// Imported state or a handoff packet started the route.
    ImportedStateOrHandoffPacket,
}

impl_as_str!(FirstUsefulEntrySource {
    SingleFileOpen => "single_file_open",
    FolderOrRepoOpen => "folder_or_repo_open",
    RepositoryClone => "repository_clone",
    ReviewOrIncidentDeepLink => "review_or_incident_deep_link",
    RestoreLastSession => "restore_last_session",
    ImportedStateOrHandoffPacket => "imported_state_or_handoff_packet",
});

/// Outcome class for workspace archetype detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionOutcome {
    /// Signals match a current certified workflow.
    CertifiedArchetypeMatch,
    /// Strong signals exist, but the claim is not certified-current.
    ProbableArchetype,
    /// Multiple stacks or roots compete.
    MixedOrAmbiguousWorkspace,
    /// No strong claim is available.
    UnknownOrGenericWorkspace,
    /// Policy or trust restrictions narrow useful setup.
    RestrictedOrPolicyBlocked,
    /// A named runtime, toolchain, container, kernel, or remote prerequisite is missing.
    MissingPrerequisite,
}

impl_as_str!(DetectionOutcome {
    CertifiedArchetypeMatch => "certified_archetype_match",
    ProbableArchetype => "probable_archetype",
    MixedOrAmbiguousWorkspace => "mixed_or_ambiguous_workspace",
    UnknownOrGenericWorkspace => "unknown_or_generic_workspace",
    RestrictedOrPolicyBlocked => "restricted_or_policy_blocked",
    MissingPrerequisite => "missing_prerequisite",
});

impl DetectionOutcome {
    /// Returns the user-facing family label for this outcome.
    pub const fn family_label(self) -> &'static str {
        match self {
            Self::CertifiedArchetypeMatch => "Certified",
            Self::ProbableArchetype => "Probable",
            Self::MixedOrAmbiguousWorkspace => "Mixed or ambiguous",
            Self::UnknownOrGenericWorkspace => "Unknown",
            Self::RestrictedOrPolicyBlocked => "Restricted",
            Self::MissingPrerequisite => "Missing prerequisite",
        }
    }
}

/// Confidence class attached to the detection outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionConfidenceClass {
    /// Certified signals match the scoped launch wedge.
    CertifiedExact,
    /// Strong non-certified signals are present.
    HighProbable,
    /// Some signals are present, but gaps are material.
    MediumProbable,
    /// Signals conflict across roots or stacks.
    MixedConflicting,
    /// The workspace stays generic because no strong signal is present.
    GenericUnknown,
    /// Policy restrictions govern the claim.
    RestrictedByPolicy,
    /// A missing prerequisite governs the claim.
    PrerequisiteMissing,
}

impl_as_str!(DetectionConfidenceClass {
    CertifiedExact => "certified_exact",
    HighProbable => "high_probable",
    MediumProbable => "medium_probable",
    MixedConflicting => "mixed_conflicting",
    GenericUnknown => "generic_unknown",
    RestrictedByPolicy => "restricted_by_policy",
    PrerequisiteMissing => "prerequisite_missing",
});

/// Scoped support claim allowed by detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClaimClass {
    /// Certified evidence is current.
    CertifiedCurrent,
    /// Certified evidence exists but requires retest before strong wording.
    CertifiedRetestPending,
    /// First-party support exists with explicit scope.
    SupportedScoped,
    /// Community or extension support exists.
    CommunityOrExtensionPath,
    /// Preview-only support exists.
    ExperimentalPreview,
    /// No support claim is made.
    GenericNoClaim,
    /// Policy blocks the claim.
    ClaimBlockedByPolicy,
    /// A missing prerequisite blocks the claim.
    ClaimUnavailableMissingPrerequisite,
}

impl_as_str!(SupportClaimClass {
    CertifiedCurrent => "certified_current",
    CertifiedRetestPending => "certified_retest_pending",
    SupportedScoped => "supported_scoped",
    CommunityOrExtensionPath => "community_or_extension_path",
    ExperimentalPreview => "experimental_preview",
    GenericNoClaim => "generic_no_claim",
    ClaimBlockedByPolicy => "claim_blocked_by_policy",
    ClaimUnavailableMissingPrerequisite => "claim_unavailable_missing_prerequisite",
});

/// Freshness and completion state for detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectorState {
    /// Detection is still running.
    Detecting,
    /// Detection is ready enough to route first useful work.
    ReadyEnough,
    /// Detection is partial.
    Partial,
    /// Detection relies on stale evidence.
    Stale,
    /// Detection needs retest before claim-bearing wording.
    RetestNeeded,
    /// Detection is blocked.
    Blocked,
    /// Detection has no useful signal.
    Unknown,
}

impl_as_str!(DetectorState {
    Detecting => "detecting",
    ReadyEnough => "ready_enough",
    Partial => "partial",
    Stale => "stale",
    RetestNeeded => "retest_needed",
    Blocked => "blocked",
    Unknown => "unknown",
});

/// Freshness class for detection evidence used by support or route language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalFreshnessClass {
    /// Evidence was verified in the current review window.
    FreshCurrent,
    /// Cached evidence is still current enough for routing or scoped recommendations.
    CachedCurrentEnough,
    /// Evidence is stale and requires retest before stronger claims.
    StaleRetestNeeded,
    /// Evidence came from an imported snapshot.
    ImportedSnapshot,
    /// Policy evidence is current for the active policy epoch.
    PolicyEpochCurrent,
    /// Policy evidence exists but its epoch is not known.
    PolicyEpochUnknown,
    /// Freshness is not known.
    UnknownFreshness,
}

impl_as_str!(SignalFreshnessClass {
    FreshCurrent => "fresh_current",
    CachedCurrentEnough => "cached_current_enough",
    StaleRetestNeeded => "stale_retest_needed",
    ImportedSnapshot => "imported_snapshot",
    PolicyEpochCurrent => "policy_epoch_current",
    PolicyEpochUnknown => "policy_epoch_unknown",
    UnknownFreshness => "unknown_freshness",
});

/// Source class for a detection signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSignalSourceClass {
    /// Project or tool manifest.
    Manifest,
    /// Workflow-bundle marker.
    BundleMarker,
    /// Workspace or workset file.
    WorkspaceFile,
    /// Portable state, handoff, migration, review, or support packet.
    ImportPacket,
    /// Signed admin policy or fleet policy.
    AdminPolicy,
    /// Installed extension contribution.
    ExtensionContribution,
    /// Prior explicit user choice.
    PreviousUserChoice,
    /// Filesystem layout signal such as nested roots.
    FilesystemLayout,
    /// Lockfile signal.
    Lockfile,
    /// Runtime probe signal.
    RuntimeProbe,
    /// Version-control metadata signal.
    VcsMetadata,
}

impl_as_str!(DetectionSignalSourceClass {
    Manifest => "manifest",
    BundleMarker => "bundle_marker",
    WorkspaceFile => "workspace_file",
    ImportPacket => "import_packet",
    AdminPolicy => "admin_policy",
    ExtensionContribution => "extension_contribution",
    PreviousUserChoice => "previous_user_choice",
    FilesystemLayout => "filesystem_layout",
    Lockfile => "lockfile",
    RuntimeProbe => "runtime_probe",
    VcsMetadata => "vcs_metadata",
});

/// Material effect that makes a detection signal inspectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalMaterialEffect {
    /// Signal affects trust explanation.
    Trust,
    /// Signal affects support claim wording.
    SupportClaim,
    /// Signal affects recommendations.
    Recommendation,
    /// Signal affects route selection.
    RouteSelection,
    /// Signal affects policy explanation.
    Policy,
    /// Signal affects readiness grouping.
    Readiness,
    /// Signal is diagnostic only.
    DiagnosticOnly,
}

impl_as_str!(SignalMaterialEffect {
    Trust => "trust",
    SupportClaim => "support_claim",
    Recommendation => "recommendation",
    RouteSelection => "route_selection",
    Policy => "policy",
    Readiness => "readiness",
    DiagnosticOnly => "diagnostic_only",
});

/// One source-labeled detection signal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectionSignal {
    /// Opaque signal reference.
    pub signal_ref: String,
    /// Source class for the signal.
    pub source_class: DetectionSignalSourceClass,
    /// Why the signal materially affects the route or checkpoint.
    pub material_effects: Vec<SignalMaterialEffect>,
    /// Freshness of the signal when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_class: Option<SignalFreshnessClass>,
    /// Redacted reviewer-facing summary.
    pub summary: String,
}

impl DetectionSignal {
    /// Builds a source-labeled detection signal.
    pub fn new(
        signal_ref: impl Into<String>,
        source_class: DetectionSignalSourceClass,
        material_effects: Vec<SignalMaterialEffect>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            signal_ref: signal_ref.into(),
            source_class,
            material_effects,
            freshness_class: None,
            summary: summary.into(),
        }
    }

    /// Adds a freshness class for this signal.
    pub const fn with_freshness_class(mut self, freshness_class: SignalFreshnessClass) -> Self {
        self.freshness_class = Some(freshness_class);
        self
    }
}

/// Evidence freshness attached to an archetype support claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DetectionEvidenceFreshness {
    /// Evidence packet, scorecard, or matrix row reference.
    pub evidence_ref: String,
    /// Freshness class for the evidence.
    pub freshness_class: SignalFreshnessClass,
    /// Date the evidence was reviewed, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_on: Option<String>,
    /// Duration or review window after which the evidence must be retested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_after: Option<String>,
    /// Redacted reviewer-facing summary.
    pub summary: String,
}

impl DetectionEvidenceFreshness {
    /// Builds an evidence-freshness row.
    pub fn new(
        evidence_ref: impl Into<String>,
        freshness_class: SignalFreshnessClass,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            evidence_ref: evidence_ref.into(),
            freshness_class,
            reviewed_on: None,
            stale_after: None,
            summary: summary.into(),
        }
    }

    /// Adds review age fields to the evidence-freshness row.
    pub fn with_review_window(
        mut self,
        reviewed_on: Option<String>,
        stale_after: Option<String>,
    ) -> Self {
        self.reviewed_on = reviewed_on;
        self.stale_after = stale_after;
        self
    }
}

/// Archetype detection truth carried by the checkpoint route record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeTruth {
    /// Detection outcome family.
    pub outcome: DetectionOutcome,
    /// Detection confidence class.
    pub confidence_class: DetectionConfidenceClass,
    /// Scoped support claim allowed by detection.
    pub support_claim_class: SupportClaimClass,
    /// Completion and freshness state for the detector.
    pub detector_state: DetectorState,
    /// Optional archetype reference.
    pub archetype_ref: Option<String>,
    /// Compatible bundle references derived from detection.
    pub compatible_bundle_refs: Vec<String>,
    /// Source-labeled signals that explain the outcome.
    pub signals: Vec<DetectionSignal>,
    /// Evidence freshness rows that support certified or probable wording.
    #[serde(default)]
    pub evidence_freshness: Vec<DetectionEvidenceFreshness>,
    /// Opaque fact references surfaced by detection.
    pub detected_fact_refs: Vec<String>,
    /// Opaque recommendation references derived from facts.
    pub recommendation_refs: Vec<String>,
    /// Opaque policy block references.
    pub policy_block_refs: Vec<String>,
    /// Redacted unknowns or gaps.
    pub unknowns: Vec<String>,
}

impl ArchetypeTruth {
    /// Builds archetype truth with the minimum required signal list.
    pub fn new(
        outcome: DetectionOutcome,
        confidence_class: DetectionConfidenceClass,
        support_claim_class: SupportClaimClass,
        detector_state: DetectorState,
        signals: Vec<DetectionSignal>,
    ) -> Self {
        Self {
            outcome,
            confidence_class,
            support_claim_class,
            detector_state,
            archetype_ref: None,
            compatible_bundle_refs: Vec::new(),
            signals,
            evidence_freshness: Vec::new(),
            detected_fact_refs: Vec::new(),
            recommendation_refs: Vec::new(),
            policy_block_refs: Vec::new(),
            unknowns: Vec::new(),
        }
    }

    /// Sets the archetype reference.
    pub fn with_archetype_ref(mut self, archetype_ref: impl Into<String>) -> Self {
        self.archetype_ref = Some(archetype_ref.into());
        self
    }

    /// Sets compatible bundle references.
    pub fn with_compatible_bundle_refs(mut self, refs: Vec<String>) -> Self {
        self.compatible_bundle_refs = refs;
        self
    }

    /// Sets evidence freshness rows.
    pub fn with_evidence_freshness(mut self, rows: Vec<DetectionEvidenceFreshness>) -> Self {
        self.evidence_freshness = rows;
        self
    }

    /// Sets detected fact references.
    pub fn with_detected_fact_refs(mut self, refs: Vec<String>) -> Self {
        self.detected_fact_refs = refs;
        self
    }

    /// Sets recommendation references.
    pub fn with_recommendation_refs(mut self, refs: Vec<String>) -> Self {
        self.recommendation_refs = refs;
        self
    }

    /// Sets policy block references.
    pub fn with_policy_block_refs(mut self, refs: Vec<String>) -> Self {
        self.policy_block_refs = refs;
        self
    }
}

/// Bucket for one readiness task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessBucket {
    /// Work required now to reach a safe requested surface.
    BlockingNow,
    /// Work recommended soon, but not required for plain editing.
    RecommendedSoon,
    /// Additive or dismissable work.
    OptionalLater,
}

impl_as_str!(ReadinessBucket {
    BlockingNow => "blocking_now",
    RecommendedSoon => "recommended_soon",
    OptionalLater => "optional_later",
});

/// Class of readiness task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessTaskClass {
    /// Workspace trust review.
    TrustReview,
    /// Policy review.
    PolicyReview,
    /// Toolchain detection.
    ToolchainDetect,
    /// Toolchain installation.
    ToolchainInstall,
    /// Dependency restore.
    DependencyRestore,
    /// Extension recommendation.
    ExtensionRecommendation,
    /// Extension installation.
    ExtensionInstall,
    /// Runtime attach.
    RuntimeAttach,
    /// Remote agent reconnect.
    RemoteAgentReconnect,
    /// Devcontainer build.
    DevcontainerBuild,
    /// Package manager selection.
    PackageManagerSelect,
    /// Index warmup.
    IndexWarmup,
    /// Documentation import.
    DocsImport,
    /// Test discovery.
    TestDiscovery,
    /// User boundary choice for mixed workspaces.
    UserBoundaryChoice,
    /// Imported state compare.
    ImportedStateCompare,
    /// Restore review.
    RestoreReview,
    /// Deep-link clone or local checkout required for full interaction.
    DeepLinkCloneRequired,
}

impl_as_str!(ReadinessTaskClass {
    TrustReview => "trust_review",
    PolicyReview => "policy_review",
    ToolchainDetect => "toolchain_detect",
    ToolchainInstall => "toolchain_install",
    DependencyRestore => "dependency_restore",
    ExtensionRecommendation => "extension_recommendation",
    ExtensionInstall => "extension_install",
    RuntimeAttach => "runtime_attach",
    RemoteAgentReconnect => "remote_agent_reconnect",
    DevcontainerBuild => "devcontainer_build",
    PackageManagerSelect => "package_manager_select",
    IndexWarmup => "index_warmup",
    DocsImport => "docs_import",
    TestDiscovery => "test_discovery",
    UserBoundaryChoice => "user_boundary_choice",
    ImportedStateCompare => "imported_state_compare",
    RestoreReview => "restore_review",
    DeepLinkCloneRequired => "deep_link_clone_required",
});

/// State for one readiness task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessTaskState {
    /// Task is already ready.
    Ready,
    /// Task is pending user or product action.
    Pending,
    /// Task is blocked by policy.
    BlockedByPolicy,
    /// Task is blocked by trust state.
    BlockedByTrust,
    /// Task is blocked by a missing prerequisite.
    MissingPrerequisite,
    /// Task was deferred by the user.
    DeferredByUser,
    /// Task is optional.
    Optional,
    /// Task is unavailable.
    Unavailable,
}

impl_as_str!(ReadinessTaskState {
    Ready => "ready",
    Pending => "pending",
    BlockedByPolicy => "blocked_by_policy",
    BlockedByTrust => "blocked_by_trust",
    MissingPrerequisite => "missing_prerequisite",
    DeferredByUser => "deferred_by_user",
    Optional => "optional",
    Unavailable => "unavailable",
});

/// Execution boundary for readiness work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionBoundary {
    /// Local machine.
    LocalMachine,
    /// Container.
    Container,
    /// Remote agent.
    RemoteAgent,
    /// Managed workspace.
    ManagedWorkspace,
    /// Browser handoff.
    BrowserHandoff,
    /// No execution will occur.
    NoExecution,
}

impl_as_str!(ExecutionBoundary {
    LocalMachine => "local_machine",
    Container => "container",
    RemoteAgent => "remote_agent",
    ManagedWorkspace => "managed_workspace",
    BrowserHandoff => "browser_handoff",
    NoExecution => "no_execution",
});

/// Side-effect class for readiness work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// Reads workspace metadata.
    ReadsWorkspace,
    /// Writes workspace files.
    WritesWorkspaceFiles,
    /// Installs packages.
    InstallsPackages,
    /// Downloads dependencies.
    DownloadsDependencies,
    /// Widens workspace trust.
    WidensTrust,
    /// Widens network access.
    WidensNetwork,
    /// Contacts a remote service.
    ContactsRemote,
    /// Starts a process.
    StartsProcess,
    /// Attaches to a runtime.
    AttachesRuntime,
    /// Changes layout.
    ChangesLayout,
    /// Has no side effect.
    NoSideEffect,
}

impl_as_str!(SideEffectClass {
    ReadsWorkspace => "reads_workspace",
    WritesWorkspaceFiles => "writes_workspace_files",
    InstallsPackages => "installs_packages",
    DownloadsDependencies => "downloads_dependencies",
    WidensTrust => "widens_trust",
    WidensNetwork => "widens_network",
    ContactsRemote => "contacts_remote",
    StartsProcess => "starts_process",
    AttachesRuntime => "attaches_runtime",
    ChangesLayout => "changes_layout",
    NoSideEffect => "no_side_effect",
});

/// Reason class for a blocking readiness task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockedReasonClass {
    /// Task is blocked by trust state.
    BlockedByTrust,
    /// Task is blocked by policy.
    BlockedByPolicy,
    /// Task is blocked by a missing prerequisite.
    BlockedByMissingPrerequisite,
    /// Task is blocked by deployment profile.
    BlockedByDeploymentProfile,
}

impl_as_str!(BlockedReasonClass {
    BlockedByTrust => "blocked_by_trust",
    BlockedByPolicy => "blocked_by_policy",
    BlockedByMissingPrerequisite => "blocked_by_missing_prerequisite",
    BlockedByDeploymentProfile => "blocked_by_deployment_profile",
});

/// Reason class for an optional readiness task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionalReasonClass {
    /// Optional additive setup.
    OptionalAdditive,
    /// Recommended but optional setup.
    OptionalRecommendedOnly,
    /// User dismissed the guidance.
    OptionalUserDismissed,
    /// Optional due to evidence freshness.
    OptionalFreshnessBased,
}

impl_as_str!(OptionalReasonClass {
    OptionalAdditive => "optional_additive",
    OptionalRecommendedOnly => "optional_recommended_only",
    OptionalUserDismissed => "optional_user_dismissed",
    OptionalFreshnessBased => "optional_freshness_based",
});

/// One readiness task with bucket, boundary, side effects, and source refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessTask {
    /// Opaque task reference.
    pub task_ref: String,
    /// Task class.
    pub task_class: ReadinessTaskClass,
    /// Bucket that owns the task.
    pub bucket: ReadinessBucket,
    /// Current task state.
    pub state: ReadinessTaskState,
    /// Execution boundary.
    pub execution_boundary: ExecutionBoundary,
    /// Side effects disclosed before the task runs.
    pub side_effects: Vec<SideEffectClass>,
    /// Source signal references.
    pub source_signal_refs: Vec<String>,
    /// Recommendation references affected by the task.
    pub recommendation_refs: Vec<String>,
    /// Policy block references affected by the task.
    pub policy_block_refs: Vec<String>,
    /// Reason for a blocking task.
    pub blocked_reason_class: Option<BlockedReasonClass>,
    /// Reason for an optional task.
    pub optional_reason_class: Option<OptionalReasonClass>,
    /// Redacted reviewer-facing summary.
    pub summary: String,
}

impl ReadinessTask {
    /// Builds a readiness task.
    pub fn new(
        task_ref: impl Into<String>,
        task_class: ReadinessTaskClass,
        bucket: ReadinessBucket,
        state: ReadinessTaskState,
        execution_boundary: ExecutionBoundary,
        side_effects: Vec<SideEffectClass>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            task_ref: task_ref.into(),
            task_class,
            bucket,
            state,
            execution_boundary,
            side_effects,
            source_signal_refs: Vec::new(),
            recommendation_refs: Vec::new(),
            policy_block_refs: Vec::new(),
            blocked_reason_class: None,
            optional_reason_class: None,
            summary: summary.into(),
        }
    }

    /// Adds a blocked reason class.
    pub const fn with_blocked_reason(mut self, reason: BlockedReasonClass) -> Self {
        self.blocked_reason_class = Some(reason);
        self
    }

    /// Adds an optional reason class.
    pub const fn with_optional_reason(mut self, reason: OptionalReasonClass) -> Self {
        self.optional_reason_class = Some(reason);
        self
    }

    /// Adds source signal references.
    pub fn with_source_signal_refs(mut self, refs: Vec<String>) -> Self {
        self.source_signal_refs = refs;
        self
    }

    /// Adds recommendation references.
    pub fn with_recommendation_refs(mut self, refs: Vec<String>) -> Self {
        self.recommendation_refs = refs;
        self
    }

    /// Adds policy block references.
    pub fn with_policy_block_refs(mut self, refs: Vec<String>) -> Self {
        self.policy_block_refs = refs;
        self
    }
}

/// Three readiness buckets kept distinct for review.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessBuckets {
    /// Work that blocks the requested route now.
    pub blocking_now: Vec<ReadinessTask>,
    /// Work that is recommended soon.
    pub recommended_soon: Vec<ReadinessTask>,
    /// Work that is optional later.
    pub optional_later: Vec<ReadinessTask>,
}

impl ReadinessBuckets {
    /// Creates empty readiness buckets.
    pub const fn new() -> Self {
        Self {
            blocking_now: Vec::new(),
            recommended_soon: Vec::new(),
            optional_later: Vec::new(),
        }
    }

    /// Returns a copy with a task inserted in the task's declared bucket.
    pub fn with_task(mut self, task: ReadinessTask) -> Self {
        match task.bucket {
            ReadinessBucket::BlockingNow => self.blocking_now.push(task),
            ReadinessBucket::RecommendedSoon => self.recommended_soon.push(task),
            ReadinessBucket::OptionalLater => self.optional_later.push(task),
        }
        self
    }

    /// Returns true when any readiness task exists.
    pub fn has_any_task(&self) -> bool {
        !(self.blocking_now.is_empty()
            && self.recommended_soon.is_empty()
            && self.optional_later.is_empty())
    }

    /// Returns the flattened task list.
    pub fn all_tasks(&self) -> impl Iterator<Item = &ReadinessTask> {
        self.blocking_now
            .iter()
            .chain(self.recommended_soon.iter())
            .chain(self.optional_later.iter())
    }

    /// Builds a summary preserving bucket separation.
    pub fn summary(&self) -> ReadinessBucketSummary {
        ReadinessBucketSummary {
            blocking_now_total: self.blocking_now.len(),
            blocking_now_by_reason: count_blocked_reasons(&self.blocking_now),
            recommended_soon_total: self.recommended_soon.len(),
            recommended_soon_by_class: count_task_classes(&self.recommended_soon),
            optional_later_total: self.optional_later.len(),
            optional_later_by_reason: count_optional_reasons(&self.optional_later),
        }
    }
}

/// Per-reason count for blocking readiness work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedReasonSummaryEntry {
    /// Blocked reason class.
    pub blocked_reason_class: BlockedReasonClass,
    /// Count of tasks for the reason.
    pub task_count: usize,
}

/// Per-class count for recommended readiness work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendedClassSummaryEntry {
    /// Readiness task class.
    pub readiness_task_class: ReadinessTaskClass,
    /// Count of tasks for the class.
    pub task_count: usize,
}

/// Per-reason count for optional readiness work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalReasonSummaryEntry {
    /// Optional reason class.
    pub optional_reason_class: OptionalReasonClass,
    /// Count of tasks for the reason.
    pub task_count: usize,
}

/// Summary of readiness buckets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadinessBucketSummary {
    /// Number of blocking tasks.
    pub blocking_now_total: usize,
    /// Blocking task counts by reason.
    pub blocking_now_by_reason: Vec<BlockedReasonSummaryEntry>,
    /// Number of recommended tasks.
    pub recommended_soon_total: usize,
    /// Recommended task counts by class.
    pub recommended_soon_by_class: Vec<RecommendedClassSummaryEntry>,
    /// Number of optional tasks.
    pub optional_later_total: usize,
    /// Optional task counts by reason.
    pub optional_later_by_reason: Vec<OptionalReasonSummaryEntry>,
}

/// Root identity class anchored by a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootIdentityClass {
    /// Local filesystem path identity.
    FilesystemPathIdentity,
    /// Repository root identity.
    RepoRootIdentity,
    /// Workspace manifest identity.
    WorkspaceManifestIdentity,
    /// Workset manifest identity.
    WorksetManifestIdentity,
    /// Remote target identity.
    RemoteTargetIdentity,
    /// Managed cloud identity.
    ManagedCloudIdentity,
    /// Handoff packet identity.
    HandoffPacketIdentity,
    /// Template or prebuild identity.
    TemplateOrPrebuildIdentity,
    /// No root identity has been anchored yet.
    NoRootIdentityYet,
}

impl_as_str!(RootIdentityClass {
    FilesystemPathIdentity => "filesystem_path_identity",
    RepoRootIdentity => "repo_root_identity",
    WorkspaceManifestIdentity => "workspace_manifest_identity",
    WorksetManifestIdentity => "workset_manifest_identity",
    RemoteTargetIdentity => "remote_target_identity",
    ManagedCloudIdentity => "managed_cloud_identity",
    HandoffPacketIdentity => "handoff_packet_identity",
    TemplateOrPrebuildIdentity => "template_or_prebuild_identity",
    NoRootIdentityYet => "no_root_identity_yet",
});

/// Trust review state exposed by a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustReviewClass {
    /// No trust review is required.
    NoTrustReviewRequired,
    /// Trust review is pending.
    TrustReviewPending,
    /// Trust review is in progress.
    TrustReviewInProgress,
    /// Trust review is blocked by policy.
    TrustReviewBlockedByPolicy,
    /// Trust revalidation is required.
    TrustRevalidationRequired,
    /// Trust review does not apply.
    TrustReviewNotApplicable,
}

impl_as_str!(TrustReviewClass {
    NoTrustReviewRequired => "no_trust_review_required",
    TrustReviewPending => "trust_review_pending",
    TrustReviewInProgress => "trust_review_in_progress",
    TrustReviewBlockedByPolicy => "trust_review_blocked_by_policy",
    TrustRevalidationRequired => "trust_revalidation_required",
    TrustReviewNotApplicable => "trust_review_not_applicable",
});

/// Admission class for the post-entry checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionClass {
    /// The activation is admitted.
    Admitted,
    /// Trust review is required.
    TrustReviewRequired,
    /// Policy blocks admission.
    PolicyBlocked,
    /// Repair is required.
    NeedsRepair,
    /// Reconnect is required.
    NeedsReconnect,
    /// Reauthentication is required.
    NeedsReauth,
}

impl_as_str!(AdmissionClass {
    Admitted => "admitted",
    TrustReviewRequired => "trust_review_required",
    PolicyBlocked => "policy_blocked",
    NeedsRepair => "needs_repair",
    NeedsReconnect => "needs_reconnect",
    NeedsReauth => "needs_reauth",
});

/// Source class for recommendations surfaced by a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeRecommendationSourceClass {
    /// Recommendation comes from detected facts.
    DetectedFacts,
    /// Recommendation comes from heuristic inference.
    HeuristicInference,
    /// Recommendation comes from bundle metadata.
    BundleMetadata,
    /// Recommendation comes from admin policy.
    AdminPolicy,
    /// Recommendation comes from a prior user choice.
    PriorUserChoice,
    /// Recommendation comes from an extension contribution.
    ExtensionContribution,
    /// Recommendation comes from a template default.
    TemplateDefault,
    /// Recommendation comes from an import packet.
    ImportPacket,
    /// Recommendation mixes multiple source classes.
    MixedRecommendationSource,
}

impl_as_str!(ArchetypeRecommendationSourceClass {
    DetectedFacts => "detected_facts",
    HeuristicInference => "heuristic_inference",
    BundleMetadata => "bundle_metadata",
    AdminPolicy => "admin_policy",
    PriorUserChoice => "prior_user_choice",
    ExtensionContribution => "extension_contribution",
    TemplateDefault => "template_default",
    ImportPacket => "import_packet",
    MixedRecommendationSource => "mixed_recommendation_source",
});

/// Setup location class summarized by a checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupLocationClass {
    /// Local machine setup.
    LocalMachine,
    /// Container setup.
    Container,
    /// Devcontainer setup.
    Devcontainer,
    /// Remote agent setup.
    RemoteAgent,
    /// Managed workspace setup.
    ManagedWorkspace,
    /// Browser handoff setup.
    BrowserHandoff,
    /// No setup execution.
    NoExecution,
    /// Setup crosses multiple locations.
    MixedSetupLocation,
}

impl_as_str!(SetupLocationClass {
    LocalMachine => "local_machine",
    Container => "container",
    Devcontainer => "devcontainer",
    RemoteAgent => "remote_agent",
    ManagedWorkspace => "managed_workspace",
    BrowserHandoff => "browser_handoff",
    NoExecution => "no_execution",
    MixedSetupLocation => "mixed_setup_location",
});

/// Safe fallback exposed instead of setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinueWithoutClass {
    /// Defer setup.
    SetUpLater,
    /// Open minimal.
    OpenMinimal,
    /// Open plain explorer.
    OpenPlainExplorer,
    /// Continue in restricted mode.
    ContinueInRestrictedMode,
    /// Inspect only.
    InspectOnly,
    /// Compare before restore.
    CompareBeforeRestore,
    /// Dismiss recommendation.
    DismissRecommendation,
    /// No safe continue-without action is available.
    NoContinueWithoutActionAvailable,
}

impl_as_str!(ContinueWithoutClass {
    SetUpLater => "set_up_later",
    OpenMinimal => "open_minimal",
    OpenPlainExplorer => "open_plain_explorer",
    ContinueInRestrictedMode => "continue_in_restricted_mode",
    InspectOnly => "inspect_only",
    CompareBeforeRestore => "compare_before_restore",
    DismissRecommendation => "dismiss_recommendation",
    NoContinueWithoutActionAvailable => "no_continue_without_action_available",
});

impl ContinueWithoutClass {
    /// Converts this fallback to the shared admission action vocabulary when possible.
    pub const fn to_admission_action(self) -> Option<AdmissionAction> {
        match self {
            Self::SetUpLater => Some(AdmissionAction::SetUpLater),
            Self::OpenMinimal => Some(AdmissionAction::OpenMinimal),
            Self::InspectOnly => Some(AdmissionAction::InspectOnly),
            Self::OpenPlainExplorer
            | Self::ContinueInRestrictedMode
            | Self::CompareBeforeRestore
            | Self::DismissRecommendation
            | Self::NoContinueWithoutActionAvailable => None,
        }
    }
}

/// Landing surface selected by first-useful-work routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandingSurface {
    /// File editor with root-discovery cues.
    FileEditorWithRootCues,
    /// Explorer plus README or changed files.
    ExplorerPlusReadmeOrChangedFiles,
    /// Post-clone handoff surface.
    PostCloneHandoff,
    /// Linked review, incident, or work item surface.
    LinkedReviewIncidentOrWorkItem,
    /// Restored layout with placeholders.
    RestoredLayoutWithPlaceholders,
    /// Import compare or restore sheet.
    ImportCompareOrRestoreSheet,
    /// Generic shell with diagnostics.
    GenericShellWithDiagnostics,
    /// Nested-root choice sheet.
    NestedRootChoiceSheet,
}

impl_as_str!(LandingSurface {
    FileEditorWithRootCues => "file_editor_with_root_cues",
    ExplorerPlusReadmeOrChangedFiles => "explorer_plus_readme_or_changed_files",
    PostCloneHandoff => "post_clone_handoff",
    LinkedReviewIncidentOrWorkItem => "linked_review_incident_or_work_item",
    RestoredLayoutWithPlaceholders => "restored_layout_with_placeholders",
    ImportCompareOrRestoreSheet => "import_compare_or_restore_sheet",
    GenericShellWithDiagnostics => "generic_shell_with_diagnostics",
    NestedRootChoiceSheet => "nested_root_choice_sheet",
});

/// Reason first-useful-work routing selected a landing surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteReasonClass {
    /// Standalone file route.
    StandaloneFile,
    /// Repository evidence route.
    RepoEvidence,
    /// Clone was materialized and setup was not run.
    CloneMaterializedNoSetupRun,
    /// Linked object route.
    LinkedObjectArrival,
    /// Restore provenance route.
    RestoreProvenance,
    /// Imported packet review route.
    ImportedPacketReview,
    /// Policy or trust narrowing route.
    PolicyOrTrustNarrowing,
    /// Mixed workspace boundary choice route.
    MixedWorkspaceBoundaryChoice,
    /// Unknown generic safe default route.
    UnknownGenericSafeDefault,
    /// Missing prerequisite limited-ready route.
    MissingPrerequisiteLimitedReady,
}

impl_as_str!(RouteReasonClass {
    StandaloneFile => "standalone_file",
    RepoEvidence => "repo_evidence",
    CloneMaterializedNoSetupRun => "clone_materialized_no_setup_run",
    LinkedObjectArrival => "linked_object_arrival",
    RestoreProvenance => "restore_provenance",
    ImportedPacketReview => "imported_packet_review",
    PolicyOrTrustNarrowing => "policy_or_trust_narrowing",
    MixedWorkspaceBoundaryChoice => "mixed_workspace_boundary_choice",
    UnknownGenericSafeDefault => "unknown_generic_safe_default",
    MissingPrerequisiteLimitedReady => "missing_prerequisite_limited_ready",
});

/// Reversible switch option near the first landing surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteSwitchOption {
    /// Open plain explorer.
    OpenPlainExplorer,
    /// Open last file.
    OpenLastFile,
    /// Open README.
    OpenReadme,
    /// Open changed files.
    OpenChangedFiles,
    /// Review trust.
    ReviewTrust,
    /// Choose root or workset.
    ChooseRootOrWorkset,
    /// Compare import.
    CompareImport,
    /// Open minimal.
    OpenMinimal,
    /// Set up later.
    SetUpLater,
    /// Open clone or local checkout.
    OpenCloneOrLocalCheckout,
    /// Open Project Doctor.
    OpenProjectDoctor,
}

impl_as_str!(RouteSwitchOption {
    OpenPlainExplorer => "open_plain_explorer",
    OpenLastFile => "open_last_file",
    OpenReadme => "open_readme",
    OpenChangedFiles => "open_changed_files",
    ReviewTrust => "review_trust",
    ChooseRootOrWorkset => "choose_root_or_workset",
    CompareImport => "compare_import",
    OpenMinimal => "open_minimal",
    SetUpLater => "set_up_later",
    OpenCloneOrLocalCheckout => "open_clone_or_local_checkout",
    OpenProjectDoctor => "open_project_doctor",
});

/// Effect of remembered routing on the current route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RememberedRoutingEffect {
    /// No remembered route was applied.
    NotRemembered,
    /// Remembered routing narrows only.
    NarrowingHintOnly,
    /// Remembered routing expired and requires review.
    ExpiredRequiresReview,
    /// Remembered routing is blocked by policy.
    BlockedByPolicy,
}

impl_as_str!(RememberedRoutingEffect {
    NotRemembered => "not_remembered",
    NarrowingHintOnly => "narrowing_hint_only",
    ExpiredRequiresReview => "expired_requires_review",
    BlockedByPolicy => "blocked_by_policy",
});

/// Explicit mixed-root or mixed-stack boundary choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MixedWorkspaceBoundaryChoice {
    /// Open the whole repository.
    OpenWholeRepo,
    /// Open the probable project.
    OpenProbableProject,
    /// Open the current folder only.
    OpenCurrentFolderOnly,
    /// Create a workset or slice.
    CreateWorksetOrSlice,
}

impl_as_str!(MixedWorkspaceBoundaryChoice {
    OpenWholeRepo => "open_whole_repo",
    OpenProbableProject => "open_probable_project",
    OpenCurrentFolderOnly => "open_current_folder_only",
    CreateWorksetOrSlice => "create_workset_or_slice",
});

/// First-useful-work route selected after admission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstUsefulWorkRoute {
    /// Entry source that triggered the route.
    pub entry_source: FirstUsefulEntrySource,
    /// Landing surface selected by the route.
    pub landing_surface: LandingSurface,
    /// Reason the landing surface was selected.
    pub route_reason_class: RouteReasonClass,
    /// Whether the plain open path is still available.
    pub plain_open_available: bool,
    /// Reversible switch options.
    pub switch_options: Vec<RouteSwitchOption>,
    /// Whether this route can be remembered.
    pub rememberable: bool,
    /// Current effect of remembered routing.
    pub remembered_routing_effect: RememberedRoutingEffect,
    /// Redacted reviewer-facing summary.
    pub summary: String,
}

/// Post-entry workspace-admission checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAdmissionCheckpoint {
    /// Stable checkpoint schema version.
    pub admission_checkpoint_schema_version: u32,
    /// Opaque checkpoint id.
    pub admission_checkpoint_id: String,
    /// Entry action reference from the reviewed entry packet.
    pub entry_action_ref: String,
    /// Entry source used for route selection.
    pub entry_source: FirstUsefulEntrySource,
    /// Target kind admitted by the entry surface.
    pub target_kind: TargetKind,
    /// Resulting mode admitted by the entry surface.
    pub resulting_mode: ResultingMode,
    /// Root identity class.
    pub root_identity_class: RootIdentityClass,
    /// Opaque root identity reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_identity_ref: Option<String>,
    /// Opaque workspace scope reference.
    pub workspace_scope_ref: String,
    /// Current trust state.
    pub trust_state: TrustState,
    /// Trust review class.
    pub trust_review_class: TrustReviewClass,
    /// Admission class.
    pub admission_class: AdmissionClass,
    /// Recommendation source classes.
    pub archetype_recommendation_source_classes: Vec<ArchetypeRecommendationSourceClass>,
    /// Setup location classes.
    pub setup_location_classes: Vec<SetupLocationClass>,
    /// Readiness bucket summary.
    pub readiness_bucket_summary: ReadinessBucketSummary,
    /// Safe fallback class.
    pub continue_without_class: ContinueWithoutClass,
    /// Whether plain open remains available.
    pub plain_open_available: bool,
    /// Whether ordinary editing remains available.
    pub ordinary_editing_available: bool,
    /// Whether blocked setup stays distinct from optional guidance.
    pub blocked_setup_distinct_from_optional: bool,
    /// Optional archetype reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archetype_ref: Option<String>,
    /// Compatible bundle references.
    pub compatible_bundle_refs: Vec<String>,
    /// Detected fact references.
    pub detected_fact_refs: Vec<String>,
    /// Recommendation references.
    pub recommendation_refs: Vec<String>,
    /// Policy block references.
    pub policy_block_refs: Vec<String>,
    /// Blocking setup references.
    pub blocked_setup_refs: Vec<String>,
    /// Optional guidance references.
    pub optional_guidance_refs: Vec<String>,
    /// Admission card references.
    pub admission_card_refs: Vec<String>,
    /// Admission banner reference.
    pub admission_banner_ref: String,
    /// Redacted reviewer-facing summary.
    pub summary: String,
}

/// Request to build a checkpoint and first-useful route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionCheckpointBuildRequest {
    /// Reviewed admission packet from the entry surface.
    pub admission_review: AdmissionReviewPacket,
    /// Entry action reference for support and UI cross-links.
    pub entry_action_ref: String,
    /// Entry source used for route selection.
    pub entry_source: FirstUsefulEntrySource,
    /// Workspace scope reference.
    pub workspace_scope_ref: Option<String>,
    /// Trust state for the admitted target.
    pub trust_state: TrustState,
    /// Trust review class.
    pub trust_review_class: TrustReviewClass,
    /// Admission class.
    pub admission_class: AdmissionClass,
    /// Archetype truth for this entry.
    pub archetype: ArchetypeTruth,
    /// Readiness buckets for this entry.
    pub readiness: ReadinessBuckets,
    /// Safe fallback class.
    pub continue_without_class: ContinueWithoutClass,
    /// Whether plain open remains available.
    pub plain_open_available: bool,
    /// Whether ordinary editing remains available.
    pub ordinary_editing_available: bool,
    /// Explicit mixed-workspace boundary choices.
    pub boundary_choices: Vec<MixedWorkspaceBoundaryChoice>,
    /// Same-weight bypasses shown with setup recommendations.
    pub same_weight_bypass_actions: Vec<ContinueWithoutClass>,
    /// Whether the route may be remembered.
    pub rememberable: bool,
    /// Current effect of remembered routing.
    pub remembered_routing_effect: RememberedRoutingEffect,
}

impl AdmissionCheckpointBuildRequest {
    /// Builds a request from the reviewed admission packet and archetype truth.
    pub fn new(
        admission_review: AdmissionReviewPacket,
        entry_action_ref: impl Into<String>,
        entry_source: FirstUsefulEntrySource,
        archetype: ArchetypeTruth,
    ) -> Self {
        Self {
            admission_review,
            entry_action_ref: entry_action_ref.into(),
            entry_source,
            workspace_scope_ref: None,
            trust_state: TrustState::PendingEvaluation,
            trust_review_class: TrustReviewClass::TrustReviewPending,
            admission_class: AdmissionClass::Admitted,
            archetype,
            readiness: ReadinessBuckets::new(),
            continue_without_class: ContinueWithoutClass::SetUpLater,
            plain_open_available: true,
            ordinary_editing_available: true,
            boundary_choices: Vec::new(),
            same_weight_bypass_actions: vec![
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::OpenMinimal,
                ContinueWithoutClass::DismissRecommendation,
            ],
            rememberable: false,
            remembered_routing_effect: RememberedRoutingEffect::NotRemembered,
        }
    }

    /// Sets readiness buckets.
    pub fn with_readiness(mut self, readiness: ReadinessBuckets) -> Self {
        self.readiness = readiness;
        self
    }

    /// Sets the trust state and review class.
    pub const fn with_trust(
        mut self,
        trust_state: TrustState,
        trust_review_class: TrustReviewClass,
    ) -> Self {
        self.trust_state = trust_state;
        self.trust_review_class = trust_review_class;
        self
    }

    /// Sets the admission class.
    pub const fn with_admission_class(mut self, admission_class: AdmissionClass) -> Self {
        self.admission_class = admission_class;
        self
    }

    /// Sets the safe fallback class.
    pub const fn with_continue_without(
        mut self,
        continue_without_class: ContinueWithoutClass,
    ) -> Self {
        self.continue_without_class = continue_without_class;
        self
    }

    /// Sets availability flags for plain open and ordinary editing.
    pub const fn with_availability(
        mut self,
        plain_open_available: bool,
        ordinary_editing_available: bool,
    ) -> Self {
        self.plain_open_available = plain_open_available;
        self.ordinary_editing_available = ordinary_editing_available;
        self
    }

    /// Sets explicit mixed-boundary choices.
    pub fn with_boundary_choices(mut self, choices: Vec<MixedWorkspaceBoundaryChoice>) -> Self {
        self.boundary_choices = choices;
        self
    }

    /// Sets same-weight bypass actions.
    pub fn with_same_weight_bypass_actions(mut self, actions: Vec<ContinueWithoutClass>) -> Self {
        self.same_weight_bypass_actions = actions;
        self
    }

    /// Sets workspace scope reference.
    pub fn with_workspace_scope_ref(mut self, workspace_scope_ref: impl Into<String>) -> Self {
        self.workspace_scope_ref = Some(workspace_scope_ref.into());
        self
    }

    /// Sets remembered-routing behavior.
    pub const fn with_remembered_routing(
        mut self,
        rememberable: bool,
        remembered_routing_effect: RememberedRoutingEffect,
    ) -> Self {
        self.rememberable = rememberable;
        self.remembered_routing_effect = remembered_routing_effect;
        self
    }
}

/// Checkpoint and route record consumed by shell surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionCheckpointRouteRecord {
    /// Record kind.
    pub record_kind: AdmissionCheckpointRouteRecordKind,
    /// Schema version.
    pub admission_checkpoint_route_schema_version: u32,
    /// Opaque route record id.
    pub route_record_id: String,
    /// Reviewed admission packet id this record consumes.
    pub admission_review_id: String,
    /// Post-entry checkpoint.
    pub checkpoint: WorkspaceAdmissionCheckpoint,
    /// Archetype truth with source-labeled signals.
    pub archetype: ArchetypeTruth,
    /// Readiness buckets, not flattened.
    pub readiness: ReadinessBuckets,
    /// First useful work route.
    pub first_useful_route: FirstUsefulWorkRoute,
    /// Mixed-root or mixed-stack choices.
    pub boundary_choices: Vec<MixedWorkspaceBoundaryChoice>,
    /// Same-weight bypass actions shown with setup recommendations.
    pub same_weight_bypass_actions: Vec<ContinueWithoutClass>,
    /// Whether detection may auto-install setup.
    pub auto_install_allowed: bool,
    /// Whether detection may auto-trust the workspace.
    pub auto_trust_allowed: bool,
    /// Whether the route preserves the user's entry intent.
    pub entry_intent_preserved: bool,
}

impl AdmissionCheckpointRouteRecord {
    /// Returns contract findings; an empty list means the record obeys the lane invariants.
    pub fn contract_findings(&self) -> Vec<String> {
        let mut findings = Vec::new();
        if self.auto_install_allowed {
            findings.push("auto_install_allowed must remain false".to_string());
        }
        if self.auto_trust_allowed {
            findings.push("auto_trust_allowed must remain false".to_string());
        }
        if !self.entry_intent_preserved {
            findings.push("entry intent must be preserved".to_string());
        }
        if self.archetype.signals.is_empty() {
            findings.push("archetype outcome must carry a detection signal or reason".to_string());
        }
        if self.checkpoint.readiness_bucket_summary.blocking_now_total
            != self.readiness.blocking_now.len()
        {
            findings.push("blocking_now summary does not match readiness tasks".to_string());
        }
        if self
            .checkpoint
            .readiness_bucket_summary
            .recommended_soon_total
            != self.readiness.recommended_soon.len()
        {
            findings.push("recommended_soon summary does not match readiness tasks".to_string());
        }
        if self
            .checkpoint
            .readiness_bucket_summary
            .optional_later_total
            != self.readiness.optional_later.len()
        {
            findings.push("optional_later summary does not match readiness tasks".to_string());
        }
        if !self.checkpoint.blocked_setup_distinct_from_optional {
            findings.push("blocked setup must stay distinct from optional guidance".to_string());
        }
        if self.readiness.has_any_task() || !self.archetype.recommendation_refs.is_empty() {
            for required in [
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::OpenMinimal,
                ContinueWithoutClass::DismissRecommendation,
            ] {
                if !self.same_weight_bypass_actions.contains(&required) {
                    findings.push(format!(
                        "same-weight bypass actions must include {}",
                        required.as_str()
                    ));
                }
            }
        }
        if matches!(
            self.archetype.outcome,
            DetectionOutcome::CertifiedArchetypeMatch | DetectionOutcome::ProbableArchetype
        ) && self.archetype.evidence_freshness.is_empty()
        {
            findings.push(
                "certified and probable archetype states must expose evidence freshness"
                    .to_string(),
            );
        }
        if self.archetype.outcome == DetectionOutcome::MixedOrAmbiguousWorkspace {
            for required in [
                MixedWorkspaceBoundaryChoice::OpenWholeRepo,
                MixedWorkspaceBoundaryChoice::OpenProbableProject,
                MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
                MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
            ] {
                if !self.boundary_choices.contains(&required) {
                    findings.push(format!(
                        "mixed workspace choices must include {}",
                        required.as_str()
                    ));
                }
            }
        }
        if !confidence_matches_outcome(self.archetype.outcome, self.archetype.confidence_class) {
            findings.push("archetype confidence does not match detection outcome".to_string());
        }
        if self.first_useful_route.entry_source != self.checkpoint.entry_source {
            findings.push("first useful route entry source must match checkpoint".to_string());
        }
        findings
    }

    /// Returns true when the record obeys the lane invariants.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }

    /// Returns compact rows for support, CLI, or shell review surfaces.
    pub fn compact_lines(&self) -> Vec<String> {
        let signal_sources = self
            .archetype
            .signals
            .iter()
            .map(|signal| signal.source_class.as_str())
            .collect::<Vec<_>>()
            .join(" + ");
        let evidence = self
            .archetype
            .evidence_freshness
            .iter()
            .map(|row| {
                let reviewed = row.reviewed_on.as_deref().unwrap_or("unknown_review_date");
                format!(
                    "{}:{}:reviewed_on={}",
                    row.evidence_ref,
                    row.freshness_class.as_str(),
                    reviewed
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let bypasses = self
            .same_weight_bypass_actions
            .iter()
            .map(|action| action.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let switches = self
            .first_useful_route
            .switch_options
            .iter()
            .map(|option| option.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        vec![
            format!(
                "admission_checkpoint: {} entry={} target={} resulting={} trust={} admission={}",
                self.checkpoint.admission_checkpoint_id,
                self.checkpoint.entry_source.as_str(),
                self.checkpoint.target_kind.as_str(),
                self.checkpoint.resulting_mode.as_str(),
                self.checkpoint.trust_state.as_str(),
                self.checkpoint.admission_class.as_str()
            ),
            format!(
                "archetype: {} confidence={} support={} detector={} sources={}",
                self.archetype.outcome.family_label(),
                self.archetype.confidence_class.as_str(),
                self.archetype.support_claim_class.as_str(),
                self.archetype.detector_state.as_str(),
                signal_sources
            ),
            format!("evidence_freshness: [{}]", evidence),
            format!(
                "readiness: blocking_now={} recommended_soon={} optional_later={} continue_without={}",
                self.checkpoint.readiness_bucket_summary.blocking_now_total,
                self.checkpoint.readiness_bucket_summary.recommended_soon_total,
                self.checkpoint.readiness_bucket_summary.optional_later_total,
                self.checkpoint.continue_without_class.as_str()
            ),
            format!(
                "route: landing={} reason={} switches=[{}]",
                self.first_useful_route.landing_surface.as_str(),
                self.first_useful_route.route_reason_class.as_str(),
                switches
            ),
            format!(
                "same_weight_bypass=[{}] auto_install={} auto_trust={} entry_intent_preserved={}",
                bypasses,
                self.auto_install_allowed,
                self.auto_trust_allowed,
                self.entry_intent_preserved
            ),
        ]
    }
}

/// Builds a checkpoint and first-useful-work route from a reviewed admission packet.
pub fn build_admission_checkpoint_route(
    request: AdmissionCheckpointBuildRequest,
) -> AdmissionCheckpointRouteRecord {
    let root_identity_class = root_identity_class_for(
        request.admission_review.target_kind,
        request.admission_class,
        request.entry_source,
    );
    let root_identity_ref =
        (root_identity_class != RootIdentityClass::NoRootIdentityYet).then(|| {
            request
                .admission_review
                .normalized_target_identity
                .identity_ref
                .clone()
        });
    let workspace_scope_ref = request.workspace_scope_ref.clone().unwrap_or_else(|| {
        format!(
            "scope:{:016x}",
            stable_hash(
                &request
                    .admission_review
                    .normalized_target_identity
                    .identity_ref
            )
        )
    });
    let readiness_summary = request.readiness.summary();
    let route = route_for(&request);
    let recommendation_sources = recommendation_sources_for(&request.archetype);
    let setup_locations = setup_locations_for(&request.readiness);
    let checkpoint_id = format!(
        "admission.checkpoint.{:016x}",
        stable_hash(&format!(
            "{}\n{}\n{}\n{}",
            request.entry_action_ref,
            request.admission_review.admission_review_id,
            request.archetype.outcome.as_str(),
            workspace_scope_ref
        ))
    );
    let banner_ref = format!("banner.{}", checkpoint_id);
    let admission_card_refs = admission_card_refs(&checkpoint_id, &request.readiness);
    let blocked_setup_refs = request
        .readiness
        .blocking_now
        .iter()
        .map(|task| task.task_ref.clone())
        .collect::<Vec<_>>();
    let optional_guidance_refs = request
        .readiness
        .optional_later
        .iter()
        .map(|task| task.task_ref.clone())
        .collect::<Vec<_>>();

    let checkpoint = WorkspaceAdmissionCheckpoint {
        admission_checkpoint_schema_version: WORKSPACE_ADMISSION_CHECKPOINT_SCHEMA_VERSION,
        admission_checkpoint_id: checkpoint_id.clone(),
        entry_action_ref: request.entry_action_ref,
        entry_source: request.entry_source,
        target_kind: request.admission_review.target_kind,
        resulting_mode: request.admission_review.resulting_mode,
        root_identity_class,
        root_identity_ref,
        workspace_scope_ref,
        trust_state: request.trust_state,
        trust_review_class: request.trust_review_class,
        admission_class: request.admission_class,
        archetype_recommendation_source_classes: recommendation_sources,
        setup_location_classes: setup_locations,
        readiness_bucket_summary: readiness_summary,
        continue_without_class: request.continue_without_class,
        plain_open_available: request.plain_open_available,
        ordinary_editing_available: request.ordinary_editing_available,
        blocked_setup_distinct_from_optional: true,
        archetype_ref: request.archetype.archetype_ref.clone(),
        compatible_bundle_refs: request.archetype.compatible_bundle_refs.clone(),
        detected_fact_refs: request.archetype.detected_fact_refs.clone(),
        recommendation_refs: request.archetype.recommendation_refs.clone(),
        policy_block_refs: request.archetype.policy_block_refs.clone(),
        blocked_setup_refs,
        optional_guidance_refs,
        admission_card_refs,
        admission_banner_ref: banner_ref,
        summary: checkpoint_summary(
            request.entry_source,
            request.archetype.outcome,
            request.continue_without_class,
        ),
    };

    AdmissionCheckpointRouteRecord {
        record_kind: AdmissionCheckpointRouteRecordKind::AdmissionCheckpointRouteRecord,
        admission_checkpoint_route_schema_version: ADMISSION_CHECKPOINT_ROUTE_SCHEMA_VERSION,
        route_record_id: format!("admission.route.{:016x}", stable_hash(&checkpoint_id)),
        admission_review_id: request.admission_review.admission_review_id,
        checkpoint,
        archetype: request.archetype,
        readiness: request.readiness,
        first_useful_route: route,
        boundary_choices: request.boundary_choices,
        same_weight_bypass_actions: request.same_weight_bypass_actions,
        auto_install_allowed: false,
        auto_trust_allowed: false,
        entry_intent_preserved: true,
    }
}

fn route_for(request: &AdmissionCheckpointBuildRequest) -> FirstUsefulWorkRoute {
    let (landing_surface, mut route_reason_class, mut switch_options) = match request.entry_source {
        FirstUsefulEntrySource::SingleFileOpen => (
            LandingSurface::FileEditorWithRootCues,
            RouteReasonClass::StandaloneFile,
            vec![
                RouteSwitchOption::OpenPlainExplorer,
                RouteSwitchOption::OpenCloneOrLocalCheckout,
            ],
        ),
        FirstUsefulEntrySource::FolderOrRepoOpen => match request.archetype.outcome {
            DetectionOutcome::MixedOrAmbiguousWorkspace => (
                LandingSurface::NestedRootChoiceSheet,
                RouteReasonClass::MixedWorkspaceBoundaryChoice,
                vec![
                    RouteSwitchOption::ChooseRootOrWorkset,
                    RouteSwitchOption::OpenPlainExplorer,
                    RouteSwitchOption::OpenReadme,
                ],
            ),
            DetectionOutcome::UnknownOrGenericWorkspace => (
                LandingSurface::GenericShellWithDiagnostics,
                RouteReasonClass::UnknownGenericSafeDefault,
                vec![
                    RouteSwitchOption::OpenPlainExplorer,
                    RouteSwitchOption::OpenReadme,
                    RouteSwitchOption::OpenProjectDoctor,
                ],
            ),
            _ => (
                LandingSurface::ExplorerPlusReadmeOrChangedFiles,
                RouteReasonClass::RepoEvidence,
                vec![
                    RouteSwitchOption::OpenPlainExplorer,
                    RouteSwitchOption::OpenReadme,
                    RouteSwitchOption::OpenChangedFiles,
                    RouteSwitchOption::ReviewTrust,
                ],
            ),
        },
        FirstUsefulEntrySource::RepositoryClone => (
            LandingSurface::PostCloneHandoff,
            RouteReasonClass::CloneMaterializedNoSetupRun,
            vec![
                RouteSwitchOption::ReviewTrust,
                RouteSwitchOption::SetUpLater,
                RouteSwitchOption::OpenPlainExplorer,
            ],
        ),
        FirstUsefulEntrySource::ReviewOrIncidentDeepLink => (
            LandingSurface::LinkedReviewIncidentOrWorkItem,
            RouteReasonClass::LinkedObjectArrival,
            vec![
                RouteSwitchOption::OpenCloneOrLocalCheckout,
                RouteSwitchOption::OpenProjectDoctor,
                RouteSwitchOption::OpenMinimal,
            ],
        ),
        FirstUsefulEntrySource::RestoreLastSession => (
            LandingSurface::RestoredLayoutWithPlaceholders,
            RouteReasonClass::RestoreProvenance,
            vec![
                RouteSwitchOption::OpenMinimal,
                RouteSwitchOption::OpenPlainExplorer,
                RouteSwitchOption::OpenProjectDoctor,
            ],
        ),
        FirstUsefulEntrySource::ImportedStateOrHandoffPacket => (
            LandingSurface::ImportCompareOrRestoreSheet,
            RouteReasonClass::ImportedPacketReview,
            vec![
                RouteSwitchOption::CompareImport,
                RouteSwitchOption::OpenMinimal,
                RouteSwitchOption::OpenPlainExplorer,
            ],
        ),
    };

    match request.archetype.outcome {
        DetectionOutcome::RestrictedOrPolicyBlocked => {
            route_reason_class = RouteReasonClass::PolicyOrTrustNarrowing;
            switch_options.push(RouteSwitchOption::OpenMinimal);
            switch_options.push(RouteSwitchOption::OpenProjectDoctor);
        }
        DetectionOutcome::MissingPrerequisite => {
            route_reason_class = RouteReasonClass::MissingPrerequisiteLimitedReady;
            switch_options.push(RouteSwitchOption::OpenProjectDoctor);
        }
        _ => {}
    }
    dedupe_switch_options(&mut switch_options);

    FirstUsefulWorkRoute {
        entry_source: request.entry_source,
        landing_surface,
        route_reason_class,
        plain_open_available: request.plain_open_available,
        switch_options,
        rememberable: request.rememberable,
        remembered_routing_effect: request.remembered_routing_effect,
        summary: route_summary(request.entry_source, landing_surface, route_reason_class),
    }
}

fn confidence_matches_outcome(
    outcome: DetectionOutcome,
    confidence: DetectionConfidenceClass,
) -> bool {
    match outcome {
        DetectionOutcome::CertifiedArchetypeMatch => {
            confidence == DetectionConfidenceClass::CertifiedExact
        }
        DetectionOutcome::ProbableArchetype => matches!(
            confidence,
            DetectionConfidenceClass::HighProbable | DetectionConfidenceClass::MediumProbable
        ),
        DetectionOutcome::MixedOrAmbiguousWorkspace => {
            confidence == DetectionConfidenceClass::MixedConflicting
        }
        DetectionOutcome::UnknownOrGenericWorkspace => {
            confidence == DetectionConfidenceClass::GenericUnknown
        }
        DetectionOutcome::RestrictedOrPolicyBlocked => {
            confidence == DetectionConfidenceClass::RestrictedByPolicy
        }
        DetectionOutcome::MissingPrerequisite => {
            confidence == DetectionConfidenceClass::PrerequisiteMissing
        }
    }
}

fn root_identity_class_for(
    target_kind: TargetKind,
    admission_class: AdmissionClass,
    entry_source: FirstUsefulEntrySource,
) -> RootIdentityClass {
    if matches!(
        admission_class,
        AdmissionClass::NeedsRepair | AdmissionClass::NeedsReconnect | AdmissionClass::NeedsReauth
    ) && entry_source == FirstUsefulEntrySource::ReviewOrIncidentDeepLink
    {
        return RootIdentityClass::NoRootIdentityYet;
    }

    match target_kind {
        TargetKind::LocalFile | TargetKind::LocalFolder => {
            RootIdentityClass::FilesystemPathIdentity
        }
        TargetKind::LocalRepoRoot => RootIdentityClass::RepoRootIdentity,
        TargetKind::WorkspaceManifest => RootIdentityClass::WorkspaceManifestIdentity,
        TargetKind::WorksetManifest => RootIdentityClass::WorksetManifestIdentity,
        TargetKind::ManagedCloudWorkspace => RootIdentityClass::ManagedCloudIdentity,
        TargetKind::RemoteRepository
        | TargetKind::SshWorkspace
        | TargetKind::ContainerWorkspace
        | TargetKind::DevcontainerWorkspace => RootIdentityClass::RemoteTargetIdentity,
        TargetKind::PortableStatePackage
        | TargetKind::HandoffPacket
        | TargetKind::CompetitorConfigRoot => RootIdentityClass::HandoffPacketIdentity,
        TargetKind::TemplateOrPrebuildSnapshot => RootIdentityClass::TemplateOrPrebuildIdentity,
        TargetKind::ReviewOrWorkItemDeepLink => RootIdentityClass::NoRootIdentityYet,
        TargetKind::RecoveryCheckpoint => RootIdentityClass::WorkspaceManifestIdentity,
    }
}

fn recommendation_sources_for(
    archetype: &ArchetypeTruth,
) -> Vec<ArchetypeRecommendationSourceClass> {
    let mut sources = Vec::new();
    for signal in &archetype.signals {
        let source = match signal.source_class {
            DetectionSignalSourceClass::BundleMarker => {
                ArchetypeRecommendationSourceClass::BundleMetadata
            }
            DetectionSignalSourceClass::AdminPolicy => {
                ArchetypeRecommendationSourceClass::AdminPolicy
            }
            DetectionSignalSourceClass::ExtensionContribution => {
                ArchetypeRecommendationSourceClass::ExtensionContribution
            }
            DetectionSignalSourceClass::PreviousUserChoice => {
                ArchetypeRecommendationSourceClass::PriorUserChoice
            }
            DetectionSignalSourceClass::ImportPacket => {
                ArchetypeRecommendationSourceClass::ImportPacket
            }
            DetectionSignalSourceClass::Manifest
            | DetectionSignalSourceClass::WorkspaceFile
            | DetectionSignalSourceClass::FilesystemLayout
            | DetectionSignalSourceClass::Lockfile
            | DetectionSignalSourceClass::RuntimeProbe
            | DetectionSignalSourceClass::VcsMetadata => {
                ArchetypeRecommendationSourceClass::DetectedFacts
            }
        };
        push_unique(&mut sources, source);
    }
    if archetype.outcome == DetectionOutcome::MixedOrAmbiguousWorkspace {
        push_unique(
            &mut sources,
            ArchetypeRecommendationSourceClass::MixedRecommendationSource,
        );
    }
    sources
}

fn setup_locations_for(readiness: &ReadinessBuckets) -> Vec<SetupLocationClass> {
    let mut locations = Vec::new();
    for task in readiness.all_tasks() {
        let location = match task.execution_boundary {
            ExecutionBoundary::LocalMachine => SetupLocationClass::LocalMachine,
            ExecutionBoundary::Container => SetupLocationClass::Container,
            ExecutionBoundary::RemoteAgent => SetupLocationClass::RemoteAgent,
            ExecutionBoundary::ManagedWorkspace => SetupLocationClass::ManagedWorkspace,
            ExecutionBoundary::BrowserHandoff => SetupLocationClass::BrowserHandoff,
            ExecutionBoundary::NoExecution => SetupLocationClass::NoExecution,
        };
        push_unique(&mut locations, location);
    }
    if locations.len() > 1 {
        push_unique(&mut locations, SetupLocationClass::MixedSetupLocation);
    }
    locations
}

fn admission_card_refs(checkpoint_id: &str, readiness: &ReadinessBuckets) -> Vec<String> {
    let mut refs = vec![format!("card.{checkpoint_id}.what_works_now")];
    if !readiness.blocking_now.is_empty() {
        refs.push(format!("card.{checkpoint_id}.blocked_setup"));
    }
    if !readiness.recommended_soon.is_empty() {
        refs.push(format!("card.{checkpoint_id}.recommended_setup"));
    }
    if !readiness.optional_later.is_empty() {
        refs.push(format!("card.{checkpoint_id}.optional_later"));
    }
    refs.push(format!("card.{checkpoint_id}.continue_without"));
    refs
}

fn count_blocked_reasons(tasks: &[ReadinessTask]) -> Vec<BlockedReasonSummaryEntry> {
    let mut counts: Vec<BlockedReasonSummaryEntry> = Vec::new();
    for task in tasks {
        let reason = task
            .blocked_reason_class
            .unwrap_or_else(|| default_blocked_reason(task.state));
        match counts
            .iter_mut()
            .find(|entry| entry.blocked_reason_class == reason)
        {
            Some(entry) => entry.task_count += 1,
            None => counts.push(BlockedReasonSummaryEntry {
                blocked_reason_class: reason,
                task_count: 1,
            }),
        }
    }
    counts
}

fn count_task_classes(tasks: &[ReadinessTask]) -> Vec<RecommendedClassSummaryEntry> {
    let mut counts: Vec<RecommendedClassSummaryEntry> = Vec::new();
    for task in tasks {
        match counts
            .iter_mut()
            .find(|entry| entry.readiness_task_class == task.task_class)
        {
            Some(entry) => entry.task_count += 1,
            None => counts.push(RecommendedClassSummaryEntry {
                readiness_task_class: task.task_class,
                task_count: 1,
            }),
        }
    }
    counts
}

fn count_optional_reasons(tasks: &[ReadinessTask]) -> Vec<OptionalReasonSummaryEntry> {
    let mut counts: Vec<OptionalReasonSummaryEntry> = Vec::new();
    for task in tasks {
        let reason = task
            .optional_reason_class
            .unwrap_or(OptionalReasonClass::OptionalAdditive);
        match counts
            .iter_mut()
            .find(|entry| entry.optional_reason_class == reason)
        {
            Some(entry) => entry.task_count += 1,
            None => counts.push(OptionalReasonSummaryEntry {
                optional_reason_class: reason,
                task_count: 1,
            }),
        }
    }
    counts
}

fn default_blocked_reason(state: ReadinessTaskState) -> BlockedReasonClass {
    match state {
        ReadinessTaskState::BlockedByPolicy => BlockedReasonClass::BlockedByPolicy,
        ReadinessTaskState::BlockedByTrust => BlockedReasonClass::BlockedByTrust,
        ReadinessTaskState::MissingPrerequisite | ReadinessTaskState::Unavailable => {
            BlockedReasonClass::BlockedByMissingPrerequisite
        }
        ReadinessTaskState::Ready
        | ReadinessTaskState::Pending
        | ReadinessTaskState::DeferredByUser
        | ReadinessTaskState::Optional => BlockedReasonClass::BlockedByTrust,
    }
}

fn checkpoint_summary(
    entry_source: FirstUsefulEntrySource,
    outcome: DetectionOutcome,
    continue_without: ContinueWithoutClass,
) -> String {
    format!(
        "{} entry uses {} detection and preserves {} as the safe fallback.",
        entry_source.as_str(),
        outcome.family_label(),
        continue_without.as_str()
    )
}

fn route_summary(
    entry_source: FirstUsefulEntrySource,
    landing_surface: LandingSurface,
    route_reason_class: RouteReasonClass,
) -> String {
    format!(
        "{} lands on {} because {}.",
        entry_source.as_str(),
        landing_surface.as_str(),
        route_reason_class.as_str()
    )
}

fn dedupe_switch_options(options: &mut Vec<RouteSwitchOption>) {
    let mut deduped = Vec::new();
    for option in options.drain(..) {
        push_unique(&mut deduped, option);
    }
    *options = deduped;
}

fn push_unique<T: PartialEq>(values: &mut Vec<T>, value: T) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn stable_hash(value: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        review_entry_admission, AdmissionReviewRequest, AdmissionSourceSurface, EntryVerb,
    };
    use std::path::Path;

    #[test]
    fn certified_repo_keeps_recommendations_distinct_from_optional_work() {
        let admission_review = review_entry_admission(AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Open,
            TargetKind::LocalRepoRoot,
            ResultingMode::RepoRoot,
            "~/Code/web-app",
        ));
        let archetype = ArchetypeTruth::new(
            DetectionOutcome::CertifiedArchetypeMatch,
            DetectionConfidenceClass::CertifiedExact,
            SupportClaimClass::CertifiedCurrent,
            DetectorState::ReadyEnough,
            vec![
                DetectionSignal::new(
                    "signal.web.manifest",
                    DetectionSignalSourceClass::Manifest,
                    vec![
                        SignalMaterialEffect::SupportClaim,
                        SignalMaterialEffect::RouteSelection,
                    ],
                    "Project manifest matches the web launch wedge.",
                ),
                DetectionSignal::new(
                    "signal.web.bundle_marker",
                    DetectionSignalSourceClass::BundleMarker,
                    vec![SignalMaterialEffect::Recommendation],
                    "Bundle marker points to a compatible reviewed bundle.",
                ),
            ],
        )
        .with_archetype_ref("archetype.ts_web_app.certified")
        .with_compatible_bundle_refs(vec!["bundle.web.fullstack.current".to_string()])
        .with_evidence_freshness(vec![DetectionEvidenceFreshness::new(
            "evidence.web.certified_scorecard",
            SignalFreshnessClass::FreshCurrent,
            "Certified web archetype evidence is current.",
        )
        .with_review_window(Some("2026-05-15".to_string()), Some("P21D".to_string()))])
        .with_detected_fact_refs(vec!["fact.web.manifest_present".to_string()])
        .with_recommendation_refs(vec!["rec.web.compare_bundle".to_string()]);
        let readiness = ReadinessBuckets::new()
            .with_task(
                ReadinessTask::new(
                    "task.web.dependency_restore",
                    ReadinessTaskClass::DependencyRestore,
                    ReadinessBucket::RecommendedSoon,
                    ReadinessTaskState::Pending,
                    ExecutionBoundary::LocalMachine,
                    vec![
                        SideEffectClass::ReadsWorkspace,
                        SideEffectClass::DownloadsDependencies,
                    ],
                    "Dependency restore is recommended, but editing is available now.",
                )
                .with_source_signal_refs(vec!["signal.web.manifest".to_string()])
                .with_recommendation_refs(vec!["rec.web.compare_bundle".to_string()]),
            )
            .with_task(
                ReadinessTask::new(
                    "task.web.extension_recommendation",
                    ReadinessTaskClass::ExtensionRecommendation,
                    ReadinessBucket::OptionalLater,
                    ReadinessTaskState::Optional,
                    ExecutionBoundary::NoExecution,
                    vec![SideEffectClass::NoSideEffect],
                    "Extension recommendations are optional and dismissable.",
                )
                .with_optional_reason(OptionalReasonClass::OptionalRecommendedOnly),
            );

        let record = build_admission_checkpoint_route(
            AdmissionCheckpointBuildRequest::new(
                admission_review,
                "entry.action.open.web_repo",
                FirstUsefulEntrySource::FolderOrRepoOpen,
                archetype,
            )
            .with_readiness(readiness)
            .with_continue_without(ContinueWithoutClass::SetUpLater),
        );

        assert_eq!(
            record.first_useful_route.landing_surface,
            LandingSurface::ExplorerPlusReadmeOrChangedFiles
        );
        assert_eq!(
            record
                .checkpoint
                .readiness_bucket_summary
                .blocking_now_total,
            0
        );
        assert_eq!(
            record
                .checkpoint
                .readiness_bucket_summary
                .recommended_soon_total,
            1
        );
        assert_eq!(
            record
                .checkpoint
                .readiness_bucket_summary
                .optional_later_total,
            1
        );
        assert!(record.checkpoint.ordinary_editing_available);
        assert!(
            record.is_contract_valid(),
            "{:?}",
            record.contract_findings()
        );
    }

    #[test]
    fn mixed_workspace_requires_explicit_boundary_choices() {
        let admission_review = review_entry_admission(AdmissionReviewRequest::new(
            AdmissionSourceSurface::StartCenter,
            EntryVerb::Open,
            TargetKind::LocalFolder,
            ResultingMode::WorkspaceCandidate,
            "~/Code/monorepo",
        ));
        let archetype = ArchetypeTruth::new(
            DetectionOutcome::MixedOrAmbiguousWorkspace,
            DetectionConfidenceClass::MixedConflicting,
            SupportClaimClass::GenericNoClaim,
            DetectorState::Partial,
            vec![
                DetectionSignal::new(
                    "signal.mixed.layout",
                    DetectionSignalSourceClass::FilesystemLayout,
                    vec![SignalMaterialEffect::RouteSelection],
                    "Nested root signals conflict.",
                ),
                DetectionSignal::new(
                    "signal.mixed.manifest",
                    DetectionSignalSourceClass::Manifest,
                    vec![SignalMaterialEffect::Readiness],
                    "Multiple stack manifests are present.",
                ),
            ],
        )
        .with_detected_fact_refs(vec![
            "fact.mixed.multi_root_signals".to_string(),
            "fact.mixed.mixed_stack_signal".to_string(),
        ])
        .with_recommendation_refs(vec!["rec.mixed.choose_boundary".to_string()]);
        let readiness = ReadinessBuckets::new().with_task(
            ReadinessTask::new(
                "task.mixed.user_boundary_choice",
                ReadinessTaskClass::UserBoundaryChoice,
                ReadinessBucket::BlockingNow,
                ReadinessTaskState::BlockedByTrust,
                ExecutionBoundary::NoExecution,
                vec![SideEffectClass::NoSideEffect],
                "Choose whole repo, probable project, current folder only, or a workset.",
            )
            .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
        );

        let record = build_admission_checkpoint_route(
            AdmissionCheckpointBuildRequest::new(
                admission_review,
                "entry.action.open.mixed_folder",
                FirstUsefulEntrySource::FolderOrRepoOpen,
                archetype,
            )
            .with_readiness(readiness)
            .with_continue_without(ContinueWithoutClass::OpenMinimal)
            .with_boundary_choices(vec![
                MixedWorkspaceBoundaryChoice::OpenWholeRepo,
                MixedWorkspaceBoundaryChoice::OpenProbableProject,
                MixedWorkspaceBoundaryChoice::OpenCurrentFolderOnly,
                MixedWorkspaceBoundaryChoice::CreateWorksetOrSlice,
            ]),
        );

        assert_eq!(
            record.first_useful_route.landing_surface,
            LandingSurface::NestedRootChoiceSheet
        );
        assert_eq!(
            record.first_useful_route.route_reason_class,
            RouteReasonClass::MixedWorkspaceBoundaryChoice
        );
        assert!(
            record.is_contract_valid(),
            "{:?}",
            record.contract_findings()
        );
    }

    #[test]
    fn missing_prerequisite_carries_runtime_probe_reason_without_auto_trust() {
        let admission_review = review_entry_admission(AdmissionReviewRequest::new(
            AdmissionSourceSurface::DeepLink,
            EntryVerb::Open,
            TargetKind::ReviewOrWorkItemDeepLink,
            ResultingMode::InspectOnly,
            "review link packet",
        ));
        let archetype = ArchetypeTruth::new(
            DetectionOutcome::MissingPrerequisite,
            DetectionConfidenceClass::PrerequisiteMissing,
            SupportClaimClass::ClaimUnavailableMissingPrerequisite,
            DetectorState::Blocked,
            vec![DetectionSignal::new(
                "signal.deep_link.remote_probe",
                DetectionSignalSourceClass::RuntimeProbe,
                vec![
                    SignalMaterialEffect::Readiness,
                    SignalMaterialEffect::RouteSelection,
                ],
                "Remote agent probe did not find an attachable agent.",
            )],
        )
        .with_detected_fact_refs(vec!["fact.deep_link.remote_agent_missing".to_string()]);
        let readiness = ReadinessBuckets::new().with_task(
            ReadinessTask::new(
                "task.deep_link.remote_agent_reconnect",
                ReadinessTaskClass::RemoteAgentReconnect,
                ReadinessBucket::BlockingNow,
                ReadinessTaskState::MissingPrerequisite,
                ExecutionBoundary::RemoteAgent,
                vec![
                    SideEffectClass::ContactsRemote,
                    SideEffectClass::AttachesRuntime,
                ],
                "Live actions wait for the remote agent; inspect-only remains available.",
            )
            .with_blocked_reason(BlockedReasonClass::BlockedByMissingPrerequisite),
        );

        let record = build_admission_checkpoint_route(
            AdmissionCheckpointBuildRequest::new(
                admission_review,
                "entry.action.deep_link.review",
                FirstUsefulEntrySource::ReviewOrIncidentDeepLink,
                archetype,
            )
            .with_readiness(readiness)
            .with_admission_class(AdmissionClass::NeedsReconnect)
            .with_trust(
                TrustState::PendingEvaluation,
                TrustReviewClass::TrustReviewNotApplicable,
            )
            .with_continue_without(ContinueWithoutClass::InspectOnly)
            .with_availability(true, false),
        );

        assert_eq!(
            record.checkpoint.root_identity_class,
            RootIdentityClass::NoRootIdentityYet
        );
        assert_eq!(
            record.first_useful_route.route_reason_class,
            RouteReasonClass::MissingPrerequisiteLimitedReady
        );
        assert!(!record.auto_trust_allowed);
        assert!(!record.auto_install_allowed);
        assert!(
            record.is_contract_valid(),
            "{:?}",
            record.contract_findings()
        );
    }

    #[test]
    fn route_fixtures_round_trip_and_preserve_contract() {
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/admission_checkpoint");
        let entries = std::fs::read_dir(&root).expect("fixture dir must exist");
        let mut seen = 0usize;
        for entry in entries {
            let path = entry.expect("fixture entry").path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            seen += 1;
            let payload = std::fs::read_to_string(&path).expect("fixture reads");
            let record: AdmissionCheckpointRouteRecord =
                serde_json::from_str(&payload).expect("fixture parses");
            assert_eq!(
                record.record_kind,
                AdmissionCheckpointRouteRecordKind::AdmissionCheckpointRouteRecord
            );
            assert_eq!(
                record.admission_checkpoint_route_schema_version,
                ADMISSION_CHECKPOINT_ROUTE_SCHEMA_VERSION
            );
            assert!(
                record.is_contract_valid(),
                "{:?}: {:?}",
                path,
                record.contract_findings()
            );
        }
        assert!(seen >= 3, "expected admission-checkpoint route fixtures");
    }
}
