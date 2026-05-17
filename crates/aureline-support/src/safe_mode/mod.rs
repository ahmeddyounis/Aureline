//! Safe-mode runtime profile, entry/exit transitions, and support-export projection.
//!
//! The safe-mode runtime profile is the bounded recovery posture a blocked
//! user can enter after a startup crash loop, an exhausted restart budget,
//! a policy-forced narrowing, or an explicit diagnostics request. The
//! profile names which hosts, services, and surfaces are disabled or
//! narrowed and why, while it preserves local editing, basic navigation,
//! and the diagnostics/export paths the recovery ladder relies on.
//!
//! This module mints two typed records that mirror the boundary schema at
//! [`/schemas/support/safe_mode_profile.schema.json`]:
//!
//! - [`SafeModeProfile`] declares the active safe-mode posture as a
//!   typed list of [`NarrowedHost`], [`NarrowedService`], and
//!   [`NarrowedSurface`] rows, plus the [`PreservedCapabilityClass`] set
//!   and the [`SafeModeReturnPath`] back to a fuller mode.
//! - [`SafeModeTransition`] records each [`TransitionClass::Enter`] or
//!   [`TransitionClass::Exit`] event. Every transition pins
//!   `user_owned_state_deleted = false` and
//!   `durable_state_deleted = false` so an entry or exit can never silently
//!   erase user-owned state.
//!
//! [`SafeModeEvaluator::support_packet`] folds one profile and its
//! transitions into a [`SafeModeSupportPacket`] that the support-export
//! pipeline can consume verbatim. The packet is metadata-only: it cites
//! ids, refs, and closed-vocabulary tokens, never raw payloads,
//! credentials, paths, or ambient authority.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a safe-mode profile record.
pub const SAFE_MODE_PROFILE_RECORD_KIND: &str = "safe_mode_profile_record";

/// Stable record-kind tag for a safe-mode transition record.
pub const SAFE_MODE_TRANSITION_RECORD_KIND: &str = "safe_mode_transition_record";

/// Stable record-kind tag for the metadata-safe support projection.
pub const SAFE_MODE_SUPPORT_PACKET_RECORD_KIND: &str = "safe_mode_support_packet_record";

/// Frozen schema version for safe-mode beta records.
pub const SAFE_MODE_PROFILE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const SAFE_MODE_PROFILE_SCHEMA_REF: &str = "schemas/support/safe_mode_profile.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const SAFE_MODE_PROFILE_DOC_REF: &str = "docs/support/m3/safe_mode_beta.md";

/// Closed safe-mode profile-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeProfileClass {
    /// Entered after the startup crash-loop budget was exhausted.
    PostCrashLoopProfile,
    /// User explicitly chose safe mode from a recovery surface.
    UserInvokedProfile,
    /// Managed policy or an admin override forced safe mode.
    PolicyForcedProfile,
    /// Diagnostics mode chosen to bound side effects during reproduction.
    DiagnosticsProfile,
}

impl SafeModeProfileClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostCrashLoopProfile => "post_crash_loop_profile",
            Self::UserInvokedProfile => "user_invoked_profile",
            Self::PolicyForcedProfile => "policy_forced_profile",
            Self::DiagnosticsProfile => "diagnostics_profile",
        }
    }
}

/// Closed host-class vocabulary for safe-mode narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeHostClass {
    /// Extension host process.
    ExtensionHost,
    /// Language runtime supervisor (formatters, language servers).
    LanguageRuntimeHost,
    /// AI runtime host.
    AiRuntimeHost,
    /// Remote helper / remote-attach host.
    RemoteHelperHost,
    /// Background indexer / heavy worker host.
    BackgroundIndexerHost,
    /// Live docs-pack fetcher host.
    DocsPackFetcherHost,
}

impl SafeModeHostClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionHost => "extension_host",
            Self::LanguageRuntimeHost => "language_runtime_host",
            Self::AiRuntimeHost => "ai_runtime_host",
            Self::RemoteHelperHost => "remote_helper_host",
            Self::BackgroundIndexerHost => "background_indexer_host",
            Self::DocsPackFetcherHost => "docs_pack_fetcher_host",
        }
    }
}

