//! Live appearance-session runtime: one inspectable session object plus a
//! checkpoint-aware state machine for the M5 depth surfaces.
//!
//! The M5 depth lanes change appearance constantly — a user previews a theme
//! package, the OS flips to dark or raises contrast, a managed policy caps
//! density, an imported theme is rolled back. Before this lane each surface
//! inferred "what is active right now" from its own pixels, and preview /
//! apply / revert were ad-hoc per-surface toggles. That made half-updated
//! state and silent restart requirements possible, and left support and
//! golden-evidence flows with nothing concrete to name.
//!
//! This module makes the live appearance state a **durable, inspectable
//! object** and routes every change through **one explicit checkpoint**:
//!
//! - [`AppearanceSession`] — the canonical "what is active right now" record:
//!   the active theme-package and revision refs, the follow-system posture, the
//!   resolved theme class, the contrast mode, the accent source, the density,
//!   the text scale, the reduced-motion posture and its source, the current
//!   preview state, and the single current checkpoint / rollback refs. It is a
//!   runtime projection of the canonical `appearance_session_record` frozen in
//!   `schemas/ux/appearance_checkpoint.schema.json`; this lane mints no parallel
//!   appearance vocabulary.
//! - [`AppearanceCheckpoint`] — one explicit preview / apply checkpoint: the
//!   pre-change and post-preview snapshot refs, the changed axes, the atomicity
//!   class, the preflight checks, the rollback path, and the apply state. Every
//!   appearance change is reversible from exactly one of these.
//! - [`AppearanceTransition`] — one edge of the checkpoint-aware state machine:
//!   open-preview, preflight-passed, commit, cancel, validation-failed, revert,
//!   and OS-signal-applied. Each transition cites the single checkpoint it
//!   flows through, the legal `from` / `to` preview states, and whether it
//!   requires a surface reload or an app restart.
//! - [`AppearanceSurfaceBinding`] — one claimed M5 surface (notebook, data /
//!   result surface, preview / browser pane, docs/help pane, companion surface,
//!   extension-hosted surface): the session it consumes, its live-apply
//!   capability, and whether a restart-or-reload requirement is disclosed.
//! - [`AppearanceSessionRuntimeReport`] — the canonical truth object binding the
//!   live session, the checkpoint ledger, the transition ledger, and the
//!   per-surface bindings, with a blocking-finding summary release tooling, the
//!   support-export wrapper, diagnostics, and golden-evidence flows all reuse.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every transition flows through exactly one checkpoint that resolves in
//!    the ledger; a transition with no checkpoint, or one that names a checkpoint
//!    not in the ledger, is a blocker (appearance changes are atomic and
//!    reversible from one explicit checkpoint).
//! 2. Every transition is a legal edge of the state machine: its `from` state is
//!    a legal predecessor for the operation, and its `to` and apply states match
//!    the operation. An illegal edge — including a validation failure that does
//!    not auto-revert — is a blocker (no half-updated state).
//! 3. A change that a surface cannot apply live must disclose its
//!    restart-or-reload posture: the transition and checkpoint atomicity class
//!    must be a reload / restart class, and a surface whose capability is not
//!    `applies_live` must set `restart_or_reload_disclosed`. A silent restart
//!    requirement is a blocker.
//! 4. Every checkpoint is reversible from a single checkpoint and carries a
//!    rollback path that restores its changed axes; a non-reversible checkpoint
//!    or a missing rollback path is a blocker.
//! 5. The live session is self-consistent: a live or committed preview cites a
//!    current checkpoint, a rolled-back session cites a rollback ref, and any
//!    current checkpoint ref resolves in the ledger.
//! 6. Every registered surface rides the shared session (it consumes the live
//!    session ref and is registered on it), and carries a canonical appearance
//!    anchor and a non-empty accessibility note; a surface that paints its own
//!    appearance outside the session model is a blocker.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/live-appearance-change/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_appearance_session_runtime`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every appearance-session runtime record.
pub const APPEARANCE_SESSION_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every appearance-session record.
pub const APPEARANCE_SESSION_SHARED_CONTRACT_REF: &str = "shell:m5_appearance_session:v1";

/// Stable record kind for [`AppearanceSessionRuntimeReport`] payloads.
pub const APPEARANCE_SESSION_REPORT_RECORD_KIND: &str =
    "shell_m5_appearance_session_runtime_report_record";

/// Stable record kind for [`AppearanceSession`] payloads.
pub const APPEARANCE_SESSION_RECORD_KIND: &str = "shell_m5_appearance_session_record";

/// Stable record kind for [`AppearanceCheckpoint`] payloads.
pub const APPEARANCE_CHECKPOINT_RECORD_KIND: &str = "shell_m5_appearance_session_checkpoint_record";

/// Stable record kind for [`AppearanceTransition`] payloads.
pub const APPEARANCE_TRANSITION_RECORD_KIND: &str = "shell_m5_appearance_session_transition_record";

/// Stable record kind for [`AppearanceSurfaceBinding`] payloads.
pub const APPEARANCE_SURFACE_RECORD_KIND: &str =
    "shell_m5_appearance_session_surface_binding_record";

/// Stable record kind for [`AppearanceSessionSupportExport`] payloads.
pub const APPEARANCE_SESSION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_appearance_session_runtime_support_export_record";

/// Stable report id quoted across surfaces.
pub const APPEARANCE_SESSION_REPORT_ID: &str = "shell:m5_appearance_session:runtime:v1";

/// Stable support-export id quoted in the published wrapper.
pub const APPEARANCE_SESSION_SUPPORT_EXPORT_ID: &str = "support-export:m5-appearance-session:001";

/// Source schema ref for the canonical appearance-session runtime contract.
pub const APPEARANCE_SESSION_SOURCE_SCHEMA_REF: &str = "schemas/ux/appearance-session.schema.json";

/// Schema ref for the canonical appearance-session / checkpoint record objects
/// this lane re-exports its vocabulary from instead of re-declaring.
pub const APPEARANCE_SESSION_CANONICAL_RECORD_SCHEMA_REF: &str =
    "schemas/ux/appearance_checkpoint.schema.json";

/// Path of the published markdown audit artifact.
pub const APPEARANCE_SESSION_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md";

/// Path of the published companion doc.
pub const APPEARANCE_SESSION_PUBLISHED_DOC_REF: &str = "docs/m5/appearance-session-runtime.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-17T00:00:00Z";

/// Resolved color theme class. Re-exported from the canonical appearance
/// `theme_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeClass {
    /// Dark reference theme.
    DarkReference,
    /// Light parity theme.
    LightParity,
    /// High-contrast dark theme.
    HighContrastDark,
    /// High-contrast light theme.
    HighContrastLight,
}

impl ThemeClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DarkReference => "dark_reference",
            Self::LightParity => "light_parity",
            Self::HighContrastDark => "high_contrast_dark",
            Self::HighContrastLight => "high_contrast_light",
        }
    }
}

/// Density class. Re-exported from the canonical appearance `density_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DensityClass {
    /// Compact density.
    Compact,
    /// Standard density.
    Standard,
    /// Comfortable density.
    Comfortable,
}

impl DensityClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Standard => "standard",
            Self::Comfortable => "comfortable",
        }
    }
}

/// Reduced-motion / accessibility posture. Re-exported from the canonical
/// `accessibility_posture_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionPosture {
    /// Standard motion.
    MotionStandard,
    /// Reduced motion.
    MotionReduced,
    /// Low-motion treatment.
    MotionLowMotion,
    /// Power-saver motion treatment.
    MotionPowerSaver,
    /// Critical hot-path motion treatment.
    MotionCriticalHotPath,
}

impl MotionPosture {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MotionStandard => "motion_standard",
            Self::MotionReduced => "motion_reduced",
            Self::MotionLowMotion => "motion_low_motion",
            Self::MotionPowerSaver => "motion_power_saver",
            Self::MotionCriticalHotPath => "motion_critical_hot_path",
        }
    }
}

/// How the session resolves OS appearance signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowSystemPosture {
    /// Resolves from the OS appearance signal.
    FollowSystem,
    /// User has overridden the OS signal manually.
    ManualOverride,
    /// A managed policy has overridden the OS signal.
    ManagedPolicyOverride,
    /// The platform exposes no appearance signal.
    UnavailablePlatformSignal,
}

impl FollowSystemPosture {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FollowSystem => "follow_system",
            Self::ManualOverride => "manual_override",
            Self::ManagedPolicyOverride => "managed_policy_override",
            Self::UnavailablePlatformSignal => "unavailable_platform_signal",
        }
    }
}

/// Effective contrast posture for the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContrastMode {
    /// Standard contrast.
    ContrastStandard,
    /// High contrast.
    ContrastHigh,
    /// OS forced-colors mode.
    ContrastForcedColors,
}

impl ContrastMode {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContrastStandard => "contrast_standard",
            Self::ContrastHigh => "contrast_high",
            Self::ContrastForcedColors => "contrast_forced_colors",
        }
    }
}

/// Source of the resolved accent color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccentSource {
    /// Accent resolved from the OS accent signal.
    SystemAccent,
    /// Accent resolved from the active theme package.
    ThemePackageAccent,
    /// Accent the user selected.
    UserSelectedAccent,
    /// Accent locked by a managed policy.
    PolicyLockedAccent,
    /// Accent is not applicable.
    NotApplicable,
}

impl AccentSource {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemAccent => "system_accent",
            Self::ThemePackageAccent => "theme_package_accent",
            Self::UserSelectedAccent => "user_selected_accent",
            Self::PolicyLockedAccent => "policy_locked_accent",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Source that determined the reduced-motion posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReducedMotionSource {
    /// OS reduce-motion signal.
    OsSignal,
    /// Explicit user setting.
    UserSetting,
    /// Managed-policy cap.
    PolicyCap,
    /// Power-saver signal.
    PowerSaverSignal,
    /// Critical hot-path override.
    CriticalHotPath,
    /// Not applicable.
    NotApplicable,
}

impl ReducedMotionSource {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsSignal => "os_signal",
            Self::UserSetting => "user_setting",
            Self::PolicyCap => "policy_cap",
            Self::PowerSaverSignal => "power_saver_signal",
            Self::CriticalHotPath => "critical_hot_path",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Source that determined the text scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextScaleSource {
    /// OS text-scale signal.
    System,
    /// Explicit user setting.
    User,
    /// Profile-level setting.
    Profile,
    /// Workspace-level setting.
    Workspace,
    /// Managed-policy setting.
    Policy,
}

impl TextScaleSource {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Profile => "profile",
            Self::Workspace => "workspace",
            Self::Policy => "policy",
        }
    }
}

