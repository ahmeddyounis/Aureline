//! Reviewed project-entry admission packets.
//!
//! The admission packet is the shared review object used after an entry verb
//! resolves and before any disk write, trust transition, setup execution, or
//! current-workspace mutation is allowed to proceed.

pub mod checkpoint;

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{EntryVerb, ResultingMode, TargetKind};

/// Schema version for [`AdmissionReviewPacket`].
pub const ADMISSION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Identifies an `admission_review_packet_record`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionReviewRecordKind {
    /// `admission_review_packet_record`
    AdmissionReviewPacketRecord,
}

/// Surface that initiated the reviewed admission flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionSourceSurface {
    /// Start Center primary or secondary entry surface.
    StartCenter,
    /// Command palette or command-preview surface.
    CommandPalette,
    /// Drag-and-drop preview surface.
    DragAndDrop,
    /// Operating-system file association or "open with" handoff.
    SystemFileAssociation,
    /// Product-owned deep-link intent review.
    DeepLink,
    /// CLI or headless preview/commit path.
    CliHeadless,
    /// In-workspace switcher or add-root surface.
    WorkspaceSwitcher,
}

impl AdmissionSourceSurface {
    /// Returns the stable snake_case token for this source surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::DragAndDrop => "drag_and_drop",
            Self::SystemFileAssociation => "system_file_association",
            Self::DeepLink => "deep_link",
            Self::CliHeadless => "cli_headless",
            Self::WorkspaceSwitcher => "workspace_switcher",
        }
    }
}

/// Request to build one admission packet for a resolved entry activation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmissionReviewRequest {
    /// Surface that initiated the reviewed admission flow.
    pub source_surface: AdmissionSourceSurface,
    /// Entry verb resolved by the entry-flow model.
    pub entry_verb: EntryVerb,
    /// Target kind resolved by the entry-flow model.
    pub target_kind: TargetKind,
    /// Resulting mode resolved before commit.
    pub resulting_mode: ResultingMode,
    /// Redaction-aware target specifier selected by the user or handoff.
    pub target_specifier: String,
    /// Optional destination that would receive new bytes or workspace scope.
    pub destination: Option<String>,
    /// Optional active workspace label when the flow mutates the current context.
    pub active_workspace_label: Option<String>,
    /// Optional network route note, such as a proxy or mirror label.
    pub network_route_label: Option<String>,
}

impl AdmissionReviewRequest {
    /// Builds a request with the required entry-flow fields.
    pub fn new(
        source_surface: AdmissionSourceSurface,
        entry_verb: EntryVerb,
        target_kind: TargetKind,
        resulting_mode: ResultingMode,
        target_specifier: impl Into<String>,
    ) -> Self {
        Self {
            source_surface,
            entry_verb,
            target_kind,
            resulting_mode,
            target_specifier: target_specifier.into(),
            destination: None,
            active_workspace_label: None,
            network_route_label: None,
        }
    }

    /// Sets the destination label used by clone, import, add-root, and restore flows.
    pub fn with_destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }

    /// Sets the active workspace label used when a flow changes the current workspace.
    pub fn with_active_workspace(mut self, active_workspace_label: impl Into<String>) -> Self {
        self.active_workspace_label = Some(active_workspace_label.into());
        self
    }

    /// Sets a proxy, mirror, or route label for clone review.
    pub fn with_network_route(mut self, network_route_label: impl Into<String>) -> Self {
        self.network_route_label = Some(network_route_label.into());
        self
    }
}

/// Normalized target identity shown before an admission commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedTargetIdentity {
    /// Identity class used by shell and support surfaces.
    pub identity_class: TargetIdentityClass,
    /// Redaction-aware label for the normalized target.
    pub normalized_label: String,
    /// Opaque reference that downstream support packets can cite.
    pub identity_ref: String,
}

/// Class of target identity anchored by an admission packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetIdentityClass {
    /// Local filesystem path identity.
    FilesystemPath,
    /// Repository root identity.
    RepoRoot,
    /// Workspace or workset manifest identity.
    WorkspaceManifest,
    /// Remote repository identity.
    RemoteRepository,
    /// Import or archive artifact identity.
    ImportArtifact,
    /// Handoff or portable-state packet identity.
    HandoffPacket,
    /// Patch-like artifact identity.
    PatchArtifact,
    /// Recovery checkpoint identity.
    RecoveryCheckpoint,
    /// Identity is not yet anchored.
    Unresolved,
}

impl TargetIdentityClass {
    /// Returns the stable snake_case token for this identity class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilesystemPath => "filesystem_path",
            Self::RepoRoot => "repo_root",
            Self::WorkspaceManifest => "workspace_manifest",
            Self::RemoteRepository => "remote_repository",
            Self::ImportArtifact => "import_artifact",
            Self::HandoffPacket => "handoff_packet",
            Self::PatchArtifact => "patch_artifact",
            Self::RecoveryCheckpoint => "recovery_checkpoint",
            Self::Unresolved => "unresolved",
        }
    }
}

/// Destination truth shown before bytes or workspace scope change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationReview {
    /// Redaction-aware destination label.
    pub destination_label: String,
    /// Disposition for the destination if the user commits.
    pub disposition: DestinationDisposition,
    /// Whether destination review is required before a write-bearing commit.
    pub review_required_before_write: bool,
    /// Whether staging is visible and named rather than implied.
    pub staging_or_temporary_location_disclosed: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Destination disposition for a reviewed admission flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationDisposition {
    /// No write will occur.
    NoWrite,
    /// Only inspect; no durable destination is mutated.
    InspectOnly,
    /// Bytes land in labelled non-durable staging.
    WriteToLabelledStaging,
    /// Bytes land at an explicit user-selected destination after review.
    WriteToUserDestination,
    /// The active workspace root set changes after review.
    AddToCurrentWorkspace,
    /// Existing durable content is reused after review.
    ReuseExistingDestination,
    /// A recovery or restore state is applied after review.
    RestoreState,
}

impl DestinationDisposition {
    /// Returns the stable snake_case token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWrite => "no_write",
            Self::InspectOnly => "inspect_only",
            Self::WriteToLabelledStaging => "write_to_labelled_staging",
            Self::WriteToUserDestination => "write_to_user_destination",
            Self::AddToCurrentWorkspace => "add_to_current_workspace",
            Self::ReuseExistingDestination => "reuse_existing_destination",
            Self::RestoreState => "restore_state",
        }
    }
}

/// Write-scope truth shown before entry commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteScopeReview {
    /// Class of write scope for the admission flow.
    pub write_scope_class: WriteScopeClass,
    /// Redaction-aware affected scope label.
    pub affected_scope_label: String,
    /// Proposed write or scope changes.
    pub proposed_items: Vec<WriteScopeItem>,
    /// Whether the flow creates or requires a recovery checkpoint.
    pub checkpoint_required: bool,
    /// Named undo group or checkpoint label when durable workspace state mutates.
    pub recovery_checkpoint_or_undo_group: Option<String>,
}

/// Write-scope class for entry admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteScopeClass {
    /// No durable write; inspect/read-only entry.
    ReadOnlyInspect,
    /// Open a local target without writing project bytes.
    OpenExistingTarget,
    /// Materialize a cloned repository at a reviewed destination.
    CloneMaterialization,
    /// Extract or compare imported content.
    ImportExtraction,
    /// Add a root to the current workspace object model.
    AddRootWorkspaceMutation,
    /// Restore workspace/session state.
    RestoreStateMutation,
}

impl WriteScopeClass {
    /// Returns the stable snake_case token for this write-scope class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyInspect => "read_only_inspect",
            Self::OpenExistingTarget => "open_existing_target",
            Self::CloneMaterialization => "clone_materialization",
            Self::ImportExtraction => "import_extraction",
            Self::AddRootWorkspaceMutation => "add_root_workspace_mutation",
            Self::RestoreStateMutation => "restore_state_mutation",
        }
    }
}

/// One proposed item in a write-scope review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteScopeItem {
    /// Kind of proposed scope item.
    pub item_kind: WriteScopeItemKind,
    /// Redaction-aware target reference.
    pub target_ref: String,
    /// Human-readable action label.
    pub action_label: String,
}

/// Kind of proposed item in a write-scope review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteScopeItemKind {
    /// Existing target will be opened or inspected.
    OpenExisting,
    /// A clone root directory would be created.
    CreateCloneRoot,
    /// Archive or packet content would be extracted.
    ExtractImportedContent,
    /// Current workspace root set would widen.
    AddWorkspaceRoot,
    /// Restore or checkpoint state would be applied.
    ApplyRestoreState,
    /// No write-bearing item is proposed.
    None,
}

