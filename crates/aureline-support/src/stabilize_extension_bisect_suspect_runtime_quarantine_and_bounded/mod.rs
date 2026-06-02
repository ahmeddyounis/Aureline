//! Stabilized extension-bisect, suspect-runtime quarantine, and bounded repair
//! orchestration for the M4 stable lane.
//!
//! This module binds the three recovery subsystems — extension bisect,
//! suspect-runtime quarantine, and bounded repair — into one typed, truthful,
//! and narrow orchestration profile. The evaluator guarantees that:
//!
//! - Every binding cites a closed vocabulary and non-empty refs.
//! - The orchestration never declares a destructive reset.
//! - User-authored files and durable state are preserved.
//! - Accessibility posture is declared for every touched surface.
//! - Recovery-ladder bindings cover every required rung.
//!
//! The [`StabilizedOrchestrationProfile`] mirrors the boundary schema at
//! [`/schemas/support/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded.schema.json`].
//!
//! The [`StabilizedOrchestrationEvaluator`] validates the profile and projects
//! a metadata-safe [`StabilizedOrchestrationSupportPacket`].

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the stabilized orchestration profile record.
pub const STABILIZED_ORCHESTRATION_PROFILE_RECORD_KIND: &str =
    "stabilized_orchestration_profile_record";

/// Stable record-kind tag for the stabilized orchestration support packet.
pub const STABILIZED_ORCHESTRATION_SUPPORT_PACKET_RECORD_KIND: &str =
    "stabilized_orchestration_support_packet_record";

/// Integer schema version for the stabilized orchestration records.
pub const STABILIZED_ORCHESTRATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const STABILIZED_ORCHESTRATION_SCHEMA_REF: &str =
    "schemas/support/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const STABILIZED_ORCHESTRATION_DOC_REF: &str =
    "docs/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const STABILIZED_ORCHESTRATION_ARTIFACT_REF: &str =
    "artifacts/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const STABILIZED_ORCHESTRATION_FIXTURE_DIR: &str =
    "fixtures/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed orchestration profile-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilizedOrchestrationProfileClass {
    /// Entered after the startup crash-loop budget was exhausted.
    PostCrashLoopOrchestration,
    /// User explicitly chose orchestration from a recovery surface.
    UserInvokedOrchestration,
    /// Managed policy or an admin override forced the orchestration.
    PolicyForcedOrchestration,
    /// Diagnostics mode chosen to bound side effects during reproduction.
    DiagnosticsOrchestration,
}

impl StabilizedOrchestrationProfileClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostCrashLoopOrchestration => "post_crash_loop_orchestration",
            Self::UserInvokedOrchestration => "user_invoked_orchestration",
            Self::PolicyForcedOrchestration => "policy_forced_orchestration",
            Self::DiagnosticsOrchestration => "diagnostics_orchestration",
        }
    }
}

/// Closed extension-bisect status vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionBisectStatusClass {
    /// Bisect session is still active.
    Active,
    /// Bisect completed and attributed the offender.
    Completed,
    /// Bisect escalated to a quarantine rung.
    EscalatedToQuarantine,
    /// Prior extension state was restored.
    RestoredPriorState,
}

impl ExtensionBisectStatusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::EscalatedToQuarantine => "escalated_to_quarantine",
            Self::RestoredPriorState => "restored_prior_state",
        }
    }
}

/// Closed quarantine-reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReasonClass {
    /// Crash loop was suspected on the lane.
    CrashLoopSuspected,
    /// Extension regression was suspected.
    ExtensionRegressionSuspected,
    /// Runtime fault domain was suspected.
    RuntimeFaultDomainSuspected,
    /// Managed policy forced the quarantine.
    PolicyForced,
}

impl QuarantineReasonClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashLoopSuspected => "crash_loop_suspected",
            Self::ExtensionRegressionSuspected => "extension_regression_suspected",
            Self::RuntimeFaultDomainSuspected => "runtime_fault_domain_suspected",
            Self::PolicyForced => "policy_forced",
        }
    }
}