/// Live preview state of the session. Re-exported from the canonical
/// `preview_state` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewState {
    /// No preview is active.
    NotPreviewing,
    /// A preview checkpoint is created and awaiting preflight validation.
    PreviewPendingValidation,
    /// A preview is live and inspectable.
    PreviewLive,
    /// A preview failed validation and auto-reverted from its checkpoint.
    PreviewFailedReverted,
    /// A preview has been committed.
    PreviewCommitted,
    /// A committed change has been rolled back from its checkpoint.
    RollbackApplied,
}

impl PreviewState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotPreviewing => "not_previewing",
            Self::PreviewPendingValidation => "preview_pending_validation",
            Self::PreviewLive => "preview_live",
            Self::PreviewFailedReverted => "preview_failed_reverted",
            Self::PreviewCommitted => "preview_committed",
            Self::RollbackApplied => "rollback_applied",
        }
    }

    /// `true` when the state requires a current checkpoint ref.
    pub const fn requires_current_checkpoint(self) -> bool {
        matches!(
            self,
            Self::PreviewPendingValidation | Self::PreviewLive | Self::PreviewCommitted
        )
    }
}

/// Checkpoint class. Re-exported from the canonical `checkpoint_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointClass {
    /// User-driven appearance preview.
    AppearancePreviewCheckpoint,
    /// Imported-theme application.
    AppearanceImportCheckpoint,
    /// Token-overlay application.
    AppearanceOverlayCheckpoint,
    /// OS-signal-driven change.
    AppearanceOsSignalCheckpoint,
    /// Managed-policy-driven change.
    AppearancePolicyCheckpoint,
}

impl CheckpointClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppearancePreviewCheckpoint => "appearance_preview_checkpoint",
            Self::AppearanceImportCheckpoint => "appearance_import_checkpoint",
            Self::AppearanceOverlayCheckpoint => "appearance_overlay_checkpoint",
            Self::AppearanceOsSignalCheckpoint => "appearance_os_signal_checkpoint",
            Self::AppearancePolicyCheckpoint => "appearance_policy_checkpoint",
        }
    }
}

/// Checkpoint scope. Re-exported from the canonical `checkpoint_scope_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointScope {
    /// Global appearance.
    GlobalAppearance,
    /// Profile-scoped appearance.
    ProfileAppearance,
    /// Workspace-scoped appearance.
    WorkspaceAppearance,
    /// Extension-surface-scoped appearance.
    ExtensionSurfaceAppearance,
    /// Preview-only scope.
    PreviewOnly,
}

impl CheckpointScope {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GlobalAppearance => "global_appearance",
            Self::ProfileAppearance => "profile_appearance",
            Self::WorkspaceAppearance => "workspace_appearance",
            Self::ExtensionSurfaceAppearance => "extension_surface_appearance",
            Self::PreviewOnly => "preview_only",
        }
    }
}

/// Appearance axis a change touches. Re-exported from the canonical
/// `appearance_axis` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceAxis {
    /// Theme package.
    ThemePackage,
    /// Follow-system posture.
    FollowSystem,
    /// Contrast mode.
    Contrast,
    /// Accent source.
    Accent,
    /// Density.
    Density,
    /// Text scale.
    TextScale,
    /// Reduced-motion posture.
    ReducedMotion,
    /// Token overlay.
    TokenOverlay,
    /// Import mapping.
    ImportMapping,
    /// Extension-surface claim.
    ExtensionSurfaceClaim,
}

impl AppearanceAxis {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemePackage => "theme_package",
            Self::FollowSystem => "follow_system",
            Self::Contrast => "contrast",
            Self::Accent => "accent",
            Self::Density => "density",
            Self::TextScale => "text_scale",
            Self::ReducedMotion => "reduced_motion",
            Self::TokenOverlay => "token_overlay",
            Self::ImportMapping => "import_mapping",
            Self::ExtensionSurfaceClaim => "extension_surface_claim",
        }
    }
}

/// Apply outcome recorded on a checkpoint or transition. Re-exported from the
/// canonical `appearance_apply_state` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyState {
    /// A checkpoint was created.
    CheckpointCreated,
    /// Preflight validation failed.
    PreflightFailed,
    /// The preview is live.
    PreviewLive,
    /// The change is committed.
    Committed,
    /// The change is reverted.
    Reverted,
    /// A rollback is required.
    RollbackRequired,
    /// A rollback failed and is blocked.
    RollbackFailedBlocked,
}

impl ApplyState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckpointCreated => "checkpoint_created",
            Self::PreflightFailed => "preflight_failed",
            Self::PreviewLive => "preview_live",
            Self::Committed => "committed",
            Self::Reverted => "reverted",
            Self::RollbackRequired => "rollback_required",
            Self::RollbackFailedBlocked => "rollback_failed_blocked",
        }
    }
}

/// Atomicity class of a change. Re-exported from the canonical
/// `atomicity_class` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AtomicityClass {
    /// Applied atomically from a single checkpoint.
    SingleCheckpointAtomic,
    /// Applied after a surface reload, from a single checkpoint.
    SurfaceReloadFromSingleCheckpoint,
    /// Applied after a full app restart, from a single checkpoint.
    FullRestartFromSingleCheckpoint,
}

impl AtomicityClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCheckpointAtomic => "single_checkpoint_atomic",
            Self::SurfaceReloadFromSingleCheckpoint => "surface_reload_from_single_checkpoint",
            Self::FullRestartFromSingleCheckpoint => "full_restart_from_single_checkpoint",
        }
    }

    /// `true` when the change applies live without a reload or restart.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::SingleCheckpointAtomic)
    }
}

/// Rollback-path class. Re-exported from the canonical `rollback_path_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPathClass {
    /// Revert from a single checkpoint.
    SingleCheckpointRevert,
    /// Reload the surface, then revert from a single checkpoint.
    SurfaceReloadThenRevert,
    /// Restart the app, then revert from a single checkpoint.
    FullRestartThenRevert,
}

impl RollbackPathClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCheckpointRevert => "single_checkpoint_revert",
            Self::SurfaceReloadThenRevert => "surface_reload_then_revert",
            Self::FullRestartThenRevert => "full_restart_then_revert",
        }
    }

    /// `true` when the rollback path applies live without a reload or restart.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::SingleCheckpointRevert)
    }
}

/// Preflight check class. Re-exported from the canonical `preflight_check_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightCheckClass {
    /// Contrast targets met.
    ContrastTargets,
    /// Protected cues preserved.
    ProtectedCuePreservation,
    /// Import-mapping report present.
    ImportMappingReportPresent,
    /// Syntax coverage present.
    SyntaxCoveragePresent,
    /// Extension inheritance disclosed.
    ExtensionInheritanceDisclosed,
    /// Rollback path present.
    RollbackPathPresent,
}

impl PreflightCheckClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContrastTargets => "contrast_targets",
            Self::ProtectedCuePreservation => "protected_cue_preservation",
            Self::ImportMappingReportPresent => "import_mapping_report_present",
            Self::SyntaxCoveragePresent => "syntax_coverage_present",
            Self::ExtensionInheritanceDisclosed => "extension_inheritance_disclosed",
            Self::RollbackPathPresent => "rollback_path_present",
        }
    }
}

/// Preflight check result state. Re-exported from the canonical
/// `check_result_state` vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckResultState {
    /// Check passed.
    Passed,
    /// Check warned.
    Warning,
    /// Check failed and is blocking.
    FailedBlocked,
}

impl CheckResultState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::FailedBlocked => "failed_blocked",
        }
    }
}

/// Redaction class. Re-exported from the canonical `redaction_class`
/// vocabulary without modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default.
    MetadataSafeDefault,
    /// Operator-only restricted.
    OperatorOnlyRestricted,
    /// Internal-support restricted.
    InternalSupportRestricted,
    /// Signing-evidence only.
    SigningEvidenceOnly,
}

impl RedactionClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// One edge of the checkpoint-aware appearance state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionOp {
    /// A change is initiated and a single checkpoint is created.
    OpenPreview,
    /// Preflight validation passed; the preview goes live.
    PreflightPassed,
    /// A live preview is committed.
    CommitPreview,
    /// A preview is cancelled and reverted from its checkpoint.
    CancelPreview,
    /// Preflight validation failed; the preview auto-reverts.
    ValidationFailed,
    /// A committed change is rolled back from its checkpoint.
    RevertCommitted,
    /// An OS appearance / contrast / accent signal is applied atomically.
    OsSignalApplied,
}

