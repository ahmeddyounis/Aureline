//! Crash-loop recovery center: evidence-first recovery choices for repeated
//! startup failures.
//!
//! When startup, reopen, or a supervised host exhausts its restart budget,
//! Aureline must stop silently retrying full restore and instead route the
//! blocked user into a product-owned recovery surface. This module owns the
//! truth model for that surface.
//!
//! It consumes one typed [`CrashLoopSignal`] — the detected breach, the
//! crash and build identity, the restore class, the suspected fault domain,
//! the last attempted reopen mode, and the recent extension/profile/layout
//! changes — and synthesizes one [`CrashLoopRecoveryCenter`]. The center
//! offers bounded, command-backed recovery choices (Safe mode, Open without
//! restore, Disable recently changed extension, Disable recently changed
//! profile/layout, Open logs, Export crash manifest, Report issue) and keeps
//! evidence-only and checkpoint/diff entry points for recovered drafts and
//! rollbackable state as their own distinct paths rather than collapsing
//! everything into one generic "try again" button.
//!
//! Every projection is metadata-only. The
//! [`CrashLoopRecoverySupportPacket`] mirrors the same closed vocabulary the
//! center renders so the in-product cards and the support-safe export packet
//! never disagree on what failed, what stays preserved, and what the next
//! safe action is.
//!
//! ## Invariants this module enforces
//!
//! - The center is never an invisible restart loop: it is only produced for a
//!   genuine restart-budget breach (or an explicit user request), and it pins
//!   `silent_restart_suppressed = true`.
//! - No recovery choice deletes user-owned state, and every choice is
//!   narrower than a full reset; destructive cleanup is never suggested.
//! - Crash id, build id, restore class, and suspected fault domain stay
//!   visible in the center and in the support packet.
//! - Safe mode and Open without restore honor no-silent-rerun semantics:
//!   for privileged or mutating sessions they require explicit, reviewed
//!   confirmation and never silently re-run the session.
//! - Every choice and entry point is keyboard-complete and carries a
//!   screen-reader label.
//!
//! The boundary schema lives at [`CRASH_LOOP_RECOVERY_SCHEMA_REF`] and the
//! reviewer doc lives at [`CRASH_LOOP_RECOVERY_DOC_REF`]; both are quoted
//! verbatim on every emitted support packet.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a crash-loop signal record (the input).
pub const CRASH_LOOP_SIGNAL_RECORD_KIND: &str = "crash_loop_signal_record";

/// Stable record-kind tag for a synthesized crash-loop recovery center.
pub const CRASH_LOOP_RECOVERY_CENTER_RECORD_KIND: &str = "crash_loop_recovery_center_record";

/// Stable record-kind tag for the metadata-safe support projection.
pub const CRASH_LOOP_RECOVERY_SUPPORT_PACKET_RECORD_KIND: &str =
    "crash_loop_recovery_support_packet_record";

/// Frozen schema version for crash-loop recovery records.
pub const CRASH_LOOP_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const CRASH_LOOP_RECOVERY_SCHEMA_REF: &str = "schemas/support/crash_loop_recovery.schema.json";

/// Reviewer doc ref quoted verbatim by every emitted support packet.
pub const CRASH_LOOP_RECOVERY_DOC_REF: &str = "docs/support/m3/crash_loop_recovery_beta.md";

/// Closed trigger-class vocabulary explaining what opened the center.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CrashLoopTriggerClass {
    /// Cold startup repeated failures exhausted the startup restart budget.
    StartupRestartBudgetExceeded,
    /// Workspace reopen repeated failures exhausted the reopen restart budget.
    ReopenRestartBudgetExceeded,
    /// A supervised runtime/language host exhausted its restart budget.
    RuntimeHostRestartBudgetExceeded,
    /// A supervised extension host exhausted its restart budget.
    ExtensionHostRestartBudgetExceeded,
    /// Restore replay failed repeatedly across launches.
    RestoreReplayFailedRepeatedly,
    /// The user explicitly opened the recovery center.
    ExplicitUserRequest,
}

impl CrashLoopTriggerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartupRestartBudgetExceeded => "startup_restart_budget_exceeded",
            Self::ReopenRestartBudgetExceeded => "reopen_restart_budget_exceeded",
            Self::RuntimeHostRestartBudgetExceeded => "runtime_host_restart_budget_exceeded",
            Self::ExtensionHostRestartBudgetExceeded => "extension_host_restart_budget_exceeded",
            Self::RestoreReplayFailedRepeatedly => "restore_replay_failed_repeatedly",
            Self::ExplicitUserRequest => "explicit_user_request",
        }
    }

    /// Returns true when the trigger is a restart-budget breach (not an
    /// explicit user request). Breach triggers require an exhausted budget
    /// before the center is shown.
    pub const fn is_budget_breach(self) -> bool {
        !matches!(self, Self::ExplicitUserRequest)
    }
}

/// Closed restore-class vocabulary, aligned with the restore hydrator and
/// restore-preview surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreClass {
    /// The prior session would be restored exactly.
    ExactRestore,
    /// The prior session would be restored with compatible substitutions.
    CompatibleRestore,
    /// Only the window/pane layout would be restored.
    LayoutOnly,
    /// Only evidence (drafts, history) would be surfaced; no live restore.
    EvidenceOnly,
    /// No restore was attempted for this launch.
    NoRestoreAttempted,
}

impl RestoreClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
            Self::NoRestoreAttempted => "no_restore_attempted",
        }
    }
}

/// Closed suspected-fault-domain vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultDomainClass {
    /// The shell interaction core (window, layout, input loop).
    ShellInteractionCore,
    /// Session restore / continuity replay.
    RestoreContinuity,
    /// An extension host or extension lane.
    ExtensionHost,
    /// A language runtime / language-server host.
    LanguageRuntimeHost,
    /// The AI runtime host.
    AiRuntimeHost,
    /// A remote helper / remote-attach host.
    RemoteHelperHost,
    /// A workspace profile or saved layout.
    WorkspaceProfileOrLayout,
    /// A disposable cache or index lane.
    CacheOrIndex,
    /// The fault domain could not be narrowed.
    Unknown,
}

