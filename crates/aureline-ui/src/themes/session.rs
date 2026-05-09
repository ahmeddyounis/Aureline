//! Appearance-session record vocabulary.
//!
//! The desktop shell persists an [`AppearanceSessionRecord`] so theme switching
//! is explicit and restart-stable. The record mirrors the schema frozen in
//! `schemas/design/appearance_session.schema.json`, but the shell currently
//! uses only the theme-class selection for rendering.

use serde::{Deserialize, Serialize};

use crate::density::DensityClass;
use crate::tokens::ThemeClass;

/// Identifies the `appearance_session_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceSessionRecordKind {
    /// `appearance_session_record`
    AppearanceSessionRecord,
}

/// Contrast mode for the in-effect appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContrastMode {
    ContrastStandard,
    ContrastHigh,
    ContrastForcedColors,
}

/// Accent source resolution for the in-effect appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccentSourceClass {
    SystemAccent,
    ThemePackageAccent,
    UserSelectedAccent,
    PolicyLockedAccent,
    NotApplicable,
}

/// Accessibility posture class for reduced-motion and hot-path rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityPostureClass {
    MotionStandard,
    MotionReduced,
    MotionLowMotion,
    MotionPowerSaver,
    MotionCriticalHotPath,
}

/// Source attribution for a text-scale selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextScaleSource {
    System,
    User,
    Profile,
    Workspace,
    Policy,
}

/// Text-scale configuration for the in-effect appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextScale {
    pub scale_percent: u32,
    pub source: TextScaleSource,
    pub locked_by_policy: bool,
}

/// Reduced-motion source attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReducedMotionSource {
    OsSignal,
    UserSetting,
    PolicyCap,
    PowerSaverSignal,
    CriticalHotPath,
    NotApplicable,
}

/// Follow-system posture for the in-effect appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowSystemPosture {
    FollowSystem,
    ManualOverride,
    ManagedPolicyOverride,
    UnavailablePlatformSignal,
}

/// Preview state for the in-effect appearance session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewState {
    NotPreviewing,
    PreviewPendingValidation,
    PreviewLive,
    PreviewFailedReverted,
    PreviewCommitted,
    RollbackApplied,
}

/// Trust-state stamp applied to appearance records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Trusted,
    Restricted,
}

/// Redaction class applied to appearance records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

/// Policy context stamped onto appearance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub policy_epoch: String,
    pub trust_state: TrustState,
    pub execution_context_id: Option<String>,
}

/// Persisted appearance session record used to restore theme selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSessionRecord {
    pub record_kind: AppearanceSessionRecordKind,
    pub appearance_session_schema_version: u32,
    pub appearance_session_id: String,
    pub session_revision: u64,
    pub active_theme_package_ref: String,
    pub active_theme_revision_ref: String,
    pub mode_theme_class: ThemeClass,
    pub contrast_mode: ContrastMode,
    pub accent_source: AccentSourceClass,
    pub density_class: DensityClass,
    pub text_scale: TextScale,
    pub reduced_motion_posture: AccessibilityPostureClass,
    pub reduced_motion_source: ReducedMotionSource,
    pub follow_system_posture: FollowSystemPosture,
    pub preview_state: PreviewState,
    pub current_checkpoint_ref: Option<String>,
    pub rollback_ref: Option<String>,
    pub live_follow_system_policy_ref: String,
    pub token_overlay_ref: Option<String>,
    pub active_import_report_refs: Vec<String>,
    pub confirm_action_ref: Option<String>,
    pub notes: Option<String>,
    pub policy_context: PolicyContext,
    pub redaction_class: RedactionClass,
    pub revision_minted_at: String,
}

impl AppearanceSessionRecord {
    /// Returns the in-effect theme class.
    pub const fn theme_class(&self) -> ThemeClass {
        self.mode_theme_class
    }

    /// Returns the in-effect density class.
    pub const fn density_class(&self) -> DensityClass {
        self.density_class
    }

