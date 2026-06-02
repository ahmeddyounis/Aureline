//! Hardened safe-mode runtime profile, retained capabilities, accessibility
//! posture, and support guidance for the M4 stable lane.
//!
//! This module promotes the safe-mode beta profile into a hardened, stable
//! contract that every recovery-ladder consumer, support-export surface, and
//! Help/About proof card can read verbatim. It adds three typed rows the beta
//! does not own:
//!
//! - [`RetainedCapabilityRecord`] — one row per preserved capability with a
//!   closed [`RetainedCapabilityClass`], a reviewer-safe rationale, and a
//!   user-facing support-guidance string so blocked users know what they can
//!   still do in safe mode.
//! - [`AccessibilityPostureRow`] — one row per capability or surface that
//!   attests keyboard, screen-reader, IME/grapheme/bidi, zoom, high-contrast,
//!   and reduced-motion behavior instead of treating accessibility as a
//!   post-pass.
//! - [`RecoveryLadderBindingRow`] — one row per recovery-ladder rung that
//!   binds the hardened profile to a [`RecoveryLadderRungClass`] so the
//!   recovery surface knows which actions are available and which are
//!   narrowed below stable.
//!
//! The [`HardenedSafeModeProfile`] wraps the beta shape with a new stable
//! record kind, a bumped schema version, and the additional row sets. The
//! [`HardenedSafeModeEvaluator`] validates that:
//!
//! - every required retained-capability class is admitted exactly once,
//! - every admitted capability carries a non-empty support-guidance string,
//! - every touched surface carries an accessibility-posture row with the six
//!   mandatory dimensions,
//! - every recovery-ladder binding cites a closed rung class and a stable
//!   support-class label,
//! - the profile never declares a destructive reset,
//! - the profile always preserves `local_editing` and `user_authored_files`.
//!
//! The [`HardenedSafeModeSupportPacket`] folds the profile, retained
//! capabilities, accessibility postures, support guidance, and recovery-ladder
//! bindings into one metadata-safe projection that the support-export pipeline
//! can consume verbatim. The packet is metadata-only: it cites ids, refs, and
//! closed-vocabulary tokens, never raw payloads, credentials, paths, or ambient
//! authority.
//!
//! The boundary schema is at
//! `/schemas/support/harden_the_safe_mode_runtime_profile_retained_capabilities.schema.json`.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the hardened safe-mode profile record.
pub const HARDENED_SAFE_MODE_PROFILE_RECORD_KIND: &str =
    "hardened_safe_mode_runtime_profile_retained_capabilities_record";

/// Stable record-kind tag for the hardened safe-mode support packet.
pub const HARDENED_SAFE_MODE_SUPPORT_PACKET_RECORD_KIND: &str =
    "hardened_safe_mode_runtime_profile_retained_capabilities_stable_packet";

/// Integer schema version for the hardened safe-mode records.
pub const HARDENED_SAFE_MODE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const HARDENED_SAFE_MODE_SCHEMA_REF: &str =
    "schemas/support/harden_the_safe_mode_runtime_profile_retained_capabilities.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const HARDENED_SAFE_MODE_DOC_REF: &str =
    "docs/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const HARDENED_SAFE_MODE_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const HARDENED_SAFE_MODE_FIXTURE_DIR: &str =
    "fixtures/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed hardened safe-mode profile-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedSafeModeProfileClass {
    /// Entered after the startup crash-loop budget was exhausted.
    PostCrashLoopProfile,
    /// User explicitly chose safe mode from a recovery surface.
    UserInvokedProfile,
    /// Managed policy or an admin override forced safe mode.
    PolicyForcedProfile,
    /// Diagnostics mode chosen to bound side effects during reproduction.
    DiagnosticsProfile,
}