/// Closed service-class vocabulary for safe-mode narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeServiceClass {
    /// Telemetry upload pipelines.
    TelemetryUpload,
    /// Background rebuild / re-index workers.
    BackgroundRebuild,
    /// Live docs-pack fetches.
    DocsPackLiveFetch,
    /// AI inference requests.
    AiInferenceRequest,
    /// Extension marketplace sync.
    ExtensionMarketplaceSync,
    /// Session restore auto-replay.
    SessionRestoreAutoReplay,
    /// Remote auto-reattach.
    RemoteAutoReattach,
    /// Managed-policy sync push.
    ManagedPolicySyncPush,
    /// Third-party extension auto-activation.
    ExtensionAutoActivation,
}

impl SafeModeServiceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TelemetryUpload => "telemetry_upload",
            Self::BackgroundRebuild => "background_rebuild",
            Self::DocsPackLiveFetch => "docs_pack_live_fetch",
            Self::AiInferenceRequest => "ai_inference_request",
            Self::ExtensionMarketplaceSync => "extension_marketplace_sync",
            Self::SessionRestoreAutoReplay => "session_restore_auto_replay",
            Self::RemoteAutoReattach => "remote_auto_reattach",
            Self::ManagedPolicySyncPush => "managed_policy_sync_push",
            Self::ExtensionAutoActivation => "extension_auto_activation",
        }
    }
}

/// Closed surface-class vocabulary for safe-mode narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeSurfaceClass {
    /// Extension marketplace panel.
    ExtensionMarketplacePanel,
    /// AI assistant panel.
    AiAssistantPanel,
    /// Remote collaboration panel.
    RemoteCollaborationPanel,
    /// Live-share surface.
    LiveShareSurface,
    /// Auto-update panel.
    AutoUpdatePanel,
    /// Managed-admin panel.
    ManagedAdminPanel,
}

impl SafeModeSurfaceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionMarketplacePanel => "extension_marketplace_panel",
            Self::AiAssistantPanel => "ai_assistant_panel",
            Self::RemoteCollaborationPanel => "remote_collaboration_panel",
            Self::LiveShareSurface => "live_share_surface",
            Self::AutoUpdatePanel => "auto_update_panel",
            Self::ManagedAdminPanel => "managed_admin_panel",
        }
    }
}

/// Closed disposition class describing how a host/service/surface is narrowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeDispositionClass {
    /// The host, service, or surface is fully disabled.
    Disabled,
    /// Reduced to read-only behavior.
    NarrowedToReadOnly,
    /// Reduced to local-only behavior; no network or remote side-effect.
    NarrowedToLocalOnly,
    /// Reduced to a reviewer-only view; user-action is gated behind review.
    NarrowedToReviewerView,
}

impl SafeModeDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::NarrowedToReadOnly => "narrowed_to_read_only",
            Self::NarrowedToLocalOnly => "narrowed_to_local_only",
            Self::NarrowedToReviewerView => "narrowed_to_reviewer_view",
        }
    }
}

/// Closed reason-class vocabulary explaining why a narrowing applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeReasonClass {
    /// Startup crash loop was detected.
    StartupCrashLoopDetected,
    /// Restart budget for the lane was exceeded.
    RestartBudgetExceeded,
    /// Compatibility regression was suspected.
    CompatibilityRegressionSuspected,
    /// Restore replay was unsafe.
    UnsafeReplayDetected,
    /// Managed policy forced safe mode.
    PolicyForcedSafeMode,
    /// Diagnostics mode was requested to bound side effects.
    DiagnosticsModeRequested,
    /// User explicitly opted in to the narrowing.
    UserOptInRequest,
}

impl SafeModeReasonClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartupCrashLoopDetected => "startup_crash_loop_detected",
            Self::RestartBudgetExceeded => "restart_budget_exceeded",
            Self::CompatibilityRegressionSuspected => "compatibility_regression_suspected",
            Self::UnsafeReplayDetected => "unsafe_replay_detected",
            Self::PolicyForcedSafeMode => "policy_forced_safe_mode",
            Self::DiagnosticsModeRequested => "diagnostics_mode_requested",
            Self::UserOptInRequest => "user_opt_in_request",
        }
    }
}