/// Closed bounded-repair blast-radius vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundedRepairBlastRadiusClass {
    /// Repair is limited to cache and index objects.
    CacheIndexOnly,
    /// Repair is limited to extension quarantine state.
    ExtensionQuarantineOnly,
    /// Repair is limited to toolchain re-resolution.
    ToolchainReresolveOnly,
    /// Repair is limited to remote-agent rollback.
    RemoteAgentRollbackOnly,
    /// Repair is limited to policy refresh.
    PolicyRefreshOnly,
    /// No local mutation; escalation only.
    EscalationOnly,
}

impl BoundedRepairBlastRadiusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CacheIndexOnly => "cache_index_only",
            Self::ExtensionQuarantineOnly => "extension_quarantine_only",
            Self::ToolchainReresolveOnly => "toolchain_reresolve_only",
            Self::RemoteAgentRollbackOnly => "remote_agent_rollback_only",
            Self::PolicyRefreshOnly => "policy_refresh_only",
            Self::EscalationOnly => "escalation_only",
        }
    }
}

/// Closed bounded-repair compensation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundedRepairCompensationClass {
    /// Repair can be rolled back automatically.
    Rollbackable,
    /// Repair can be reversed by an explicit action.
    Reversible,
    /// No compensation; escalation only.
    EscalationOnly,
}

impl BoundedRepairCompensationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rollbackable => "rollbackable",
            Self::Reversible => "reversible",
            Self::EscalationOnly => "escalation_only",
        }
    }
}

/// Closed bounded-repair status vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundedRepairStatusClass {
    /// Repair is still in preview.
    Previewed,
    /// Repair was applied.
    Applied,
    /// Repair was reversed.
    Reversed,
    /// Repair was escalated.
    Escalated,
}

impl BoundedRepairStatusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Previewed => "previewed",
            Self::Applied => "applied",
            Self::Reversed => "reversed",
            Self::Escalated => "escalated",
        }
    }
}

/// Closed preserved-state vocabulary. Every orchestration profile must preserve
/// each variant exactly once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedStateClass {
    /// User-authored files and buffers.
    UserAuthoredFiles,
    /// Selection, caret, and scroll position for open buffers.
    OpenBufferSelection,
    /// Workspace trust state.
    WorkspaceTrustStore,
    /// Credential handles and stores.
    CredentialStore,
    /// Session restore records.
    SessionRestoreStore,
    /// Support export records and staging state.
    SupportExportStore,
    /// Extension activation state and install set.
    ExtensionStateStore,
    /// Runtime quarantine records.
    RuntimeQuarantineStore,
}

impl PreservedStateClass {
    /// Every required preserved state class, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::UserAuthoredFiles,
        Self::OpenBufferSelection,
        Self::WorkspaceTrustStore,
        Self::CredentialStore,
        Self::SessionRestoreStore,
        Self::SupportExportStore,
        Self::ExtensionStateStore,
        Self::RuntimeQuarantineStore,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredFiles => "user_authored_files",
            Self::OpenBufferSelection => "open_buffer_selection",
            Self::WorkspaceTrustStore => "workspace_trust_store",
            Self::CredentialStore => "credential_store",
            Self::SessionRestoreStore => "session_restore_store",
            Self::SupportExportStore => "support_export_store",
            Self::ExtensionStateStore => "extension_state_store",
            Self::RuntimeQuarantineStore => "runtime_quarantine_store",
        }
    }
}

/// Closed retained-capability vocabulary. Every orchestration profile must admit
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
    /// The dimension is fully supported in the orchestration profile.
    FullySupported,
    /// The dimension works but with degraded fidelity.
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

/// Closed recovery-ladder rung vocabulary. Every orchestration profile must bind
/// each rung that the recovery surface surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderRungClass {
    /// Safe-mode rung.
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
    /// Extension bisect rung.
    ExtensionBisect,
    /// Bounded repair rung.
    BoundedRepair,
}