impl WriteScopeItemKind {
    /// Returns the stable snake_case token for this item kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExisting => "open_existing",
            Self::CreateCloneRoot => "create_clone_root",
            Self::ExtractImportedContent => "extract_imported_content",
            Self::AddWorkspaceRoot => "add_workspace_root",
            Self::ApplyRestoreState => "apply_restore_state",
            Self::None => "none",
        }
    }
}

/// Trust and setup gates that admission surfaces must preserve.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustAndSetupReview {
    /// No entry path silently grants workspace trust.
    pub no_silent_trust_grant: bool,
    /// No entry path silently executes setup.
    pub no_setup_execution: bool,
    /// No entry path silently runs repo hooks, tasks, or templates.
    pub no_task_or_hook_execution: bool,
    /// No hidden temporary directory materialization is implied.
    pub no_hidden_temporary_materialization: bool,
    /// Review summary for setup/trust posture.
    pub summary: String,
}

/// Recovery posture shown when a reviewed entry fails or is cancelled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPosture {
    /// Recovery path class for this admission flow.
    pub recovery_path_class: RecoveryPathClass,
    /// Safe recovery and follow-on actions.
    pub available_actions: Vec<AdmissionAction>,
    /// Typed input fields survive validation and runtime failure.
    pub typed_inputs_preserved_on_failure: bool,
    /// Diagnostics exposed to the user are redacted.
    pub redacted_diagnostics_on_failure: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Recovery path class for reviewed entry flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPathClass {
    /// Cancel leaves no durable change.
    CancelNoChange,
    /// User can inspect or reveal the existing target and choose a different destination.
    RevealOrChooseElsewhere,
    /// A checkpoint or undo group names a durable mutation.
    CheckpointOrUndoGroup,
    /// Import rollback checkpoint is retained.
    RollbackCheckpoint,
    /// Inspect-only path has no mutation to recover.
    InspectOnlyNoMutation,
    /// User can open minimal or set up later.
    OpenMinimalOrSetUpLater,
}

impl RecoveryPathClass {
    /// Returns the stable snake_case token for this recovery path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CancelNoChange => "cancel_no_change",
            Self::RevealOrChooseElsewhere => "reveal_or_choose_elsewhere",
            Self::CheckpointOrUndoGroup => "checkpoint_or_undo_group",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::InspectOnlyNoMutation => "inspect_only_no_mutation",
            Self::OpenMinimalOrSetUpLater => "open_minimal_or_set_up_later",
        }
    }
}

/// Action vocabulary shared across entry review and drag/drop transfer review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionAction {
    /// Open the reviewed target.
    Open,
    /// Import reviewed artifact content.
    Import,
    /// Add the target as a workspace root.
    AddRoot,
    /// Clone into the reviewed destination.
    CloneHere,
    /// Split the target into another editor group or window.
    Split,
    /// Copy the payload.
    Copy,
    /// Move the payload.
    Move,
    /// Clone but do not open afterward.
    CloneOnly,
    /// Clone and keep review before open.
    CloneAndReview,
    /// Clone and open after review.
    CloneAndOpen,
    /// Clone and add to the current workspace after review.
    CloneAndAdd,
    /// Reuse an existing destination.
    ReuseExisting,
    /// Add an existing target as a root.
    AddExistingAsRoot,
    /// Choose another clone destination.
    CloneElsewhere,
    /// Reveal the target in the system shell.
    RevealTarget,
    /// Inspect without durable mutation.
    InspectOnly,
    /// Open minimal without optional setup.
    OpenMinimal,
    /// Defer setup.
    SetUpLater,
    /// Cancel without durable change.
    Cancel,
}

impl AdmissionAction {
    /// Returns the stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Import => "import",
            Self::AddRoot => "add_root",
            Self::CloneHere => "clone_here",
            Self::Split => "split",
            Self::Copy => "copy",
            Self::Move => "move",
            Self::CloneOnly => "clone_only",
            Self::CloneAndReview => "clone_and_review",
            Self::CloneAndOpen => "clone_and_open",
            Self::CloneAndAdd => "clone_and_add",
            Self::ReuseExisting => "reuse_existing",
            Self::AddExistingAsRoot => "add_existing_as_root",
            Self::CloneElsewhere => "clone_elsewhere",
            Self::RevealTarget => "reveal_target",
            Self::InspectOnly => "inspect_only",
            Self::OpenMinimal => "open_minimal",
            Self::SetUpLater => "set_up_later",
            Self::Cancel => "cancel",
        }
    }
}

/// Follow-on state shown after materialization or admission review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowOnReview {
    /// Preferred post-entry action.
    pub post_entry_action: AdmissionAction,
    /// Side effects Aureline deliberately has not run.
    pub deliberately_not_run: Vec<DeliberateNonAction>,
    /// Summary for post-entry handoff surfaces.
    pub summary: String,
}

/// Side effect deliberately not run by an entry flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliberateNonAction {
    /// Workspace trust was not granted.
    TrustGrant,
    /// Dependency restore was not run.
    DependencyRestore,
    /// Repository tasks were not run.
    RepoTasks,
    /// Hooks were not run.
    RepoHooks,
    /// Extensions or bundles were not installed.
    ExtensionOrBundleInstall,
    /// Template/bootstrap work was not run.
    TemplateOrBootstrap,
    /// Runtime attach was not started.
    RuntimeAttach,
}

impl DeliberateNonAction {
    /// Returns the stable snake_case token for this non-action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustGrant => "trust_grant",
            Self::DependencyRestore => "dependency_restore",
            Self::RepoTasks => "repo_tasks",
            Self::RepoHooks => "repo_hooks",
            Self::ExtensionOrBundleInstall => "extension_or_bundle_install",
            Self::TemplateOrBootstrap => "template_or_bootstrap",
            Self::RuntimeAttach => "runtime_attach",
        }
    }
}

/// Clone-specific review fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneAdmissionReview {
    /// Redaction-aware normalized remote URL label.
    pub normalized_remote_label: String,
    /// Host label extracted from the remote URL.
    pub host_label: String,
    /// Host or certificate posture.
    pub certificate_posture: CertificatePosture,
    /// Authentication mode posture.
    pub auth_mode: CloneAuthMode,
    /// Branch or ref posture.
    pub ref_choice: RefChoice,
    /// Submodule posture.
    pub submodule_posture: SubmodulePosture,
    /// Git LFS posture.
    pub lfs_posture: LfsPosture,
    /// Proxy or mirror route note.
    pub route_note: String,
    /// Explicit clone actions offered to the user.
    pub explicit_actions: Vec<AdmissionAction>,
}

/// Host or certificate posture for clone review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificatePosture {
    /// TLS uses the system trust store and will be verified by Git.
    TlsSystemTrustPending,
    /// SSH host key must be checked through the system Git/SSH boundary.
    SshHostKeyReviewRequired,
    /// A custom certificate or self-signed posture requires review.
    CertificateReviewRequired,
    /// No certificate posture applies to the source.
    NotApplicable,
}

impl CertificatePosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TlsSystemTrustPending => "tls_system_trust_pending",
            Self::SshHostKeyReviewRequired => "ssh_host_key_review_required",
            Self::CertificateReviewRequired => "certificate_review_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Authentication mode posture for clone review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloneAuthMode {
    /// SSH agent or configured SSH identity.
    SshAgent,
    /// OAuth or browser/device-code handoff may be required.
    #[serde(rename = "oauth_or_browser_handoff")]
    OAuthOrBrowserHandoff,
    /// Git credential helper or anonymous read.
    CredentialHelperOrAnonymous,
    /// Authentication mode is not yet known.
    UnknownUntilCredentialReview,
}

impl CloneAuthMode {
    /// Returns the stable snake_case token for this auth mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SshAgent => "ssh_agent",
            Self::OAuthOrBrowserHandoff => "oauth_or_browser_handoff",
            Self::CredentialHelperOrAnonymous => "credential_helper_or_anonymous",
            Self::UnknownUntilCredentialReview => "unknown_until_credential_review",
        }
    }
}

/// Branch or ref posture for clone review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefChoice {
    /// Use the remote's default branch.
    DefaultBranch,
    /// Use a user-pinned branch.
    UserSelectedBranch,
    /// Use a user-pinned ref or commit.
    UserSelectedRef,
    /// Ref has not been resolved yet.
    UnresolvedUntilRemoteQuery,
}