    /// Returns a stable default appearance session record for first-party surfaces.
    pub fn first_party_default(minted_at: String) -> Self {
        let theme = ThemeClass::DarkReference;
        Self {
            record_kind: AppearanceSessionRecordKind::AppearanceSessionRecord,
            appearance_session_schema_version: 1,
            appearance_session_id: "appearance_session:shell.local:01".to_string(),
            session_revision: 0,
            active_theme_package_ref: "theme_package:default.aureline:01".to_string(),
            active_theme_revision_ref: "theme_package_rev:default.aureline:2026-04-29:01"
                .to_string(),
            mode_theme_class: theme,
            contrast_mode: contrast_mode_for_theme(theme),
            accent_source: AccentSourceClass::ThemePackageAccent,
            density_class: DensityClass::Standard,
            text_scale: TextScale {
                scale_percent: 100,
                source: TextScaleSource::User,
                locked_by_policy: false,
            },
            reduced_motion_posture: AccessibilityPostureClass::MotionStandard,
            reduced_motion_source: ReducedMotionSource::NotApplicable,
            follow_system_posture: FollowSystemPosture::ManualOverride,
            preview_state: PreviewState::NotPreviewing,
            current_checkpoint_ref: None,
            rollback_ref: None,
            live_follow_system_policy_ref: "live_follow_system_policy:shell.local:01"
                .to_string(),
            token_overlay_ref: None,
            active_import_report_refs: Vec::new(),
            confirm_action_ref: None,
            notes: None,
            policy_context: PolicyContext {
                policy_epoch: "pe:2026-04-29:01".to_string(),
                trust_state: TrustState::Trusted,
                execution_context_id: None,
            },
            redaction_class: RedactionClass::MetadataSafeDefault,
            revision_minted_at: minted_at,
        }
    }

    /// Applies a theme-class change, bumping the session revision and updating derived fields.
    pub fn apply_theme_class(&mut self, theme: ThemeClass, minted_at: String) {
        if self.mode_theme_class == theme {
            self.revision_minted_at = minted_at;
            return;
        }
        self.session_revision = self.session_revision.saturating_add(1);
        self.mode_theme_class = theme;
        self.contrast_mode = contrast_mode_for_theme(theme);
        self.follow_system_posture = FollowSystemPosture::ManualOverride;
        self.preview_state = PreviewState::NotPreviewing;
        self.current_checkpoint_ref = None;
        self.rollback_ref = None;
        self.revision_minted_at = minted_at;
    }

    /// Toggles between dark and light within the active contrast posture.
    pub fn toggle_light_dark(&mut self, minted_at: String) {
        let next = match self.mode_theme_class {
            ThemeClass::DarkReference => ThemeClass::LightParity,
            ThemeClass::LightParity => ThemeClass::DarkReference,
            ThemeClass::HighContrastDark => ThemeClass::HighContrastLight,
            ThemeClass::HighContrastLight => ThemeClass::HighContrastDark,
        };
        self.apply_theme_class(next, minted_at);
    }

    /// Toggles between standard and high-contrast theme rows.
    pub fn toggle_high_contrast(&mut self, minted_at: String) {
        let next = match self.mode_theme_class {
            ThemeClass::DarkReference => ThemeClass::HighContrastDark,
            ThemeClass::LightParity => ThemeClass::HighContrastLight,
            ThemeClass::HighContrastDark => ThemeClass::DarkReference,
            ThemeClass::HighContrastLight => ThemeClass::LightParity,
        };
        self.apply_theme_class(next, minted_at);
    }

    /// Applies a density-class change, bumping the session revision.
    pub fn apply_density_class(&mut self, density: DensityClass, minted_at: String) {
        if self.density_class == density {
            self.revision_minted_at = minted_at;
            return;
        }
        self.session_revision = self.session_revision.saturating_add(1);
        self.density_class = density;
        self.follow_system_posture = FollowSystemPosture::ManualOverride;
        self.preview_state = PreviewState::NotPreviewing;
        self.current_checkpoint_ref = None;
        self.rollback_ref = None;
        self.revision_minted_at = minted_at;
    }

    /// Cycles the density class (`compact` → `standard` → `comfortable`).
    pub fn cycle_density_class(&mut self, minted_at: String) {
        self.apply_density_class(self.density_class.next(), minted_at);
    }
}

/// Identifies the `live_follow_system_policy_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveFollowSystemPolicyRecordKind {
    /// `live_follow_system_policy_record`
    LiveFollowSystemPolicyRecord,
}

/// Appearance axis covered by a live follow-system policy record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceAxis {
    ModeThemeClass,
    ContrastMode,
    AccentSource,
    DensityClass,
    TextScale,
    ReducedMotionPosture,
    FollowSystemPosture,
}

/// Live-update policy class for one appearance axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveUpdateClass {
    LiveApplyNoReview,
    LiveApplyWithRevertableCheckpoint,
    ConfirmReviewRequired,
    PolicyBlocked,
}

/// OS signal class mapped to a policy axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OsSignalClass {
    OsThemeSignal,
    OsContrastSignal,
    OsAccentSignal,
    OsDensitySignal,
    OsTextScaleSignal,
    OsReducedMotionSignal,
    OsForcedColorsSignal,
    None,
}

/// Policy lock reason applied to a live follow-system axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyLockReasonClass {
    NotLocked,
    ManagedPolicyCap,
    RestrictedWorkspaceCap,
    CriticalHotPathCap,
    PlatformUnavailable,
}