impl RecoveryLadderRungClass {
    /// Every required recovery-ladder rung, in declaration order.
    pub const REQUIRED: [Self; 9] = [
        Self::SafeMode,
        Self::OpenWithoutRestore,
        Self::DisableRecentExtension,
        Self::DisableRecentLayout,
        Self::OpenLogs,
        Self::ExportCrashManifest,
        Self::ReportIssue,
        Self::ExtensionBisect,
        Self::BoundedRepair,
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
            Self::ExtensionBisect => "extension_bisect",
            Self::BoundedRepair => "bounded_repair",
        }
    }
}

/// Closed support-class vocabulary for recovery-ladder bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrchestrationSupportClass {
    /// The rung is launch-stable in the orchestration profile.
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

impl OrchestrationSupportClass {
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

// ---------------------------------------------------------------------------
// Row types
// ---------------------------------------------------------------------------

/// Binding to an extension-bisect session and its results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionBisectBinding {
    /// Extension-bisect session reference.
    pub session_ref: String,
    /// Step references within the session.
    pub step_refs: Vec<String>,
    /// Finding reference produced by the bisect.
    pub finding_ref: String,
    /// Restore reference for returning to the prior extension state.
    pub restore_ref: String,
    /// Support packet reference for the bisect.
    pub support_packet_ref: String,
    /// Current status of the bisect.
    pub status_class: ExtensionBisectStatusClass,
}

/// Binding to a suspect-runtime quarantine record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspectRuntimeQuarantineBinding {
    /// Quarantine record reference.
    pub quarantine_ref: String,
    /// Quarantined lane reference.
    pub lane_ref: String,
    /// Owner responsible for the quarantine state.
    pub owner_ref: String,
    /// Reason class for the quarantine.
    pub reason_class: QuarantineReasonClass,
    /// UTC timestamp after which the quarantine must block until reviewed.
    pub expires_at: String,
    /// Explicit action that clears the quarantine record.
    pub clear_action_ref: String,
    /// Explicit action that re-enables or retries the lane.
    pub reenable_action_ref: String,
    /// Evidence refs that justify the quarantine.
    pub evidence_refs: Vec<String>,
}

/// Binding to a bounded repair transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedRepairBinding {
    /// Repair transaction reference.
    pub transaction_ref: String,
    /// Repair preview reference.
    pub preview_ref: String,
    /// Repair outcome reference.
    pub outcome_ref: String,
    /// Blast-radius class for the repair.
    pub blast_radius_class: BoundedRepairBlastRadiusClass,
    /// Compensation class for the repair.
    pub compensation_class: BoundedRepairCompensationClass,
    /// Current status of the repair.
    pub status_class: BoundedRepairStatusClass,
}

/// One retained capability with support guidance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetainedCapabilityRecord {
    /// Stable capability class.
    pub capability_class: RetainedCapabilityClass,
    /// Reviewer-safe rationale for why the capability is retained.
    pub rationale: String,
    /// User-facing support-guidance string explaining what the user can do.
    pub support_guidance: String,
    /// Whether the capability is explicitly tested.
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
    pub support_class: OrchestrationSupportClass,
    /// Whether the rung requires review before execution.
    pub requires_review: bool,
    /// Reviewer-safe summary of the rung behavior.
    pub rung_summary: String,
    /// Evidence refs justifying the support-class label.
    pub evidence_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Profile and support packet
// ---------------------------------------------------------------------------

/// Stabilized orchestration profile record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedOrchestrationProfile {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable profile identifier.
    pub profile_id: String,
    /// Profile class.
    pub profile_class: StabilizedOrchestrationProfileClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Project Doctor finding that justified the profile.
    pub doctor_finding_ref: String,
    /// Support packet ref that consumes the profile.
    pub support_packet_ref: String,
    /// Extension bisect binding.
    pub extension_bisect_binding: ExtensionBisectBinding,
    /// Suspect runtime quarantine binding.
    pub suspect_runtime_quarantine_binding: SuspectRuntimeQuarantineBinding,
    /// Bounded repair binding.
    pub bounded_repair_binding: BoundedRepairBinding,
    /// Retained capability records (must admit every required class).
    pub retained_capabilities: Vec<RetainedCapabilityRecord>,
    /// Preserved state classes (must include all required classes).
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Accessibility posture rows (must cover every touched target).
    pub accessibility_postures: Vec<AccessibilityPostureRow>,
    /// Recovery-ladder binding rows.
    pub recovery_ladder_bindings: Vec<RecoveryLadderBindingRow>,
    /// Whether the profile carries any destructive reset.
    pub destructive_resets_present: bool,
}

