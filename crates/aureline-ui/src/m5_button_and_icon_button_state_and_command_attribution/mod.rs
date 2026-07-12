//! Implemented M5 button and icon-button primitives.
//!
//! The frozen [core action / input component matrix][matrix] names Aureline's most reused atomic
//! action and input controls and locks their controlled vocabulary. This module is the first
//! implement lane over that matrix: it turns the two action-trigger components — the **button** and
//! the **icon button** — into resolvers that produce export-safe, honest projections, so a user can
//! trust that an action trigger means the same thing, looks appropriately risky, and stays
//! inspectable whether it appears in a pane header, review sheet, settings row, start center, or
//! support flow.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement button variants for primary, secondary, tertiary/quiet, destructive, and ghost-icon
//!   use with stable token/state wiring and no feature-local style forks.** [`resolve_button`]
//!   refuses to read as a clean, attributable trigger when the action label is unstated, the surface
//!   context or loading behavior is unresolved, a feature-local style is forked instead of reusing the
//!   shared emphasis grammar, emphasis is encoded by color alone, or a locked/degraded state hides
//!   behind generic disabled chrome; it degrades instead.
//! * **Preserve button width and primary label during loading/pending states so the in-flight action
//!   remains attributable.** [`resolve_button`] degrades with `loading_relabeled_or_resized` whenever
//!   a loading button relabels the action or changes width enough to lose attribution.
//! * **Require accessible names, tooltip parity, and canonical command IDs for icon-only buttons,
//!   including context-menu / help / palette alignment where the same action appears elsewhere.**
//!   [`resolve_icon_button`] degrades when the accessible name is unstated, the label mode or command
//!   surface is unresolved, a brand-only affordance is invented, an icon-only destructive action is
//!   left unlabeled, tooltip parity is missing, the canonical command ID is unstated, or command
//!   parity across the context menu / palette / help surfaces is broken.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5CoreControlDisposition`] interaction-state vocabulary, the [`M5ButtonEmphasis`] emphasis
//! vocabulary, and the [`M5IconLabelMode`] icon-label vocabulary — so forms, settings, search, entry,
//! review, repair, and support surfaces can never fork their own action-state or icon-label wording or
//! invent a surface-local style for the same action. Raw secret values and private endpoints stay
//! outside the export boundary.
//!
//! [matrix]: crate::m5_core_action_input_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_button_icon_button_controls,
    seeded_m5_button_icon_button_controls_forms_ui_beta_narrowed,
    seeded_m5_button_icon_button_controls_review_ui_preview_narrowed,
    M5_BUTTON_ICON_BUTTON_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_core_action_input_component_matrix::{
    M5ButtonEmphasis, M5CoreControlAccessibilityRoute, M5CoreControlConsumerSurface,
    M5CoreControlDeploymentLine, M5CoreControlDisposition, M5CoreControlDowngradeTrigger,
    M5CoreControlFamily, M5CoreControlQualificationClass, M5CoreControlRequiredLabel,
    M5IconLabelMode, M5_BUTTON_SCHEMA_REF, M5_CORE_CONTROL_COMPONENT_DOC_REF,
    M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_ICON_BUTTON_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ButtonIconButtonControlsPacket`].
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_RECORD_KIND: &str =
    "implement_m5_button_and_icon_button_controls";

/// Schema version for M5 button / icon-button controls records.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-button-icon-button-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_DOC_REF: &str =
    "docs/components/m5_button_and_icon_button_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-button-icon-button-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-button-icon-button-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-button-icon-button-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUTTON_ICON_BUTTON_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-button-icon-button-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5ButtonIconButtonConsumerSurface = M5CoreControlConsumerSurface;

/// Controlled loading / pending behavior a button names, so an in-flight action never relabels the
/// action or changes width enough to lose attribution. Minted by this lane because the frozen matrix
/// carries the `loading` interaction *state* but not the width / label preservation posture the
/// button acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ButtonLoadingBehavior {
    /// The button is at rest; no in-flight action.
    NotLoading,
    /// The label is preserved with a leading spinner; width is held.
    LabelPreservedSpinnerLeading,
    /// The label is preserved with a trailing spinner; width is held.
    LabelPreservedSpinnerTrailing,
    /// The label is preserved with an inline progress indicator; width is held.
    InlineProgressLabelKept,
    /// Width is reserved and the label is kept while the control is non-interactive.
    WidthReservedLabelKept,
    /// The loading behavior cannot currently be resolved.
    BehaviorUnknown,
}

impl M5ButtonLoadingBehavior {
    /// Every loading behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotLoading,
        Self::LabelPreservedSpinnerLeading,
        Self::LabelPreservedSpinnerTrailing,
        Self::InlineProgressLabelKept,
        Self::WidthReservedLabelKept,
        Self::BehaviorUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotLoading => "not_loading",
            Self::LabelPreservedSpinnerLeading => "label_preserved_spinner_leading",
            Self::LabelPreservedSpinnerTrailing => "label_preserved_spinner_trailing",
            Self::InlineProgressLabelKept => "inline_progress_label_kept",
            Self::WidthReservedLabelKept => "width_reserved_label_kept",
            Self::BehaviorUnknown => "behavior_unknown",
        }
    }

    /// Whether the button is in an in-flight / pending state.
    pub const fn is_loading(self) -> bool {
        matches!(
            self,
            Self::LabelPreservedSpinnerLeading
                | Self::LabelPreservedSpinnerTrailing
                | Self::InlineProgressLabelKept
                | Self::WidthReservedLabelKept
        )
    }

    /// Whether the loading behavior is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::BehaviorUnknown)
    }
}

/// Controlled command surface an icon-only action aligns its canonical command ID across, so the same
/// action means the same thing in the inline trigger, the context menu, the command palette, the help
/// reference, and the keyboard shortcut. Minted by this lane because the frozen matrix carries no
/// command-parity surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActionCommandSurface {
    /// The inline icon-button trigger itself.
    InlineTrigger,
    /// The context menu entry for the same command.
    ContextMenu,
    /// The command-palette entry for the same command.
    CommandPalette,
    /// The help / documentation reference for the same command.
    HelpReference,
    /// The keyboard-shortcut binding for the same command.
    KeyboardShortcut,
    /// The command surface cannot currently be resolved.
    SurfaceUnknown,
}

impl M5ActionCommandSurface {
    /// Every command surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InlineTrigger,
        Self::ContextMenu,
        Self::CommandPalette,
        Self::HelpReference,
        Self::KeyboardShortcut,
        Self::SurfaceUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineTrigger => "inline_trigger",
            Self::ContextMenu => "context_menu",
            Self::CommandPalette => "command_palette",
            Self::HelpReference => "help_reference",
            Self::KeyboardShortcut => "keyboard_shortcut",
            Self::SurfaceUnknown => "surface_unknown",
        }
    }

    /// Whether the command surface is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::SurfaceUnknown)
    }
}

/// Controlled render context — which claimed M5 surface renders the action trigger, so a trigger's
/// meaning stays stable whether it appears in a pane header, review sheet, settings row, start center,
/// or support flow. Minted by this lane, tracking the exit-gate anchor surfaces directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ActionSurfaceContext {
    /// A pane / panel header.
    PaneHeader,
    /// A review sheet.
    ReviewSheet,
    /// A settings row.
    SettingsRow,
    /// The start-center entry surface.
    StartCenter,
    /// A support / recovery flow.
    SupportFlow,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ActionSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PaneHeader,
        Self::ReviewSheet,
        Self::SettingsRow,
        Self::StartCenter,
        Self::SupportFlow,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PaneHeader => "pane_header",
            Self::ReviewSheet => "review_sheet",
            Self::SettingsRow => "settings_row",
            Self::StartCenter => "start_center",
            Self::SupportFlow => "support_flow",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a button or icon button must be able to show, so no action, state, or
/// command fact is left implicit behind loading chrome, a tooltip, or a secondary panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ButtonIconButtonAnatomyPart {
    /// The component's stable identity / permanent label.
    Identity,
    /// The component's current typed interaction disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The button emphasis (button).
    ButtonEmphasis,
    /// The loading / pending behavior preserving width and label (button).
    LoadingBehavior,
    /// The render / surface context (both components).
    SurfaceContext,
    /// The accessible name (icon button).
    AccessibleName,
    /// The icon-label mode (icon button).
    IconLabelMode,
    /// The tooltip parity with the accessible name (icon button).
    TooltipParity,
    /// The canonical command binding (both components).
    CommandBinding,
    /// The command parity across menu / palette / help surfaces (icon button).
    CommandParity,
}

impl M5ButtonIconButtonAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ButtonEmphasis,
        Self::LoadingBehavior,
        Self::SurfaceContext,
        Self::AccessibleName,
        Self::IconLabelMode,
        Self::TooltipParity,
        Self::CommandBinding,
        Self::CommandParity,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ButtonEmphasis => "button_emphasis",
            Self::LoadingBehavior => "loading_behavior",
            Self::SurfaceContext => "surface_context",
            Self::AccessibleName => "accessible_name",
            Self::IconLabelMode => "icon_label_mode",
            Self::TooltipParity => "tooltip_parity",
            Self::CommandBinding => "command_binding",
            Self::CommandParity => "command_parity",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to inspect the
/// action, state, or command behind a degraded trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ButtonIconButtonNextAction {
    /// Open the command-backed action / command detail.
    OpenCommandDetail,
    /// Inspect the button state / emphasis.
    InspectButtonState,
    /// Inspect the icon-button accessible name / label mode.
    InspectIconButton,
    /// Review a locked / blocked / disabled trigger.
    ReviewBlockedOrLocked,
    /// Review command parity across menu / palette / help surfaces.
    ReviewCommandParity,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5ButtonIconButtonNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenCommandDetail,
        Self::InspectButtonState,
        Self::InspectIconButton,
        Self::ReviewBlockedOrLocked,
        Self::ReviewCommandParity,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCommandDetail => "open_command_detail",
            Self::InspectButtonState => "inspect_button_state",
            Self::InspectIconButton => "inspect_icon_button",
            Self::ReviewBlockedOrLocked => "review_blocked_or_locked",
            Self::ReviewCommandParity => "review_command_parity",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ButtonIconButtonExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The interaction dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The button emphasis named by the button.
    ButtonEmphasis,
    /// The loading behavior named by the button.
    LoadingBehavior,
    /// The icon-label mode named by the icon button.
    IconLabelMode,
    /// The render / surface context named by both components.
    SurfaceContext,
    /// The command surface aligned by the icon button.
    CommandSurface,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ButtonIconButtonExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ButtonEmphasis,
        Self::LoadingBehavior,
        Self::IconLabelMode,
        Self::SurfaceContext,
        Self::CommandSurface,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::ButtonEmphasis => "button_emphasis",
            Self::LoadingBehavior => "loading_behavior",
            Self::IconLabelMode => "icon_label_mode",
            Self::SurfaceContext => "surface_context",
            Self::CommandSurface => "command_surface",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a button degraded below a clean, attributable state. The degrade-first ladder returns one
/// of these instead of ever letting an ambiguous trigger read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ButtonDegradeReason {
    /// The action label is unstated; a user cannot tell what the trigger does.
    ActionLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The loading / pending behavior cannot currently be resolved.
    LoadingBehaviorUnresolved,
    /// A feature-local style was forked instead of reusing the shared emphasis grammar.
    FeatureLocalStyleForked,
    /// The emphasis is encoded by color alone rather than named.
    EmphasisEncodedByColorAlone,
    /// A loading button relabeled the action or changed width enough to lose attribution.
    LoadingRelabeledOrResized,
    /// A locked / degraded state hides behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the action is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ButtonDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ActionLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::LoadingBehaviorUnresolved,
        Self::FeatureLocalStyleForked,
        Self::EmphasisEncodedByColorAlone,
        Self::LoadingRelabeledOrResized,
        Self::LockedOrDegradedHiddenBehindDisabled,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionLabelUnstated => "action_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LoadingBehaviorUnresolved => "loading_behavior_unresolved",
            Self::FeatureLocalStyleForked => "feature_local_style_forked",
            Self::EmphasisEncodedByColorAlone => "emphasis_encoded_by_color_alone",
            Self::LoadingRelabeledOrResized => "loading_relabeled_or_resized",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
            }
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ButtonIconButtonNextAction {
        match self {
            Self::ActionLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::LoadingBehaviorUnresolved
            | Self::EmphasisEncodedByColorAlone => M5ButtonIconButtonNextAction::InspectButtonState,
            Self::FeatureLocalStyleForked
            | Self::CommandBindingUnstated
            | Self::CommandTracePathMissing => M5ButtonIconButtonNextAction::OpenCommandDetail,
            Self::LoadingRelabeledOrResized | Self::LockedOrDegradedHiddenBehindDisabled => {
                M5ButtonIconButtonNextAction::ReviewBlockedOrLocked
            }
            Self::ProofStale => M5ButtonIconButtonNextAction::ReviewCommandParity,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::ActionLabelUnstated => M5CoreControlDowngradeTrigger::PlaceholderUsedAsLabel,
            Self::LoadingRelabeledOrResized => {
                M5CoreControlDowngradeTrigger::LoadingRelabeledOrResized
            }
            Self::LockedOrDegradedHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::FeatureLocalStyleForked | Self::EmphasisEncodedByColorAlone => {
                M5CoreControlDowngradeTrigger::StateTaxonomyDrifted
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SurfaceContextUnresolved | Self::LoadingBehaviorUnresolved => {
                M5CoreControlDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// Reason an icon button degraded below a clean, labeled, command-parity-preserving state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconButtonDegradeReason {
    /// The accessible name is unstated; the icon-only control is unlabeled.
    AccessibleNameUnstated,
    /// The icon-label mode cannot currently be resolved.
    LabelModeUnresolved,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The command surface cannot currently be resolved.
    CommandSurfaceUnresolved,
    /// A brand-only affordance was invented instead of a labeled, command-backed control.
    BrandOnlyAffordanceInvented,
    /// An icon-only destructive action was left unlabeled.
    IconOnlyDestructiveUnlabeled,
    /// The tooltip does not match the accessible name.
    TooltipParityMissing,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// Command parity across the context menu / palette / help surfaces is broken.
    CommandParityBrokenAcrossSurfaces,
    /// No command-backed path to inspect the action is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5IconButtonDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::AccessibleNameUnstated,
        Self::LabelModeUnresolved,
        Self::SurfaceContextUnresolved,
        Self::CommandSurfaceUnresolved,
        Self::BrandOnlyAffordanceInvented,
        Self::IconOnlyDestructiveUnlabeled,
        Self::TooltipParityMissing,
        Self::CommandBindingUnstated,
        Self::CommandParityBrokenAcrossSurfaces,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessibleNameUnstated => "accessible_name_unstated",
            Self::LabelModeUnresolved => "label_mode_unresolved",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CommandSurfaceUnresolved => "command_surface_unresolved",
            Self::BrandOnlyAffordanceInvented => "brand_only_affordance_invented",
            Self::IconOnlyDestructiveUnlabeled => "icon_only_destructive_unlabeled",
            Self::TooltipParityMissing => "tooltip_parity_missing",
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandParityBrokenAcrossSurfaces => "command_parity_broken_across_surfaces",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ButtonIconButtonNextAction {
        match self {
            Self::AccessibleNameUnstated
            | Self::LabelModeUnresolved
            | Self::SurfaceContextUnresolved => M5ButtonIconButtonNextAction::InspectIconButton,
            Self::CommandSurfaceUnresolved
            | Self::CommandBindingUnstated
            | Self::CommandParityBrokenAcrossSurfaces => {
                M5ButtonIconButtonNextAction::ReviewCommandParity
            }
            Self::BrandOnlyAffordanceInvented
            | Self::IconOnlyDestructiveUnlabeled
            | Self::TooltipParityMissing => M5ButtonIconButtonNextAction::ReviewBlockedOrLocked,
            Self::CommandTracePathMissing => M5ButtonIconButtonNextAction::OpenCommandDetail,
            Self::ProofStale => M5ButtonIconButtonNextAction::OpenCommandDetail,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::IconOnlyDestructiveUnlabeled => {
                M5CoreControlDowngradeTrigger::IconOnlyDestructiveUnlabeled
            }
            Self::BrandOnlyAffordanceInvented => {
                M5CoreControlDowngradeTrigger::StateTaxonomyDrifted
            }
            Self::CommandSurfaceUnresolved
            | Self::CommandBindingUnstated
            | Self::CommandParityBrokenAcrossSurfaces
            | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::AccessibleNameUnstated
            | Self::LabelModeUnresolved
            | Self::SurfaceContextUnresolved
            | Self::TooltipParityMissing => M5CoreControlDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// True when a disposition names a locked / degraded state that must never hide behind generic
/// disabled chrome.
fn disposition_is_locked_or_degraded(disposition: M5CoreControlDisposition) -> bool {
    matches!(
        disposition,
        M5CoreControlDisposition::Locked | M5CoreControlDisposition::Degraded
    )
}

/// True when an icon-label mode carries a real accessible name (never a decorative-only glyph or an
/// unresolved mode).
fn icon_label_carries_accessible_name(mode: M5IconLabelMode) -> bool {
    !matches!(
        mode,
        M5IconLabelMode::DecorativeOnly | M5IconLabelMode::LabelUnresolved
    )
}

/// Input to [`resolve_button`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ButtonResolutionInput {
    /// Stable identity of the button instance.
    pub button_id: String,
    /// The action label shown; empty means unstated.
    pub action_label: String,
    /// The button emphasis (from the frozen matrix vocabulary).
    pub emphasis: M5ButtonEmphasis,
    /// True when the emphasis is stated non-color-only (weight / label, never color alone).
    pub emphasis_stated: bool,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// The render / surface context.
    pub surface_context: M5ActionSurfaceContext,
    /// The loading / pending behavior.
    pub loading_behavior: M5ButtonLoadingBehavior,
    /// True when a loading button preserves its primary label and width, keeping attribution.
    pub loading_preserves_label_and_width: bool,
    /// True when a locked / degraded state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// The canonical command ID this button binds back to; empty means unstated.
    pub command_id: String,
    /// True when the button forks a feature-local style instead of reusing the shared emphasis
    /// grammar.
    pub forks_feature_local_style: bool,
    /// True when a command-backed path to inspect the action is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe button projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedButton {
    /// Stable identity of the button instance.
    pub button_id: String,
    /// The action label named by the button.
    pub action_label: String,
    /// The button-emphasis token named by the button.
    pub emphasis: String,
    /// Whether the emphasis names a destructive action.
    pub emphasis_is_destructive: bool,
    /// The interaction-disposition token named by the button.
    pub disposition: String,
    /// Whether the disposition names a locked / degraded state.
    pub disposition_is_blocked: bool,
    /// The render / surface-context token named by the button.
    pub surface_context: String,
    /// The loading-behavior token named by the button.
    pub loading_behavior: String,
    /// Whether the button is in an in-flight / pending state.
    pub is_loading: bool,
    /// Whether a loading button preserves its primary label and width.
    pub loading_preserves_label_and_width: bool,
    /// Whether a locked / degraded state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// The canonical command ID named by the button.
    pub command_id: String,
    /// Guardrail (MUST be `false` on a clean button): a feature-local style was forked.
    pub forks_feature_local_style: bool,
    /// Whether a command-backed path to inspect the action is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the button could not read as a clean, attributable state.
    pub degrade_reason: Option<M5ButtonDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ButtonIconButtonNextAction,
    /// Whether the action is attributable at a glance (clean button naming every fact).
    pub action_attributable_at_a_glance: bool,
}

impl M5ResolvedButton {
    /// Whether this button reads as a clean, attributable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_icon_button`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5IconButtonResolutionInput {
    /// Stable identity of the icon-button instance.
    pub icon_button_id: String,
    /// The accessible name shown; empty means unstated.
    pub accessible_name: String,
    /// The icon-label mode (from the frozen matrix vocabulary).
    pub label_mode: M5IconLabelMode,
    /// The button emphasis (from the frozen matrix vocabulary).
    pub emphasis: M5ButtonEmphasis,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// The render / surface context.
    pub surface_context: M5ActionSurfaceContext,
    /// The command surface this icon aligns its canonical command ID across.
    pub command_surface: M5ActionCommandSurface,
    /// True when the tooltip matches the accessible name.
    pub tooltip_parity: bool,
    /// The canonical command ID this icon binds back to; empty means unstated.
    pub command_id: String,
    /// True when command parity holds across the context menu / palette / help surfaces.
    pub command_parity_across_surfaces: bool,
    /// True when the icon invents a brand-only affordance instead of a labeled, command-backed
    /// control.
    pub invents_brand_only_affordance: bool,
    /// True when a command-backed path to inspect the action is reachable, never brand-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe icon-button projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedIconButton {
    /// Stable identity of the icon-button instance.
    pub icon_button_id: String,
    /// The accessible name named by the icon button.
    pub accessible_name: String,
    /// The icon-label-mode token named by the icon button.
    pub label_mode: String,
    /// Whether the icon button exposes a real accessible name (never decorative-only / unresolved).
    pub exposes_accessible_name: bool,
    /// The button-emphasis token named by the icon button.
    pub emphasis: String,
    /// Whether the emphasis names a destructive action.
    pub emphasis_is_destructive: bool,
    /// The interaction-disposition token named by the icon button.
    pub disposition: String,
    /// The render / surface-context token named by the icon button.
    pub surface_context: String,
    /// The command-surface token named by the icon button.
    pub command_surface: String,
    /// Whether the tooltip matches the accessible name.
    pub tooltip_parity: bool,
    /// The canonical command ID named by the icon button.
    pub command_id: String,
    /// Whether command parity holds across the context menu / palette / help surfaces.
    pub command_parity_across_surfaces: bool,
    /// Guardrail (MUST be `false` on a clean icon button): a brand-only affordance was invented.
    pub invents_brand_only_affordance: bool,
    /// Whether a command-backed path to inspect the action is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the icon button could not read as a clean, labeled state.
    pub degrade_reason: Option<M5IconButtonDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ButtonIconButtonNextAction,
    /// Whether the name and command are legible at a glance (clean icon button naming every fact).
    pub name_and_command_legible_at_a_glance: bool,
}

impl M5ResolvedIconButton {
    /// Whether this icon button reads as a clean, labeled state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ButtonIconButtonResolutionError {
    /// The button id was empty.
    EmptyButtonId,
    /// The icon-button id was empty.
    EmptyIconButtonId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ButtonIconButtonResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyButtonId => "empty_button_id",
            Self::EmptyIconButtonId => "empty_icon_button_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ButtonIconButtonResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 button / icon-button resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ButtonIconButtonResolutionError {}

/// Resolves a button so its action is attributable at a glance: the button names its permanent action
/// label, emphasis (with no-color-only semantics), interaction disposition, and surface context,
/// preserves its width and label while loading, never forks a feature-local style, never hides a
/// locked / degraded state behind generic disabled chrome, and always binds back to one canonical
/// command.
pub fn resolve_button(
    input: M5ButtonResolutionInput,
) -> Result<M5ResolvedButton, M5ButtonIconButtonResolutionError> {
    if input.button_id.trim().is_empty() {
        return Err(M5ButtonIconButtonResolutionError::EmptyButtonId);
    }
    if string_is_forbidden(&input.button_id)
        || string_is_forbidden(&input.action_label)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5ButtonIconButtonResolutionError::ForbiddenMaterial);
    }

    let is_loading = input.loading_behavior.is_loading();
    let disposition_is_blocked = disposition_is_locked_or_degraded(input.disposition);

    let degrade_reason = if input.action_label.trim().is_empty() {
        Some(M5ButtonDegradeReason::ActionLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ButtonDegradeReason::SurfaceContextUnresolved)
    } else if !input.loading_behavior.is_resolved() {
        Some(M5ButtonDegradeReason::LoadingBehaviorUnresolved)
    } else if input.forks_feature_local_style {
        Some(M5ButtonDegradeReason::FeatureLocalStyleForked)
    } else if !input.emphasis_stated {
        Some(M5ButtonDegradeReason::EmphasisEncodedByColorAlone)
    } else if is_loading && !input.loading_preserves_label_and_width {
        Some(M5ButtonDegradeReason::LoadingRelabeledOrResized)
    } else if disposition_is_blocked && !input.blocked_state_distinct {
        Some(M5ButtonDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    } else if input.command_id.trim().is_empty() {
        Some(M5ButtonDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5ButtonDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5ButtonDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ButtonIconButtonNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedButton {
        button_id: input.button_id,
        action_label: input.action_label,
        emphasis: input.emphasis.as_str().to_owned(),
        emphasis_is_destructive: input.emphasis.is_destructive(),
        disposition: input.disposition.as_str().to_owned(),
        disposition_is_blocked,
        surface_context: input.surface_context.as_str().to_owned(),
        loading_behavior: input.loading_behavior.as_str().to_owned(),
        is_loading,
        loading_preserves_label_and_width: input.loading_preserves_label_and_width,
        blocked_state_distinct: input.blocked_state_distinct,
        command_id: input.command_id,
        forks_feature_local_style: input.forks_feature_local_style,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        action_attributable_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves an icon button so its name and command are legible at a glance: the icon names its
/// accessible name, label mode, emphasis, interaction disposition, and surface context, keeps tooltip
/// parity with the accessible name, never invents a brand-only affordance, never leaves an icon-only
/// destructive action unlabeled, and aligns one canonical command ID across the context menu, palette,
/// and help surfaces.
pub fn resolve_icon_button(
    input: M5IconButtonResolutionInput,
) -> Result<M5ResolvedIconButton, M5ButtonIconButtonResolutionError> {
    if input.icon_button_id.trim().is_empty() {
        return Err(M5ButtonIconButtonResolutionError::EmptyIconButtonId);
    }
    if string_is_forbidden(&input.icon_button_id)
        || string_is_forbidden(&input.accessible_name)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5ButtonIconButtonResolutionError::ForbiddenMaterial);
    }

    let is_destructive = input.emphasis.is_destructive();
    let carries_name = icon_label_carries_accessible_name(input.label_mode);
    let exposes_accessible_name = carries_name && !input.accessible_name.trim().is_empty();

    let degrade_reason = if input.accessible_name.trim().is_empty() {
        Some(M5IconButtonDegradeReason::AccessibleNameUnstated)
    } else if matches!(input.label_mode, M5IconLabelMode::LabelUnresolved) {
        Some(M5IconButtonDegradeReason::LabelModeUnresolved)
    } else if !input.surface_context.is_resolved() {
        Some(M5IconButtonDegradeReason::SurfaceContextUnresolved)
    } else if !input.command_surface.is_resolved() {
        Some(M5IconButtonDegradeReason::CommandSurfaceUnresolved)
    } else if input.invents_brand_only_affordance {
        Some(M5IconButtonDegradeReason::BrandOnlyAffordanceInvented)
    } else if is_destructive && !carries_name {
        Some(M5IconButtonDegradeReason::IconOnlyDestructiveUnlabeled)
    } else if !input.tooltip_parity {
        Some(M5IconButtonDegradeReason::TooltipParityMissing)
    } else if input.command_id.trim().is_empty() {
        Some(M5IconButtonDegradeReason::CommandBindingUnstated)
    } else if !input.command_parity_across_surfaces {
        Some(M5IconButtonDegradeReason::CommandParityBrokenAcrossSurfaces)
    } else if !input.command_route_available {
        Some(M5IconButtonDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5IconButtonDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ButtonIconButtonNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedIconButton {
        icon_button_id: input.icon_button_id,
        accessible_name: input.accessible_name,
        label_mode: input.label_mode.as_str().to_owned(),
        exposes_accessible_name,
        emphasis: input.emphasis.as_str().to_owned(),
        emphasis_is_destructive: is_destructive,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        command_surface: input.command_surface.as_str().to_owned(),
        tooltip_parity: input.tooltip_parity,
        command_id: input.command_id,
        command_parity_across_surfaces: input.command_parity_across_surfaces,
        invents_brand_only_affordance: input.invents_brand_only_affordance,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        name_and_command_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved button and icon-button examples it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ButtonIconButtonControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ButtonIconButtonConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5CoreControlQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5CoreControlDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5CoreControlRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5CoreControlAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ButtonIconButtonAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ButtonIconButtonExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    /// Resolved button examples.
    pub button_examples: Vec<M5ResolvedButton>,
    /// Resolved icon-button examples.
    pub icon_button_examples: Vec<M5ResolvedIconButton>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a button never relabels the action or resizes losing attribution while loading.
    pub buttons_relabel_or_resize_when_loading: bool,
    /// Hard invariant: an icon-only destructive action never goes unlabeled.
    pub icon_only_destructive_actions_go_unlabeled: bool,
    /// Hard invariant: locked / degraded semantics never hide behind generic disabled chrome.
    pub locked_or_degraded_semantics_hidden_behind_disabled: bool,
    /// Hard invariant: controls never fork feature-local styles instead of the shared grammar.
    pub controls_fork_feature_local_styles: bool,
}

impl M5ButtonIconButtonControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ButtonIconButtonAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ButtonIconButtonAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ButtonIconButtonExportField> =
            self.export_fields.iter().copied().collect();
        M5ButtonIconButtonExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.buttons_relabel_or_resize_when_loading
            && !self.icon_only_destructive_actions_go_unlabeled
            && !self.locked_or_degraded_semantics_hidden_behind_disabled
            && !self.controls_fork_feature_local_styles
    }

    /// True when a clean button preserves attribution: it never forks a style, keeps its label / width
    /// while loading, keeps a locked / degraded state distinct, and offers a command trace path.
    fn button_is_honest(ex: &M5ResolvedButton) -> bool {
        !ex.is_clean()
            || (!ex.forks_feature_local_style
                && (!ex.is_loading || ex.loading_preserves_label_and_width)
                && (!ex.disposition_is_blocked || ex.blocked_state_distinct)
                && ex.command_route_available)
    }

    /// True when a clean icon button preserves labeling: it never invents a brand-only affordance,
    /// exposes an accessible name, keeps command parity, and offers a command trace path.
    fn icon_is_honest(ex: &M5ResolvedIconButton) -> bool {
        !ex.is_clean()
            || (!ex.invents_brand_only_affordance
                && ex.exposes_accessible_name
                && ex.command_parity_across_surfaces
                && ex.command_route_available)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.button_examples.iter().all(Self::button_is_honest)
            && self.icon_button_examples.iter().all(Self::icon_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ButtonIconButtonVocabularySet {
    /// Interaction-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Button-emphasis tokens (bound from the frozen matrix).
    pub button_emphases: Vec<String>,
    /// Icon-label-mode tokens (bound from the frozen matrix).
    pub icon_label_modes: Vec<String>,
    /// Loading-behavior tokens (minted by this lane).
    pub loading_behaviors: Vec<String>,
    /// Command-surface tokens (minted by this lane).
    pub command_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Button degrade-reason tokens.
    pub button_degrade_reasons: Vec<String>,
    /// Icon-button degrade-reason tokens.
    pub icon_button_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ButtonIconButtonVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5CoreControlDisposition::ALL, |v| v.as_str()),
            button_emphases: tokens(&M5ButtonEmphasis::ALL, |v| v.as_str()),
            icon_label_modes: tokens(&M5IconLabelMode::ALL, |v| v.as_str()),
            loading_behaviors: tokens(&M5ButtonLoadingBehavior::ALL, |v| v.as_str()),
            command_surfaces: tokens(&M5ActionCommandSurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ActionSurfaceContext::ALL, |v| v.as_str()),
            button_degrade_reasons: tokens(&M5ButtonDegradeReason::ALL, |v| v.as_str()),
            icon_button_degrade_reasons: tokens(&M5IconButtonDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ButtonIconButtonAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ButtonIconButtonNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ButtonIconButtonExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5CoreControlConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5ButtonIconButtonGovernanceReview {
    /// The button names its permanent action label and stable emphasis.
    pub button_names_label_and_emphasis: bool,
    /// The button preserves its width and primary label while loading.
    pub button_preserves_width_and_label_while_loading: bool,
    /// The button never forks a feature-local style instead of the shared emphasis grammar.
    pub button_never_forks_feature_local_style: bool,
    /// The icon button always exposes an accessible name for its action.
    pub icon_button_always_exposes_accessible_name: bool,
    /// The icon button keeps tooltip parity with the accessible name.
    pub icon_button_keeps_tooltip_parity: bool,
    /// The icon button never leaves a destructive action unlabeled.
    pub icon_button_never_unlabeled_when_destructive: bool,
    /// The icon button binds one canonical command ID with parity across menu / palette / help.
    pub icon_button_binds_canonical_command_with_parity: bool,
    /// Locked / degraded semantics are never hidden behind generic disabled chrome.
    pub locked_and_degraded_never_hidden_behind_disabled: bool,
    /// Emphasis and state are never encoded by color alone.
    pub emphasis_and_state_never_encoded_by_color_alone: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ButtonIconButtonConsumerProjection {
    /// Forms surfaces consume the shared button and icon-button vocabulary.
    pub forms_surfaces_consume_button_vocabulary: bool,
    /// Settings surfaces consume the shared button and icon-button vocabulary.
    pub settings_surfaces_consume_button_vocabulary: bool,
    /// Review surfaces consume the shared action and command vocabulary.
    pub review_surfaces_consume_action_and_command_vocabulary: bool,
    /// Entry / start-center surfaces consume the shared button vocabulary.
    pub entry_surfaces_consume_button_vocabulary: bool,
    /// Action and command facts trace back to one canonical component contract.
    pub action_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical control source.
    pub support_export_reads_single_control_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ButtonIconButtonProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ButtonIconButtonReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ButtonIconButtonControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ButtonIconButtonControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ButtonIconButtonControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ButtonIconButtonVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ButtonIconButtonGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ButtonIconButtonConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ButtonIconButtonProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ButtonIconButtonReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 button / icon-button controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ButtonIconButtonControlsPacket {
    /// Record kind; must equal [`M5_BUTTON_ICON_BUTTON_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ButtonIconButtonControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ButtonIconButtonVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ButtonIconButtonGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ButtonIconButtonConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ButtonIconButtonProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ButtonIconButtonReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ButtonIconButtonControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ButtonIconButtonControlsPacketInput) -> Self {
        Self {
            record_kind: M5_BUTTON_ICON_BUTTON_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5ButtonIconButtonControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUTTON_ICON_BUTTON_CONTROLS_RECORD_KIND {
            violations.push(M5ButtonIconButtonControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ButtonIconButtonControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ButtonIconButtonControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ButtonIconButtonControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 button / icon-button controls packet serializes"),
        ) {
            violations.push(M5ButtonIconButtonControlsViolation::RawMaterialInExport);
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
            .expect("m5 button / icon-button controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,button_examples,icon_button_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .button_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.icon_button_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.button_examples.len(),
                row.icon_button_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Button and Icon-Button Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Button emphases: {}\n",
            self.vocabulary_set.button_emphases.join(", ")
        ));
        out.push_str(&format!(
            "- Loading behaviors: {}\n",
            self.vocabulary_set.loading_behaviors.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Button examples: {} / icon-button examples: {}\n",
                row.button_examples.len(),
                row.icon_button_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ButtonIconButtonControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ButtonIconButtonControlsViolation>),
}

impl fmt::Display for M5ButtonIconButtonControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 button / icon-button controls export parse failed: {error}"
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
                    "m5 button / icon-button controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ButtonIconButtonControlsArtifactError {}

/// Validation failures emitted by [`M5ButtonIconButtonControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ButtonIconButtonControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (style fork, loading relabel, hidden lock,
    /// brand-only affordance, unlabeled destructive, broken parity, or missing trace).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Button-state behavior is not proven: clean buttons do not cover the primary / destructive /
    /// quiet emphasis grammar with focus / loading / disabled / locked states, or no loading-relabel /
    /// hidden-lock example degrades.
    ButtonStateBehaviorNotProven,
    /// Icon accessible-name and command parity is not proven: no clean icon button exposes an
    /// accessible name and command parity, or no unlabeled-destructive / brand-only / broken-parity
    /// example degrades.
    IconAccessibleNameAndCommandParityNotProven,
    /// State traceability is not proven: no clean button and clean icon button both offer a
    /// command-backed detail entrypoint.
    ButtonStateTraceabilityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ButtonIconButtonControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::ButtonStateBehaviorNotProven => "button_state_behavior_not_proven",
            Self::IconAccessibleNameAndCommandParityNotProven => {
                "icon_accessible_name_and_command_parity_not_proven"
            }
            Self::ButtonStateTraceabilityNotProven => "button_state_traceability_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_button_icon_button_controls_export(
) -> Result<M5ButtonIconButtonControlsPacket, M5ButtonIconButtonControlsArtifactError> {
    let packet: M5ButtonIconButtonControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-button-icon-button-controls-proof/support_export.json"
    )))
    .map_err(M5ButtonIconButtonControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ButtonIconButtonControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUTTON_ICON_BUTTON_CONTROLS_SCHEMA_REF,
        M5_BUTTON_ICON_BUTTON_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_BUTTON_SCHEMA_REF,
        M5_ICON_BUTTON_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ButtonIconButtonControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ButtonIconButtonControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5ButtonIconButtonControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ButtonIconButtonControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ButtonIconButtonControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_BUTTON_SCHEMA_REF) || !refs.contains(M5_ICON_BUTTON_SCHEMA_REF) {
            violations.push(M5ButtonIconButtonControlsViolation::ComponentSchemaRefMissing);
        }
        if row.button_examples.is_empty() || row.icon_button_examples.is_empty() {
            violations.push(M5ButtonIconButtonControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ButtonIconButtonControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ButtonIconButtonControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.button_names_label_and_emphasis,
        review.button_preserves_width_and_label_while_loading,
        review.button_never_forks_feature_local_style,
        review.icon_button_always_exposes_accessible_name,
        review.icon_button_keeps_tooltip_parity,
        review.icon_button_never_unlabeled_when_destructive,
        review.icon_button_binds_canonical_command_with_parity,
        review.locked_and_degraded_never_hidden_behind_disabled,
        review.emphasis_and_state_never_encoded_by_color_alone,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ButtonIconButtonControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.forms_surfaces_consume_button_vocabulary,
        projection.settings_surfaces_consume_button_vocabulary,
        projection.review_surfaces_consume_action_and_command_vocabulary,
        projection.entry_surfaces_consume_button_vocabulary,
        projection.action_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_control_source,
    ] {
        if !ok {
            violations.push(M5ButtonIconButtonControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ButtonIconButtonControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ButtonIconButtonControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ButtonIconButtonControlsPacket,
    violations: &mut Vec<M5ButtonIconButtonControlsViolation>,
) {
    let buttons = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.button_examples.iter())
    };
    let icons = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.icon_button_examples.iter())
    };

    // AC1: the first claimed M5 consumers show stable primary / destructive / quiet action behavior
    // with correct focus, loading, disabled, and locked states. Clean buttons cover at least the
    // primary, destructive, and quiet emphases and the focus-visible, loading, disabled, and locked
    // dispositions, a loading-relabel example degrades, a hidden-lock example degrades, and no clean
    // button relabels when loading or hides a lock.
    let clean_emphases: BTreeSet<String> = buttons()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.emphasis.clone())
        .collect();
    let clean_dispositions: BTreeSet<String> = buttons()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.disposition.clone())
        .collect();
    let emphasis_grammar_covered = ["primary", "destructive", "quiet"]
        .iter()
        .all(|e| clean_emphases.contains(*e));
    let state_grammar_covered = ["focus_visible", "loading", "disabled", "locked"]
        .iter()
        .all(|s| clean_dispositions.contains(*s));
    let loading_relabel_degrades = buttons()
        .any(|ex| ex.degrade_reason == Some(M5ButtonDegradeReason::LoadingRelabeledOrResized));
    let hidden_lock_degrades = buttons().any(|ex| {
        ex.degrade_reason == Some(M5ButtonDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    });
    let no_clean_relabel_or_hidden_lock = buttons().all(|ex| {
        !(ex.is_clean()
            && ((ex.is_loading && !ex.loading_preserves_label_and_width)
                || (ex.disposition_is_blocked && !ex.blocked_state_distinct)))
    });
    if !(emphasis_grammar_covered
        && state_grammar_covered
        && loading_relabel_degrades
        && hidden_lock_degrades
        && no_clean_relabel_or_hidden_lock)
    {
        violations.push(M5ButtonIconButtonControlsViolation::ButtonStateBehaviorNotProven);
    }

    // AC2: icon-only buttons expose accessible names and command parity rather than hidden or
    // brand-only affordances. At least one clean icon button exposes an accessible name and command
    // parity, an unlabeled-destructive example degrades, a brand-only example degrades, a broken-parity
    // example degrades, and no clean icon button is unlabeled-destructive or brand-only.
    let clean_named_icon = icons().any(|ex| {
        ex.is_clean()
            && ex.exposes_accessible_name
            && ex.command_parity_across_surfaces
            && !ex.command_id.trim().is_empty()
    });
    let unlabeled_destructive_degrades = icons().any(|ex| {
        ex.degrade_reason == Some(M5IconButtonDegradeReason::IconOnlyDestructiveUnlabeled)
    });
    let brand_only_degrades = icons().any(|ex| {
        ex.degrade_reason == Some(M5IconButtonDegradeReason::BrandOnlyAffordanceInvented)
    });
    let broken_parity_degrades = icons().any(|ex| {
        ex.degrade_reason == Some(M5IconButtonDegradeReason::CommandParityBrokenAcrossSurfaces)
    });
    let no_clean_unlabeled_or_brand = icons().all(|ex| {
        !(ex.is_clean()
            && (ex.invents_brand_only_affordance
                || (ex.emphasis_is_destructive && !ex.exposes_accessible_name)))
    });
    if !(clean_named_icon
        && unlabeled_destructive_degrades
        && brand_only_degrades
        && broken_parity_degrades
        && no_clean_unlabeled_or_brand)
    {
        violations
            .push(M5ButtonIconButtonControlsViolation::IconAccessibleNameAndCommandParityNotProven);
    }

    // AC3: button state drift is caught by fixtures before release evidence turns green — a user can
    // trace action and command state back to one canonical component contract and command-backed
    // detail entrypoints. At least one clean button and one clean icon button both expose a
    // command-backed detail entrypoint.
    let traceable_button = buttons().any(|ex| ex.is_clean() && ex.command_route_available);
    let traceable_icon = icons().any(|ex| ex.is_clean() && ex.command_route_available);
    if !(traceable_button && traceable_icon) {
        violations.push(M5ButtonIconButtonControlsViolation::ButtonStateTraceabilityNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5CoreControlFamily; 2] =
    [M5CoreControlFamily::Button, M5CoreControlFamily::IconButton];