impl HardenedSafeModeProfileClass {
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

/// Closed retained-capability vocabulary. Every hardened profile must admit
/// each variant exactly once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedCapabilityClass {
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

impl RetainedCapabilityClass {
    /// Every required retained capability, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::LocalEditing,
        Self::BasicNavigation,
        Self::LocalSearch,
        Self::LocalGitOperations,
        Self::LocalDiagnosticsExport,
        Self::SupportBundlePreview,
        Self::ProjectDoctorSurfaces,
        Self::SafeModeExitAction,
    ];

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

/// Closed accessibility-dimension vocabulary. Every accessibility-posture row
/// must address all six dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityDimensionClass {
    /// Keyboard navigation and focus behavior.
    Keyboard,
    /// Screen-reader narration and role/label behavior.
    ScreenReader,
    /// IME, grapheme-cluster, bidirectional-text behavior.
    ImeGraphemeBidi,
    /// Zoom and reflow behavior.
    Zoom,
    /// High-contrast theme behavior.
    HighContrast,
    /// Reduced-motion preference behavior.
    ReducedMotion,
}

impl AccessibilityDimensionClass {
    /// Every required accessibility dimension, in declaration order.
    pub const REQUIRED: [Self; 6] = [
        Self::Keyboard,
        Self::ScreenReader,
        Self::ImeGraphemeBidi,
        Self::Zoom,
        Self::HighContrast,
        Self::ReducedMotion,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::ImeGraphemeBidi => "ime_grapheme_bidi",
            Self::Zoom => "zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
        }
    }
}

/// Closed accessibility-posture vocabulary per dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityPostureClass {
    /// The dimension is fully supported in safe mode.
    FullySupported,
    /// The dimension works but with degraded fidelity in safe mode.
    DegradedButFunctional,
    /// The dimension does not apply to the capability or surface.
    NotApplicable,
    /// The dimension is blocked because the capability or surface is disabled.
    BlockedInSafeMode,
}

impl AccessibilityPostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::DegradedButFunctional => "degraded_but_functional",
            Self::NotApplicable => "not_applicable",
            Self::BlockedInSafeMode => "blocked_in_safe_mode",
        }
    }
}

/// Closed recovery-ladder rung vocabulary. Every hardened profile must bind
/// each rung that the recovery surface surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderRungClass {
    /// Safe-mode rung — the profile itself.
    SafeMode,
    /// Open without restore rung.
    OpenWithoutRestore,
    /// Disable recently changed extension rung.
    DisableRecentExtension,
    /// Disable recently changed profile or layout rung.
    DisableRecentLayout,
    /// Open logs rung.
    OpenLogs,
    /// Export crash manifest rung.
    ExportCrashManifest,
    /// Report issue rung.
    ReportIssue,
}

impl RecoveryLadderRungClass {
    /// Every required recovery-ladder rung, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::SafeMode,
        Self::OpenWithoutRestore,
        Self::DisableRecentExtension,
        Self::DisableRecentLayout,
        Self::OpenLogs,
        Self::ExportCrashManifest,
        Self::ReportIssue,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::DisableRecentExtension => "disable_recent_extension",
            Self::DisableRecentLayout => "disable_recent_layout",
            Self::OpenLogs => "open_logs",
            Self::ExportCrashManifest => "export_crash_manifest",
            Self::ReportIssue => "report_issue",
        }
    }
}

/// Closed support-class vocabulary for recovery-ladder bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedSafeModeSupportClass {
    /// The rung is launch-stable in safe mode.
    LaunchStable,
    /// The rung is available but narrowed below launch-stable.
    LaunchStableBelow,
    /// The rung is beta-grade only.
    BetaGradeOnly,
    /// The rung is preview-only.
    PreviewOnly,
    /// The rung is unsupported in this profile.
    Unsupported,
}

impl HardenedSafeModeSupportClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchStable => "launch_stable",
            Self::LaunchStableBelow => "launch_stable_below",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Closed disposition class describing how a host/service/surface is narrowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedSafeModeDispositionClass {
    /// The host, service, or surface is fully disabled.
    Disabled,
    /// Reduced to read-only behavior.
    NarrowedToReadOnly,
    /// Reduced to local-only behavior; no network or remote side-effect.
    NarrowedToLocalOnly,
    /// Reduced to a reviewer-only view; user-action is gated behind review.
    NarrowedToReviewerView,
}

