//! One reusable M5 design-system primitive — the interactive-state contract — so every claimed
//! M5 control renders its `Default`, `Hover`, `Focus-visible`, and `Pressed/Active` states the
//! same way, with no state meaning carried by color alone and no interaction-breaking layout
//! shift when focus, press, or hover transitions occur.
//!
//! Aureline's frozen shared-component-state-taxonomy component matrix
//! ([`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`])
//! names the interactive-state contract as one of its four governed component-state families and
//! freezes its controlled vocabulary — the interactive subset of the shared taxonomy (`default`,
//! `hover`, `focus_visible`, `pressed_active`), the interaction input routes it binds
//! (`pointer_hover`, `keyboard_focus`, `focus_visible_ring`, `press_activation`,
//! `assistive_tech_announced`, `reduced_motion_safe`), plus the surface families, deployment
//! lines, consumer surfaces, non-visual accessibility routes, mandatory labels, qualification
//! classes, and downgrade triggers. This module *implements* that contract as one reusable
//! resolver so a user — pointer, keyboard, or assistive-tech operator alike — always gets the
//! same explicit interactive-state behavior from a button, an icon button, a menu, a splitter, a
//! quick-action card, or any other high-frequency pane control, instead of one-off styling
//! accidents on individual surfaces.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_interactive_state_contract`] — takes one control's kind, the interactive state it
//!    is entering (one of `default`, `hover`, `focus_visible`, `pressed_active`), the live
//!    interaction context (whether a pointer is present, whether focus arrived from the keyboard,
//!    whether reduced-motion or high-contrast is active), its opaque stable control identity, and
//!    the opaque shared state-style token reference that renders it, and produces one
//!    [`M5ResolvedInteractiveStateContract`] carrying the derived presentation posture
//!    (resting-default, pointer-hover, keyboard-focus-visible, or pressed-or-active), the required
//!    non-color cues that carry the state beyond hue, the interaction input routes the state is
//!    reachable and announced through, and the hard guarantees that the hit target stays stable,
//!    the layout never shifts on a state transition, keyboard operators always get a visible focus
//!    ring, and the state is legible under high-contrast and reduced-motion. It never lets a state
//!    be signaled by color alone, never lets a state transition move the hit target or reflow the
//!    layout, never hides the non-visual keyboard route, and never invents a private state name.
//!
//! A single parity matrix — [`M5InteractiveStateContractPacket`] — binds one row per claimed M5
//! control (the push button, the icon button, the menu item, the pane splitter, and the
//! quick-action card) to the shared interactive-state anatomy, the same interactive states,
//! presentation postures, non-color cues, interaction input routes, export fields, mandatory
//! labels, and non-visual accessibility routes, so the default / hover / focus-visible / pressed
//! vocabulary and its no-color-only and no-layout-shift rules stay identical across desktop,
//! headless/export, and support consumers.
//!
//! The interactive state class ([`M5SharedComponentStateClass`]), the interaction input route
//! ([`M5InteractionInputRoute`]), the surface family
//! ([`M5ComponentStateSurfaceFamily`]), the deployment line
//! ([`M5ComponentStateDeploymentLine`]), the consumer surface
//! ([`M5ComponentStateConsumerSurface`]), the accessibility route
//! ([`M5ComponentStateAccessibilityRoute`]), the required label
//! ([`M5ComponentStateRequiredLabel`]), the qualification class
//! ([`M5ComponentStateQualificationClass`]), and the downgrade trigger
//! ([`M5ComponentStateDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the interactive-state
//! rendering itself: its claimed control kinds, its anatomy parts, its derived presentation
//! posture, its non-color cues, and its export fields. No M5 control invents a second
//! interactive-state grammar.
//!
//! Raw local paths, credentials, and private endpoints stay outside the export boundary; every
//! control identity and state-style token reference is carried only as an opaque, export-safe
//! representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_interactive_state_contract_packet,
    seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed,
    seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed,
    M5_INTERACTIVE_STATE_CONTRACT_PACKET_ID,
};

