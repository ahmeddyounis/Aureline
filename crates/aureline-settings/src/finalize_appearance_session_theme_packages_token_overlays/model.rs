//! Canonical stable truth model for the **appearance-session finalization**
//! certification: versioned theme packages, inspectable session summaries,
//! token-overlay validation, imported-theme mapping honesty, extension
//! appearance descriptors, live-change disclosure, and provenance preservation.
//!
//! ## Why one governed certification record
//!
//! Appearance state is consumed by every shell surface, by extension-contributed
//! UI, by migration and import flows, and by support/diagnostics exports. If each
//! surface improvises its own theme reading, silently drops unsupported tokens,
//! flattens imported-theme gaps into generic settings, or hides extension
//! inheritance gaps behind host chrome, then a theme change can silently break
//! trust cues, severity badges, or focus rings — and the exported packet may
//! claim parity that the runtime cannot prove. The risk this closes: a green
//! "appearance finalized" claim that is really an average over surfaces that each
//! diverge a little, with no proof that gaps stay visible or that provenance
//! survives export.
//!
//! A [`AppearanceSessionFinalizationCertification`] mints, for one posture:
//!
//! - **One appearance-session value** — the binding records the active session id
//!   and revision, and every row's export packet cites that same value.
//! - **Versioned theme package manifests** — every active package carries a
//!   manifest ref, version label, supported modes, density defaults, motion flags,
//!   and minimum contrast metadata.
//! - **Inspectable session summaries** — active package refs, follow-system state,
//!   theme/mode, accent source, text scale, density, reduced-motion/high-contrast
//!   state, and checkpoint/rollback information.
//! - **Token-overlay validation** — overlays are validated by scope; unknown or
//!   unsupported tokens are preserved inert or downgraded, never silently dropped.
//! - **Imported-theme mapping honesty** — mapping reports name translated slots,
//!   unsupported slots, syntax coverage, parity notes, and fallback behavior.
//! - **Extension appearance descriptors** — every UI-bearing extension declares
//!   inheritance or surfaces a visible gap in product, export, and diagnostics.
//! - **Live-change disclosure** — every OS appearance signal declares whether it
//!   applies live, behind a checkpoint, requires confirmation, or requires a
//!   disclosed reload/restart.
//! - **Provenance preservation** — package identity, unresolved slots, overlay
//!   lineage, and inheritance gaps survive import/export/sync unflattened.
//! - **A public claim ceiling** and **automatic narrowing** — a posture that cannot
//!   prove a pillar narrows below Stable with a named reason.

use std::collections::BTreeSet;

use aureline_ui::density::DensityClass;
use aureline_ui::themes::{
    AccentSourceClass, AccessibilityPostureClass, ContrastMode, FollowSystemPosture,
};
use aureline_ui::tokens::ThemeClass;
use serde::{Deserialize, Serialize};

pub use aureline_design_system::LaunchSurfaceClass;

/// Stable record-kind tag carried in serialized certification records.
pub const APPEARANCE_SESSION_FINALIZATION_RECORD_KIND: &str =
    "appearance_session_finalization_certification_record";

/// Schema version for the [`AppearanceSessionFinalizationCertification`] payload shape.
pub const APPEARANCE_SESSION_FINALIZATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const APPEARANCE_SESSION_FINALIZATION_SHARED_CONTRACT_REF: &str =
    "settings:appearance_session_finalization:v1";

/// Reviewer-facing notice rendered on every certification surface.
pub const APPEARANCE_SESSION_FINALIZATION_NOTICE: &str =
    "Appearance-session finalization certification: every active theme package carries a versioned \
     manifest with supported modes, density defaults, motion flags, and minimum contrast metadata; \
     the appearance-session summary cites one active package, follow-system state, theme/mode, \
     accent source, text scale, density, reduced-motion/high-contrast state, and live-preview \
     checkpoint/rollback information; token overlays are validated by scope and unknown or \
     unsupported tokens are preserved inert or downgraded rather than silently dropped; \
     imported-theme mapping reports name translated slots, unsupported slots, syntax coverage, \
     parity notes, and fallback behavior so imported themes cannot claim full fidelity without \
     evidence; UI-bearing extensions declare whether they inherit theme, density, contrast, focus, \
     and reduced-motion posture or surface a visible inheritance gap in product, diagnostics, and \
     support export; live OS appearance changes apply coherently or disclose a reload/restart \
     instead of silently drifting; appearance provenance — package identity, unresolved slots, \
     overlay lineage, and inheritance gaps — survives import/export/sync without flattening into \
     generic profile settings; and a posture that cannot prove a pillar narrows below Stable with \
     a named reason rather than inheriting an adjacent green row.";

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The appearance session is replacement-grade across the claimed rows.
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

/// Surface a certification can be reached from.
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

/// The single appearance-session value every row is attributed to.
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
// Theme package manifest rows
// ---------------------------------------------------------------------------

/// One versioned theme package manifest row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePackageManifestRow {
    /// Package ref.
    pub package_ref: String,
    /// Package revision ref.
    pub package_revision_ref: String,
    /// Package version label.
    pub package_version_label: String,
    /// Supported theme classes, sorted.
    pub supported_theme_classes: Vec<ThemeClass>,
    /// Supported density classes, sorted.
    pub supported_density_classes: Vec<DensityClass>,
    /// Supported motion postures, sorted.
    pub supported_motion_postures: Vec<AccessibilityPostureClass>,
    /// Default density class.
    pub default_density_class: DensityClass,
    /// Default motion posture.
    pub default_motion_posture: AccessibilityPostureClass,
    /// Minimum text contrast target for the active theme.
    pub minimum_text_contrast_target: f32,
    /// Minimum UI contrast target for the active theme.
    pub minimum_ui_contrast_target: f32,
    /// Whether the manifest is versioned and inspectable.
    pub manifest_versioned: bool,
    /// Whether provenance is declared (inheritance expectations, source identity).
    pub provenance_declared: bool,
    /// Whether the manifest cannot silently redefine trust/severity semantics.
    pub trust_severity_semantics_preserved: bool,
    /// Row lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Optional bounded waiver ref for a package narrowed below Stable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the package manifest is versioned, provenance is declared, and
    /// trust/severity semantics are preserved.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Appearance-session summary rows