impl FaultDomainClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellInteractionCore => "shell_interaction_core",
            Self::RestoreContinuity => "restore_continuity",
            Self::ExtensionHost => "extension_host",
            Self::LanguageRuntimeHost => "language_runtime_host",
            Self::AiRuntimeHost => "ai_runtime_host",
            Self::RemoteHelperHost => "remote_helper_host",
            Self::WorkspaceProfileOrLayout => "workspace_profile_or_layout",
            Self::CacheOrIndex => "cache_or_index",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed reopen-mode vocabulary describing the last attempted launch posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenModeClass {
    /// Full restore replay was attempted.
    FullRestore,
    /// Restore was attempted with extensions held back.
    RestoreWithoutExtensions,
    /// The workspace was opened without restore replay.
    OpenWithoutRestore,
    /// Safe mode was the last attempted posture.
    SafeMode,
    /// Only a minimal shell was launched.
    MinimalShell,
}

impl ReopenModeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRestore => "full_restore",
            Self::RestoreWithoutExtensions => "restore_without_extensions",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::SafeMode => "safe_mode",
            Self::MinimalShell => "minimal_shell",
        }
    }
}

/// Closed recent-change vocabulary for suspected-change disable flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecentChangeClass {
    /// An extension was installed.
    ExtensionInstalled,
    /// An extension was updated.
    ExtensionUpdated,
    /// An extension was enabled.
    ExtensionEnabled,
    /// The active workspace profile was switched.
    ProfileSwitched,
    /// The active workspace profile was edited.
    ProfileEdited,
    /// A saved layout was changed.
    LayoutChanged,
    /// A startup-affecting setting was changed.
    SettingChanged,
}

impl RecentChangeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionInstalled => "extension_installed",
            Self::ExtensionUpdated => "extension_updated",
            Self::ExtensionEnabled => "extension_enabled",
            Self::ProfileSwitched => "profile_switched",
            Self::ProfileEdited => "profile_edited",
            Self::LayoutChanged => "layout_changed",
            Self::SettingChanged => "setting_changed",
        }
    }

    /// Returns true when the change is an extension change.
    pub const fn is_extension_change(self) -> bool {
        matches!(
            self,
            Self::ExtensionInstalled | Self::ExtensionUpdated | Self::ExtensionEnabled
        )
    }

    /// Returns true when the change is a profile, layout, or setting change.
    pub const fn is_profile_or_layout_change(self) -> bool {
        matches!(
            self,
            Self::ProfileSwitched
                | Self::ProfileEdited
                | Self::LayoutChanged
                | Self::SettingChanged
        )
    }
}

/// Closed recovery-choice vocabulary. There is no generic "try again" or
/// "reset" class: repeated startup failure routes to bounded, named actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryChoiceClass {
    /// Enter the bounded safe-mode runtime profile.
    EnterSafeMode,
    /// Open the workspace without restore replay.
    OpenWithoutRestore,
    /// Disable a recently changed extension suspected of the crash loop.
    DisableRecentlyChangedExtension,
    /// Disable a recently changed profile or layout suspected of the crash loop.
    DisableRecentlyChangedProfileOrLayout,
    /// Open the local logs.
    OpenLogs,
    /// Export a crash manifest for support.
    ExportCrashManifest,
    /// Report an issue with the crash evidence attached by reference.
    ReportIssue,
}

impl RecoveryChoiceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterSafeMode => "enter_safe_mode",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::DisableRecentlyChangedExtension => "disable_recently_changed_extension",
            Self::DisableRecentlyChangedProfileOrLayout => {
                "disable_recently_changed_profile_or_layout"
            }
            Self::OpenLogs => "open_logs",
            Self::ExportCrashManifest => "export_crash_manifest",
            Self::ReportIssue => "report_issue",
        }
    }

    /// Stable command id bound to this choice (command-backed and keyboard
    /// reachable).
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::EnterSafeMode => "command.recovery.enter_safe_mode",
            Self::OpenWithoutRestore => "command.recovery.open_without_restore",
            Self::DisableRecentlyChangedExtension => "command.recovery.disable_recent_extension",
            Self::DisableRecentlyChangedProfileOrLayout => {
                "command.recovery.disable_recent_profile_or_layout"
            }
            Self::OpenLogs => "command.recovery.open_logs",
            Self::ExportCrashManifest => "command.recovery.export_crash_manifest",
            Self::ReportIssue => "command.recovery.report_issue",
        }
    }

    /// Returns true when this choice re-enters or relaunches the user's
    /// session and therefore must honor no-silent-rerun semantics.
    pub const fn is_session_reentry(self) -> bool {
        matches!(self, Self::EnterSafeMode | Self::OpenWithoutRestore)
    }

    /// The base set of choices offered for every crash-loop center,
    /// regardless of the suspected change set.
    pub const BASE_CHOICES: [Self; 5] = [
        Self::EnterSafeMode,
        Self::OpenWithoutRestore,
        Self::OpenLogs,
        Self::ExportCrashManifest,
        Self::ReportIssue,
    ];
}

/// Closed evidence-entry-point vocabulary for recovered drafts and
/// rollbackable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceEntryClass {
    /// Inspect a recovered unsaved draft without applying it.
    RecoveredDraft,
    /// Open a checkpoint or diff for rollbackable state.
    CheckpointDiff,
    /// Inspect rollbackable state without applying a rollback.
    RollbackableState,
    /// Open the local-history timeline for the affected files.
    LocalHistoryTimeline,
}

impl EvidenceEntryClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoveredDraft => "recovered_draft",
            Self::CheckpointDiff => "checkpoint_diff",
            Self::RollbackableState => "rollbackable_state",
            Self::LocalHistoryTimeline => "local_history_timeline",
        }
    }

    /// Stable command id bound to this entry point.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::RecoveredDraft => "command.recovery.inspect_recovered_draft",
            Self::CheckpointDiff => "command.recovery.open_checkpoint_diff",
            Self::RollbackableState => "command.recovery.inspect_rollbackable_state",
            Self::LocalHistoryTimeline => "command.recovery.open_local_history_timeline",
        }
    }
}

