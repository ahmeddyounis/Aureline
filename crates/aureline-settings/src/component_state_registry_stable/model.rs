//! Canonical stable truth model for the **component-state registry
//! certification**: one shared component-state vocabulary, normalized
//! degraded-state treatments, extension/embedded inheritance honesty,
//! shell-zoning and responsive-fallback semantics, and per-permutation
//! state-fixture coverage across launch-critical shell, settings, and
//! notification surfaces.
//!
//! ## Why one governed certification record
//!
//! Component state is rendered by every launch-critical surface — and by
//! extension-contributed and webview-adjacent surfaces that sit beside them. If
//! each surface re-invents what "disabled", "blocked", "policy-locked",
//! "reconnecting", "warming", "partial", "stale", or "recovering" looks like and
//! says, then the same state forks by surface or theme: a focus ring vanishes in
//! one place, a blocked row reads as a spinner in another, an extension webview
//! quietly drops the host's high-contrast tokens, and the release packet ships a
//! single happy-path screenshot that hides all of it. The risk this closes: a
//! green "component states are consistent" claim that is really an average over
//! surfaces that each diverge a little, with no proof that extension gaps are
//! disclosed or that every launch-critical permutation was actually captured.
//!
//! A [`ComponentStateRegistryCertification`] mints, for one registry posture:
//!
//! - **One registry value** — the binding records the active registry id and
//!   revision; every family, normalized-state, zone, and fixture row resolves
//!   against the same shared taxonomy ([`aureline_ui::components::ComponentStateClass`])
//!   so no surface re-invents the vocabulary.
//! - **Family coverage** — one [`ComponentFamilyRow`] per core control, dense
//!   row, tab, tree, palette, popover, dialog, banner, job row, and inline
//!   notice, each declaring its supported states, required affordances, and
//!   accessibility note, and proving it is token-driven with focus visibility and
//!   screen-reader semantics preserved.
//! - **Normalized states** — one [`NormalizedStateRow`] per disabled, blocked,
//!   policy-locked, reconnecting, warming, partial, stale, and recovering state,
//!   proving the treatment stays visually and verbally consistent across the
//!   shell, review, settings, and support surfaces, with a narratable reason and
//!   an action path and never hue or animation alone.
//! - **Extension inheritance honesty** — one [`ExtensionInheritanceRow`] per
//!   appearance axis, projected from the live extension appearance-conformance
//!   packet, declaring whether contributed/embedded surfaces inherit the host
//!   axis fully, partially, not at all, or undisclosed — and proving any gap
//!   surfaces in review, diagnostics, and support/export.
//! - **Shell-zoning semantics** — one [`ShellZoneRow`] per declared slot, proving
//!   docked-versus-sheet state, min/max chrome metrics, density semantics,
//!   reduced-motion behavior, and placeholder-card states are token-driven rather
//!   than hard-coded per feature.
//! - **State-fixture coverage** — one [`StateFixtureRow`] per launch-critical
//!   surface/state permutation, projected from the live screenshot-diff packet,
//!   proving every permutation has a stable capture and fixture with focus
//!   visibility and screen-reader semantics preserved through the transition.
//! - **A public claim ceiling** and **automatic narrowing** — a posture that
//!   cannot prove a pillar, or whose lowest family marker is below Stable,
//!   narrows below Stable with a named reason instead of inheriting an adjacent
//!   green row.
//!
//! The family set, the extension inheritance, and the state-fixture matrix are
//! **not** reinvented here: every record is a genuine projection of the live
//! design-system registry, the live extension appearance-conformance packet, and
//! the live screenshot-diff packet assembled in
//! [`crate::component_state_registry_stable::corpus`].

use std::collections::BTreeSet;

use aureline_extensions::appearance_conformance::{AppearanceAxisClass, AppearanceSupportClass};
use aureline_ui::components::ComponentStateClass;
use aureline_ui::density::DensityClass;
use aureline_ui::themes::AccessibilityPostureClass;
use serde::{Deserialize, Serialize};

pub use aureline_design_system::{CanonicalStateClass, LaunchSurfaceClass};

/// Stable record-kind tag carried in serialized certification records.
pub const COMPONENT_STATE_REGISTRY_RECORD_KIND: &str =
    "component_state_registry_certification_record";

/// Schema version for the [`ComponentStateRegistryCertification`] payload shape.
pub const COMPONENT_STATE_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every surface that ingests this record.
pub const COMPONENT_STATE_REGISTRY_SHARED_CONTRACT_REF: &str =
    "settings:component_state_registry_stable:v1";

/// Reviewer-facing notice rendered on every certification surface.
pub const COMPONENT_STATE_REGISTRY_NOTICE: &str =
    "Component-state registry certification: product and extension surfaces share one component-state \
     vocabulary instead of local reinvention; core controls, dense rows, tabs, trees, palettes, \
     popovers, dialogs, banners, job rows, and inline notices each declare their states, required \
     affordances, and accessibility notes and render from the token runtime; disabled, blocked, \
     policy-locked, reconnecting, warming, partial, stale, and recovering states stay visually and \
     verbally consistent across shell, review, settings, and support with a narratable reason and an \
     action path; extension-contributed and embedded surfaces declare whether they inherit host \
     theme, density, focus, contrast, motion, and host metrics fully, partially, not at all, or \
     undisclosed, and every gap surfaces in review, diagnostics, and support export; shell-zoning \
     slot names, docked-versus-sheet state, min/max chrome metrics, density semantics, reduced-motion \
     behavior, and placeholder cards are token-driven; every launch-critical surface/state \
     permutation has a stable capture and fixture with focus visibility and screen-reader semantics \
     preserved through the transition; no state relies on hue or animation alone; and a posture that \
     cannot prove a pillar, or whose lowest family marker is below Stable, narrows below Stable with a \
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
    /// The component-state registry is replacement-grade across the claimed rows.
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

/// Lifecycle marker carried by a component family.
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
    /// The settings appearance / design-system panel — the authoritative surface.
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
    /// Opens or focuses the authoritative component-state registry surface.
    Primary,
    /// Inspects or recovers the registry / fixture state.
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
// Non-color cues + affordances
// ---------------------------------------------------------------------------

/// Non-color carrier a component state may use so meaning never rides on hue
/// or animation alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonColorCueClass {
    /// Visible label text names the state.
    LabelText,
    /// Icon or glyph carries a stable metaphor.
    Icon,
    /// Border participates in the state treatment.
    Border,
    /// Shape differentiates the state beyond hue.
    Shape,
    /// Focus ring is present where the surface can receive focus.
    FocusRing,
    /// Progress indicator accompanies an in-flight state.
    ProgressIndicator,
    /// Lock or shield glyph identifies trust, policy, or permission constraints.
    LockOrShieldGlyph,
    /// Check or selection marker differentiates completed / selected states.
    SelectionMarker,
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
            Self::ProgressIndicator => "progress_indicator",
            Self::LockOrShieldGlyph => "lock_or_shield_glyph",
            Self::SelectionMarker => "selection_marker",
        }
    }
}

/// Affordance a component family must keep for a state to be narratable and
/// reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredAffordanceClass {
    /// Keyboard focus stays visible in the state.
    KeyboardFocusVisible,
    /// The state carries a screen-reader label.
    ScreenReaderLabel,
    /// The state remains visible without a tooltip / hover.
    PersistentDisclosure,
    /// A high-risk / degraded state names why it changed.
    NarratableReason,
    /// A high-risk / degraded state offers an action path.
    ActionPath,
    /// At least one non-color cue carries the meaning.
    NonColorCue,
}

