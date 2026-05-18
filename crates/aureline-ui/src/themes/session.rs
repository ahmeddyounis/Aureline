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

impl AccessibilityPostureClass {
    /// Returns the canonical posture token identifier.
    pub const fn token(self) -> &'static str {
        match self {
            Self::MotionStandard => "motion_standard",
            Self::MotionReduced => "motion_reduced",
            Self::MotionLowMotion => "motion_low_motion",
            Self::MotionPowerSaver => "motion_power_saver",
            Self::MotionCriticalHotPath => "motion_critical_hot_path",
        }
    }
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

/// Identifies the `appearance_session_revision_event_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceSessionRevisionEventRecordKind {
    /// `appearance_session_revision_event_record`
    AppearanceSessionRevisionEventRecord,
}

/// Cause class for a governed appearance-session revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CauseClass {
    /// The operating system emitted an appearance signal.
    OsSignalChange,
    /// The user explicitly changed an appearance setting.
    UserExplicitSetting,
    /// Managed policy changed the effective appearance state.
    PolicyChange,
    /// An imported appearance package was applied.
    ImportApply,
    /// An imported appearance package was reverted.
    ImportRevert,
    /// A token overlay was applied.
    OverlayApply,
    /// A token overlay was reverted.
    OverlayRevert,
    /// A preview checkpoint began.
    PreviewStart,
    /// A preview checkpoint was committed.
    PreviewCommit,
    /// A preview checkpoint was reverted.
    PreviewRevert,
    /// A checkpoint rollback restored earlier appearance state.
    CheckpointRollback,
    /// Power-saver mode engaged a more restrictive motion posture.
    PowerSaverEngaged,
    /// The renderer engaged the critical-hot-path motion posture.
    CriticalHotPathEngaged,
}

/// Event emitted after one atomic appearance-session revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSessionRevisionEvent {
    record_kind: AppearanceSessionRevisionEventRecordKind,
    appearance_session_schema_version: u32,
    event_id: String,
    appearance_session_ref: String,
    prior_session_revision: u64,
    resulting_session_revision: u64,
    cause_class: CauseClass,
    os_signal_ref: Option<String>,
    confirm_action_ref: Option<String>,
    checkpoint_ref: Option<String>,
    rollback_ref: Option<String>,
    changed_axes: Vec<AppearanceAxis>,
    applied_under_live_class: LiveUpdateClass,
    notes: Option<String>,
    policy_context: PolicyContext,
    redaction_class: RedactionClass,
    recorded_at: String,
}

impl AppearanceSessionRevisionEvent {
    /// Returns the stable event identifier.
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Returns the prior session revision.
    pub const fn prior_session_revision(&self) -> u64 {
        self.prior_session_revision
    }

    /// Returns the resulting session revision.
    pub const fn resulting_session_revision(&self) -> u64 {
        self.resulting_session_revision
    }

    /// Returns the event cause class.
    pub const fn cause_class(&self) -> CauseClass {
        self.cause_class
    }

    /// Returns the changed appearance axes.
    pub fn changed_axes(&self) -> &[AppearanceAxis] {
        &self.changed_axes
    }
}

/// Revertable checkpoint captured before an appearance-session mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppearanceSessionCheckpoint {
    checkpoint_ref: String,
    rollback_ref: String,
    prior_session: AppearanceSessionRecord,
}

impl AppearanceSessionCheckpoint {
    /// Returns the checkpoint reference.
    pub fn checkpoint_ref(&self) -> &str {
        &self.checkpoint_ref
    }

    /// Returns the rollback reference.
    pub fn rollback_ref(&self) -> &str {
        &self.rollback_ref
    }

    /// Returns the session revision captured by the checkpoint.
    pub const fn prior_session_revision(&self) -> u64 {
        self.prior_session.session_revision
    }
}

/// Batched appearance changes applied as a single session revision.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppearanceChangeSet {
    theme_package_refs: Option<(String, String)>,
    theme_class: Option<ThemeClass>,
    contrast_mode: Option<ContrastMode>,
    accent_source: Option<AccentSourceClass>,
    density_class: Option<DensityClass>,
    text_scale: Option<TextScale>,
    reduced_motion: Option<(AccessibilityPostureClass, ReducedMotionSource)>,
    follow_system_posture: Option<FollowSystemPosture>,
    token_overlay_ref: Option<Option<String>>,
    active_import_report_refs: Option<Vec<String>>,
    confirm_action_ref: Option<Option<String>>,
}