/// Closed recovery state class. Mirrors the recovery-ladder and safe-mode
/// preservation vocabulary so support exports read one truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Selection, caret, and scroll state for open buffers.
    OpenBufferSelection,
    /// Recovered unsaved drafts.
    RecoveredDrafts,
    /// Durable workspace indexes that must not be deleted as collateral.
    DurableWorkspaceIndexes,
    /// Workspace trust state.
    WorkspaceTrustStore,
    /// Credential handles and stores.
    CredentialStore,
    /// Session restore records.
    SessionRestoreStore,
    /// Support export records and staging state.
    SupportExportStore,
    /// Checkpoint and rollbackable state.
    CheckpointHistory,
}

impl RecoveryStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredFiles => "user_authored_files",
            Self::OpenBufferSelection => "open_buffer_selection",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::DurableWorkspaceIndexes => "durable_workspace_indexes",
            Self::WorkspaceTrustStore => "workspace_trust_store",
            Self::CredentialStore => "credential_store",
            Self::SessionRestoreStore => "session_restore_store",
            Self::SupportExportStore => "support_export_store",
            Self::CheckpointHistory => "checkpoint_history",
        }
    }
}

/// Closed session-sensitivity vocabulary that drives no-silent-rerun gating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionSensitivityClass {
    /// Local, read-only session with no pending mutations.
    LocalReadOnly,
    /// Local session with pending mutating work.
    LocalMutating,
    /// Privileged or remote session (remote attach, delegated authority).
    PrivilegedOrRemote,
}

impl SessionSensitivityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReadOnly => "local_read_only",
            Self::LocalMutating => "local_mutating",
            Self::PrivilegedOrRemote => "privileged_or_remote",
        }
    }

    /// Returns true when the session is privileged or carries pending
    /// mutations, so re-entry must require explicit, reviewed confirmation.
    pub const fn requires_no_silent_rerun(self) -> bool {
        matches!(self, Self::LocalMutating | Self::PrivilegedOrRemote)
    }
}

/// Closed diagnostic data class for evidence refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDataClass {
    /// Metadata-only evidence.
    Metadata,
    /// Environment-adjacent evidence such as version or fault-domain refs.
    EnvironmentAdjacent,
    /// Code-adjacent evidence that is forbidden in the recovery center.
    CodeAdjacent,
    /// Secret-bearing evidence that is forbidden in the recovery center.
    SecretBearing,
}

impl EvidenceDataClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Metadata => "metadata",
            Self::EnvironmentAdjacent => "environment_adjacent",
            Self::CodeAdjacent => "code_adjacent",
            Self::SecretBearing => "secret_bearing",
        }
    }
}

/// Closed redaction class for evidence refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default redaction.
    MetadataSafeDefault,
    /// Opt-in support evidence.
    OptInOnly,
    /// Prohibited from the recovery center and support projection.
    Prohibited,
}

impl RedactionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OptInOnly => "opt_in_only",
            Self::Prohibited => "prohibited",
        }
    }
}

/// Redaction-safe evidence reference cited by the signal and the center.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopEvidenceRef {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Evidence kind or source role.
    pub evidence_kind: String,
    /// Diagnostic data class.
    pub data_class: EvidenceDataClass,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Reviewer-safe summary without raw private content.
    pub summary: String,
}

/// A recent change that may be the suspect behind the crash loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecentChange {
    /// Stable change identifier.
    pub change_id: String,
    /// Change class.
    pub change_class: RecentChangeClass,
    /// Opaque subject ref (extension id, profile id, layout id) safe for export.
    pub subject_ref: String,
    /// Reviewer-facing label that excludes raw paths and private content.
    pub display_label: String,
    /// UTC timestamp the change was observed.
    pub observed_at: String,
    /// Whether disabling the change is reversible.
    pub reversible: bool,
}

/// A recovered artifact (draft or rollbackable state) preserved for inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveredArtifact {
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Evidence entry class for this artifact.
    pub entry_class: EvidenceEntryClass,
    /// Reviewer-safe summary of the artifact.
    pub summary: String,
    /// State classes this artifact preserves.
    pub preserves: Vec<RecoveryStateClass>,
}

/// Detected crash-loop signal consumed by the center evaluator (input record).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopSignal {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable signal identifier.
    pub signal_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Trigger that opened the center.
    pub trigger_class: CrashLoopTriggerClass,
    /// Restart strikes observed in the active window.
    pub strike_count: u32,
    /// Automatic restart budget for the window.
    pub strike_budget: u32,
    /// Hidden restarts already attempted before the center was shown.
    pub hidden_restart_attempts: u32,
    /// Visible crash identifier (kept visible in-product and in exports).
    pub crash_id: String,
    /// Visible build identifier (kept visible in-product and in exports).
    pub build_id: String,
    /// Crash envelope ref.
    pub crash_envelope_ref: String,
    /// Crash manifest ref exported by the Export crash manifest choice.
    pub crash_manifest_ref: String,
    /// Restore class for the failed launch.
    pub restore_class: RestoreClass,
    /// Suspected fault domain.
    pub suspected_fault_domain: FaultDomainClass,
    /// Opaque fault-domain ref safe for export.
    pub fault_domain_ref: String,
    /// Last attempted reopen mode.
    pub last_reopen_mode: ReopenModeClass,
    /// Project Doctor finding that justified routing into the center.
    pub doctor_finding_ref: String,
    /// Session sensitivity that drives no-silent-rerun gating.
    pub session_sensitivity_class: SessionSensitivityClass,
    /// Recent extension/profile/layout changes that may be the suspect.
    #[serde(default)]
    pub recent_changes: Vec<RecentChange>,
    /// Recovered drafts and rollbackable state preserved for inspection.
    #[serde(default)]
    pub recovered_artifacts: Vec<RecoveredArtifact>,
    /// Metadata-safe evidence refs.
    pub evidence: Vec<CrashLoopEvidenceRef>,
}