/// Metadata-safe support projection for the stabilized orchestration profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilizedOrchestrationSupportPacket {
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
    pub profile_class: StabilizedOrchestrationProfileClass,
    /// Project Doctor finding ref the packet cites.
    pub doctor_finding_ref: String,
    /// Extension bisect binding.
    pub extension_bisect_binding: ExtensionBisectBinding,
    /// Suspect runtime quarantine binding.
    pub suspect_runtime_quarantine_binding: SuspectRuntimeQuarantineBinding,
    /// Bounded repair binding.
    pub bounded_repair_binding: BoundedRepairBinding,
    /// Retained capability rows.
    pub retained_capability_rows: Vec<RetainedCapabilityRecord>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Accessibility posture rows.
    pub accessibility_posture_rows: Vec<AccessibilityPostureRow>,
    /// Recovery-ladder binding rows.
    pub recovery_ladder_binding_rows: Vec<RecoveryLadderBindingRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether the projection records a destructive reset.
    pub destructive_resets_present: bool,
}

impl StabilizedOrchestrationSupportPacket {
    /// Returns true when the packet preserves the bounded orchestration contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_resets_present
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && !self.retained_capability_rows.is_empty()
            && !self.accessibility_posture_rows.is_empty()
            && !self.recovery_ladder_binding_rows.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizedOrchestrationViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizedOrchestrationValidationReport {
    /// Validation failures.
    pub violations: Vec<StabilizedOrchestrationViolation>,
}

impl fmt::Display for StabilizedOrchestrationValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} stabilized orchestration violation(s)",
            self.violations.len()
        )
    }
}

impl Error for StabilizedOrchestrationValidationReport {}

/// Loads a stabilized orchestration profile from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`StabilizedOrchestrationProfile`].
pub fn load_stabilized_orchestration_profile(
    yaml: &str,
) -> Result<StabilizedOrchestrationProfile, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Stabilized orchestration stable evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct StabilizedOrchestrationEvaluator;