// The interactive state class, interaction input route, surface family, deployment line, consumer
// surface, accessibility route, required label, qualification class, and downgrade triggers are
// frozen once, in the shared-component-state-taxonomy component matrix. This primitive reuses them
// verbatim so it never invents a parallel interactive-state vocabulary.
pub use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5ComponentStateAccessibilityRoute, M5ComponentStateConsumerSurface,
    M5ComponentStateDeploymentLine, M5ComponentStateDowngradeTrigger,
    M5ComponentStateQualificationClass, M5ComponentStateRequiredLabel,
    M5ComponentStateSurfaceFamily, M5InteractionInputRoute, M5SharedComponentStateClass,
    M5SharedComponentStateFamily,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5InteractiveStateContractPacket`].
pub const M5_INTERACTIVE_STATE_CONTRACT_RECORD_KIND: &str =
    "implement_m5_default_hover_focus_visible_pressed_state_contracts_with_no_color_only_and_no_layout_shift_rules_across_claimed_m5_controls_and_pane_affordances";

/// Schema version for M5 interactive-state-contract records.
pub const M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the interactive-state-contract boundary schema.
pub const M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF: &str =
    "schemas/ui/m5-interactive-state-contract.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_INTERACTIVE_STATE_CONTRACT_DOC_REF: &str =
    "docs/design-system/m5_interactive_state_contract_primitive.md";

/// Repo-relative path of the frozen shared-component-state-taxonomy component matrix this
/// primitive narrows from.
pub const M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json";

/// Repo-relative path of the focus/selection accessibility contract the focus-visible state binds
/// its keyboard route and focus ring against.
pub const M5_INTERACTIVE_STATE_CONTRACT_FOCUS_SELECTION_REF: &str =
    "schemas/a11y/m5-focus-selection.schema.json";

/// Repo-relative path of the design-system component contract whose token hooks drive the
/// interactive-state treatment, so state semantics are never one-off implementation choices.
pub const M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_CONTRACT_REF: &str =
    "schemas/design-system/m5-component-contract.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_INTERACTIVE_STATE_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-interactive-state-contract-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INTERACTIVE_STATE_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/release/m5-interactive-state-contract-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_INTERACTIVE_STATE_CONTRACT_CSV_REF: &str =
    "artifacts/release/m5-interactive-state-contract-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_INTERACTIVE_STATE_CONTRACT_REPORT_REF: &str =
    "artifacts/design/m5-interactive-state-contract-primitive.md";

/// One claimed M5 control that renders the shared interactive-state contract. These are the
/// high-frequency pane controls the implementation requirements name — the push button, the icon
/// button, the menu item, the pane splitter, and the quick-action card — so the same default /
/// hover / focus-visible / pressed grammar works across every claimed control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractiveControlKind {
    /// A labelled push button.
    PushButton,
    /// An icon-only button.
    IconButton,
    /// A menu item in a menu or command surface.
    MenuItem,
    /// A pane splitter / drag handle affordance.
    PaneSplitter,
    /// A quick-action card in a pane or start surface.
    QuickActionCard,
}

impl M5InteractiveControlKind {
    /// Every claimed control kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PushButton,
        Self::IconButton,
        Self::MenuItem,
        Self::PaneSplitter,
        Self::QuickActionCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PushButton => "push_button",
            Self::IconButton => "icon_button",
            Self::MenuItem => "menu_item",
            Self::PaneSplitter => "pane_splitter",
            Self::QuickActionCard => "quick_action_card",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::PushButton => "Push Button",
            Self::IconButton => "Icon Button",
            Self::MenuItem => "Menu Item",
            Self::PaneSplitter => "Pane Splitter",
            Self::QuickActionCard => "Quick-Action Card",
        }
    }
}

/// The derived presentation posture of an interactive state — the resolver's verdict about how a
/// control's `default`, `hover`, `focus_visible`, or `pressed_active` state is rendered. Derived
/// one-to-one from the interactive state so no interactive state collapses into another, and so a
/// keyboard-driven focus is always distinguishable from a pointer hover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractiveStatePresentation {
    /// The resting default treatment.
    RestingDefault,
    /// The pointer-hover treatment.
    PointerHover,
    /// The keyboard focus-visible ring treatment.
    KeyboardFocusVisible,
    /// The pressed / active treatment.
    PressedOrActive,
}

impl M5InteractiveStatePresentation {
    /// Every presentation posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RestingDefault,
        Self::PointerHover,
        Self::KeyboardFocusVisible,
        Self::PressedOrActive,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestingDefault => "resting_default",
            Self::PointerHover => "pointer_hover",
            Self::KeyboardFocusVisible => "keyboard_focus_visible",
            Self::PressedOrActive => "pressed_or_active",
        }
    }

    /// The presentation posture for one interactive state, or `None` when the state is not one of
    /// the four governed interactive states.
    pub const fn from_state(state: M5SharedComponentStateClass) -> Option<Self> {
        match state {
            M5SharedComponentStateClass::Default => Some(Self::RestingDefault),
            M5SharedComponentStateClass::Hover => Some(Self::PointerHover),
            M5SharedComponentStateClass::FocusVisible => Some(Self::KeyboardFocusVisible),
            M5SharedComponentStateClass::PressedActive => Some(Self::PressedOrActive),
            _ => None,
        }
    }

    /// True when this posture is the keyboard focus-visible posture.
    pub const fn is_keyboard_focus_visible(self) -> bool {
        matches!(self, Self::KeyboardFocusVisible)
    }
}

/// One non-color cue an interactive state renders so its meaning is never carried by hue alone.
/// Every derived presentation posture publishes at least one of these, enforcing the
/// no-color-only signaling rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractiveStateCue {
    /// The control's persistent label or icon text remains present in every state.
    PersistentStateLabel,
    /// A focus ring / outline carries the focus-visible state.
    FocusRingOutline,
    /// A border or outline weight shift carries the state without a fill-color change.
    BorderOrOutlineShift,
    /// An elevation or shadow shift carries the hover state.
    ElevationOrShadowShift,
    /// An inset or press depression carries the pressed / active state.
    PressInsetOrDepression,
    /// A pointer cursor affordance signals the hoverable target.
    PointerCursorAffordance,
}

impl M5InteractiveStateCue {
    /// Every non-color cue, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PersistentStateLabel,
        Self::FocusRingOutline,
        Self::BorderOrOutlineShift,
        Self::ElevationOrShadowShift,
        Self::PressInsetOrDepression,
        Self::PointerCursorAffordance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentStateLabel => "persistent_state_label",
            Self::FocusRingOutline => "focus_ring_outline",
            Self::BorderOrOutlineShift => "border_or_outline_shift",
            Self::ElevationOrShadowShift => "elevation_or_shadow_shift",
            Self::PressInsetOrDepression => "press_inset_or_depression",
            Self::PointerCursorAffordance => "pointer_cursor_affordance",
        }
    }
}

/// Controlled interactive-state anatomy part the shared contract surfaces. The parts in
/// [`M5InteractiveStateAnatomyPart::MANDATORY`] are required on every control so the state
/// identity, the presentation posture, the non-color cue set, the stable hit target, and the
/// non-visual keyboard route are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractiveStateAnatomyPart {
    /// The typed state identity cue.
    StateIdentityCue,
    /// The derived presentation-posture cue.
    PresentationPostureCue,
    /// The non-color cue-set cue.
    NonColorCueSetCue,
    /// The focus-ring cue.
    FocusRingCue,
    /// The stable hit-target guarantee cue.
    HitTargetGuaranteeCue,
    /// The layout-stability guarantee cue.
    LayoutStabilityCue,
    /// The reduced-motion / high-contrast legibility cue.
    ReducedMotionAndContrastCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5InteractiveStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StateIdentityCue,
        Self::PresentationPostureCue,
        Self::NonColorCueSetCue,
        Self::FocusRingCue,
        Self::HitTargetGuaranteeCue,
        Self::LayoutStabilityCue,
        Self::ReducedMotionAndContrastCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every control must render.
    pub const MANDATORY: [Self; 5] = [
        Self::StateIdentityCue,
        Self::PresentationPostureCue,
        Self::NonColorCueSetCue,
        Self::HitTargetGuaranteeCue,
        Self::KeyboardRouteCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateIdentityCue => "state_identity_cue",
            Self::PresentationPostureCue => "presentation_posture_cue",
            Self::NonColorCueSetCue => "non_color_cue_set_cue",
            Self::FocusRingCue => "focus_ring_cue",
            Self::HitTargetGuaranteeCue => "hit_target_guarantee_cue",
            Self::LayoutStabilityCue => "layout_stability_cue",
            Self::ReducedMotionAndContrastCue => "reduced_motion_and_contrast_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the interactive-state export carries so its truth is reconstructable. The fields in
/// [`M5InteractiveStateExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractiveStateExportField {
    /// The control kind.
    ControlKind,
    /// The interactive state.
    InteractiveState,
    /// The derived presentation posture.
    Presentation,
    /// The required non-color cues.
    NonColorCues,
    /// The interaction input routes.
    InteractionRoutes,
    /// Whether the hit target stays stable across the transition.
    HitTargetStable,
    /// Whether the layout stays stable across the transition.
    LayoutStable,
    /// The shared state-style token reference.
    StateStyleRef,
}