/// Command binding for a recovery choice or entry point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandRef {
    /// Stable command id.
    pub command_id: String,
    /// Whether review is required before execution.
    pub requires_review: bool,
    /// Whether explicit user confirmation is required before execution.
    pub requires_explicit_confirmation: bool,
    /// Whether the action enforces no-silent-rerun for the session.
    pub no_silent_rerun: bool,
}

/// Accessibility posture for one choice or entry point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityPosture {
    /// Whether the action is fully reachable and operable by keyboard.
    pub keyboard_complete: bool,
    /// Screen-reader label describing the action and its effect.
    pub screen_reader_label: String,
    /// Stable focus order within the surface.
    pub focus_order: u32,
}

/// One bounded, command-backed recovery choice in the center.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryChoice {
    /// Stable choice identifier.
    pub choice_id: String,
    /// Choice class.
    pub choice_class: RecoveryChoiceClass,
    /// Command binding.
    pub command: CommandRef,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing description.
    pub description: String,
    /// What the choice explicitly preserves, discards, or defers.
    pub disposition_summary: String,
    /// State classes preserved by the choice.
    pub preserves: Vec<RecoveryStateClass>,
    /// Whether the choice deletes any user-owned state (must be false).
    pub deletes_user_owned_state: bool,
    /// Whether the choice is narrower than a full reset (must be true).
    pub narrower_than_full_reset: bool,
    /// Recent change this choice targets, when applicable.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub targets_recent_change_ref: Option<String>,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
}

/// One evidence-only / checkpoint-diff entry point preserved by the center.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceEntryPoint {
    /// Stable entry id.
    pub entry_id: String,
    /// Entry class.
    pub entry_class: EvidenceEntryClass,
    /// Command binding.
    pub command: CommandRef,
    /// Reviewer-safe summary.
    pub summary: String,
    /// State classes preserved by inspecting this entry.
    pub preserves: Vec<RecoveryStateClass>,
    /// Accessibility posture.
    pub accessibility: AccessibilityPosture,
}

/// Accessibility posture for the whole center surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CenterAccessibility {
    /// Whether the surface is fully reachable and operable by keyboard.
    pub keyboard_complete: bool,
    /// Screen-reader summary of the center.
    pub screen_reader_summary: String,
    /// Whether focus is trapped safely within the surface.
    pub focus_trap_safe: bool,
}

/// Synthesized crash-loop recovery center (output record).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopRecoveryCenter {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable center identifier.
    pub center_id: String,
    /// Source signal id.
    pub signal_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Trigger that opened the center.
    pub trigger_class: CrashLoopTriggerClass,
    /// Visible crash id.
    pub crash_id: String,
    /// Visible build id.
    pub build_id: String,
    /// Restore class for the failed launch.
    pub restore_class: RestoreClass,
    /// Suspected fault domain.
    pub suspected_fault_domain: FaultDomainClass,
    /// Opaque fault-domain ref.
    pub fault_domain_ref: String,
    /// Last attempted reopen mode.
    pub last_reopen_mode: ReopenModeClass,
    /// Restart strikes observed in the active window.
    pub strike_count: u32,
    /// Automatic restart budget for the window.
    pub strike_budget: u32,
    /// Whether the invisible restart loop was suppressed by routing here.
    pub silent_restart_suppressed: bool,
    /// Project Doctor finding ref.
    pub doctor_finding_ref: String,
    /// Bounded, command-backed recovery choices.
    pub recovery_choices: Vec<RecoveryChoice>,
    /// Evidence-only and checkpoint/diff entry points.
    pub evidence_entry_points: Vec<EvidenceEntryPoint>,
    /// Recent changes surfaced as candidate suspects.
    pub recent_changes: Vec<RecentChange>,
    /// Evidence refs cited by the center.
    pub evidence: Vec<CrashLoopEvidenceRef>,
    /// Whether any destructive cleanup is suggested (must be false).
    pub destructive_cleanup_suggested: bool,
    /// Support packet ref that consumes this center.
    pub support_packet_ref: String,
    /// Accessibility posture for the surface.
    pub accessibility: CenterAccessibility,
}

impl CrashLoopRecoveryCenter {
    /// Returns the recovery choice with the given class, if present.
    pub fn choice(&self, class: RecoveryChoiceClass) -> Option<&RecoveryChoice> {
        self.recovery_choices
            .iter()
            .find(|choice| choice.choice_class == class)
    }

    /// Returns true when the center routes a real breach into a visible,
    /// bounded surface that preserves user-owned state.
    pub fn is_bounded_recovery_surface(&self) -> bool {
        self.silent_restart_suppressed
            && !self.destructive_cleanup_suggested
            && !self.crash_id.is_empty()
            && !self.build_id.is_empty()
            && !self.recovery_choices.is_empty()
            && self
                .recovery_choices
                .iter()
                .all(|choice| !choice.deletes_user_owned_state && choice.narrower_than_full_reset)
    }
}

/// One support-projection row for a recovery choice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopSupportChoiceRow {
    /// Choice id.
    pub choice_id: String,
    /// Choice class.
    pub choice_class: RecoveryChoiceClass,
    /// Command id.
    pub command_id: String,
    /// Whether the command requires review.
    pub requires_review: bool,
    /// Whether the command requires explicit confirmation.
    pub requires_explicit_confirmation: bool,
    /// Whether the command enforces no-silent-rerun.
    pub no_silent_rerun: bool,
    /// Disposition summary.
    pub disposition_summary: String,
    /// Preserved state classes.
    pub preserves: Vec<RecoveryStateClass>,
    /// Whether the choice deletes user-owned state.
    pub deletes_user_owned_state: bool,
    /// Targeted recent-change ref, when applicable.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub targets_recent_change_ref: Option<String>,
    /// Screen-reader label.
    pub screen_reader_label: String,
}

/// One support-projection row for an evidence entry point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopSupportEntryRow {
    /// Entry id.
    pub entry_id: String,
    /// Entry class.
    pub entry_class: EvidenceEntryClass,
    /// Command id.
    pub command_id: String,
    /// Summary.
    pub summary: String,
    /// Preserved state classes.
    pub preserves: Vec<RecoveryStateClass>,
}