impl RefChoice {
    /// Returns the stable snake_case token for this ref choice.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultBranch => "default_branch",
            Self::UserSelectedBranch => "user_selected_branch",
            Self::UserSelectedRef => "user_selected_ref",
            Self::UnresolvedUntilRemoteQuery => "unresolved_until_remote_query",
        }
    }
}

/// Submodule posture for clone review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmodulePosture {
    /// No submodules are requested.
    NotRequested,
    /// Submodules are detected later and not initialized by clone admission.
    DetectOnly,
    /// User explicitly requested submodule initialization.
    UserRequested,
}

impl SubmodulePosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::DetectOnly => "detect_only",
            Self::UserRequested => "user_requested",
        }
    }
}

/// Git LFS posture for clone review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LfsPosture {
    /// No LFS hydrate is requested.
    NotRequested,
    /// LFS pointer detection may happen without hydration.
    DetectOnly,
    /// User explicitly requested LFS hydration.
    UserRequestedHydration,
}

impl LfsPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::DetectOnly => "detect_only",
            Self::UserRequestedHydration => "user_requested_hydration",
        }
    }
}

/// Destination collision review for clone/import/add-root flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationCollisionReview {
    /// Collision class.
    pub collision_class: DestinationCollisionClass,
    /// Existing target identity label.
    pub existing_target_label: String,
    /// Whether an explicit user choice is required before writing.
    pub requires_explicit_choice: bool,
    /// Safe actions shown instead of a generic overwrite prompt.
    pub safe_actions: Vec<AdmissionAction>,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Destination collision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationCollisionClass {
    /// No destination collision.
    NoCollision,
    /// Existing non-empty path is not clearly reusable.
    ExistingPathNonEmpty,
    /// Existing path is already a repository root.
    ExistingRepoRoot,
    /// Existing path is a workspace manifest.
    ExistingWorkspaceFile,
    /// Destination is blocked by policy or permissions.
    DestinationBlocked,
}

impl DestinationCollisionClass {
    /// Returns the stable snake_case token for this collision class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCollision => "no_collision",
            Self::ExistingPathNonEmpty => "existing_path_non_empty",
            Self::ExistingRepoRoot => "existing_repo_root",
            Self::ExistingWorkspaceFile => "existing_workspace_file",
            Self::DestinationBlocked => "destination_blocked",
        }
    }
}

/// Import-specific review fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportAdmissionReview {
    /// Artifact class.
    pub artifact_class: ImportArtifactClass,
    /// Import action class.
    pub import_action: ImportAction,
    /// Schema or producer label shown to the user.
    pub schema_or_producer_label: String,
    /// Extraction or restore target label.
    pub extraction_or_restore_target_label: String,
    /// Machine-local exclusions disclosed before commit.
    pub machine_local_exclusions: Vec<String>,
    /// Cleanup posture.
    pub cleanup_posture: CleanupPosture,
    /// Whether any temporary staging location is explicitly labelled.
    pub temporary_staging_disclosed: bool,
}

/// Artifact class for import admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportArtifactClass {
    /// Portable state package.
    PortableStatePackage,
    /// Handoff packet.
    HandoffPacket,
    /// Competitor configuration root.
    CompetitorConfigRoot,
    /// Archive-like bundle.
    ArchiveBundle,
    /// Patch-like artifact.
    PatchArtifact,
    /// Workspace manifest bundle.
    WorkspaceManifestBundle,
    /// Unknown artifact class.
    Unknown,
}

impl ImportArtifactClass {
    /// Returns the stable snake_case token for this artifact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableStatePackage => "portable_state_package",
            Self::HandoffPacket => "handoff_packet",
            Self::CompetitorConfigRoot => "competitor_config_root",
            Self::ArchiveBundle => "archive_bundle",
            Self::PatchArtifact => "patch_artifact",
            Self::WorkspaceManifestBundle => "workspace_manifest_bundle",
            Self::Unknown => "unknown",
        }
    }
}

/// Import action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportAction {
    /// Inspect without extracting into a durable destination.
    InspectOnly,
    /// Extract and review before open.
    ExtractAndReview,
    /// Extract and open after review.
    ExtractAndOpen,
    /// Restore from a packet after compare/review.
    RestoreFromPacket,
    /// Add imported root to current workspace.
    AddToCurrentWorkspace,
    /// Compare before restore or apply.
    CompareBeforeRestore,
}

impl ImportAction {
    /// Returns the stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::ExtractAndReview => "extract_and_review",
            Self::ExtractAndOpen => "extract_and_open",
            Self::RestoreFromPacket => "restore_from_packet",
            Self::AddToCurrentWorkspace => "add_to_current_workspace",
            Self::CompareBeforeRestore => "compare_before_restore",
        }
    }
}

/// Cleanup posture for import admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupPosture {
    /// No cleanup is required.
    NoCleanupRequired,
    /// Labelled staging is retained for review/recovery.
    RetainLabelledStaging,
    /// Cleanup occurs only after reviewed success.
    CleanupAfterReviewedSuccess,
    /// Manual cleanup may be needed after failure.
    ManualCleanupMayBeRequired,
}

impl CleanupPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCleanupRequired => "no_cleanup_required",
            Self::RetainLabelledStaging => "retain_labelled_staging",
            Self::CleanupAfterReviewedSuccess => "cleanup_after_reviewed_success",
            Self::ManualCleanupMayBeRequired => "manual_cleanup_may_be_required",
        }
    }
}

/// Drag-and-drop admission request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragDropAdmissionRequest {
    /// Payload kind being dropped.
    pub payload_kind: DragDropPayloadKind,
    /// Redaction-aware payload label or specifier.
    pub payload_specifier: String,
    /// Optional destination label for the drop.
    pub destination: Option<String>,
    /// Optional active workspace label.
    pub active_workspace_label: Option<String>,
    /// Aggregate byte estimate for the drop, when known.
    pub aggregate_bytes: Option<u64>,
    /// Whether the drop target is inside an active workspace.
    pub target_inside_active_workspace: bool,
}

impl DragDropAdmissionRequest {
    /// Builds a drag/drop request.
    pub fn new(payload_kind: DragDropPayloadKind, payload_specifier: impl Into<String>) -> Self {
        Self {
            payload_kind,
            payload_specifier: payload_specifier.into(),
            destination: None,
            active_workspace_label: None,
            aggregate_bytes: None,
            target_inside_active_workspace: false,
        }
    }

    /// Sets a destination label for the drop.
    pub fn with_destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }

    /// Sets an active workspace label for current-workspace mutations.
    pub fn with_active_workspace(mut self, active_workspace_label: impl Into<String>) -> Self {
        self.active_workspace_label = Some(active_workspace_label.into());
        self.target_inside_active_workspace = true;
        self
    }

    /// Sets the aggregate byte estimate.
    pub const fn with_aggregate_bytes(mut self, aggregate_bytes: u64) -> Self {
        self.aggregate_bytes = Some(aggregate_bytes);
        self
    }
}

/// Payload kind for drag/drop admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DragDropPayloadKind {
    /// Single local file.
    File,
    /// Local folder.
    Folder,
    /// Local repository root.
    Repository,
    /// Workspace or workset manifest.
    WorkspaceFile,
    /// Patch-like artifact.
    Patch,
    /// Archive-like artifact.
    Archive,
    /// Remote repository URL.
    RemoteRepositoryUrl,
}

impl DragDropPayloadKind {
    /// Returns the stable snake_case token for this payload kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Folder => "folder",
            Self::Repository => "repository",
            Self::WorkspaceFile => "workspace_file",
            Self::Patch => "patch",
            Self::Archive => "archive",
            Self::RemoteRepositoryUrl => "remote_repository_url",
        }
    }
}

/// Drag/drop-specific review fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DragDropAdmissionReview {
    /// Dropped payload kind.
    pub payload_kind: DragDropPayloadKind,
    /// Advertised verb shown before commit.
    pub advertised_verb: AdmissionAction,
    /// Whether the drop uses the same admission model as explicit entry.
    pub uses_same_admission_model: bool,
    /// Transfer progress class.
    pub progress_class: TransferProgressClass,
    /// Cancel route label.
    pub cancel_action_label: String,
    /// Checkpoint or undo group for durable mutations.
    pub checkpoint_or_undo_group: Option<String>,
    /// Whether collision review is part of the same drop packet.
    pub collision_review_included: bool,
}

/// Transfer progress class for drag/drop and import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferProgressClass {
    /// No progress surface is needed.
    NotRequired,
    /// Inline progress is enough.
    InlineProgress,
    /// Durable progress and cancel are required.
    DurableProgressWithCancel,
}