impl StabilizedOrchestrationEvaluator {
    /// Creates a new stabilized orchestration evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a [`StabilizedOrchestrationProfile`].
    ///
    /// # Errors
    ///
    /// Returns [`StabilizedOrchestrationValidationReport`] when the profile
    /// omits required retained capabilities, declares a destructive reset,
    /// fails to explain a binding, misses accessibility postures, or omits
    /// recovery-ladder bindings.
    pub fn validate_profile(
        &self,
        profile: &StabilizedOrchestrationProfile,
    ) -> Result<(), StabilizedOrchestrationValidationReport> {
        let violations = validate_profile(profile);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(StabilizedOrchestrationValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`StabilizedOrchestrationValidationReport`] when the profile
    /// fails validation.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        profile: &StabilizedOrchestrationProfile,
    ) -> Result<StabilizedOrchestrationSupportPacket, StabilizedOrchestrationValidationReport> {
        let violations = validate_profile(profile);
        if !violations.is_empty() {
            return Err(StabilizedOrchestrationValidationReport { violations });
        }

        Ok(StabilizedOrchestrationSupportPacket {
            record_kind: STABILIZED_ORCHESTRATION_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: STABILIZED_ORCHESTRATION_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: STABILIZED_ORCHESTRATION_DOC_REF.to_owned(),
            schema_ref: STABILIZED_ORCHESTRATION_SCHEMA_REF.to_owned(),
            profile_id: profile.profile_id.clone(),
            profile_class: profile.profile_class,
            doctor_finding_ref: profile.doctor_finding_ref.clone(),
            extension_bisect_binding: profile.extension_bisect_binding.clone(),
            suspect_runtime_quarantine_binding: profile.suspect_runtime_quarantine_binding.clone(),
            bounded_repair_binding: profile.bounded_repair_binding.clone(),
            retained_capability_rows: profile.retained_capabilities.clone(),
            preserved_state_classes: profile.preserved_state_classes.clone(),
            accessibility_posture_rows: profile.accessibility_postures.clone(),
            recovery_ladder_binding_rows: profile.recovery_ladder_bindings.clone(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
        })
    }
}

fn validate_profile(
    profile: &StabilizedOrchestrationProfile,
) -> Vec<StabilizedOrchestrationViolation> {
    let mut violations = Vec::new();

    if profile.schema_version != STABILIZED_ORCHESTRATION_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "stabilized_orchestration.schema_version",
            &profile.profile_id,
            "profile schema_version must be 1",
        );
    }
    if profile.record_kind != STABILIZED_ORCHESTRATION_PROFILE_RECORD_KIND {
        push_violation(
            &mut violations,
            "stabilized_orchestration.record_kind",
            &profile.profile_id,
            "profile record_kind must be stabilized_orchestration_profile_record",
        );
    }
    if profile.profile_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "stabilized_orchestration.profile_id_empty",
            &profile.profile_id,
            "profile_id must be non-empty",
        );
    }
    if !profile.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "stabilized_orchestration.doctor_finding_ref_missing",
            &profile.profile_id,
            "profile must cite a Project Doctor finding ref",
        );
    }
    if profile.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "stabilized_orchestration.support_packet_ref_empty",
            &profile.profile_id,
            "support_packet_ref must be non-empty",
        );
    }
    if profile.destructive_resets_present {
        push_violation(
            &mut violations,
            "stabilized_orchestration.destructive_resets_present",
            &profile.profile_id,
            "orchestration profile must not declare destructive resets",
        );
    }

    let required_preserved: BTreeSet<_> = PreservedStateClass::REQUIRED.iter().copied().collect();
    let actual_preserved: BTreeSet<_> = profile.preserved_state_classes.iter().copied().collect();
    for required in &required_preserved {
        if !actual_preserved.contains(required) {
            push_violation(
                &mut violations,
                "stabilized_orchestration.preserved_state_missing",
                &profile.profile_id,
                &format!("preserved state class {} is required", required.as_str()),
            );
        }
    }

    let required_capabilities: BTreeSet<_> =
        RetainedCapabilityClass::REQUIRED.iter().copied().collect();
    let actual_capabilities: BTreeSet<_> = profile
        .retained_capabilities
        .iter()
        .map(|r| r.capability_class)
        .collect();
    for required in &required_capabilities {
        if !actual_capabilities.contains(required) {
            push_violation(
                &mut violations,
                "stabilized_orchestration.retained_capability_missing",
                &profile.profile_id,
                &format!("retained capability {} is required", required.as_str()),
            );
        }
    }
    for record in &profile.retained_capabilities {
        if record.rationale.trim().is_empty() {
            push_violation(
                &mut violations,
                "stabilized_orchestration.retained_capability_rationale_empty",
                &profile.profile_id,
                "retained capability rationale must be non-empty",
            );
        }
        if record.support_guidance.trim().is_empty() {
            push_violation(
                &mut violations,
                "stabilized_orchestration.retained_capability_guidance_empty",
                &profile.profile_id,
                "retained capability support_guidance must be non-empty",
            );
        }
    }

    let required_rungs: BTreeSet<_> = RecoveryLadderRungClass::REQUIRED.iter().copied().collect();
    let actual_rungs: BTreeSet<_> = profile
        .recovery_ladder_bindings
        .iter()
        .map(|r| r.rung_class)
        .collect();
    for required in &required_rungs {
        if !actual_rungs.contains(required) {
            push_violation(
                &mut violations,
                "stabilized_orchestration.recovery_ladder_rung_missing",
                &profile.profile_id,
                &format!("recovery ladder rung {} is required", required.as_str()),
            );
        }
    }
    for binding in &profile.recovery_ladder_bindings {
        if binding.rung_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "stabilized_orchestration.recovery_ladder_rung_summary_empty",
                &profile.profile_id,
                "recovery ladder rung_summary must be non-empty",
            );
        }
        if binding.evidence_refs.is_empty() {
            push_violation(
                &mut violations,
                "stabilized_orchestration.recovery_ladder_evidence_empty",
                &profile.profile_id,
                "recovery ladder evidence_refs must be non-empty",
            );
        }
    }

    if profile.accessibility_postures.is_empty() {
        push_violation(
            &mut violations,
            "stabilized_orchestration.accessibility_postures_empty",
            &profile.profile_id,
            "accessibility_postures must not be empty",
        );
    }

    validate_extension_bisect_binding(profile, &mut violations);
    validate_quarantine_binding(profile, &mut violations);
    validate_repair_binding(profile, &mut violations);

    violations
}