/// One support-projection row for a recent change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopSupportChangeRow {
    /// Change id.
    pub change_id: String,
    /// Change class.
    pub change_class: RecentChangeClass,
    /// Opaque subject ref.
    pub subject_ref: String,
    /// Reviewer-facing label.
    pub display_label: String,
    /// Whether disabling the change is reversible.
    pub reversible: bool,
}

/// Metadata-safe support projection of one crash-loop recovery center.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrashLoopRecoverySupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Doc ref the packet quotes.
    pub doc_ref: String,
    /// Boundary schema ref the packet mirrors.
    pub schema_ref: String,
    /// Center id projected by the packet.
    pub center_id: String,
    /// Visible crash id.
    pub crash_id: String,
    /// Visible build id.
    pub build_id: String,
    /// Restore class.
    pub restore_class: RestoreClass,
    /// Suspected fault domain.
    pub suspected_fault_domain: FaultDomainClass,
    /// Opaque fault-domain ref.
    pub fault_domain_ref: String,
    /// Last attempted reopen mode.
    pub last_reopen_mode: ReopenModeClass,
    /// Trigger class.
    pub trigger_class: CrashLoopTriggerClass,
    /// Project Doctor finding ref.
    pub doctor_finding_ref: String,
    /// Whether the invisible restart loop was suppressed.
    pub silent_restart_suppressed: bool,
    /// Recovery choice rows.
    pub choice_rows: Vec<CrashLoopSupportChoiceRow>,
    /// Evidence entry rows.
    pub entry_rows: Vec<CrashLoopSupportEntryRow>,
    /// Recent change rows.
    pub change_rows: Vec<CrashLoopSupportChangeRow>,
    /// Evidence refs cited by the packet.
    pub evidence_refs: Vec<String>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether destructive cleanup is suggested (must be false).
    pub destructive_cleanup_suggested: bool,
}

impl CrashLoopRecoverySupportPacket {
    /// Returns true when the packet preserves the bounded crash-loop contract
    /// and stays metadata-only.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_cleanup_suggested
            && self.silent_restart_suppressed
            && !self.crash_id.is_empty()
            && !self.build_id.is_empty()
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && !self.choice_rows.is_empty()
            && !self.evidence_refs.is_empty()
            && self
                .choice_rows
                .iter()
                .all(|row| !row.deletes_user_owned_state)
    }

    /// Renders a support-safe, screen-reader-legible plaintext view of the
    /// packet. The view carries opaque ids and closed-vocabulary tokens only.
    pub fn render_support_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("Crash-loop recovery center\n");
        out.push_str(&format!("  crash id: {}\n", self.crash_id));
        out.push_str(&format!("  build id: {}\n", self.build_id));
        out.push_str(&format!(
            "  restore class: {}\n",
            self.restore_class.as_str()
        ));
        out.push_str(&format!(
            "  suspected fault domain: {}\n",
            self.suspected_fault_domain.as_str()
        ));
        out.push_str(&format!(
            "  last reopen mode: {}\n",
            self.last_reopen_mode.as_str()
        ));
        out.push_str(&format!(
            "  silent restart suppressed: {}\n",
            self.silent_restart_suppressed
        ));
        out.push_str("  recovery choices:\n");
        for row in &self.choice_rows {
            out.push_str(&format!(
                "    - {} ({}) no_silent_rerun={} review={}\n",
                row.choice_class.as_str(),
                row.command_id,
                row.no_silent_rerun,
                row.requires_review
            ));
        }
        if !self.entry_rows.is_empty() {
            out.push_str("  evidence entry points:\n");
            for row in &self.entry_rows {
                out.push_str(&format!(
                    "    - {} ({})\n",
                    row.entry_class.as_str(),
                    row.command_id
                ));
            }
        }
        out
    }
}

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashLoopViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashLoopValidationReport {
    /// Validation failures.
    pub violations: Vec<CrashLoopViolation>,
}

impl CrashLoopValidationReport {
    /// Returns true when the report contains a violation with the given check id.
    pub fn contains(&self, check_id: &str) -> bool {
        self.violations
            .iter()
            .any(|violation| violation.check_id == check_id)
    }
}

impl fmt::Display for CrashLoopValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} crash-loop violation(s)", self.violations.len())
    }
}

impl Error for CrashLoopValidationReport {}

/// Loads a crash-loop signal from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`CrashLoopSignal`].
pub fn load_signal(yaml: &str) -> Result<CrashLoopSignal, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Crash-loop recovery center beta evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct CrashLoopRecoveryCenterBeta;