impl TransferProgressClass {
    /// Returns the stable snake_case token for this progress class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::InlineProgress => "inline_progress",
            Self::DurableProgressWithCancel => "durable_progress_with_cancel",
        }
    }
}

/// Complete reviewed admission packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionReviewPacket {
    /// Stable record kind.
    pub record_kind: AdmissionReviewRecordKind,
    /// Schema version for this packet.
    pub admission_review_schema_version: u32,
    /// Opaque review id.
    pub admission_review_id: String,
    /// Surface that initiated the flow.
    pub source_surface: AdmissionSourceSurface,
    /// Entry verb being reviewed.
    pub entry_verb: EntryVerb,
    /// Target kind being reviewed.
    pub target_kind: TargetKind,
    /// Resulting mode reviewed before commit.
    pub resulting_mode: ResultingMode,
    /// Normalized target identity.
    pub normalized_target_identity: NormalizedTargetIdentity,
    /// Destination truth.
    pub destination_review: DestinationReview,
    /// Write-scope truth.
    pub write_scope: WriteScopeReview,
    /// Trust and setup guardrails.
    pub trust_and_setup_review: TrustAndSetupReview,
    /// Recovery posture.
    pub recovery_posture: RecoveryPosture,
    /// Follow-on handoff truth.
    pub follow_on_review: FollowOnReview,
    /// Clone-specific review, when applicable.
    pub clone_review: Option<CloneAdmissionReview>,
    /// Import-specific review, when applicable.
    pub import_review: Option<ImportAdmissionReview>,
    /// Drag/drop-specific review, when applicable.
    pub drag_drop_review: Option<DragDropAdmissionReview>,
    /// Collision review, when applicable.
    pub collision_review: Option<DestinationCollisionReview>,
    /// Summary for compact shell rows.
    pub summary: String,
}

impl AdmissionReviewPacket {
    /// Returns true when the packet blocks commit until a collision choice is made.
    pub fn requires_collision_choice(&self) -> bool {
        self.collision_review
            .as_ref()
            .is_some_and(|review| review.requires_explicit_choice)
    }

    /// Returns compact lines suitable for a shell review surface.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "identity: {} ({})",
                self.normalized_target_identity.normalized_label,
                self.normalized_target_identity.identity_class.as_str()
            ),
            format!(
                "destination: {} [{}]",
                self.destination_review.destination_label,
                self.destination_review.disposition.as_str()
            ),
            format!(
                "write_scope: {} -> {}",
                self.write_scope.write_scope_class.as_str(),
                self.write_scope.affected_scope_label
            ),
        ];

        if let Some(clone) = self.clone_review.as_ref() {
            lines.push(format!(
                "clone_review: host={} auth={} ref={} submodules={} lfs={} route={}",
                clone.host_label,
                clone.auth_mode.as_str(),
                clone.ref_choice.as_str(),
                clone.submodule_posture.as_str(),
                clone.lfs_posture.as_str(),
                clone.route_note
            ));
        }

        if let Some(import) = self.import_review.as_ref() {
            lines.push(format!(
                "import_review: artifact={} action={} cleanup={}",
                import.artifact_class.as_str(),
                import.import_action.as_str(),
                import.cleanup_posture.as_str()
            ));
        }

        if let Some(drop) = self.drag_drop_review.as_ref() {
            lines.push(format!(
                "drop_review: payload={} verb={} progress={}",
                drop.payload_kind.as_str(),
                drop.advertised_verb.as_str(),
                drop.progress_class.as_str()
            ));
        }

        if let Some(collision) = self.collision_review.as_ref() {
            let actions = collision
                .safe_actions
                .iter()
                .map(|action| action.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "collision: {} actions=[{}]",
                collision.collision_class.as_str(),
                actions
            ));
        }

        lines.push(format!(
            "not_run: trust={} setup={} tasks_hooks={} hidden_temp={}",
            self.trust_and_setup_review.no_silent_trust_grant,
            self.trust_and_setup_review.no_setup_execution,
            self.trust_and_setup_review.no_task_or_hook_execution,
            self.trust_and_setup_review
                .no_hidden_temporary_materialization
        ));
        lines.push(format!(
            "recovery: {} inputs_preserved={} diagnostics_redacted={}",
            self.recovery_posture.recovery_path_class.as_str(),
            self.recovery_posture.typed_inputs_preserved_on_failure,
            self.recovery_posture.redacted_diagnostics_on_failure
        ));
        lines
    }
}

/// Builds a reviewed admission packet for an explicit entry activation.
pub fn review_entry_admission(request: AdmissionReviewRequest) -> AdmissionReviewPacket {
    match request.entry_verb {
        EntryVerb::Clone => clone_admission_packet(request),
        EntryVerb::Import | EntryVerb::StartFromSnapshot => import_admission_packet(request),
        EntryVerb::AddRoot => add_root_admission_packet(request),
        EntryVerb::Restore | EntryVerb::Resume => restore_admission_packet(request),
        EntryVerb::Open => open_admission_packet(request),
    }
}

/// Builds a reviewed admission packet for a drag/drop activation.
pub fn review_drag_drop_admission(request: DragDropAdmissionRequest) -> AdmissionReviewPacket {
    let (entry_verb, target_kind, resulting_mode, advertised_verb) =
        drag_drop_entry_tuple(&request);
    let mut entry_request = AdmissionReviewRequest::new(
        AdmissionSourceSurface::DragAndDrop,
        entry_verb,
        target_kind,
        resulting_mode,
        request.payload_specifier.clone(),
    );
    if let Some(destination) = request.destination.clone() {
        entry_request = entry_request.with_destination(destination);
    }
    if let Some(active_workspace) = request.active_workspace_label.clone() {
        entry_request = entry_request.with_active_workspace(active_workspace);
    }

    let mut packet = review_entry_admission(entry_request);
    let progress_class = transfer_progress_class(request.aggregate_bytes);
    let checkpoint_or_undo_group = if matches!(
        packet.write_scope.write_scope_class,
        WriteScopeClass::AddRootWorkspaceMutation
            | WriteScopeClass::ImportExtraction
            | WriteScopeClass::RestoreStateMutation
    ) {
        Some(format!(
            "entry-drop-{}-{}",
            request.payload_kind.as_str(),
            packet.write_scope.write_scope_class.as_str()
        ))
    } else {
        None
    };

    packet.drag_drop_review = Some(DragDropAdmissionReview {
        payload_kind: request.payload_kind,
        advertised_verb,
        uses_same_admission_model: true,
        progress_class,
        cancel_action_label: if progress_class == TransferProgressClass::DurableProgressWithCancel {
            "Cancel transfer".to_string()
        } else {
            "Cancel drop".to_string()
        },
        checkpoint_or_undo_group,
        collision_review_included: packet.collision_review.is_some(),
    });
    packet.source_surface = AdmissionSourceSurface::DragAndDrop;
    packet.admission_review_id = review_id_for(&packet);
    packet.summary = format!(
        "Drop advertises {} and uses the same {} admission review before commit.",
        advertised_verb.as_str(),
        packet.entry_verb.as_str()
    );
    packet
}

/// Writes an admission review packet to a JSON file.
///
/// # Errors
///
/// Returns an I/O or serialization error when the packet cannot be written.
pub fn write_admission_review_log(
    packet: &AdmissionReviewPacket,
    path: impl AsRef<Path>,
) -> Result<(), AdmissionReviewLogError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(AdmissionReviewLogError::CreateDir)?;
    }
    let payload =
        serde_json::to_string_pretty(packet).map_err(AdmissionReviewLogError::Serialize)?;
    std::fs::write(path, payload).map_err(AdmissionReviewLogError::Write)
}

/// Errors returned when writing admission review logs.
#[derive(Debug)]
pub enum AdmissionReviewLogError {
    /// Parent directory creation failed.
    CreateDir(std::io::Error),
    /// Packet serialization failed.
    Serialize(serde_json::Error),
    /// File write failed.
    Write(std::io::Error),
}

impl std::fmt::Display for AdmissionReviewLogError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateDir(err) => write!(formatter, "create admission log dir failed: {err}"),
            Self::Serialize(err) => write!(formatter, "serialize admission log failed: {err}"),
            Self::Write(err) => write!(formatter, "write admission log failed: {err}"),
        }
    }
}

impl std::error::Error for AdmissionReviewLogError {}