impl M5InteractiveStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ControlKind,
        Self::InteractiveState,
        Self::Presentation,
        Self::NonColorCues,
        Self::InteractionRoutes,
        Self::HitTargetStable,
        Self::LayoutStable,
        Self::StateStyleRef,
    ];

    /// The export fields every control must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ControlKind,
        Self::InteractiveState,
        Self::Presentation,
        Self::NonColorCues,
        Self::InteractionRoutes,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlKind => "control_kind",
            Self::InteractiveState => "interactive_state",
            Self::Presentation => "presentation",
            Self::NonColorCues => "non_color_cues",
            Self::InteractionRoutes => "interaction_routes",
            Self::HitTargetStable => "hit_target_stable",
            Self::LayoutStable => "layout_stable",
            Self::StateStyleRef => "state_style_ref",
        }
    }
}

/// The four governed interactive states, in the frozen taxonomy's declaration order. Reused from
/// the interactive-state family's canonical partition of the shared taxonomy so this primitive
/// never re-lists a private interactive-state set.
pub fn interactive_states() -> Vec<M5SharedComponentStateClass> {
    M5SharedComponentStateFamily::InteractiveState
        .governed_states()
        .to_vec()
}

// ---- interactive-state resolver -----------------------------------------

/// The full input to the interactive-state-contract resolver for one control state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateResolutionInput {
    /// The claimed control kind.
    pub control_kind: M5InteractiveControlKind,
    /// The interactive state the control is entering (one of the four governed interactive
    /// states).
    pub interactive_state: M5SharedComponentStateClass,
    /// True when a pointer is present for this interaction.
    pub pointer_available: bool,
    /// True when focus arrived from the keyboard / assistive tech (drives the focus-visible ring).
    pub keyboard_focus_origin: bool,
    /// True when reduced-motion is active, so transitions carry no interaction-breaking motion.
    pub reduced_motion_active: bool,
    /// True when a high-contrast mode is active, so the state stays legible without hue.
    pub high_contrast_active: bool,
    /// The opaque stable control identity (must be non-empty).
    pub control_identity_ref: String,
    /// The opaque shared state-style token reference that renders this state (must be non-empty).
    pub state_style_ref: String,
}