// ---------------------------------------------------------------------------

/// One appearance-session summary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppearanceSessionSummaryRow {
    /// Session id.
    pub appearance_session_id: String,
    /// Session revision.
    pub session_revision: u64,
    /// Active theme package ref.
    pub active_theme_package_ref: String,
    /// Active theme revision ref.
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
    /// Whether a live-preview checkpoint is active.
    pub checkpoint_active: bool,
    /// Current checkpoint ref, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_checkpoint_ref: Option<String>,
    /// Current rollback ref, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_ref: Option<String>,
    /// Whether the session summary is exportable and inspectable.
    pub summary_exportable: bool,
    /// Whether the session cites one canonical theme package source.
    pub cites_one_package_source: bool,
    /// Derived: the summary is exportable and cites one package source.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Token-overlay validation
// ---------------------------------------------------------------------------

/// Scope that owns a token overlay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlayScopeClass {
    /// User-level overlay.
    User,
    /// Profile-level overlay.
    Profile,
    /// Workspace-level overlay.
    Workspace,
    /// Policy-level overlay.
    Policy,
}

impl OverlayScopeClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Profile => "profile",
            Self::Workspace => "workspace",
            Self::Policy => "policy",
        }
    }

    /// The closed required scope set in canonical order.
    pub const REQUIRED: [Self; 4] = [Self::User, Self::Profile, Self::Workspace, Self::Policy];
}

/// How an unknown or unsupported token is preserved round-trip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenPreservationClass {
    /// The token is fully supported and resolves.
    Supported,
    /// The token is preserved inert (stored but not applied).
    Inert,
    /// The token is downgraded to a known fallback.
    Downgraded,
    /// The token was silently dropped (this must be false for conformance).
    SilentlyDropped,
}

impl TokenPreservationClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Inert => "inert",
            Self::Downgraded => "downgraded",
            Self::SilentlyDropped => "silently_dropped",
        }
    }

    /// Returns `true` when the token survives round-trip (not silently dropped).
    pub const fn survives_round_trip(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }
}

/// One token-overlay validation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenOverlayValidationRow {
    /// Overlay scope.
    pub scope: OverlayScopeClass,
    /// Overlay ref.
    pub overlay_ref: String,
    /// Number of supported tokens in this overlay.
    pub supported_token_count: u32,
    /// Number of tokens preserved inert.
    pub inert_token_count: u32,
    /// Number of tokens downgraded to fallback.
    pub downgraded_token_count: u32,
    /// Number of tokens silently dropped (must be zero for conformance).
    pub silently_dropped_token_count: u32,
    /// Whether unknown tokens are preserved rather than dropped.
    pub unknown_tokens_preserved: bool,
    /// Whether unsupported tokens are preserved rather than dropped.
    pub unsupported_tokens_preserved: bool,
    /// Whether overlay scope lineage is recorded.
    pub scope_lineage_recorded: bool,
    /// Derived: no tokens are silently dropped and lineage is recorded.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Imported-theme mapping reports
// ---------------------------------------------------------------------------

/// One imported-theme mapping report row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedThemeMappingReportRow {
    /// Import report ref.
    pub report_ref: String,
    /// Source editor or theme format.
    pub source_format: String,
    /// Number of translated slots.
    pub translated_slot_count: u32,
    /// Number of unsupported slots.
    pub unsupported_slot_count: u32,
    /// Number of unresolved slots.
    pub unresolved_slot_count: u32,
    /// Number of slots substituted through fallback.
    pub fallback_substituted_count: u32,
    /// Whether syntax coverage is reported.
    pub syntax_coverage_reported: bool,
    /// Whether parity notes are visible.
    pub parity_notes_visible: bool,
    /// Whether fallback behavior is documented.
    pub fallback_behavior_documented: bool,
    /// Whether the import report carries a rollback path.
    pub rollback_path_present: bool,
    /// Whether the imported theme is blocked from claiming full fidelity without evidence.
    pub full_fidelity_claim_blocked_when_unsupported: bool,
    /// Derived: every honesty check is present and the rollback path is present.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Extension appearance descriptors
// ---------------------------------------------------------------------------

/// One extension/embedded-surface appearance descriptor row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionAppearanceDescriptorRow {
    /// Extension or surface id.
    pub surface_id: String,
    /// Extension or surface label.
    pub surface_label: String,
    /// Theme inheritance state.
    pub theme_inheritance: ExtensionInheritanceState,
    /// Density inheritance state.
    pub density_inheritance: ExtensionInheritanceState,
    /// High-contrast inheritance state.
    pub high_contrast_inheritance: ExtensionInheritanceState,
    /// Focus-token inheritance state.
    pub focus_inheritance: ExtensionInheritanceState,
    /// Reduced-motion inheritance state.
    pub reduced_motion_inheritance: ExtensionInheritanceState,
    /// Whether the gap (if any) is visible in-product.
    pub gap_visible_in_product: bool,
    /// Whether the gap (if any) is visible in exported appearance packets.
    pub gap_visible_in_export: bool,
    /// Whether the gap (if any) is visible in migration/support diagnostics.
    pub gap_visible_in_diagnostics: bool,
    /// Whether the descriptor prevents a quiet Stable parity claim.
    pub prevents_quiet_stable_claim: bool,
    /// Derived: every axis is `inherits` or the gap is visible everywhere.
    pub conforms: bool,
}

/// Inheritance state for one appearance axis on an extension surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionInheritanceState {
    /// The surface fully inherits the host axis.
    Inherits,
    /// The surface partially inherits the host axis.
    Partial,
    /// The surface does not inherit the host axis.
    DoesNotInherit,
}