/// Closed preserved-capability vocabulary. A safe-mode profile must
/// preserve at least [`PreservedCapabilityClass::LocalEditing`] so the
/// blocked user can keep working on user-authored files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedCapabilityClass {
    /// Local editing of user-authored files.
    LocalEditing,
    /// Basic navigation (file tree, quick-open, go-to-definition for local files).
    BasicNavigation,
    /// Local search.
    LocalSearch,
    /// Local Git operations (status, diff, commit).
    LocalGitOperations,
    /// Local diagnostics export and support-bundle preview.
    LocalDiagnosticsExport,
    /// Support-bundle preview surface.
    SupportBundlePreview,
    /// Project Doctor surfaces remain reachable.
    ProjectDoctorSurfaces,
    /// Explicit safe-mode exit action is reachable.
    SafeModeExitAction,
}

impl PreservedCapabilityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditing => "local_editing",
            Self::BasicNavigation => "basic_navigation",
            Self::LocalSearch => "local_search",
            Self::LocalGitOperations => "local_git_operations",
            Self::LocalDiagnosticsExport => "local_diagnostics_export",
            Self::SupportBundlePreview => "support_bundle_preview",
            Self::ProjectDoctorSurfaces => "project_doctor_surfaces",
            Self::SafeModeExitAction => "safe_mode_exit_action",
        }
    }
}

/// Closed preserved-state vocabulary. Mirrors the recovery-ladder seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Selection, caret, and scroll position for open buffers.
    OpenBufferSelection,
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
}

impl PreservedStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredFiles => "user_authored_files",
            Self::OpenBufferSelection => "open_buffer_selection",
            Self::DurableWorkspaceIndexes => "durable_workspace_indexes",
            Self::WorkspaceTrustStore => "workspace_trust_store",
            Self::CredentialStore => "credential_store",
            Self::SessionRestoreStore => "session_restore_store",
            Self::SupportExportStore => "support_export_store",
        }
    }
}

/// Fuller runtime posture a safe-mode return path aims to restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullerModeClass {
    /// Full mode with normal runtime services enabled.
    FullMode,
    /// Restore-enabled workspace entry.
    RestoreEnabledEntry,
    /// Lane re-enabled under normal admission checks.
    LaneReadmitted,
}

impl FullerModeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullMode => "full_mode",
            Self::RestoreEnabledEntry => "restore_enabled_entry",
            Self::LaneReadmitted => "lane_readmitted",
        }
    }
}

/// Closed transition-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionClass {
    /// Safe mode was entered.
    Enter,
    /// Safe mode was exited toward a fuller mode.
    Exit,
}

impl TransitionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enter => "enter",
            Self::Exit => "exit",
        }
    }
}

/// Closed entry-reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeEntryReasonClass {
    /// Startup crash loop exceeded the configured strike budget.
    CrashLoopDetected,
    /// Restart budget for a supervised lane was exceeded.
    RestartBudgetExceeded,
    /// User explicitly chose safe mode from the recovery surface.
    ExplicitUserChoice,
    /// Managed policy or admin override forced safe mode.
    PolicyForced,
    /// Restore replay was unsafe.
    UnsafeReplayDetected,
    /// Diagnostics mode was requested.
    DiagnosticsModeRequested,
}

impl SafeModeEntryReasonClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashLoopDetected => "crash_loop_detected",
            Self::RestartBudgetExceeded => "restart_budget_exceeded",
            Self::ExplicitUserChoice => "explicit_user_choice",
            Self::PolicyForced => "policy_forced",
            Self::UnsafeReplayDetected => "unsafe_replay_detected",
            Self::DiagnosticsModeRequested => "diagnostics_mode_requested",
        }
    }
}

/// Closed exit-reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeModeExitReasonClass {
    /// User confirmed a return to full mode.
    UserConfirmedFullMode,
    /// The Project Doctor finding behind the entry was reviewed.
    DoctorFindingReviewed,
    /// Managed policy released the safe-mode forcing.
    PolicyReleased,
    /// User exported diagnostics evidence and then exited.
    ExportedEvidenceAndExited,
}