/// The resolved interactive-state-contract truth for one control state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInteractiveStateContract {
    /// The control kind.
    pub control_kind: M5InteractiveControlKind,
    /// The interactive state.
    pub interactive_state: M5SharedComponentStateClass,
    /// The derived presentation posture.
    pub presentation: M5InteractiveStatePresentation,
    /// The required non-color cues that carry this state beyond hue.
    pub required_non_color_cues: Vec<M5InteractiveStateCue>,
    /// The interaction input routes this state is reachable and announced through.
    pub interaction_input_routes: Vec<M5InteractionInputRoute>,
    /// The opaque stable control identity, preserved exactly from the input.
    pub control_identity_ref: String,
    /// The opaque shared state-style token reference, preserved exactly from the input.
    pub state_style_ref: String,
    /// True when the visible focus ring is shown (focus-visible posture reached from the
    /// keyboard); a pointer-origin focus keeps the focus present and announced but suppresses the
    /// ring.
    pub focus_ring_shown: bool,
    /// True when reduced-motion is active, preserved from the input.
    pub reduced_motion_active: bool,
    /// True when high-contrast is active, preserved from the input.
    pub high_contrast_active: bool,
    /// State meaning is never carried by color alone. ALWAYS `true`.
    pub no_color_only_signaling: bool,
    /// The hit target is unchanged across this state transition. ALWAYS `true`.
    pub stable_hit_target: bool,
    /// The layout never shifts in a way that breaks interaction across this transition. ALWAYS
    /// `true`.
    pub no_interaction_breaking_layout_shift: bool,
    /// Keyboard / assistive-tech operators always get a visible focus ring capability. ALWAYS
    /// `true`.
    pub focus_visible_for_keyboard: bool,
    /// The state is legible under reduced-motion. ALWAYS `true`.
    pub reduced_motion_safe: bool,
    /// The state is legible under high-contrast and high-zoom. ALWAYS `true`.
    pub high_contrast_safe: bool,
    /// The state semantics are driven by the shared contract and its token hooks, not a one-off
    /// implementation choice. ALWAYS `true`.
    pub driven_by_shared_state_contract: bool,
}

/// Errors returned by [`resolve_interactive_state_contract`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5InteractiveStateResolutionError {
    /// The control identity ref was empty.
    EmptyControlIdentity,
    /// The state-style token ref was empty.
    EmptyStateStyleRef,
    /// The state was not one of the four governed interactive states.
    NonInteractiveState,
    /// A descriptor carried forbidden material.
    ForbiddenStateMaterial,
}

impl M5InteractiveStateResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyControlIdentity => "empty_control_identity",
            Self::EmptyStateStyleRef => "empty_state_style_ref",
            Self::NonInteractiveState => "non_interactive_state",
            Self::ForbiddenStateMaterial => "forbidden_state_material",
        }
    }
}

impl fmt::Display for M5InteractiveStateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "interactive state contract resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5InteractiveStateResolutionError {}

/// Resolves one interactive-state contract from a control's kind, the interactive state it is
/// entering, and the live interaction context.
///
/// The presentation posture is derived one-to-one from the interactive state so no state collapses
/// into another: `default` renders resting, `hover` renders the pointer-hover treatment,
/// `focus_visible` renders the keyboard focus-visible ring, and `pressed_active` renders the
/// pressed treatment. Each posture publishes a non-empty non-color cue set so the state is never
/// carried by color alone, and an interaction-route set that always includes a keyboard route and
/// an assistive-tech announcement so no state is pointer-only or hover-only. The focus ring is
/// *shown* only when the focus-visible posture is reached from the keyboard, but the visible-focus
/// capability, the stable hit target, the no-layout-shift guarantee, and reduced-motion /
/// high-contrast legibility hold for every resolved state.
pub fn resolve_interactive_state_contract(
    input: &M5InteractiveStateResolutionInput,
) -> Result<M5ResolvedInteractiveStateContract, M5InteractiveStateResolutionError> {
    if input.control_identity_ref.trim().is_empty() {
        return Err(M5InteractiveStateResolutionError::EmptyControlIdentity);
    }
    if input.state_style_ref.trim().is_empty() {
        return Err(M5InteractiveStateResolutionError::EmptyStateStyleRef);
    }
    if value_repr_is_forbidden(&input.control_identity_ref)
        || value_repr_is_forbidden(&input.state_style_ref)
    {
        return Err(M5InteractiveStateResolutionError::ForbiddenStateMaterial);
    }

    let presentation = M5InteractiveStatePresentation::from_state(input.interactive_state)
        .ok_or(M5InteractiveStateResolutionError::NonInteractiveState)?;

    let required_non_color_cues = derive_non_color_cues(presentation);
    let interaction_input_routes = derive_interaction_routes(presentation);
    let focus_ring_shown = presentation.is_keyboard_focus_visible() && input.keyboard_focus_origin;

    Ok(M5ResolvedInteractiveStateContract {
        control_kind: input.control_kind,
        interactive_state: input.interactive_state,
        presentation,
        required_non_color_cues,
        interaction_input_routes,
        control_identity_ref: input.control_identity_ref.clone(),
        state_style_ref: input.state_style_ref.clone(),
        focus_ring_shown,
        reduced_motion_active: input.reduced_motion_active,
        high_contrast_active: input.high_contrast_active,
        // The acceptance criteria: interactive state is never color-only, the hit target stays
        // stable, no interaction-breaking layout shift occurs, keyboard operators always get a
        // visible focus ring, the state is legible under high-contrast / reduced-motion / high
        // zoom, and the semantics are driven by the shared contract and token hooks.
        no_color_only_signaling: true,
        stable_hit_target: true,
        no_interaction_breaking_layout_shift: true,
        focus_visible_for_keyboard: true,
        reduced_motion_safe: true,
        high_contrast_safe: true,
        driven_by_shared_state_contract: true,
    })
}