impl ExtensionInheritanceState {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherits => "inherits",
            Self::Partial => "partial",
            Self::DoesNotInherit => "does_not_inherit",
        }
    }

    /// Returns `true` when the surface fully inherits the axis.
    pub const fn inherits_fully(self) -> bool {
        matches!(self, Self::Inherits)
    }
}

// ---------------------------------------------------------------------------
// Live-appearance changes
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

/// One live-appearance change row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveAppearanceChangeRow {
    /// OS signal axis (theme, contrast, accent, density, text-scale, reduced-motion).
    pub axis: LiveAppearanceAxisClass,
    /// Live-apply class for this axis.
    pub live_apply_class: LiveApplyClass,
    /// Whether a reload/restart is disclosed to the user.
    pub disclosure_required: bool,
    /// Whether the axis silently lags the system state (must be false).
    pub silently_lags_system: bool,
    /// Whether Aureline applies the change coherently or names surfaces that require reload.
    pub applies_coherently_or_discloses: bool,
    /// Reviewer-facing note.
    pub note: String,
    /// Derived: no silent lag and coherent apply or disclosure.
    pub conforms: bool,
}

/// OS appearance signal axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveAppearanceAxisClass {
    /// OS theme signal.
    OsTheme,
    /// OS contrast signal.
    OsContrast,
    /// OS accent signal.
    OsAccent,
    /// OS density signal.
    OsDensity,
    /// OS text-scale signal.
    OsTextScale,
    /// OS reduced-motion signal.
    OsReducedMotion,
}

impl LiveAppearanceAxisClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsTheme => "os_theme",
            Self::OsContrast => "os_contrast",
            Self::OsAccent => "os_accent",
            Self::OsDensity => "os_density",
            Self::OsTextScale => "os_text_scale",
            Self::OsReducedMotion => "os_reduced_motion",
        }
    }

    /// The closed required axis set in canonical order.
    pub const REQUIRED: [Self; 6] = [
        Self::OsTheme,
        Self::OsContrast,
        Self::OsAccent,
        Self::OsDensity,
        Self::OsTextScale,
        Self::OsReducedMotion,
    ];
}

// ---------------------------------------------------------------------------
// Provenance preservation
// ---------------------------------------------------------------------------

/// One provenance-preservation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenancePreservationRow {
    /// Provenance dimension.
    pub dimension: ProvenanceDimensionClass,
    /// Whether package/source identity survives export.
    pub package_identity_survives_export: bool,
    /// Whether unresolved-slot notes survive export.
    pub unresolved_slots_survive_export: bool,
    /// Whether overlay-scope lineage survives export.
    pub overlay_lineage_survives_export: bool,
    /// Whether extension/webview inheritance gaps survive export.
    pub inheritance_gaps_survive_export: bool,
    /// Whether the dimension survives sync without flattening.
    pub survives_sync_without_flattening: bool,
    /// Derived: every preservation check passes.
    pub conforms: bool,
}

/// Dimension of appearance provenance that must survive import/export/sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceDimensionClass {
    /// Theme package identity and revision.
    ThemePackageIdentity,
    /// Token overlay scope lineage.
    OverlayScopeLineage,
    /// Unresolved imported-slot notes.
    UnresolvedImportSlots,
    /// Extension/embedded inheritance gaps.
    ExtensionInheritanceGaps,
}

impl ProvenanceDimensionClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemePackageIdentity => "theme_package_identity",
            Self::OverlayScopeLineage => "overlay_scope_lineage",
            Self::UnresolvedImportSlots => "unresolved_import_slots",
            Self::ExtensionInheritanceGaps => "extension_inheritance_gaps",
        }
    }

    /// The closed required dimension set in canonical order.
    pub const REQUIRED: [Self; 4] = [
        Self::ThemePackageIdentity,
        Self::OverlayScopeLineage,
        Self::UnresolvedImportSlots,
        Self::ExtensionInheritanceGaps,
    ];
}

// ---------------------------------------------------------------------------
// Pillars, claim ceiling, qualification, upstream
// ---------------------------------------------------------------------------

/// The derived pillar verdicts (what the posture can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPillars {
    /// Every theme package manifest is versioned and declares provenance.
    pub theme_packages_versioned: bool,
    /// Every session summary is exportable and cites one package source.
    pub appearance_sessions_inspectable: bool,
    /// Every token overlay is validated and no token is silently dropped.
    pub token_overlays_validated: bool,
    /// Every imported theme carries a mapping report with visible gaps.
    pub import_reports_honest: bool,
    /// Every UI-bearing extension declares inheritance or surfaces a visible gap.
    pub extension_gaps_visible: bool,
    /// Every live OS appearance change applies coherently or discloses reload/restart.
    pub live_changes_disclosed: bool,
    /// Appearance provenance survives import/export/sync without flattening.
    pub provenance_intact: bool,
}

/// The public claim ceiling: what a posture is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationClaimCeiling {
    /// May claim every theme package is versioned and declares provenance.
    pub asserts_theme_packages_versioned: bool,
    /// May claim appearance sessions are inspectable and cite one package source.
    pub asserts_sessions_inspectable: bool,
    /// May claim token overlays are validated with no silent drops.
    pub asserts_token_overlays_validated: bool,
    /// May claim import reports are honest and gaps are visible.
    pub asserts_import_reports_honest: bool,
    /// May claim extension gaps are visible everywhere.
    pub asserts_extension_gaps_visible: bool,
    /// May claim live changes apply coherently or disclose reload/restart.
    pub asserts_live_changes_disclosed: bool,
    /// May claim provenance survives import/export/sync without flattening.
    pub asserts_provenance_intact: bool,
}