fn open_admission_packet(request: AdmissionReviewRequest) -> AdmissionReviewPacket {
    let identity = target_identity_for(&request);
    let destination_label = request
        .destination
        .clone()
        .unwrap_or_else(|| identity.normalized_label.clone());
    base_packet(
        request,
        identity,
        DestinationReview {
            destination_label: destination_label.clone(),
            disposition: DestinationDisposition::NoWrite,
            review_required_before_write: false,
            staging_or_temporary_location_disclosed: false,
            summary: "Open inspects an existing target and does not write project bytes."
                .to_string(),
        },
        WriteScopeReview {
            write_scope_class: WriteScopeClass::OpenExistingTarget,
            affected_scope_label: destination_label.clone(),
            proposed_items: vec![WriteScopeItem {
                item_kind: WriteScopeItemKind::OpenExisting,
                target_ref: destination_label,
                action_label: "Open existing target".to_string(),
            }],
            checkpoint_required: false,
            recovery_checkpoint_or_undo_group: None,
        },
        RecoveryPosture {
            recovery_path_class: RecoveryPathClass::OpenMinimalOrSetUpLater,
            available_actions: vec![
                AdmissionAction::Open,
                AdmissionAction::OpenMinimal,
                AdmissionAction::Cancel,
            ],
            typed_inputs_preserved_on_failure: true,
            redacted_diagnostics_on_failure: true,
            summary: "Open can be cancelled before trust review; typed target remains available."
                .to_string(),
        },
        FollowOnReview {
            post_entry_action: AdmissionAction::Open,
            deliberately_not_run: vec![
                DeliberateNonAction::TrustGrant,
                DeliberateNonAction::DependencyRestore,
                DeliberateNonAction::RepoTasks,
                DeliberateNonAction::RepoHooks,
                DeliberateNonAction::RuntimeAttach,
            ],
            summary: "Open does not grant trust, run tasks, or start setup before admission."
                .to_string(),
        },
        "Open target reviewed with no disk-write side effect.".to_string(),
    )
}

fn clone_admission_packet(request: AdmissionReviewRequest) -> AdmissionReviewPacket {
    let remote = normalize_remote_label(&request.target_specifier);
    let auth_mode = auth_mode_for(&request.target_specifier);
    let destination = request
        .destination
        .clone()
        .unwrap_or_else(|| default_clone_destination(&remote.normalized_label));
    let collision = destination_collision(&destination);
    let has_collision = collision
        .as_ref()
        .is_some_and(|review| review.requires_explicit_choice);
    let disposition = if has_collision {
        DestinationDisposition::ReuseExistingDestination
    } else {
        DestinationDisposition::WriteToUserDestination
    };
    let route_note = request
        .network_route_label
        .clone()
        .unwrap_or_else(|| route_note_for(&request.target_specifier));
    let mut packet = base_packet(
        request,
        NormalizedTargetIdentity {
            identity_class: TargetIdentityClass::RemoteRepository,
            normalized_label: remote.normalized_label.clone(),
            identity_ref: opaque_ref("remote", &remote.normalized_label),
        },
        DestinationReview {
            destination_label: destination.clone(),
            disposition,
            review_required_before_write: true,
            staging_or_temporary_location_disclosed: false,
            summary: if has_collision {
                "Destination already exists; choose reuse, add root, clone elsewhere, reveal target, or cancel.".to_string()
            } else {
                "Clone writes only to the reviewed destination after the user commits.".to_string()
            },
        },
        WriteScopeReview {
            write_scope_class: WriteScopeClass::CloneMaterialization,
            affected_scope_label: destination.clone(),
            proposed_items: vec![WriteScopeItem {
                item_kind: WriteScopeItemKind::CreateCloneRoot,
                target_ref: destination.clone(),
                action_label: "Create clone root at reviewed destination".to_string(),
            }],
            checkpoint_required: false,
            recovery_checkpoint_or_undo_group: Some(
                "clone-admission-destination-review".to_string(),
            ),
        },
        RecoveryPosture {
            recovery_path_class: if has_collision {
                RecoveryPathClass::RevealOrChooseElsewhere
            } else {
                RecoveryPathClass::CancelNoChange
            },
            available_actions: vec![
                AdmissionAction::CloneHere,
                AdmissionAction::CloneAndReview,
                AdmissionAction::CloneAndOpen,
                AdmissionAction::CloneAndAdd,
                AdmissionAction::CloneOnly,
                AdmissionAction::Cancel,
            ],
            typed_inputs_preserved_on_failure: true,
            redacted_diagnostics_on_failure: true,
            summary:
                "Clone failures preserve the typed URL, destination, and redacted diagnostics."
                    .to_string(),
        },
        FollowOnReview {
            post_entry_action: AdmissionAction::CloneAndReview,
            deliberately_not_run: vec![
                DeliberateNonAction::TrustGrant,
                DeliberateNonAction::DependencyRestore,
                DeliberateNonAction::RepoTasks,
                DeliberateNonAction::RepoHooks,
                DeliberateNonAction::ExtensionOrBundleInstall,
                DeliberateNonAction::TemplateOrBootstrap,
            ],
            summary: "After clone, trust review and setup remain separate steps.".to_string(),
        },
        "Clone admission discloses remote, destination, write scope, and recovery before Git runs."
            .to_string(),
    );
    packet.clone_review = Some(CloneAdmissionReview {
        normalized_remote_label: remote.normalized_label,
        host_label: remote.host_label,
        certificate_posture: certificate_posture_for(&remote.scheme),
        auth_mode,
        ref_choice: RefChoice::UnresolvedUntilRemoteQuery,
        submodule_posture: SubmodulePosture::DetectOnly,
        lfs_posture: LfsPosture::DetectOnly,
        route_note,
        explicit_actions: vec![
            AdmissionAction::CloneOnly,
            AdmissionAction::CloneAndReview,
            AdmissionAction::CloneAndOpen,
            AdmissionAction::CloneAndAdd,
        ],
    });
    packet.collision_review = collision;
    packet.admission_review_id = review_id_for(&packet);
    packet
}

fn import_admission_packet(request: AdmissionReviewRequest) -> AdmissionReviewPacket {
    let artifact = import_artifact_class(&request);
    let action = import_action_for(request.resulting_mode, artifact);
    let identity_class = match artifact {
        ImportArtifactClass::HandoffPacket | ImportArtifactClass::PortableStatePackage => {
            TargetIdentityClass::HandoffPacket
        }
        ImportArtifactClass::PatchArtifact => TargetIdentityClass::PatchArtifact,
        _ => TargetIdentityClass::ImportArtifact,
    };
    let target_label = normalize_path_label(&request.target_specifier);
    let destination = request
        .destination
        .clone()
        .unwrap_or_else(|| default_import_destination(action));
    let inspect_only = action == ImportAction::InspectOnly;
    let disposition = if inspect_only {
        DestinationDisposition::InspectOnly
    } else if action == ImportAction::AddToCurrentWorkspace {
        DestinationDisposition::AddToCurrentWorkspace
    } else {
        DestinationDisposition::WriteToLabelledStaging
    };
    let packet = base_packet(
        request,
        NormalizedTargetIdentity {
            identity_class,
            normalized_label: target_label.clone(),
            identity_ref: opaque_ref("import", &target_label),
        },
        DestinationReview {
            destination_label: destination.clone(),
            disposition,
            review_required_before_write: !inspect_only,
            staging_or_temporary_location_disclosed: !inspect_only,
            summary: if inspect_only {
                "Import is inspect-only; no extraction target is mutated.".to_string()
            } else {
                "Import extracts to labelled staging or the reviewed workspace target only.".to_string()
            },
        },
        WriteScopeReview {
            write_scope_class: if inspect_only {
                WriteScopeClass::ReadOnlyInspect
            } else {
                WriteScopeClass::ImportExtraction
            },
            affected_scope_label: destination.clone(),
            proposed_items: vec![WriteScopeItem {
                item_kind: if inspect_only {
                    WriteScopeItemKind::None
                } else {
                    WriteScopeItemKind::ExtractImportedContent
                },
                target_ref: destination.clone(),
                action_label: if inspect_only {
                    "Inspect artifact without extraction".to_string()
                } else {
                    "Extract imported content for review".to_string()
                },
            }],
            checkpoint_required: !inspect_only,
            recovery_checkpoint_or_undo_group: (!inspect_only)
                .then(|| "import-admission-rollback-checkpoint".to_string()),
        },
        RecoveryPosture {
            recovery_path_class: if inspect_only {
                RecoveryPathClass::InspectOnlyNoMutation
            } else {
                RecoveryPathClass::RollbackCheckpoint
            },
            available_actions: vec![
                AdmissionAction::Import,
                AdmissionAction::InspectOnly,
                AdmissionAction::SetUpLater,
                AdmissionAction::OpenMinimal,
                AdmissionAction::Cancel,
            ],
            typed_inputs_preserved_on_failure: true,
            redacted_diagnostics_on_failure: true,
            summary: "Import failures preserve the selected artifact, destination, and redacted diagnostics.".to_string(),
        },
        FollowOnReview {
            post_entry_action: if inspect_only {
                AdmissionAction::InspectOnly
            } else {
                AdmissionAction::Import
            },
            deliberately_not_run: vec![
                DeliberateNonAction::TrustGrant,
                DeliberateNonAction::DependencyRestore,
                DeliberateNonAction::RepoTasks,
                DeliberateNonAction::RepoHooks,
                DeliberateNonAction::TemplateOrBootstrap,
                DeliberateNonAction::RuntimeAttach,
            ],
            summary: "Import does not restore state, grant trust, or run setup until review admits it.".to_string(),
        },
        "Import admission discloses inspect/write posture, target, exclusions, and rollback before extraction.".to_string(),
    );
    with_import_review(packet, artifact, action)
}