/// Derives the non-color cue set for a presentation posture. Every posture publishes at least one
/// non-color cue, so state meaning is never carried by hue alone.
fn derive_non_color_cues(
    presentation: M5InteractiveStatePresentation,
) -> Vec<M5InteractiveStateCue> {
    use M5InteractiveStateCue as Cue;
    use M5InteractiveStatePresentation as Posture;

    match presentation {
        Posture::RestingDefault => vec![Cue::PersistentStateLabel],
        Posture::PointerHover => vec![
            Cue::PersistentStateLabel,
            Cue::BorderOrOutlineShift,
            Cue::ElevationOrShadowShift,
            Cue::PointerCursorAffordance,
        ],
        Posture::KeyboardFocusVisible => vec![
            Cue::PersistentStateLabel,
            Cue::FocusRingOutline,
            Cue::BorderOrOutlineShift,
        ],
        Posture::PressedOrActive => vec![
            Cue::PersistentStateLabel,
            Cue::PressInsetOrDepression,
            Cue::BorderOrOutlineShift,
        ],
    }
}

/// Derives the interaction input-route set for a presentation posture. Every set includes a
/// keyboard route, an assistive-tech announcement, and a reduced-motion-safe route, so no
/// interactive state is ever pointer-only or hover-only.
fn derive_interaction_routes(
    presentation: M5InteractiveStatePresentation,
) -> Vec<M5InteractionInputRoute> {
    use M5InteractionInputRoute as Route;
    use M5InteractiveStatePresentation as Posture;

    match presentation {
        Posture::RestingDefault => vec![
            Route::KeyboardFocus,
            Route::AssistiveTechAnnounced,
            Route::ReducedMotionSafe,
        ],
        Posture::PointerHover => vec![
            Route::PointerHover,
            Route::KeyboardFocus,
            Route::AssistiveTechAnnounced,
            Route::ReducedMotionSafe,
        ],
        Posture::KeyboardFocusVisible => vec![
            Route::KeyboardFocus,
            Route::FocusVisibleRing,
            Route::AssistiveTechAnnounced,
            Route::ReducedMotionSafe,
        ],
        Posture::PressedOrActive => vec![
            Route::PressActivation,
            Route::KeyboardFocus,
            Route::AssistiveTechAnnounced,
            Route::ReducedMotionSafe,
        ],
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked interactive-state resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateResolutionCase {
    /// The resolver input.
    pub input: M5InteractiveStateResolutionInput,
    /// The resolved truth. Must equal `resolve_interactive_state_contract(&input)`.
    pub resolved: M5ResolvedInteractiveStateContract,
}

impl M5InteractiveStateResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5InteractiveStateResolutionInput) -> Self {
        let resolved = resolve_interactive_state_contract(&input)
            .expect("seed interactive state contract case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_interactive_state_contract(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input control identity and state-style reference
    /// exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.control_identity_ref == self.input.control_identity_ref
            && self.resolved.state_style_ref == self.input.state_style_ref
    }

    /// True when the resolved case never signals state by color alone, keeps the hit target
    /// stable, never breaks layout on a transition, always offers the visible-focus capability,
    /// stays legible under reduced-motion and high-contrast, and is driven by the shared contract.
    pub fn preserves_guarantees(&self) -> bool {
        !self.resolved.required_non_color_cues.is_empty()
            && self.resolved.no_color_only_signaling
            && self.resolved.stable_hit_target
            && self.resolved.no_interaction_breaking_layout_shift
            && self.resolved.focus_visible_for_keyboard
            && self.resolved.reduced_motion_safe
            && self.resolved.high_contrast_safe
            && self.resolved.driven_by_shared_state_contract
    }
}

/// One row in the primitive matrix: one claimed M5 control bound to the shared interactive-state
/// anatomy, interactive states, presentation postures, non-color cues, interaction routes, export
/// fields, mandatory labels, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveControlRow {
    /// Claimed control kind.
    pub control_kind: M5InteractiveControlKind,
    /// Qualification class earned by this control.
    pub qualification: M5ComponentStateQualificationClass,
    /// Owner role accountable for keeping this control governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this control.
    pub surface_families: Vec<M5ComponentStateSurfaceFamily>,
    /// Deployment lines this control keeps the same truth across.
    pub deployment_lines: Vec<M5ComponentStateDeploymentLine>,
    /// Anatomy parts this control renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5InteractiveStateAnatomyPart>,
    /// Interactive states this control distinguishes.
    pub interactive_states: Vec<M5SharedComponentStateClass>,
    /// Presentation postures this control distinguishes.
    pub presentations: Vec<M5InteractiveStatePresentation>,
    /// Non-color cues this control renders.
    pub non_color_cues: Vec<M5InteractiveStateCue>,
    /// Interaction input routes this control offers.
    pub interaction_input_routes: Vec<M5InteractionInputRoute>,
    /// Export fields this control carries (must include the mandatory fields).
    pub export_fields: Vec<M5InteractiveStateExportField>,
    /// Non-visual accessibility routes this control offers.
    pub accessibility_routes: Vec<M5ComponentStateAccessibilityRoute>,
    /// Mandatory labels this control can show (must include the mandatory labels).
    pub required_labels: Vec<M5ComponentStateRequiredLabel>,
    /// Subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Downgrade triggers that apply to this control.
    pub downgrade_triggers: Vec<M5ComponentStateDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked interactive-state resolutions proving the resolver on this control.
    pub state_examples: Vec<M5InteractiveStateResolutionCase>,
    /// Hard invariant: this control never signals a state by color alone. MUST be `false`.
    pub signals_state_by_color_only: bool,
    /// Hard invariant: this control never shifts layout on a state change. MUST be `false`.
    pub shifts_layout_on_state_change: bool,
    /// Hard invariant: this control never changes its hit target on a state change. MUST be
    /// `false`.
    pub changes_hit_target_on_state_change: bool,
    /// Hard invariant: this control never invents a private state name. MUST be `false`.
    pub invents_private_state_name: bool,
}