impl SafeModeExitReasonClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserConfirmedFullMode => "user_confirmed_full_mode",
            Self::DoctorFindingReviewed => "doctor_finding_reviewed",
            Self::PolicyReleased => "policy_released",
            Self::ExportedEvidenceAndExited => "exported_evidence_and_exited",
        }
    }
}

/// Action surfaced to leave safe mode toward a fuller mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeReturnAction {
    /// Stable action identifier.
    pub action_id: String,
    /// Whether review is required before execution.
    pub requires_review: bool,
    /// Reviewer-safe summary of what the action does.
    pub summary: String,
}

/// Declared return path from safe mode toward a fuller mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeReturnPath {
    /// Fuller mode targeted by the return path.
    pub fuller_mode_class: FullerModeClass,
    /// Action used to leave safe mode.
    pub return_action: SafeModeReturnAction,
    /// Conditions that must hold before restoring fuller mode.
    pub restore_conditions: Vec<String>,
}

/// One narrowed host row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowedHost {
    /// Opaque host identifier safe for support and release packets.
    pub host_id: String,
    /// Host class.
    pub host_class: SafeModeHostClass,
    /// Disposition applied to the host.
    pub disposition_class: SafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: SafeModeReasonClass,
    /// Reviewer-safe summary that excludes paths and private content.
    pub narrowing_summary: String,
}

/// One narrowed service row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowedService {
    /// Opaque service identifier.
    pub service_id: String,
    /// Service class.
    pub service_class: SafeModeServiceClass,
    /// Disposition applied to the service.
    pub disposition_class: SafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: SafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One narrowed surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowedSurface {
    /// Opaque surface identifier.
    pub surface_id: String,
    /// Surface class.
    pub surface_class: SafeModeSurfaceClass,
    /// Disposition applied to the surface.
    pub disposition_class: SafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: SafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// Evidence reference that justifies a profile or transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeEvidenceRef {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Evidence kind or source role.
    pub evidence_kind: String,
    /// Reviewer-safe summary without raw private content.
    pub summary: String,
}

/// Safe-mode runtime profile record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeProfile {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable profile identifier.
    pub profile_id: String,
    /// Profile class.
    pub profile_class: SafeModeProfileClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Project Doctor finding that justified the profile.
    pub doctor_finding_ref: String,
    /// Support packet ref that consumes the profile.
    pub support_packet_ref: String,
    /// Declared narrowed hosts.
    pub declared_hosts: Vec<NarrowedHost>,
    /// Declared narrowed services.
    pub declared_services: Vec<NarrowedService>,
    /// Declared narrowed surfaces.
    pub declared_surfaces: Vec<NarrowedSurface>,
    /// Preserved capability classes (must include local editing).
    pub preserved_capabilities: Vec<PreservedCapabilityClass>,
    /// Preserved state classes (must include user-authored files).
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Whether the profile carries any destructive reset.
    pub destructive_resets_present: bool,
    /// Return path toward fuller mode.
    pub return_path: SafeModeReturnPath,
    /// Evidence refs justifying the profile.
    pub evidence: Vec<SafeModeEvidenceRef>,
}

/// Safe-mode transition (enter or exit) record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeTransition {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable transition identifier.
    pub transition_id: String,
    /// Transition class.
    pub transition_class: TransitionClass,
    /// Profile id this transition binds to.
    pub profile_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Entry reason class (required when `transition_class == Enter`).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub entry_reason_class: Option<SafeModeEntryReasonClass>,
    /// Exit reason class (required when `transition_class == Exit`).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exit_reason_class: Option<SafeModeExitReasonClass>,
    /// Preserved state classes observed at the transition boundary.
    pub preserved_state_classes_observed: Vec<PreservedStateClass>,
    /// Whether user-owned state was deleted by the transition.
    pub user_owned_state_deleted: bool,
    /// Whether durable non-disposable state was deleted by the transition.
    pub durable_state_deleted: bool,
    /// Support packet ref that consumes the transition.
    pub support_packet_ref: String,
}