impl TransitionOp {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenPreview => "open_preview",
            Self::PreflightPassed => "preflight_passed",
            Self::CommitPreview => "commit_preview",
            Self::CancelPreview => "cancel_preview",
            Self::ValidationFailed => "validation_failed",
            Self::RevertCommitted => "revert_committed",
            Self::OsSignalApplied => "os_signal_applied",
        }
    }

    /// The legal predecessor preview states for this operation.
    pub fn legal_from_states(self) -> &'static [PreviewState] {
        match self {
            Self::OpenPreview => &[PreviewState::NotPreviewing],
            Self::PreflightPassed => &[PreviewState::PreviewPendingValidation],
            Self::CommitPreview => &[PreviewState::PreviewLive],
            Self::CancelPreview => &[
                PreviewState::PreviewPendingValidation,
                PreviewState::PreviewLive,
            ],
            Self::ValidationFailed => &[
                PreviewState::PreviewPendingValidation,
                PreviewState::PreviewLive,
            ],
            Self::RevertCommitted => &[PreviewState::PreviewCommitted],
            Self::OsSignalApplied => &[PreviewState::NotPreviewing, PreviewState::PreviewCommitted],
        }
    }

    /// The preview state this operation lands in.
    pub const fn expected_to_state(self) -> PreviewState {
        match self {
            Self::OpenPreview => PreviewState::PreviewPendingValidation,
            Self::PreflightPassed => PreviewState::PreviewLive,
            Self::CommitPreview => PreviewState::PreviewCommitted,
            Self::CancelPreview => PreviewState::NotPreviewing,
            Self::ValidationFailed => PreviewState::PreviewFailedReverted,
            Self::RevertCommitted => PreviewState::RollbackApplied,
            Self::OsSignalApplied => PreviewState::PreviewCommitted,
        }
    }

    /// The apply state this operation records.
    pub const fn expected_apply_state(self) -> ApplyState {
        match self {
            Self::OpenPreview => ApplyState::CheckpointCreated,
            Self::PreflightPassed => ApplyState::PreviewLive,
            Self::CommitPreview => ApplyState::Committed,
            Self::CancelPreview => ApplyState::Reverted,
            Self::ValidationFailed => ApplyState::PreflightFailed,
            Self::RevertCommitted => ApplyState::Reverted,
            Self::OsSignalApplied => ApplyState::Committed,
        }
    }

    /// `true` when this operation makes a live or committed appearance change
    /// (rather than a revert / cancel / failure).
    pub const fn is_live_change(self) -> bool {
        matches!(
            self,
            Self::PreflightPassed | Self::CommitPreview | Self::OsSignalApplied
        )
    }
}

/// Trigger that drove a transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionTrigger {
    /// A direct user action.
    UserAction,
    /// An OS appearance / contrast / accent signal.
    OsSignal,
    /// A managed-policy change.
    ManagedPolicy,
    /// A sync / import operation.
    SyncImport,
}

impl TransitionTrigger {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAction => "user_action",
            Self::OsSignal => "os_signal",
            Self::ManagedPolicy => "managed_policy",
            Self::SyncImport => "sync_import",
        }
    }
}

/// Family of an M5 surface that consumes the appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    /// Notebook cell chrome.
    Notebook,
    /// Data / result-grid surface.
    DataResultSurface,
    /// Preview / browser pane.
    PreviewBrowserPane,
    /// Docs / help pane.
    DocsHelpPane,
    /// Companion surface.
    CompanionSurface,
    /// Extension-hosted surface.
    ExtensionHostedSurface,
}

impl SurfaceFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataResultSurface => "data_result_surface",
            Self::PreviewBrowserPane => "preview_browser_pane",
            Self::DocsHelpPane => "docs_help_pane",
            Self::CompanionSurface => "companion_surface",
            Self::ExtensionHostedSurface => "extension_hosted_surface",
        }
    }
}

/// How a surface applies an appearance change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveApplyCapability {
    /// Applies appearance changes live with no reload or restart.
    AppliesLive,
    /// Requires a surface reload to apply some changes.
    RequiresSurfaceReload,
    /// Requires an app restart to apply some changes.
    RequiresAppRestart,
    /// The platform signal needed to apply the change is unavailable.
    PlatformSignalUnavailable,
}

impl LiveApplyCapability {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppliesLive => "applies_live",
            Self::RequiresSurfaceReload => "requires_surface_reload",
            Self::RequiresAppRestart => "requires_app_restart",
            Self::PlatformSignalUnavailable => "platform_signal_unavailable",
        }
    }

    /// `true` when the surface can apply changes live.
    pub const fn applies_live(self) -> bool {
        matches!(self, Self::AppliesLive)
    }
}

/// Effective text scale for the session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextScale {
    /// Scale percentage (75–200).
    pub scale_percent: u32,
    /// Source that determined the scale.
    pub source: TextScaleSource,
    /// `true` when a managed policy locks the scale.
    pub locked_by_policy: bool,
}

/// Policy context attached to a session or checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    /// Policy epoch label.
    pub policy_epoch: String,
    /// Trust state: `trusted` or `restricted`.
    pub trust_state: String,
    /// Optional execution-context id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
}

/// The single rollback path that restores a checkpoint's changed axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackPath {
    /// Rollback-path class.
    pub rollback_path_class: RollbackPathClass,
    /// Opaque rollback ref.
    pub rollback_ref: String,
    /// User-visible action id that triggers the rollback.
    pub user_visible_action_id: String,
    /// Axes this rollback restores.
    pub restores_axes: Vec<AppearanceAxis>,
    /// `true` when the rollback requires an app restart.
    pub requires_restart: bool,
}

/// One preflight check recorded on a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreflightCheck {
    /// Check class.
    pub check_class: PreflightCheckClass,
    /// Result state.
    pub result_state: CheckResultState,
    /// Optional finding ref when the check warned or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_ref: Option<String>,
}

/// The live appearance session: a runtime projection of the canonical
/// `appearance_session_record` describing what is active right now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSession {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Canonical record schema ref this session projects.
    pub canonical_record_schema_ref: String,
    /// Stable session ref every surface consumes.
    pub session_ref: String,
    /// Active theme-package ref.
    pub active_theme_package_ref: String,
    /// Active theme-package revision ref.
    pub active_theme_revision_ref: String,
    /// How the session resolves OS appearance signals.
    pub follow_system_posture: FollowSystemPosture,
    /// Resolved theme class.
    pub resolved_theme_class: ThemeClass,
    /// Effective contrast mode.
    pub contrast_mode: ContrastMode,
    /// Resolved accent source.
    pub accent_source: AccentSource,
    /// Effective density.
    pub density_class: DensityClass,
    /// Effective text scale.
    pub text_scale: TextScale,
    /// Effective reduced-motion posture.
    pub reduced_motion_posture: MotionPosture,
    /// Source of the reduced-motion posture.
    pub reduced_motion_source: ReducedMotionSource,
    /// OS appearance signal refs the session has observed.
    pub os_signal_refs: Vec<String>,
    /// Current preview state.
    pub preview_state: PreviewState,
    /// Current checkpoint ref, when a preview / committed change is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_checkpoint_ref: Option<String>,
    /// Rollback ref, when a rollback is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_ref: Option<String>,
    /// Active imported-theme report refs.
    pub active_import_report_refs: Vec<String>,
    /// Active token-overlay report refs.
    pub active_token_overlay_report_refs: Vec<String>,
    /// Extension-surface refs that ride this session.
    pub extension_surface_refs: Vec<String>,
    /// Policy context.
    pub policy_context: PolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Timestamp when the session record was minted.
    pub minted_at: String,
}

/// One explicit appearance checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceCheckpoint {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Canonical record schema ref this checkpoint projects.
    pub canonical_record_schema_ref: String,
    /// Stable checkpoint ref.
    pub checkpoint_ref: String,
    /// Owning session ref.
    pub session_ref: String,
    /// Checkpoint class.
    pub checkpoint_class: CheckpointClass,
    /// Checkpoint scope.
    pub checkpoint_scope: CheckpointScope,
    /// Pre-change snapshot ref.
    pub pre_change_snapshot_ref: String,
    /// Post-preview snapshot ref, when a preview produced one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_preview_snapshot_ref: Option<String>,
    /// Axes this checkpoint changed.
    pub changed_axes: Vec<AppearanceAxis>,
    /// Atomicity class.
    pub atomicity_class: AtomicityClass,
    /// Always `true`: the change is reversible from this single checkpoint.
    pub reversible_from_single_checkpoint: bool,
    /// The single rollback path.
    pub rollback_path: RollbackPath,
    /// Preflight checks recorded for the change.
    pub preflight_checks: Vec<PreflightCheck>,
    /// Apply outcome.
    pub apply_state: ApplyState,
    /// Policy context.
    pub policy_context: PolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Timestamp when the checkpoint was minted.
    pub minted_at: String,
}

/// One transition through the checkpoint-aware state machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceTransition {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable transition ref.
    pub transition_ref: String,
    /// Owning session ref.
    pub session_ref: String,
    /// Deterministic display ordering index.
    pub sequence_index: u32,
    /// State-machine operation.
    pub op: TransitionOp,
    /// Trigger that drove the transition.
    pub trigger: TransitionTrigger,
    /// The single checkpoint this transition flows through.
    pub checkpoint_ref: String,
    /// Preview state before the transition.
    pub from_preview_state: PreviewState,
    /// Preview state after the transition.
    pub to_preview_state: PreviewState,
    /// Apply state recorded by the transition.
    pub resulting_apply_state: ApplyState,
    /// Axes the transition changed.
    pub changed_axes: Vec<AppearanceAxis>,
    /// Atomicity class.
    pub atomicity_class: AtomicityClass,
    /// `true` when the change requires a surface reload or app restart.
    pub requires_restart_or_reload: bool,
    /// Always `true`: reversible from the single checkpoint.
    pub reversible_from_single_checkpoint: bool,
    /// Short privacy-safe summary.
    pub summary: String,
    /// Blocking findings detected for this transition.
    pub blocking_findings: Vec<AppearanceBlockingFinding>,
}

/// One claimed M5 surface bound to the appearance session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSurfaceBinding {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable surface id.
    pub surface_id: String,
    /// Surface family.
    pub surface_family: SurfaceFamily,
    /// Descriptor revision ref.
    pub descriptor_revision_ref: String,
    /// Canonical appearance anchor ref.
    pub appearance_anchor_ref: String,
    /// Non-empty accessibility note.
    pub accessibility_note: String,
    /// The session ref this surface consumes.
    pub consumes_session_ref: String,
    /// `true` when the surface rides the shared session model.
    pub registered_on_session: bool,
    /// How the surface applies appearance changes.
    pub live_apply_capability: LiveApplyCapability,
    /// `true` when any restart-or-reload requirement is disclosed.
    pub restart_or_reload_disclosed: bool,
    /// The checkpoint this surface last consumed, when any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_checkpoint_ref: Option<String>,
    /// `true` when the surface is marketed on desktop appearance rows.
    pub marketed: bool,
    /// Blocking findings detected for this surface.
    pub blocking_findings: Vec<AppearanceBlockingFinding>,
}