impl CrashLoopRecoveryCenterBeta {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a [`CrashLoopSignal`] without synthesizing a center.
    ///
    /// # Errors
    ///
    /// Returns [`CrashLoopValidationReport`] when the signal is malformed,
    /// omits visible crash/build identity, fails to represent a genuine
    /// restart-budget breach, or carries unsafe evidence.
    pub fn validate_signal(
        &self,
        signal: &CrashLoopSignal,
    ) -> Result<(), CrashLoopValidationReport> {
        let violations = validate_signal(signal);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(CrashLoopValidationReport { violations })
        }
    }

    /// Evaluates one signal into a bounded crash-loop recovery center.
    ///
    /// The synthesized center always offers Safe mode, Open without restore,
    /// Open logs, Export crash manifest, and Report issue, plus a targeted
    /// disable choice for every recent extension and profile/layout change.
    /// Recovered drafts and rollbackable state become distinct evidence
    /// entry points. No choice deletes user-owned state, and Safe mode /
    /// Open without restore honor no-silent-rerun semantics for privileged
    /// or mutating sessions.
    ///
    /// # Errors
    ///
    /// Returns [`CrashLoopValidationReport`] when the signal fails validation.
    pub fn evaluate(
        &self,
        signal: &CrashLoopSignal,
    ) -> Result<CrashLoopRecoveryCenter, CrashLoopValidationReport> {
        let violations = validate_signal(signal);
        if !violations.is_empty() {
            return Err(CrashLoopValidationReport { violations });
        }

        let support_packet_ref = format!(
            "support:crash-loop-center:{}",
            sanitize_ref(&signal.signal_id)
        );
        let mut focus_order: u32 = 0;
        let mut recovery_choices = Vec::new();

        // Session re-entry choices first.
        recovery_choices.push(build_base_choice(
            signal,
            RecoveryChoiceClass::EnterSafeMode,
            &mut focus_order,
        ));
        recovery_choices.push(build_base_choice(
            signal,
            RecoveryChoiceClass::OpenWithoutRestore,
            &mut focus_order,
        ));

        // Targeted suspect-disable choices, narrowing blast radius.
        for change in &signal.recent_changes {
            if change.change_class.is_extension_change() {
                recovery_choices.push(build_disable_choice(
                    RecoveryChoiceClass::DisableRecentlyChangedExtension,
                    change,
                    &mut focus_order,
                ));
            } else if change.change_class.is_profile_or_layout_change() {
                recovery_choices.push(build_disable_choice(
                    RecoveryChoiceClass::DisableRecentlyChangedProfileOrLayout,
                    change,
                    &mut focus_order,
                ));
            }
        }

        // Inspect/export/report choices last.
        recovery_choices.push(build_base_choice(
            signal,
            RecoveryChoiceClass::OpenLogs,
            &mut focus_order,
        ));
        recovery_choices.push(build_base_choice(
            signal,
            RecoveryChoiceClass::ExportCrashManifest,
            &mut focus_order,
        ));
        recovery_choices.push(build_base_choice(
            signal,
            RecoveryChoiceClass::ReportIssue,
            &mut focus_order,
        ));

        let evidence_entry_points = signal
            .recovered_artifacts
            .iter()
            .map(|artifact| build_entry_point(artifact, &mut focus_order))
            .collect::<Vec<_>>();

        let center = CrashLoopRecoveryCenter {
            schema_version: CRASH_LOOP_RECOVERY_SCHEMA_VERSION,
            record_kind: CRASH_LOOP_RECOVERY_CENTER_RECORD_KIND.to_owned(),
            center_id: format!("center:crash-loop:{}", sanitize_ref(&signal.signal_id)),
            signal_ref: signal.signal_id.clone(),
            captured_at: signal.captured_at.clone(),
            trigger_class: signal.trigger_class,
            crash_id: signal.crash_id.clone(),
            build_id: signal.build_id.clone(),
            restore_class: signal.restore_class,
            suspected_fault_domain: signal.suspected_fault_domain,
            fault_domain_ref: signal.fault_domain_ref.clone(),
            last_reopen_mode: signal.last_reopen_mode,
            strike_count: signal.strike_count,
            strike_budget: signal.strike_budget,
            silent_restart_suppressed: true,
            doctor_finding_ref: signal.doctor_finding_ref.clone(),
            recovery_choices,
            evidence_entry_points,
            recent_changes: signal.recent_changes.clone(),
            evidence: signal.evidence.clone(),
            destructive_cleanup_suggested: false,
            support_packet_ref,
            accessibility: CenterAccessibility {
                keyboard_complete: true,
                screen_reader_summary: format!(
                    "Recovery center for crash {} on build {}: {} bounded recovery choices, restore class {}, suspected fault domain {}.",
                    signal.crash_id,
                    signal.build_id,
                    RecoveryChoiceClass::BASE_CHOICES.len()
                        + signal
                            .recent_changes
                            .iter()
                            .filter(|change| {
                                change.change_class.is_extension_change()
                                    || change.change_class.is_profile_or_layout_change()
                            })
                            .count(),
                    signal.restore_class.as_str(),
                    signal.suspected_fault_domain.as_str()
                ),
                focus_trap_safe: true,
            },
        };

        Ok(center)
    }

    /// Builds the metadata-safe support projection for one center.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        center: &CrashLoopRecoveryCenter,
    ) -> CrashLoopRecoverySupportPacket {
        CrashLoopRecoverySupportPacket {
            record_kind: CRASH_LOOP_RECOVERY_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: CRASH_LOOP_RECOVERY_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: center.captured_at.clone(),
            doc_ref: CRASH_LOOP_RECOVERY_DOC_REF.to_owned(),
            schema_ref: CRASH_LOOP_RECOVERY_SCHEMA_REF.to_owned(),
            center_id: center.center_id.clone(),
            crash_id: center.crash_id.clone(),
            build_id: center.build_id.clone(),
            restore_class: center.restore_class,
            suspected_fault_domain: center.suspected_fault_domain,
            fault_domain_ref: center.fault_domain_ref.clone(),
            last_reopen_mode: center.last_reopen_mode,
            trigger_class: center.trigger_class,
            doctor_finding_ref: center.doctor_finding_ref.clone(),
            silent_restart_suppressed: center.silent_restart_suppressed,
            choice_rows: center
                .recovery_choices
                .iter()
                .map(CrashLoopSupportChoiceRow::from)
                .collect(),
            entry_rows: center
                .evidence_entry_points
                .iter()
                .map(CrashLoopSupportEntryRow::from)
                .collect(),
            change_rows: center
                .recent_changes
                .iter()
                .map(CrashLoopSupportChangeRow::from)
                .collect(),
            evidence_refs: center
                .evidence
                .iter()
                .map(|evidence| evidence.evidence_ref.clone())
                .collect(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_cleanup_suggested: center.destructive_cleanup_suggested,
        }
    }
}

impl From<&RecoveryChoice> for CrashLoopSupportChoiceRow {
    fn from(choice: &RecoveryChoice) -> Self {
        Self {
            choice_id: choice.choice_id.clone(),
            choice_class: choice.choice_class,
            command_id: choice.command.command_id.clone(),
            requires_review: choice.command.requires_review,
            requires_explicit_confirmation: choice.command.requires_explicit_confirmation,
            no_silent_rerun: choice.command.no_silent_rerun,
            disposition_summary: choice.disposition_summary.clone(),
            preserves: choice.preserves.clone(),
            deletes_user_owned_state: choice.deletes_user_owned_state,
            targets_recent_change_ref: choice.targets_recent_change_ref.clone(),
            screen_reader_label: choice.accessibility.screen_reader_label.clone(),
        }
    }
}