/// Metadata-safe support projection joining one profile and its transitions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeSupportPacket {
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
    /// Profile id projected by the packet.
    pub profile_id: String,
    /// Profile class projected by the packet.
    pub profile_class: SafeModeProfileClass,
    /// Project Doctor finding ref the packet cites.
    pub doctor_finding_ref: String,
    /// Narrowed host rows.
    pub host_rows: Vec<SafeModeSupportHostRow>,
    /// Narrowed service rows.
    pub service_rows: Vec<SafeModeSupportServiceRow>,
    /// Narrowed surface rows.
    pub surface_rows: Vec<SafeModeSupportSurfaceRow>,
    /// Preserved capability classes.
    pub preserved_capabilities: Vec<PreservedCapabilityClass>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Transition rows that bind the profile.
    pub transition_rows: Vec<SafeModeSupportTransitionRow>,
    /// Return path summary.
    pub return_path: SafeModeReturnPath,
    /// Evidence refs cited by the packet.
    pub evidence_refs: Vec<String>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether the projection records a destructive reset.
    pub destructive_resets_present: bool,
}

impl SafeModeSupportPacket {
    /// Returns true when the packet preserves the bounded safe-mode contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_resets_present
            && self
                .preserved_capabilities
                .contains(&PreservedCapabilityClass::LocalEditing)
            && self
                .preserved_state_classes
                .contains(&PreservedStateClass::UserAuthoredFiles)
            && !self.host_rows.is_empty()
            && !self.service_rows.is_empty()
            && !self.evidence_refs.is_empty()
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && self
                .transition_rows
                .iter()
                .all(SafeModeSupportTransitionRow::is_export_safe)
    }
}

/// One narrowed-host row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeSupportHostRow {
    /// Opaque host id.
    pub host_id: String,
    /// Host class.
    pub host_class: SafeModeHostClass,
    /// Disposition class.
    pub disposition_class: SafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: SafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One narrowed-service row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeSupportServiceRow {
    /// Opaque service id.
    pub service_id: String,
    /// Service class.
    pub service_class: SafeModeServiceClass,
    /// Disposition class.
    pub disposition_class: SafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: SafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One narrowed-surface row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeSupportSurfaceRow {
    /// Opaque surface id.
    pub surface_id: String,
    /// Surface class.
    pub surface_class: SafeModeSurfaceClass,
    /// Disposition class.
    pub disposition_class: SafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: SafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One transition row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeModeSupportTransitionRow {
    /// Transition id.
    pub transition_id: String,
    /// Transition class.
    pub transition_class: TransitionClass,
    /// Entry reason class (set when transition class is enter).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub entry_reason_class: Option<SafeModeEntryReasonClass>,
    /// Exit reason class (set when transition class is exit).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exit_reason_class: Option<SafeModeExitReasonClass>,
    /// Preserved state classes observed at the boundary.
    pub preserved_state_classes_observed: Vec<PreservedStateClass>,
    /// Whether user-owned state was deleted.
    pub user_owned_state_deleted: bool,
    /// Whether durable state was deleted.
    pub durable_state_deleted: bool,
}

impl SafeModeSupportTransitionRow {
    /// Returns true when this transition row preserves the contract.
    pub fn is_export_safe(&self) -> bool {
        !self.user_owned_state_deleted
            && !self.durable_state_deleted
            && self
                .preserved_state_classes_observed
                .contains(&PreservedStateClass::UserAuthoredFiles)
            && match self.transition_class {
                TransitionClass::Enter => {
                    self.entry_reason_class.is_some() && self.exit_reason_class.is_none()
                }
                TransitionClass::Exit => {
                    self.exit_reason_class.is_some() && self.entry_reason_class.is_none()
                }
            }
    }
}

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeModeViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeModeValidationReport {
    /// Validation failures.
    pub violations: Vec<SafeModeViolation>,
}

impl fmt::Display for SafeModeValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} safe-mode violation(s)", self.violations.len())
    }
}

impl Error for SafeModeValidationReport {}

/// Loads a safe-mode profile from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`SafeModeProfile`].
pub fn load_safe_mode_profile(yaml: &str) -> Result<SafeModeProfile, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a safe-mode transition from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`SafeModeTransition`].
pub fn load_safe_mode_transition(yaml: &str) -> Result<SafeModeTransition, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Safe-mode beta evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct SafeModeEvaluator;