impl HardenedSafeModeDispositionClass {
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
pub enum HardenedSafeModeReasonClass {
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

impl HardenedSafeModeReasonClass {
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

/// Closed host-class vocabulary for safe-mode narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedSafeModeHostClass {
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

impl HardenedSafeModeHostClass {
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
pub enum HardenedSafeModeServiceClass {
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

impl HardenedSafeModeServiceClass {
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
pub enum HardenedSafeModeSurfaceClass {
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

impl HardenedSafeModeSurfaceClass {
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

/// Closed preserved-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenedPreservedStateClass {
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

impl HardenedPreservedStateClass {
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
pub enum HardenedFullerModeClass {
    /// Full mode with normal runtime services enabled.
    FullMode,
    /// Restore-enabled workspace entry.
    RestoreEnabledEntry,
    /// Lane re-enabled under normal admission checks.
    LaneReadmitted,
}

impl HardenedFullerModeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullMode => "full_mode",
            Self::RestoreEnabledEntry => "restore_enabled_entry",
            Self::LaneReadmitted => "lane_readmitted",
        }
    }
}

// ---------------------------------------------------------------------------
// Row types
// ---------------------------------------------------------------------------

/// One retained capability with support guidance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedCapabilityRecord {
    /// Stable capability class.
    pub capability_class: RetainedCapabilityClass,
    /// Reviewer-safe rationale for why the capability is retained.
    pub rationale: String,
    /// User-facing support-guidance string explaining what the user can do.
    pub support_guidance: String,
    /// Whether the capability is explicitly tested in safe mode.
    pub explicitly_tested: bool,
}

/// One accessibility posture row for a capability or surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityPostureRow {
    /// Target id — opaque reference to the capability or surface this row
    /// describes.
    pub target_id: String,
    /// Target kind — `capability` or `surface`.
    pub target_kind: String,
    /// Accessibility dimension.
    pub dimension: AccessibilityDimensionClass,
    /// Posture for this dimension on the target.
    pub posture: AccessibilityPostureClass,
    /// Reviewer-safe explanation of the posture.
    pub explanation: String,
}

/// One recovery-ladder binding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderBindingRow {
    /// Recovery-ladder rung class.
    pub rung_class: RecoveryLadderRungClass,
    /// Support class for this rung in the current profile.
    pub support_class: HardenedSafeModeSupportClass,
    /// Whether the rung requires review before execution.
    pub requires_review: bool,
    /// Reviewer-safe summary of the rung behavior in safe mode.
    pub rung_summary: String,
    /// Evidence refs justifying the support-class label.
    pub evidence_refs: Vec<String>,
}

/// One narrowed host row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedNarrowedHost {
    /// Opaque host identifier safe for support and release packets.
    pub host_id: String,
    /// Host class.
    pub host_class: HardenedSafeModeHostClass,
    /// Disposition applied to the host.
    pub disposition_class: HardenedSafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: HardenedSafeModeReasonClass,
    /// Reviewer-safe summary that excludes paths and private content.
    pub narrowing_summary: String,
}

/// One narrowed service row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedNarrowedService {
    /// Opaque service identifier.
    pub service_id: String,
    /// Service class.
    pub service_class: HardenedSafeModeServiceClass,
    /// Disposition applied to the service.
    pub disposition_class: HardenedSafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: HardenedSafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One narrowed surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedNarrowedSurface {
    /// Opaque surface identifier.
    pub surface_id: String,
    /// Surface class.
    pub surface_class: HardenedSafeModeSurfaceClass,
    /// Disposition applied to the surface.
    pub disposition_class: HardenedSafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: HardenedSafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// Action surfaced to leave safe mode toward a fuller mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeReturnAction {
    /// Stable action identifier.
    pub action_id: String,
    /// Whether review is required before execution.
    pub requires_review: bool,
    /// Reviewer-safe summary of what the action does.
    pub summary: String,
}

/// Declared return path from safe mode toward a fuller mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeReturnPath {
    /// Fuller mode targeted by the return path.
    pub fuller_mode_class: HardenedFullerModeClass,
    /// Action used to leave safe mode.
    pub return_action: HardenedSafeModeReturnAction,
    /// Conditions that must hold before restoring fuller mode.
    pub restore_conditions: Vec<String>,
}

/// Evidence reference that justifies a profile or transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeEvidenceRef {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Evidence kind or source role.
    pub evidence_kind: String,
    /// Reviewer-safe summary without raw private content.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Profile and support packet
// ---------------------------------------------------------------------------

/// Hardened safe-mode runtime profile record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeProfile {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable profile identifier.
    pub profile_id: String,
    /// Profile class.
    pub profile_class: HardenedSafeModeProfileClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Project Doctor finding that justified the profile.
    pub doctor_finding_ref: String,
    /// Support packet ref that consumes the profile.
    pub support_packet_ref: String,
    /// Declared narrowed hosts.
    pub declared_hosts: Vec<HardenedNarrowedHost>,
    /// Declared narrowed services.
    pub declared_services: Vec<HardenedNarrowedService>,
    /// Declared narrowed surfaces.
    pub declared_surfaces: Vec<HardenedNarrowedSurface>,
    /// Retained capability records (must admit every required class).
    pub retained_capabilities: Vec<RetainedCapabilityRecord>,
    /// Preserved state classes (must include user-authored files).
    pub preserved_state_classes: Vec<HardenedPreservedStateClass>,
    /// Accessibility posture rows (must cover every touched surface).
    pub accessibility_postures: Vec<AccessibilityPostureRow>,
    /// Recovery-ladder binding rows.
    pub recovery_ladder_bindings: Vec<RecoveryLadderBindingRow>,
    /// Whether the profile carries any destructive reset.
    pub destructive_resets_present: bool,
    /// Return path toward fuller mode.
    pub return_path: HardenedSafeModeReturnPath,
    /// Evidence refs justifying the profile.
    pub evidence: Vec<HardenedSafeModeEvidenceRef>,
}