/// A blocking finding detected by the appearance-session runtime audit.
///
/// Every variant is always blocking: a clean audit carries none. The
/// `surface_id`, `checkpoint_ref`, `transition_ref`, and `session_ref` fields
/// are quoted so support, diagnostics, and golden-evidence flows can pivot
/// straight to the object that flagged the problem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum AppearanceBlockingFinding {
    /// A surface does not ride the shared appearance session.
    SurfaceNotOnSession {
        /// Surface id.
        surface_id: String,
    },
    /// A surface carries no canonical appearance anchor.
    SurfaceMissingAppearanceAnchor {
        /// Surface id.
        surface_id: String,
    },
    /// A surface carries no accessibility note.
    SurfaceMissingAccessibilityNote {
        /// Surface id.
        surface_id: String,
    },
    /// A surface consumes a session ref other than the live session.
    SurfaceSessionRefMismatch {
        /// Surface id.
        surface_id: String,
        /// The mismatched session ref the surface consumes.
        consumes_session_ref: String,
    },
    /// A surface needs a reload or restart but does not disclose it.
    SurfaceRestartReloadUndisclosed {
        /// Surface id.
        surface_id: String,
    },
    /// A surface's last-observed checkpoint is not in the ledger.
    SurfaceUnknownCheckpoint {
        /// Surface id.
        surface_id: String,
        /// The unresolved checkpoint ref.
        checkpoint_ref: String,
    },
    /// A transition is not a legal edge of the state machine.
    TransitionIllegalState {
        /// Transition ref.
        transition_ref: String,
    },
    /// A transition flows through no checkpoint.
    TransitionWithoutCheckpoint {
        /// Transition ref.
        transition_ref: String,
    },
    /// A transition names a checkpoint not in the ledger.
    TransitionUnknownCheckpoint {
        /// Transition ref.
        transition_ref: String,
        /// The unresolved checkpoint ref.
        checkpoint_ref: String,
    },
    /// A validation failure did not auto-revert (a half-updated state).
    ValidationFailureNotReverted {
        /// Transition ref.
        transition_ref: String,
    },
    /// A transition needs a reload or restart but its atomicity class hides it.
    TransitionRestartReloadUndisclosed {
        /// Transition ref.
        transition_ref: String,
    },
    /// A transition's atomicity class disagrees with its restart-or-reload flag.
    TransitionAtomicityMismatch {
        /// Transition ref.
        transition_ref: String,
    },
    /// A transition is not reversible from a single checkpoint.
    TransitionNonReversible {
        /// Transition ref.
        transition_ref: String,
    },
    /// A checkpoint is not reversible from a single checkpoint.
    CheckpointNonReversible {
        /// Checkpoint ref.
        checkpoint_ref: String,
    },
    /// A checkpoint carries no usable rollback path.
    CheckpointMissingRollbackPath {
        /// Checkpoint ref.
        checkpoint_ref: String,
    },
    /// A reload / restart checkpoint hides the requirement in its rollback path.
    CheckpointRestartReloadUndisclosed {
        /// Checkpoint ref.
        checkpoint_ref: String,
    },
    /// The live session is in a preview state with no current checkpoint.
    SessionPreviewWithoutCheckpoint {
        /// Session ref.
        session_ref: String,
    },
    /// The live session is rolled back with no rollback ref.
    SessionRollbackWithoutRef {
        /// Session ref.
        session_ref: String,
    },
    /// The live session's current checkpoint is not in the ledger.
    SessionUnknownCurrentCheckpoint {
        /// Session ref.
        session_ref: String,
        /// The unresolved checkpoint ref.
        checkpoint_ref: String,
    },
}

impl AppearanceBlockingFinding {
    /// Stable class token quoted in summaries and the CI gate.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceNotOnSession { .. } => "surface_not_on_session",
            Self::SurfaceMissingAppearanceAnchor { .. } => "surface_missing_appearance_anchor",
            Self::SurfaceMissingAccessibilityNote { .. } => "surface_missing_accessibility_note",
            Self::SurfaceSessionRefMismatch { .. } => "surface_session_ref_mismatch",
            Self::SurfaceRestartReloadUndisclosed { .. } => "surface_restart_reload_undisclosed",
            Self::SurfaceUnknownCheckpoint { .. } => "surface_unknown_checkpoint",
            Self::TransitionIllegalState { .. } => "transition_illegal_state",
            Self::TransitionWithoutCheckpoint { .. } => "transition_without_checkpoint",
            Self::TransitionUnknownCheckpoint { .. } => "transition_unknown_checkpoint",
            Self::ValidationFailureNotReverted { .. } => "validation_failure_not_reverted",
            Self::TransitionRestartReloadUndisclosed { .. } => {
                "transition_restart_reload_undisclosed"
            }
            Self::TransitionAtomicityMismatch { .. } => "transition_atomicity_mismatch",
            Self::TransitionNonReversible { .. } => "transition_non_reversible",
            Self::CheckpointNonReversible { .. } => "checkpoint_non_reversible",
            Self::CheckpointMissingRollbackPath { .. } => "checkpoint_missing_rollback_path",
            Self::CheckpointRestartReloadUndisclosed { .. } => {
                "checkpoint_restart_reload_undisclosed"
            }
            Self::SessionPreviewWithoutCheckpoint { .. } => "session_preview_without_checkpoint",
            Self::SessionRollbackWithoutRef { .. } => "session_rollback_without_ref",
            Self::SessionUnknownCurrentCheckpoint { .. } => "session_unknown_current_checkpoint",
        }
    }

    /// The owning object ref (surface, transition, checkpoint, or session).
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::SurfaceNotOnSession { surface_id }
            | Self::SurfaceMissingAppearanceAnchor { surface_id }
            | Self::SurfaceMissingAccessibilityNote { surface_id }
            | Self::SurfaceSessionRefMismatch { surface_id, .. }
            | Self::SurfaceRestartReloadUndisclosed { surface_id }
            | Self::SurfaceUnknownCheckpoint { surface_id, .. } => surface_id,
            Self::TransitionIllegalState { transition_ref }
            | Self::TransitionWithoutCheckpoint { transition_ref }
            | Self::TransitionUnknownCheckpoint { transition_ref, .. }
            | Self::ValidationFailureNotReverted { transition_ref }
            | Self::TransitionRestartReloadUndisclosed { transition_ref }
            | Self::TransitionAtomicityMismatch { transition_ref }
            | Self::TransitionNonReversible { transition_ref } => transition_ref,
            Self::CheckpointNonReversible { checkpoint_ref }
            | Self::CheckpointMissingRollbackPath { checkpoint_ref }
            | Self::CheckpointRestartReloadUndisclosed { checkpoint_ref } => checkpoint_ref,
            Self::SessionPreviewWithoutCheckpoint { session_ref }
            | Self::SessionRollbackWithoutRef { session_ref }
            | Self::SessionUnknownCurrentCheckpoint { session_ref, .. } => session_ref,
        }
    }
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceFindingSummary {
    /// Total blocking findings across every object.
    pub total_blocking_findings: usize,
    /// Surface-scoped blocking findings.
    pub surface_findings: usize,
    /// Transition-scoped blocking findings.
    pub transition_findings: usize,
    /// Checkpoint-scoped blocking findings.
    pub checkpoint_findings: usize,
    /// Session-scoped blocking findings.
    pub session_findings: usize,
}

impl AppearanceFindingSummary {
    /// Records one finding into the summary.
    fn record(&mut self, finding: &AppearanceBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            AppearanceBlockingFinding::SurfaceNotOnSession { .. }
            | AppearanceBlockingFinding::SurfaceMissingAppearanceAnchor { .. }
            | AppearanceBlockingFinding::SurfaceMissingAccessibilityNote { .. }
            | AppearanceBlockingFinding::SurfaceSessionRefMismatch { .. }
            | AppearanceBlockingFinding::SurfaceRestartReloadUndisclosed { .. }
            | AppearanceBlockingFinding::SurfaceUnknownCheckpoint { .. } => {
                self.surface_findings += 1;
            }
            AppearanceBlockingFinding::TransitionIllegalState { .. }
            | AppearanceBlockingFinding::TransitionWithoutCheckpoint { .. }
            | AppearanceBlockingFinding::TransitionUnknownCheckpoint { .. }
            | AppearanceBlockingFinding::ValidationFailureNotReverted { .. }
            | AppearanceBlockingFinding::TransitionRestartReloadUndisclosed { .. }
            | AppearanceBlockingFinding::TransitionAtomicityMismatch { .. }
            | AppearanceBlockingFinding::TransitionNonReversible { .. } => {
                self.transition_findings += 1;
            }
            AppearanceBlockingFinding::CheckpointNonReversible { .. }
            | AppearanceBlockingFinding::CheckpointMissingRollbackPath { .. }
            | AppearanceBlockingFinding::CheckpointRestartReloadUndisclosed { .. } => {
                self.checkpoint_findings += 1;
            }
            AppearanceBlockingFinding::SessionPreviewWithoutCheckpoint { .. }
            | AppearanceBlockingFinding::SessionRollbackWithoutRef { .. }
            | AppearanceBlockingFinding::SessionUnknownCurrentCheckpoint { .. } => {
                self.session_findings += 1;
            }
        }
    }
}