fn validate_extension_bisect_binding(
    profile: &StabilizedOrchestrationProfile,
    violations: &mut Vec<StabilizedOrchestrationViolation>,
) {
    let binding = &profile.extension_bisect_binding;
    if binding.session_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.bisect_session_ref_empty",
            &profile.profile_id,
            "extension_bisect_binding.session_ref must be non-empty",
        );
    }
    if binding.finding_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.bisect_finding_ref_empty",
            &profile.profile_id,
            "extension_bisect_binding.finding_ref must be non-empty",
        );
    }
    if binding.restore_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.bisect_restore_ref_empty",
            &profile.profile_id,
            "extension_bisect_binding.restore_ref must be non-empty",
        );
    }
    if binding.support_packet_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.bisect_support_packet_ref_empty",
            &profile.profile_id,
            "extension_bisect_binding.support_packet_ref must be non-empty",
        );
    }
}

fn validate_quarantine_binding(
    profile: &StabilizedOrchestrationProfile,
    violations: &mut Vec<StabilizedOrchestrationViolation>,
) {
    let binding = &profile.suspect_runtime_quarantine_binding;
    if binding.quarantine_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.quarantine_ref_empty",
            &profile.profile_id,
            "suspect_runtime_quarantine_binding.quarantine_ref must be non-empty",
        );
    }
    if binding.lane_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.quarantine_lane_ref_empty",
            &profile.profile_id,
            "suspect_runtime_quarantine_binding.lane_ref must be non-empty",
        );
    }
    if binding.owner_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.quarantine_owner_ref_empty",
            &profile.profile_id,
            "suspect_runtime_quarantine_binding.owner_ref must be non-empty",
        );
    }
    if binding.clear_action_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.quarantine_clear_action_empty",
            &profile.profile_id,
            "suspect_runtime_quarantine_binding.clear_action_ref must be non-empty",
        );
    }
    if binding.reenable_action_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.quarantine_reenable_action_empty",
            &profile.profile_id,
            "suspect_runtime_quarantine_binding.reenable_action_ref must be non-empty",
        );
    }
    if binding.evidence_refs.is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.quarantine_evidence_empty",
            &profile.profile_id,
            "suspect_runtime_quarantine_binding.evidence_refs must be non-empty",
        );
    }
}

fn validate_repair_binding(
    profile: &StabilizedOrchestrationProfile,
    violations: &mut Vec<StabilizedOrchestrationViolation>,
) {
    let binding = &profile.bounded_repair_binding;
    if binding.transaction_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.repair_transaction_ref_empty",
            &profile.profile_id,
            "bounded_repair_binding.transaction_ref must be non-empty",
        );
    }
    if binding.preview_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.repair_preview_ref_empty",
            &profile.profile_id,
            "bounded_repair_binding.preview_ref must be non-empty",
        );
    }
    if binding.outcome_ref.trim().is_empty() {
        push_violation(
            violations,
            "stabilized_orchestration.repair_outcome_ref_empty",
            &profile.profile_id,
            "bounded_repair_binding.outcome_ref must be non-empty",
        );
    }
}

fn push_violation(
    violations: &mut Vec<StabilizedOrchestrationViolation>,
    check_id: &str,
    subject_ref: &str,
    message: &str,
) {
    violations.push(StabilizedOrchestrationViolation {
        check_id: check_id.to_owned(),
        subject_ref: subject_ref.to_owned(),
        message: message.to_owned(),
    });
}