/// Metadata-safe support projection for the hardened safe-mode profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeSupportPacket {
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
    pub profile_class: HardenedSafeModeProfileClass,
    /// Project Doctor finding ref the packet cites.
    pub doctor_finding_ref: String,
    /// Narrowed host rows.
    pub host_rows: Vec<HardenedSafeModeSupportHostRow>,
    /// Narrowed service rows.
    pub service_rows: Vec<HardenedSafeModeSupportServiceRow>,
    /// Narrowed surface rows.
    pub surface_rows: Vec<HardenedSafeModeSupportSurfaceRow>,
    /// Retained capability rows.
    pub retained_capability_rows: Vec<HardenedSafeModeSupportRetainedCapabilityRow>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<HardenedPreservedStateClass>,
    /// Accessibility posture rows.
    pub accessibility_posture_rows: Vec<AccessibilityPostureRow>,
    /// Recovery-ladder binding rows.
    pub recovery_ladder_binding_rows: Vec<RecoveryLadderBindingRow>,
    /// Return path summary.
    pub return_path: HardenedSafeModeReturnPath,
    /// Evidence refs cited by the packet.
    pub evidence_refs: Vec<String>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether the projection records a destructive reset.
    pub destructive_resets_present: bool,
}

impl HardenedSafeModeSupportPacket {
    /// Returns true when the packet preserves the bounded safe-mode contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_resets_present
            && self
                .preserved_state_classes
                .contains(&HardenedPreservedStateClass::UserAuthoredFiles)
            && !self.host_rows.is_empty()
            && !self.service_rows.is_empty()
            && !self.evidence_refs.is_empty()
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && self.retained_capability_rows.len() == RetainedCapabilityClass::REQUIRED.len()
            && !self.accessibility_posture_rows.is_empty()
            && self.recovery_ladder_binding_rows.len() == RecoveryLadderRungClass::REQUIRED.len()
    }
}