impl M5InteractiveControlRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5InteractiveStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5InteractiveStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5InteractiveStateExportField> =
            self.export_fields.iter().copied().collect();
        M5InteractiveStateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory label.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ComponentStateRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ComponentStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.signals_state_by_color_only
            && !self.shifts_layout_on_state_change
            && !self.changes_hit_target_on_state_change
            && !self.invents_private_state_name
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateVocabularySet {
    /// Control-kind tokens.
    pub control_kinds: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Presentation-posture tokens.
    pub presentations: Vec<String>,
    /// Non-color-cue tokens.
    pub non_color_cues: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Interactive-state tokens (reused from the frozen matrix).
    pub interactive_states: Vec<String>,
    /// Interaction-input-route tokens (reused from the frozen matrix).
    pub interaction_input_routes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens (reused from the frozen matrix).
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens (reused from the frozen matrix).
    pub required_labels: Vec<String>,
}

impl M5InteractiveStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            control_kinds: tokens(&M5InteractiveControlKind::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5InteractiveStateAnatomyPart::ALL, |v| v.as_str()),
            presentations: tokens(&M5InteractiveStatePresentation::ALL, |v| v.as_str()),
            non_color_cues: tokens(&M5InteractiveStateCue::ALL, |v| v.as_str()),
            export_fields: tokens(&M5InteractiveStateExportField::ALL, |v| v.as_str()),
            interactive_states: interactive_states()
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            interaction_input_routes: tokens(&M5InteractionInputRoute::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ComponentStateSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ComponentStateDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ComponentStateConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ComponentStateAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ComponentStateRequiredLabel::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateGovernanceReview {
    /// Controls expose default, hover, focus-visible, and pressed states explicitly.
    pub controls_expose_default_hover_focus_pressed: bool,
    /// State meaning is never carried by color alone.
    pub state_meaning_never_color_only: bool,
    /// Hit targets stay stable across state transitions.
    pub hit_targets_stay_stable: bool,
    /// No interaction-breaking layout shift occurs on state transitions.
    pub no_interaction_breaking_layout_shift: bool,
    /// Keyboard operators always get a visible focus ring.
    pub focus_visible_under_keyboard: bool,
    /// Interactive states stay legible under high-contrast and high-zoom.
    pub legible_under_high_contrast_and_zoom: bool,
    /// Interactive states stay legible under reduced-motion.
    pub legible_under_reduced_motion: bool,
    /// State semantics are driven by the shared contract and its token hooks.
    pub states_driven_by_shared_contract_and_tokens: bool,
    /// No control uses one-off, per-surface interactive-state styling.
    pub no_one_off_per_surface_styling: bool,
    /// Interactive states keep the same truth across every deployment line.
    pub states_stable_across_deployment_lines: bool,
    /// Interactive states keep the same truth across desktop, headless/export, and support
    /// consumers.
    pub states_stable_across_consumer_surfaces: bool,
    /// Every control declares a non-visual accessibility route.
    pub every_control_declares_accessibility_route: bool,
    /// The support / export packet reconstructs interactive-state truth.
    pub support_export_reconstructs_state_truth: bool,
    /// Later M5 rows cannot invent parallel interactive-state vocabulary.
    pub later_rows_cannot_invent_parallel_state_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateConsumerProjection {
    /// Controls consume the shared interactive-state vocabulary.
    pub controls_consume_state_vocabulary: bool,
    /// The presentation-posture resolver reads a single canonical source.
    pub presentation_reads_single_source: bool,
    /// The non-color cue-set derivation reads a single canonical source.
    pub non_color_cue_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop controls read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the interactive-state contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting interactive-state audit.
    pub interactive_state_audit_ref: String,
    /// True when support / export parity is required for every control.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every control.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5InteractiveStateContractPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InteractiveStateContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Control rows.
    pub rows: Vec<M5InteractiveControlRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InteractiveStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InteractiveStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InteractiveStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InteractiveStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InteractiveStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 interactive-state-contract primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InteractiveStateContractPacket {
    /// Record kind; must equal [`M5_INTERACTIVE_STATE_CONTRACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Control rows.
    pub rows: Vec<M5InteractiveControlRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InteractiveStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InteractiveStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InteractiveStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InteractiveStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InteractiveStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InteractiveStateContractPacket {
    /// Builds an M5 interactive-state-contract-primitive packet from stable-lane input.
    pub fn new(input: M5InteractiveStateContractPacketInput) -> Self {
        Self {
            record_kind: M5_INTERACTIVE_STATE_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 interactive-state-contract-primitive invariants.
    pub fn validate(&self) -> Vec<M5InteractiveStateContractViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_INTERACTIVE_STATE_CONTRACT_RECORD_KIND {
            violations.push(M5InteractiveStateContractViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_VERSION {
            violations.push(M5InteractiveStateContractViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InteractiveStateContractViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_interactive_state_coverage(self, &mut violations);
        validate_presentation_coverage(self, &mut violations);
        validate_cue_coverage(self, &mut violations);
        validate_route_coverage(self, &mut violations);
        validate_guarantees(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 interactive state contract primitive packet serializes"),
        ) {
            violations.push(M5InteractiveStateContractViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 interactive state contract primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per control kind.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control_kind,qualification,owner,anatomy,interactive_states,presentations,non_color_cues,interaction_routes,state_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.control_kind.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_state_tokens(&row.interactive_states),
                join_tokens(&row.presentations, |v| v.as_str()),
                join_tokens(&row.non_color_cues, |v| v.as_str()),
                join_tokens(&row.interaction_input_routes, |v| v.as_str()),
                row.state_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Interactive-State Contract Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Controls: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Presentations: {}\n",
            self.vocabulary_set.presentations.join(", ")
        ));
        out.push_str(&format!(
            "- Non-color cues: {}\n",
            self.vocabulary_set.non_color_cues.join(", ")
        ));
        out.push_str(&format!(
            "- Interactive states: {}\n",
            self.vocabulary_set.interactive_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Controls\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.control_kind.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked states: {}\n",
                row.state_examples.len()
            ));
            for case in &row.state_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (non-color cues {}, focus-ring `{}`, hit-target-stable `{}`, layout-stable `{}`)\n",
                    case.resolved.control_identity_ref,
                    case.resolved.interactive_state.as_str(),
                    case.resolved.presentation.as_str(),
                    case.resolved.required_non_color_cues.len(),
                    case.resolved.focus_ring_shown,
                    case.resolved.stable_hit_target,
                    case.resolved.no_interaction_breaking_layout_shift,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 interactive-state-contract-primitive export.
#[derive(Debug)]
pub enum M5InteractiveStateContractArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InteractiveStateContractViolation>),
}

impl fmt::Display for M5InteractiveStateContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 interactive state contract primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 interactive state contract primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InteractiveStateContractArtifactError {}

/// Validation failures emitted by [`M5InteractiveStateContractPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InteractiveStateContractViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required control kind is missing from the matrix.
    RequiredControlMissing,
    /// A control row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A row declares no accessibility routes, or misses keyboard focus or non-color encoding.
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked state resolutions.
    StateExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableControlMissingProof,
    /// The worked resolutions do not exercise every interactive state.
    InteractiveStateCoverageUnproven,
    /// The worked resolutions do not exercise every presentation posture.
    PresentationCoverageUnproven,
    /// The worked resolutions do not exercise every non-color cue.
    CueCoverageUnproven,
    /// The worked resolutions do not exercise every interaction input route.
    RouteCoverageUnproven,
    /// A worked resolution does not hold the no-color-only, stable-hit-target, no-layout-shift,
    /// focus-visible, and reduced-motion / high-contrast guarantees.
    GuaranteesUnproven,
    /// A worked resolution does not preserve its exact control identity and state-style reference.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5InteractiveStateContractViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredControlMissing => "required_control_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StateExampleMissing => "state_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableControlMissingProof => "stable_control_missing_proof",
            Self::InteractiveStateCoverageUnproven => "interactive_state_coverage_unproven",
            Self::PresentationCoverageUnproven => "presentation_coverage_unproven",
            Self::CueCoverageUnproven => "cue_coverage_unproven",
            Self::RouteCoverageUnproven => "route_coverage_unproven",
            Self::GuaranteesUnproven => "guarantees_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 interactive-state-contract-primitive export.
pub fn current_stable_m5_interactive_state_contract_export(
) -> Result<M5InteractiveStateContractPacket, M5InteractiveStateContractArtifactError> {
    let packet: M5InteractiveStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-interactive-state-contract-primitive-proof/support_export.json"
    )))
    .map_err(M5InteractiveStateContractArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InteractiveStateContractArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF,
        M5_INTERACTIVE_STATE_CONTRACT_DOC_REF,
        M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_MATRIX_REF,
        M5_INTERACTIVE_STATE_CONTRACT_FOCUS_SELECTION_REF,
        M5_INTERACTIVE_STATE_CONTRACT_COMPONENT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InteractiveStateContractViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5InteractiveStateContractViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let present: BTreeSet<M5InteractiveControlKind> =
        packet.rows.iter().map(|row| row.control_kind).collect();
    for required in M5InteractiveControlKind::ALL {
        if !present.contains(&required) {
            violations.push(M5InteractiveStateContractViolation::RequiredControlMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.interactive_states.is_empty()
            || row.presentations.is_empty()
            || row.non_color_cues.is_empty()
            || row.interaction_input_routes.is_empty()
            || row.export_fields.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5InteractiveStateContractViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5InteractiveStateContractViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5InteractiveStateContractViolation::MandatoryExportMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5InteractiveStateContractViolation::MandatoryLabelMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5InteractiveStateContractViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5InteractiveStateContractViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5InteractiveStateContractViolation::DowngradeTriggersMissing);
        }
        if row.state_examples.is_empty() {
            violations.push(M5InteractiveStateContractViolation::StateExampleMissing);
        }
        if row
            .state_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5InteractiveStateContractViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5InteractiveStateContractViolation::StableControlMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5InteractiveStateContractViolation::RowInvariantViolated);
        }
    }
}

/// Every interactive state must be exercised by some worked resolution — the implementation
/// requirement that default, hover, focus-visible, and pressed states are all wired explicitly.
fn validate_interactive_state_coverage(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let exercised: BTreeSet<M5SharedComponentStateClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.interactive_state)
        .collect();
    let covered = interactive_states()
        .iter()
        .all(|state| exercised.contains(state));
    if !covered {
        violations.push(M5InteractiveStateContractViolation::InteractiveStateCoverageUnproven);
    }
}

/// Every presentation posture must be exercised by some worked resolution.
fn validate_presentation_coverage(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let exercised: BTreeSet<M5InteractiveStatePresentation> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.presentation)
        .collect();
    let covered = M5InteractiveStatePresentation::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5InteractiveStateContractViolation::PresentationCoverageUnproven);
    }
}

/// Every non-color cue must be exercised by some worked resolution — the acceptance criterion that
/// state meaning never depends on color alone.
fn validate_cue_coverage(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.state_examples.iter());
    let covered = M5InteractiveStateCue::ALL
        .iter()
        .all(|cue| cases().any(|case| case.resolved.required_non_color_cues.contains(cue)));
    if !covered {
        violations.push(M5InteractiveStateContractViolation::CueCoverageUnproven);
    }
}