impl AppearanceChangeSet {
    /// Returns an empty change set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the active theme package and revision refs.
    pub fn theme_package_refs(
        mut self,
        package_ref: impl Into<String>,
        revision_ref: impl Into<String>,
    ) -> Self {
        self.theme_package_refs = Some((package_ref.into(), revision_ref.into()));
        self
    }

    /// Sets the resolved theme class.
    pub const fn theme_class(mut self, theme_class: ThemeClass) -> Self {
        self.theme_class = Some(theme_class);
        self
    }

    /// Sets the resolved contrast mode.
    pub const fn contrast_mode(mut self, contrast_mode: ContrastMode) -> Self {
        self.contrast_mode = Some(contrast_mode);
        self
    }

    /// Sets the resolved accent source.
    pub const fn accent_source(mut self, accent_source: AccentSourceClass) -> Self {
        self.accent_source = Some(accent_source);
        self
    }

    /// Sets the resolved density class.
    pub const fn density_class(mut self, density_class: DensityClass) -> Self {
        self.density_class = Some(density_class);
        self
    }

    /// Sets the resolved text scale.
    pub const fn text_scale(mut self, text_scale: TextScale) -> Self {
        self.text_scale = Some(text_scale);
        self
    }

    /// Sets the reduced-motion posture and source.
    pub const fn reduced_motion(
        mut self,
        posture: AccessibilityPostureClass,
        source: ReducedMotionSource,
    ) -> Self {
        self.reduced_motion = Some((posture, source));
        self
    }

    /// Sets the follow-system posture.
    pub const fn follow_system_posture(mut self, posture: FollowSystemPosture) -> Self {
        self.follow_system_posture = Some(posture);
        self
    }

    /// Sets or clears the token overlay ref.
    pub fn token_overlay_ref(mut self, token_overlay_ref: Option<String>) -> Self {
        self.token_overlay_ref = Some(token_overlay_ref);
        self
    }

    /// Replaces the active import-report refs.
    pub fn active_import_report_refs(mut self, refs: Vec<String>) -> Self {
        self.active_import_report_refs = Some(refs);
        self
    }

    /// Sets or clears the confirm action ref associated with the change.
    pub fn confirm_action_ref(mut self, confirm_action_ref: Option<String>) -> Self {
        self.confirm_action_ref = Some(confirm_action_ref);
        self
    }

    fn apply_to(&self, session: &mut AppearanceSessionRecord) -> Vec<AppearanceAxis> {
        let mut changed_axes = Vec::new();
        if let Some((package_ref, revision_ref)) = &self.theme_package_refs {
            if session.active_theme_package_ref != *package_ref
                || session.active_theme_revision_ref != *revision_ref
            {
                session.active_theme_package_ref = package_ref.clone();
                session.active_theme_revision_ref = revision_ref.clone();
                push_axis_once(&mut changed_axes, AppearanceAxis::ModeThemeClass);
            }
        }
        if let Some(theme_class) = self.theme_class {
            if session.mode_theme_class != theme_class {
                session.mode_theme_class = theme_class;
                session.contrast_mode = contrast_mode_for_theme(theme_class);
                push_axis_once(&mut changed_axes, AppearanceAxis::ModeThemeClass);
                push_axis_once(&mut changed_axes, AppearanceAxis::ContrastMode);
            }
        }
        if let Some(contrast_mode) = self.contrast_mode {
            if session.contrast_mode != contrast_mode {
                session.contrast_mode = contrast_mode;
                push_axis_once(&mut changed_axes, AppearanceAxis::ContrastMode);
            }
        }
        if let Some(accent_source) = self.accent_source {
            if session.accent_source != accent_source {
                session.accent_source = accent_source;
                push_axis_once(&mut changed_axes, AppearanceAxis::AccentSource);
            }
        }
        if let Some(density_class) = self.density_class {
            if session.density_class != density_class {
                session.density_class = density_class;
                push_axis_once(&mut changed_axes, AppearanceAxis::DensityClass);
            }
        }
        if let Some(text_scale) = self.text_scale {
            if session.text_scale != text_scale {
                session.text_scale = text_scale;
                push_axis_once(&mut changed_axes, AppearanceAxis::TextScale);
            }
        }
        if let Some((posture, source)) = self.reduced_motion {
            if session.reduced_motion_posture != posture || session.reduced_motion_source != source
            {
                session.reduced_motion_posture = posture;
                session.reduced_motion_source = source;
                push_axis_once(&mut changed_axes, AppearanceAxis::ReducedMotionPosture);
            }
        }
        if let Some(posture) = self.follow_system_posture {
            if session.follow_system_posture != posture {
                session.follow_system_posture = posture;
                push_axis_once(&mut changed_axes, AppearanceAxis::FollowSystemPosture);
            }
        }
        if let Some(token_overlay_ref) = &self.token_overlay_ref {
            session.token_overlay_ref = token_overlay_ref.clone();
        }
        if let Some(refs) = &self.active_import_report_refs {
            session.active_import_report_refs = refs.clone();
        }
        if let Some(confirm_action_ref) = &self.confirm_action_ref {
            session.confirm_action_ref = confirm_action_ref.clone();
        }
        changed_axes
    }
}