/// Reason a posture is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowingReason {
    /// A theme package manifest is unversioned or lacks provenance.
    ThemePackageUnversioned,
    /// An appearance session is not inspectable or does not cite one package source.
    AppearanceSessionNotInspectable,
    /// A token overlay silently drops unknown or unsupported tokens.
    TokenOverlaySilentlyDropped,
    /// An imported theme lacks a mapping report or hides visible gaps.
    ImportReportMissingVisibleGaps,
    /// An extension gap is undisclosed in product, export, or diagnostics.
    ExtensionGapUndisclosed,
    /// A live OS appearance change silently lags or hides a reload/restart.
    LiveChangeSilentLag,
    /// Appearance provenance is flattened during import/export/sync.
    ProvenanceFlattened,
    /// The lowest row marker is below Stable, so the posture must not inherit Stable.
    SurfaceNotYetStable,
}

impl CertificationNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemePackageUnversioned => "theme_package_unversioned",
            Self::AppearanceSessionNotInspectable => "appearance_session_not_inspectable",
            Self::TokenOverlaySilentlyDropped => "token_overlay_silently_dropped",
            Self::ImportReportMissingVisibleGaps => "import_report_missing_visible_gaps",
            Self::ExtensionGapUndisclosed => "extension_gap_undisclosed",
            Self::LiveChangeSilentLag => "live_change_silent_lag",
            Self::ProvenanceFlattened => "provenance_flattened",
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
    /// Extension appearance-conformance packet ref.
    pub appearance_conformance_packet_ref: String,
    /// Capture refs that contributed to this posture, sorted and deduped.
    pub contributing_capture_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`AppearanceSessionFinalizationCertification`].
#[derive(Debug, Clone, PartialEq)]
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
    /// The single appearance-session value rows attribute to.
    pub appearance_session: AppearanceSessionBinding,
    /// Theme package manifest rows.
    pub theme_packages: Vec<ThemePackageManifestRow>,
    /// Appearance session summary rows.
    pub session_summaries: Vec<AppearanceSessionSummaryRow>,
    /// Token overlay validation rows.
    pub token_overlays: Vec<TokenOverlayValidationRow>,
    /// Imported theme mapping report rows.
    pub import_reports: Vec<ImportedThemeMappingReportRow>,
    /// Extension appearance descriptor rows.
    pub extension_descriptors: Vec<ExtensionAppearanceDescriptorRow>,
    /// Live appearance change rows.
    pub live_changes: Vec<LiveAppearanceChangeRow>,
    /// Provenance preservation rows.
    pub provenance: Vec<ProvenancePreservationRow>,
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

/// The canonical, governed appearance-session finalization certification record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppearanceSessionFinalizationCertification {
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
    /// The lowest row marker — the record's overall surface marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Theme package manifest rows, in canonical package order.
    pub theme_packages: Vec<ThemePackageManifestRow>,
    /// Appearance session summary rows, in canonical session order.
    pub session_summaries: Vec<AppearanceSessionSummaryRow>,
    /// Token overlay validation rows, in canonical scope order.
    pub token_overlays: Vec<TokenOverlayValidationRow>,
    /// Imported theme mapping report rows, in canonical report order.
    pub import_reports: Vec<ImportedThemeMappingReportRow>,
    /// Extension appearance descriptor rows, in canonical surface order.
    pub extension_descriptors: Vec<ExtensionAppearanceDescriptorRow>,
    /// Live appearance change rows, in canonical axis order.
    pub live_changes: Vec<LiveAppearanceChangeRow>,
    /// Provenance preservation rows, in canonical dimension order.
    pub provenance: Vec<ProvenancePreservationRow>,
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