/// Surface scope that owns a follow-system policy row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceScopeClass {
    GlobalAppearance,
    ProfileAppearance,
    WorkspaceAppearance,
    ExtensionSurfaceAppearance,
}

/// One live-follow-system policy axis row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveAxisRow {
    pub axis: AppearanceAxis,
    pub live_update_class: LiveUpdateClass,
    pub os_signal_class: OsSignalClass,
    pub requires_checkpoint: bool,
    pub requires_user_confirm: bool,
    pub policy_lock_reason_class: PolicyLockReasonClass,
    pub surface_scope: SurfaceScopeClass,
    pub notes: Option<String>,
}

/// Persisted live follow-system policy record cited by [`AppearanceSessionRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveFollowSystemPolicyRecord {
    pub record_kind: LiveFollowSystemPolicyRecordKind,
    pub appearance_session_schema_version: u32,
    pub live_follow_system_policy_id: String,
    pub appearance_session_ref: String,
    pub axes: Vec<LiveAxisRow>,
    pub policy_context: PolicyContext,
    pub redaction_class: RedactionClass,
    pub minted_at: String,
}

impl LiveFollowSystemPolicyRecord {
    /// Returns a stable default live-follow-system policy record for first-party shells.
    pub fn first_party_default(appearance_session_ref: String, minted_at: String) -> Self {
        Self {
            record_kind: LiveFollowSystemPolicyRecordKind::LiveFollowSystemPolicyRecord,
            appearance_session_schema_version: 1,
            live_follow_system_policy_id: "live_follow_system_policy:shell.local:01".to_string(),
            appearance_session_ref,
            axes: vec![
                LiveAxisRow {
                    axis: AppearanceAxis::ModeThemeClass,
                    live_update_class: LiveUpdateClass::LiveApplyNoReview,
                    os_signal_class: OsSignalClass::OsThemeSignal,
                    requires_checkpoint: false,
                    requires_user_confirm: false,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: Some("Theme-class flips apply live on supported platforms.".to_string()),
                },
                LiveAxisRow {
                    axis: AppearanceAxis::ContrastMode,
                    live_update_class: LiveUpdateClass::LiveApplyWithRevertableCheckpoint,
                    os_signal_class: OsSignalClass::OsContrastSignal,
                    requires_checkpoint: true,
                    requires_user_confirm: false,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: None,
                },
                LiveAxisRow {
                    axis: AppearanceAxis::AccentSource,
                    live_update_class: LiveUpdateClass::LiveApplyNoReview,
                    os_signal_class: OsSignalClass::OsAccentSignal,
                    requires_checkpoint: false,
                    requires_user_confirm: false,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: None,
                },
                LiveAxisRow {
                    axis: AppearanceAxis::DensityClass,
                    live_update_class: LiveUpdateClass::ConfirmReviewRequired,
                    os_signal_class: OsSignalClass::OsDensitySignal,
                    requires_checkpoint: true,
                    requires_user_confirm: true,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: None,
                },
                LiveAxisRow {
                    axis: AppearanceAxis::TextScale,
                    live_update_class: LiveUpdateClass::ConfirmReviewRequired,
                    os_signal_class: OsSignalClass::OsTextScaleSignal,
                    requires_checkpoint: true,
                    requires_user_confirm: true,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: None,
                },
                LiveAxisRow {
                    axis: AppearanceAxis::ReducedMotionPosture,
                    live_update_class: LiveUpdateClass::LiveApplyWithRevertableCheckpoint,
                    os_signal_class: OsSignalClass::OsReducedMotionSignal,
                    requires_checkpoint: true,
                    requires_user_confirm: false,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: None,
                },
                LiveAxisRow {
                    axis: AppearanceAxis::FollowSystemPosture,
                    live_update_class: LiveUpdateClass::ConfirmReviewRequired,
                    os_signal_class: OsSignalClass::None,
                    requires_checkpoint: true,
                    requires_user_confirm: true,
                    policy_lock_reason_class: PolicyLockReasonClass::NotLocked,
                    surface_scope: SurfaceScopeClass::GlobalAppearance,
                    notes: None,
                },
            ],
            policy_context: PolicyContext {
                policy_epoch: "pe:2026-04-29:01".to_string(),
                trust_state: TrustState::Trusted,
                execution_context_id: None,
            },
            redaction_class: RedactionClass::MetadataSafeDefault,
            minted_at,
        }
    }
}

fn contrast_mode_for_theme(theme: ThemeClass) -> ContrastMode {
    match theme {
        ThemeClass::HighContrastDark | ThemeClass::HighContrastLight => ContrastMode::ContrastHigh,
        _ => ContrastMode::ContrastStandard,
    }
}