/// Error returned when a checkpointed appearance mutation cannot be applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppearanceAtomicApplyError {
    /// The checkpoint belongs to a different appearance session.
    SessionMismatch,
    /// The checkpoint was captured for an earlier revision than the current session.
    StaleCheckpoint {
        checkpoint_revision: u64,
        current_revision: u64,
    },
    /// The live policy required user confirmation but no confirm action was recorded.
    MissingConfirmAction,
    /// The proposed change set did not alter any appearance axis.
    NoChangedAxes,
}

impl std::fmt::Display for AppearanceAtomicApplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SessionMismatch => write!(f, "appearance checkpoint belongs to another session"),
            Self::StaleCheckpoint {
                checkpoint_revision,
                current_revision,
            } => write!(
                f,
                "appearance checkpoint revision {checkpoint_revision} is stale for current revision {current_revision}"
            ),
            Self::MissingConfirmAction => write!(
                f,
                "confirm-required appearance change is missing a confirm action ref"
            ),
            Self::NoChangedAxes => write!(f, "appearance change set did not change any axis"),
        }
    }
}

impl std::error::Error for AppearanceAtomicApplyError {}

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
            live_follow_system_policy_ref: "live_follow_system_policy:shell.local:01".to_string(),
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

    /// Captures a revertable checkpoint for the current session revision.
    pub fn create_checkpoint(
        &self,
        checkpoint_ref: impl Into<String>,
        rollback_ref: impl Into<String>,
    ) -> AppearanceSessionCheckpoint {
        AppearanceSessionCheckpoint {
            checkpoint_ref: checkpoint_ref.into(),
            rollback_ref: rollback_ref.into(),
            prior_session: self.clone(),
        }
    }

    /// Applies a group of appearance changes as one checkpointed session revision.
    ///
    /// # Errors
    ///
    /// Returns an error when the checkpoint belongs to another session, when the
    /// checkpoint is stale, or when the change set does not alter an appearance
    /// axis.
    pub fn apply_checkpointed_changes(
        &mut self,
        changes: AppearanceChangeSet,
        checkpoint: &AppearanceSessionCheckpoint,
        cause_class: CauseClass,
        applied_under_live_class: LiveUpdateClass,
        minted_at: String,
    ) -> Result<AppearanceSessionRevisionEvent, AppearanceAtomicApplyError> {
        if checkpoint.prior_session.appearance_session_id != self.appearance_session_id {
            return Err(AppearanceAtomicApplyError::SessionMismatch);
        }
        if checkpoint.prior_session.session_revision != self.session_revision {
            return Err(AppearanceAtomicApplyError::StaleCheckpoint {
                checkpoint_revision: checkpoint.prior_session.session_revision,
                current_revision: self.session_revision,
            });
        }

        let mut next = self.clone();
        let changed_axes = changes.apply_to(&mut next);
        if changed_axes.is_empty() {
            return Err(AppearanceAtomicApplyError::NoChangedAxes);
        }
        if applied_under_live_class == LiveUpdateClass::ConfirmReviewRequired
            && next.confirm_action_ref.is_none()
        {
            return Err(AppearanceAtomicApplyError::MissingConfirmAction);
        }

        let prior_revision = self.session_revision;
        next.session_revision = prior_revision.saturating_add(1);
        next.current_checkpoint_ref = Some(checkpoint.checkpoint_ref.clone());
        next.rollback_ref = Some(checkpoint.rollback_ref.clone());
        next.preview_state = match cause_class {
            CauseClass::PreviewStart => PreviewState::PreviewLive,
            CauseClass::PreviewCommit => PreviewState::PreviewCommitted,
            CauseClass::PreviewRevert => PreviewState::PreviewFailedReverted,
            _ => next.preview_state,
        };
        next.revision_minted_at = minted_at.clone();

        let event = next.revision_event(RevisionEventInput {
            prior_revision,
            cause_class,
            os_signal_ref: None,
            confirm_action_ref: next.confirm_action_ref.clone(),
            checkpoint_ref: Some(checkpoint.checkpoint_ref.clone()),
            rollback_ref: Some(checkpoint.rollback_ref.clone()),
            changed_axes,
            applied_under_live_class,
            recorded_at: minted_at,
        });
        *self = next;
        Ok(event)
    }

    /// Restores a prior checkpoint as a new rollback-applied revision.
    ///
    /// # Errors
    ///
    /// Returns an error when the checkpoint belongs to another session or the
    /// current state already matches the checkpointed appearance axes.
    pub fn revert_to_checkpoint(
        &mut self,
        checkpoint: &AppearanceSessionCheckpoint,
        minted_at: String,
    ) -> Result<AppearanceSessionRevisionEvent, AppearanceAtomicApplyError> {
        if checkpoint.prior_session.appearance_session_id != self.appearance_session_id {
            return Err(AppearanceAtomicApplyError::SessionMismatch);
        }

        let changed_axes = changed_axes_between(self, &checkpoint.prior_session);
        if changed_axes.is_empty() {
            return Err(AppearanceAtomicApplyError::NoChangedAxes);
        }

        let prior_revision = self.session_revision;
        let mut restored = checkpoint.prior_session.clone();
        restored.session_revision = prior_revision.saturating_add(1);
        restored.preview_state = PreviewState::RollbackApplied;
        restored.current_checkpoint_ref = Some(checkpoint.checkpoint_ref.clone());
        restored.rollback_ref = Some(checkpoint.rollback_ref.clone());
        restored.revision_minted_at = minted_at.clone();

        let event = restored.revision_event(RevisionEventInput {
            prior_revision,
            cause_class: CauseClass::CheckpointRollback,
            os_signal_ref: None,
            confirm_action_ref: None,
            checkpoint_ref: Some(checkpoint.checkpoint_ref.clone()),
            rollback_ref: Some(checkpoint.rollback_ref.clone()),
            changed_axes,
            applied_under_live_class: LiveUpdateClass::LiveApplyWithRevertableCheckpoint,
            recorded_at: minted_at,
        });
        *self = restored;
        Ok(event)
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

    /// Applies a reduced-motion posture change, bumping the session revision.
    pub fn apply_reduced_motion_posture(
        &mut self,
        posture: AccessibilityPostureClass,
        source: ReducedMotionSource,
        minted_at: String,
    ) {
        if self.reduced_motion_posture == posture && self.reduced_motion_source == source {
            self.revision_minted_at = minted_at;
            return;
        }
        self.session_revision = self.session_revision.saturating_add(1);
        self.reduced_motion_posture = posture;
        self.reduced_motion_source = source;
        self.follow_system_posture = FollowSystemPosture::ManualOverride;
        self.preview_state = PreviewState::NotPreviewing;
        self.current_checkpoint_ref = None;
        self.rollback_ref = None;
        self.revision_minted_at = minted_at;
    }

    /// Cycles the reduced-motion posture (`standard` → `reduced` → `low_motion`).
    pub fn cycle_reduced_motion_posture(&mut self, minted_at: String) {
        let (next_posture, next_source) = match self.reduced_motion_posture {
            AccessibilityPostureClass::MotionStandard => (
                AccessibilityPostureClass::MotionReduced,
                ReducedMotionSource::UserSetting,
            ),
            AccessibilityPostureClass::MotionReduced
            | AccessibilityPostureClass::MotionPowerSaver => (
                AccessibilityPostureClass::MotionLowMotion,
                ReducedMotionSource::UserSetting,
            ),
            AccessibilityPostureClass::MotionLowMotion
            | AccessibilityPostureClass::MotionCriticalHotPath => (
                AccessibilityPostureClass::MotionStandard,
                ReducedMotionSource::UserSetting,
            ),
        };
        self.apply_reduced_motion_posture(next_posture, next_source, minted_at);
    }

    fn revision_event(&self, input: RevisionEventInput) -> AppearanceSessionRevisionEvent {
        AppearanceSessionRevisionEvent {
            record_kind:
                AppearanceSessionRevisionEventRecordKind::AppearanceSessionRevisionEventRecord,
            appearance_session_schema_version: self.appearance_session_schema_version,
            event_id: format!(
                "appearance_session_event:{}:{}",
                self.appearance_session_id, self.session_revision
            ),
            appearance_session_ref: self.appearance_session_id.clone(),
            prior_session_revision: input.prior_revision,
            resulting_session_revision: self.session_revision,
            cause_class: input.cause_class,
            os_signal_ref: input.os_signal_ref,
            confirm_action_ref: input.confirm_action_ref,
            checkpoint_ref: input.checkpoint_ref,
            rollback_ref: input.rollback_ref,
            changed_axes: input.changed_axes,
            applied_under_live_class: input.applied_under_live_class,
            notes: None,
            policy_context: self.policy_context.clone(),
            redaction_class: self.redaction_class,
            recorded_at: input.recorded_at,
        }
    }
}