impl RequiredAffordanceClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusVisible => "keyboard_focus_visible",
            Self::ScreenReaderLabel => "screen_reader_label",
            Self::PersistentDisclosure => "persistent_disclosure",
            Self::NarratableReason => "narratable_reason",
            Self::ActionPath => "action_path",
            Self::NonColorCue => "non_color_cue",
        }
    }

    /// Affordances every conforming family must declare, in canonical order.
    pub const REQUIRED: [Self; 4] = [
        Self::KeyboardFocusVisible,
        Self::ScreenReaderLabel,
        Self::PersistentDisclosure,
        Self::NonColorCue,
    ];
}

// ---------------------------------------------------------------------------
// Canonical component-state vocabulary
// ---------------------------------------------------------------------------

/// The shared component-state vocabulary product and extension surfaces resolve
/// against instead of minting local state labels.
///
/// Each name binds to a [`ComponentStateClass`] in the live UI taxonomy via
/// [`Self::taxonomy`], so the registry can never drift into a parallel
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStateName {
    /// Pointer hover.
    Hover,
    /// Visible keyboard / assistive-tech focus.
    Focus,
    /// Durable selection across focus changes.
    Selected,
    /// Unavailable and non-actionable in the current context.
    Disabled,
    /// Background work in progress for this surface.
    Loading,
    /// Warning posture worth surfacing.
    Warning,
    /// Failed or invalid state requiring retry, repair, or diagnostics.
    Error,
    /// Action prevented by trust, permission, ownership, source, or capability.
    Blocked,
    /// Action prevented by admin / managed policy.
    PolicyLocked,
    /// Live reconnecting posture (remote attach, collaboration, provider).
    Reconnecting,
    /// Warm-restore / preparation in progress before content is ready.
    Warming,
    /// Reduced availability; the surface names what still works.
    Partial,
    /// Last-known-good shown while a refresh lags.
    Stale,
    /// Post-failure recovery in progress.
    Recovering,
    /// Reduced capability remains and certainty / freshness is lowered.
    Degraded,
}

impl ComponentStateName {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hover => "hover",
            Self::Focus => "focus",
            Self::Selected => "selected",
            Self::Disabled => "disabled",
            Self::Loading => "loading",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Blocked => "blocked",
            Self::PolicyLocked => "policy_locked",
            Self::Reconnecting => "reconnecting",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Recovering => "recovering",
            Self::Degraded => "degraded",
        }
    }

    /// Resolves this name to the shared UI component-state taxonomy entry. This
    /// is the binding that keeps the registry from re-inventing the vocabulary.
    pub const fn taxonomy(self) -> ComponentStateClass {
        match self {
            Self::Hover => ComponentStateClass::Hover,
            Self::Focus => ComponentStateClass::FocusVisible,
            Self::Selected => ComponentStateClass::Selected,
            Self::Disabled => ComponentStateClass::Disabled,
            Self::Loading => ComponentStateClass::Loading,
            Self::Warning => ComponentStateClass::Warning,
            Self::Error => ComponentStateClass::Warning,
            Self::Blocked => ComponentStateClass::Locked,
            Self::PolicyLocked => ComponentStateClass::PolicyBlocked,
            Self::Reconnecting => ComponentStateClass::Reconnecting,
            Self::Warming => ComponentStateClass::Pending,
            Self::Partial => ComponentStateClass::Degraded,
            Self::Stale => ComponentStateClass::Stale,
            Self::Recovering => ComponentStateClass::Restored,
            Self::Degraded => ComponentStateClass::Degraded,
        }
    }

    /// The full canonical state set the registry must cover, in stable order.
    pub const REQUIRED: [Self; 15] = [
        Self::Hover,
        Self::Focus,
        Self::Selected,
        Self::Disabled,
        Self::Loading,
        Self::Warning,
        Self::Error,
        Self::Blocked,
        Self::PolicyLocked,
        Self::Reconnecting,
        Self::Warming,
        Self::Partial,
        Self::Stale,
        Self::Recovering,
        Self::Degraded,
    ];

    /// The normalized degraded / high-risk states whose treatment must stay
    /// consistent across surfaces, in stable order.
    pub const NORMALIZED_REQUIRED: [Self; 8] = [
        Self::Disabled,
        Self::Blocked,
        Self::PolicyLocked,
        Self::Reconnecting,
        Self::Warming,
        Self::Partial,
        Self::Stale,
        Self::Recovering,
    ];
}

// ---------------------------------------------------------------------------
// Component families
// ---------------------------------------------------------------------------

/// Component families the registry governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentFamilyClass {
    /// Buttons, inputs, toggles, and other core controls.
    CoreControl,
    /// Dense list / table rows.
    DenseRow,
    /// Tab strips and tab items.
    Tab,
    /// Tree rows and disclosure trees.
    Tree,
    /// Command palettes and quick-open surfaces.
    Palette,
    /// Popovers and transient overlays.
    Popover,
    /// Dialogs and capability sheets.
    Dialog,
    /// Banners and inline alert strips.
    Banner,
    /// Durable job / activity rows.
    JobRow,
    /// Inline notices.
    InlineNotice,
}

impl ComponentFamilyClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreControl => "core_control",
            Self::DenseRow => "dense_row",
            Self::Tab => "tab",
            Self::Tree => "tree",
            Self::Palette => "palette",
            Self::Popover => "popover",
            Self::Dialog => "dialog",
            Self::Banner => "banner",
            Self::JobRow => "job_row",
            Self::InlineNotice => "inline_notice",
        }
    }

    /// The closed required family set in canonical order.
    pub const REQUIRED: [Self; 10] = [
        Self::CoreControl,
        Self::DenseRow,
        Self::Tab,
        Self::Tree,
        Self::Palette,
        Self::Popover,
        Self::Dialog,
        Self::Banner,
        Self::JobRow,
        Self::InlineNotice,
    ];
}

/// One component family's registry conformance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentFamilyRow {
    /// Family class.
    pub family_class: ComponentFamilyClass,
    /// Human-visible family label.
    pub display_label: String,
    /// Component states this family supports, sorted and deduped.
    pub supported_states: Vec<ComponentStateName>,
    /// Required affordances this family declares, sorted and deduped.
    pub required_affordances: Vec<RequiredAffordanceClass>,
    /// Non-color cues used across the family's states.
    pub non_color_cues: Vec<NonColorCueClass>,
    /// Reviewer-facing accessibility note.
    pub accessibility_note: String,
    /// Whether the family renders from the design-token runtime.
    pub token_driven: bool,
    /// Whether hard-coded colors / density / motion are absent.
    pub hardcoded_styling_absent: bool,
    /// Whether hue-only meaning is forbidden.
    pub hue_only_forbidden: bool,
    /// Whether animation-only meaning is forbidden.
    pub animation_only_forbidden: bool,
    /// Whether keyboard focus visibility is preserved through transitions.
    pub focus_visible_preserved: bool,
    /// Whether screen-reader semantics are preserved through transitions.
    pub screen_reader_semantics_preserved: bool,
    /// Family lifecycle marker.
    pub surface_marker: LifecycleMarker,
    /// Optional bounded waiver ref for a family narrowed below Stable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the family substantively conforms (token-driven, focus/SR
    /// preserved, affordances and cues present) and stays at Stable.
    pub conforms: bool,
}

impl ComponentFamilyRow {
    /// Whether the family substantively conforms, independent of its marker.
    fn substantively_conforms(&self) -> bool {
        self.token_driven
            && self.hardcoded_styling_absent
            && self.hue_only_forbidden
            && self.animation_only_forbidden
            && self.focus_visible_preserved
            && self.screen_reader_semantics_preserved
            && !self.non_color_cues.is_empty()
            && !self.supported_states.is_empty()
            && RequiredAffordanceClass::REQUIRED
                .iter()
                .all(|affordance| self.required_affordances.contains(affordance))
    }
}