impl SafeModeEvaluator {
    /// Creates a new safe-mode evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a [`SafeModeProfile`].
    ///
    /// # Errors
    ///
    /// Returns [`SafeModeValidationReport`] when the profile omits the
    /// required preservation, declares a destructive reset, fails to
    /// explain a narrowing, or duplicates host/service/surface ids.
    pub fn validate_profile(
        &self,
        profile: &SafeModeProfile,
    ) -> Result<(), SafeModeValidationReport> {
        let violations = validate_profile(profile);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(SafeModeValidationReport { violations })
        }
    }

    /// Validates a [`SafeModeTransition`].
    ///
    /// # Errors
    ///
    /// Returns [`SafeModeValidationReport`] when the transition deletes
    /// user-owned or durable state, omits the user-authored-files
    /// preservation observation, or mixes entry/exit reason classes
    /// against the declared transition class.
    pub fn validate_transition(
        &self,
        transition: &SafeModeTransition,
    ) -> Result<(), SafeModeValidationReport> {
        let violations = validate_transition(transition);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(SafeModeValidationReport { violations })
        }
    }

    /// Validates a transition against the profile it binds to.
    ///
    /// # Errors
    ///
    /// Returns [`SafeModeValidationReport`] when the transition's
    /// `profile_ref` does not match the supplied profile.
    pub fn validate_transition_against_profile(
        &self,
        profile: &SafeModeProfile,
        transition: &SafeModeTransition,
    ) -> Result<(), SafeModeValidationReport> {
        let mut violations = validate_transition(transition);
        if transition.profile_ref != profile.profile_id {
            push_violation(
                &mut violations,
                "safe_mode.transition_profile_ref_mismatch",
                &transition.transition_id,
                "transition profile_ref must equal the bound profile_id",
            );
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(SafeModeValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`SafeModeValidationReport`] when the profile or any
    /// transition fails validation, or when a transition's `profile_ref`
    /// does not bind to the supplied profile.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        profile: &SafeModeProfile,
        transitions: &[SafeModeTransition],
    ) -> Result<SafeModeSupportPacket, SafeModeValidationReport> {
        let mut violations = validate_profile(profile);
        for transition in transitions {
            violations.extend(validate_transition(transition));
            if transition.profile_ref != profile.profile_id {
                push_violation(
                    &mut violations,
                    "safe_mode.transition_profile_ref_mismatch",
                    &transition.transition_id,
                    "transition profile_ref must equal the bound profile_id",
                );
            }
        }
        if !violations.is_empty() {
            return Err(SafeModeValidationReport { violations });
        }

        let host_rows = profile
            .declared_hosts
            .iter()
            .map(SafeModeSupportHostRow::from)
            .collect::<Vec<_>>();
        let service_rows = profile
            .declared_services
            .iter()
            .map(SafeModeSupportServiceRow::from)
            .collect::<Vec<_>>();
        let surface_rows = profile
            .declared_surfaces
            .iter()
            .map(SafeModeSupportSurfaceRow::from)
            .collect::<Vec<_>>();
        let transition_rows = transitions
            .iter()
            .map(SafeModeSupportTransitionRow::from)
            .collect::<Vec<_>>();
        let evidence_refs = profile
            .evidence
            .iter()
            .map(|evidence| evidence.evidence_ref.clone())
            .collect::<Vec<_>>();

        Ok(SafeModeSupportPacket {
            record_kind: SAFE_MODE_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: SAFE_MODE_PROFILE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: SAFE_MODE_PROFILE_DOC_REF.to_owned(),
            schema_ref: SAFE_MODE_PROFILE_SCHEMA_REF.to_owned(),
            profile_id: profile.profile_id.clone(),
            profile_class: profile.profile_class,
            doctor_finding_ref: profile.doctor_finding_ref.clone(),
            host_rows,
            service_rows,
            surface_rows,
            preserved_capabilities: profile.preserved_capabilities.clone(),
            preserved_state_classes: profile.preserved_state_classes.clone(),
            transition_rows,
            return_path: profile.return_path.clone(),
            evidence_refs,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
        })
    }
}

impl From<&NarrowedHost> for SafeModeSupportHostRow {
    fn from(host: &NarrowedHost) -> Self {
        Self {
            host_id: host.host_id.clone(),
            host_class: host.host_class,
            disposition_class: host.disposition_class,
            reason_class: host.reason_class,
            narrowing_summary: host.narrowing_summary.clone(),
        }
    }
}