/// Reasons a [`AppearanceSessionFinalizationCertification`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// A required theme package manifest row was missing.
    ThemePackageRowMissing { package_ref: String },
    /// A theme package manifest row was duplicated.
    DuplicateThemePackageRow { package_ref: String },
    /// A theme package narrowed below Stable without a bounded waiver.
    ThemePackageNarrowedWithoutWaiver { package_ref: String },
    /// A required appearance session summary row was missing.
    SessionSummaryMissing { session_id: String },
    /// An appearance session summary row was duplicated.
    DuplicateSessionSummary { session_id: String },
    /// A session summary did not cite the same appearance session as the binding.
    SessionSummarySessionMismatch { session_id: String },
    /// A required token overlay scope row was missing.
    OverlayScopeMissing { scope: OverlayScopeClass },
    /// A token overlay scope row was duplicated.
    DuplicateOverlayScope { scope: OverlayScopeClass },
    /// A token overlay silently dropped tokens.
    OverlaySilentlyDroppedTokens { scope: OverlayScopeClass },
    /// A required imported-theme mapping report row was missing.
    ImportReportMissing { report_ref: String },
    /// An imported-theme mapping report row was duplicated.
    DuplicateImportReport { report_ref: String },
    /// An imported theme claimed full fidelity without evidence.
    ImportOverclaimedFullFidelity { report_ref: String },
    /// A required extension appearance descriptor row was missing.
    ExtensionDescriptorMissing { surface_id: String },
    /// An extension appearance descriptor row was duplicated.
    DuplicateExtensionDescriptor { surface_id: String },
    /// An extension gap was not disclosed in product, export, and diagnostics.
    ExtensionGapUndisclosed { surface_id: String },
    /// A required live-appearance change axis row was missing.
    LiveChangeAxisMissing { axis: LiveAppearanceAxisClass },
    /// A live-appearance change axis row was duplicated.
    DuplicateLiveChangeAxis { axis: LiveAppearanceAxisClass },
    /// A live-appearance axis silently lags the system or hides a reload/restart.
    LiveChangeSilentLag { axis: LiveAppearanceAxisClass },
    /// A required provenance dimension row was missing.
    ProvenanceDimensionMissing { dimension: ProvenanceDimensionClass },
    /// A provenance dimension row was duplicated.
    DuplicateProvenanceDimension { dimension: ProvenanceDimensionClass },
    /// A provenance dimension did not survive export/sync.
    ProvenanceNotPreserved { dimension: ProvenanceDimensionClass },
    /// The claim ceiling asserted theme-package versioning it cannot prove.
    OverclaimsThemePackagesVersioned,
    /// The claim ceiling asserted session inspectability it cannot prove.
    OverclaimsSessionsInspectable,
    /// The claim ceiling asserted token-overlay validation it cannot prove.
    OverclaimsTokenOverlaysValidated,
    /// The claim ceiling asserted import-report honesty it cannot prove.
    OverclaimsImportReportsHonest,
    /// The claim ceiling asserted extension-gap visibility it cannot prove.
    OverclaimsExtensionGapsVisible,
    /// The claim ceiling asserted live-change disclosure it cannot prove.
    OverclaimsLiveChangesDisclosed,
    /// The claim ceiling asserted provenance preservation it cannot prove.
    OverclaimsProvenanceIntact,
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
            Self::ThemePackageRowMissing { package_ref } => {
                write!(
                    f,
                    "theme package manifest row `{package_ref}` is missing"
                )
            }
            Self::DuplicateThemePackageRow { package_ref } => {
                write!(
                    f,
                    "theme package manifest row `{package_ref}` is duplicated"
                )
            }
            Self::ThemePackageNarrowedWithoutWaiver { package_ref } => write!(
                f,
                "theme package `{package_ref}` is narrowed below Stable but carries no bounded waiver ref"
            ),
            Self::SessionSummaryMissing { session_id } => {
                write!(
                    f,
                    "appearance session summary row `{session_id}` is missing"
                )
            }
            Self::DuplicateSessionSummary { session_id } => {
                write!(
                    f,
                    "appearance session summary row `{session_id}` is duplicated"
                )
            }
            Self::SessionSummarySessionMismatch { session_id } => write!(
                f,
                "session summary `{session_id}` does not cite the same appearance session as the binding"
            ),
            Self::OverlayScopeMissing { scope } => {
                write!(f, "token overlay scope row `{}` is missing", scope.as_str())
            }
            Self::DuplicateOverlayScope { scope } => {
                write!(
                    f,
                    "token overlay scope row `{}` is duplicated",
                    scope.as_str()
                )
            }
            Self::OverlaySilentlyDroppedTokens { scope } => write!(
                f,
                "token overlay scope `{}` silently drops unknown or unsupported tokens",
                scope.as_str()
            ),
            Self::ImportReportMissing { report_ref } => {
                write!(f, "imported-theme mapping report `{report_ref}` is missing")
            }
            Self::DuplicateImportReport { report_ref } => {
                write!(
                    f,
                    "imported-theme mapping report `{report_ref}` is duplicated"
                )
            }
            Self::ImportOverclaimedFullFidelity { report_ref } => write!(
                f,
                "imported-theme mapping report `{report_ref}` claims full fidelity without evidence"
            ),
            Self::ExtensionDescriptorMissing { surface_id } => {
                write!(
                    f,
                    "extension appearance descriptor `{surface_id}` is missing"
                )
            }
            Self::DuplicateExtensionDescriptor { surface_id } => {
                write!(
                    f,
                    "extension appearance descriptor `{surface_id}` is duplicated"
                )
            }
            Self::ExtensionGapUndisclosed { surface_id } => write!(
                f,
                "extension appearance descriptor `{surface_id}` does not disclose its gap in product, export, and diagnostics"
            ),
            Self::LiveChangeAxisMissing { axis } => {
                write!(
                    f,
                    "live-appearance change axis row `{}` is missing",
                    axis.as_str()
                )
            }
            Self::DuplicateLiveChangeAxis { axis } => {
                write!(
                    f,
                    "live-appearance change axis row `{}` is duplicated",
                    axis.as_str()
                )
            }
            Self::LiveChangeSilentLag { axis } => write!(
                f,
                "live-appearance axis `{}` must disclose a reload/restart and may not silently lag the system",
                axis.as_str()
            ),
            Self::ProvenanceDimensionMissing { dimension } => {
                write!(
                    f,
                    "provenance dimension row `{}` is missing",
                    dimension.as_str()
                )
            }
            Self::DuplicateProvenanceDimension { dimension } => {
                write!(
                    f,
                    "provenance dimension row `{}` is duplicated",
                    dimension.as_str()
                )
            }
            Self::ProvenanceNotPreserved { dimension } => write!(
                f,
                "provenance dimension `{}` does not survive export/sync",
                dimension.as_str()
            ),
            Self::OverclaimsThemePackagesVersioned => write!(
                f,
                "claim ceiling may not assert theme-package versioning when a package is unversioned"
            ),
            Self::OverclaimsSessionsInspectable => write!(
                f,
                "claim ceiling may not assert session inspectability when a summary is not exportable"
            ),
            Self::OverclaimsTokenOverlaysValidated => write!(
                f,
                "claim ceiling may not assert token-overlay validation when tokens are silently dropped"
            ),
            Self::OverclaimsImportReportsHonest => write!(
                f,
                "claim ceiling may not assert import-report honesty when gaps are hidden"
            ),
            Self::OverclaimsExtensionGapsVisible => write!(
                f,
                "claim ceiling may not assert extension-gap visibility when a gap is undisclosed"
            ),
            Self::OverclaimsLiveChangesDisclosed => write!(
                f,
                "claim ceiling may not assert live-change disclosure when an axis lags or hides a reload"
            ),
            Self::OverclaimsProvenanceIntact => write!(
                f,
                "claim ceiling may not assert provenance preservation when a dimension is flattened"
            ),
            Self::MissingRecoveryRoute { action } => {
                write!(f, "record must expose recovery route `{}`", action.as_str())
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
                write!(f, "accessibility layout mode `{}` is missing", mode.as_str())
            }
            Self::AccessibilityLayoutModeUnreachable { mode } => write!(
                f,
                "accessibility layout mode `{}` must keep narration and reachable affordances",
                mode.as_str()
            ),
            Self::AccessibilityActionLabelsMismatch => {
                write!(f, "accessibility action labels must match the recovery routes in order")
            }
            Self::HiddenWithoutAccount => {
                write!(
                    f,
                    "an appearance-session finalization certification must stay available without an account"
                )
            }
            Self::HiddenWithoutManagedServices => {
                write!(
                    f,
                    "an appearance-session finalization certification must stay available without managed services"
                )
            }
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