// ---------------------------------------------------------------------------
// Normalized states
// ---------------------------------------------------------------------------

/// Product surface across which a normalized state's treatment must stay
/// consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrySurfaceClass {
    /// Shell chrome and durable shell rows.
    Shell,
    /// Review / diff surfaces.
    Review,
    /// Settings rows.
    Settings,
    /// Support / diagnostics export surfaces.
    Support,
}

impl RegistrySurfaceClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Review => "review",
            Self::Settings => "settings",
            Self::Support => "support",
        }
    }

    /// The surfaces a normalized state must read consistently across.
    pub const REQUIRED: [Self; 4] = [Self::Shell, Self::Review, Self::Settings, Self::Support];
}

/// One normalized degraded / high-risk state's consistency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalizedStateRow {
    /// Canonical state name.
    pub state_name: ComponentStateName,
    /// The shared taxonomy entry the name resolves to.
    pub taxonomy_ref: ComponentStateClass,
    /// Human-visible state label.
    pub display_label: String,
    /// Non-color cues required for this state, sorted and deduped.
    pub non_color_cues: Vec<NonColorCueClass>,
    /// Whether hue-only meaning is forbidden.
    pub hue_only_forbidden: bool,
    /// Whether animation-only meaning is forbidden.
    pub animation_only_forbidden: bool,
    /// Surfaces the treatment is held consistent across, sorted and deduped.
    pub consistent_across_surfaces: Vec<RegistrySurfaceClass>,
    /// Narratable reason rendered when a component enters the state.
    pub narratable_reason: String,
    /// Action path offered out of the state.
    pub action_path: String,
    /// Screen-reader label for the state.
    pub screen_reader_label: String,
    /// Derived: the state is consistent, non-color, and narratable.
    pub conforms: bool,
}

// ---------------------------------------------------------------------------
// Extension / embedded inheritance
// ---------------------------------------------------------------------------

/// One appearance axis' extension/embedded inheritance honesty row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInheritanceRow {
    /// Appearance axis (theme, density, focus ring, high contrast, reduced
    /// motion, host token).
    pub axis: AppearanceAxisClass,
    /// Effective support class projected from the live conformance packet.
    pub support_class: AppearanceSupportClass,
    /// Whether the gap (if any) is disclosed in the review surface.
    pub gap_disclosed_in_review: bool,
    /// Whether the gap (if any) surfaces in diagnostics.
    pub gap_surfaced_in_diagnostics: bool,
    /// Whether the gap (if any) surfaces in the support export.
    pub gap_surfaced_in_support_export: bool,
    /// Reviewer-facing caveat for the axis.
    pub caveat: String,
    /// Optional bounded waiver ref for an undisclosed / unsupported axis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: the axis inherits fully or discloses its gap everywhere.
    pub conforms: bool,
}

impl ExtensionInheritanceRow {
    /// Whether the gap is disclosed across review, diagnostics, and support.
    fn gap_disclosed_everywhere(&self) -> bool {
        self.gap_disclosed_in_review
            && self.gap_surfaced_in_diagnostics
            && self.gap_surfaced_in_support_export
    }
}

// ---------------------------------------------------------------------------
// Shell zoning / responsive fallback
// ---------------------------------------------------------------------------

/// Shell-zoning slot the registry governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellZoneClass {
    /// Primary navigation rail.
    NavigationRail,
    /// Primary sidebar / explorer.
    PrimarySidebar,
    /// Editor group region.
    EditorGroup,
    /// Secondary panel region.
    SecondaryPanel,
    /// Status strip.
    StatusStrip,
    /// Companion sheet / overflow surface.
    CompanionSheet,
}

impl ShellZoneClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NavigationRail => "navigation_rail",
            Self::PrimarySidebar => "primary_sidebar",
            Self::EditorGroup => "editor_group",
            Self::SecondaryPanel => "secondary_panel",
            Self::StatusStrip => "status_strip",
            Self::CompanionSheet => "companion_sheet",
        }
    }

    /// The closed required zone set in canonical order.
    pub const REQUIRED: [Self; 6] = [
        Self::NavigationRail,
        Self::PrimarySidebar,
        Self::EditorGroup,
        Self::SecondaryPanel,
        Self::StatusStrip,
        Self::CompanionSheet,
    ];
}

/// Docked-versus-sheet responsive state for a shell zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoneLayoutMode {
    /// The zone is docked into the chrome.
    Docked,
    /// The zone falls back to a sheet / overlay at narrow widths.
    Sheet,
}

impl ZoneLayoutMode {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docked => "docked",
            Self::Sheet => "sheet",
        }
    }
}

/// One shell-zone / responsive-fallback row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellZoneRow {
    /// Zone class.
    pub zone_class: ShellZoneClass,
    /// Declared slot name (token-driven, not a hard-coded literal).
    pub slot_name: String,
    /// Docked-versus-sheet responsive state.
    pub layout_mode: ZoneLayoutMode,
    /// Minimum chrome metric in density-independent pixels.
    pub min_chrome_px: u32,
    /// Maximum chrome metric in density-independent pixels.
    pub max_chrome_px: u32,
    /// Density semantics the zone honors.
    pub density_class: DensityClass,
    /// Reduced-motion posture the zone honors.
    pub reduced_motion_posture: AccessibilityPostureClass,
    /// Whether min/max chrome metrics are token-driven rather than hard-coded.
    pub metrics_token_driven: bool,
    /// Whether the placeholder-card state is token-driven.
    pub placeholder_token_driven: bool,
    /// Whether the responsive docked-versus-sheet fallback is token-driven.
    pub responsive_fallback_token_driven: bool,
    /// Optional bounded waiver ref for a zone that hard-codes a metric.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
    /// Derived: every zoning metric and placeholder is token-driven.
    pub conforms: bool,
}

impl ShellZoneRow {
    /// Whether every zoning metric and placeholder is token-driven.
    fn token_driven_throughout(&self) -> bool {
        self.metrics_token_driven
            && self.placeholder_token_driven
            && self.responsive_fallback_token_driven
            && self.min_chrome_px <= self.max_chrome_px
    }
}

// ---------------------------------------------------------------------------
// State fixtures
// ---------------------------------------------------------------------------

/// One launch-critical surface/state fixture-coverage row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateFixtureRow {
    /// Launch-critical surface the permutation captures.
    pub surface_class: LaunchSurfaceClass,
    /// Canonical state the permutation captures.
    pub state_class: CanonicalStateClass,
    /// Stable screenshot capture ref.
    pub screenshot_ref: String,
    /// Stable state-fixture ref.
    pub fixture_ref: String,
    /// Whether keyboard focus visibility is preserved in the capture.
    pub focus_visible_preserved: bool,
    /// Whether screen-reader semantics are preserved through the transition.
    pub screen_reader_semantics_preserved: bool,
    /// Whether the transition into the state is narrated.
    pub transition_narrated: bool,
    /// Whether the state carries a non-color cue.
    pub non_color_cue_present: bool,
    /// Whether no critical action depends on hover-only discovery.
    pub hover_only_critical_action_absent: bool,
    /// Derived: the permutation is a stable, accessible capture.
    pub conforms: bool,
}

impl StateFixtureRow {
    fn substantively_conforms(&self) -> bool {
        self.focus_visible_preserved
            && self.screen_reader_semantics_preserved
            && self.transition_narrated
            && self.non_color_cue_present
            && self.hover_only_critical_action_absent
    }
}

// ---------------------------------------------------------------------------
// Registry binding
// ---------------------------------------------------------------------------

