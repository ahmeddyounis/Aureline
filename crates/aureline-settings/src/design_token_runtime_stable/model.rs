//! Canonical stable truth model for the **design-token runtime certification**:
//! mode conformance, non-color cue survival, live-apply honesty, motion
//! suppression, and launch-surface conformance across dark, light,
//! high-contrast, reduced-motion, and density rows.
//!
//! ## Why one governed certification record
//!
//! The design-token runtime is consumed by every launch-critical shell surface.
//! If each surface improvises colors, density, or motion, then a theme or
//! contrast change can silently break a focus ring, a state badge, a severity
//! cue, or a keyboard-visible affordance — and the screenshots in the release
//! packet might be captured from a different appearance state than the one that
//! actually ships. The risk this closes: a green "theme parity" claim that is
//! really an average over surfaces that each diverge a little, with evidence
//! that cannot be tied back to one runtime state.
//!
//! A [`DesignTokenRuntimeCertification`] mints, for one appearance posture:
//!
//! - **One appearance-session value** — the binding records the active
//!   appearance-session id and revision, and every mode row's golden capture and
//!   accessibility packet must cite that same value, so captured artifacts and
//!   runtime inspection use one source of truth.
//! - **Mode conformance** — one [`AppearanceModeRow`] per dark, light,
//!   high-contrast (dark and light), reduced-motion, and density mode, each
//!   proving the semantic token registry resolves for the row's theme and that
//!   focus rings, state badges, severity cues, and keyboard affordances survive
//!   the mode change.
//! - **Non-color cue survival** — one [`ProtectedCueRow`] per diagnostics,
//!   policy lock, trust warning, execution target, selection, and focus cue,
//!   each proving the cue carries a non-color carrier (text, shape, border,
//!   icon, or focus ring) that survives high-contrast, forced-colors, and
//!   reduced-motion modes.
//! - **Live-apply honesty** — one [`LiveApplyAxisRow`] per appearance axis,
//!   declaring whether an OS change applies live, applies live behind a
//!   checkpoint, requires confirmation, or requires a disclosed reload/restart;
//!   a reload/restart row must disclose, and no row may silently lag the system.
//! - **Motion suppression in the runtime** — one [`MotionSuppressionRow`] per
//!   posture, proving non-essential motion suppression is modeled in the token
//!   runtime's motion presets rather than improvised per surface.
//! - **Launch-surface conformance** — one [`LaunchSurfaceRow`] per
//!   launch-critical shell surface, proving it honors the semantic token runtime
//!   with no hard-coded styling, or narrowing the claim if it cannot yet.
//! - **A public claim ceiling** and **automatic narrowing** — a posture that
//!   cannot prove a pillar, or whose lowest surface marker is below Stable,
//!   narrows below Stable with a named reason instead of inheriting an adjacent
//!   green row.
//!
//! The mode conformance, the token resolution, and the motion presets are **not**
//! reinvented here: every record is a genuine projection of the live appearance
//! runtime assembled in [`crate::design_token_runtime_stable::corpus`].

use std::collections::BTreeSet;

use aureline_ui::density::DensityClass;
use aureline_ui::themes::{
    AccentSourceClass, AccessibilityPostureClass, AppearanceAxis, ContrastMode, FollowSystemPosture,
};
use aureline_ui::tokens::ThemeClass;
use serde::{Deserialize, Serialize};

pub use aureline_design_system::LaunchSurfaceClass;

/// Stable record-kind tag carried in serialized certification records.
pub const DESIGN_TOKEN_RUNTIME_RECORD_KIND: &str = "design_token_runtime_certification_record";

/// Schema version for the [`DesignTokenRuntimeCertification`] payload shape.
pub const DESIGN_TOKEN_RUNTIME_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const DESIGN_TOKEN_RUNTIME_SHARED_CONTRACT_REF: &str =
    "settings:design_token_runtime_stable:v1";

/// Reviewer-facing notice rendered on every certification surface.
pub const DESIGN_TOKEN_RUNTIME_NOTICE: &str =
    "Design-token runtime certification: every launch-critical shell surface renders from one \
     semantic token runtime; dark, light, high-contrast, reduced-motion, and density rows each \
     prove the token registry resolves and that focus rings, state badges, severity cues, and \
     keyboard affordances survive the mode change; diagnostics, policy locks, trust warnings, \
     execution targets, selection, and focus never rely on hue alone but carry a text, shape, \
     border, icon, or focus-ring cue that survives contrast and motion modes; every appearance \
     axis declares whether an OS change applies live, applies live behind a checkpoint, requires \
     confirmation, or requires a disclosed reload/restart, never a silent lag; reduced-motion and \
     power-saving suppression is modeled in the token runtime rather than improvised per surface; \
     golden captures and accessibility review packets are attributable to one appearance-session \
     value so screenshots and shipped behavior use one source of truth; and a posture that cannot \
     prove a pillar, or whose lowest surface marker is below Stable, narrows below Stable with a \
     named reason rather than inheriting an adjacent green row.";

/// Upper bound on a reviewable explanation sentence.
const MAX_SENTENCE_CHARS: usize = 1024;
/// Upper bound on a present ref.
const MAX_REF_CHARS: usize = 200;
/// Canonical durable-object scheme used by minted refs.
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";
/// Ref classes that are generic landing targets, not certification objects.
const GENERIC_LANDING_CLASSES: [&str; 3] = ["home", "landing", "root"];

// ---------------------------------------------------------------------------
// Shared governance vocabulary
// ---------------------------------------------------------------------------

/// Public claim class for the lane, reusing the stable lifecycle cutline.
///
/// `Stable` sits at or above the launch cutline; everything else is narrowed
/// below it. The builder *derives* this from the evidence, so a posture can
/// never publish a claim wider than its proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The design-token runtime is replacement-grade across the claimed rows.
    Stable,
    /// Narrowed to the beta promise.
    Beta,
    /// Narrowed to the preview / limited-availability promise.
    Preview,
    /// No public promise yet.
    NotClaimed,
}

impl StableClaimClass {
    /// Returns the stable string vocabulary for this claim class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::NotClaimed => "not_claimed",
        }
    }
}

/// Lifecycle marker carried by an appearance row or a launch surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMarker {
    /// Preview / limited-availability.
    Preview,
    /// Beta promise.
    Beta,
    /// Replacement-grade stable.
    Stable,
}

impl LifecycleMarker {
    /// Returns the stable string vocabulary for this marker.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Returns `true` when the marker sits below the stable cutline.
    pub const fn is_below_stable(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Surface a certification can be reached from. The same record must be
/// reachable from all four so keyboard-only and assistive-technology users find
/// it consistently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteSurface {
    /// The settings appearance panel — the authoritative surface.
    SettingsAppearance,
    /// The command palette.
    CommandPalette,
    /// The status bar / status overflow.
    StatusBar,
    /// An application menu command.
    MenuCommand,
}

impl RouteSurface {
    /// Returns the stable string vocabulary for this route surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsAppearance => "settings_appearance",
            Self::CommandPalette => "command_palette",
            Self::StatusBar => "status_bar",
            Self::MenuCommand => "menu_command",
        }
    }

    /// The four surfaces that must all be able to reach a record.
    pub const REQUIRED: [Self; 4] = [
        Self::SettingsAppearance,
        Self::CommandPalette,
        Self::StatusBar,
        Self::MenuCommand,
    ];
}