fn add_root_admission_packet(request: AdmissionReviewRequest) -> AdmissionReviewPacket {
    let identity = target_identity_for(&request);
    let active = request
        .active_workspace_label
        .clone()
        .unwrap_or_else(|| "current workspace".to_string());
    let target_label = identity.normalized_label.clone();
    base_packet(
        request,
        identity,
        DestinationReview {
            destination_label: active.clone(),
            disposition: DestinationDisposition::AddToCurrentWorkspace,
            review_required_before_write: true,
            staging_or_temporary_location_disclosed: false,
            summary: "Add root changes the current workspace object model after review."
                .to_string(),
        },
        WriteScopeReview {
            write_scope_class: WriteScopeClass::AddRootWorkspaceMutation,
            affected_scope_label: active,
            proposed_items: vec![WriteScopeItem {
                item_kind: WriteScopeItemKind::AddWorkspaceRoot,
                target_ref: target_label,
                action_label: "Add reviewed root to current workspace".to_string(),
            }],
            checkpoint_required: true,
            recovery_checkpoint_or_undo_group: Some("add-root-admission-undo-group".to_string()),
        },
        RecoveryPosture {
            recovery_path_class: RecoveryPathClass::CheckpointOrUndoGroup,
            available_actions: vec![
                AdmissionAction::AddRoot,
                AdmissionAction::OpenMinimal,
                AdmissionAction::Cancel,
            ],
            typed_inputs_preserved_on_failure: true,
            redacted_diagnostics_on_failure: true,
            summary: "Add-root failure preserves the selected root and current workspace state."
                .to_string(),
        },
        FollowOnReview {
            post_entry_action: AdmissionAction::AddRoot,
            deliberately_not_run: vec![
                DeliberateNonAction::TrustGrant,
                DeliberateNonAction::DependencyRestore,
                DeliberateNonAction::RepoTasks,
                DeliberateNonAction::RepoHooks,
            ],
            summary: "The added root receives its own admission and trust review.".to_string(),
        },
        "Add-root admission discloses current workspace mutation before scope widens.".to_string(),
    )
}

fn restore_admission_packet(request: AdmissionReviewRequest) -> AdmissionReviewPacket {
    let identity = target_identity_for(&request);
    let target_label = identity.normalized_label.clone();
    base_packet(
        request,
        identity,
        DestinationReview {
            destination_label: target_label.clone(),
            disposition: DestinationDisposition::RestoreState,
            review_required_before_write: true,
            staging_or_temporary_location_disclosed: false,
            summary: "Restore rehydrates reviewed state without rerunning live work.".to_string(),
        },
        WriteScopeReview {
            write_scope_class: WriteScopeClass::RestoreStateMutation,
            affected_scope_label: target_label.clone(),
            proposed_items: vec![WriteScopeItem {
                item_kind: WriteScopeItemKind::ApplyRestoreState,
                target_ref: target_label,
                action_label: "Apply reviewed restore state".to_string(),
            }],
            checkpoint_required: true,
            recovery_checkpoint_or_undo_group: Some("restore-admission-checkpoint".to_string()),
        },
        RecoveryPosture {
            recovery_path_class: RecoveryPathClass::OpenMinimalOrSetUpLater,
            available_actions: vec![
                AdmissionAction::Open,
                AdmissionAction::OpenMinimal,
                AdmissionAction::Cancel,
            ],
            typed_inputs_preserved_on_failure: true,
            redacted_diagnostics_on_failure: true,
            summary: "Restore can open minimal or skip restored execution surfaces.".to_string(),
        },
        FollowOnReview {
            post_entry_action: AdmissionAction::Open,
            deliberately_not_run: vec![
                DeliberateNonAction::RepoTasks,
                DeliberateNonAction::RepoHooks,
                DeliberateNonAction::RuntimeAttach,
                DeliberateNonAction::TemplateOrBootstrap,
            ],
            summary:
                "Restore does not rerun tasks, terminals, debug sessions, or shared authority."
                    .to_string(),
        },
        "Restore admission discloses state mutation and non-rerun posture.".to_string(),
    )
}

fn with_import_review(
    mut packet: AdmissionReviewPacket,
    artifact: ImportArtifactClass,
    action: ImportAction,
) -> AdmissionReviewPacket {
    let destination = packet.destination_review.destination_label.clone();
    packet.import_review = Some(ImportAdmissionReview {
        artifact_class: artifact,
        import_action: action,
        schema_or_producer_label: schema_or_producer_label_for(artifact),
        extraction_or_restore_target_label: destination,
        machine_local_exclusions: vec![
            "live auth tokens".to_string(),
            "machine-local trust anchors".to_string(),
            "runtime handles".to_string(),
        ],
        cleanup_posture: if action == ImportAction::InspectOnly {
            CleanupPosture::NoCleanupRequired
        } else {
            CleanupPosture::RetainLabelledStaging
        },
        temporary_staging_disclosed: action != ImportAction::InspectOnly,
    });
    packet.admission_review_id = review_id_for(&packet);
    packet
}

fn base_packet(
    request: AdmissionReviewRequest,
    identity: NormalizedTargetIdentity,
    destination_review: DestinationReview,
    write_scope: WriteScopeReview,
    recovery_posture: RecoveryPosture,
    follow_on_review: FollowOnReview,
    summary: String,
) -> AdmissionReviewPacket {
    let mut packet = AdmissionReviewPacket {
        record_kind: AdmissionReviewRecordKind::AdmissionReviewPacketRecord,
        admission_review_schema_version: ADMISSION_REVIEW_SCHEMA_VERSION,
        admission_review_id: String::new(),
        source_surface: request.source_surface,
        entry_verb: request.entry_verb,
        target_kind: request.target_kind,
        resulting_mode: request.resulting_mode,
        normalized_target_identity: identity,
        destination_review,
        write_scope,
        trust_and_setup_review: TrustAndSetupReview {
            no_silent_trust_grant: true,
            no_setup_execution: true,
            no_task_or_hook_execution: true,
            no_hidden_temporary_materialization: true,
            summary: "Trust, setup, task execution, and hidden temp materialization remain gated by later reviewed steps.".to_string(),
        },
        recovery_posture,
        follow_on_review,
        clone_review: None,
        import_review: None,
        drag_drop_review: None,
        collision_review: None,
        summary,
    };
    packet.admission_review_id = review_id_for(&packet);
    packet
}