/// The single registry value every family, state, zone, and fixture row
/// attributes to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryBinding {
    /// Active component-state registry id.
    pub registry_id: String,
    /// Active registry revision.
    pub registry_revision: u64,
    /// Canonical value ref every row cites.
    pub value_ref: String,
    /// Shared UI component-state taxonomy ref this registry resolves against.
    pub taxonomy_ref: String,
    /// Source-of-truth document and contract refs, sorted and deduped.
    pub source_refs: Vec<String>,
}

impl RegistryBinding {
    /// Builds the canonical value ref for a registry id and revision.
    pub fn value_ref_for(registry_id: &str, registry_revision: u64) -> String {
        format!("aureline://component-state-registry/{registry_id}@rev{registry_revision}")
    }
}

// ---------------------------------------------------------------------------
// Pillars, claim ceiling, qualification, upstream
// ---------------------------------------------------------------------------

/// The derived pillar verdicts (what the posture can actually prove).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPillars {
    /// Every required family is present, token-driven, and covers the vocabulary.
    pub registry_covers_all_families: bool,
    /// Every normalized state stays consistent across the required surfaces.
    pub states_normalized_consistently: bool,
    /// Every extension axis inherits fully or discloses its gap everywhere.
    pub extension_gaps_disclosed: bool,
    /// Every shell zone keeps its metrics and placeholders token-driven.
    pub shell_zoning_token_driven: bool,
    /// Every launch-critical permutation has a conforming fixture.
    pub state_fixtures_cover_permutations: bool,
    /// Focus visibility and screen-reader semantics survive every transition.
    pub focus_and_screen_reader_preserved: bool,
    /// No family or normalized state relies on hue or animation alone.
    pub no_hue_or_animation_only: bool,
}

/// The public claim ceiling: what a posture is allowed to assert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CertificationClaimCeiling {
    /// May claim the registry covers every family and the full vocabulary.
    pub asserts_registry_family_coverage: bool,
    /// May claim normalized states stay consistent across surfaces.
    pub asserts_state_normalization: bool,
    /// May claim extension gaps are disclosed everywhere.
    pub asserts_extension_gaps_disclosed: bool,
    /// May claim shell zoning is token-driven.
    pub asserts_shell_zoning_token_driven: bool,
    /// May claim state-fixture coverage is complete.
    pub asserts_state_fixture_coverage: bool,
    /// May claim focus and screen-reader semantics survive transitions.
    pub asserts_focus_and_screen_reader: bool,
    /// May claim no state relies on hue or animation alone.
    pub asserts_no_hue_or_animation_only: bool,
}

/// Reason a posture is narrowed below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowingReason {
    /// A required family is missing or does not cover the vocabulary.
    RegistryFamilyCoverageIncomplete,
    /// A normalized state's treatment forks by surface.
    StateNormalizationInconsistent,
    /// An extension axis gap is not disclosed everywhere.
    ExtensionGapUndisclosed,
    /// A shell zone hard-codes a chrome metric or placeholder.
    ShellZoningHardcoded,
    /// A launch-critical permutation lacks a conforming fixture.
    StateFixtureCoverageIncomplete,
    /// Focus visibility or screen-reader semantics regress through a transition.
    FocusOrScreenReaderRegression,
    /// A state relies on hue or animation alone.
    HueOrAnimationOnlyCue,
    /// The lowest family marker is below Stable, so the posture must not inherit
    /// Stable by adjacency.
    SurfaceNotYetStable,
}

impl CertificationNarrowingReason {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryFamilyCoverageIncomplete => "registry_family_coverage_incomplete",
            Self::StateNormalizationInconsistent => "state_normalization_inconsistent",
            Self::ExtensionGapUndisclosed => "extension_gap_undisclosed",
            Self::ShellZoningHardcoded => "shell_zoning_hardcoded",
            Self::StateFixtureCoverageIncomplete => "state_fixture_coverage_incomplete",
            Self::FocusOrScreenReaderRegression => "focus_or_screen_reader_regression",
            Self::HueOrAnimationOnlyCue => "hue_or_animation_only_cue",
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
    /// Design-system component-state registry id.
    pub component_state_registry_ref: String,
    /// Extension appearance-conformance packet id.
    pub appearance_conformance_packet_ref: String,
    /// Design-system screenshot-diff packet id.
    pub screenshot_diff_packet_ref: String,
    /// Shared UI component-state taxonomy ref.
    pub taxonomy_ref: String,
    /// Capture / fixture refs that contributed to this posture, sorted, deduped.
    pub contributing_fixture_refs: Vec<String>,
}

// ---------------------------------------------------------------------------
// Input + record
// ---------------------------------------------------------------------------

/// Validated input used to mint a [`ComponentStateRegistryCertification`].
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
    /// The single registry value rows attribute to.
    pub registry_binding: RegistryBinding,
    /// Component family rows.
    pub component_families: Vec<ComponentFamilyRow>,
    /// Normalized state rows.
    pub normalized_states: Vec<NormalizedStateRow>,
    /// Extension inheritance rows.
    pub extension_inheritance: Vec<ExtensionInheritanceRow>,
    /// Shell-zone rows.
    pub shell_zones: Vec<ShellZoneRow>,
    /// State-fixture coverage rows.
    pub state_fixtures: Vec<StateFixtureRow>,
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

/// The canonical, governed component-state registry certification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentStateRegistryCertification {
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
    /// The single registry value.
    pub registry_binding: RegistryBinding,
    /// The lowest family marker — the record's overall surface marker.
    pub surface_lifecycle_marker: LifecycleMarker,
    /// Component family rows, in canonical family order.
    pub component_families: Vec<ComponentFamilyRow>,
    /// Normalized state rows, in canonical state order.
    pub normalized_states: Vec<NormalizedStateRow>,
    /// Extension inheritance rows, in canonical axis order.
    pub extension_inheritance: Vec<ExtensionInheritanceRow>,
    /// Shell-zone rows, in canonical zone order.
    pub shell_zones: Vec<ShellZoneRow>,
    /// State-fixture rows, in canonical surface/state order.
    pub state_fixtures: Vec<StateFixtureRow>,
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