/// Layout mode an accessibility disclosure is checked under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutMode {
    /// Default desktop layout.
    Normal,
    /// High-contrast theme.
    HighContrast,
    /// Zoomed / enlarged layout.
    Zoomed,
}

impl LayoutMode {
    /// Returns the stable string vocabulary for this layout mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::HighContrast => "high_contrast",
            Self::Zoomed => "zoomed",
        }
    }

    /// The three layout modes every disclosure must hold in.
    pub const REQUIRED: [Self; 3] = [Self::Normal, Self::HighContrast, Self::Zoomed];
}

/// Role a recovery action plays, used for placement and confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionRole {
    /// Opens or focuses the authoritative appearance settings.
    Primary,
    /// Inspects or recovers the appearance runtime state.
    Recovery,
    /// Non-mutating inspect / export.
    Secondary,
}

impl RecoveryActionRole {
    /// Returns the stable string vocabulary for this role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Recovery => "recovery",
            Self::Secondary => "secondary",
        }
    }
}

/// One recovery route exposed on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRouteRecord {
    /// Stable action id from the canonical recovery vocabulary.
    pub action_id: String,
    /// Compact label rendered in rows and narrated by assistive tech.
    pub action_label: String,
    /// Placement / confirmation role.
    pub action_role: RecoveryActionRole,
    /// Whether the action is keyboard reachable.
    pub keyboard_reachable: bool,
}

/// One route to the same record from one entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryRouteRecord {
    /// Surface that exposes the route.
    pub surface: RouteSurface,
    /// Canonical route ref pointing at the record on this surface.
    pub route_ref: String,
    /// Whether the route is keyboard reachable.
    pub keyboard_reachable: bool,
    /// Whether the route activates the same certification record.
    pub activates_same_record: bool,
}

/// Accessibility disclosure for one layout mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutModeDisclosure {
    /// Layout mode this disclosure was checked under.
    pub mode: LayoutMode,
    /// Whether the row narration is available in this mode.
    pub row_narration_available: bool,
    /// Whether the recovery affordances stay reachable in this mode.
    pub recovery_affordances_reachable: bool,
}

/// Accessibility disclosure for the record across the required layout modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityDisclosure {
    /// Position of the record in the surface tab order.
    pub focus_order_index: u32,
    /// Number of keyboard tab stops the record and its actions expose.
    pub tab_stop_count: u32,
    /// Record narration read by assistive tech.
    pub row_narration: String,
    /// Action labels in rendered order, narrated by assistive technology.
    pub action_labels: Vec<String>,
    /// Per-layout-mode disclosures for normal, high-contrast, and zoomed.
    pub layout_modes: Vec<LayoutModeDisclosure>,
}

/// Returns true when `reference` is a canonical object ref of the form
/// `aureline://<class>/<id>` where `<class>` is not a generic landing page.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    if class.is_empty() || ident.is_empty() {
        return false;
    }
    !GENERIC_LANDING_CLASSES.contains(&class)
}

/// Compact snake_case token for any of the upstream enums, derived through serde
/// so this record never maintains a parallel vocabulary.
pub fn snake_token<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(str::to_owned))
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Appearance-session binding
// ---------------------------------------------------------------------------

/// The single appearance-session value every capture is attributed to.
///
/// Golden captures and accessibility packets cite [`Self::value_ref`]; the
/// builder rejects any mode row whose capture is attributed to a different
/// value, so screenshots and shipped behavior provably use one source of truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSessionBinding {
    /// Active appearance-session id.
    pub appearance_session_id: String,
    /// Active appearance-session revision.
    pub session_revision: u64,
    /// Active theme-package ref.
    pub active_theme_package_ref: String,
    /// Active theme-revision ref.
    pub active_theme_revision_ref: String,
    /// Resolved theme class.
    pub mode_theme_class: ThemeClass,
    /// Resolved contrast mode.
    pub contrast_mode: ContrastMode,
    /// Resolved accent source.
    pub accent_source: AccentSourceClass,
    /// Resolved density class.
    pub density_class: DensityClass,
    /// Resolved text-scale percent.
    pub text_scale_percent: u32,
    /// Resolved reduced-motion posture.
    pub reduced_motion_posture: AccessibilityPostureClass,
    /// Resolved follow-system posture.
    pub follow_system_posture: FollowSystemPosture,
    /// Live follow-system policy ref cited by the session.
    pub live_follow_system_policy_ref: String,
    /// Canonical value ref every capture must cite.
    pub value_ref: String,
}

impl AppearanceSessionBinding {
    /// Builds the canonical value ref for an appearance-session id and revision.
    pub fn value_ref_for(appearance_session_id: &str, session_revision: u64) -> String {
        format!("aureline://appearance-session/{appearance_session_id}@rev{session_revision}")
    }
}

// ---------------------------------------------------------------------------
// Appearance mode rows
// ---------------------------------------------------------------------------

/// Canonical appearance mode certified by the lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppearanceModeClass {
    /// Dark reference theme.
    Dark,
    /// Light parity theme.
    Light,
    /// High-contrast dark theme.
    HighContrastDark,
    /// High-contrast light theme.
    HighContrastLight,
    /// Reduced-motion posture row.
    ReducedMotion,
    /// Density variant row.
    Density,
}

impl AppearanceModeClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::HighContrastDark => "high_contrast_dark",
            Self::HighContrastLight => "high_contrast_light",
            Self::ReducedMotion => "reduced_motion",
            Self::Density => "density",
        }
    }

    /// The closed required mode set in canonical order.
    pub const REQUIRED: [Self; 6] = [
        Self::Dark,
        Self::Light,
        Self::HighContrastDark,
        Self::HighContrastLight,
        Self::ReducedMotion,
        Self::Density,
    ];
}

/// One certified appearance mode row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceModeRow {
    /// Mode class.
    pub mode_class: AppearanceModeClass,
    /// Theme class the capture was rendered with.
    pub theme_class: ThemeClass,
    /// Density class the capture was rendered with.
    pub density_class: DensityClass,
    /// Motion posture the capture was rendered with.
    pub motion_posture: AccessibilityPostureClass,
    /// Whether the semantic token registry resolves for this row's theme.
    pub token_registry_resolves: bool,
    /// Semantic tokens proven present in this row's theme.
    pub certified_token_refs: Vec<String>,
    /// Whether the focus ring is preserved under the mode change.
    pub focus_ring_preserved: bool,
    /// Whether state badges are preserved under the mode change.
    pub state_badges_preserved: bool,
    /// Whether severity cues are preserved under the mode change.
    pub severity_cues_preserved: bool,
    /// Whether keyboard-visible affordances are preserved under the mode change.
    pub keyboard_affordances_preserved: bool,
    /// Whether protected non-color cues survive the mode change.
    pub protected_cues_survive: bool,
    /// Golden capture ref attributable to the appearance-session value.
    pub golden_capture_ref: String,
    /// Accessibility review packet ref attributable to the same value.
    pub accessibility_packet_ref: String,
    /// Appearance-session value ref this capture is attributed to.
    pub appearance_session_value_ref: String,
    /// Row surface lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Derived: the row conforms to the stable design-token runtime.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Protected cues