impl From<&EvidenceEntryPoint> for CrashLoopSupportEntryRow {
    fn from(entry: &EvidenceEntryPoint) -> Self {
        Self {
            entry_id: entry.entry_id.clone(),
            entry_class: entry.entry_class,
            command_id: entry.command.command_id.clone(),
            summary: entry.summary.clone(),
            preserves: entry.preserves.clone(),
        }
    }
}

impl From<&RecentChange> for CrashLoopSupportChangeRow {
    fn from(change: &RecentChange) -> Self {
        Self {
            change_id: change.change_id.clone(),
            change_class: change.change_class,
            subject_ref: change.subject_ref.clone(),
            display_label: change.display_label.clone(),
            reversible: change.reversible,
        }
    }
}

fn build_base_choice(
    signal: &CrashLoopSignal,
    class: RecoveryChoiceClass,
    focus_order: &mut u32,
) -> RecoveryChoice {
    let gated =
        class.is_session_reentry() && signal.session_sensitivity_class.requires_no_silent_rerun();
    let command = CommandRef {
        command_id: class.command_id().to_owned(),
        requires_review: gated,
        // Re-entry always requires explicit confirmation; never silent.
        requires_explicit_confirmation: class.is_session_reentry(),
        no_silent_rerun: class.is_session_reentry(),
    };
    let (title, description, disposition_summary, preserves) = base_choice_copy(class);
    let order = *focus_order;
    *focus_order += 1;
    RecoveryChoice {
        choice_id: format!(
            "choice:{}:{}",
            class.as_str(),
            sanitize_ref(&signal.signal_id)
        ),
        choice_class: class,
        command,
        title: title.to_owned(),
        description: description.to_owned(),
        disposition_summary: disposition_summary.to_owned(),
        preserves,
        deletes_user_owned_state: false,
        narrower_than_full_reset: true,
        targets_recent_change_ref: None,
        accessibility: AccessibilityPosture {
            keyboard_complete: true,
            screen_reader_label: format!("{title}. {disposition_summary}"),
            focus_order: order,
        },
    }
}

fn build_disable_choice(
    class: RecoveryChoiceClass,
    change: &RecentChange,
    focus_order: &mut u32,
) -> RecoveryChoice {
    let command = CommandRef {
        command_id: class.command_id().to_owned(),
        requires_review: false,
        requires_explicit_confirmation: true,
        // Disabling a suspect change is reversible and does not re-run the
        // session, but it is never applied silently.
        no_silent_rerun: false,
    };
    let title = format!("Disable recently changed: {}", change.display_label);
    let disposition_summary = format!(
        "Reversibly disables {} and keeps your files, drafts, and settings; re-enable it after the next clean launch.",
        change.display_label
    );
    let order = *focus_order;
    *focus_order += 1;
    RecoveryChoice {
        choice_id: format!(
            "choice:{}:{}",
            class.as_str(),
            sanitize_ref(&change.change_id)
        ),
        choice_class: class,
        command,
        title: title.clone(),
        description: format!(
            "{} was changed at {} and may be the suspect for this crash loop.",
            change.display_label, change.observed_at
        ),
        disposition_summary: disposition_summary.clone(),
        preserves: vec![
            RecoveryStateClass::UserAuthoredFiles,
            RecoveryStateClass::RecoveredDrafts,
            RecoveryStateClass::SessionRestoreStore,
        ],
        deletes_user_owned_state: false,
        narrower_than_full_reset: true,
        targets_recent_change_ref: Some(change.change_id.clone()),
        accessibility: AccessibilityPosture {
            keyboard_complete: true,
            screen_reader_label: format!("{title}. {disposition_summary}"),
            focus_order: order,
        },
    }
}

fn build_entry_point(artifact: &RecoveredArtifact, focus_order: &mut u32) -> EvidenceEntryPoint {
    let order = *focus_order;
    *focus_order += 1;
    EvidenceEntryPoint {
        entry_id: format!(
            "entry:{}:{}",
            artifact.entry_class.as_str(),
            sanitize_ref(&artifact.artifact_id)
        ),
        entry_class: artifact.entry_class,
        command: CommandRef {
            command_id: artifact.entry_class.command_id().to_owned(),
            requires_review: false,
            requires_explicit_confirmation: false,
            no_silent_rerun: false,
        },
        summary: artifact.summary.clone(),
        preserves: artifact.preserves.clone(),
        accessibility: AccessibilityPosture {
            keyboard_complete: true,
            screen_reader_label: format!(
                "Inspect {}: {}",
                artifact.entry_class.as_str(),
                artifact.summary
            ),
            focus_order: order,
        },
    }
}