fn target_identity_for(request: &AdmissionReviewRequest) -> NormalizedTargetIdentity {
    let normalized = normalize_path_label(&request.target_specifier);
    let identity_class = match request.target_kind {
        TargetKind::LocalRepoRoot => TargetIdentityClass::RepoRoot,
        TargetKind::WorkspaceManifest | TargetKind::WorksetManifest => {
            TargetIdentityClass::WorkspaceManifest
        }
        TargetKind::RemoteRepository
        | TargetKind::SshWorkspace
        | TargetKind::ContainerWorkspace
        | TargetKind::DevcontainerWorkspace
        | TargetKind::ManagedCloudWorkspace => TargetIdentityClass::RemoteRepository,
        TargetKind::PortableStatePackage
        | TargetKind::CompetitorConfigRoot
        | TargetKind::TemplateOrPrebuildSnapshot => TargetIdentityClass::ImportArtifact,
        TargetKind::HandoffPacket => TargetIdentityClass::HandoffPacket,
        TargetKind::RecoveryCheckpoint => TargetIdentityClass::RecoveryCheckpoint,
        TargetKind::LocalFile | TargetKind::LocalFolder => TargetIdentityClass::FilesystemPath,
        TargetKind::ReviewOrWorkItemDeepLink => TargetIdentityClass::Unresolved,
    };
    NormalizedTargetIdentity {
        identity_class,
        normalized_label: normalized.clone(),
        identity_ref: opaque_ref(identity_class.as_str(), &normalized),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NormalizedRemote {
    normalized_label: String,
    host_label: String,
    scheme: String,
}

fn normalize_remote_label(specifier: &str) -> NormalizedRemote {
    let trimmed = specifier.trim();
    if let Some(rest) = trimmed.strip_prefix("https://") {
        return normalize_http_remote("https", rest);
    }
    if let Some(rest) = trimmed.strip_prefix("http://") {
        return normalize_http_remote("http", rest);
    }
    if let Some(rest) = trimmed.strip_prefix("ssh://") {
        let without_credentials = rest.rsplit('@').next().unwrap_or(rest);
        let host = without_credentials
            .split('/')
            .next()
            .unwrap_or(without_credentials);
        return NormalizedRemote {
            normalized_label: trim_git_suffix(without_credentials).to_string(),
            host_label: host.to_string(),
            scheme: "ssh".to_string(),
        };
    }
    if let Some((user_host, path)) = trimmed.split_once(':') {
        if user_host.contains('@') && !path.is_empty() {
            let host = user_host.rsplit('@').next().unwrap_or(user_host);
            return NormalizedRemote {
                normalized_label: format!("{}/{}", host, trim_git_suffix(path)),
                host_label: host.to_string(),
                scheme: "ssh".to_string(),
            };
        }
    }
    NormalizedRemote {
        normalized_label: redact_credentials(trimmed),
        host_label: "unknown host".to_string(),
        scheme: "unknown".to_string(),
    }
}

fn normalize_http_remote(scheme: &str, rest: &str) -> NormalizedRemote {
    let without_fragment = rest.split(['?', '#']).next().unwrap_or(rest);
    let without_credentials = without_fragment
        .rsplit('@')
        .next()
        .unwrap_or(without_fragment);
    let host = without_credentials
        .split('/')
        .next()
        .unwrap_or(without_credentials);
    NormalizedRemote {
        normalized_label: trim_git_suffix(without_credentials).to_string(),
        host_label: host.to_string(),
        scheme: scheme.to_string(),
    }
}

fn redact_credentials(value: &str) -> String {
    if let Some((prefix, suffix)) = value.rsplit_once('@') {
        if prefix.contains(':') || prefix.contains("//") {
            return suffix.to_string();
        }
    }
    value.to_string()
}

fn trim_git_suffix(value: &str) -> &str {
    value.strip_suffix(".git").unwrap_or(value)
}

fn normalize_path_label(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "unresolved target".to_string();
    }
    trimmed.to_string()
}

fn default_clone_destination(remote_label: &str) -> String {
    let repo_name = remote_label
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or("repository");
    format!("~/Code/{repo_name}")
}

fn default_import_destination(action: ImportAction) -> String {
    match action {
        ImportAction::InspectOnly => "inspect-only view".to_string(),
        ImportAction::AddToCurrentWorkspace => "current workspace".to_string(),
        ImportAction::RestoreFromPacket | ImportAction::CompareBeforeRestore => {
            "reviewed restore target".to_string()
        }
        ImportAction::ExtractAndReview | ImportAction::ExtractAndOpen => {
            "labelled import staging".to_string()
        }
    }
}

fn destination_collision(destination: &str) -> Option<DestinationCollisionReview> {
    if destination.trim().is_empty() || destination.starts_with("~/") {
        return None;
    }
    let path = Path::new(destination);
    if !path.exists() {
        return None;
    }
    let collision_class = if path.is_file()
        && path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("code-workspace"))
    {
        DestinationCollisionClass::ExistingWorkspaceFile
    } else if path.is_dir() && path.join(".git").exists() {
        DestinationCollisionClass::ExistingRepoRoot
    } else if path.is_dir() {
        match std::fs::read_dir(path) {
            Ok(mut entries) => {
                if entries.next().is_none() {
                    return None;
                }
                DestinationCollisionClass::ExistingPathNonEmpty
            }
            Err(_) => DestinationCollisionClass::DestinationBlocked,
        }
    } else {
        DestinationCollisionClass::ExistingPathNonEmpty
    };

    Some(DestinationCollisionReview {
        collision_class,
        existing_target_label: destination.to_string(),
        requires_explicit_choice: true,
        safe_actions: vec![
            AdmissionAction::ReuseExisting,
            AdmissionAction::AddExistingAsRoot,
            AdmissionAction::CloneElsewhere,
            AdmissionAction::RevealTarget,
            AdmissionAction::Cancel,
        ],
        summary:
            "Destination already exists; choose a specific recovery action instead of overwriting."
                .to_string(),
    })
}

fn certificate_posture_for(scheme: &str) -> CertificatePosture {
    match scheme {
        "https" => CertificatePosture::TlsSystemTrustPending,
        "http" => CertificatePosture::CertificateReviewRequired,
        "ssh" => CertificatePosture::SshHostKeyReviewRequired,
        _ => CertificatePosture::NotApplicable,
    }
}

fn auth_mode_for(specifier: &str) -> CloneAuthMode {
    let lower = specifier.to_ascii_lowercase();
    if lower.starts_with("ssh://")
        || (!lower.contains("://") && lower.contains('@') && lower.contains(':'))
    {
        CloneAuthMode::SshAgent
    } else if lower.contains("github.com") || lower.contains("gitlab.com") {
        CloneAuthMode::OAuthOrBrowserHandoff
    } else if lower.starts_with("https://") || lower.starts_with("http://") {
        CloneAuthMode::CredentialHelperOrAnonymous
    } else {
        CloneAuthMode::UnknownUntilCredentialReview
    }
}

fn route_note_for(specifier: &str) -> String {
    let lower = specifier.to_ascii_lowercase();
    if lower.contains("mirror") {
        "mirror route configured by source".to_string()
    } else if std::env::var_os("HTTPS_PROXY").is_some()
        || std::env::var_os("https_proxy").is_some()
        || std::env::var_os("ALL_PROXY").is_some()
        || std::env::var_os("all_proxy").is_some()
    {
        "system proxy environment detected".to_string()
    } else {
        "direct or system Git route".to_string()
    }
}

fn import_artifact_class(request: &AdmissionReviewRequest) -> ImportArtifactClass {
    match request.target_kind {
        TargetKind::PortableStatePackage => ImportArtifactClass::PortableStatePackage,
        TargetKind::HandoffPacket => ImportArtifactClass::HandoffPacket,
        TargetKind::CompetitorConfigRoot => ImportArtifactClass::CompetitorConfigRoot,
        TargetKind::WorkspaceManifest | TargetKind::WorksetManifest => {
            ImportArtifactClass::WorkspaceManifestBundle
        }
        _ => {
            let lower = request.target_specifier.to_ascii_lowercase();
            if lower.ends_with(".patch") || lower.ends_with(".diff") {
                ImportArtifactClass::PatchArtifact
            } else if lower.ends_with(".zip")
                || lower.ends_with(".tar")
                || lower.ends_with(".tar.gz")
                || lower.ends_with(".tgz")
            {
                ImportArtifactClass::ArchiveBundle
            } else {
                ImportArtifactClass::Unknown
            }
        }
    }
}

fn import_action_for(
    resulting_mode: ResultingMode,
    artifact_class: ImportArtifactClass,
) -> ImportAction {
    match resulting_mode {
        ResultingMode::InspectOnly => ImportAction::InspectOnly,
        ResultingMode::CompareBeforeRestore => ImportAction::CompareBeforeRestore,
        ResultingMode::ApplyToActiveWorkspace => ImportAction::AddToCurrentWorkspace,
        ResultingMode::ExtractThenReview => {
            if artifact_class == ImportArtifactClass::HandoffPacket {
                ImportAction::CompareBeforeRestore
            } else {
                ImportAction::ExtractAndReview
            }
        }
        ResultingMode::OpenPrebuildWithSetupActions | ResultingMode::OpenPrebuildMinimal => {
            ImportAction::ExtractAndReview
        }
        _ => ImportAction::ExtractAndReview,
    }
}