/// Computes the blocking findings for one transition against the checkpoint
/// ledger and the state-machine edges.
fn compute_transition_findings(
    transition: &AppearanceTransition,
    checkpoint_refs: &[String],
) -> Vec<AppearanceBlockingFinding> {
    let mut findings = Vec::new();
    let transition_ref = transition.transition_ref.clone();
    let op = transition.op;

    // Every change flows through exactly one checkpoint that resolves.
    if transition.checkpoint_ref.trim().is_empty() {
        findings.push(AppearanceBlockingFinding::TransitionWithoutCheckpoint {
            transition_ref: transition_ref.clone(),
        });
    } else if !checkpoint_refs.contains(&transition.checkpoint_ref) {
        findings.push(AppearanceBlockingFinding::TransitionUnknownCheckpoint {
            transition_ref: transition_ref.clone(),
            checkpoint_ref: transition.checkpoint_ref.clone(),
        });
    }

    // The edge must be legal: from-state allowed, and to / apply states match.
    let legal_from = op
        .legal_from_states()
        .contains(&transition.from_preview_state);
    let to_matches = transition.to_preview_state == op.expected_to_state();
    let apply_matches = transition.resulting_apply_state == op.expected_apply_state();
    if !legal_from || !to_matches || !apply_matches {
        findings.push(AppearanceBlockingFinding::TransitionIllegalState {
            transition_ref: transition_ref.clone(),
        });
    }

    // A validation failure must auto-revert; landing anywhere else is a
    // half-updated state.
    if op == TransitionOp::ValidationFailed
        && transition.to_preview_state != PreviewState::PreviewFailedReverted
    {
        findings.push(AppearanceBlockingFinding::ValidationFailureNotReverted {
            transition_ref: transition_ref.clone(),
        });
    }

    // Reversibility and restart/reload disclosure.
    if !transition.reversible_from_single_checkpoint {
        findings.push(AppearanceBlockingFinding::TransitionNonReversible {
            transition_ref: transition_ref.clone(),
        });
    }
    if transition.requires_restart_or_reload {
        if transition.atomicity_class.is_live() {
            findings.push(
                AppearanceBlockingFinding::TransitionRestartReloadUndisclosed {
                    transition_ref: transition_ref.clone(),
                },
            );
        }
    } else if !transition.atomicity_class.is_live() {
        findings.push(AppearanceBlockingFinding::TransitionAtomicityMismatch { transition_ref });
    }

    findings
}

/// Computes the blocking findings for one checkpoint.
fn compute_checkpoint_findings(
    checkpoint: &AppearanceCheckpoint,
) -> Vec<AppearanceBlockingFinding> {
    let mut findings = Vec::new();
    let checkpoint_ref = checkpoint.checkpoint_ref.clone();

    if !checkpoint.reversible_from_single_checkpoint {
        findings.push(AppearanceBlockingFinding::CheckpointNonReversible {
            checkpoint_ref: checkpoint_ref.clone(),
        });
    }

    let rollback = &checkpoint.rollback_path;
    if rollback.rollback_ref.trim().is_empty()
        || rollback.user_visible_action_id.trim().is_empty()
        || rollback.restores_axes.is_empty()
    {
        findings.push(AppearanceBlockingFinding::CheckpointMissingRollbackPath {
            checkpoint_ref: checkpoint_ref.clone(),
        });
    }

    // A reload / restart checkpoint must expose the requirement through its
    // rollback-path class; a live rollback path on a reload / restart change
    // hides the requirement.
    if !checkpoint.atomicity_class.is_live() && rollback.rollback_path_class.is_live() {
        findings
            .push(AppearanceBlockingFinding::CheckpointRestartReloadUndisclosed { checkpoint_ref });
    }

    findings
}

/// Computes the blocking findings for the live session against the ledger.
fn compute_session_findings(
    session: &AppearanceSession,
    checkpoint_refs: &[String],
) -> Vec<AppearanceBlockingFinding> {
    let mut findings = Vec::new();
    let session_ref = session.session_ref.clone();

    if session.preview_state.requires_current_checkpoint()
        && session.current_checkpoint_ref.is_none()
    {
        findings.push(AppearanceBlockingFinding::SessionPreviewWithoutCheckpoint {
            session_ref: session_ref.clone(),
        });
    }
    if session.preview_state == PreviewState::RollbackApplied && session.rollback_ref.is_none() {
        findings.push(AppearanceBlockingFinding::SessionRollbackWithoutRef {
            session_ref: session_ref.clone(),
        });
    }
    if let Some(checkpoint_ref) = &session.current_checkpoint_ref {
        if !checkpoint_refs.contains(checkpoint_ref) {
            findings.push(AppearanceBlockingFinding::SessionUnknownCurrentCheckpoint {
                session_ref,
                checkpoint_ref: checkpoint_ref.clone(),
            });
        }
    }

    findings
}

/// Computes the blocking findings for one surface binding.
fn compute_surface_findings(
    surface_id: &str,
    appearance_anchor_ref: &str,
    accessibility_note: &str,
    consumes_session_ref: &str,
    registered_on_session: bool,
    live_apply_capability: LiveApplyCapability,
    restart_or_reload_disclosed: bool,
    last_observed_checkpoint_ref: Option<&str>,
    session_ref: &str,
    checkpoint_refs: &[String],
) -> Vec<AppearanceBlockingFinding> {
    let mut findings = Vec::new();

    if !registered_on_session {
        findings.push(AppearanceBlockingFinding::SurfaceNotOnSession {
            surface_id: surface_id.to_owned(),
        });
    }
    if appearance_anchor_ref.trim().is_empty() {
        findings.push(AppearanceBlockingFinding::SurfaceMissingAppearanceAnchor {
            surface_id: surface_id.to_owned(),
        });
    }
    if accessibility_note.trim().is_empty() {
        findings.push(AppearanceBlockingFinding::SurfaceMissingAccessibilityNote {
            surface_id: surface_id.to_owned(),
        });
    }
    if consumes_session_ref != session_ref {
        findings.push(AppearanceBlockingFinding::SurfaceSessionRefMismatch {
            surface_id: surface_id.to_owned(),
            consumes_session_ref: consumes_session_ref.to_owned(),
        });
    }
    if !live_apply_capability.applies_live() && !restart_or_reload_disclosed {
        findings.push(AppearanceBlockingFinding::SurfaceRestartReloadUndisclosed {
            surface_id: surface_id.to_owned(),
        });
    }
    if let Some(checkpoint_ref) = last_observed_checkpoint_ref {
        if !checkpoint_ref.trim().is_empty()
            && !checkpoint_refs.contains(&checkpoint_ref.to_owned())
        {
            findings.push(AppearanceBlockingFinding::SurfaceUnknownCheckpoint {
                surface_id: surface_id.to_owned(),
                checkpoint_ref: checkpoint_ref.to_owned(),
            });
        }
    }

    findings
}