fn base_choice_copy(
    class: RecoveryChoiceClass,
) -> (
    &'static str,
    &'static str,
    &'static str,
    Vec<RecoveryStateClass>,
) {
    match class {
        RecoveryChoiceClass::EnterSafeMode => (
            "Enter safe mode",
            "Launch with extensions, restore replay, and heavy background services held back.",
            "Preserves your files, drafts, and restore records; defers extensions and background services until you exit safe mode.",
            vec![
                RecoveryStateClass::UserAuthoredFiles,
                RecoveryStateClass::OpenBufferSelection,
                RecoveryStateClass::RecoveredDrafts,
                RecoveryStateClass::SessionRestoreStore,
                RecoveryStateClass::CheckpointHistory,
            ],
        ),
        RecoveryChoiceClass::OpenWithoutRestore => (
            "Open without restore",
            "Open the workspace with restore replay disabled while keeping the restore records intact.",
            "Preserves your restore records and drafts; defers replay so nothing is re-run silently.",
            vec![
                RecoveryStateClass::UserAuthoredFiles,
                RecoveryStateClass::RecoveredDrafts,
                RecoveryStateClass::SessionRestoreStore,
                RecoveryStateClass::CheckpointHistory,
            ],
        ),
        RecoveryChoiceClass::OpenLogs => (
            "Open logs",
            "Open the local logs for the crashed launches.",
            "Preserves everything; opens read-only local logs for inspection.",
            vec![RecoveryStateClass::UserAuthoredFiles],
        ),
        RecoveryChoiceClass::ExportCrashManifest => (
            "Export crash manifest",
            "Export a metadata-safe crash manifest for support, attaching evidence by reference.",
            "Preserves everything; writes a redacted manifest to the support export store.",
            vec![
                RecoveryStateClass::UserAuthoredFiles,
                RecoveryStateClass::SupportExportStore,
            ],
        ),
        RecoveryChoiceClass::ReportIssue => (
            "Report issue",
            "Open the report-issue flow with the crash and build identity carried by reference.",
            "Preserves everything; nothing is uploaded until you explicitly confirm.",
            vec![RecoveryStateClass::UserAuthoredFiles],
        ),
        // Disable choices are built by `build_disable_choice`; provide a safe
        // default so the function remains total.
        RecoveryChoiceClass::DisableRecentlyChangedExtension
        | RecoveryChoiceClass::DisableRecentlyChangedProfileOrLayout => (
            "Disable recently changed item",
            "Reversibly disable a recently changed extension, profile, or layout.",
            "Preserves your files and drafts; reversibly disables the suspect.",
            vec![RecoveryStateClass::UserAuthoredFiles],
        ),
    }
}

fn validate_signal(signal: &CrashLoopSignal) -> Vec<CrashLoopViolation> {
    let mut violations = Vec::new();

    if signal.schema_version != CRASH_LOOP_RECOVERY_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "crash_loop.schema_version",
            &signal.signal_id,
            "signal schema_version must be 1",
        );
    }
    if signal.record_kind != CRASH_LOOP_SIGNAL_RECORD_KIND {
        push_violation(
            &mut violations,
            "crash_loop.record_kind",
            &signal.signal_id,
            "signal record_kind must be crash_loop_signal_record",
        );
    }
    if signal.signal_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "crash_loop.signal_id_empty",
            &signal.signal_id,
            "signal_id must be non-empty",
        );
    }
    if signal.crash_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "crash_loop.crash_id_missing",
            &signal.signal_id,
            "crash_id must be visible and non-empty",
        );
    }
    if signal.build_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "crash_loop.build_id_missing",
            &signal.signal_id,
            "build_id must be visible and non-empty",
        );
    }
    if signal.crash_envelope_ref.trim().is_empty() || signal.crash_manifest_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "crash_loop.crash_refs_missing",
            &signal.signal_id,
            "crash_envelope_ref and crash_manifest_ref must be non-empty",
        );
    }
    if signal.fault_domain_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "crash_loop.fault_domain_ref_missing",
            &signal.signal_id,
            "fault_domain_ref must be non-empty",
        );
    }
    if !signal.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "crash_loop.doctor_finding_ref_missing",
            &signal.signal_id,
            "signal must cite a Project Doctor finding ref",
        );
    }

    // A budget-breach trigger must represent a genuine breach so the center is
    // never opened as an invisible spurious loop.
    if signal.trigger_class.is_budget_breach()
        && !(signal.strike_budget > 0 && signal.strike_count >= signal.strike_budget)
    {
        push_violation(
            &mut violations,
            "crash_loop.budget_breach_not_proven",
            &signal.signal_id,
            "budget-breach triggers require strike_count >= strike_budget > 0",
        );
    }

    if signal.evidence.is_empty() {
        push_violation(
            &mut violations,
            "crash_loop.evidence_missing",
            &signal.signal_id,
            "signal must cite at least one metadata-safe evidence ref",
        );
    }
    for evidence in &signal.evidence {
        if matches!(
            evidence.data_class,
            EvidenceDataClass::CodeAdjacent | EvidenceDataClass::SecretBearing
        ) {
            push_violation(
                &mut violations,
                "crash_loop.evidence_private_data_class",
                &evidence.evidence_ref,
                "evidence must stay metadata or environment-adjacent",
            );
        }
        if evidence.redaction_class != RedactionClass::MetadataSafeDefault {
            push_violation(
                &mut violations,
                "crash_loop.evidence_redaction_not_metadata_safe",
                &evidence.evidence_ref,
                "evidence must use metadata_safe_default redaction",
            );
        }
    }

    let mut change_ids: BTreeSet<&str> = BTreeSet::new();
    for change in &signal.recent_changes {
        if change.change_id.trim().is_empty()
            || change.subject_ref.trim().is_empty()
            || change.display_label.trim().is_empty()
        {
            push_violation(
                &mut violations,
                "crash_loop.recent_change_field_empty",
                &change.change_id,
                "recent change change_id, subject_ref, and display_label must be non-empty",
            );
        }
        if !change_ids.insert(change.change_id.as_str()) {
            push_violation(
                &mut violations,
                "crash_loop.duplicate_recent_change_id",
                &change.change_id,
                "duplicate recent change_id is forbidden",
            );
        }
    }

    let mut artifact_ids: BTreeSet<&str> = BTreeSet::new();
    for artifact in &signal.recovered_artifacts {
        if artifact.artifact_id.trim().is_empty() || artifact.summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "crash_loop.recovered_artifact_field_empty",
                &artifact.artifact_id,
                "recovered artifact artifact_id and summary must be non-empty",
            );
        }
        if artifact.preserves.is_empty() {
            push_violation(
                &mut violations,
                "crash_loop.recovered_artifact_preserves_missing",
                &artifact.artifact_id,
                "recovered artifact must name at least one preserved state class",
            );
        }
        if !artifact_ids.insert(artifact.artifact_id.as_str()) {
            push_violation(
                &mut violations,
                "crash_loop.duplicate_recovered_artifact_id",
                &artifact.artifact_id,
                "duplicate recovered artifact_id is forbidden",
            );
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<CrashLoopViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(CrashLoopViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

fn sanitize_ref(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}