fn schema_or_producer_label_for(artifact: ImportArtifactClass) -> String {
    match artifact {
        ImportArtifactClass::PortableStatePackage => {
            "portable state package schema reviewed".to_string()
        }
        ImportArtifactClass::HandoffPacket => "handoff packet producer reviewed".to_string(),
        ImportArtifactClass::CompetitorConfigRoot => {
            "competitor configuration mapping reviewed".to_string()
        }
        ImportArtifactClass::ArchiveBundle => {
            "archive manifest or directory listing reviewed".to_string()
        }
        ImportArtifactClass::PatchArtifact => "patch metadata reviewed".to_string(),
        ImportArtifactClass::WorkspaceManifestBundle => {
            "workspace manifest schema reviewed".to_string()
        }
        ImportArtifactClass::Unknown => "artifact class unresolved; review required".to_string(),
    }
}

fn drag_drop_entry_tuple(
    request: &DragDropAdmissionRequest,
) -> (EntryVerb, TargetKind, ResultingMode, AdmissionAction) {
    match request.payload_kind {
        DragDropPayloadKind::File => (
            EntryVerb::Open,
            TargetKind::LocalFile,
            ResultingMode::SingleFile,
            AdmissionAction::Open,
        ),
        DragDropPayloadKind::Folder => {
            if request.target_inside_active_workspace {
                (
                    EntryVerb::AddRoot,
                    TargetKind::LocalFolder,
                    ResultingMode::WorkspaceWithRoots,
                    AdmissionAction::AddRoot,
                )
            } else {
                (
                    EntryVerb::Open,
                    TargetKind::LocalFolder,
                    ResultingMode::Folder,
                    AdmissionAction::Open,
                )
            }
        }
        DragDropPayloadKind::Repository => {
            if request.target_inside_active_workspace {
                (
                    EntryVerb::AddRoot,
                    TargetKind::LocalRepoRoot,
                    ResultingMode::WorkspaceWithRoots,
                    AdmissionAction::AddRoot,
                )
            } else {
                (
                    EntryVerb::Open,
                    TargetKind::LocalRepoRoot,
                    ResultingMode::RepoRoot,
                    AdmissionAction::Open,
                )
            }
        }
        DragDropPayloadKind::WorkspaceFile => (
            EntryVerb::Open,
            TargetKind::WorkspaceManifest,
            ResultingMode::WorkspaceWithRoots,
            AdmissionAction::Open,
        ),
        DragDropPayloadKind::Patch => (
            EntryVerb::Import,
            TargetKind::HandoffPacket,
            ResultingMode::ApplyToActiveWorkspace,
            AdmissionAction::Import,
        ),
        DragDropPayloadKind::Archive => (
            EntryVerb::Import,
            TargetKind::PortableStatePackage,
            ResultingMode::ExtractThenReview,
            AdmissionAction::Import,
        ),
        DragDropPayloadKind::RemoteRepositoryUrl => (
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            AdmissionAction::CloneHere,
        ),
    }
}

fn transfer_progress_class(bytes: Option<u64>) -> TransferProgressClass {
    const LARGE_TRANSFER_BYTES: u64 = 16 * 1024 * 1024;
    match bytes {
        Some(bytes) if bytes >= LARGE_TRANSFER_BYTES => {
            TransferProgressClass::DurableProgressWithCancel
        }
        Some(_) => TransferProgressClass::InlineProgress,
        None => TransferProgressClass::InlineProgress,
    }
}

fn opaque_ref(prefix: &str, value: &str) -> String {
    format!("{prefix}:{:016x}", fnv1a_64(value))
}

fn review_id_for(packet: &AdmissionReviewPacket) -> String {
    format!(
        "admission-review-{:016x}",
        fnv1a_64(&format!(
            "{}\n{}\n{}\n{}\n{}",
            packet.source_surface.as_str(),
            packet.entry_verb.as_str(),
            packet.target_kind.as_str(),
            packet.resulting_mode.as_str(),
            packet.normalized_target_identity.normalized_label
        ))
    )
}

fn fnv1a_64(value: &str) -> u64 {
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

    use std::path::PathBuf;

    #[test]
    fn clone_review_redacts_remote_and_reports_collision_choices() {
        let temp = tempfile_dir_with_file();
        let packet = review_entry_admission(
            AdmissionReviewRequest::new(
                AdmissionSourceSurface::StartCenter,
                EntryVerb::Clone,
                TargetKind::RemoteRepository,
                ResultingMode::CloneThenReview,
                "https://token:secret@github.com/acme/payments.git",
            )
            .with_destination(temp.display().to_string()),
        );

        let clone = packet.clone_review.as_ref().expect("clone review");
        assert_eq!(clone.normalized_remote_label, "github.com/acme/payments");
        assert_eq!(clone.host_label, "github.com");
        assert_eq!(clone.auth_mode, CloneAuthMode::OAuthOrBrowserHandoff);
        assert!(packet.requires_collision_choice());
        let collision = packet.collision_review.as_ref().expect("collision review");
        assert_eq!(
            collision.safe_actions,
            vec![
                AdmissionAction::ReuseExisting,
                AdmissionAction::AddExistingAsRoot,
                AdmissionAction::CloneElsewhere,
                AdmissionAction::RevealTarget,
                AdmissionAction::Cancel,
            ]
        );
        assert!(packet.trust_and_setup_review.no_silent_trust_grant);
        assert!(packet.recovery_posture.typed_inputs_preserved_on_failure);
    }

    #[test]
    fn import_archive_review_discloses_staging_and_exclusions() {
        let packet = review_entry_admission(AdmissionReviewRequest::new(
            AdmissionSourceSurface::CommandPalette,
            EntryVerb::Import,
            TargetKind::PortableStatePackage,
            ResultingMode::ExtractThenReview,
            "~/Downloads/workspace.zip",
        ));

        let import = packet.import_review.as_ref().expect("import review");
        assert_eq!(
            import.artifact_class,
            ImportArtifactClass::PortableStatePackage
        );
        assert_eq!(import.import_action, ImportAction::ExtractAndReview);
        assert!(import.temporary_staging_disclosed);
        assert!(import
            .machine_local_exclusions
            .iter()
            .any(|item| item == "live auth tokens"));
        assert_eq!(
            packet.destination_review.disposition,
            DestinationDisposition::WriteToLabelledStaging
        );
    }

    #[test]
    fn drag_drop_archive_uses_import_admission_with_progress_and_checkpoint() {
        let packet = review_drag_drop_admission(
            DragDropAdmissionRequest::new(
                DragDropPayloadKind::Archive,
                "~/Downloads/project.tar.gz",
            )
            .with_destination("~/Code/project")
            .with_aggregate_bytes(64 * 1024 * 1024),
        );

        assert_eq!(packet.source_surface, AdmissionSourceSurface::DragAndDrop);
        assert_eq!(packet.entry_verb, EntryVerb::Import);
        let drop = packet.drag_drop_review.as_ref().expect("drop review");
        assert_eq!(drop.advertised_verb, AdmissionAction::Import);
        assert_eq!(
            drop.progress_class,
            TransferProgressClass::DurableProgressWithCancel
        );
        assert!(drop.uses_same_admission_model);
        assert!(drop.checkpoint_or_undo_group.is_some());
    }

    #[test]
    fn fixture_packets_match_required_admission_fields() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/project_entry");
        let mut seen = 0usize;
        for entry in std::fs::read_dir(root).expect("fixture dir") {
            let entry = entry.expect("fixture entry");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            seen += 1;
            let payload = std::fs::read_to_string(&path).expect("fixture reads");
            let packet: AdmissionReviewPacket =
                serde_json::from_str(&payload).expect("fixture parses");
            assert_eq!(
                packet.record_kind,
                AdmissionReviewRecordKind::AdmissionReviewPacketRecord
            );
            assert_eq!(
                packet.admission_review_schema_version,
                ADMISSION_REVIEW_SCHEMA_VERSION
            );
            assert!(packet.trust_and_setup_review.no_silent_trust_grant);
            assert!(packet.trust_and_setup_review.no_setup_execution);
            assert!(packet.trust_and_setup_review.no_task_or_hook_execution);
            assert!(packet.recovery_posture.typed_inputs_preserved_on_failure);
            assert!(packet.recovery_posture.redacted_diagnostics_on_failure);
        }
        assert!(seen >= 3, "expected project-entry admission fixtures");
    }

    fn tempfile_dir_with_file() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "aureline-admission-test-{:016x}",
            fnv1a_64(&format!("{:?}", std::time::SystemTime::now()))
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create temp dir");
        std::fs::write(root.join("README.md"), "occupied\n").expect("seed file");
        root
    }
}