/// One narrowed-host row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeSupportHostRow {
    /// Opaque host id.
    pub host_id: String,
    /// Host class.
    pub host_class: HardenedSafeModeHostClass,
    /// Disposition class.
    pub disposition_class: HardenedSafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: HardenedSafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One narrowed-service row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeSupportServiceRow {
    /// Opaque service id.
    pub service_id: String,
    /// Service class.
    pub service_class: HardenedSafeModeServiceClass,
    /// Disposition class.
    pub disposition_class: HardenedSafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: HardenedSafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One narrowed-surface row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeSupportSurfaceRow {
    /// Opaque surface id.
    pub surface_id: String,
    /// Surface class.
    pub surface_class: HardenedSafeModeSurfaceClass,
    /// Disposition class.
    pub disposition_class: HardenedSafeModeDispositionClass,
    /// Reason class for the narrowing.
    pub reason_class: HardenedSafeModeReasonClass,
    /// Reviewer-safe summary.
    pub narrowing_summary: String,
}

/// One retained-capability row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenedSafeModeSupportRetainedCapabilityRow {
    /// Capability class.
    pub capability_class: RetainedCapabilityClass,
    /// Rationale.
    pub rationale: String,
    /// Support guidance.
    pub support_guidance: String,
    /// Explicitly tested flag.
    pub explicitly_tested: bool,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardenedSafeModeViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardenedSafeModeValidationReport {
    /// Validation failures.
    pub violations: Vec<HardenedSafeModeViolation>,
}

impl fmt::Display for HardenedSafeModeValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} hardened safe-mode violation(s)",
            self.violations.len()
        )
    }
}

impl Error for HardenedSafeModeValidationReport {}

/// Loads a hardened safe-mode profile from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`HardenedSafeModeProfile`].
pub fn load_hardened_safe_mode_profile(
    yaml: &str,
) -> Result<HardenedSafeModeProfile, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Hardened safe-mode stable evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct HardenedSafeModeEvaluator;

impl HardenedSafeModeEvaluator {
    /// Creates a new hardened safe-mode evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a [`HardenedSafeModeProfile`].
    ///
    /// # Errors
    ///
    /// Returns [`HardenedSafeModeValidationReport`] when the profile omits the
    /// required retained capabilities, declares a destructive reset, fails to
    /// explain a narrowing, duplicates host/service/surface ids, misses
    /// accessibility postures, or omits recovery-ladder bindings.
    pub fn validate_profile(
        &self,
        profile: &HardenedSafeModeProfile,
    ) -> Result<(), HardenedSafeModeValidationReport> {
        let violations = validate_profile(profile);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(HardenedSafeModeValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`HardenedSafeModeValidationReport`] when the profile fails
    /// validation.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        profile: &HardenedSafeModeProfile,
    ) -> Result<HardenedSafeModeSupportPacket, HardenedSafeModeValidationReport> {
        let violations = validate_profile(profile);
        if !violations.is_empty() {
            return Err(HardenedSafeModeValidationReport { violations });
        }

        let host_rows = profile
            .declared_hosts
            .iter()
            .map(HardenedSafeModeSupportHostRow::from)
            .collect::<Vec<_>>();
        let service_rows = profile
            .declared_services
            .iter()
            .map(HardenedSafeModeSupportServiceRow::from)
            .collect::<Vec<_>>();
        let surface_rows = profile
            .declared_surfaces
            .iter()
            .map(HardenedSafeModeSupportSurfaceRow::from)
            .collect::<Vec<_>>();
        let retained_capability_rows = profile
            .retained_capabilities
            .iter()
            .map(HardenedSafeModeSupportRetainedCapabilityRow::from)
            .collect::<Vec<_>>();
        let evidence_refs = profile
            .evidence
            .iter()
            .map(|evidence| evidence.evidence_ref.clone())
            .collect::<Vec<_>>();

        Ok(HardenedSafeModeSupportPacket {
            record_kind: HARDENED_SAFE_MODE_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: HARDENED_SAFE_MODE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: HARDENED_SAFE_MODE_DOC_REF.to_owned(),
            schema_ref: HARDENED_SAFE_MODE_SCHEMA_REF.to_owned(),
            profile_id: profile.profile_id.clone(),
            profile_class: profile.profile_class,
            doctor_finding_ref: profile.doctor_finding_ref.clone(),
            host_rows,
            service_rows,
            surface_rows,
            retained_capability_rows,
            preserved_state_classes: profile.preserved_state_classes.clone(),
            accessibility_posture_rows: profile.accessibility_postures.clone(),
            recovery_ladder_binding_rows: profile.recovery_ladder_bindings.clone(),
            return_path: profile.return_path.clone(),
            evidence_refs,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
        })
    }
}