/// The canonical appearance-session runtime audit: the live session, the
/// checkpoint ledger, the transition ledger, and the per-surface bindings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSessionRuntimeReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Schema ref for the canonical appearance-session / checkpoint records.
    pub canonical_record_schema_ref: String,
    /// The live appearance session — what is active right now.
    pub session: AppearanceSession,
    /// The checkpoint ledger, sorted by `checkpoint_ref`.
    pub checkpoints: Vec<AppearanceCheckpoint>,
    /// The transition ledger, sorted by `sequence_index`.
    pub transitions: Vec<AppearanceTransition>,
    /// The per-surface bindings, sorted by `surface_id`.
    pub surfaces: Vec<AppearanceSurfaceBinding>,
    /// Per-class blocking-finding summary.
    pub findings_summary: AppearanceFindingSummary,
    /// Every blocking finding across the audit, sorted by class then subject.
    pub blocking_findings: Vec<AppearanceBlockingFinding>,
    /// Number of registered surfaces.
    pub registered_surface_count: usize,
    /// Number of surfaces marketed on desktop appearance rows.
    pub marketed_surface_count: usize,
    /// Number of surfaces that need a reload or restart for some change.
    pub restart_or_reload_surface_count: usize,
    /// `true` when at least one transition makes a live appearance change.
    pub live_change_demonstrated: bool,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl AppearanceSessionRuntimeReport {
    /// Returns the checkpoint registered under `checkpoint_ref`, if any.
    pub fn checkpoint(&self, checkpoint_ref: &str) -> Option<&AppearanceCheckpoint> {
        self.checkpoints
            .iter()
            .find(|checkpoint| checkpoint.checkpoint_ref == checkpoint_ref)
    }

    /// Returns `true` when every transition flows through a checkpoint that
    /// resolves in the ledger.
    pub fn every_transition_checkpoint_resolved(&self) -> bool {
        self.transitions
            .iter()
            .all(|transition| self.checkpoint(&transition.checkpoint_ref).is_some())
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: checkpoints={}, transitions={}, surfaces={}, marketed={}, restart_or_reload={}, blocking={}, clean={}",
            self.checkpoints.len(),
            self.transitions.len(),
            self.registered_surface_count,
            self.marketed_surface_count,
            self.restart_or_reload_surface_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        lines.push(format!(
            "session: {} -- theme={}, follow={}, contrast={}, accent={}, density={}, motion={}, preview={}",
            self.session.session_ref,
            self.session.resolved_theme_class.as_str(),
            self.session.follow_system_posture.as_str(),
            self.session.contrast_mode.as_str(),
            self.session.accent_source.as_str(),
            self.session.density_class.as_str(),
            self.session.reduced_motion_posture.as_str(),
            self.session.preview_state.as_str(),
        ));
        for transition in &self.transitions {
            lines.push(format!(
                "transition: {} -- op={}, {} -> {}, checkpoint={}, restart_or_reload={}",
                transition.transition_ref,
                transition.op.as_str(),
                transition.from_preview_state.as_str(),
                transition.to_preview_state.as_str(),
                transition.checkpoint_ref,
                transition.requires_restart_or_reload,
            ));
        }
        for surface in &self.surfaces {
            lines.push(format!(
                "surface: {} -- family={}, capability={}, restart_or_reload_disclosed={}",
                surface.surface_id,
                surface.surface_family.as_str(),
                surface.live_apply_capability.as_str(),
                surface.restart_or_reload_disclosed,
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref(),
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 appearance-session runtime audit\n\n");
        out.push_str(
            "Generated from the seeded runtime in\n\
             [`crate::appearance_session`](../../../../crates/aureline-shell/src/appearance_session/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- report-md > \\\n  artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Canonical record schema: `{}`\n",
            self.canonical_record_schema_ref
        ));
        out.push_str(&format!("- Live session: `{}`\n", self.session.session_ref));
        out.push_str(&format!("- Checkpoints: `{}`\n", self.checkpoints.len()));
        out.push_str(&format!("- Transitions: `{}`\n", self.transitions.len()));
        out.push_str(&format!(
            "- Registered surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_surface_count
        ));
        out.push_str(&format!(
            "- Surfaces needing reload/restart: `{}`\n",
            self.restart_or_reload_surface_count
        ));
        out.push_str(&format!(
            "- Live change demonstrated: `{}`\n",
            self.live_change_demonstrated
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Live appearance session\n\n");
        out.push_str("| Axis | Value | Source |\n| ---- | ----- | ------ |\n");
        out.push_str(&format!(
            "| Theme package | `{}` | `{}` |\n",
            self.session.active_theme_package_ref, self.session.active_theme_revision_ref
        ));
        out.push_str(&format!(
            "| Resolved theme | `{}` | `{}` |\n",
            self.session.resolved_theme_class.as_str(),
            self.session.follow_system_posture.as_str()
        ));
        out.push_str(&format!(
            "| Contrast | `{}` | — |\n",
            self.session.contrast_mode.as_str()
        ));
        out.push_str(&format!(
            "| Accent | `{}` | — |\n",
            self.session.accent_source.as_str()
        ));
        out.push_str(&format!(
            "| Density | `{}` | — |\n",
            self.session.density_class.as_str()
        ));
        out.push_str(&format!(
            "| Text scale | `{}%` | `{}` |\n",
            self.session.text_scale.scale_percent,
            self.session.text_scale.source.as_str()
        ));
        out.push_str(&format!(
            "| Reduced motion | `{}` | `{}` |\n",
            self.session.reduced_motion_posture.as_str(),
            self.session.reduced_motion_source.as_str()
        ));
        out.push_str(&format!(
            "| Preview state | `{}` | checkpoint `{}` |\n\n",
            self.session.preview_state.as_str(),
            self.session
                .current_checkpoint_ref
                .as_deref()
                .unwrap_or("none")
        ));

        out.push_str("## Checkpoint ledger\n\n");
        out.push_str(
            "| Checkpoint | Class | Scope | Atomicity | Apply | Reversible |\n\
             | ---------- | ----- | ----- | --------- | ----- | ---------- |\n",
        );
        for checkpoint in &self.checkpoints {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                checkpoint.checkpoint_ref,
                checkpoint.checkpoint_class.as_str(),
                checkpoint.checkpoint_scope.as_str(),
                checkpoint.atomicity_class.as_str(),
                checkpoint.apply_state.as_str(),
                checkpoint.reversible_from_single_checkpoint,
            ));
        }
        out.push('\n');

        out.push_str("## State-machine transitions\n\n");
        out.push_str(
            "| Seq | Op | Trigger | From | To | Checkpoint | Restart/Reload |\n\
             | --: | -- | ------- | ---- | -- | ---------- | -------------- |\n",
        );
        for transition in &self.transitions {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                transition.sequence_index,
                transition.op.as_str(),
                transition.trigger.as_str(),
                transition.from_preview_state.as_str(),
                transition.to_preview_state.as_str(),
                transition.checkpoint_ref,
                transition.requires_restart_or_reload,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface bindings\n\n");
        out.push_str(
            "| Surface | Family | Capability | Reload/Restart disclosed | Marketed |\n\
             | ------- | ------ | ---------- | ------------------------ | -------- |\n",
        );
        for surface in &self.surfaces {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                surface.surface_id,
                surface.surface_family.as_str(),
                surface.live_apply_capability.as_str(),
                surface.restart_or_reload_disclosed,
                surface.marketed,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Scope | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `session` | {} |\n",
            self.findings_summary.session_findings
        ));
        out.push_str(&format!(
            "| `checkpoint` | {} |\n",
            self.findings_summary.checkpoint_findings
        ));
        out.push_str(&format!(
            "| `transition` | {} |\n",
            self.findings_summary.transition_findings
        ));
        out.push_str(&format!(
            "| `surface` | {} |\n",
            self.findings_summary.surface_findings
        ));
        out.push_str(&format!(
            "| `total` | {} |\n\n",
            self.findings_summary.total_blocking_findings
        ));

        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            out.push_str("Findings:\n\n");
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_appearance_session_fixtures\n");
        out.push_str("python3 tools/ci/m5/appearance_session_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the appearance-session runtime audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSessionSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: AppearanceSessionRuntimeReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AppearanceSessionSupportExport {
    /// Builds the support-export wrapper for an audit report.
    ///
    /// Every report id, the session ref, each checkpoint ref, each transition
    /// ref, and each surface id and descriptor revision is quoted as a case id
    /// so a support reviewer — or a golden-evidence pack — can name the same
    /// appearance-session object the runtime used.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: AppearanceSessionRuntimeReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone(), report.session.session_ref.clone()];
        for checkpoint in &report.checkpoints {
            case_ids.push(checkpoint.checkpoint_ref.clone());
        }
        for transition in &report.transitions {
            case_ids.push(transition.transition_ref.clone());
        }
        for surface in &report.surfaces {
            case_ids.push(surface.surface_id.clone());
            case_ids.push(surface.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: APPEARANCE_SESSION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: APPEARANCE_SESSION_SCHEMA_VERSION,
            shared_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Builds an [`AppearanceSurfaceBinding`], computing its blocking findings
/// against the live session ref and the checkpoint ledger.
#[allow(clippy::too_many_arguments)]
pub fn build_appearance_surface_binding(
    surface_id: impl Into<String>,
    surface_family: SurfaceFamily,
    descriptor_revision_ref: impl Into<String>,
    appearance_anchor_ref: impl Into<String>,
    accessibility_note: impl Into<String>,
    consumes_session_ref: impl Into<String>,
    registered_on_session: bool,
    live_apply_capability: LiveApplyCapability,
    restart_or_reload_disclosed: bool,
    last_observed_checkpoint_ref: Option<String>,
    marketed: bool,
    session_ref: &str,
    checkpoint_refs: &[String],
) -> AppearanceSurfaceBinding {
    let surface_id = surface_id.into();
    let appearance_anchor_ref = appearance_anchor_ref.into();
    let accessibility_note = accessibility_note.into();
    let consumes_session_ref = consumes_session_ref.into();
    let blocking_findings = compute_surface_findings(
        &surface_id,
        &appearance_anchor_ref,
        &accessibility_note,
        &consumes_session_ref,
        registered_on_session,
        live_apply_capability,
        restart_or_reload_disclosed,
        last_observed_checkpoint_ref.as_deref(),
        session_ref,
        checkpoint_refs,
    );
    AppearanceSurfaceBinding {
        record_kind: APPEARANCE_SURFACE_RECORD_KIND.to_owned(),
        schema_version: APPEARANCE_SESSION_SCHEMA_VERSION,
        shared_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
        surface_id,
        surface_family,
        descriptor_revision_ref: descriptor_revision_ref.into(),
        appearance_anchor_ref,
        accessibility_note,
        consumes_session_ref,
        registered_on_session,
        live_apply_capability,
        restart_or_reload_disclosed,
        last_observed_checkpoint_ref,
        marketed,
        blocking_findings,
    }
}

/// Builds a full [`AppearanceSessionRuntimeReport`] from the live session, the
/// checkpoint ledger, the transition ledger, and the per-surface bindings.
///
/// Transition findings are (re)computed against the checkpoint ledger, so the
/// transitions passed in need not carry findings; surface bindings keep the
/// findings computed by [`build_appearance_surface_binding`].
pub fn build_appearance_session_runtime(
    session: AppearanceSession,
    checkpoints: Vec<AppearanceCheckpoint>,
    transitions: Vec<AppearanceTransition>,
    surfaces: Vec<AppearanceSurfaceBinding>,
) -> AppearanceSessionRuntimeReport {
    let mut checkpoints = checkpoints;
    checkpoints.sort_by(|left, right| left.checkpoint_ref.cmp(&right.checkpoint_ref));
    let mut transitions = transitions;
    transitions.sort_by(|left, right| left.sequence_index.cmp(&right.sequence_index));
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    let checkpoint_refs: Vec<String> = checkpoints
        .iter()
        .map(|checkpoint| checkpoint.checkpoint_ref.clone())
        .collect();

    // Recompute transition findings against the ledger so the report is the
    // single source of truth even if callers pass bare transitions.
    for transition in &mut transitions {
        transition.blocking_findings = compute_transition_findings(transition, &checkpoint_refs);
    }

    let mut findings_summary = AppearanceFindingSummary::default();
    let mut blocking_findings: Vec<AppearanceBlockingFinding> = Vec::new();

    for finding in compute_session_findings(&session, &checkpoint_refs) {
        findings_summary.record(&finding);
        blocking_findings.push(finding);
    }
    for checkpoint in &checkpoints {
        for finding in compute_checkpoint_findings(checkpoint) {
            findings_summary.record(&finding);
            blocking_findings.push(finding);
        }
    }
    for transition in &transitions {
        for finding in &transition.blocking_findings {
            findings_summary.record(finding);
            blocking_findings.push(finding.clone());
        }
    }
    for surface in &surfaces {
        for finding in &surface.blocking_findings {
            findings_summary.record(finding);
            blocking_findings.push(finding.clone());
        }
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });

    let registered_surface_count = surfaces.len();
    let marketed_surface_count = surfaces.iter().filter(|s| s.marketed).count();
    let restart_or_reload_surface_count = surfaces
        .iter()
        .filter(|s| !s.live_apply_capability.applies_live())
        .count();
    let live_change_demonstrated = transitions
        .iter()
        .any(|transition| transition.op.is_live_change());
    let report_clean = findings_summary.total_blocking_findings == 0;

    AppearanceSessionRuntimeReport {
        record_kind: APPEARANCE_SESSION_REPORT_RECORD_KIND.to_owned(),
        schema_version: APPEARANCE_SESSION_SCHEMA_VERSION,
        shared_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
        report_id: APPEARANCE_SESSION_REPORT_ID.to_owned(),
        source_schema_ref: APPEARANCE_SESSION_SOURCE_SCHEMA_REF.to_owned(),
        canonical_record_schema_ref: APPEARANCE_SESSION_CANONICAL_RECORD_SCHEMA_REF.to_owned(),
        session,
        checkpoints,
        transitions,
        surfaces,
        findings_summary,
        blocking_findings,
        registered_surface_count,
        marketed_surface_count,
        restart_or_reload_surface_count,
        live_change_demonstrated,
        report_clean,
        published_report_ref: APPEARANCE_SESSION_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: APPEARANCE_SESSION_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            APPEARANCE_SESSION_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/theme-package-and-appearance-objects.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-appearance-session".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_appearance_session_runtime`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AppearanceSessionValidationError {
    /// The audit has no registered checkpoints.
    NoRegisteredCheckpoints,
    /// The audit has no registered transitions.
    NoRegisteredTransitions,
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A transition's checkpoint does not resolve in the ledger.
    TransitionCheckpointUnresolved {
        /// Transition ref.
        transition_ref: String,
        /// The unresolved checkpoint ref.
        checkpoint_ref: String,
    },
    /// A blocking finding remains in the audit.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning object ref.
        subject_ref: String,
    },
    /// No transition demonstrates a live appearance change.
    NoLiveChangeDemonstrated,
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates an audit report against the appearance-session acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_appearance_session_runtime(
    report: &AppearanceSessionRuntimeReport,
) -> Result<(), Vec<AppearanceSessionValidationError>> {
    let mut errors = Vec::new();

    if report.checkpoints.is_empty() {
        errors.push(AppearanceSessionValidationError::NoRegisteredCheckpoints);
    }
    if report.transitions.is_empty() {
        errors.push(AppearanceSessionValidationError::NoRegisteredTransitions);
    }
    if report.surfaces.is_empty() {
        errors.push(AppearanceSessionValidationError::NoRegisteredSurfaces);
    }

    for transition in &report.transitions {
        if report.checkpoint(&transition.checkpoint_ref).is_none() {
            errors.push(
                AppearanceSessionValidationError::TransitionCheckpointUnresolved {
                    transition_ref: transition.transition_ref.clone(),
                    checkpoint_ref: transition.checkpoint_ref.clone(),
                },
            );
        }
    }

    for finding in &report.blocking_findings {
        errors.push(AppearanceSessionValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if !report.live_change_demonstrated {
        errors.push(AppearanceSessionValidationError::NoLiveChangeDemonstrated);
    }
    if report.published_report_ref.trim().is_empty() {
        errors.push(AppearanceSessionValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(AppearanceSessionValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Deterministic policy context shared by every seeded record.
fn seed_policy_context() -> PolicyContext {
    PolicyContext {
        policy_epoch: "policy-epoch:2026-06".to_owned(),
        trust_state: "trusted".to_owned(),
        execution_context_id: Some("exec:shell:primary".to_owned()),
    }
}

/// A passed preflight check.
fn preflight_passed(check_class: PreflightCheckClass) -> PreflightCheck {
    PreflightCheck {
        check_class,
        result_state: CheckResultState::Passed,
        finding_ref: None,
    }
}

/// A failed preflight check that cites a finding ref.
fn preflight_failed(check_class: PreflightCheckClass, finding_ref: &str) -> PreflightCheck {
    PreflightCheck {
        check_class,
        result_state: CheckResultState::FailedBlocked,
        finding_ref: Some(finding_ref.to_owned()),
    }
}

/// Builds one seeded checkpoint.
#[allow(clippy::too_many_arguments)]
fn seed_checkpoint(
    checkpoint_ref: &str,
    checkpoint_class: CheckpointClass,
    checkpoint_scope: CheckpointScope,
    pre_change_snapshot_ref: &str,
    post_preview_snapshot_ref: Option<&str>,
    changed_axes: Vec<AppearanceAxis>,
    atomicity_class: AtomicityClass,
    rollback_path_class: RollbackPathClass,
    rollback_ref: &str,
    user_visible_action_id: &str,
    rollback_requires_restart: bool,
    preflight_checks: Vec<PreflightCheck>,
    apply_state: ApplyState,
) -> AppearanceCheckpoint {
    AppearanceCheckpoint {
        record_kind: APPEARANCE_CHECKPOINT_RECORD_KIND.to_owned(),
        schema_version: APPEARANCE_SESSION_SCHEMA_VERSION,
        shared_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
        canonical_record_schema_ref: APPEARANCE_SESSION_CANONICAL_RECORD_SCHEMA_REF.to_owned(),
        checkpoint_ref: checkpoint_ref.to_owned(),
        session_ref: "appearance-session:primary".to_owned(),
        checkpoint_class,
        checkpoint_scope,
        pre_change_snapshot_ref: pre_change_snapshot_ref.to_owned(),
        post_preview_snapshot_ref: post_preview_snapshot_ref.map(str::to_owned),
        rollback_path: RollbackPath {
            rollback_path_class,
            rollback_ref: rollback_ref.to_owned(),
            user_visible_action_id: user_visible_action_id.to_owned(),
            restores_axes: changed_axes.clone(),
            requires_restart: rollback_requires_restart,
        },
        changed_axes,
        atomicity_class,
        reversible_from_single_checkpoint: true,
        preflight_checks,
        apply_state,
        policy_context: seed_policy_context(),
        redaction_class: RedactionClass::MetadataSafeDefault,
        minted_at: GENERATED_AT.to_owned(),
    }
}

/// Builds one seeded transition. The `to` and apply states are derived from the
/// operation so the seed can never describe an illegal edge by accident.
#[allow(clippy::too_many_arguments)]
fn seed_transition(
    transition_ref: &str,
    sequence_index: u32,
    op: TransitionOp,
    trigger: TransitionTrigger,
    checkpoint_ref: &str,
    from_preview_state: PreviewState,
    changed_axes: Vec<AppearanceAxis>,
    atomicity_class: AtomicityClass,
    requires_restart_or_reload: bool,
    summary: &str,
) -> AppearanceTransition {
    AppearanceTransition {
        record_kind: APPEARANCE_TRANSITION_RECORD_KIND.to_owned(),
        schema_version: APPEARANCE_SESSION_SCHEMA_VERSION,
        shared_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
        transition_ref: transition_ref.to_owned(),
        session_ref: "appearance-session:primary".to_owned(),
        sequence_index,
        op,
        trigger,
        checkpoint_ref: checkpoint_ref.to_owned(),
        from_preview_state,
        to_preview_state: op.expected_to_state(),
        resulting_apply_state: op.expected_apply_state(),
        changed_axes,
        atomicity_class,
        requires_restart_or_reload,
        reversible_from_single_checkpoint: true,
        summary: summary.to_owned(),
        blocking_findings: Vec::new(),
    }
}

/// Returns the seeded, deterministic appearance-session runtime audit.
///
/// The audit is the single mint-from-truth source for the fixtures under
/// `fixtures/ux/m5/live-appearance-change/` and the markdown artifact under
/// `artifacts/ux/m5/appearance-session-checkpoints/`.
///
/// The seed exercises every disclosed-appearance scenario the contract keeps
/// honest: a live, inspectable theme preview; an atomically applied OS contrast
/// signal; a cancelled overlay preview; a contrast-preflight failure that
/// auto-reverts; a managed-policy density commit; and an imported-theme
/// rollback that discloses its surface-reload requirement.
pub fn seeded_appearance_session_runtime() -> AppearanceSessionRuntimeReport {
    let session = AppearanceSession {
        record_kind: APPEARANCE_SESSION_RECORD_KIND.to_owned(),
        schema_version: APPEARANCE_SESSION_SCHEMA_VERSION,
        shared_contract_ref: APPEARANCE_SESSION_SHARED_CONTRACT_REF.to_owned(),
        canonical_record_schema_ref: APPEARANCE_SESSION_CANONICAL_RECORD_SCHEMA_REF.to_owned(),
        session_ref: "appearance-session:primary".to_owned(),
        active_theme_package_ref: "theme-pkg:aureline-default".to_owned(),
        active_theme_revision_ref: "theme-rev:aureline-default:1.4.0".to_owned(),
        follow_system_posture: FollowSystemPosture::FollowSystem,
        resolved_theme_class: ThemeClass::DarkReference,
        contrast_mode: ContrastMode::ContrastStandard,
        accent_source: AccentSource::SystemAccent,
        density_class: DensityClass::Standard,
        text_scale: TextScale {
            scale_percent: 100,
            source: TextScaleSource::System,
            locked_by_policy: false,
        },
        reduced_motion_posture: MotionPosture::MotionStandard,
        reduced_motion_source: ReducedMotionSource::OsSignal,
        os_signal_refs: vec![
            "os-signal:appearance:dark".to_owned(),
            "os-signal:contrast:standard".to_owned(),
        ],
        preview_state: PreviewState::PreviewLive,
        current_checkpoint_ref: Some("appearance-checkpoint:preview-light".to_owned()),
        rollback_ref: Some("rollback:preview-light".to_owned()),
        active_import_report_refs: vec!["theme-import-report:partner-dusk".to_owned()],
        active_token_overlay_report_refs: vec!["token-overlay-report:accent-warmth".to_owned()],
        extension_surface_refs: vec!["surface:extension.dusk-panel".to_owned()],
        policy_context: seed_policy_context(),
        redaction_class: RedactionClass::MetadataSafeDefault,
        minted_at: GENERATED_AT.to_owned(),
    };

    let checkpoints = vec![
        seed_checkpoint(
            "appearance-checkpoint:preview-light",
            CheckpointClass::AppearancePreviewCheckpoint,
            CheckpointScope::GlobalAppearance,
            "snapshot:pre-light",
            Some("snapshot:post-light"),
            vec![AppearanceAxis::ThemePackage, AppearanceAxis::FollowSystem],
            AtomicityClass::SingleCheckpointAtomic,
            RollbackPathClass::SingleCheckpointRevert,
            "rollback:preview-light",
            "action:appearance.revert-preview",
            false,
            vec![
                preflight_passed(PreflightCheckClass::ContrastTargets),
                preflight_passed(PreflightCheckClass::ProtectedCuePreservation),
                preflight_passed(PreflightCheckClass::RollbackPathPresent),
            ],
            ApplyState::PreviewLive,
        ),
        seed_checkpoint(
            "appearance-checkpoint:os-contrast",
            CheckpointClass::AppearanceOsSignalCheckpoint,
            CheckpointScope::GlobalAppearance,
            "snapshot:pre-contrast",
            Some("snapshot:post-contrast"),
            vec![AppearanceAxis::Contrast],
            AtomicityClass::SingleCheckpointAtomic,
            RollbackPathClass::SingleCheckpointRevert,
            "rollback:os-contrast",
            "action:appearance.revert-contrast",
            false,
            vec![
                preflight_passed(PreflightCheckClass::ContrastTargets),
                preflight_passed(PreflightCheckClass::ProtectedCuePreservation),
                preflight_passed(PreflightCheckClass::RollbackPathPresent),
            ],
            ApplyState::Committed,
        ),
        seed_checkpoint(
            "appearance-checkpoint:overlay-accent",
            CheckpointClass::AppearanceOverlayCheckpoint,
            CheckpointScope::ProfileAppearance,
            "snapshot:pre-overlay",
            Some("snapshot:post-overlay"),
            vec![AppearanceAxis::TokenOverlay, AppearanceAxis::Accent],
            AtomicityClass::SingleCheckpointAtomic,
            RollbackPathClass::SingleCheckpointRevert,
            "rollback:overlay-accent",
            "action:appearance.revert-overlay",
            false,
            vec![
                preflight_passed(PreflightCheckClass::ContrastTargets),
                preflight_passed(PreflightCheckClass::RollbackPathPresent),
            ],
            ApplyState::Reverted,
        ),
        seed_checkpoint(
            "appearance-checkpoint:partner-preview-failed",
            CheckpointClass::AppearancePreviewCheckpoint,
            CheckpointScope::PreviewOnly,
            "snapshot:pre-partner",
            None,
            vec![AppearanceAxis::ThemePackage],
            AtomicityClass::SingleCheckpointAtomic,
            RollbackPathClass::SingleCheckpointRevert,
            "rollback:partner-preview",
            "action:appearance.revert-partner-preview",
            false,
            vec![
                preflight_failed(
                    PreflightCheckClass::ContrastTargets,
                    "finding:partner-preview:contrast",
                ),
                preflight_passed(PreflightCheckClass::RollbackPathPresent),
            ],
            ApplyState::PreflightFailed,
        ),
        seed_checkpoint(
            "appearance-checkpoint:import-dusk",
            CheckpointClass::AppearanceImportCheckpoint,
            CheckpointScope::WorkspaceAppearance,
            "snapshot:pre-import",
            Some("snapshot:post-import"),
            vec![AppearanceAxis::ThemePackage, AppearanceAxis::ImportMapping],
            AtomicityClass::SurfaceReloadFromSingleCheckpoint,
            RollbackPathClass::SurfaceReloadThenRevert,
            "rollback:import-dusk",
            "action:appearance.revert-import",
            false,
            vec![
                preflight_passed(PreflightCheckClass::ImportMappingReportPresent),
                preflight_passed(PreflightCheckClass::SyntaxCoveragePresent),
                preflight_passed(PreflightCheckClass::RollbackPathPresent),
            ],
            ApplyState::Committed,
        ),
        seed_checkpoint(
            "appearance-checkpoint:policy-density",
            CheckpointClass::AppearancePolicyCheckpoint,
            CheckpointScope::GlobalAppearance,
            "snapshot:pre-density",
            Some("snapshot:post-density"),
            vec![AppearanceAxis::Density],
            AtomicityClass::SingleCheckpointAtomic,
            RollbackPathClass::SingleCheckpointRevert,
            "rollback:policy-density",
            "action:appearance.revert-density",
            false,
            vec![
                preflight_passed(PreflightCheckClass::ProtectedCuePreservation),
                preflight_passed(PreflightCheckClass::RollbackPathPresent),
            ],
            ApplyState::Committed,
        ),
    ];

    let transitions = vec![
        seed_transition(
            "transition:os-contrast-applied",
            1,
            TransitionOp::OsSignalApplied,
            TransitionTrigger::OsSignal,
            "appearance-checkpoint:os-contrast",
            PreviewState::NotPreviewing,
            vec![AppearanceAxis::Contrast],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "An OS contrast change is applied atomically through one checkpoint; no half-updated state.",
        ),
        seed_transition(
            "transition:open-light-preview",
            2,
            TransitionOp::OpenPreview,
            TransitionTrigger::UserAction,
            "appearance-checkpoint:preview-light",
            PreviewState::NotPreviewing,
            vec![AppearanceAxis::ThemePackage, AppearanceAxis::FollowSystem],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "The user opens a light-theme preview; a single checkpoint is created.",
        ),
        seed_transition(
            "transition:light-preview-live",
            3,
            TransitionOp::PreflightPassed,
            TransitionTrigger::UserAction,
            "appearance-checkpoint:preview-light",
            PreviewState::PreviewPendingValidation,
            vec![AppearanceAxis::ThemePackage, AppearanceAxis::FollowSystem],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "Preflight passes; the light-theme preview goes live and is inspectable.",
        ),
        seed_transition(
            "transition:cancel-overlay-preview",
            4,
            TransitionOp::CancelPreview,
            TransitionTrigger::UserAction,
            "appearance-checkpoint:overlay-accent",
            PreviewState::PreviewLive,
            vec![AppearanceAxis::TokenOverlay, AppearanceAxis::Accent],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "The user cancels the accent-overlay preview; the single checkpoint reverts it cleanly.",
        ),
        seed_transition(
            "transition:partner-preview-failed",
            5,
            TransitionOp::ValidationFailed,
            TransitionTrigger::UserAction,
            "appearance-checkpoint:partner-preview-failed",
            PreviewState::PreviewPendingValidation,
            vec![AppearanceAxis::ThemePackage],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "A partner-theme preview fails the contrast preflight and auto-reverts from its checkpoint.",
        ),
        seed_transition(
            "transition:revert-imported-dusk",
            6,
            TransitionOp::RevertCommitted,
            TransitionTrigger::SyncImport,
            "appearance-checkpoint:import-dusk",
            PreviewState::PreviewCommitted,
            vec![AppearanceAxis::ThemePackage, AppearanceAxis::ImportMapping],
            AtomicityClass::SurfaceReloadFromSingleCheckpoint,
            true,
            "An imported partner theme is rolled back; the surface reloads from one checkpoint, disclosed.",
        ),
        seed_transition(
            "transition:commit-policy-density",
            7,
            TransitionOp::CommitPreview,
            TransitionTrigger::ManagedPolicy,
            "appearance-checkpoint:policy-density",
            PreviewState::PreviewLive,
            vec![AppearanceAxis::Density],
            AtomicityClass::SingleCheckpointAtomic,
            false,
            "A managed density change is committed through one checkpoint.",
        ),
    ];

    let session_ref = "appearance-session:primary";
    let checkpoint_refs: Vec<String> = checkpoints
        .iter()
        .map(|checkpoint| checkpoint.checkpoint_ref.clone())
        .collect();

    let surfaces = vec![
        build_appearance_surface_binding(
            "surface:notebook.cell-chrome",
            SurfaceFamily::Notebook,
            "rev:notebook.cell-chrome:1",
            "anchor:notebook.cell-chrome",
            "Notebook cell chrome keeps trust, severity, and lifecycle cues across the live session.",
            session_ref,
            true,
            LiveApplyCapability::AppliesLive,
            false,
            Some("appearance-checkpoint:preview-light".to_owned()),
            true,
            session_ref,
            &checkpoint_refs,
        ),
        build_appearance_surface_binding(
            "surface:data.result-grid",
            SurfaceFamily::DataResultSurface,
            "rev:data.result-grid:1",
            "anchor:data.result-grid",
            "Result-grid rows keep semantic status legible across contrast and density changes.",
            session_ref,
            true,
            LiveApplyCapability::AppliesLive,
            false,
            Some("appearance-checkpoint:os-contrast".to_owned()),
            true,
            session_ref,
            &checkpoint_refs,
        ),
        build_appearance_surface_binding(
            "surface:preview.browser-pane",
            SurfaceFamily::PreviewBrowserPane,
            "rev:preview.browser-pane:1",
            "anchor:preview.browser-pane",
            "Preview/browser pane discloses that an imported theme applies after a surface reload.",
            session_ref,
            true,
            LiveApplyCapability::RequiresSurfaceReload,
            true,
            Some("appearance-checkpoint:import-dusk".to_owned()),
            true,
            session_ref,
            &checkpoint_refs,
        ),
        build_appearance_surface_binding(
            "surface:docs.help-pane",
            SurfaceFamily::DocsHelpPane,
            "rev:docs.help-pane:1",
            "anchor:docs.help-pane",
            "Docs/help pane reflects the live appearance session and keeps focus visible.",
            session_ref,
            true,
            LiveApplyCapability::AppliesLive,
            false,
            Some("appearance-checkpoint:policy-density".to_owned()),
            true,
            session_ref,
            &checkpoint_refs,
        ),
        build_appearance_surface_binding(
            "surface:companion.sidecar",
            SurfaceFamily::CompanionSurface,
            "rev:companion.sidecar:1",
            "anchor:companion.sidecar",
            "Companion sidecar discloses that an OS theme change applies after an app restart.",
            session_ref,
            true,
            LiveApplyCapability::RequiresAppRestart,
            true,
            Some("appearance-checkpoint:os-contrast".to_owned()),
            true,
            session_ref,
            &checkpoint_refs,
        ),
        build_appearance_surface_binding(
            "surface:extension.dusk-panel",
            SurfaceFamily::ExtensionHostedSurface,
            "rev:extension.dusk-panel:1",
            "anchor:extension.dusk-panel",
            "Extension-hosted panel discloses that an overlay applies after a surface reload.",
            session_ref,
            true,
            LiveApplyCapability::RequiresSurfaceReload,
            true,
            Some("appearance-checkpoint:overlay-accent".to_owned()),
            false,
            session_ref,
            &checkpoint_refs,
        ),
    ];

    build_appearance_session_runtime(session, checkpoints, transitions, surfaces)
}