impl AppearanceSessionFinalizationCertification {
    /// Builds a governed certification record from validated input.
    ///
    /// The pillar verdicts are *derived* from the rows, so a record can never
    /// publish a claim wider than its proof. Structural lies are rejected
    /// outright; provable-but-imperfect postures are minted but narrowed below
    /// Stable with a named reason.
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

        let _session_value_ref = input.appearance_session.value_ref.clone();

        // --- theme packages --------------------------------------------------
        let mut seen_packages: BTreeSet<String> = BTreeSet::new();
        for row in &input.theme_packages {
            if !seen_packages.insert(row.package_ref.clone()) {
                return Err(BuildError::DuplicateThemePackageRow {
                    package_ref: row.package_ref.clone(),
                });
            }
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("theme_packages.waiver_ref", waiver)?;
            }
        }
        let mut theme_packages: Vec<ThemePackageManifestRow> = input.theme_packages.clone();
        theme_packages.sort_by(|a, b| a.package_ref.cmp(&b.package_ref));
        for row in &mut theme_packages {
            row.supported_theme_classes.sort_by_key(|t| t.token());
            row.supported_theme_classes.dedup();
            row.supported_density_classes.sort_by_key(|d| d.token());
            row.supported_density_classes.dedup();
            row.supported_motion_postures.sort_by_key(|m| m.token());
            row.supported_motion_postures.dedup();
            row.conforms = row.manifest_versioned
                && row.provenance_declared
                && row.trust_severity_semantics_preserved
                && !row.surface_marker.is_below_stable();
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::ThemePackageNarrowedWithoutWaiver {
                    package_ref: row.package_ref.clone(),
                });
            }
        }
        let theme_packages_versioned = theme_packages.iter().all(|row| row.conforms);

        // --- session summaries -----------------------------------------------
        let mut seen_sessions: BTreeSet<String> = BTreeSet::new();
        for row in &input.session_summaries {
            if !seen_sessions.insert(row.appearance_session_id.clone()) {
                return Err(BuildError::DuplicateSessionSummary {
                    session_id: row.appearance_session_id.clone(),
                });
            }
            if row.appearance_session_id != input.appearance_session.appearance_session_id {
                return Err(BuildError::SessionSummarySessionMismatch {
                    session_id: row.appearance_session_id.clone(),
                });
            }
        }
        let mut session_summaries: Vec<AppearanceSessionSummaryRow> =
            input.session_summaries.clone();
        session_summaries.sort_by(|a, b| a.appearance_session_id.cmp(&b.appearance_session_id));
        for row in &mut session_summaries {
            row.conforms = row.summary_exportable && row.cites_one_package_source;
        }
        let appearance_sessions_inspectable = session_summaries.iter().all(|row| row.conforms);

        // --- token overlays --------------------------------------------------
        let mut seen_scopes: BTreeSet<OverlayScopeClass> = BTreeSet::new();
        for row in &input.token_overlays {
            if !seen_scopes.insert(row.scope) {
                return Err(BuildError::DuplicateOverlayScope { scope: row.scope });
            }
        }
        for required in OverlayScopeClass::REQUIRED {
            if !seen_scopes.contains(&required) {
                return Err(BuildError::OverlayScopeMissing { scope: required });
            }
        }
        let mut token_overlays: Vec<TokenOverlayValidationRow> = input.token_overlays.clone();
        token_overlays.sort_by_key(|row| row.scope);
        for row in &mut token_overlays {
            row.conforms = row.unknown_tokens_preserved
                && row.unsupported_tokens_preserved
                && row.scope_lineage_recorded
                && row.silently_dropped_token_count == 0;
        }
        let token_overlays_validated = token_overlays.iter().all(|row| row.conforms);

        // --- import reports --------------------------------------------------
        let mut seen_reports: BTreeSet<String> = BTreeSet::new();
        for row in &input.import_reports {
            if !seen_reports.insert(row.report_ref.clone()) {
                return Err(BuildError::DuplicateImportReport {
                    report_ref: row.report_ref.clone(),
                });
            }
        }
        let mut import_reports: Vec<ImportedThemeMappingReportRow> = input.import_reports.clone();
        import_reports.sort_by(|a, b| a.report_ref.cmp(&b.report_ref));
        for row in &mut import_reports {
            row.conforms = row.syntax_coverage_reported
                && row.parity_notes_visible
                && row.fallback_behavior_documented
                && row.rollback_path_present
                && row.full_fidelity_claim_blocked_when_unsupported;
        }
        let import_reports_honest = import_reports.iter().all(|row| row.conforms);

        // --- extension descriptors -------------------------------------------
        let mut seen_descriptors: BTreeSet<String> = BTreeSet::new();
        for row in &input.extension_descriptors {
            if !seen_descriptors.insert(row.surface_id.clone()) {
                return Err(BuildError::DuplicateExtensionDescriptor {
                    surface_id: row.surface_id.clone(),
                });
            }
            let all_inherit = row.theme_inheritance.inherits_fully()
                && row.density_inheritance.inherits_fully()
                && row.high_contrast_inheritance.inherits_fully()
                && row.focus_inheritance.inherits_fully()
                && row.reduced_motion_inheritance.inherits_fully();
            if !all_inherit
                && (!row.gap_visible_in_product
                    || !row.gap_visible_in_export
                    || !row.gap_visible_in_diagnostics)
            {
                return Err(BuildError::ExtensionGapUndisclosed {
                    surface_id: row.surface_id.clone(),
                });
            }
        }
        let mut extension_descriptors: Vec<ExtensionAppearanceDescriptorRow> =
            input.extension_descriptors.clone();
        extension_descriptors.sort_by(|a, b| a.surface_id.cmp(&b.surface_id));
        for row in &mut extension_descriptors {
            let all_inherit = row.theme_inheritance.inherits_fully()
                && row.density_inheritance.inherits_fully()
                && row.high_contrast_inheritance.inherits_fully()
                && row.focus_inheritance.inherits_fully()
                && row.reduced_motion_inheritance.inherits_fully();
            row.conforms = all_inherit
                || (row.gap_visible_in_product
                    && row.gap_visible_in_export
                    && row.gap_visible_in_diagnostics
                    && row.prevents_quiet_stable_claim);
        }
        let extension_gaps_visible = extension_descriptors.iter().all(|row| row.conforms);

        // --- live changes ----------------------------------------------------
        let mut seen_axes: BTreeSet<LiveAppearanceAxisClass> = BTreeSet::new();
        for row in &input.live_changes {
            if !seen_axes.insert(row.axis) {
                return Err(BuildError::DuplicateLiveChangeAxis { axis: row.axis });
            }
            if !is_reviewable_sentence(&row.note) {
                return Err(BuildError::InvalidSentence {
                    field: "live_changes.note",
                });
            }
            if row.silently_lags_system
                || (row.live_apply_class.requires_reload_or_restart() && !row.disclosure_required)
                || !row.applies_coherently_or_discloses
            {
                return Err(BuildError::LiveChangeSilentLag { axis: row.axis });
            }
        }
        for required in LiveAppearanceAxisClass::REQUIRED {
            if !seen_axes.contains(&required) {
                return Err(BuildError::LiveChangeAxisMissing { axis: required });
            }
        }
        let mut live_changes: Vec<LiveAppearanceChangeRow> = input.live_changes.clone();
        live_changes.sort_by_key(|row| row.axis);
        for row in &mut live_changes {
            row.conforms = !row.silently_lags_system
                && row.applies_coherently_or_discloses
                && (!row.live_apply_class.requires_reload_or_restart() || row.disclosure_required);
        }
        let live_changes_disclosed = live_changes.iter().all(|row| row.conforms);

        // --- provenance ------------------------------------------------------
        let mut seen_dimensions: BTreeSet<ProvenanceDimensionClass> = BTreeSet::new();
        for row in &input.provenance {
            if !seen_dimensions.insert(row.dimension) {
                return Err(BuildError::DuplicateProvenanceDimension {
                    dimension: row.dimension,
                });
            }
            if !row.survives_sync_without_flattening {
                return Err(BuildError::ProvenanceNotPreserved {
                    dimension: row.dimension,
                });
            }
        }
        for required in ProvenanceDimensionClass::REQUIRED {
            if !seen_dimensions.contains(&required) {
                return Err(BuildError::ProvenanceDimensionMissing {
                    dimension: required,
                });
            }
        }
        let mut provenance: Vec<ProvenancePreservationRow> = input.provenance.clone();
        provenance.sort_by_key(|row| row.dimension);
        for row in &mut provenance {
            row.conforms = row.package_identity_survives_export
                && row.unresolved_slots_survive_export
                && row.overlay_lineage_survives_export
                && row.inheritance_gaps_survive_export
                && row.survives_sync_without_flattening;
        }
        let provenance_intact = provenance.iter().all(|row| row.conforms);

        // --- derive pillars --------------------------------------------------
        let pillars = CertificationPillars {
            theme_packages_versioned,
            appearance_sessions_inspectable,
            token_overlays_validated,
            import_reports_honest,
            extension_gaps_visible,
            live_changes_disclosed,
            provenance_intact,
        };

        // --- claim ceiling: never claim what cannot be proven ----------------
        if input.claim_ceiling.asserts_theme_packages_versioned && !theme_packages_versioned {
            return Err(BuildError::OverclaimsThemePackagesVersioned);
        }
        if input.claim_ceiling.asserts_sessions_inspectable && !appearance_sessions_inspectable {
            return Err(BuildError::OverclaimsSessionsInspectable);
        }
        if input.claim_ceiling.asserts_token_overlays_validated && !token_overlays_validated {
            return Err(BuildError::OverclaimsTokenOverlaysValidated);
        }
        if input.claim_ceiling.asserts_import_reports_honest && !import_reports_honest {
            return Err(BuildError::OverclaimsImportReportsHonest);
        }
        if input.claim_ceiling.asserts_extension_gaps_visible && !extension_gaps_visible {
            return Err(BuildError::OverclaimsExtensionGapsVisible);
        }
        if input.claim_ceiling.asserts_live_changes_disclosed && !live_changes_disclosed {
            return Err(BuildError::OverclaimsLiveChangesDisclosed);
        }
        if input.claim_ceiling.asserts_provenance_intact && !provenance_intact {
            return Err(BuildError::OverclaimsProvenanceIntact);
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

        // --- surface marker = lowest among row markers -----------------------
        let surface_lifecycle_marker = theme_packages
            .iter()
            .map(|row| row.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !theme_packages_versioned {
            narrowing_reasons.push(CertificationNarrowingReason::ThemePackageUnversioned);
        }
        if !appearance_sessions_inspectable {
            narrowing_reasons.push(CertificationNarrowingReason::AppearanceSessionNotInspectable);
        }
        if !token_overlays_validated {
            narrowing_reasons.push(CertificationNarrowingReason::TokenOverlaySilentlyDropped);
        }
        if !import_reports_honest {
            narrowing_reasons.push(CertificationNarrowingReason::ImportReportMissingVisibleGaps);
        }
        if !extension_gaps_visible {
            narrowing_reasons.push(CertificationNarrowingReason::ExtensionGapUndisclosed);
        }
        if !live_changes_disclosed {
            narrowing_reasons.push(CertificationNarrowingReason::LiveChangeSilentLag);
        }
        if !provenance_intact {
            narrowing_reasons.push(CertificationNarrowingReason::ProvenanceFlattened);
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
            record_kind: APPEARANCE_SESSION_FINALIZATION_RECORD_KIND.to_string(),
            schema_version: APPEARANCE_SESSION_FINALIZATION_SCHEMA_VERSION,
            notice: APPEARANCE_SESSION_FINALIZATION_NOTICE.to_string(),
            shared_contract_ref: APPEARANCE_SESSION_FINALIZATION_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            appearance_session: input.appearance_session,
            surface_lifecycle_marker,
            theme_packages,
            session_summaries,
            token_overlays,
            import_reports,
            extension_descriptors,
            live_changes,
            provenance,
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
                appearance_conformance_packet_ref: input.upstream.appearance_conformance_packet_ref,
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
            format!(
                "appearance_session_finalization_certification: {}",
                self.record_id
            ),
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
                "pillars: packages={} sessions={} overlays={} imports={} extensions={} live={} provenance={}",
                self.pillars.theme_packages_versioned,
                self.pillars.appearance_sessions_inspectable,
                self.pillars.token_overlays_validated,
                self.pillars.import_reports_honest,
                self.pillars.extension_gaps_visible,
                self.pillars.live_changes_disclosed,
                self.pillars.provenance_intact
            ),
        ];
        lines.push("theme_packages:".to_string());
        for row in &self.theme_packages {
            lines.push(format!(
                "  - {} version={} modes=[{}] densities=[{}] motions=[{}] provenance={} conforms={} marker={}",
                row.package_ref,
                row.package_version_label,
                row.supported_theme_classes.iter().map(|t| snake_token(t)).collect::<Vec<_>>().join(", "),
                row.supported_density_classes.iter().map(|d| snake_token(d)).collect::<Vec<_>>().join(", "),
                row.supported_motion_postures.iter().map(|m| m.token()).collect::<Vec<_>>().join(", "),
                row.provenance_declared,
                row.conforms,
                row.surface_marker.as_str()
            ));
        }
        lines.push("session_summaries:".to_string());
        for row in &self.session_summaries {
            lines.push(format!(
                "  - {} rev={} theme={} contrast={} density={} checkpoint={} conforms={}",
                row.appearance_session_id,
                row.session_revision,
                snake_token(&row.mode_theme_class),
                snake_token(&row.contrast_mode),
                snake_token(&row.density_class),
                row.checkpoint_active,
                row.conforms
            ));
        }
        lines.push("token_overlays:".to_string());
        for row in &self.token_overlays {
            lines.push(format!(
                "  - {} supported={} inert={} downgraded={} dropped={} lineage={} conforms={}",
                row.scope.as_str(),
                row.supported_token_count,
                row.inert_token_count,
                row.downgraded_token_count,
                row.silently_dropped_token_count,
                row.scope_lineage_recorded,
                row.conforms
            ));
        }
        lines.push("import_reports:".to_string());
        for row in &self.import_reports {
            lines.push(format!(
                "  - {} format={} translated={} unsupported={} unresolved={} fallback={} rollback={} conforms={}",
                row.report_ref,
                row.source_format,
                row.translated_slot_count,
                row.unsupported_slot_count,
                row.unresolved_slot_count,
                row.fallback_substituted_count,
                row.rollback_path_present,
                row.conforms
            ));
        }
        lines.push("extension_descriptors:".to_string());
        for row in &self.extension_descriptors {
            lines.push(format!(
                "  - {} theme={} density={} hc={} focus={} motion={} product={} export={} diag={} conforms={}",
                row.surface_id,
                row.theme_inheritance.as_str(),
                row.density_inheritance.as_str(),
                row.high_contrast_inheritance.as_str(),
                row.focus_inheritance.as_str(),
                row.reduced_motion_inheritance.as_str(),
                row.gap_visible_in_product,
                row.gap_visible_in_export,
                row.gap_visible_in_diagnostics,
                row.conforms
            ));
        }
        lines.push("live_changes:".to_string());
        for row in &self.live_changes {
            lines.push(format!(
                "  - {} class={} disclosure={} lags={} coherent={} conforms={}",
                row.axis.as_str(),
                row.live_apply_class.as_str(),
                row.disclosure_required,
                row.silently_lags_system,
                row.applies_coherently_or_discloses,
                row.conforms
            ));
        }
        lines.push("provenance:".to_string());
        for row in &self.provenance {
            lines.push(format!(
                "  - {} identity={} unresolved={} lineage={} gaps={} sync={} conforms={}",
                row.dimension.as_str(),
                row.package_identity_survives_export,
                row.unresolved_slots_survive_export,
                row.overlay_lineage_survives_export,
                row.inheritance_gaps_survive_export,
                row.survives_sync_without_flattening,
                row.conforms
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

// ---------------------------------------------------------------------------
// Recovery vocabulary
// ---------------------------------------------------------------------------

/// Closed recovery-action vocabulary exposed on a certification record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationRecoveryAction {
    /// Open the settings appearance panel.
    OpenAppearanceSettings,
    /// Inspect the active theme package manifests.
    InspectThemePackages,
    /// Inspect the token overlay validation state.
    InspectTokenOverlays,
    /// Export a redacted appearance-session support packet.
    ExportAppearanceSupport,
}

impl CertificationRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenAppearanceSettings => "open_appearance_settings",
            Self::InspectThemePackages => "inspect_theme_packages",
            Self::InspectTokenOverlays => "inspect_token_overlays",
            Self::ExportAppearanceSupport => "export_appearance_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenAppearanceSettings => "Open appearance settings",
            Self::InspectThemePackages => "Inspect theme packages",
            Self::InspectTokenOverlays => "Inspect token overlays",
            Self::ExportAppearanceSupport => "Export appearance support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenAppearanceSettings => RecoveryActionRole::Primary,
            Self::InspectThemePackages | Self::InspectTokenOverlays => RecoveryActionRole::Recovery,
            Self::ExportAppearanceSupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every record must expose, in rendered order.
    pub const REQUIRED: [Self; 4] = [
        Self::OpenAppearanceSettings,
        Self::InspectThemePackages,
        Self::InspectTokenOverlays,
        Self::ExportAppearanceSupport,
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