impl From<&HardenedNarrowedHost> for HardenedSafeModeSupportHostRow {
    fn from(host: &HardenedNarrowedHost) -> Self {
        Self {
            host_id: host.host_id.clone(),
            host_class: host.host_class,
            disposition_class: host.disposition_class,
            reason_class: host.reason_class,
            narrowing_summary: host.narrowing_summary.clone(),
        }
    }
}

impl From<&HardenedNarrowedService> for HardenedSafeModeSupportServiceRow {
    fn from(service: &HardenedNarrowedService) -> Self {
        Self {
            service_id: service.service_id.clone(),
            service_class: service.service_class,
            disposition_class: service.disposition_class,
            reason_class: service.reason_class,
            narrowing_summary: service.narrowing_summary.clone(),
        }
    }
}

impl From<&HardenedNarrowedSurface> for HardenedSafeModeSupportSurfaceRow {
    fn from(surface: &HardenedNarrowedSurface) -> Self {
        Self {
            surface_id: surface.surface_id.clone(),
            surface_class: surface.surface_class,
            disposition_class: surface.disposition_class,
            reason_class: surface.reason_class,
            narrowing_summary: surface.narrowing_summary.clone(),
        }
    }
}

impl From<&RetainedCapabilityRecord> for HardenedSafeModeSupportRetainedCapabilityRow {
    fn from(record: &RetainedCapabilityRecord) -> Self {
        Self {
            capability_class: record.capability_class,
            rationale: record.rationale.clone(),
            support_guidance: record.support_guidance.clone(),
            explicitly_tested: record.explicitly_tested,
        }
    }
}

