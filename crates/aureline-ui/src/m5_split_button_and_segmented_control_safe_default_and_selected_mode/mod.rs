//! Implemented M5 split-button and segmented-control primitives.
//!
//! The frozen [core action / input component matrix][matrix] names Aureline's most reused atomic
//! action and input controls and locks their controlled vocabulary. This module is the alternate-action
//! and mode-toggle implement lane over that matrix: it turns the **split button** and the **segmented
//! control** into resolvers that produce export-safe, honest projections, so a user can trust that a
//! split button's default click is the safest sensible action (never a riskier alternate promoted by
//! stale state), that its alternates stay visible in the adjacent menu, and that a segmented control
//! stays a small mode / view toggle with explicit selected-mode truth instead of quietly becoming a
//! second global navigation system.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement split buttons so the primary action is the safest sensible default, alternates remain
//!   visible in the adjacent menu, and riskier variants cannot silently become the default because of
//!   stale state.** [`resolve_split_button`] refuses to read as a clean, safe-default trigger when the
//!   primary action label is unstated, the surface context or default posture is unresolved, the
//!   alternate visibility is unresolved, emphasis is encoded by color alone, stale state promoted a
//!   riskier alternate to the default, an alternate is hidden behind the default, a broadened scope goes
//!   undisclosed, a locked / degraded state hides behind generic disabled chrome, or the canonical
//!   command binding / trace path is missing; it degrades instead.
//! * **Implement segmented controls for compact view / mode toggles only, with current / selected /
//!   disabled / locked / degraded truth and keyboard cycling that does not masquerade as top-level
//!   navigation.** [`resolve_segmented_control`] degrades when the group or selected-segment label is
//!   unstated, the surface context or mode is unresolved, the control masquerades as stealth navigation,
//!   the selected state is encoded by color alone, keyboard cycling is missing, the segment set is
//!   oversized, mode-scope continuity is broken, or a locked / degraded state hides behind disabled
//!   chrome.
//! * **Preserve command IDs, support/export metadata, and review-state continuity when alternate actions
//!   or mode toggles affect broader scope.** Both resolvers degrade with a scope-continuity reason
//!   whenever a broadened scope is left undisclosed, and both always bind back to one canonical command.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5CoreControlDisposition`] interaction-state vocabulary, the [`M5SplitDefaultPosture`] split default
//! posture vocabulary, the [`M5SegmentedMode`] segmented-mode vocabulary, and the [`M5ButtonEmphasis`]
//! emphasis vocabulary — so forms, settings, search, review, support, and product surfaces can never fork
//! their own alternate-action or mode-toggle wording. Raw secret values and private endpoints stay
//! outside the export boundary.
//!
//! [matrix]: crate::m5_core_action_input_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_split_button_segmented_control_controls,
    seeded_m5_split_button_segmented_control_controls_review_ui_beta_narrowed,
    seeded_m5_split_button_segmented_control_controls_search_ui_preview_narrowed,
    M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_core_action_input_component_matrix::{
    M5ButtonEmphasis, M5CoreControlAccessibilityRoute, M5CoreControlConsumerSurface,
    M5CoreControlDeploymentLine, M5CoreControlDisposition, M5CoreControlDowngradeTrigger,
    M5CoreControlFamily, M5CoreControlQualificationClass, M5CoreControlRequiredLabel,
    M5SegmentedMode, M5SplitDefaultPosture, M5_CORE_CONTROL_COMPONENT_DOC_REF,
    M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_SEGMENTED_CONTROL_SCHEMA_REF,
    M5_SPLIT_BUTTON_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SplitButtonSegmentedControlControlsPacket`].
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_RECORD_KIND: &str =
    "implement_m5_split_button_and_segmented_control_controls";

/// Schema version for M5 split-button / segmented-control controls records.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-split-button-segmented-control-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_DOC_REF: &str =
    "docs/components/m5_split_button_and_segmented_control_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-split-button-segmented-control-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-split-button-segmented-control-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-split-button-segmented-control-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-split-button-segmented-control-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5SplitSegmentedConsumerSurface = M5CoreControlConsumerSurface;

/// Controlled render context — which claimed M5 surface renders the alternate-action or mode toggle, so a
/// control's meaning stays stable whether it appears in a pane header, review sheet, settings row, start
/// center, or support flow. Minted by this lane, tracking the exit-gate anchor surfaces directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitSegmentedSurfaceContext {
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

impl M5SplitSegmentedSurfaceContext {
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

/// Controlled alternate visibility — how a split button exposes its non-default alternates, so an
/// alternate is never hidden behind the default click. Minted by this lane because the frozen matrix
/// carries the safe-default posture but not the alternate-visibility posture the split-button acceptance
/// criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitAlternateVisibility {
    /// Alternates are visible in the adjacent menu.
    AdjacentMenuVisible,
    /// Alternates are disclosed on an explicit expand affordance.
    DisclosedOnExpand,
    /// A secondary alternate is visible inline beside the default.
    InlineSecondaryVisible,
    /// Alternates are grouped in a labeled overflow, still reachable and named.
    OverflowGroupedVisible,
    /// The alternate is hidden behind the default click (disallowed).
    AlternateHidden,
    /// The alternate visibility cannot currently be resolved.
    VisibilityUnknown,
}

impl M5SplitAlternateVisibility {
    /// Every alternate visibility, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AdjacentMenuVisible,
        Self::DisclosedOnExpand,
        Self::InlineSecondaryVisible,
        Self::OverflowGroupedVisible,
        Self::AlternateHidden,
        Self::VisibilityUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdjacentMenuVisible => "adjacent_menu_visible",
            Self::DisclosedOnExpand => "disclosed_on_expand",
            Self::InlineSecondaryVisible => "inline_secondary_visible",
            Self::OverflowGroupedVisible => "overflow_grouped_visible",
            Self::AlternateHidden => "alternate_hidden",
            Self::VisibilityUnknown => "visibility_unknown",
        }
    }

    /// Whether the alternates stay visible (never hidden behind the default, never unresolved).
    pub const fn is_visible(self) -> bool {
        matches!(
            self,
            Self::AdjacentMenuVisible
                | Self::DisclosedOnExpand
                | Self::InlineSecondaryVisible
                | Self::OverflowGroupedVisible
        )
    }

    /// Whether the alternate visibility is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::VisibilityUnknown)
    }
}

/// Controlled scope impact — how broadly an alternate action or mode toggle affects state, so a control
/// that widens scope beyond the obvious target always discloses it and preserves review-state continuity.
/// Minted by this lane and shared by both resolvers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitScopeImpact {
    /// The action affects a single target only.
    SingleTarget,
    /// The action affects the current selection only.
    CurrentSelection,
    /// The action affects a whole batch and must disclose that scope.
    WholeBatch,
    /// The action affects a broader cross-surface scope and must disclose it.
    CrossSurface,
    /// The action is broad and irreversible and must disclose it.
    IrreversibleBroad,
    /// The scope impact cannot currently be resolved.
    ScopeUnknown,
}

impl M5SplitScopeImpact {
    /// Every scope impact, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleTarget,
        Self::CurrentSelection,
        Self::WholeBatch,
        Self::CrossSurface,
        Self::IrreversibleBroad,
        Self::ScopeUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleTarget => "single_target",
            Self::CurrentSelection => "current_selection",
            Self::WholeBatch => "whole_batch",
            Self::CrossSurface => "cross_surface",
            Self::IrreversibleBroad => "irreversible_broad",
            Self::ScopeUnknown => "scope_unknown",
        }
    }

    /// Whether this scope broadens beyond the obvious target and must be disclosed (also true for the
    /// unresolved sentinel, which can never be presented as a narrow scope).
    pub const fn needs_disclosure(self) -> bool {
        matches!(
            self,
            Self::WholeBatch | Self::CrossSurface | Self::IrreversibleBroad | Self::ScopeUnknown
        )
    }
}

/// One mandatory rendered part a split button or segmented control must be able to show, so no default,
/// alternate, mode, state, or command fact is left implicit behind a menu, a tooltip, or a secondary
/// panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitSegmentedAnatomyPart {
    /// The control's stable identity / permanent label.
    Identity,
    /// The control's current typed interaction disposition.
    State,
    /// The non-visual keyboard route to the control.
    KeyboardRoute,
    /// The split default posture (split button).
    DefaultPosture,
    /// The alternate visibility (split button).
    AlternateVisibility,
    /// The scope impact of the action / mode toggle (both controls).
    ScopeImpact,
    /// The render / surface context (both controls).
    SurfaceContext,
    /// The selected segment (segmented control).
    SelectedSegment,
    /// The segmented mode (segmented control).
    SegmentedMode,
    /// The keyboard-cycling route (segmented control).
    KeyboardCycling,
    /// The canonical command binding (both controls).
    CommandBinding,
}

impl M5SplitSegmentedAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::DefaultPosture,
        Self::AlternateVisibility,
        Self::ScopeImpact,
        Self::SurfaceContext,
        Self::SelectedSegment,
        Self::SegmentedMode,
        Self::KeyboardCycling,
        Self::CommandBinding,
    ];

    /// The three parts every claimed control must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::DefaultPosture => "default_posture",
            Self::AlternateVisibility => "alternate_visibility",
            Self::ScopeImpact => "scope_impact",
            Self::SurfaceContext => "surface_context",
            Self::SelectedSegment => "selected_segment",
            Self::SegmentedMode => "segmented_mode",
            Self::KeyboardCycling => "keyboard_cycling",
            Self::CommandBinding => "command_binding",
        }
    }
}

/// Next safe action a control surfaces so a user is never left without a route to inspect the default,
/// alternate, mode, state, or command behind a degraded control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitSegmentedNextAction {
    /// Open the command-backed action / command detail.
    OpenCommandDetail,
    /// Inspect the split default posture / alternates.
    InspectSplitDefault,
    /// Inspect the segmented mode / selected segment.
    InspectSegmentedMode,
    /// Review a locked / blocked / disabled control.
    ReviewBlockedOrLocked,
    /// Review the alternate menu or broadened scope.
    ReviewAlternateOrScope,
    /// No action is needed; the control is clean.
    NoActionNeeded,
}

impl M5SplitSegmentedNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenCommandDetail,
        Self::InspectSplitDefault,
        Self::InspectSegmentedMode,
        Self::ReviewBlockedOrLocked,
        Self::ReviewAlternateOrScope,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCommandDetail => "open_command_detail",
            Self::InspectSplitDefault => "inspect_split_default",
            Self::InspectSegmentedMode => "inspect_segmented_mode",
            Self::ReviewBlockedOrLocked => "review_blocked_or_locked",
            Self::ReviewAlternateOrScope => "review_alternate_or_scope",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitSegmentedExportField {
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
    /// The split default posture named by the split button.
    DefaultPosture,
    /// The alternate visibility named by the split button.
    AlternateVisibility,
    /// The segmented mode named by the segmented control.
    SegmentedMode,
    /// The selected segment named by the segmented control.
    SelectedSegment,
    /// The render / surface context named by both controls.
    SurfaceContext,
    /// The accountable owner role.
    OwnerRole,
}

impl M5SplitSegmentedExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::DefaultPosture,
        Self::AlternateVisibility,
        Self::SegmentedMode,
        Self::SelectedSegment,
        Self::SurfaceContext,
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
            Self::DefaultPosture => "default_posture",
            Self::AlternateVisibility => "alternate_visibility",
            Self::SegmentedMode => "segmented_mode",
            Self::SelectedSegment => "selected_segment",
            Self::SurfaceContext => "surface_context",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a split button degraded below a clean, safe-default state. The degrade-first ladder returns one
/// of these instead of ever letting a riskier or ambiguous split read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitButtonDegradeReason {
    /// The primary action label is unstated; a user cannot tell what the default click does.
    PrimaryActionUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The split default posture cannot currently be resolved.
    DefaultPostureUnresolved,
    /// The alternate visibility cannot currently be resolved.
    AlternateVisibilityUnresolved,
    /// The emphasis is encoded by color alone rather than named.
    EmphasisEncodedByColorAlone,
    /// Stale state promoted a riskier alternate to the default click.
    RiskierAlternateBecameDefault,
    /// An alternate is hidden behind the default click instead of visible in the adjacent menu.
    AlternateHiddenBehindDefault,
    /// A broadened scope is left undisclosed, breaking review-state continuity.
    BroadenedScopeUndisclosed,
    /// A locked / degraded state hides behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the action is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SplitButtonDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PrimaryActionUnstated,
        Self::SurfaceContextUnresolved,
        Self::DefaultPostureUnresolved,
        Self::AlternateVisibilityUnresolved,
        Self::EmphasisEncodedByColorAlone,
        Self::RiskierAlternateBecameDefault,
        Self::AlternateHiddenBehindDefault,
        Self::BroadenedScopeUndisclosed,
        Self::LockedOrDegradedHiddenBehindDisabled,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryActionUnstated => "primary_action_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DefaultPostureUnresolved => "default_posture_unresolved",
            Self::AlternateVisibilityUnresolved => "alternate_visibility_unresolved",
            Self::EmphasisEncodedByColorAlone => "emphasis_encoded_by_color_alone",
            Self::RiskierAlternateBecameDefault => "riskier_alternate_became_default",
            Self::AlternateHiddenBehindDefault => "alternate_hidden_behind_default",
            Self::BroadenedScopeUndisclosed => "broadened_scope_undisclosed",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
            }
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SplitSegmentedNextAction {
        match self {
            Self::PrimaryActionUnstated
            | Self::SurfaceContextUnresolved
            | Self::DefaultPostureUnresolved
            | Self::EmphasisEncodedByColorAlone => M5SplitSegmentedNextAction::InspectSplitDefault,
            Self::AlternateVisibilityUnresolved
            | Self::RiskierAlternateBecameDefault
            | Self::AlternateHiddenBehindDefault
            | Self::BroadenedScopeUndisclosed => M5SplitSegmentedNextAction::ReviewAlternateOrScope,
            Self::LockedOrDegradedHiddenBehindDisabled => {
                M5SplitSegmentedNextAction::ReviewBlockedOrLocked
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing | Self::ProofStale => {
                M5SplitSegmentedNextAction::OpenCommandDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::PrimaryActionUnstated => M5CoreControlDowngradeTrigger::PlaceholderUsedAsLabel,
            Self::RiskierAlternateBecameDefault | Self::AlternateHiddenBehindDefault => {
                M5CoreControlDowngradeTrigger::SplitDefaultedToRiskierAlternate
            }
            Self::LockedOrDegradedHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::EmphasisEncodedByColorAlone => {
                M5CoreControlDowngradeTrigger::StateTaxonomyDrifted
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SurfaceContextUnresolved
            | Self::DefaultPostureUnresolved
            | Self::AlternateVisibilityUnresolved
            | Self::BroadenedScopeUndisclosed => {
                M5CoreControlDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// Reason a segmented control degraded below a clean, small-mode-toggle state with explicit selected-mode
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SegmentedControlDegradeReason {
    /// The group label is unstated; the control's identity is unclear.
    GroupLabelUnstated,
    /// The selected-segment label is unstated; the current mode is unclear.
    SelectedSegmentUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The segmented mode cannot currently be resolved.
    ModeUnresolved,
    /// The control masquerades as top-level / stealth navigation.
    UsedAsStealthNavigation,
    /// The selected state is encoded by color alone rather than named.
    SelectedStateEncodedByColorAlone,
    /// Keyboard cycling across the segments is missing.
    KeyboardCyclingMissing,
    /// The segment set is oversized, reading as navigation rather than a compact mode toggle.
    OversizedSegmentSet,
    /// A broadened mode scope is left undisclosed, breaking review-state continuity.
    ModeScopeContinuityBroken,
    /// A locked / degraded state hides behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the mode toggle is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SegmentedControlDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::GroupLabelUnstated,
        Self::SelectedSegmentUnstated,
        Self::SurfaceContextUnresolved,
        Self::ModeUnresolved,
        Self::UsedAsStealthNavigation,
        Self::SelectedStateEncodedByColorAlone,
        Self::KeyboardCyclingMissing,
        Self::OversizedSegmentSet,
        Self::ModeScopeContinuityBroken,
        Self::LockedOrDegradedHiddenBehindDisabled,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GroupLabelUnstated => "group_label_unstated",
            Self::SelectedSegmentUnstated => "selected_segment_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ModeUnresolved => "mode_unresolved",
            Self::UsedAsStealthNavigation => "used_as_stealth_navigation",
            Self::SelectedStateEncodedByColorAlone => "selected_state_encoded_by_color_alone",
            Self::KeyboardCyclingMissing => "keyboard_cycling_missing",
            Self::OversizedSegmentSet => "oversized_segment_set",
            Self::ModeScopeContinuityBroken => "mode_scope_continuity_broken",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
            }
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SplitSegmentedNextAction {
        match self {
            Self::GroupLabelUnstated
            | Self::SelectedSegmentUnstated
            | Self::SurfaceContextUnresolved
            | Self::ModeUnresolved
            | Self::SelectedStateEncodedByColorAlone => {
                M5SplitSegmentedNextAction::InspectSegmentedMode
            }
            Self::UsedAsStealthNavigation
            | Self::OversizedSegmentSet
            | Self::ModeScopeContinuityBroken => M5SplitSegmentedNextAction::ReviewAlternateOrScope,
            Self::KeyboardCyclingMissing | Self::LockedOrDegradedHiddenBehindDisabled => {
                M5SplitSegmentedNextAction::ReviewBlockedOrLocked
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing | Self::ProofStale => {
                M5SplitSegmentedNextAction::OpenCommandDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::GroupLabelUnstated => M5CoreControlDowngradeTrigger::PlaceholderUsedAsLabel,
            Self::SelectedSegmentUnstated
            | Self::UsedAsStealthNavigation
            | Self::SelectedStateEncodedByColorAlone
            | Self::OversizedSegmentSet => M5CoreControlDowngradeTrigger::StateTaxonomyDrifted,
            Self::LockedOrDegradedHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SurfaceContextUnresolved
            | Self::ModeUnresolved
            | Self::KeyboardCyclingMissing
            | Self::ModeScopeContinuityBroken => {
                M5CoreControlDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// True when a disposition names a locked / degraded state that must never hide behind generic disabled
/// chrome.
fn disposition_is_locked_or_degraded(disposition: M5CoreControlDisposition) -> bool {
    matches!(
        disposition,
        M5CoreControlDisposition::Locked | M5CoreControlDisposition::Degraded
    )
}

/// True when a split default posture is resolved (not the unknown sentinel).
fn split_posture_is_resolved(posture: M5SplitDefaultPosture) -> bool {
    !matches!(posture, M5SplitDefaultPosture::PostureUnknown)
}

/// True when a segmented mode is resolved (not the unknown sentinel).
fn segmented_mode_is_resolved(mode: M5SegmentedMode) -> bool {
    !matches!(mode, M5SegmentedMode::ModeUnknown)
}

/// Input to [`resolve_split_button`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SplitButtonResolutionInput {
    /// Stable identity of the split-button instance.
    pub split_button_id: String,
    /// The primary (default-click) action label shown; empty means unstated.
    pub primary_action_label: String,
    /// The split default posture (from the frozen matrix vocabulary).
    pub default_posture: M5SplitDefaultPosture,
    /// The default action emphasis (from the frozen matrix vocabulary).
    pub default_emphasis: M5ButtonEmphasis,
    /// True when the emphasis is stated non-color-only (weight / label, never color alone).
    pub emphasis_stated: bool,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// The render / surface context.
    pub surface_context: M5SplitSegmentedSurfaceContext,
    /// How the alternates are exposed.
    pub alternate_visibility: M5SplitAlternateVisibility,
    /// The scope impact of the default / alternate action.
    pub scope_impact: M5SplitScopeImpact,
    /// True when a broadened scope is disclosed to the user, preserving review-state continuity.
    pub scope_disclosed: bool,
    /// True when stale state promoted a riskier alternate to the default click.
    pub stale_state_promoted_riskier_alternate: bool,
    /// True when a locked / degraded state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// The canonical command ID this split button binds back to; empty means unstated.
    pub command_id: String,
    /// True when a command-backed path to inspect the action is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe split-button projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSplitButton {
    /// Stable identity of the split-button instance.
    pub split_button_id: String,
    /// The primary action label named by the split button.
    pub primary_action_label: String,
    /// The split-default-posture token named by the split button.
    pub default_posture: String,
    /// The default-emphasis token named by the split button.
    pub default_emphasis: String,
    /// Whether the emphasis names a destructive action.
    pub emphasis_is_destructive: bool,
    /// The interaction-disposition token named by the split button.
    pub disposition: String,
    /// Whether the disposition names a locked / degraded state.
    pub disposition_is_blocked: bool,
    /// The render / surface-context token named by the split button.
    pub surface_context: String,
    /// The alternate-visibility token named by the split button.
    pub alternate_visibility: String,
    /// Whether the alternates stay visible (never hidden behind the default).
    pub alternate_visible: bool,
    /// The scope-impact token named by the split button.
    pub scope_impact: String,
    /// Whether this scope broadens beyond the obvious target and must be disclosed.
    pub scope_needs_disclosure: bool,
    /// Whether a broadened scope is disclosed to the user.
    pub scope_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean split button): stale state promoted a riskier alternate.
    pub stale_state_promoted_riskier_alternate: bool,
    /// Whether a locked / degraded state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// The canonical command ID named by the split button.
    pub command_id: String,
    /// Whether a command-backed path to inspect the action is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the split button could not read as a clean, safe-default state.
    pub degrade_reason: Option<M5SplitButtonDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SplitSegmentedNextAction,
    /// Whether the default action is safe and attributable at a glance (clean split naming every fact).
    pub default_is_safe_at_a_glance: bool,
}

impl M5ResolvedSplitButton {
    /// Whether this split button reads as a clean, safe-default state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_segmented_control`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SegmentedControlResolutionInput {
    /// Stable identity of the segmented-control instance.
    pub segmented_control_id: String,
    /// The group label shown; empty means unstated.
    pub group_label: String,
    /// The selected-segment label shown; empty means unstated.
    pub selected_segment_label: String,
    /// The segmented mode (from the frozen matrix vocabulary).
    pub mode: M5SegmentedMode,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// The render / surface context.
    pub surface_context: M5SplitSegmentedSurfaceContext,
    /// True when the selected state is stated non-color-only (label / marker, never color alone).
    pub selected_state_explicit: bool,
    /// True when keyboard cycling across the segments is available.
    pub keyboard_cycling_available: bool,
    /// True when the segment set is oversized, reading as navigation rather than a compact toggle.
    pub oversized_segment_set: bool,
    /// True when the control masquerades as top-level / stealth navigation.
    pub masquerades_as_navigation: bool,
    /// The scope impact of the mode toggle.
    pub scope_impact: M5SplitScopeImpact,
    /// True when a broadened scope is disclosed to the user, preserving review-state continuity.
    pub scope_disclosed: bool,
    /// True when a locked / degraded state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// The canonical command ID this segmented control binds back to; empty means unstated.
    pub command_id: String,
    /// True when a command-backed path to inspect the mode toggle is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe segmented-control projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSegmentedControl {
    /// Stable identity of the segmented-control instance.
    pub segmented_control_id: String,
    /// The group label named by the segmented control.
    pub group_label: String,
    /// The selected-segment label named by the segmented control.
    pub selected_segment_label: String,
    /// Whether the selected segment is shown (a named label with a non-color-only marker).
    pub selected_segment_shown: bool,
    /// The segmented-mode token named by the segmented control.
    pub mode: String,
    /// The interaction-disposition token named by the segmented control.
    pub disposition: String,
    /// Whether the disposition names a locked / degraded state.
    pub disposition_is_blocked: bool,
    /// The render / surface-context token named by the segmented control.
    pub surface_context: String,
    /// Whether keyboard cycling across the segments is available.
    pub keyboard_cycling_available: bool,
    /// Guardrail (MUST be `false` on a clean segmented control): the set is oversized.
    pub oversized_segment_set: bool,
    /// Guardrail (MUST be `false` on a clean segmented control): masquerades as navigation.
    pub masquerades_as_navigation: bool,
    /// The scope-impact token named by the segmented control.
    pub scope_impact: String,
    /// Whether this scope broadens beyond the obvious target and must be disclosed.
    pub scope_needs_disclosure: bool,
    /// Whether a broadened scope is disclosed to the user.
    pub scope_disclosed: bool,
    /// Whether a locked / degraded state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// The canonical command ID named by the segmented control.
    pub command_id: String,
    /// Whether a command-backed path to inspect the mode toggle is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the segmented control could not read as a clean, small-mode-toggle state.
    pub degrade_reason: Option<M5SegmentedControlDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SplitSegmentedNextAction,
    /// Whether the selected mode is explicit at a glance (clean segmented naming every fact).
    pub selected_mode_explicit_at_a_glance: bool,
}

impl M5ResolvedSegmentedControl {
    /// Whether this segmented control reads as a clean, small-mode-toggle state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SplitSegmentedResolutionError {
    /// The split-button id was empty.
    EmptySplitButtonId,
    /// The segmented-control id was empty.
    EmptySegmentedControlId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SplitSegmentedResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySplitButtonId => "empty_split_button_id",
            Self::EmptySegmentedControlId => "empty_segmented_control_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SplitSegmentedResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 split-button / segmented-control resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SplitSegmentedResolutionError {}

/// Resolves a split button so its default action is safe and attributable at a glance: the split button
/// names its permanent primary action label, safe default posture (with no-color-only emphasis), and
/// interaction disposition, keeps its alternates visible in the adjacent menu, never lets stale state
/// promote a riskier alternate to the default, discloses any broadened scope, never hides a locked /
/// degraded state behind generic disabled chrome, and always binds back to one canonical command.
pub fn resolve_split_button(
    input: M5SplitButtonResolutionInput,
) -> Result<M5ResolvedSplitButton, M5SplitSegmentedResolutionError> {
    if input.split_button_id.trim().is_empty() {
        return Err(M5SplitSegmentedResolutionError::EmptySplitButtonId);
    }
    if string_is_forbidden(&input.split_button_id)
        || string_is_forbidden(&input.primary_action_label)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5SplitSegmentedResolutionError::ForbiddenMaterial);
    }

    let disposition_is_blocked = disposition_is_locked_or_degraded(input.disposition);
    let alternate_visible = input.alternate_visibility.is_visible();
    let scope_needs_disclosure = input.scope_impact.needs_disclosure();

    let degrade_reason = if input.primary_action_label.trim().is_empty() {
        Some(M5SplitButtonDegradeReason::PrimaryActionUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SplitButtonDegradeReason::SurfaceContextUnresolved)
    } else if !split_posture_is_resolved(input.default_posture) {
        Some(M5SplitButtonDegradeReason::DefaultPostureUnresolved)
    } else if !input.alternate_visibility.is_resolved() {
        Some(M5SplitButtonDegradeReason::AlternateVisibilityUnresolved)
    } else if !input.emphasis_stated {
        Some(M5SplitButtonDegradeReason::EmphasisEncodedByColorAlone)
    } else if input.stale_state_promoted_riskier_alternate {
        Some(M5SplitButtonDegradeReason::RiskierAlternateBecameDefault)
    } else if !alternate_visible {
        Some(M5SplitButtonDegradeReason::AlternateHiddenBehindDefault)
    } else if scope_needs_disclosure && !input.scope_disclosed {
        Some(M5SplitButtonDegradeReason::BroadenedScopeUndisclosed)
    } else if disposition_is_blocked && !input.blocked_state_distinct {
        Some(M5SplitButtonDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    } else if input.command_id.trim().is_empty() {
        Some(M5SplitButtonDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5SplitButtonDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5SplitButtonDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SplitSegmentedNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedSplitButton {
        split_button_id: input.split_button_id,
        primary_action_label: input.primary_action_label,
        default_posture: input.default_posture.as_str().to_owned(),
        default_emphasis: input.default_emphasis.as_str().to_owned(),
        emphasis_is_destructive: input.default_emphasis.is_destructive(),
        disposition: input.disposition.as_str().to_owned(),
        disposition_is_blocked,
        surface_context: input.surface_context.as_str().to_owned(),
        alternate_visibility: input.alternate_visibility.as_str().to_owned(),
        alternate_visible,
        scope_impact: input.scope_impact.as_str().to_owned(),
        scope_needs_disclosure,
        scope_disclosed: input.scope_disclosed,
        stale_state_promoted_riskier_alternate: input.stale_state_promoted_riskier_alternate,
        blocked_state_distinct: input.blocked_state_distinct,
        command_id: input.command_id,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        default_is_safe_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a segmented control so its selected mode is explicit at a glance: the control names its group
/// label, selected segment (with a non-color-only marker), mode, interaction disposition, and surface
/// context, stays a compact small mode / view toggle, never masquerades as top-level navigation, offers
/// keyboard cycling, discloses any broadened mode scope, never hides a locked / degraded state behind
/// generic disabled chrome, and always binds back to one canonical command.
pub fn resolve_segmented_control(
    input: M5SegmentedControlResolutionInput,
) -> Result<M5ResolvedSegmentedControl, M5SplitSegmentedResolutionError> {
    if input.segmented_control_id.trim().is_empty() {
        return Err(M5SplitSegmentedResolutionError::EmptySegmentedControlId);
    }
    if string_is_forbidden(&input.segmented_control_id)
        || string_is_forbidden(&input.group_label)
        || string_is_forbidden(&input.selected_segment_label)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5SplitSegmentedResolutionError::ForbiddenMaterial);
    }

    let disposition_is_blocked = disposition_is_locked_or_degraded(input.disposition);
    let scope_needs_disclosure = input.scope_impact.needs_disclosure();
    let selected_segment_shown =
        !input.selected_segment_label.trim().is_empty() && input.selected_state_explicit;

    let degrade_reason = if input.group_label.trim().is_empty() {
        Some(M5SegmentedControlDegradeReason::GroupLabelUnstated)
    } else if input.selected_segment_label.trim().is_empty() {
        Some(M5SegmentedControlDegradeReason::SelectedSegmentUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SegmentedControlDegradeReason::SurfaceContextUnresolved)
    } else if !segmented_mode_is_resolved(input.mode) {
        Some(M5SegmentedControlDegradeReason::ModeUnresolved)
    } else if input.masquerades_as_navigation {
        Some(M5SegmentedControlDegradeReason::UsedAsStealthNavigation)
    } else if !input.selected_state_explicit {
        Some(M5SegmentedControlDegradeReason::SelectedStateEncodedByColorAlone)
    } else if !input.keyboard_cycling_available {
        Some(M5SegmentedControlDegradeReason::KeyboardCyclingMissing)
    } else if input.oversized_segment_set {
        Some(M5SegmentedControlDegradeReason::OversizedSegmentSet)
    } else if scope_needs_disclosure && !input.scope_disclosed {
        Some(M5SegmentedControlDegradeReason::ModeScopeContinuityBroken)
    } else if disposition_is_blocked && !input.blocked_state_distinct {
        Some(M5SegmentedControlDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    } else if input.command_id.trim().is_empty() {
        Some(M5SegmentedControlDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5SegmentedControlDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5SegmentedControlDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SplitSegmentedNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedSegmentedControl {
        segmented_control_id: input.segmented_control_id,
        group_label: input.group_label,
        selected_segment_label: input.selected_segment_label,
        selected_segment_shown,
        mode: input.mode.as_str().to_owned(),
        disposition: input.disposition.as_str().to_owned(),
        disposition_is_blocked,
        surface_context: input.surface_context.as_str().to_owned(),
        keyboard_cycling_available: input.keyboard_cycling_available,
        oversized_segment_set: input.oversized_segment_set,
        masquerades_as_navigation: input.masquerades_as_navigation,
        scope_impact: input.scope_impact.as_str().to_owned(),
        scope_needs_disclosure,
        scope_disclosed: input.scope_disclosed,
        blocked_state_distinct: input.blocked_state_distinct,
        command_id: input.command_id,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        selected_mode_explicit_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved split-button and segmented-control
/// examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SplitButtonSegmentedControlControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SplitSegmentedConsumerSurface,
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
    pub anatomy_parts: Vec<M5SplitSegmentedAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5SplitSegmentedExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    /// Resolved split-button examples.
    pub split_button_examples: Vec<M5ResolvedSplitButton>,
    /// Resolved segmented-control examples.
    pub segmented_control_examples: Vec<M5ResolvedSegmentedControl>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a split button never defaults to a riskier alternate.
    pub split_buttons_default_to_riskier_alternate: bool,
    /// Hard invariant: alternate actions never hide behind the default click.
    pub alternate_actions_hidden_behind_default: bool,
    /// Hard invariant: a segmented control never masquerades as top-level navigation.
    pub segmented_controls_masquerade_as_navigation: bool,
    /// Hard invariant: locked / degraded semantics never hide behind generic disabled chrome.
    pub locked_or_degraded_semantics_hidden_behind_disabled: bool,
}

impl M5SplitButtonSegmentedControlControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SplitSegmentedAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SplitSegmentedAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SplitSegmentedExportField> =
            self.export_fields.iter().copied().collect();
        M5SplitSegmentedExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.split_buttons_default_to_riskier_alternate
            && !self.alternate_actions_hidden_behind_default
            && !self.segmented_controls_masquerade_as_navigation
            && !self.locked_or_degraded_semantics_hidden_behind_disabled
    }

    /// True when a clean split button keeps its default safe: it never lets stale state promote a
    /// riskier alternate, keeps alternates visible, discloses a broadened scope, keeps a locked /
    /// degraded state distinct, and offers a command trace path.
    fn split_is_honest(ex: &M5ResolvedSplitButton) -> bool {
        !ex.is_clean()
            || (!ex.stale_state_promoted_riskier_alternate
                && ex.alternate_visible
                && (!ex.scope_needs_disclosure || ex.scope_disclosed)
                && (!ex.disposition_is_blocked || ex.blocked_state_distinct)
                && ex.command_route_available)
    }

    /// True when a clean segmented control stays a compact mode toggle: it never masquerades as
    /// navigation, is never oversized, shows the selected segment, offers keyboard cycling, discloses a
    /// broadened scope, keeps a locked / degraded state distinct, and offers a command trace path.
    fn segmented_is_honest(ex: &M5ResolvedSegmentedControl) -> bool {
        !ex.is_clean()
            || (!ex.masquerades_as_navigation
                && !ex.oversized_segment_set
                && ex.selected_segment_shown
                && ex.keyboard_cycling_available
                && (!ex.scope_needs_disclosure || ex.scope_disclosed)
                && (!ex.disposition_is_blocked || ex.blocked_state_distinct)
                && ex.command_route_available)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.split_button_examples.iter().all(Self::split_is_honest)
            && self
                .segmented_control_examples
                .iter()
                .all(Self::segmented_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SplitButtonSegmentedControlVocabularySet {
    /// Interaction-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Split-default-posture tokens (bound from the frozen matrix).
    pub split_default_postures: Vec<String>,
    /// Segmented-mode tokens (bound from the frozen matrix).
    pub segmented_modes: Vec<String>,
    /// Button-emphasis tokens (bound from the frozen matrix).
    pub button_emphases: Vec<String>,
    /// Alternate-visibility tokens (minted by this lane).
    pub alternate_visibilities: Vec<String>,
    /// Scope-impact tokens (minted by this lane).
    pub scope_impacts: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Split-button degrade-reason tokens.
    pub split_button_degrade_reasons: Vec<String>,
    /// Segmented-control degrade-reason tokens.
    pub segmented_control_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SplitButtonSegmentedControlVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5CoreControlDisposition::ALL, |v| v.as_str()),
            split_default_postures: tokens(&M5SplitDefaultPosture::ALL, |v| v.as_str()),
            segmented_modes: tokens(&M5SegmentedMode::ALL, |v| v.as_str()),
            button_emphases: tokens(&M5ButtonEmphasis::ALL, |v| v.as_str()),
            alternate_visibilities: tokens(&M5SplitAlternateVisibility::ALL, |v| v.as_str()),
            scope_impacts: tokens(&M5SplitScopeImpact::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5SplitSegmentedSurfaceContext::ALL, |v| v.as_str()),
            split_button_degrade_reasons: tokens(&M5SplitButtonDegradeReason::ALL, |v| v.as_str()),
            segmented_control_degrade_reasons: tokens(&M5SegmentedControlDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5SplitSegmentedAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5SplitSegmentedNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SplitSegmentedExportField::ALL, |v| v.as_str()),
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
pub struct M5SplitButtonSegmentedControlGovernanceReview {
    /// The split button names its permanent primary action label and default posture.
    pub split_names_primary_label_and_posture: bool,
    /// The split button keeps its default the safe action.
    pub split_keeps_default_the_safe_action: bool,
    /// The split button keeps alternates visible in the adjacent menu.
    pub split_keeps_alternates_visible_in_adjacent_menu: bool,
    /// The split button never promotes a riskier alternate to the default on stale state.
    pub split_never_promotes_riskier_alternate_on_stale_state: bool,
    /// The segmented control stays a small mode / view toggle.
    pub segmented_stays_a_small_mode_or_view_toggle: bool,
    /// The segmented control never masquerades as top-level navigation.
    pub segmented_never_masquerades_as_navigation: bool,
    /// The segmented control exposes explicit selected-mode truth.
    pub segmented_exposes_selected_mode_truth: bool,
    /// The segmented control offers keyboard cycling.
    pub segmented_offers_keyboard_cycling: bool,
    /// A broadened scope is always disclosed, preserving review-state continuity.
    pub broadened_scope_is_always_disclosed: bool,
    /// Locked / degraded semantics are never hidden behind generic disabled chrome.
    pub locked_and_degraded_never_hidden_behind_disabled: bool,
    /// Both controls bind one canonical command with a command trace path.
    pub both_bind_canonical_command_with_trace: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SplitButtonSegmentedControlConsumerProjection {
    /// Review surfaces consume the shared split-button and segmented-control vocabulary.
    pub review_surfaces_consume_split_and_segmented_vocabulary: bool,
    /// Forms surfaces consume the shared split-button vocabulary.
    pub forms_surfaces_consume_split_vocabulary: bool,
    /// Settings surfaces consume the shared segmented-control vocabulary.
    pub settings_surfaces_consume_segmented_vocabulary: bool,
    /// Search surfaces consume the shared segmented-control vocabulary.
    pub search_surfaces_consume_segmented_vocabulary: bool,
    /// Default-action and mode facts trace back to one canonical component contract.
    pub default_and_mode_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical control source.
    pub support_export_reads_single_control_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SplitButtonSegmentedControlProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the control.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SplitButtonSegmentedControlReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SplitButtonSegmentedControlControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SplitButtonSegmentedControlControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5SplitButtonSegmentedControlControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SplitButtonSegmentedControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SplitButtonSegmentedControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SplitButtonSegmentedControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SplitButtonSegmentedControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SplitButtonSegmentedControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 split-button / segmented-control controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SplitButtonSegmentedControlControlsPacket {
    /// Record kind; must equal [`M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5SplitButtonSegmentedControlControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SplitButtonSegmentedControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SplitButtonSegmentedControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SplitButtonSegmentedControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SplitButtonSegmentedControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SplitButtonSegmentedControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SplitButtonSegmentedControlControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5SplitButtonSegmentedControlControlsPacketInput) -> Self {
        Self {
            record_kind: M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SplitButtonSegmentedControlControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_RECORD_KIND {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_VERSION {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 split-button / segmented-control controls packet serializes"),
        ) {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::RawMaterialInExport);
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
            .expect("m5 split-button / segmented-control controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,split_button_examples,segmented_control_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .split_button_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.segmented_control_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.split_button_examples.len(),
                row.segmented_control_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Split-Button and Segmented-Control Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Split default postures: {}\n",
            self.vocabulary_set.split_default_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Segmented modes: {}\n",
            self.vocabulary_set.segmented_modes.join(", ")
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
                "  - Split-button examples: {} / segmented-control examples: {}\n",
                row.split_button_examples.len(),
                row.segmented_control_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5SplitButtonSegmentedControlControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SplitButtonSegmentedControlControlsViolation>),
}

impl fmt::Display for M5SplitButtonSegmentedControlControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 split-button / segmented-control controls export parse failed: {error}"
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
                    "m5 split-button / segmented-control controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SplitButtonSegmentedControlControlsArtifactError {}

/// Validation failures emitted by [`M5SplitButtonSegmentedControlControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SplitButtonSegmentedControlControlsViolation {
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
    /// A controls row carries a dishonest clean example (riskier default, hidden alternate, undisclosed
    /// scope, stealth navigation, oversized set, hidden lock, or missing trace).
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
    /// Safe-default and alternate behavior is not proven: clean split buttons do not cover the safe
    /// default postures with alternates visible, or no riskier-default / hidden-alternate example
    /// degrades, or a clean split defaults riskier or hides an alternate.
    SafeDefaultAndAlternateBehaviorNotProven,
    /// Selected-mode and keyboard truth is not proven: no clean segmented control exposes an explicit
    /// selected mode with keyboard cycling, or no stealth-navigation / keyboard-missing example
    /// degrades, or a clean segmented control masquerades as navigation or is oversized.
    SelectedModeAndKeyboardTruthNotProven,
    /// Default-action and mode traceability is not proven: no clean split button and clean segmented
    /// control both offer a command-backed detail entrypoint, or no undisclosed-scope example degrades.
    DefaultAndModeTraceabilityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SplitButtonSegmentedControlControlsViolation {
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
            Self::SafeDefaultAndAlternateBehaviorNotProven => {
                "safe_default_and_alternate_behavior_not_proven"
            }
            Self::SelectedModeAndKeyboardTruthNotProven => {
                "selected_mode_and_keyboard_truth_not_proven"
            }
            Self::DefaultAndModeTraceabilityNotProven => "default_and_mode_traceability_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_split_button_segmented_control_controls_export() -> Result<
    M5SplitButtonSegmentedControlControlsPacket,
    M5SplitButtonSegmentedControlControlsArtifactError,
> {
    let packet: M5SplitButtonSegmentedControlControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-split-button-segmented-control-controls-proof/support_export.json"
        )))
        .map_err(M5SplitButtonSegmentedControlControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SplitButtonSegmentedControlControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_SCHEMA_REF,
        M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_SPLIT_BUTTON_SCHEMA_REF,
        M5_SEGMENTED_CONTROL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5SplitButtonSegmentedControlControlsViolation::NoControlsRows);
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
            violations.push(M5SplitButtonSegmentedControlControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5SplitButtonSegmentedControlControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5SplitButtonSegmentedControlControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SPLIT_BUTTON_SCHEMA_REF)
            || !refs.contains(M5_SEGMENTED_CONTROL_SCHEMA_REF)
        {
            violations
                .push(M5SplitButtonSegmentedControlControlsViolation::ComponentSchemaRefMissing);
        }
        if row.split_button_examples.is_empty() || row.segmented_control_examples.is_empty() {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5SplitButtonSegmentedControlControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.split_names_primary_label_and_posture,
        review.split_keeps_default_the_safe_action,
        review.split_keeps_alternates_visible_in_adjacent_menu,
        review.split_never_promotes_riskier_alternate_on_stale_state,
        review.segmented_stays_a_small_mode_or_view_toggle,
        review.segmented_never_masquerades_as_navigation,
        review.segmented_exposes_selected_mode_truth,
        review.segmented_offers_keyboard_cycling,
        review.broadened_scope_is_always_disclosed,
        review.locked_and_degraded_never_hidden_behind_disabled,
        review.both_bind_canonical_command_with_trace,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5SplitButtonSegmentedControlControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_surfaces_consume_split_and_segmented_vocabulary,
        projection.forms_surfaces_consume_split_vocabulary,
        projection.settings_surfaces_consume_segmented_vocabulary,
        projection.search_surfaces_consume_segmented_vocabulary,
        projection.default_and_mode_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_control_source,
    ] {
        if !ok {
            violations
                .push(M5SplitButtonSegmentedControlControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SplitButtonSegmentedControlControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SplitButtonSegmentedControlControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5SplitButtonSegmentedControlControlsPacket,
    violations: &mut Vec<M5SplitButtonSegmentedControlControlsViolation>,
) {
    let splits = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.split_button_examples.iter())
    };
    let segmenteds = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.segmented_control_examples.iter())
    };

    // AC1: the first claimed M5 consumers use split buttons without hiding alternate behavior or
    // widening risk. Clean split buttons cover at least the safe default postures with alternates
    // visible, a riskier-alternate-default example degrades, a hidden-alternate example degrades, and no
    // clean split defaults riskier or hides an alternate.
    let clean_postures: BTreeSet<String> = splits()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.default_posture.clone())
        .collect();
    let posture_grammar_covered = ["primary_default_safe", "explicit_alternate"]
        .iter()
        .all(|p| clean_postures.contains(*p));
    let riskier_default_degrades = splits().any(|ex| {
        ex.degrade_reason == Some(M5SplitButtonDegradeReason::RiskierAlternateBecameDefault)
    });
    let hidden_alternate_degrades = splits().any(|ex| {
        ex.degrade_reason == Some(M5SplitButtonDegradeReason::AlternateHiddenBehindDefault)
    });
    let no_clean_riskier_or_hidden = splits().all(|ex| {
        !(ex.is_clean() && (ex.stale_state_promoted_riskier_alternate || !ex.alternate_visible))
    });
    if !(posture_grammar_covered
        && riskier_default_degrades
        && hidden_alternate_degrades
        && no_clean_riskier_or_hidden)
    {
        violations
            .push(M5SplitButtonSegmentedControlControlsViolation::SafeDefaultAndAlternateBehaviorNotProven);
    }

    // AC2: segmented controls keep mode state and default-action truth explicit and never become stealth
    // navigation. At least one clean segmented control exposes an explicit selected mode with keyboard
    // cycling, a stealth-navigation example degrades, a keyboard-missing example degrades, and no clean
    // segmented control masquerades as navigation or is oversized.
    let clean_mode_toggle = segmenteds().any(|ex| {
        ex.is_clean()
            && ex.selected_segment_shown
            && ex.keyboard_cycling_available
            && !ex.command_id.trim().is_empty()
    });
    let stealth_navigation_degrades = segmenteds().any(|ex| {
        ex.degrade_reason == Some(M5SegmentedControlDegradeReason::UsedAsStealthNavigation)
    });
    let keyboard_missing_degrades = segmenteds().any(|ex| {
        ex.degrade_reason == Some(M5SegmentedControlDegradeReason::KeyboardCyclingMissing)
    });
    let no_clean_navigation_or_oversized = segmenteds()
        .all(|ex| !(ex.is_clean() && (ex.masquerades_as_navigation || ex.oversized_segment_set)));
    if !(clean_mode_toggle
        && stealth_navigation_degrades
        && keyboard_missing_degrades
        && no_clean_navigation_or_oversized)
    {
        violations.push(
            M5SplitButtonSegmentedControlControlsViolation::SelectedModeAndKeyboardTruthNotProven,
        );
    }

    // AC3: release/help/support packets can explain why a split-button default or segmented choice was
    // active at the time of export — a user can trace default-action and mode state back to one
    // canonical component contract and command-backed detail entrypoints, and a broadened scope left
    // undisclosed is caught. At least one clean split button and one clean segmented control both expose
    // a command-backed detail entrypoint, and at least one example degrades on undisclosed scope.
    let traceable_split = splits().any(|ex| ex.is_clean() && ex.command_route_available);
    let traceable_segmented = segmenteds().any(|ex| ex.is_clean() && ex.command_route_available);
    let undisclosed_scope_degrades = splits()
        .any(|ex| ex.degrade_reason == Some(M5SplitButtonDegradeReason::BroadenedScopeUndisclosed))
        || segmenteds().any(|ex| {
            ex.degrade_reason == Some(M5SegmentedControlDegradeReason::ModeScopeContinuityBroken)
        });
    if !(traceable_split && traceable_segmented && undisclosed_scope_degrades) {
        violations.push(
            M5SplitButtonSegmentedControlControlsViolation::DefaultAndModeTraceabilityNotProven,
        );
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
pub const IMPLEMENTED_FAMILIES: [M5CoreControlFamily; 2] = [
    M5CoreControlFamily::SplitButton,
    M5CoreControlFamily::SegmentedControl,
];