struct RevisionEventInput {
    prior_revision: u64,
    cause_class: CauseClass,
    os_signal_ref: Option<String>,
    confirm_action_ref: Option<String>,
    checkpoint_ref: Option<String>,
    rollback_ref: Option<String>,
    changed_axes: Vec<AppearanceAxis>,
    applied_under_live_class: LiveUpdateClass,
    recorded_at: String,
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
                    live_update_class: LiveUpdateClass::LiveApplyWithRevertableCheckpoint,
                    os_signal_class: OsSignalClass::OsDensitySignal,
                    requires_checkpoint: true,
                    requires_user_confirm: false,
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

fn push_axis_once(axes: &mut Vec<AppearanceAxis>, axis: AppearanceAxis) {
    if !axes.contains(&axis) {
        axes.push(axis);
    }
}

fn changed_axes_between(
    before: &AppearanceSessionRecord,
    after: &AppearanceSessionRecord,
) -> Vec<AppearanceAxis> {
    let mut axes = Vec::new();
    if before.mode_theme_class != after.mode_theme_class
        || before.active_theme_package_ref != after.active_theme_package_ref
        || before.active_theme_revision_ref != after.active_theme_revision_ref
    {
        push_axis_once(&mut axes, AppearanceAxis::ModeThemeClass);
    }
    if before.contrast_mode != after.contrast_mode {
        push_axis_once(&mut axes, AppearanceAxis::ContrastMode);
    }
    if before.accent_source != after.accent_source {
        push_axis_once(&mut axes, AppearanceAxis::AccentSource);
    }
    if before.density_class != after.density_class {
        push_axis_once(&mut axes, AppearanceAxis::DensityClass);
    }
    if before.text_scale != after.text_scale {
        push_axis_once(&mut axes, AppearanceAxis::TextScale);
    }
    if before.reduced_motion_posture != after.reduced_motion_posture
        || before.reduced_motion_source != after.reduced_motion_source
    {
        push_axis_once(&mut axes, AppearanceAxis::ReducedMotionPosture);
    }
    if before.follow_system_posture != after.follow_system_posture {
        push_axis_once(&mut axes, AppearanceAxis::FollowSystemPosture);
    }
    axes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> AppearanceSessionRecord {
        AppearanceSessionRecord::first_party_default("2026-05-13T00:00:00Z".to_string())
    }

    #[test]
    fn checkpointed_changes_apply_as_one_revision_event() {
        let mut session = session();
        let checkpoint = session.create_checkpoint(
            "appearance_checkpoint:test:01",
            "appearance_rollback:test:01",
        );
        let event = session
            .apply_checkpointed_changes(
                AppearanceChangeSet::new()
                    .theme_class(ThemeClass::LightParity)
                    .density_class(DensityClass::Compact)
                    .reduced_motion(
                        AccessibilityPostureClass::MotionReduced,
                        ReducedMotionSource::UserSetting,
                    )
                    .confirm_action_ref(Some("action:appearance.confirm_theme_switch".to_string())),
                &checkpoint,
                CauseClass::UserExplicitSetting,
                LiveUpdateClass::ConfirmReviewRequired,
                "2026-05-13T00:01:00Z".to_string(),
            )
            .expect("checkpointed apply");

        assert_eq!(session.session_revision, 1);
        assert_eq!(session.mode_theme_class, ThemeClass::LightParity);
        assert_eq!(session.density_class, DensityClass::Compact);
        assert_eq!(
            session.reduced_motion_posture,
            AccessibilityPostureClass::MotionReduced
        );
        assert_eq!(
            session.current_checkpoint_ref.as_deref(),
            Some("appearance_checkpoint:test:01")
        );
        assert_eq!(event.prior_session_revision(), 0);
        assert_eq!(event.resulting_session_revision(), 1);
        assert!(event
            .changed_axes()
            .contains(&AppearanceAxis::ModeThemeClass));
        assert!(event.changed_axes().contains(&AppearanceAxis::DensityClass));
        assert!(event
            .changed_axes()
            .contains(&AppearanceAxis::ReducedMotionPosture));
    }

    #[test]
    fn checkpoint_revert_restores_prior_appearance_as_new_revision() {
        let mut session = session();
        let checkpoint = session.create_checkpoint(
            "appearance_checkpoint:test:02",
            "appearance_rollback:test:02",
        );
        session
            .apply_checkpointed_changes(
                AppearanceChangeSet::new().theme_class(ThemeClass::HighContrastLight),
                &checkpoint,
                CauseClass::PreviewStart,
                LiveUpdateClass::LiveApplyWithRevertableCheckpoint,
                "2026-05-13T00:01:00Z".to_string(),
            )
            .expect("checkpointed apply");

        let event = session
            .revert_to_checkpoint(&checkpoint, "2026-05-13T00:02:00Z".to_string())
            .expect("checkpoint revert");

        assert_eq!(session.session_revision, 2);
        assert_eq!(session.mode_theme_class, ThemeClass::DarkReference);
        assert_eq!(session.preview_state, PreviewState::RollbackApplied);
        assert_eq!(event.cause_class(), CauseClass::CheckpointRollback);
        assert_eq!(event.prior_session_revision(), 1);
        assert_eq!(event.resulting_session_revision(), 2);
    }

    #[test]
    fn stale_checkpoint_refuses_without_mutating_session() {
        let mut session = session();
        let checkpoint = session.create_checkpoint(
            "appearance_checkpoint:test:03",
            "appearance_rollback:test:03",
        );
        session.apply_density_class(DensityClass::Compact, "2026-05-13T00:01:00Z".to_string());
        let before = session.clone();

        let err = session
            .apply_checkpointed_changes(
                AppearanceChangeSet::new()
                    .theme_class(ThemeClass::LightParity)
                    .confirm_action_ref(Some("action:appearance.confirm_theme_switch".to_string())),
                &checkpoint,
                CauseClass::UserExplicitSetting,
                LiveUpdateClass::ConfirmReviewRequired,
                "2026-05-13T00:02:00Z".to_string(),
            )
            .expect_err("stale checkpoint should fail");

        assert_eq!(
            err,
            AppearanceAtomicApplyError::StaleCheckpoint {
                checkpoint_revision: 0,
                current_revision: 1,
            }
        );
        assert_eq!(session, before);
    }

    #[test]
    fn confirm_required_apply_refuses_missing_confirm_action() {
        let mut session = session();
        let checkpoint = session.create_checkpoint(
            "appearance_checkpoint:test:04",
            "appearance_rollback:test:04",
        );
        let before = session.clone();

        let err = session
            .apply_checkpointed_changes(
                AppearanceChangeSet::new().theme_class(ThemeClass::LightParity),
                &checkpoint,
                CauseClass::UserExplicitSetting,
                LiveUpdateClass::ConfirmReviewRequired,
                "2026-05-13T00:02:00Z".to_string(),
            )
            .expect_err("missing confirm action should fail");

        assert_eq!(err, AppearanceAtomicApplyError::MissingConfirmAction);
        assert_eq!(session, before);
    }
}