fn validate_profile(profile: &HardenedSafeModeProfile) -> Vec<HardenedSafeModeViolation> {
    let mut violations = Vec::new();

    if profile.schema_version != HARDENED_SAFE_MODE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "hardened_safe_mode.schema_version",
            &profile.profile_id,
            "profile schema_version must be 1",
        );
    }
    if profile.record_kind != HARDENED_SAFE_MODE_PROFILE_RECORD_KIND {
        push_violation(
            &mut violations,
            "hardened_safe_mode.record_kind",
            &profile.profile_id,
            "profile record_kind must be hardened_safe_mode_runtime_profile_retained_capabilities_record",
        );
    }
    if profile.profile_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "hardened_safe_mode.profile_id_empty",
            &profile.profile_id,
            "profile_id must be non-empty",
        );
    }
    if !profile.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "hardened_safe_mode.doctor_finding_ref_missing",
            &profile.profile_id,
            "profile must cite a Project Doctor finding ref",
        );
    }
    if profile.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "hardened_safe_mode.support_packet_ref_missing",
            &profile.profile_id,
            "profile must cite a support_packet_ref",
        );
    }
    if profile.declared_hosts.is_empty() {
        push_violation(
            &mut violations,
            "hardened_safe_mode.declared_hosts_missing",
            &profile.profile_id,
            "profile must declare at least one narrowed host",
        );
    }
    if profile.declared_services.is_empty() {
        push_violation(
            &mut violations,
            "hardened_safe_mode.declared_services_missing",
            &profile.profile_id,
            "profile must declare at least one narrowed service",
        );
    }
    if profile.evidence.is_empty() {
        push_violation(
            &mut violations,
            "hardened_safe_mode.evidence_missing",
            &profile.profile_id,
            "profile must cite at least one evidence ref",
        );
    }
    if !profile
        .preserved_state_classes
        .contains(&HardenedPreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "hardened_safe_mode.user_authored_files_must_be_preserved",
            &profile.profile_id,
            "profile must preserve user-authored files",
        );
    }
    if profile.destructive_resets_present {
        push_violation(
            &mut violations,
            "hardened_safe_mode.destructive_reset_declared",
            &profile.profile_id,
            "safe-mode profiles must not declare a destructive reset",
        );
    }
    if profile
        .return_path
        .return_action
        .action_id
        .trim()
        .is_empty()
        || profile.return_path.restore_conditions.is_empty()
        || profile.return_path.return_action.summary.trim().is_empty()
    {
        push_violation(
            &mut violations,
            "hardened_safe_mode.return_path_missing",
            &profile.profile_id,
            "profile must name a return action, summary, and restore conditions",
        );
    }

    // Validate retained capabilities admit every required class exactly once.
    let mut admitted: BTreeSet<RetainedCapabilityClass> = BTreeSet::new();
    for record in &profile.retained_capabilities {
        if !admitted.insert(record.capability_class) {
            push_violation(
                &mut violations,
                "hardened_safe_mode.duplicate_retained_capability",
                &profile.profile_id,
                format!(
                    "duplicate retained capability class: {}",
                    record.capability_class.as_str()
                ),
            );
        }
        if record.rationale.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.retained_capability_rationale_empty",
                &profile.profile_id,
                format!(
                    "retained capability {} must have a non-empty rationale",
                    record.capability_class.as_str()
                ),
            );
        }
        if record.support_guidance.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.retained_capability_guidance_empty",
                &profile.profile_id,
                format!(
                    "retained capability {} must have non-empty support guidance",
                    record.capability_class.as_str()
                ),
            );
        }
    }
    for required in RetainedCapabilityClass::REQUIRED {
        if !admitted.contains(&required) {
            push_violation(
                &mut violations,
                "hardened_safe_mode.required_retained_capability_missing",
                &profile.profile_id,
                format!(
                    "required retained capability class missing: {}",
                    required.as_str()
                ),
            );
        }
    }

    // Validate accessibility postures cover every touched surface.
    let mut touched_surface_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &profile.declared_surfaces {
        touched_surface_ids.insert(surface.surface_id.as_str());
    }
    // Also consider retained capabilities as touched targets.
    let mut touched_capability_ids: BTreeSet<&str> = BTreeSet::new();
    for record in &profile.retained_capabilities {
        touched_capability_ids.insert(record.capability_class.as_str());
    }

    let mut seen_surface_dimensions: BTreeSet<(&str, AccessibilityDimensionClass)> =
        BTreeSet::new();
    let mut seen_capability_dimensions: BTreeSet<(&str, AccessibilityDimensionClass)> =
        BTreeSet::new();
    for posture in &profile.accessibility_postures {
        if posture.explanation.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.accessibility_posture_explanation_empty",
                &profile.profile_id,
                "accessibility posture row must have a non-empty explanation",
            );
        }
        match posture.target_kind.as_str() {
            "surface" => {
                seen_surface_dimensions.insert((posture.target_id.as_str(), posture.dimension));
            }
            "capability" => {
                seen_capability_dimensions.insert((posture.target_id.as_str(), posture.dimension));
            }
            _ => {
                push_violation(
                    &mut violations,
                    "hardened_safe_mode.accessibility_posture_target_kind_invalid",
                    &profile.profile_id,
                    format!(
                        "accessibility posture target_kind must be 'surface' or 'capability', got: {}",
                        posture.target_kind
                    ),
                );
            }
        }
    }
    for surface_id in &touched_surface_ids {
        for dimension in AccessibilityDimensionClass::REQUIRED {
            if !seen_surface_dimensions.contains(&(surface_id, dimension)) {
                push_violation(
                    &mut violations,
                    "hardened_safe_mode.accessibility_posture_missing_for_surface",
                    &profile.profile_id,
                    format!(
                        "accessibility posture missing for surface {} dimension {}",
                        surface_id,
                        dimension.as_str()
                    ),
                );
            }
        }
    }
    for capability_id in &touched_capability_ids {
        for dimension in AccessibilityDimensionClass::REQUIRED {
            if !seen_capability_dimensions.contains(&(capability_id, dimension)) {
                push_violation(
                    &mut violations,
                    "hardened_safe_mode.accessibility_posture_missing_for_capability",
                    &profile.profile_id,
                    format!(
                        "accessibility posture missing for capability {} dimension {}",
                        capability_id,
                        dimension.as_str()
                    ),
                );
            }
        }
    }

    // Validate recovery-ladder bindings cover every required rung exactly once.
    let mut bound_rungs: BTreeSet<RecoveryLadderRungClass> = BTreeSet::new();
    for binding in &profile.recovery_ladder_bindings {
        if !bound_rungs.insert(binding.rung_class) {
            push_violation(
                &mut violations,
                "hardened_safe_mode.duplicate_recovery_ladder_rung",
                &profile.profile_id,
                format!(
                    "duplicate recovery-ladder rung class: {}",
                    binding.rung_class.as_str()
                ),
            );
        }
        if binding.rung_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.recovery_ladder_rung_summary_empty",
                &profile.profile_id,
                format!(
                    "recovery-ladder rung {} must have a non-empty summary",
                    binding.rung_class.as_str()
                ),
            );
        }
        if binding.evidence_refs.is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.recovery_ladder_evidence_missing",
                &profile.profile_id,
                format!(
                    "recovery-ladder rung {} must cite at least one evidence ref",
                    binding.rung_class.as_str()
                ),
            );
        }
    }
    for required in RecoveryLadderRungClass::REQUIRED {
        if !bound_rungs.contains(&required) {
            push_violation(
                &mut violations,
                "hardened_safe_mode.required_recovery_ladder_rung_missing",
                &profile.profile_id,
                format!(
                    "required recovery-ladder rung missing: {}",
                    required.as_str()
                ),
            );
        }
    }

    // Validate host/service/surface ids are unique and non-empty.
    let mut host_ids: BTreeSet<&str> = BTreeSet::new();
    for host in &profile.declared_hosts {
        if !host_ids.insert(host.host_id.as_str()) {
            push_violation(
                &mut violations,
                "hardened_safe_mode.duplicate_host_id",
                &host.host_id,
                "duplicate host_id is forbidden",
            );
        }
        if host.host_id.trim().is_empty() || host.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.host_field_empty",
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
                "hardened_safe_mode.duplicate_service_id",
                &service.service_id,
                "duplicate service_id is forbidden",
            );
        }
        if service.service_id.trim().is_empty() || service.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.service_field_empty",
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
                "hardened_safe_mode.duplicate_surface_id",
                &surface.surface_id,
                "duplicate surface_id is forbidden",
            );
        }
        if surface.surface_id.trim().is_empty() || surface.narrowing_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "hardened_safe_mode.surface_field_empty",
                &surface.surface_id,
                "surface_id and narrowing_summary must be non-empty",
            );
        }
    }

    violations
}

fn push_violation(
    violations: &mut Vec<HardenedSafeModeViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(HardenedSafeModeViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