// ---------------------------------------------------------------------------

/// Cue families that must never rely on hue alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedCueClass {
    /// Diagnostics severity (error/warning/info).
    Diagnostics,
    /// Policy / managed lock.
    PolicyLock,
    /// Trust / permission warning.
    TrustWarning,
    /// Execution target (local / remote / container).
    ExecutionTarget,
    /// Selection state.
    Selection,
    /// Focus state.
    Focus,
}

impl ProtectedCueClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagnostics => "diagnostics",
            Self::PolicyLock => "policy_lock",
            Self::TrustWarning => "trust_warning",
            Self::ExecutionTarget => "execution_target",
            Self::Selection => "selection",
            Self::Focus => "focus",
        }
    }

    /// The closed required protected-cue set in canonical order.
    pub const REQUIRED: [Self; 6] = [
        Self::Diagnostics,
        Self::PolicyLock,
        Self::TrustWarning,
        Self::ExecutionTarget,
        Self::Selection,
        Self::Focus,
    ];
}

/// Non-color carrier a protected cue may use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonColorCueClass {
    /// Visible label text.
    LabelText,
    /// Icon or glyph metaphor.
    Icon,
    /// Border treatment.
    Border,
    /// Shape differentiation.
    Shape,
    /// Focus ring.
    FocusRing,
}

impl NonColorCueClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabelText => "label_text",
            Self::Icon => "icon",
            Self::Border => "border",
            Self::Shape => "shape",
            Self::FocusRing => "focus_ring",
        }
    }
}

/// One protected cue row proving non-color survival.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedCueRow {
    /// Cue class.
    pub cue_class: ProtectedCueClass,
    /// Non-color carriers the cue uses, sorted.
    pub non_color_cues: Vec<NonColorCueClass>,
    /// Whether hue-only meaning is forbidden for this cue.
    pub hue_only_forbidden: bool,
    /// Whether the cue survives high-contrast modes.
    pub survives_high_contrast: bool,
    /// Whether the cue survives forced-colors mode.
    pub survives_forced_colors: bool,
    /// Whether the cue survives reduced-motion postures.
    pub survives_reduced_motion: bool,
    /// Derived: the cue never relies on hue alone and survives every mode.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Live-apply axes
// ---------------------------------------------------------------------------

/// How an appearance axis responds to an OS / system change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveApplyClass {
    /// Applies live with no review.
    ApplyLive,
    /// Applies live behind a revertable checkpoint.
    ApplyLiveCheckpointed,
    /// Held for explicit user confirmation.
    ConfirmRequired,
    /// Requires a disclosed surface reload.
    ReloadRequired,
    /// Requires a disclosed application restart.
    RestartRequired,
    /// Blocked by policy.
    PolicyBlocked,
}

impl LiveApplyClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApplyLive => "apply_live",
            Self::ApplyLiveCheckpointed => "apply_live_checkpointed",
            Self::ConfirmRequired => "confirm_required",
            Self::ReloadRequired => "reload_required",
            Self::RestartRequired => "restart_required",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// Whether this class defers application behind a reload or restart.
    pub const fn requires_reload_or_restart(self) -> bool {
        matches!(self, Self::ReloadRequired | Self::RestartRequired)
    }
}

/// One appearance axis' live-apply posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveApplyAxisRow {
    /// Appearance axis.
    pub axis: AppearanceAxis,
    /// Live-apply class for this axis.
    pub live_apply_class: LiveApplyClass,
    /// Whether a reload/restart is disclosed to the user (required for those).
    pub disclosure_required: bool,
    /// Whether the axis silently lags the system state (must be false).
    pub silently_lags_system: bool,
    /// Reviewer-facing note.
    pub note: String,
}

// ---------------------------------------------------------------------------
// Motion suppression
// ---------------------------------------------------------------------------

/// One posture's motion-suppression certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionSuppressionRow {
    /// Motion posture.
    pub posture: AccessibilityPostureClass,
    /// Whether suppression is modeled in the token runtime's motion presets.
    pub modeled_in_token_runtime: bool,
    /// Whether non-essential motion is suppressed for this posture.
    pub non_essential_motion_suppressed: bool,
    /// Substitution class the runtime resolves (e.g. crossfade_only), or "none".
    pub substitution_class: String,
    /// Whether per-surface improvisation is absent (the runtime owns the rule).
    pub per_surface_improvisation_absent: bool,
}

// ---------------------------------------------------------------------------
// Launch surfaces
// ---------------------------------------------------------------------------

/// One launch-critical shell surface's token-runtime conformance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchSurfaceRow {
    /// Launch-critical surface class.
    pub surface_class: LaunchSurfaceClass,
    /// Whether the surface renders from the semantic token runtime.
    pub honors_token_runtime: bool,
    /// Whether hard-coded colors/density/motion are absent on this surface.
    pub hardcoded_styling_absent: bool,
    /// Surface lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Mode classes this surface is certified under, sorted.
    pub certified_modes: Vec<AppearanceModeClass>,
    /// Optional bounded waiver ref for a surface narrowed below Stable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the surface conforms (honors runtime, no hard-coded styling, Stable).
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Pillars, claim ceiling, qualification, upstream
// ---------------------------------------------------------------------------

/// The derived pillar verdicts (what the posture can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPillars {
    /// Every required mode row conforms.
    pub all_modes_conform: bool,
    /// Every protected cue carries a non-color carrier that survives modes.
    pub protected_cues_never_hue_only: bool,
    /// Every capture is attributable to one appearance-session value.
    pub captures_share_one_session: bool,
    /// Every appearance axis applies live or discloses a reload/restart.
    pub live_apply_no_silent_lag: bool,
    /// Motion suppression is modeled in the token runtime.
    pub motion_suppression_in_runtime: bool,
    /// No launch-critical Stable surface hard-codes styling.
    pub no_hardcoded_stable_styling: bool,
}

/// The public claim ceiling: what a posture is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationClaimCeiling {
    /// May claim every required mode conforms.
    pub asserts_all_modes_conform: bool,
    /// May claim protected cues never rely on hue alone.
    pub asserts_protected_cues_non_color: bool,
    /// May claim every capture is attributable to one appearance-session value.
    pub asserts_one_appearance_session: bool,
    /// May claim live apply never silently lags the system.
    pub asserts_live_apply_no_silent_lag: bool,
    /// May claim motion suppression is modeled in the token runtime.
    pub asserts_motion_suppression_runtime: bool,
    /// May claim no launch-critical Stable surface hard-codes styling.
    pub asserts_no_hardcoded_styling: bool,
}