impl From<&NarrowedService> for SafeModeSupportServiceRow {
    fn from(service: &NarrowedService) -> Self {
        Self {
            service_id: service.service_id.clone(),
            service_class: service.service_class,
            disposition_class: service.disposition_class,
            reason_class: service.reason_class,
            narrowing_summary: service.narrowing_summary.clone(),
        }
    }
}

impl From<&NarrowedSurface> for SafeModeSupportSurfaceRow {
    fn from(surface: &NarrowedSurface) -> Self {
        Self {
            surface_id: surface.surface_id.clone(),
            surface_class: surface.surface_class,
            disposition_class: surface.disposition_class,
            reason_class: surface.reason_class,
            narrowing_summary: surface.narrowing_summary.clone(),
        }
    }
}

impl From<&SafeModeTransition> for SafeModeSupportTransitionRow {
    fn from(transition: &SafeModeTransition) -> Self {
        Self {
            transition_id: transition.transition_id.clone(),
            transition_class: transition.transition_class,
            entry_reason_class: transition.entry_reason_class,
            exit_reason_class: transition.exit_reason_class,
            preserved_state_classes_observed: transition.preserved_state_classes_observed.clone(),
            user_owned_state_deleted: transition.user_owned_state_deleted,
            durable_state_deleted: transition.durable_state_deleted,
        }
    }
}

fn validate_profile(profile: &SafeModeProfile) -> Vec<SafeModeViolation> {
    let mut violations = Vec::new();

    if profile.schema_version != SAFE_MODE_PROFILE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "safe_mode.schema_version",
            &profile.profile_id,
            "profile schema_version must be 1",
        );
    }
    if profile.record_kind != SAFE_MODE_PROFILE_RECORD_KIND {
        push_violation(
            &mut violations,
            "safe_mode.record_kind",
            &profile.profile_id,
            "profile record_kind must be safe_mode_profile_record",
        );
    }
    if profile.profile_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.profile_id_empty",
            &profile.profile_id,
            "profile_id must be non-empty",
        );
    }
    if !profile.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "safe_mode.doctor_finding_ref_missing",
            &profile.profile_id,
            "profile must cite a Project Doctor finding ref",
        );
    }
    if profile.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.support_packet_ref_missing",
            &profile.profile_id,
            "profile must cite a support_packet_ref",
        );
    }
    if profile.declared_hosts.is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.declared_hosts_missing",
            &profile.profile_id,
            "profile must declare at least one narrowed host",
        );
    }
    if profile.declared_services.is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.declared_services_missing",
            &profile.profile_id,
            "profile must declare at least one narrowed service",
        );
    }
    if profile.evidence.is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.evidence_missing",
            &profile.profile_id,
            "profile must cite at least one evidence ref",
        );
    }
    if !profile
        .preserved_capabilities
        .contains(&PreservedCapabilityClass::LocalEditing)
    {
        push_violation(
            &mut violations,
            "safe_mode.local_editing_must_be_preserved",
            &profile.profile_id,
            "profile must preserve local editing",
        );
    }
    if !profile
        .preserved_state_classes
        .contains(&PreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "safe_mode.user_authored_files_must_be_preserved",
            &profile.profile_id,
            "profile must preserve user-authored files",
        );
    }
    if profile.destructive_resets_present {
        push_violation(
            &mut violations,
            "safe_mode.destructive_reset_declared",
            &profile.profile_id,
            "safe-mode profiles must not declare a destructive reset",
        );
    }
    if profile.return_path.return_action.action_id.trim().is_empty()
        || profile.return_path.restore_conditions.is_empty()
        || profile.return_path.return_action.summary.trim().is_empty()
    {
        push_violation(
            &mut violations,
            "safe_mode.return_path_missing",
            &profile.profile_id,
            "profile must name a return action, summary, and restore conditions",
        );
    }

    let mut host_ids: BTreeSet<&str> = BTreeSet::new();
    for host in &profile.declared_hosts {
        if !host_ids.insert(host.host_id.as_str()) {
            push_violation(
                &mut violations,
                "safe_mode.duplicate_host_id",
                &host.host_id,
                "duplicate host_id is forbidden",
            );
        }
        if host.host_id.trim().is_empty() || host.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "safe_mode.host_field_empty",
                &host.host_id,
                "host_id and narrowing_summary must be non-empty",
            );
        }
    }
    let mut service_ids: BTreeSet<&str> = BTreeSet::new();
    for service in &profile.declared_services {
        if !service_ids.insert(service.service_id.as_str()) {
            push_violation(
                &mut violations,
                "safe_mode.duplicate_service_id",
                &service.service_id,
                "duplicate service_id is forbidden",
            );
        }
        if service.service_id.trim().is_empty() || service.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "safe_mode.service_field_empty",
                &service.service_id,
                "service_id and narrowing_summary must be non-empty",
            );
        }
    }
    let mut surface_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &profile.declared_surfaces {
        if !surface_ids.insert(surface.surface_id.as_str()) {
            push_violation(
                &mut violations,
                "safe_mode.duplicate_surface_id",
                &surface.surface_id,
                "duplicate surface_id is forbidden",
            );
        }
        if surface.surface_id.trim().is_empty() || surface.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "safe_mode.surface_field_empty",
                &surface.surface_id,
                "surface_id and narrowing_summary must be non-empty",
            );
        }
    }

    violations
}