/// Reasons a [`ComponentStateRegistryCertification`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A field that must be a reviewable sentence was empty or too long.
    InvalidSentence { field: &'static str },
    /// A field that must be a canonical object ref was not.
    NonCanonicalRef { field: &'static str, value: String },
    /// A field that must be a present ref was empty or too long.
    MissingRef { field: &'static str },
    /// A required component family row was missing.
    FamilyRowMissing { family: ComponentFamilyClass },
    /// A component family row was duplicated.
    DuplicateFamilyRow { family: ComponentFamilyClass },
    /// A family narrowed below Stable without a bounded waiver.
    FamilyNarrowedWithoutWaiver { family: ComponentFamilyClass },
    /// The registry does not cover a required canonical state across families.
    StateVocabularyNotCovered { state: ComponentStateName },
    /// A required normalized state row was missing.
    NormalizedStateMissing { state: ComponentStateName },
    /// A normalized state row was duplicated.
    DuplicateNormalizedState { state: ComponentStateName },
    /// A normalized state row resolves to a different taxonomy entry than the name.
    NormalizedStateTaxonomyMismatch { state: ComponentStateName },
    /// A required appearance axis row was missing.
    AxisRowMissing { axis: AppearanceAxisClass },
    /// An appearance axis row was duplicated.
    DuplicateAxisRow { axis: AppearanceAxisClass },
    /// An extension axis gap was not disclosed and carries no bounded waiver.
    AxisGapUndisclosedWithoutWaiver { axis: AppearanceAxisClass },
    /// A required shell zone row was missing.
    ZoneRowMissing { zone: ShellZoneClass },
    /// A shell zone row was duplicated.
    DuplicateZoneRow { zone: ShellZoneClass },
    /// A shell zone that hard-codes a metric carries no bounded waiver.
    ZoneHardcodedWithoutWaiver { zone: ShellZoneClass },
    /// A required launch-critical surface lacks any fixture row.
    FixtureSurfaceMissing { surface: LaunchSurfaceClass },
    /// No fixture row exercises a required canonical state.
    FixtureStateMissing { state: CanonicalStateClass },
    /// The claim ceiling asserted family coverage it cannot prove.
    OverclaimsFamilyCoverage,
    /// The claim ceiling asserted state normalization it cannot prove.
    OverclaimsStateNormalization,
    /// The claim ceiling asserted extension-gap disclosure it cannot prove.
    OverclaimsExtensionGaps,
    /// The claim ceiling asserted token-driven shell zoning it cannot prove.
    OverclaimsShellZoning,
    /// The claim ceiling asserted fixture coverage it cannot prove.
    OverclaimsFixtureCoverage,
    /// The claim ceiling asserted focus/screen-reader survival it cannot prove.
    OverclaimsFocusAndScreenReader,
    /// The claim ceiling asserted no-hue/animation-only it cannot prove.
    OverclaimsNoHueOrAnimationOnly,
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
            Self::FamilyRowMissing { family } => {
                write!(f, "component family row `{}` is missing", family.as_str())
            }
            Self::DuplicateFamilyRow { family } => {
                write!(f, "component family row `{}` is duplicated", family.as_str())
            }
            Self::FamilyNarrowedWithoutWaiver { family } => write!(
                f,
                "component family `{}` is narrowed below Stable but carries no bounded waiver ref",
                family.as_str()
            ),
            Self::StateVocabularyNotCovered { state } => write!(
                f,
                "no component family covers the canonical state `{}`; the shared vocabulary is incomplete",
                state.as_str()
            ),
            Self::NormalizedStateMissing { state } => {
                write!(f, "normalized state row `{}` is missing", state.as_str())
            }
            Self::DuplicateNormalizedState { state } => {
                write!(f, "normalized state row `{}` is duplicated", state.as_str())
            }
            Self::NormalizedStateTaxonomyMismatch { state } => write!(
                f,
                "normalized state `{}` must resolve to the shared taxonomy entry it names",
                state.as_str()
            ),
            Self::AxisRowMissing { axis } => {
                write!(f, "extension appearance axis row `{}` is missing", axis.as_str())
            }
            Self::DuplicateAxisRow { axis } => {
                write!(f, "extension appearance axis row `{}` is duplicated", axis.as_str())
            }
            Self::AxisGapUndisclosedWithoutWaiver { axis } => write!(
                f,
                "extension axis `{}` does not inherit fully and does not disclose its gap in review, \
                 diagnostics, and support, and carries no bounded waiver",
                axis.as_str()
            ),
            Self::ZoneRowMissing { zone } => {
                write!(f, "shell zone row `{}` is missing", zone.as_str())
            }
            Self::DuplicateZoneRow { zone } => {
                write!(f, "shell zone row `{}` is duplicated", zone.as_str())
            }
            Self::ZoneHardcodedWithoutWaiver { zone } => write!(
                f,
                "shell zone `{}` hard-codes a metric or placeholder and carries no bounded waiver",
                zone.as_str()
            ),
            Self::FixtureSurfaceMissing { surface } => write!(
                f,
                "no state fixture covers launch-critical surface `{}`",
                surface.as_str()
            ),
            Self::FixtureStateMissing { state } => write!(
                f,
                "no state fixture exercises canonical state `{}`",
                state.as_str()
            ),
            Self::OverclaimsFamilyCoverage => write!(
                f,
                "claim ceiling may not assert family coverage when a family is missing or incomplete"
            ),
            Self::OverclaimsStateNormalization => write!(
                f,
                "claim ceiling may not assert state normalization it cannot prove"
            ),
            Self::OverclaimsExtensionGaps => write!(
                f,
                "claim ceiling may not assert extension-gap disclosure when a gap is undisclosed"
            ),
            Self::OverclaimsShellZoning => write!(
                f,
                "claim ceiling may not assert token-driven shell zoning when a zone hard-codes a metric"
            ),
            Self::OverclaimsFixtureCoverage => write!(
                f,
                "claim ceiling may not assert fixture coverage it cannot prove"
            ),
            Self::OverclaimsFocusAndScreenReader => write!(
                f,
                "claim ceiling may not assert focus/screen-reader survival when a row regresses"
            ),
            Self::OverclaimsNoHueOrAnimationOnly => write!(
                f,
                "claim ceiling may not assert no-hue/animation-only when a state relies on one"
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
                write!(f, "entry route surface `{}` is duplicated", surface.as_str())
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
            Self::AccessibilityActionLabelsMismatch => write!(
                f,
                "accessibility action labels must match the recovery routes in order"
            ),
            Self::HiddenWithoutAccount => write!(
                f,
                "a component-state registry certification must stay available without an account"
            ),
            Self::HiddenWithoutManagedServices => write!(
                f,
                "a component-state registry certification must stay available without managed services"
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

fn family_order(family: ComponentFamilyClass) -> usize {
    ComponentFamilyClass::REQUIRED
        .iter()
        .position(|candidate| *candidate == family)
        .unwrap_or(usize::MAX)
}

fn normalized_state_order(state: ComponentStateName) -> usize {
    ComponentStateName::NORMALIZED_REQUIRED
        .iter()
        .position(|candidate| *candidate == state)
        .unwrap_or(usize::MAX)
}

fn zone_order(zone: ShellZoneClass) -> usize {
    ShellZoneClass::REQUIRED
        .iter()
        .position(|candidate| *candidate == zone)
        .unwrap_or(usize::MAX)
}

fn surface_order(surface: LaunchSurfaceClass) -> usize {
    LaunchSurfaceClass::required()
        .iter()
        .position(|candidate| *candidate == surface)
        .unwrap_or(usize::MAX)
}

fn state_order(state: CanonicalStateClass) -> usize {
    CanonicalStateClass::required()
        .iter()
        .position(|candidate| *candidate == state)
        .unwrap_or(usize::MAX)
}

impl ComponentStateRegistryCertification {
    /// Builds a governed certification record from validated input.
    ///
    /// The pillar verdicts are *derived* from the family, normalized-state,
    /// extension, zone, and fixture rows, so a record can never publish a claim
    /// wider than its proof. Structural lies (a missing required row, a
    /// normalized state that resolves to a different taxonomy entry than it
    /// names, an undisclosed gap with no waiver) are rejected outright;
    /// provable-but-imperfect postures (a family still in Preview, a zone that
    /// hard-codes a metric behind a bounded waiver) are minted but narrowed below
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
            "registry_binding.value_ref",
            &input.registry_binding.value_ref,
        )?;
        require_present_ref(
            "registry_binding.taxonomy_ref",
            &input.registry_binding.taxonomy_ref,
        )?;
        require_present_ref(
            "upstream.component_state_registry_ref",
            &input.upstream.component_state_registry_ref,
        )?;
        require_present_ref(
            "upstream.appearance_conformance_packet_ref",
            &input.upstream.appearance_conformance_packet_ref,
        )?;
        require_present_ref(
            "upstream.screenshot_diff_packet_ref",
            &input.upstream.screenshot_diff_packet_ref,
        )?;

        // --- component families ----------------------------------------------
        let mut seen_families: BTreeSet<ComponentFamilyClass> = BTreeSet::new();
        for row in &input.component_families {
            if !seen_families.insert(row.family_class) {
                return Err(BuildError::DuplicateFamilyRow {
                    family: row.family_class,
                });
            }
            if !is_reviewable_sentence(&row.accessibility_note) {
                return Err(BuildError::InvalidSentence {
                    field: "component_families.accessibility_note",
                });
            }
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("component_families.waiver_ref", waiver)?;
            }
        }
        for required in ComponentFamilyClass::REQUIRED {
            if !seen_families.contains(&required) {
                return Err(BuildError::FamilyRowMissing { family: required });
            }
        }
        let mut component_families: Vec<ComponentFamilyRow> = input.component_families.clone();
        component_families.sort_by_key(|row| family_order(row.family_class));
        for row in &mut component_families {
            row.supported_states.sort();
            row.supported_states.dedup();
            row.required_affordances.sort();
            row.required_affordances.dedup();
            row.non_color_cues.sort();
            row.non_color_cues.dedup();
            row.conforms = row.substantively_conforms() && !row.surface_marker.is_below_stable();
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::FamilyNarrowedWithoutWaiver {
                    family: row.family_class,
                });
            }
        }
        // Vocabulary coverage: the union of supported states across families must
        // include every required canonical state.
        let mut covered_states: BTreeSet<ComponentStateName> = BTreeSet::new();
        for row in &component_families {
            for state in &row.supported_states {
                covered_states.insert(*state);
            }
        }
        for required in ComponentStateName::REQUIRED {
            if !covered_states.contains(&required) {
                return Err(BuildError::StateVocabularyNotCovered { state: required });
            }
        }
        let families_substantively_conform = component_families
            .iter()
            .all(|row| row.substantively_conforms());
        let registry_covers_all_families = families_substantively_conform;

        // --- normalized states -----------------------------------------------
        let mut seen_normalized: BTreeSet<ComponentStateName> = BTreeSet::new();
        for row in &input.normalized_states {
            if !seen_normalized.insert(row.state_name) {
                return Err(BuildError::DuplicateNormalizedState {
                    state: row.state_name,
                });
            }
            if row.taxonomy_ref != row.state_name.taxonomy() {
                return Err(BuildError::NormalizedStateTaxonomyMismatch {
                    state: row.state_name,
                });
            }
            for sentence in [&row.narratable_reason, &row.action_path] {
                if !is_reviewable_sentence(sentence) {
                    return Err(BuildError::InvalidSentence {
                        field: "normalized_states.reason_or_action",
                    });
                }
            }
        }
        for required in ComponentStateName::NORMALIZED_REQUIRED {
            if !seen_normalized.contains(&required) {
                return Err(BuildError::NormalizedStateMissing { state: required });
            }
        }
        let mut normalized_states: Vec<NormalizedStateRow> = input.normalized_states.clone();
        normalized_states.sort_by_key(|row| normalized_state_order(row.state_name));
        for row in &mut normalized_states {
            row.non_color_cues.sort();
            row.non_color_cues.dedup();
            row.consistent_across_surfaces.sort();
            row.consistent_across_surfaces.dedup();
            let all_surfaces = RegistrySurfaceClass::REQUIRED
                .iter()
                .all(|surface| row.consistent_across_surfaces.contains(surface));
            row.conforms = !row.non_color_cues.is_empty()
                && row.hue_only_forbidden
                && row.animation_only_forbidden
                && all_surfaces;
        }
        let states_normalized_consistently = normalized_states.iter().all(|row| row.conforms);

        // --- extension inheritance -------------------------------------------
        let mut seen_axes: BTreeSet<AppearanceAxisClass> = BTreeSet::new();
        for row in &input.extension_inheritance {
            if !seen_axes.insert(row.axis) {
                return Err(BuildError::DuplicateAxisRow { axis: row.axis });
            }
            if !is_reviewable_sentence(&row.caveat) {
                return Err(BuildError::InvalidSentence {
                    field: "extension_inheritance.caveat",
                });
            }
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("extension_inheritance.waiver_ref", waiver)?;
            }
        }
        for required in aureline_extensions::appearance_conformance::APPEARANCE_AXES {
            if !seen_axes.contains(&required) {
                return Err(BuildError::AxisRowMissing { axis: required });
            }
        }
        let mut extension_inheritance: Vec<ExtensionInheritanceRow> =
            input.extension_inheritance.clone();
        extension_inheritance.sort_by_key(|row| row.axis);
        for row in &mut extension_inheritance {
            let inherits_fully = row.support_class == AppearanceSupportClass::FullInheritance;
            row.conforms = inherits_fully || row.gap_disclosed_everywhere();
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::AxisGapUndisclosedWithoutWaiver { axis: row.axis });
            }
        }
        let extension_gaps_disclosed = extension_inheritance.iter().all(|row| row.conforms);

        // --- shell zones -----------------------------------------------------
        let mut seen_zones: BTreeSet<ShellZoneClass> = BTreeSet::new();
        for row in &input.shell_zones {
            if !seen_zones.insert(row.zone_class) {
                return Err(BuildError::DuplicateZoneRow {
                    zone: row.zone_class,
                });
            }
            require_present_ref("shell_zones.slot_name", &row.slot_name)?;
            if let Some(waiver) = &row.waiver_ref {
                require_present_ref("shell_zones.waiver_ref", waiver)?;
            }
        }
        for required in ShellZoneClass::REQUIRED {
            if !seen_zones.contains(&required) {
                return Err(BuildError::ZoneRowMissing { zone: required });
            }
        }
        let mut shell_zones: Vec<ShellZoneRow> = input.shell_zones.clone();
        shell_zones.sort_by_key(|row| zone_order(row.zone_class));
        for row in &mut shell_zones {
            row.conforms = row.token_driven_throughout();
            if !row.conforms && row.waiver_ref.is_none() {
                return Err(BuildError::ZoneHardcodedWithoutWaiver {
                    zone: row.zone_class,
                });
            }
        }
        let shell_zoning_token_driven = shell_zones.iter().all(|row| row.conforms);

        // --- state fixtures --------------------------------------------------
        let mut fixture_surfaces: BTreeSet<LaunchSurfaceClass> = BTreeSet::new();
        let mut fixture_states: BTreeSet<CanonicalStateClass> = BTreeSet::new();
        for row in &input.state_fixtures {
            require_canonical_ref("state_fixtures.screenshot_ref", &row.screenshot_ref)?;
            require_canonical_ref("state_fixtures.fixture_ref", &row.fixture_ref)?;
            fixture_surfaces.insert(row.surface_class);
            fixture_states.insert(row.state_class);
        }
        for required in LaunchSurfaceClass::required() {
            if !fixture_surfaces.contains(required) {
                return Err(BuildError::FixtureSurfaceMissing { surface: *required });
            }
        }
        for required in CanonicalStateClass::required() {
            if !fixture_states.contains(required) {
                return Err(BuildError::FixtureStateMissing { state: *required });
            }
        }
        let mut state_fixtures: Vec<StateFixtureRow> = input.state_fixtures.clone();
        state_fixtures.sort_by(|a, b| {
            surface_order(a.surface_class)
                .cmp(&surface_order(b.surface_class))
                .then_with(|| state_order(a.state_class).cmp(&state_order(b.state_class)))
        });
        for row in &mut state_fixtures {
            row.conforms = row.substantively_conforms();
        }
        let state_fixtures_cover_permutations = state_fixtures.iter().all(|row| row.conforms);

        // --- cross-cutting pillars -------------------------------------------
        let focus_and_screen_reader_preserved = component_families
            .iter()
            .all(|row| row.focus_visible_preserved && row.screen_reader_semantics_preserved)
            && state_fixtures
                .iter()
                .all(|row| row.focus_visible_preserved && row.screen_reader_semantics_preserved);
        let no_hue_or_animation_only = component_families
            .iter()
            .all(|row| row.hue_only_forbidden && row.animation_only_forbidden)
            && normalized_states
                .iter()
                .all(|row| row.hue_only_forbidden && row.animation_only_forbidden);

        let pillars = CertificationPillars {
            registry_covers_all_families,
            states_normalized_consistently,
            extension_gaps_disclosed,
            shell_zoning_token_driven,
            state_fixtures_cover_permutations,
            focus_and_screen_reader_preserved,
            no_hue_or_animation_only,
        };

        // --- claim ceiling: never claim what cannot be proven ----------------
        let ceiling = input.claim_ceiling;
        if ceiling.asserts_registry_family_coverage && !registry_covers_all_families {
            return Err(BuildError::OverclaimsFamilyCoverage);
        }
        if ceiling.asserts_state_normalization && !states_normalized_consistently {
            return Err(BuildError::OverclaimsStateNormalization);
        }
        if ceiling.asserts_extension_gaps_disclosed && !extension_gaps_disclosed {
            return Err(BuildError::OverclaimsExtensionGaps);
        }
        if ceiling.asserts_shell_zoning_token_driven && !shell_zoning_token_driven {
            return Err(BuildError::OverclaimsShellZoning);
        }
        if ceiling.asserts_state_fixture_coverage && !state_fixtures_cover_permutations {
            return Err(BuildError::OverclaimsFixtureCoverage);
        }
        if ceiling.asserts_focus_and_screen_reader && !focus_and_screen_reader_preserved {
            return Err(BuildError::OverclaimsFocusAndScreenReader);
        }
        if ceiling.asserts_no_hue_or_animation_only && !no_hue_or_animation_only {
            return Err(BuildError::OverclaimsNoHueOrAnimationOnly);
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

        // --- surface marker = lowest among family markers --------------------
        let surface_lifecycle_marker = component_families
            .iter()
            .map(|row| row.surface_marker)
            .min()
            .unwrap_or(LifecycleMarker::Stable);

        // --- derive the stable-claim verdict ---------------------------------
        let mut narrowing_reasons = Vec::new();
        if !registry_covers_all_families {
            narrowing_reasons.push(CertificationNarrowingReason::RegistryFamilyCoverageIncomplete);
        }
        if !states_normalized_consistently {
            narrowing_reasons.push(CertificationNarrowingReason::StateNormalizationInconsistent);
        }
        if !extension_gaps_disclosed {
            narrowing_reasons.push(CertificationNarrowingReason::ExtensionGapUndisclosed);
        }
        if !shell_zoning_token_driven {
            narrowing_reasons.push(CertificationNarrowingReason::ShellZoningHardcoded);
        }
        if !state_fixtures_cover_permutations {
            narrowing_reasons.push(CertificationNarrowingReason::StateFixtureCoverageIncomplete);
        }
        if !focus_and_screen_reader_preserved {
            narrowing_reasons.push(CertificationNarrowingReason::FocusOrScreenReaderRegression);
        }
        if !no_hue_or_animation_only {
            narrowing_reasons.push(CertificationNarrowingReason::HueOrAnimationOnlyCue);
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

        // --- normalise binding + upstream refs -------------------------------
        let mut registry_binding = input.registry_binding;
        registry_binding.source_refs.sort();
        registry_binding.source_refs.dedup();
        let mut contributing_fixture_refs = input.upstream.contributing_fixture_refs.clone();
        contributing_fixture_refs.sort();
        contributing_fixture_refs.dedup();

        Ok(Self {
            record_kind: COMPONENT_STATE_REGISTRY_RECORD_KIND.to_string(),
            schema_version: COMPONENT_STATE_REGISTRY_SCHEMA_VERSION,
            notice: COMPONENT_STATE_REGISTRY_NOTICE.to_string(),
            shared_contract_ref: COMPONENT_STATE_REGISTRY_SHARED_CONTRACT_REF.to_string(),
            record_id: input.record_id,
            as_of: input.as_of,
            posture_id: input.posture_id,
            posture_label: input.posture_label,
            title: input.title,
            summary: input.summary,
            registry_binding,
            surface_lifecycle_marker,
            component_families,
            normalized_states,
            extension_inheritance,
            shell_zones,
            state_fixtures,
            pillars,
            claim_ceiling: ceiling,
            stable_qualification,
            recovery_routes: input.recovery_routes,
            routes: input.routes,
            accessibility: input.accessibility,
            available_without_account: input.available_without_account,
            available_without_managed_services: input.available_without_managed_services,
            honesty_marker_present,
            upstream: CertificationUpstream {
                component_state_registry_ref: input.upstream.component_state_registry_ref,
                appearance_conformance_packet_ref: input.upstream.appearance_conformance_packet_ref,
                screenshot_diff_packet_ref: input.upstream.screenshot_diff_packet_ref,
                taxonomy_ref: input.upstream.taxonomy_ref,
                contributing_fixture_refs,
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
            format!("component_state_registry_certification: {}", self.record_id),
            format!("as_of: {}", self.as_of),
            format!("posture: {} ({})", self.posture_id, self.posture_label),
            format!(
                "surface_lifecycle_marker: {}",
                self.surface_lifecycle_marker.as_str()
            ),
            format!("title: {}", self.title),
            format!("summary: {}", self.summary),
            format!(
                "registry_binding: id={} rev={} taxonomy={} value_ref={}",
                self.registry_binding.registry_id,
                self.registry_binding.registry_revision,
                self.registry_binding.taxonomy_ref,
                self.registry_binding.value_ref
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
                "pillars: families={} normalized={} extension_gaps={} zoning={} fixtures={} focus_sr={} no_hue_anim={}",
                self.pillars.registry_covers_all_families,
                self.pillars.states_normalized_consistently,
                self.pillars.extension_gaps_disclosed,
                self.pillars.shell_zoning_token_driven,
                self.pillars.state_fixtures_cover_permutations,
                self.pillars.focus_and_screen_reader_preserved,
                self.pillars.no_hue_or_animation_only
            ),
        ];
        lines.push("component_families:".to_string());
        for row in &self.component_families {
            lines.push(format!(
                "  - {} states=[{}] token_driven={} marker={} conforms={} waiver={:?}",
                row.family_class.as_str(),
                row.supported_states
                    .iter()
                    .map(|state| state.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                row.token_driven,
                row.surface_marker.as_str(),
                row.conforms,
                row.waiver_ref
            ));
        }
        lines.push("normalized_states:".to_string());
        for row in &self.normalized_states {
            lines.push(format!(
                "  - {} taxonomy={} surfaces=[{}] hue_forbidden={} anim_forbidden={} conforms={}",
                row.state_name.as_str(),
                snake_token(&row.taxonomy_ref),
                row.consistent_across_surfaces
                    .iter()
                    .map(|surface| surface.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                row.hue_only_forbidden,
                row.animation_only_forbidden,
                row.conforms
            ));
        }
        lines.push("extension_inheritance:".to_string());
        for row in &self.extension_inheritance {
            lines.push(format!(
                "  - {} support={} review={} diagnostics={} support_export={} conforms={}",
                row.axis.as_str(),
                row.support_class.as_str(),
                row.gap_disclosed_in_review,
                row.gap_surfaced_in_diagnostics,
                row.gap_surfaced_in_support_export,
                row.conforms
            ));
        }
        lines.push("shell_zones:".to_string());
        for row in &self.shell_zones {
            lines.push(format!(
                "  - {} slot={} layout={} min={} max={} density={} motion={} token_driven={} conforms={}",
                row.zone_class.as_str(),
                row.slot_name,
                row.layout_mode.as_str(),
                row.min_chrome_px,
                row.max_chrome_px,
                row.density_class.token(),
                row.reduced_motion_posture.token(),
                row.metrics_token_driven,
                row.conforms
            ));
        }
        lines.push("state_fixtures:".to_string());
        for row in &self.state_fixtures {
            lines.push(format!(
                "  - {}/{} focus={} sr={} narrated={} non_color={} conforms={}",
                row.surface_class.as_str(),
                row.state_class.as_str(),
                row.focus_visible_preserved,
                row.screen_reader_semantics_preserved,
                row.transition_narrated,
                row.non_color_cue_present,
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
    /// Open the component-state registry surface — the authoritative surface.
    OpenComponentStateRegistry,
    /// Inspect the per-permutation state fixtures.
    InspectStateFixtures,
    /// Inspect the extension/embedded inheritance disclosure.
    InspectExtensionInheritance,
    /// Export a redacted component-state registry support packet.
    ExportRegistrySupport,
}

impl CertificationRecoveryAction {
    /// Stable action id quoted across surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenComponentStateRegistry => "open_component_state_registry",
            Self::InspectStateFixtures => "inspect_state_fixtures",
            Self::InspectExtensionInheritance => "inspect_extension_inheritance",
            Self::ExportRegistrySupport => "export_registry_support",
        }
    }

    /// Reviewer-facing label.
    pub const fn surface_label(self) -> &'static str {
        match self {
            Self::OpenComponentStateRegistry => "Open component-state registry",
            Self::InspectStateFixtures => "Inspect state fixtures",
            Self::InspectExtensionInheritance => "Inspect extension inheritance",
            Self::ExportRegistrySupport => "Export registry support",
        }
    }

    /// Placement / confirmation role.
    pub const fn role(self) -> RecoveryActionRole {
        match self {
            Self::OpenComponentStateRegistry => RecoveryActionRole::Primary,
            Self::InspectStateFixtures | Self::InspectExtensionInheritance => {
                RecoveryActionRole::Recovery
            }
            Self::ExportRegistrySupport => RecoveryActionRole::Secondary,
        }
    }

    /// The recovery actions every record must expose, in rendered order.
    pub const REQUIRED: [Self; 4] = [
        Self::OpenComponentStateRegistry,
        Self::InspectStateFixtures,
        Self::InspectExtensionInheritance,
        Self::ExportRegistrySupport,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component_state_registry_stable::corpus::component_state_registry_corpus;

    fn nominal_input() -> CertificationInput {
        // Reconstruct a known-good input by round-tripping the nominal record's
        // rows back through a fresh input, so the unit tests can perturb one
        // field at a time without re-deriving the whole corpus by hand.
        let record = component_state_registry_corpus()
            .into_iter()
            .find(|scenario| scenario.scenario_id == "nominal")
            .expect("nominal scenario exists")
            .record();
        CertificationInput {
            record_id: record.record_id,
            as_of: record.as_of,
            posture_id: record.posture_id,
            posture_label: record.posture_label,
            title: record.title,
            summary: record.summary,
            registry_binding: record.registry_binding,
            component_families: record.component_families,
            normalized_states: record.normalized_states,
            extension_inheritance: record.extension_inheritance,
            shell_zones: record.shell_zones,
            state_fixtures: record.state_fixtures,
            claim_ceiling: record.claim_ceiling,
            recovery_routes: record.recovery_routes,
            routes: record.routes,
            accessibility: record.accessibility,
            available_without_account: record.available_without_account,
            available_without_managed_services: record.available_without_managed_services,
            upstream: record.upstream,
            diagnostics_export_ref: record.diagnostics_export_ref,
            support_export_ref: record.support_export_ref,
            evidence_refs: record.evidence_refs,
            narrative_refs: record.narrative_refs,
        }
    }

    #[test]
    fn nominal_input_rebuilds_to_stable() {
        let record = ComponentStateRegistryCertification::build(nominal_input())
            .expect("nominal input builds");
        assert!(record.stable_qualification.qualifies_stable);
        assert_eq!(
            record.stable_qualification.claim_class,
            StableClaimClass::Stable
        );
    }

    #[test]
    fn missing_family_is_rejected() {
        let mut input = nominal_input();
        input
            .component_families
            .retain(|row| row.family_class != ComponentFamilyClass::Popover);
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::FamilyRowMissing {
                family: ComponentFamilyClass::Popover
            })
        ));
    }

    #[test]
    fn vocabulary_gap_is_rejected() {
        let mut input = nominal_input();
        // Drop every family's "recovering" support; only the job-row family
        // carries it, so the union no longer covers the canonical state.
        for row in &mut input.component_families {
            row.supported_states
                .retain(|state| *state != ComponentStateName::Recovering);
        }
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::StateVocabularyNotCovered {
                state: ComponentStateName::Recovering
            })
        ));
    }

    #[test]
    fn normalized_state_taxonomy_mismatch_is_rejected() {
        let mut input = nominal_input();
        input.normalized_states[0].taxonomy_ref = ComponentStateClass::Completed;
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::NormalizedStateTaxonomyMismatch { .. })
        ));
    }

    #[test]
    fn family_below_stable_without_waiver_is_rejected() {
        let mut input = nominal_input();
        input.component_families[0].surface_marker = LifecycleMarker::Beta;
        input.component_families[0].waiver_ref = None;
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::FamilyNarrowedWithoutWaiver { .. })
        ));
    }

    #[test]
    fn undisclosed_extension_gap_without_waiver_is_rejected() {
        let mut input = nominal_input();
        let row = &mut input.extension_inheritance[0];
        row.support_class = AppearanceSupportClass::UnsupportedPrivateStyling;
        row.gap_disclosed_in_review = false;
        row.gap_surfaced_in_diagnostics = false;
        row.gap_surfaced_in_support_export = false;
        row.waiver_ref = None;
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::AxisGapUndisclosedWithoutWaiver { .. })
        ));
    }

    #[test]
    fn overclaiming_shell_zoning_is_rejected() {
        let mut input = nominal_input();
        input.shell_zones[0].metrics_token_driven = false;
        input.shell_zones[0].waiver_ref = Some("aureline://waiver/csr-test-hardcoded".to_owned());
        // The claim ceiling still asserts token-driven zoning it can no longer
        // prove, so the build must refuse the over-claim.
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::OverclaimsShellZoning)
        ));
    }

    #[test]
    fn hardcoded_zone_with_waiver_and_lowered_ceiling_narrows() {
        let mut input = nominal_input();
        input.shell_zones[0].metrics_token_driven = false;
        input.shell_zones[0].waiver_ref = Some("aureline://waiver/csr-test-hardcoded".to_owned());
        input.claim_ceiling.asserts_shell_zoning_token_driven = false;
        let record = ComponentStateRegistryCertification::build(input)
            .expect("narrowed-but-honest posture builds");
        assert!(!record.stable_qualification.qualifies_stable);
        assert!(record
            .stable_qualification
            .narrowing_reasons
            .contains(&CertificationNarrowingReason::ShellZoningHardcoded));
    }

    #[test]
    fn missing_fixture_surface_is_rejected() {
        let mut input = nominal_input();
        input
            .state_fixtures
            .retain(|row| row.surface_class != LaunchSurfaceClass::CommandPalette);
        assert!(matches!(
            ComponentStateRegistryCertification::build(input),
            Err(BuildError::FixtureSurfaceMissing {
                surface: LaunchSurfaceClass::CommandPalette
            })
        ));
    }
}