/// Reason a posture is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowingReason {
    /// A required mode row does not conform.
    ModeConformanceNotProven,
    /// A protected cue could rely on hue alone or not survive a mode.
    ProtectedCueHueOnlyRisk,
    /// Captures are not attributable to one appearance-session value.
    CapturesNotOneSession,
    /// An appearance axis silently lags the system or hides a reload/restart.
    LiveApplySilentLag,
    /// Motion suppression is not modeled in the token runtime.
    MotionSuppressionNotInRuntime,
    /// A launch-critical Stable surface hard-codes styling.
    HardcodedStableStyling,
    /// The lowest surface marker is below Stable, so the posture must not inherit
    /// Stable by adjacency.
    SurfaceNotYetStable,
}

impl CertificationNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeConformanceNotProven => "mode_conformance_not_proven",
            Self::ProtectedCueHueOnlyRisk => "protected_cue_hue_only_risk",
            Self::CapturesNotOneSession => "captures_not_one_session",
            Self::LiveApplySilentLag => "live_apply_silent_lag",
            Self::MotionSuppressionNotInRuntime => "motion_suppression_not_in_runtime",
            Self::HardcodedStableStyling => "hardcoded_stable_styling",
            Self::SurfaceNotYetStable => "surface_not_yet_stable",
        }
    }
}

/// The derived stable-claim verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationQualification {
    /// The derived claim class.
    pub claim_class: StableClaimClass,
    /// Whether the posture qualifies at or above the launch cutline.
    pub qualifies_stable: bool,
    /// Reasons the posture is narrowed below Stable, in canonical order.
    pub narrowing_reasons: Vec<CertificationNarrowingReason>,
}