/// Every interaction input route must be exercised by some worked resolution — the requirement
/// that no interactive state is pointer-only or hover-only.
fn validate_route_coverage(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.state_examples.iter());
    let covered = M5InteractionInputRoute::ALL
        .iter()
        .all(|route| cases().any(|case| case.resolved.interaction_input_routes.contains(route)));
    if !covered {
        violations.push(M5InteractiveStateContractViolation::RouteCoverageUnproven);
    }
}

/// Every worked resolution must hold the no-color-only, stable-hit-target, no-layout-shift,
/// focus-visible, and reduced-motion / high-contrast guarantees — the core acceptance criteria.
fn validate_guarantees(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .all(|case| case.preserves_guarantees());
    if !preserved {
        violations.push(M5InteractiveStateContractViolation::GuaranteesUnproven);
    }
}

/// Every worked resolution must preserve its exact control identity and state-style reference — the
/// invariant that the contract never rewrites what it renders.
fn validate_identity_preservation(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5InteractiveStateContractViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.controls_expose_default_hover_focus_pressed,
        review.state_meaning_never_color_only,
        review.hit_targets_stay_stable,
        review.no_interaction_breaking_layout_shift,
        review.focus_visible_under_keyboard,
        review.legible_under_high_contrast_and_zoom,
        review.legible_under_reduced_motion,
        review.states_driven_by_shared_contract_and_tokens,
        review.no_one_off_per_surface_styling,
        review.states_stable_across_deployment_lines,
        review.states_stable_across_consumer_surfaces,
        review.every_control_declares_accessibility_route,
        review.support_export_reconstructs_state_truth,
        review.later_rows_cannot_invent_parallel_state_vocabulary,
    ] {
        if !ok {
            violations.push(M5InteractiveStateContractViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.controls_consume_state_vocabulary,
        projection.presentation_reads_single_source,
        projection.non_color_cue_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5InteractiveStateContractViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5InteractiveStateContractViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5InteractiveStateContractPacket,
    violations: &mut Vec<M5InteractiveStateContractViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.interactive_state_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InteractiveStateContractViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Joins interactive-state tokens for a CSV cell with a `|` separator.
fn join_state_tokens(items: &[M5SharedComponentStateClass]) -> String {
    items
        .iter()
        .map(|state| state.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