fn validate_transition(transition: &SafeModeTransition) -> Vec<SafeModeViolation> {
    let mut violations = Vec::new();

    if transition.schema_version != SAFE_MODE_PROFILE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "safe_mode.transition_schema_version",
            &transition.transition_id,
            "transition schema_version must be 1",
        );
    }
    if transition.record_kind != SAFE_MODE_TRANSITION_RECORD_KIND {
        push_violation(
            &mut violations,
            "safe_mode.transition_record_kind",
            &transition.transition_id,
            "transition record_kind must be safe_mode_transition_record",
        );
    }
    if transition.transition_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.transition_id_empty",
            &transition.transition_id,
            "transition_id must be non-empty",
        );
    }
    if transition.profile_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.transition_profile_ref_empty",
            &transition.transition_id,
            "profile_ref must be non-empty",
        );
    }
    if transition.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "safe_mode.transition_support_packet_ref_empty",
            &transition.transition_id,
            "support_packet_ref must be non-empty",
        );
    }
    if !transition
        .preserved_state_classes_observed
        .contains(&PreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "safe_mode.transition_must_preserve_user_authored_files",
            &transition.transition_id,
            "transition must observe preservation of user-authored files",
        );
    }
    if transition.user_owned_state_deleted {
        push_violation(
            &mut violations,
            "safe_mode.transition_deletes_user_owned_state",
            &transition.transition_id,
            "safe-mode entry/exit must not delete user-owned state",
        );
    }
    if transition.durable_state_deleted {
        push_violation(
            &mut violations,
            "safe_mode.transition_deletes_durable_state",
            &transition.transition_id,
            "safe-mode entry/exit must not delete durable non-disposable state",
        );
    }
    match transition.transition_class {
        TransitionClass::Enter => {
            if transition.entry_reason_class.is_none() {
                push_violation(
                    &mut violations,
                    "safe_mode.transition_enter_missing_entry_reason",
                    &transition.transition_id,
                    "enter transitions must declare an entry_reason_class",
                );
            }
            if transition.exit_reason_class.is_some() {
                push_violation(
                    &mut violations,
                    "safe_mode.transition_enter_has_exit_reason",
                    &transition.transition_id,
                    "enter transitions must not declare an exit_reason_class",
                );
            }
        }
        TransitionClass::Exit => {
            if transition.exit_reason_class.is_none() {
                push_violation(
                    &mut violations,
                    "safe_mode.transition_exit_missing_exit_reason",
                    &transition.transition_id,
                    "exit transitions must declare an exit_reason_class",
                );
            }
            if transition.entry_reason_class.is_some() {
                push_violation(
                    &mut violations,
                    "safe_mode.transition_exit_has_entry_reason",
                    &transition.transition_id,
                    "exit transitions must not declare an entry_reason_class",
                );
            }
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<SafeModeViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(SafeModeViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