/// Upstream ids the record is a genuine projection of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationUpstream {
    /// Design-system appearance-session beta contract packet id.
    pub appearance_contract_ref: String,
    /// Design-system component-state registry id.
    pub component_state_registry_ref: String,
    /// Theme token-registry refs scanned, sorted and deduped.
    pub token_registry_refs: Vec<String>,
    /// Capture refs that contributed to this posture, sorted and deduped.
    pub contributing_capture_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`DesignTokenRuntimeCertification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token (the snapshot scenario).
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The single appearance-session value captures attribute to.
    pub appearance_session: AppearanceSessionBinding,
    /// Mode conformance rows.
    pub mode_rows: Vec<AppearanceModeRow>,
    /// Protected cue rows.
    pub protected_cues: Vec<ProtectedCueRow>,
    /// Live-apply axis rows.
    pub live_apply_axes: Vec<LiveApplyAxisRow>,
    /// Motion-suppression rows.
    pub motion_suppression: Vec<MotionSuppressionRow>,
    /// Launch-surface conformance rows.
    pub launch_surfaces: Vec<LaunchSurfaceRow>,
    /// Public claim ceiling.
    pub claim_ceiling: CertificationClaimCeiling,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the record.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the record stays available without an account.
    pub available_without_account: bool,
    /// Whether the record stays available without managed services.
    pub available_without_managed_services: bool,
    /// Upstream ids the record projects from.
    pub upstream: CertificationUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// The canonical, governed design-token runtime certification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignTokenRuntimeCertification {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp.
    pub as_of: String,
    /// Stable posture token.
    pub posture_id: String,
    /// Compact posture label.
    pub posture_label: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Reviewer-facing summary.
    pub summary: String,
    /// The single appearance-session value.
    pub appearance_session: AppearanceSessionBinding,
    /// The lowest surface marker — the record's overall surface marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Mode conformance rows, in canonical mode order.
    pub mode_rows: Vec<AppearanceModeRow>,
    /// Protected cue rows, in canonical cue order.
    pub protected_cues: Vec<ProtectedCueRow>,
    /// Live-apply axis rows, in canonical axis order.
    pub live_apply_axes: Vec<LiveApplyAxisRow>,
    /// Motion-suppression rows, in canonical posture order.
    pub motion_suppression: Vec<MotionSuppressionRow>,
    /// Launch-surface conformance rows, in canonical surface order.
    pub launch_surfaces: Vec<LaunchSurfaceRow>,
    /// The derived pillar verdicts.
    pub pillars: CertificationPillars,
    /// The public claim ceiling.
    pub claim_ceiling: CertificationClaimCeiling,
    /// The derived stable-claim verdict.
    pub stable_qualification: CertificationQualification,
    /// Recovery routes in rendered order.
    pub recovery_routes: Vec<RecoveryRouteRecord>,
    /// Per-surface entry routes to the record.
    pub routes: Vec<EntryRouteRecord>,
    /// Accessibility disclosure across required layout modes.
    pub accessibility: AccessibilityDisclosure,
    /// Whether the record stays available without an account.
    pub available_without_account: bool,
    /// Whether the record stays available without managed services.
    pub available_without_managed_services: bool,
    /// True when there is anything narrowed or below-stable to disclose.
    pub honesty_marker_present: bool,
    /// Upstream ids the record projects from.
    pub upstream: CertificationUpstream,
    /// Canonical diagnostics-export ref.
    pub diagnostics_export_ref: String,
    /// Canonical support-export ref.
    pub support_export_ref: String,
    /// Canonical evidence refs.
    pub evidence_refs: Vec<String>,
    /// Canonical narrative refs.
    pub narrative_refs: Vec<String>,
}

/// Reasons a [`DesignTokenRuntimeCertification`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// A required appearance mode row was missing.
    ModeRowMissing { mode: AppearanceModeClass },
    /// An appearance mode row was duplicated.
    DuplicateModeRow { mode: AppearanceModeClass },
    /// A mode row's capture was attributed to a different appearance-session
    /// value than the binding.
    CaptureSessionMismatch { mode: AppearanceModeClass },
    /// A required protected cue row was missing.
    ProtectedCueMissing { cue: ProtectedCueClass },
    /// A protected cue row was duplicated.
    DuplicateProtectedCue { cue: ProtectedCueClass },
    /// A required appearance axis row was missing.
    AxisRowMissing { axis: AppearanceAxis },
    /// An appearance axis row was duplicated.
    DuplicateAxisRow { axis: AppearanceAxis },
    /// A reload/restart axis row did not disclose, or an axis silently lags.
    SilentLiveApplyLag { axis: AppearanceAxis },
    /// A required motion posture row was missing.
    MotionRowMissing { posture: AccessibilityPostureClass },
    /// A required launch-critical surface row was missing.
    SurfaceRowMissing { surface: LaunchSurfaceClass },
    /// A launch surface narrowed below Stable without a bounded waiver.
    SurfaceNarrowedWithoutWaiver { surface: LaunchSurfaceClass },
    /// The claim ceiling asserted all-modes-conform it cannot prove.
    OverclaimsAllModesConform,
    /// The claim ceiling asserted protected-cue non-color survival it cannot prove.
    OverclaimsProtectedCues,
    /// The claim ceiling asserted one-appearance-session it cannot prove.
    OverclaimsOneSession,
    /// The claim ceiling asserted live-apply-no-silent-lag it cannot prove.
    OverclaimsLiveApply,
    /// The claim ceiling asserted motion-suppression-runtime it cannot prove.
    OverclaimsMotionSuppression,
    /// The claim ceiling asserted no-hardcoded-styling it cannot prove.
    OverclaimsNoHardcodedStyling,
    /// A required recovery route was missing.
    MissingRecoveryRoute { action: CertificationRecoveryAction },
    /// A recovery route was not keyboard reachable.
    RecoveryRouteNotKeyboardReachable { action_id: String },
    /// A required entry-route surface was missing.
    RouteSurfaceMissing { surface: RouteSurface },
    /// An entry-route surface was duplicated.
    DuplicateRouteSurface { surface: RouteSurface },
    /// An entry route was not keyboard reachable.
    RouteNotKeyboardReachable { surface: RouteSurface },
    /// An entry route did not activate the same record.
    RouteTargetsDifferentRecord { surface: RouteSurface },
    /// A required accessibility layout mode was missing.
    AccessibilityLayoutModeMissing { mode: LayoutMode },
    /// An accessibility layout mode was unreachable or lost narration.
    AccessibilityLayoutModeUnreachable { mode: LayoutMode },
    /// The accessibility action labels did not match the recovery routes.
    AccessibilityActionLabelsMismatch,
    /// The record was hidden when no account was present.
    HiddenWithoutAccount,
    /// The record was hidden when managed services were absent.
    HiddenWithoutManagedServices,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSentence { field } => {
                write!(f, "field `{field}` must be a non-empty reviewable sentence")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(
                    f,
                    "field `{field}` must be a canonical object ref, got {value:?}"
                )
            }
            Self::MissingRef { field } => write!(f, "ref `{field}` must be present"),
            Self::ModeRowMissing { mode } => {
                write!(f, "appearance mode row `{}` is missing", mode.as_str())
            }
            Self::DuplicateModeRow { mode } => {
                write!(f, "appearance mode row `{}` is duplicated", mode.as_str())
            }
            Self::CaptureSessionMismatch { mode } => write!(
                f,
                "mode `{}` capture is attributed to a different appearance-session value than the \
                 binding; screenshots and runtime would not use one source of truth",
                mode.as_str()
            ),
            Self::ProtectedCueMissing { cue } => {
                write!(f, "protected cue row `{}` is missing", cue.as_str())
            }
            Self::DuplicateProtectedCue { cue } => {
                write!(f, "protected cue row `{}` is duplicated", cue.as_str())
            }
            Self::AxisRowMissing { axis } => {
                write!(f, "appearance axis row `{}` is missing", snake_token(axis))
            }
            Self::DuplicateAxisRow { axis } => {
                write!(f, "appearance axis row `{}` is duplicated", snake_token(axis))
            }
            Self::SilentLiveApplyLag { axis } => write!(
                f,
                "axis `{}` must disclose a reload/restart and may not silently lag the system",
                snake_token(axis)
            ),
            Self::MotionRowMissing { posture } => {
                write!(f, "motion posture row `{}` is missing", posture.token())
            }
            Self::SurfaceRowMissing { surface } => {
                write!(
                    f,
                    "launch-critical surface row `{}` is missing",
                    surface.as_str()
                )
            }
            Self::SurfaceNarrowedWithoutWaiver { surface } => write!(
                f,
                "launch surface `{}` is narrowed below Stable but carries no bounded waiver ref",
                surface.as_str()
            ),
            Self::OverclaimsAllModesConform => write!(
                f,
                "claim ceiling may not assert all-modes-conform when a mode row does not conform"
            ),
            Self::OverclaimsProtectedCues => write!(
                f,
                "claim ceiling may not assert protected-cue non-color survival it cannot prove"
            ),
            Self::OverclaimsOneSession => write!(
                f,
                "claim ceiling may not assert one appearance session when captures diverge"
            ),
            Self::OverclaimsLiveApply => write!(
                f,
                "claim ceiling may not assert live-apply honesty when an axis lags or hides a reload"
            ),
            Self::OverclaimsMotionSuppression => write!(
                f,
                "claim ceiling may not assert runtime motion suppression it cannot prove"
            ),
            Self::OverclaimsNoHardcodedStyling => write!(
                f,
                "claim ceiling may not assert no hard-coded styling when a Stable surface diverges"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(
                    f,
                    "record must expose recovery route `{}`",
                    action.as_str()
                )
            }
            Self::RecoveryRouteNotKeyboardReachable { action_id } => {
                write!(f, "recovery route `{action_id}` must be keyboard reachable")
            }
            Self::RouteSurfaceMissing { surface } => {
                write!(f, "entry route surface `{}` is missing", surface.as_str())
            }
            Self::DuplicateRouteSurface { surface } => {
                write!(
                    f,
                    "entry route surface `{}` is duplicated",
                    surface.as_str()
                )
            }
            Self::RouteNotKeyboardReachable { surface } => write!(
                f,
                "entry route surface `{}` must be keyboard reachable",
                surface.as_str()
            ),
            Self::RouteTargetsDifferentRecord { surface } => write!(
                f,
                "entry route surface `{}` must activate the same certification record",
                surface.as_str()
            ),
            Self::AccessibilityLayoutModeMissing { mode } => {
                write!(
                    f,
                    "accessibility layout mode `{}` is missing",
                    mode.as_str()
                )
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::HiddenWithoutAccount => {
                write!(
                    f,
                    "a design-token runtime certification must stay available without an account"
                )
            }
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a design-token runtime certification must stay available without managed services"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

fn is_reviewable_sentence(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_SENTENCE_CHARS
}

fn is_present_ref(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed.len() <= MAX_REF_CHARS
}

fn require_canonical_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_string(),
        })
    }
}

fn require_present_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_present_ref(value) {
        Ok(())
    } else {
        Err(BuildError::MissingRef { field })
    }
}

fn required_motion_postures() -> [AccessibilityPostureClass; 5] {
    [
        AccessibilityPostureClass::MotionStandard,
        AccessibilityPostureClass::MotionReduced,
        AccessibilityPostureClass::MotionLowMotion,
        AccessibilityPostureClass::MotionPowerSaver,
        AccessibilityPostureClass::MotionCriticalHotPath,
    ]
}

fn required_axes() -> [AppearanceAxis; 7] {
    [
        AppearanceAxis::ModeThemeClass,
        AppearanceAxis::ContrastMode,
        AppearanceAxis::AccentSource,
        AppearanceAxis::DensityClass,
        AppearanceAxis::TextScale,
        AppearanceAxis::ReducedMotionPosture,
        AppearanceAxis::FollowSystemPosture,
    ]
}

impl DesignTokenRuntimeCertification {
    /// Builds a governed certification record from validated input.
    ///
    /// The pillar verdicts are *derived* from the mode, cue, axis, motion, and
    /// surface rows, so a record can never publish a claim wider than its proof.
    /// Structural lies (a capture attributed to a different appearance session, a
    /// reload that does not disclose, a missing required row) are rejected
    /// outright; provable-but-imperfect postures (a mode that does not conform, a
    /// below-Stable surface) are minted but narrowed below Stable with a named
    /// reason.
    pub fn build(input: CertificationInput) -> Result<Self, BuildError> {
        // --- text / ref validation -------------------------------------------
        for (field, value) in [
            ("title", &input.title),
            ("summary", &input.summary),
            ("posture_label", &input.posture_label),
        ] {
            if !is_reviewable_sentence(value) {
                return Err(BuildError::InvalidSentence { field });
            }
        }
        require_canonical_ref("diagnostics_export_ref", &input.diagnostics_export_ref)?;
        require_canonical_ref("support_export_ref", &input.support_export_ref)?;
        for evidence in &input.evidence_refs {
            require_canonical_ref("evidence_refs", evidence)?;
        }
        for narrative in &input.narrative_refs {
            require_canonical_ref("narrative_refs", narrative)?;
        }
        require_canonical_ref(
            "appearance_session.value_ref",
            &input.appearance_session.value_ref,
        )?;
        require_present_ref(
            "upstream.appearance_contract_ref",
            &input.upstream.appearance_contract_ref,
        )?;
        require_present_ref(
            "upstream.component_state_registry_ref",
            &input.upstream.component_state_registry_ref,
        )?;

        let session_value_ref = input.appearance_session.value_ref.clone();

        // --- mode rows -------------------------------------------------------
        let mut seen_modes: BTreeSet<AppearanceModeClass> = BTreeSet::new();
        for row in &input.mode_rows {
            if !seen_modes.insert(row.mode_class) {
                return Err(BuildError::DuplicateModeRow {
                    mode: row.mode_class,
                });
            }
            require_canonical_ref("mode_rows.golden_capture_ref", &row.golden_capture_ref)?;
            require_canonical_ref(
                "mode_rows.accessibility_packet_ref",
                &row.accessibility_packet_ref,
            )?;
            if row.appearance_session_value_ref != session_value_ref {
                return Err(BuildError::CaptureSessionMismatch {
                    mode: row.mode_class,
                });
            }
        }
        for required in AppearanceModeClass::REQUIRED {
            if !seen_modes.contains(&required) {
                return Err(BuildError::ModeRowMissing { mode: required });
            }
        }
        let mut mode_rows: Vec<AppearanceModeRow> = input.mode_rows.clone();
        mode_rows.sort_by_key(|row| row.mode_class);
        for row in &mut mode_rows {
            row.certified_token_refs.sort();
            row.certified_token_refs.dedup();
            row.conforms = row.token_registry_resolves
                && row.focus_ring_preserved
                && row.state_badges_preserved
                && row.severity_cues_preserved
                && row.keyboard_affordances_preserved
                && row.protected_cues_survive;
        }
        let all_modes_conform = mode_rows.iter().all(|row| row.conforms);
        // Captures share one session by construction (mismatches hard-errored).
        let captures_share_one_session = true;

        // --- protected cues --------------------------------------------------
        let mut seen_cues: BTreeSet<ProtectedCueClass> = BTreeSet::new();
        for row in &input.protected_cues {
            if !seen_cues.insert(row.cue_class) {
                return Err(BuildError::DuplicateProtectedCue { cue: row.cue_class });
            }
        }
        for required in ProtectedCueClass::REQUIRED {
            if !seen_cues.contains(&required) {
                return Err(BuildError::ProtectedCueMissing { cue: required });
            }
        }
        let mut protected_cues: Vec<ProtectedCueRow> = input.protected_cues.clone();
        protected_cues.sort_by_key(|row| row.cue_class);
        for row in &mut protected_cues {
            row.non_color_cues.sort();
            row.non_color_cues.dedup();
            row.conforms = !row.non_color_cues.is_empty()
                && row.hue_only_forbidden
                && row.survives_high_contrast
                && row.survives_forced_colors
                && row.survives_reduced_motion;
        }
        let protected_cues_never_hue_only = protected_cues.iter().all(|row| row.conforms);

        // --- live-apply axes -------------------------------------------------
        let mut seen_axes: Vec<AppearanceAxis> = Vec::new();
        for row in &input.live_apply_axes {
            if seen_axes.contains(&row.axis) {
                return Err(BuildError::DuplicateAxisRow { axis: row.axis });
            }
            seen_axes.push(row.axis);
            if !is_reviewable_sentence(&row.note) {
                return Err(BuildError::InvalidSentence {
                    field: "live_apply_axes.note",
                });
            }
            if row.silently_lags_system
                || (row.live_apply_class.requires_reload_or_restart() && !row.disclosure_required)
            {
                return Err(BuildError::SilentLiveApplyLag { axis: row.axis });
            }
        }
        for required in required_axes() {
            if !seen_axes.contains(&required) {
                return Err(BuildError::AxisRowMissing { axis: required });
            }
        }
        let mut live_apply_axes: Vec<LiveApplyAxisRow> = input.live_apply_axes.clone();
        live_apply_axes.sort_by_key(|row| axis_order(row.axis));
        // No silent lag by construction (lags hard-errored).
        let live_apply_no_silent_lag = true;

        // --- motion suppression ----------------------------------------------
        let present_postures: Vec<AccessibilityPostureClass> = input
            .motion_suppression
            .iter()
            .map(|row| row.posture)
            .collect();
        for posture in required_motion_postures() {
            if !present_postures.contains(&posture) {
                return Err(BuildError::MotionRowMissing { posture });
            }
        }
        let mut motion_suppression: Vec<MotionSuppressionRow> = input.motion_suppression.clone();
        motion_suppression.sort_by_key(|row| posture_order(row.posture));
        let motion_suppression_in_runtime = motion_suppression.iter().all(|row| {
            row.modeled_in_token_runtime
                && row.per_surface_improvisation_absent
                && (row.posture == AccessibilityPostureClass::MotionStandard
                    || row.non_essential_motion_suppressed)
        });

        // --- launch surfaces -------------------------------------------------
        let mut seen_surfaces: BTreeSet<LaunchSurfaceClass> = BTreeSet::new();
        for row in &input.launch_surfaces {
            seen_surfaces.insert(row.surface_class);
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("launch_surfaces.waiver_ref", waiver)?;
            }
        }
        for required in LaunchSurfaceClass::required() {
            if !seen_surfaces.contains(required) {
                return Err(BuildError::SurfaceRowMissing { surface: *required });
            }
        }
        let mut launch_surfaces: Vec<LaunchSurfaceRow> = input.launch_surfaces.clone();
        launch_surfaces.sort_by_key(|row| surface_order(row.surface_class));
        for row in &mut launch_surfaces {
            row.certified_modes.sort();
            row.certified_modes.dedup();
            row.conforms = row.honors_token_runtime
                && row.hardcoded_styling_absent
                && !row.surface_marker.is_below_stable();
            // A surface narrowed below Stable must carry a bounded waiver.
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::SurfaceNarrowedWithoutWaiver {
                    surface: row.surface_class,
                });
            }
        }
        let no_hardcoded_stable_styling = launch_surfaces
            .iter()
            .all(|row| row.honors_token_runtime && row.hardcoded_styling_absent);

        // --- derive pillars --------------------------------------------------
        let pillars = CertificationPillars {
            all_modes_conform,
            protected_cues_never_hue_only,
            captures_share_one_session,
            live_apply_no_silent_lag,
            motion_suppression_in_runtime,
            no_hardcoded_stable_styling,
        };

        // --- claim ceiling: never claim what cannot be proven ----------------
        if input.claim_ceiling.asserts_all_modes_conform && !all_modes_conform {
            return Err(BuildError::OverclaimsAllModesConform);
        }
        if input.claim_ceiling.asserts_protected_cues_non_color && !protected_cues_never_hue_only {
            return Err(BuildError::OverclaimsProtectedCues);
        }
        if input.claim_ceiling.asserts_one_appearance_session && !captures_share_one_session {
            return Err(BuildError::OverclaimsOneSession);
        }
        if input.claim_ceiling.asserts_live_apply_no_silent_lag && !live_apply_no_silent_lag {
            return Err(BuildError::OverclaimsLiveApply);
        }
        if input.claim_ceiling.asserts_motion_suppression_runtime && !motion_suppression_in_runtime
        {
            return Err(BuildError::OverclaimsMotionSuppression);
        }
        if input.claim_ceiling.asserts_no_hardcoded_styling && !no_hardcoded_stable_styling {
            return Err(BuildError::OverclaimsNoHardcodedStyling);
        }

        // --- recovery routes -------------------------------------------------
        let route_ids: Vec<&str> = input
            .recovery_routes
            .iter()
            .map(|route| route.action_id.as_str())
            .collect();
        for required in CertificationRecoveryAction::REQUIRED {
            if !route_ids.iter().any(|id| *id == required.as_str()) {
                return Err(BuildError::MissingRecoveryRoute { action: required });
            }
        }
        for route in &input.recovery_routes {
            if !route.keyboard_reachable {
                return Err(BuildError::RecoveryRouteNotKeyboardReachable {
                    action_id: route.action_id.clone(),
                });
            }
        }

        // --- entry routes ----------------------------------------------------
        let mut seen_route_surfaces: Vec<RouteSurface> = Vec::new();
        for route in &input.routes {
            if seen_route_surfaces.contains(&route.surface) {
                return Err(BuildError::DuplicateRouteSurface {
                    surface: route.surface,
                });
            }
            seen_route_surfaces.push(route.surface);
            require_canonical_ref("routes.route_ref", &route.route_ref)?;
            if !route.keyboard_reachable {
                return Err(BuildError::RouteNotKeyboardReachable {
                    surface: route.surface,
                });
            }
            if !route.activates_same_record {
                return Err(BuildError::RouteTargetsDifferentRecord {
                    surface: route.surface,
                });
            }
        }
        for required in RouteSurface::REQUIRED {
            if !seen_route_surfaces.contains(&required) {
                return Err(BuildError::RouteSurfaceMissing { surface: required });
            }
        }

        // --- accessibility ---------------------------------------------------
        if input.accessibility.action_labels.len() != input.recovery_routes.len() {
            return Err(BuildError::AccessibilityActionLabelsMismatch);
        }
        for (label, route) in input
            .accessibility
            .action_labels
            .iter()
            .zip(input.recovery_routes.iter())
        {
            if label != &route.action_label {
                return Err(BuildError::AccessibilityActionLabelsMismatch);
            }
        }
        for required in LayoutMode::REQUIRED {
            let Some(disclosure) = input
                .accessibility
                .layout_modes
                .iter()
                .find(|mode| mode.mode == required)
            else {
                return Err(BuildError::AccessibilityLayoutModeMissing { mode: required });
            };
            if !disclosure.row_narration_available || !disclosure.recovery_affordances_reachable {
                return Err(BuildError::AccessibilityLayoutModeUnreachable { mode: required });
            }
        }

        // --- availability ----------------------------------------------------
        if !input.available_without_account {
            return Err(BuildError::HiddenWithoutAccount);
        }
        if !input.available_without_managed_services {
            return Err(BuildError::HiddenWithoutManagedServices);
        }

        // --- surface marker = lowest among mode + surface markers ------------
        let surface_lifecycle_marker = mode_rows
            .iter()
            .map(|row| row.surface_marker)
            .chain(launch_surfaces.iter().map(|row| row.surface_marker))
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !all_modes_conform {
            narrowing_reasons.push(CertificationNarrowingReason::ModeConformanceNotProven);
        }
        if !protected_cues_never_hue_only {
            narrowing_reasons.push(CertificationNarrowingReason::ProtectedCueHueOnlyRisk);
        }
        if !captures_share_one_session {
            narrowing_reasons.push(CertificationNarrowingReason::CapturesNotOneSession);
        }
        if !live_apply_no_silent_lag {
            narrowing_reasons.push(CertificationNarrowingReason::LiveApplySilentLag);
        }
        if !motion_suppression_in_runtime {
            narrowing_reasons.push(CertificationNarrowingReason::MotionSuppressionNotInRuntime);
        }
        if !no_hardcoded_stable_styling {
            narrowing_reasons.push(CertificationNarrowingReason::HardcodedStableStyling);
        }
        if surface_lifecycle_marker.is_below_stable() {
            narrowing_reasons.push(CertificationNarrowingReason::SurfaceNotYetStable);
        }
        let qualifies_stable = narrowing_reasons.is_empty();
        let claim_class = if qualifies_stable {
            StableClaimClass::Stable
        } else if narrowing_reasons.len() == 1
            && narrowing_reasons[0] == CertificationNarrowingReason::SurfaceNotYetStable
        {
            match surface_lifecycle_marker {
                LifecycleMarker::Preview => StableClaimClass::Preview,
                _ => StableClaimClass::Beta,
            }
        } else {
            StableClaimClass::Beta
        };
        let stable_qualification = CertificationQualification {
            claim_class,
            qualifies_stable,
            narrowing_reasons,
        };
        let honesty_marker_present =
            !qualifies_stable || surface_lifecycle_marker.is_below_stable();

        // --- normalise upstream refs -----------------------------------------
        let mut token_registry_refs = input.upstream.token_registry_refs.clone();
        token_registry_refs.sort();
        token_registry_refs.dedup();
        let mut contributing_capture_refs = input.upstream.contributing_capture_refs.clone();
        contributing_capture_refs.sort();
        contributing_capture_refs.dedup();

        Ok(Self {
            record_kind: DESIGN_TOKEN_RUNTIME_RECORD_KIND.to_string(),
            schema_version: DESIGN_TOKEN_RUNTIME_SCHEMA_VERSION,
            notice: DESIGN_TOKEN_RUNTIME_NOTICE.to_string(),
            shared_contract_ref: DESIGN_TOKEN_RUNTIME_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            appearance_session: input.appearance_session,
            surface_lifecycle_marker,
            mode_rows,
            protected_cues,
            live_apply_axes,
            motion_suppression,
            launch_surfaces,
            pillars,
            claim_ceiling: input.claim_ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: CertificationUpstream {
                appearance_contract_ref: input.upstream.appearance_contract_ref,
                component_state_registry_ref: input.upstream.component_state_registry_ref,
                token_registry_refs,
                contributing_capture_refs,
            },
            diagnostics_export_ref: input.diagnostics_export_ref,
            support_export_ref: input.support_export_ref,
            evidence_refs: input.evidence_refs,
            narrative_refs: input.narrative_refs,
        })
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("design_token_runtime_certification: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "appearance_session: id={} rev={} theme={} contrast={} density={} text_scale={}% motion={} value_ref={}",
                self.appearance_session.appearance_session_id,
                self.appearance_session.session_revision,
                snake_token(&self.appearance_session.mode_theme_class),
                snake_token(&self.appearance_session.contrast_mode),
                snake_token(&self.appearance_session.density_class),
                self.appearance_session.text_scale_percent,
                self.appearance_session.reduced_motion_posture.token(),
                self.appearance_session.value_ref
            ),
            format!(
                "stable_qualification: class={} qualifies_stable={} narrowing=[{}]",
                self.stable_qualification.claim_class.as_str(),
                self.stable_qualification.qualifies_stable,
                self.stable_qualification
                    .narrowing_reasons
                    .iter()
                    .map(|reason| reason.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "pillars: all_modes_conform={} protected_cues_non_color={} one_session={} live_apply_no_lag={} motion_runtime={} no_hardcoded={}",
                self.pillars.all_modes_conform,
                self.pillars.protected_cues_never_hue_only,
                self.pillars.captures_share_one_session,
                self.pillars.live_apply_no_silent_lag,
                self.pillars.motion_suppression_in_runtime,
                self.pillars.no_hardcoded_stable_styling
            ),
        ];
        lines.push("mode_rows:".to_string());
        for row in &self.mode_rows {
            lines.push(format!(
                "  - {} theme={} density={} motion={} resolves={} conforms={} marker={} capture={}",
                row.mode_class.as_str(),
                snake_token(&row.theme_class),
                snake_token(&row.density_class),
                row.motion_posture.token(),
                row.token_registry_resolves,
                row.conforms,
                row.surface_marker.as_str(),
                row.golden_capture_ref
            ));
        }
        lines.push("protected_cues:".to_string());
        for row in &self.protected_cues {
            lines.push(format!(
                "  - {} cues=[{}] hue_only_forbidden={} hc={} forced={} reduced={} conforms={}",
                row.cue_class.as_str(),
                row.non_color_cues
                    .iter()
                    .map(|cue| cue.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                row.hue_only_forbidden,
                row.survives_high_contrast,
                row.survives_forced_colors,
                row.survives_reduced_motion,
                row.conforms
            ));
        }
        lines.push("live_apply_axes:".to_string());
        for row in &self.live_apply_axes {
            lines.push(format!(
                "  - {} class={} disclosure_required={} silently_lags={}",
                snake_token(&row.axis),
                row.live_apply_class.as_str(),
                row.disclosure_required,
                row.silently_lags_system
            ));
        }
        lines.push("motion_suppression:".to_string());
        for row in &self.motion_suppression {
            lines.push(format!(
                "  - {} runtime={} suppressed={} substitution={} no_improvisation={}",
                row.posture.token(),
                row.modeled_in_token_runtime,
                row.non_essential_motion_suppressed,
                row.substitution_class,
                row.per_surface_improvisation_absent
            ));
        }
        lines.push("launch_surfaces:".to_string());
        for row in &self.launch_surfaces {
            lines.push(format!(
                "  - {} honors={} no_hardcoded={} marker={} conforms={} waiver={:?}",
                row.surface_class.as_str(),
                row.honors_token_runtime,
                row.hardcoded_styling_absent,
                row.surface_marker.as_str(),
                row.conforms,
                row.waiver_ref
            ));
        }
        lines.push(format!(
            "availability: without_account={} without_managed_services={}",
            self.available_without_account, self.available_without_managed_services
        ));
        lines.push(format!(
            "honesty_marker_present: {}",
            self.honesty_marker_present
        ));
        lines.push(format!(
            "diagnostics_export_ref: {}",
            self.diagnostics_export_ref
        ));
        lines.push(format!("support_export_ref: {}", self.support_export_ref));
        lines
    }
}

fn axis_order(axis: AppearanceAxis) -> u8 {
    match axis {
        AppearanceAxis::ModeThemeClass => 0,
        AppearanceAxis::ContrastMode => 1,
        AppearanceAxis::AccentSource => 2,
        AppearanceAxis::DensityClass => 3,
        AppearanceAxis::TextScale => 4,
        AppearanceAxis::ReducedMotionPosture => 5,
        AppearanceAxis::FollowSystemPosture => 6,
    }
}

fn posture_order(posture: AccessibilityPostureClass) -> u8 {
    match posture {
        AccessibilityPostureClass::MotionStandard => 0,
        AccessibilityPostureClass::MotionReduced => 1,
        AccessibilityPostureClass::MotionLowMotion => 2,
        AccessibilityPostureClass::MotionPowerSaver => 3,
        AccessibilityPostureClass::MotionCriticalHotPath => 4,
    }
}

fn surface_order(surface: LaunchSurfaceClass) -> usize {
    LaunchSurfaceClass::required()
        .iter()
        .position(|candidate| *candidate == surface)
        .unwrap_or(usize::MAX)
}

// ---------------------------------------------------------------------------
// Recovery vocabulary
// ---------------------------------------------------------------------------

/// Closed recovery-action vocabulary exposed on a certification record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationRecoveryAction {
    /// Open the settings appearance panel — the authoritative surface.
    OpenAppearanceSettings,
    /// Inspect the certified appearance modes and their captures.
    InspectAppearanceModes,
    /// Inspect the live-apply posture per appearance axis.
    InspectLiveApplyPosture,
    /// Export a redacted design-token runtime support packet.
    ExportRuntimeSupport,
}

impl CertificationRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenAppearanceSettings => "open_appearance_settings",
            Self::InspectAppearanceModes => "inspect_appearance_modes",
            Self::InspectLiveApplyPosture => "inspect_live_apply_posture",
            Self::ExportRuntimeSupport => "export_runtime_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenAppearanceSettings => "Open appearance settings",
            Self::InspectAppearanceModes => "Inspect appearance modes",
            Self::InspectLiveApplyPosture => "Inspect live-apply posture",
            Self::ExportRuntimeSupport => "Export runtime support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenAppearanceSettings => RecoveryActionRole::Primary,
            Self::InspectAppearanceModes | Self::InspectLiveApplyPosture => {
                RecoveryActionRole::Recovery
            }
            Self::ExportRuntimeSupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every record must expose, in rendered order.
    pub const REQUIRED: [Self; 4] = [
        Self::OpenAppearanceSettings,
        Self::InspectAppearanceModes,
        Self::InspectLiveApplyPosture,
        Self::ExportRuntimeSupport,
    ];

    /// Builds a route record for this action.
    pub fn route(self) -> RecoveryRouteRecord {
        RecoveryRouteRecord {
            action_id: self.as_str().to_string(),
            action_label: self.surface_label().to_string(),
            action_role: self.role(),
            keyboard_reachable: true,
        }
    }
}

/// Returns the recovery routes every record must expose, in rendered order.
pub fn required_recovery_routes() -> Vec<RecoveryRouteRecord> {
    CertificationRecoveryAction::REQUIRED
        .into_iter()
        .map(CertificationRecoveryAction::route)
        .collect()
}
