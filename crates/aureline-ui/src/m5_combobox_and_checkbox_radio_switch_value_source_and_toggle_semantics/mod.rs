//! Implemented M5 combobox and checkbox / radio / switch primitives.
//!
//! The frozen [core action / input component matrix][matrix] names Aureline's most reused atomic
//! action and input controls and locks their controlled vocabulary. This module is the filterable-
//! selection and boolean-control implement lane over that matrix: it turns the **combobox** and the
//! **checkbox / radio / switch** (toggle control) into resolvers that produce export-safe, honest
//! projections, so a user never has to guess where a combobox's committed value came from, never has
//! to infer whether a checkbox applies immediately or waits for an explicit save, and never loses
//! policy-lock, read-only, or source-of-value truth when the same control is reused across a different
//! M5 lane.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement comboboxes with filterable sets, selected-value disclosure, source-of-value or
//!   support-class tags where relevant, and stable keyboard / screen-reader navigation.**
//!   [`resolve_combobox`] refuses to read as a clean, source-honest selector when the selected value
//!   is undisclosed, the surface context or value source is unresolved, a claimed filterable set does
//!   not offer filtering, a remote / unverified value is presented as a canonical option without a
//!   support-class tag, the effective-value provenance is unresolved or a material provenance is left
//!   undisclosed, keyboard navigation is unstable, a locked / read-only state hides behind generic
//!   disabled chrome, or the canonical command binding / trace path is missing; it degrades instead.
//! * **Implement checkbox, radio, and switch components with explicit immediate-versus-deferred
//!   semantics, current / effective / locked / read-only truth, and no-guesswork differences between
//!   one-of-many and multi-select behavior.** [`resolve_toggle`] degrades when the selected state is
//!   undisclosed, the surface context, toggle semantics, or apply timing is unresolved, a switch is
//!   blurred with a deferred checkbox, one-of-many versus multi-select is ambiguous, a radio group
//!   loses its exclusivity, provenance is unresolved or undisclosed, a locked / read-only state hides
//!   behind generic disabled chrome, or the canonical command binding / trace path is missing.
//! * **Carry policy, imported, detected, default, and user-override provenance into the first reusable
//!   selection / toggle consumers instead of feature-local hint text.** Both resolvers name the
//!   [`M5ControlValueProvenance`] of the effective value and degrade when a provenance that materially
//!   changes trust is left undisclosed, and both always bind back to one canonical command.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5CoreControlDisposition`] interaction-state vocabulary, the [`M5ComboboxValueSource`] value-
//! source vocabulary, and the [`M5ToggleSemantics`] boolean-control vocabulary — so forms, settings,
//! provider, admin, request, and entry surfaces can never fork their own selection or toggle wording.
//! Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_core_action_input_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_combobox_checkbox_radio_switch_controls,
    seeded_m5_combobox_checkbox_radio_switch_controls_entry_ui_preview_narrowed,
    seeded_m5_combobox_checkbox_radio_switch_controls_settings_ui_beta_narrowed,
    M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_core_action_input_component_matrix::{
    M5ComboboxValueSource, M5CoreControlAccessibilityRoute, M5CoreControlConsumerSurface,
    M5CoreControlDeploymentLine, M5CoreControlDisposition, M5CoreControlDowngradeTrigger,
    M5CoreControlFamily, M5CoreControlQualificationClass, M5CoreControlRequiredLabel,
    M5ToggleSemantics, M5_COMBOBOX_SCHEMA_REF, M5_CORE_CONTROL_COMPONENT_DOC_REF,
    M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_TOGGLE_CONTROL_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ComboboxToggleControlsPacket`].
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_RECORD_KIND: &str =
    "implement_m5_combobox_and_checkbox_radio_switch_controls";

/// Schema version for M5 combobox / checkbox-radio-switch controls records.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-combobox-checkbox-radio-switch-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_DOC_REF: &str =
    "docs/components/m5_combobox_and_checkbox_radio_switch_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-combobox-checkbox-radio-switch-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-combobox-checkbox-radio-switch-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-combobox-checkbox-radio-switch-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-combobox-checkbox-radio-switch-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5ControlConsumerSurface = M5CoreControlConsumerSurface;

/// Controlled render context — which claimed M5 surface renders the combobox or toggle control, so a
/// control's meaning stays stable whether it appears in a settings row, a provider row, an admin row, a
/// request flow, or a start-center entry field. Minted by this lane, tracking the goal surfaces directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlSurfaceContext {
    /// A settings row.
    SettingsRow,
    /// A provider / model configuration row.
    ProviderRow,
    /// An admin / policy row.
    AdminRow,
    /// A request / run configuration flow.
    RequestFlow,
    /// The start-center entry field.
    EntryField,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ControlSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SettingsRow,
        Self::ProviderRow,
        Self::AdminRow,
        Self::RequestFlow,
        Self::EntryField,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsRow => "settings_row",
            Self::ProviderRow => "provider_row",
            Self::AdminRow => "admin_row",
            Self::RequestFlow => "request_flow",
            Self::EntryField => "entry_field",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled effective-value provenance — where a combobox's or toggle's *current effective* value came
/// from, so policy, imported, detected, default, and user-override origins stay first-class facts instead
/// of feature-local hint text. Minted by this lane and consumed by both resolvers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlValueProvenance {
    /// The value is enforced by policy and cannot be freely changed.
    PolicyEnforced,
    /// The value was imported from an external source.
    Imported,
    /// The value was auto-detected by the system.
    Detected,
    /// The value is the applied default.
    DefaultApplied,
    /// The value was explicitly chosen by the user.
    UserOverride,
    /// The provenance cannot currently be resolved.
    ProvenanceUnknown,
}

impl M5ControlValueProvenance {
    /// Every provenance, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyEnforced,
        Self::Imported,
        Self::Detected,
        Self::DefaultApplied,
        Self::UserOverride,
        Self::ProvenanceUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyEnforced => "policy_enforced",
            Self::Imported => "imported",
            Self::Detected => "detected",
            Self::DefaultApplied => "default_applied",
            Self::UserOverride => "user_override",
            Self::ProvenanceUnknown => "provenance_unknown",
        }
    }

    /// Whether this provenance materially changes trust and must therefore be disclosed (a non-user origin
    /// or the unresolved sentinel, which can never be presented as a plain user-chosen value).
    pub const fn needs_disclosure(self) -> bool {
        matches!(
            self,
            Self::PolicyEnforced | Self::Imported | Self::Detected | Self::ProvenanceUnknown
        )
    }

    /// Whether the provenance is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ProvenanceUnknown)
    }
}

/// Controlled toggle apply timing — when a checkbox / radio / switch change takes effect, so an immediate
/// switch is never blurred with a deferred checkbox and a user always knows whether a change is live or
/// waiting for an explicit save. Minted by this lane because the frozen matrix carries the boolean-control
/// *kind* but not the apply timing the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToggleApplyTiming {
    /// The change applies immediately.
    AppliesImmediately,
    /// The change is deferred until an explicit save.
    DeferredUntilSave,
    /// The change is staged in a review / batch before it applies.
    StagedInReview,
    /// The change requires an explicit confirmation step.
    RequiresConfirmation,
    /// The change is blocked (policy / permission), shown distinctly.
    ApplyBlocked,
    /// The apply timing cannot currently be resolved.
    TimingUnknown,
}

impl M5ToggleApplyTiming {
    /// Every apply timing, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AppliesImmediately,
        Self::DeferredUntilSave,
        Self::StagedInReview,
        Self::RequiresConfirmation,
        Self::ApplyBlocked,
        Self::TimingUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AppliesImmediately => "applies_immediately",
            Self::DeferredUntilSave => "deferred_until_save",
            Self::StagedInReview => "staged_in_review",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::ApplyBlocked => "apply_blocked",
            Self::TimingUnknown => "timing_unknown",
        }
    }

    /// Whether the apply timing is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::TimingUnknown)
    }

    /// Whether the change is deferred rather than immediate (deferred-until-save, staged, or confirm).
    pub const fn is_deferred(self) -> bool {
        matches!(
            self,
            Self::DeferredUntilSave | Self::StagedInReview | Self::RequiresConfirmation
        )
    }

    /// Whether applying the change is blocked and must be shown distinctly, never behind disabled chrome.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ApplyBlocked)
    }
}

/// One mandatory rendered part a combobox or toggle control must be able to show, so no label, selected
/// value, option list, filter, value-source tag, lock state, apply-timing, or command fact is left
/// implicit behind a placeholder, a tooltip, or a secondary panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlAnatomyPart {
    /// The control's stable identity / permanent label.
    Identity,
    /// The control's current typed interaction disposition.
    State,
    /// The non-visual keyboard route to the control.
    KeyboardRoute,
    /// The permanent label text.
    LabelText,
    /// The disclosed selected value / state.
    SelectedValue,
    /// The filter input (combobox).
    FilterInput,
    /// The option list (combobox).
    OptionList,
    /// The source-of-value / support-class tag.
    ValueSourceTag,
    /// The lock / read-only state cue.
    LockState,
    /// The apply-timing cue (toggle control).
    ApplyTimingCue,
    /// The render / surface context.
    SurfaceContext,
    /// The canonical command binding.
    CommandBinding,
}

impl M5ControlAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::LabelText,
        Self::SelectedValue,
        Self::FilterInput,
        Self::OptionList,
        Self::ValueSourceTag,
        Self::LockState,
        Self::ApplyTimingCue,
        Self::SurfaceContext,
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
            Self::LabelText => "label_text",
            Self::SelectedValue => "selected_value",
            Self::FilterInput => "filter_input",
            Self::OptionList => "option_list",
            Self::ValueSourceTag => "value_source_tag",
            Self::LockState => "lock_state",
            Self::ApplyTimingCue => "apply_timing_cue",
            Self::SurfaceContext => "surface_context",
            Self::CommandBinding => "command_binding",
        }
    }
}

/// Next safe action a control surfaces so a user is never left without a route to inspect the selected
/// value, value source, lock state, apply timing, or command behind a degraded control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlNextAction {
    /// Open the command-backed action / command detail.
    OpenCommandDetail,
    /// Inspect the selected value / state.
    InspectSelectedValue,
    /// Inspect the value source / provenance tag.
    InspectValueSource,
    /// Review a locked / read-only / blocked control.
    ReviewLockedOrReadOnly,
    /// Review the apply-timing / immediate-versus-deferred behavior.
    ReviewApplyTiming,
    /// No action is needed; the control is clean.
    NoActionNeeded,
}

impl M5ControlNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenCommandDetail,
        Self::InspectSelectedValue,
        Self::InspectValueSource,
        Self::ReviewLockedOrReadOnly,
        Self::ReviewApplyTiming,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCommandDetail => "open_command_detail",
            Self::InspectSelectedValue => "inspect_selected_value",
            Self::InspectValueSource => "inspect_value_source",
            Self::ReviewLockedOrReadOnly => "review_locked_or_read_only",
            Self::ReviewApplyTiming => "review_apply_timing",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlExportField {
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
    /// The combobox value source named by the combobox.
    ValueSource,
    /// The effective-value provenance named by both controls.
    ValueProvenance,
    /// The toggle semantics named by the toggle control.
    ToggleSemantics,
    /// The apply timing named by the toggle control.
    ApplyTiming,
    /// The render / surface context named by both controls.
    SurfaceContext,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ControlExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ValueSource,
        Self::ValueProvenance,
        Self::ToggleSemantics,
        Self::ApplyTiming,
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
            Self::ValueSource => "value_source",
            Self::ValueProvenance => "value_provenance",
            Self::ToggleSemantics => "toggle_semantics",
            Self::ApplyTiming => "apply_timing",
            Self::SurfaceContext => "surface_context",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a combobox degraded below a clean, source-honest, filterable-selector state. The degrade-first
/// ladder returns one of these instead of ever letting an undisclosed or unverified selection read as a
/// clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComboboxDegradeReason {
    /// The selected value is not disclosed; a user cannot tell what is currently chosen.
    SelectedValueUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The value source cannot currently be resolved.
    ValueSourceUnresolved,
    /// A claimed filterable set does not offer filtering.
    FilterabilityMissing,
    /// A remote / unverified value is presented as a canonical option without a support-class tag.
    UnverifiedValuePresentedAsCanonical,
    /// The effective-value provenance cannot currently be resolved.
    ValueProvenanceUnresolved,
    /// A provenance that materially changes trust is left undisclosed.
    ValueProvenanceUndisclosed,
    /// Keyboard / screen-reader navigation is unstable.
    KeyboardNavigationUnstable,
    /// A locked / read-only state hides behind generic disabled chrome.
    LockedOrReadOnlyHiddenBehindDisabled,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the control is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ComboboxDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::SelectedValueUnstated,
        Self::SurfaceContextUnresolved,
        Self::ValueSourceUnresolved,
        Self::FilterabilityMissing,
        Self::UnverifiedValuePresentedAsCanonical,
        Self::ValueProvenanceUnresolved,
        Self::ValueProvenanceUndisclosed,
        Self::KeyboardNavigationUnstable,
        Self::LockedOrReadOnlyHiddenBehindDisabled,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedValueUnstated => "selected_value_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ValueSourceUnresolved => "value_source_unresolved",
            Self::FilterabilityMissing => "filterability_missing",
            Self::UnverifiedValuePresentedAsCanonical => "unverified_value_presented_as_canonical",
            Self::ValueProvenanceUnresolved => "value_provenance_unresolved",
            Self::ValueProvenanceUndisclosed => "value_provenance_undisclosed",
            Self::KeyboardNavigationUnstable => "keyboard_navigation_unstable",
            Self::LockedOrReadOnlyHiddenBehindDisabled => {
                "locked_or_read_only_hidden_behind_disabled"
            }
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ControlNextAction {
        match self {
            Self::SelectedValueUnstated
            | Self::SurfaceContextUnresolved
            | Self::FilterabilityMissing
            | Self::KeyboardNavigationUnstable => M5ControlNextAction::InspectSelectedValue,
            Self::ValueSourceUnresolved
            | Self::UnverifiedValuePresentedAsCanonical
            | Self::ValueProvenanceUnresolved
            | Self::ValueProvenanceUndisclosed => M5ControlNextAction::InspectValueSource,
            Self::LockedOrReadOnlyHiddenBehindDisabled => {
                M5ControlNextAction::ReviewLockedOrReadOnly
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing | Self::ProofStale => {
                M5ControlNextAction::OpenCommandDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::SelectedValueUnstated
            | Self::ValueSourceUnresolved
            | Self::UnverifiedValuePresentedAsCanonical
            | Self::ValueProvenanceUnresolved
            | Self::ValueProvenanceUndisclosed => {
                M5CoreControlDowngradeTrigger::ValueSourceUnstated
            }
            Self::LockedOrReadOnlyHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SurfaceContextUnresolved
            | Self::FilterabilityMissing
            | Self::KeyboardNavigationUnstable => {
                M5CoreControlDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// Reason a toggle control (checkbox / radio / switch) degraded below a clean state that keeps its
/// semantics, apply timing, provenance, and lock truth explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToggleDegradeReason {
    /// The selected on / off / indeterminate state is not disclosed.
    SelectedStateUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The toggle semantics cannot currently be resolved.
    ToggleSemanticsUnresolved,
    /// The apply timing cannot currently be resolved.
    ApplyTimingUnresolved,
    /// A switch is blurred with a deferred checkbox (a switch must apply immediately).
    SwitchBlurredWithDeferredCheckbox,
    /// One-of-many versus multi-select behavior is ambiguous.
    OneOfManyVersusMultiSelectAmbiguous,
    /// A radio group has lost its exclusivity.
    GroupExclusivityLost,
    /// The effective-value provenance cannot currently be resolved.
    ValueProvenanceUnresolved,
    /// A provenance that materially changes trust is left undisclosed.
    ValueProvenanceUndisclosed,
    /// A locked / read-only state hides behind generic disabled chrome.
    LockedOrReadOnlyHiddenBehindDisabled,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the control is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ToggleDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::SelectedStateUnstated,
        Self::SurfaceContextUnresolved,
        Self::ToggleSemanticsUnresolved,
        Self::ApplyTimingUnresolved,
        Self::SwitchBlurredWithDeferredCheckbox,
        Self::OneOfManyVersusMultiSelectAmbiguous,
        Self::GroupExclusivityLost,
        Self::ValueProvenanceUnresolved,
        Self::ValueProvenanceUndisclosed,
        Self::LockedOrReadOnlyHiddenBehindDisabled,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedStateUnstated => "selected_state_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ToggleSemanticsUnresolved => "toggle_semantics_unresolved",
            Self::ApplyTimingUnresolved => "apply_timing_unresolved",
            Self::SwitchBlurredWithDeferredCheckbox => "switch_blurred_with_deferred_checkbox",
            Self::OneOfManyVersusMultiSelectAmbiguous => {
                "one_of_many_versus_multi_select_ambiguous"
            }
            Self::GroupExclusivityLost => "group_exclusivity_lost",
            Self::ValueProvenanceUnresolved => "value_provenance_unresolved",
            Self::ValueProvenanceUndisclosed => "value_provenance_undisclosed",
            Self::LockedOrReadOnlyHiddenBehindDisabled => {
                "locked_or_read_only_hidden_behind_disabled"
            }
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ControlNextAction {
        match self {
            Self::SelectedStateUnstated
            | Self::SurfaceContextUnresolved
            | Self::OneOfManyVersusMultiSelectAmbiguous
            | Self::GroupExclusivityLost => M5ControlNextAction::InspectSelectedValue,
            Self::ToggleSemanticsUnresolved
            | Self::ApplyTimingUnresolved
            | Self::SwitchBlurredWithDeferredCheckbox => M5ControlNextAction::ReviewApplyTiming,
            Self::ValueProvenanceUnresolved | Self::ValueProvenanceUndisclosed => {
                M5ControlNextAction::InspectValueSource
            }
            Self::LockedOrReadOnlyHiddenBehindDisabled => {
                M5ControlNextAction::ReviewLockedOrReadOnly
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing | Self::ProofStale => {
                M5ControlNextAction::OpenCommandDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::SwitchBlurredWithDeferredCheckbox => {
                M5CoreControlDowngradeTrigger::SwitchAndDeferredCheckboxBlurred
            }
            Self::ToggleSemanticsUnresolved
            | Self::OneOfManyVersusMultiSelectAmbiguous
            | Self::GroupExclusivityLost => M5CoreControlDowngradeTrigger::StateTaxonomyDrifted,
            Self::ValueProvenanceUnresolved | Self::ValueProvenanceUndisclosed => {
                M5CoreControlDowngradeTrigger::ValueSourceUnstated
            }
            Self::LockedOrReadOnlyHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SelectedStateUnstated
            | Self::SurfaceContextUnresolved
            | Self::ApplyTimingUnresolved => {
                M5CoreControlDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// True when a disposition names a locked / read-only / degraded state that must never hide behind
/// generic disabled chrome.
fn disposition_requires_distinct_treatment(disposition: M5CoreControlDisposition) -> bool {
    matches!(
        disposition,
        M5CoreControlDisposition::Locked
            | M5CoreControlDisposition::ReadOnly
            | M5CoreControlDisposition::Degraded
    )
}

/// True when a combobox value source is resolved (not the unknown sentinel).
fn value_source_is_resolved(source: M5ComboboxValueSource) -> bool {
    !matches!(source, M5ComboboxValueSource::SourceUnknown)
}

/// True when a combobox value source is unverified and must carry a support-class tag before it can be
/// presented alongside canonical options (remote-backed or custom-unverified).
fn value_source_is_unverified(source: M5ComboboxValueSource) -> bool {
    matches!(
        source,
        M5ComboboxValueSource::RemoteBacked | M5ComboboxValueSource::CustomUnverified
    )
}

/// True when a toggle semantics is resolved (not the unknown sentinel).
fn toggle_semantics_is_resolved(semantics: M5ToggleSemantics) -> bool {
    !matches!(semantics, M5ToggleSemantics::SemanticsUnknown)
}

/// True when a toggle semantics is a switch, which must apply immediately.
fn toggle_semantics_is_switch(semantics: M5ToggleSemantics) -> bool {
    matches!(semantics, M5ToggleSemantics::SwitchImmediate)
}

/// True when a toggle semantics is an exclusive radio, whose group exclusivity must be enforced.
fn toggle_semantics_is_exclusive(semantics: M5ToggleSemantics) -> bool {
    matches!(semantics, M5ToggleSemantics::RadioExclusive)
}

/// Input to [`resolve_combobox`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ComboboxResolutionInput {
    /// Stable identity of the combobox instance.
    pub combobox_id: String,
    /// The permanent label shown.
    pub label: String,
    /// The disclosed selected value; empty means undisclosed.
    pub selected_value: String,
    /// True when the selected value is disclosed.
    pub selected_value_disclosed: bool,
    /// The value source (from the frozen matrix vocabulary).
    pub value_source: M5ComboboxValueSource,
    /// The support-class / source tag shown; empty means untagged.
    pub support_class_tag: String,
    /// True when a support-class / source tag is present.
    pub support_class_tagged: bool,
    /// True when this combobox claims a filterable set.
    pub requires_filter: bool,
    /// True when a filter input is offered.
    pub filter_offered: bool,
    /// The effective-value provenance.
    pub value_provenance: M5ControlValueProvenance,
    /// True when a provenance that materially changes trust is disclosed.
    pub provenance_disclosed: bool,
    /// True when keyboard / screen-reader navigation is stable.
    pub keyboard_navigation_stable: bool,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// True when a locked / read-only state is shown distinctly, never behind disabled chrome.
    pub blocked_state_distinct: bool,
    /// The render / surface context.
    pub surface_context: M5ControlSurfaceContext,
    /// The canonical command ID this control binds back to; empty means unstated.
    pub command_id: String,
    /// True when a command-backed path to inspect the control is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe combobox projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCombobox {
    /// Stable identity of the combobox instance.
    pub combobox_id: String,
    /// The permanent label named by the combobox.
    pub label: String,
    /// The disclosed selected value.
    pub selected_value: String,
    /// Whether the selected value is disclosed.
    pub selected_value_disclosed: bool,
    /// The value-source token named by the combobox.
    pub value_source: String,
    /// Whether the value source is resolved (not the unknown sentinel).
    pub value_source_resolved: bool,
    /// Whether the value source is unverified (remote-backed or custom-unverified).
    pub value_source_is_unverified: bool,
    /// The support-class / source tag named by the combobox.
    pub support_class_tag: String,
    /// Whether a support-class / source tag is present.
    pub support_class_tagged: bool,
    /// Whether this combobox claims a filterable set.
    pub requires_filter: bool,
    /// Whether a filter input is offered.
    pub filter_offered: bool,
    /// The effective-value provenance token named by the combobox.
    pub value_provenance: String,
    /// Whether the provenance is resolved (not the unknown sentinel).
    pub value_provenance_resolved: bool,
    /// Whether the provenance materially changes trust and must be disclosed.
    pub value_provenance_needs_disclosure: bool,
    /// Whether a material provenance is disclosed.
    pub provenance_disclosed: bool,
    /// Whether keyboard / screen-reader navigation is stable.
    pub keyboard_navigation_stable: bool,
    /// The interaction-disposition token named by the combobox.
    pub disposition: String,
    /// Whether the disposition needs distinct blocked treatment (locked / read-only / degraded).
    pub disposition_requires_distinct_treatment: bool,
    /// Whether a locked / read-only state is shown distinctly, never behind disabled chrome.
    pub blocked_state_distinct: bool,
    /// The render / surface-context token named by the combobox.
    pub surface_context: String,
    /// The canonical command ID named by the combobox.
    pub command_id: String,
    /// Whether a command-backed path to inspect the control is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the combobox could not read as a clean, source-honest selector.
    pub degrade_reason: Option<M5ComboboxDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ControlNextAction,
    /// Whether the value source and lock state read honestly at a glance (clean selector naming facts).
    pub value_source_and_lock_honest_at_a_glance: bool,
}

impl M5ResolvedCombobox {
    /// Whether this combobox reads as a clean, source-honest selector.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_toggle`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ToggleResolutionInput {
    /// Stable identity of the toggle-control instance.
    pub toggle_id: String,
    /// The permanent label shown.
    pub label: String,
    /// The disclosed selected state (e.g. "on" / "off" / "indeterminate"); empty means undisclosed.
    pub selected_state: String,
    /// True when the selected state is disclosed.
    pub selected_state_disclosed: bool,
    /// The toggle semantics (from the frozen matrix vocabulary).
    pub toggle_semantics: M5ToggleSemantics,
    /// The apply timing.
    pub apply_timing: M5ToggleApplyTiming,
    /// True when one-of-many versus multi-select behavior is explicit and unambiguous.
    pub selection_arity_explicit: bool,
    /// True when a radio group's exclusivity is enforced.
    pub group_exclusivity_enforced: bool,
    /// The effective-value provenance.
    pub value_provenance: M5ControlValueProvenance,
    /// True when a provenance that materially changes trust is disclosed.
    pub provenance_disclosed: bool,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// True when a locked / read-only state is shown distinctly, never behind disabled chrome.
    pub blocked_state_distinct: bool,
    /// The render / surface context.
    pub surface_context: M5ControlSurfaceContext,
    /// The canonical command ID this control binds back to; empty means unstated.
    pub command_id: String,
    /// True when a command-backed path to inspect the control is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe toggle-control projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedToggle {
    /// Stable identity of the toggle-control instance.
    pub toggle_id: String,
    /// The permanent label named by the toggle control.
    pub label: String,
    /// The disclosed selected state.
    pub selected_state: String,
    /// Whether the selected state is disclosed.
    pub selected_state_disclosed: bool,
    /// The toggle-semantics token named by the toggle control.
    pub toggle_semantics: String,
    /// Whether the toggle semantics is resolved (not the unknown sentinel).
    pub toggle_semantics_resolved: bool,
    /// Whether the semantics is a switch (must apply immediately).
    pub semantics_is_switch: bool,
    /// Whether the semantics is an exclusive radio.
    pub semantics_is_exclusive: bool,
    /// The apply-timing token named by the toggle control.
    pub apply_timing: String,
    /// Whether the apply timing is resolved (not the unknown sentinel).
    pub apply_timing_resolved: bool,
    /// Whether the change is deferred rather than immediate.
    pub apply_timing_is_deferred: bool,
    /// Whether applying the change is blocked.
    pub apply_timing_is_blocked: bool,
    /// Whether one-of-many versus multi-select behavior is explicit.
    pub selection_arity_explicit: bool,
    /// Whether a radio group's exclusivity is enforced.
    pub group_exclusivity_enforced: bool,
    /// The effective-value provenance token named by the toggle control.
    pub value_provenance: String,
    /// Whether the provenance is resolved (not the unknown sentinel).
    pub value_provenance_resolved: bool,
    /// Whether the provenance materially changes trust and must be disclosed.
    pub value_provenance_needs_disclosure: bool,
    /// Whether a material provenance is disclosed.
    pub provenance_disclosed: bool,
    /// The interaction-disposition token named by the toggle control.
    pub disposition: String,
    /// Whether the disposition needs distinct blocked treatment (locked / read-only / degraded).
    pub disposition_requires_distinct_treatment: bool,
    /// Whether a locked / read-only state is shown distinctly, never behind disabled chrome.
    pub blocked_state_distinct: bool,
    /// The render / surface-context token named by the toggle control.
    pub surface_context: String,
    /// The canonical command ID named by the toggle control.
    pub command_id: String,
    /// Whether a command-backed path to inspect the control is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the toggle could not read as a clean semantics / timing state.
    pub degrade_reason: Option<M5ToggleDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ControlNextAction,
    /// Whether the semantics and apply timing read honestly at a glance (clean control naming facts).
    pub semantics_and_timing_honest_at_a_glance: bool,
}

impl M5ResolvedToggle {
    /// Whether this toggle reads as a clean semantics / timing state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ControlResolutionError {
    /// The combobox id was empty.
    EmptyComboboxId,
    /// The toggle id was empty.
    EmptyToggleId,
    /// A control carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ControlResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyComboboxId => "empty_combobox_id",
            Self::EmptyToggleId => "empty_toggle_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ControlResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 combobox / toggle-control resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ControlResolutionError {}

/// Resolves a combobox so its value source and lock state read honestly at a glance: the combobox names
/// its disclosed selected value, its value source (canonical / filtered / free-text / remote / custom),
/// any support-class tag a remote or unverified value requires, its effective-value provenance (policy /
/// imported / detected / default / user-override) with disclosure when it materially changes trust, and
/// its interaction disposition, offers a filter when it claims a filterable set and stable keyboard
/// navigation, keeps a locked / read-only state distinct from generic disabled chrome, and always binds
/// back to one canonical command.
pub fn resolve_combobox(
    input: M5ComboboxResolutionInput,
) -> Result<M5ResolvedCombobox, M5ControlResolutionError> {
    if input.combobox_id.trim().is_empty() {
        return Err(M5ControlResolutionError::EmptyComboboxId);
    }
    if string_is_forbidden(&input.combobox_id)
        || string_is_forbidden(&input.label)
        || string_is_forbidden(&input.selected_value)
        || string_is_forbidden(&input.support_class_tag)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5ControlResolutionError::ForbiddenMaterial);
    }

    let disposition_requires_distinct = disposition_requires_distinct_treatment(input.disposition);
    let value_source_resolved = value_source_is_resolved(input.value_source);
    let unverified = value_source_is_unverified(input.value_source);
    let provenance_needs_disclosure = input.value_provenance.needs_disclosure();

    let degrade_reason = if !input.selected_value_disclosed {
        Some(M5ComboboxDegradeReason::SelectedValueUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ComboboxDegradeReason::SurfaceContextUnresolved)
    } else if !value_source_resolved {
        Some(M5ComboboxDegradeReason::ValueSourceUnresolved)
    } else if input.requires_filter && !input.filter_offered {
        Some(M5ComboboxDegradeReason::FilterabilityMissing)
    } else if unverified && !input.support_class_tagged {
        Some(M5ComboboxDegradeReason::UnverifiedValuePresentedAsCanonical)
    } else if !input.value_provenance.is_resolved() {
        Some(M5ComboboxDegradeReason::ValueProvenanceUnresolved)
    } else if provenance_needs_disclosure && !input.provenance_disclosed {
        Some(M5ComboboxDegradeReason::ValueProvenanceUndisclosed)
    } else if !input.keyboard_navigation_stable {
        Some(M5ComboboxDegradeReason::KeyboardNavigationUnstable)
    } else if disposition_requires_distinct && !input.blocked_state_distinct {
        Some(M5ComboboxDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
    } else if input.command_id.trim().is_empty() {
        Some(M5ComboboxDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5ComboboxDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5ComboboxDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ControlNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedCombobox {
        combobox_id: input.combobox_id,
        label: input.label,
        selected_value: input.selected_value,
        selected_value_disclosed: input.selected_value_disclosed,
        value_source: input.value_source.as_str().to_owned(),
        value_source_resolved,
        value_source_is_unverified: unverified,
        support_class_tag: input.support_class_tag,
        support_class_tagged: input.support_class_tagged,
        requires_filter: input.requires_filter,
        filter_offered: input.filter_offered,
        value_provenance: input.value_provenance.as_str().to_owned(),
        value_provenance_resolved: input.value_provenance.is_resolved(),
        value_provenance_needs_disclosure: provenance_needs_disclosure,
        provenance_disclosed: input.provenance_disclosed,
        keyboard_navigation_stable: input.keyboard_navigation_stable,
        disposition: input.disposition.as_str().to_owned(),
        disposition_requires_distinct_treatment: disposition_requires_distinct,
        blocked_state_distinct: input.blocked_state_distinct,
        surface_context: input.surface_context.as_str().to_owned(),
        command_id: input.command_id,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        value_source_and_lock_honest_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a toggle control so its semantics and apply timing read honestly at a glance: the control
/// names its disclosed selected state, its semantics (checkbox / radio / switch / tri-state), its apply
/// timing (immediate versus deferred) so a switch is never blurred with a deferred checkbox, its
/// one-of-many versus multi-select arity, any radio exclusivity, its effective-value provenance with
/// disclosure when it materially changes trust, and its interaction disposition, keeps a locked /
/// read-only state distinct from generic disabled chrome, and always binds back to one canonical command.
pub fn resolve_toggle(
    input: M5ToggleResolutionInput,
) -> Result<M5ResolvedToggle, M5ControlResolutionError> {
    if input.toggle_id.trim().is_empty() {
        return Err(M5ControlResolutionError::EmptyToggleId);
    }
    if string_is_forbidden(&input.toggle_id)
        || string_is_forbidden(&input.label)
        || string_is_forbidden(&input.selected_state)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5ControlResolutionError::ForbiddenMaterial);
    }

    let disposition_requires_distinct = disposition_requires_distinct_treatment(input.disposition);
    let semantics_is_switch = toggle_semantics_is_switch(input.toggle_semantics);
    let semantics_is_exclusive = toggle_semantics_is_exclusive(input.toggle_semantics);
    let apply_timing_is_deferred = input.apply_timing.is_deferred();
    let provenance_needs_disclosure = input.value_provenance.needs_disclosure();

    let degrade_reason = if !input.selected_state_disclosed {
        Some(M5ToggleDegradeReason::SelectedStateUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ToggleDegradeReason::SurfaceContextUnresolved)
    } else if !toggle_semantics_is_resolved(input.toggle_semantics) {
        Some(M5ToggleDegradeReason::ToggleSemanticsUnresolved)
    } else if !input.apply_timing.is_resolved() {
        Some(M5ToggleDegradeReason::ApplyTimingUnresolved)
    } else if semantics_is_switch && apply_timing_is_deferred {
        Some(M5ToggleDegradeReason::SwitchBlurredWithDeferredCheckbox)
    } else if !input.selection_arity_explicit {
        Some(M5ToggleDegradeReason::OneOfManyVersusMultiSelectAmbiguous)
    } else if semantics_is_exclusive && !input.group_exclusivity_enforced {
        Some(M5ToggleDegradeReason::GroupExclusivityLost)
    } else if !input.value_provenance.is_resolved() {
        Some(M5ToggleDegradeReason::ValueProvenanceUnresolved)
    } else if provenance_needs_disclosure && !input.provenance_disclosed {
        Some(M5ToggleDegradeReason::ValueProvenanceUndisclosed)
    } else if disposition_requires_distinct && !input.blocked_state_distinct {
        Some(M5ToggleDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
    } else if input.command_id.trim().is_empty() {
        Some(M5ToggleDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5ToggleDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5ToggleDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ControlNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedToggle {
        toggle_id: input.toggle_id,
        label: input.label,
        selected_state: input.selected_state,
        selected_state_disclosed: input.selected_state_disclosed,
        toggle_semantics: input.toggle_semantics.as_str().to_owned(),
        toggle_semantics_resolved: toggle_semantics_is_resolved(input.toggle_semantics),
        semantics_is_switch,
        semantics_is_exclusive,
        apply_timing: input.apply_timing.as_str().to_owned(),
        apply_timing_resolved: input.apply_timing.is_resolved(),
        apply_timing_is_deferred,
        apply_timing_is_blocked: input.apply_timing.is_blocked(),
        selection_arity_explicit: input.selection_arity_explicit,
        group_exclusivity_enforced: input.group_exclusivity_enforced,
        value_provenance: input.value_provenance.as_str().to_owned(),
        value_provenance_resolved: input.value_provenance.is_resolved(),
        value_provenance_needs_disclosure: provenance_needs_disclosure,
        provenance_disclosed: input.provenance_disclosed,
        disposition: input.disposition.as_str().to_owned(),
        disposition_requires_distinct_treatment: disposition_requires_distinct,
        blocked_state_distinct: input.blocked_state_distinct,
        surface_context: input.surface_context.as_str().to_owned(),
        command_id: input.command_id,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        semantics_and_timing_honest_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved combobox and toggle examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComboboxToggleControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ControlConsumerSurface,
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
    pub anatomy_parts: Vec<M5ControlAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ControlExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    /// Resolved combobox examples.
    pub combobox_examples: Vec<M5ResolvedCombobox>,
    /// Resolved toggle-control examples.
    pub toggle_examples: Vec<M5ResolvedToggle>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: value-source / provenance truth is never dropped on a selection / toggle control.
    pub value_source_or_provenance_truth_dropped: bool,
    /// Hard invariant: a switch is never blurred with a deferred checkbox.
    pub switch_blurred_with_deferred_checkbox: bool,
    /// Hard invariant: one-of-many versus multi-select behavior is never blurred.
    pub one_of_many_versus_multi_select_blurred: bool,
    /// Hard invariant: locked / read-only semantics never hide behind generic disabled chrome.
    pub locked_or_read_only_semantics_hidden_behind_disabled: bool,
}

impl M5ComboboxToggleControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ControlAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5ControlAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ControlExportField> = self.export_fields.iter().copied().collect();
        M5ControlExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.value_source_or_provenance_truth_dropped
            && !self.switch_blurred_with_deferred_checkbox
            && !self.one_of_many_versus_multi_select_blurred
            && !self.locked_or_read_only_semantics_hidden_behind_disabled
    }

    /// True when a clean combobox discloses its selected value, resolves and (where unverified) tags its
    /// value source, offers a filter where it claims one, resolves and (where material) discloses its
    /// provenance, keeps keyboard navigation stable, and keeps a locked / read-only state distinct.
    fn combobox_is_honest(ex: &M5ResolvedCombobox) -> bool {
        !ex.is_clean()
            || (ex.selected_value_disclosed
                && ex.value_source_resolved
                && (!ex.requires_filter || ex.filter_offered)
                && (!ex.value_source_is_unverified || ex.support_class_tagged)
                && ex.value_provenance_resolved
                && (!ex.value_provenance_needs_disclosure || ex.provenance_disclosed)
                && ex.keyboard_navigation_stable
                && (!ex.disposition_requires_distinct_treatment || ex.blocked_state_distinct)
                && ex.command_route_available)
    }

    /// True when a clean toggle discloses its selected state, resolves its semantics and timing, keeps a
    /// switch immediate rather than deferred, keeps its arity explicit and any radio exclusivity enforced,
    /// resolves and (where material) discloses provenance, and keeps a locked / read-only state distinct.
    fn toggle_is_honest(ex: &M5ResolvedToggle) -> bool {
        !ex.is_clean()
            || (ex.selected_state_disclosed
                && ex.toggle_semantics_resolved
                && ex.apply_timing_resolved
                && (!ex.semantics_is_switch || !ex.apply_timing_is_deferred)
                && ex.selection_arity_explicit
                && (!ex.semantics_is_exclusive || ex.group_exclusivity_enforced)
                && ex.value_provenance_resolved
                && (!ex.value_provenance_needs_disclosure || ex.provenance_disclosed)
                && (!ex.disposition_requires_distinct_treatment || ex.blocked_state_distinct)
                && ex.command_route_available)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.combobox_examples.iter().all(Self::combobox_is_honest)
            && self.toggle_examples.iter().all(Self::toggle_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComboboxToggleVocabularySet {
    /// Interaction-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Combobox value-source tokens (bound from the frozen matrix).
    pub combobox_value_sources: Vec<String>,
    /// Toggle-semantics tokens (bound from the frozen matrix).
    pub toggle_semantics: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Value-provenance tokens (minted by this lane).
    pub value_provenances: Vec<String>,
    /// Apply-timing tokens (minted by this lane).
    pub apply_timings: Vec<String>,
    /// Combobox degrade-reason tokens.
    pub combobox_degrade_reasons: Vec<String>,
    /// Toggle degrade-reason tokens.
    pub toggle_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ComboboxToggleVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5CoreControlDisposition::ALL, |v| v.as_str()),
            combobox_value_sources: tokens(&M5ComboboxValueSource::ALL, |v| v.as_str()),
            toggle_semantics: tokens(&M5ToggleSemantics::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ControlSurfaceContext::ALL, |v| v.as_str()),
            value_provenances: tokens(&M5ControlValueProvenance::ALL, |v| v.as_str()),
            apply_timings: tokens(&M5ToggleApplyTiming::ALL, |v| v.as_str()),
            combobox_degrade_reasons: tokens(&M5ComboboxDegradeReason::ALL, |v| v.as_str()),
            toggle_degrade_reasons: tokens(&M5ToggleDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ControlAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ControlNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ControlExportField::ALL, |v| v.as_str()),
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
pub struct M5ComboboxToggleGovernanceReview {
    /// The combobox discloses its selected value and value source.
    pub combobox_discloses_selected_value_and_source: bool,
    /// The combobox keeps a filterable set filterable and stable under keyboard navigation.
    pub combobox_keeps_filterable_and_keyboard_stable: bool,
    /// The combobox never presents a remote / unverified value as canonical without a support-class tag.
    pub combobox_never_presents_unverified_as_canonical: bool,
    /// The toggle names explicit immediate-versus-deferred apply timing.
    pub toggle_names_immediate_versus_deferred_timing: bool,
    /// A switch is never blurred with a deferred checkbox.
    pub switch_never_blurred_with_deferred_checkbox: bool,
    /// One-of-many versus multi-select behavior stays unambiguous.
    pub one_of_many_versus_multi_select_unambiguous: bool,
    /// Policy / imported / detected / default / user-override provenance is carried, not feature-local.
    pub provenance_carried_not_feature_local: bool,
    /// A provenance that materially changes trust is always disclosed.
    pub material_provenance_always_disclosed: bool,
    /// Locked / read-only semantics are never hidden behind generic disabled chrome.
    pub locked_and_read_only_never_hidden_behind_disabled: bool,
    /// Both controls bind one canonical command with a command trace path.
    pub both_bind_canonical_command_with_trace: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComboboxToggleConsumerProjection {
    /// Settings surfaces consume the shared combobox and toggle vocabulary.
    pub settings_surfaces_consume_combobox_and_toggle_vocabulary: bool,
    /// Provider / admin surfaces consume the shared value-source vocabulary.
    pub provider_admin_surfaces_consume_value_source_vocabulary: bool,
    /// Request / entry surfaces consume the shared toggle-semantics vocabulary.
    pub request_entry_surfaces_consume_toggle_vocabulary: bool,
    /// Value source, lock state, and apply timing trace back to one canonical component contract.
    pub value_source_lock_and_timing_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical control source.
    pub support_export_reads_single_control_source: bool,
    /// Support / export can reconstruct the chosen selection / toggle state and editability.
    pub support_export_reconstructs_selection_and_editability: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComboboxToggleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the control.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComboboxToggleReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ComboboxToggleControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ComboboxToggleControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ComboboxToggleControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ComboboxToggleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ComboboxToggleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ComboboxToggleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ComboboxToggleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ComboboxToggleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 combobox / checkbox-radio-switch controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComboboxToggleControlsPacket {
    /// Record kind; must equal [`M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ComboboxToggleControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ComboboxToggleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ComboboxToggleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ComboboxToggleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ComboboxToggleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ComboboxToggleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ComboboxToggleControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ComboboxToggleControlsPacketInput) -> Self {
        Self {
            record_kind: M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ComboboxToggleControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_RECORD_KIND {
            violations.push(M5ComboboxToggleControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ComboboxToggleControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ComboboxToggleControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ComboboxToggleControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 combobox / toggle-control controls packet serializes"),
        ) {
            violations.push(M5ComboboxToggleControlsViolation::RawMaterialInExport);
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
            .expect("m5 combobox / toggle-control controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,combobox_examples,toggle_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .combobox_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.toggle_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.combobox_examples.len(),
                row.toggle_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Combobox and Checkbox-Radio-Switch Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Combobox value sources: {}\n",
            self.vocabulary_set.combobox_value_sources.join(", ")
        ));
        out.push_str(&format!(
            "- Toggle semantics: {}\n",
            self.vocabulary_set.toggle_semantics.join(", ")
        ));
        out.push_str(&format!(
            "- Apply timings: {}\n",
            self.vocabulary_set.apply_timings.join(", ")
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
                "  - Combobox examples: {} / toggle examples: {}\n",
                row.combobox_examples.len(),
                row.toggle_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ComboboxToggleControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ComboboxToggleControlsViolation>),
}

impl fmt::Display for M5ComboboxToggleControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 combobox / toggle-control controls export parse failed: {error}"
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
                    "m5 combobox / toggle-control controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ComboboxToggleControlsArtifactError {}

/// Validation failures emitted by [`M5ComboboxToggleControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ComboboxToggleControlsViolation {
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
    /// A controls row carries a dishonest clean example (undisclosed value, unresolved / undisclosed
    /// source, unverified-untagged value, blurred switch, ambiguous arity, hidden lock, or missing trace).
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
    /// Value source, lock state, and immediate/deferred timing are not proven without contradiction: clean
    /// controls do not cover the value-source and timing grammar, or no value-source-unresolved / switch-
    /// blur / provenance-undisclosed example degrades, or a clean control is contradictory.
    ValueSourceLockAndTimingNotProven,
    /// Accessibility and distinct-state truth is not proven: no clean combobox and toggle keep keyboard
    /// navigation stable with a distinct blocked state, or no locked-hidden / keyboard-unstable example
    /// degrades, or a clean control hides a locked / read-only state behind generic disabled chrome.
    AccessibilityAndDistinctStateNotProven,
    /// Selection-state and editability trace is not proven: no clean combobox and toggle reconstruct the
    /// chosen value / state with provenance and a command trace, or the provenance grammar does not cover
    /// a user-override and a disclosed non-user origin, or no command-trace-missing example degrades.
    SelectionStateAndEditabilityTraceNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ComboboxToggleControlsViolation {
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
            Self::ValueSourceLockAndTimingNotProven => "value_source_lock_and_timing_not_proven",
            Self::AccessibilityAndDistinctStateNotProven => {
                "accessibility_and_distinct_state_not_proven"
            }
            Self::SelectionStateAndEditabilityTraceNotProven => {
                "selection_state_and_editability_trace_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_combobox_checkbox_radio_switch_controls_export(
) -> Result<M5ComboboxToggleControlsPacket, M5ComboboxToggleControlsArtifactError> {
    let packet: M5ComboboxToggleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-combobox-checkbox-radio-switch-controls-proof/support_export.json"
    )))
    .map_err(M5ComboboxToggleControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ComboboxToggleControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_SCHEMA_REF,
        M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_COMBOBOX_SCHEMA_REF,
        M5_TOGGLE_CONTROL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ComboboxToggleControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ComboboxToggleControlsViolation::NoControlsRows);
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
            violations.push(M5ComboboxToggleControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ComboboxToggleControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ComboboxToggleControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_COMBOBOX_SCHEMA_REF) || !refs.contains(M5_TOGGLE_CONTROL_SCHEMA_REF) {
            violations.push(M5ComboboxToggleControlsViolation::ComponentSchemaRefMissing);
        }
        if row.combobox_examples.is_empty() || row.toggle_examples.is_empty() {
            violations.push(M5ComboboxToggleControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ComboboxToggleControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ComboboxToggleControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.combobox_discloses_selected_value_and_source,
        review.combobox_keeps_filterable_and_keyboard_stable,
        review.combobox_never_presents_unverified_as_canonical,
        review.toggle_names_immediate_versus_deferred_timing,
        review.switch_never_blurred_with_deferred_checkbox,
        review.one_of_many_versus_multi_select_unambiguous,
        review.provenance_carried_not_feature_local,
        review.material_provenance_always_disclosed,
        review.locked_and_read_only_never_hidden_behind_disabled,
        review.both_bind_canonical_command_with_trace,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ComboboxToggleControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.settings_surfaces_consume_combobox_and_toggle_vocabulary,
        projection.provider_admin_surfaces_consume_value_source_vocabulary,
        projection.request_entry_surfaces_consume_toggle_vocabulary,
        projection.value_source_lock_and_timing_trace_to_single_component_contract,
        projection.support_export_reads_single_control_source,
        projection.support_export_reconstructs_selection_and_editability,
    ] {
        if !ok {
            violations.push(M5ComboboxToggleControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ComboboxToggleControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ComboboxToggleControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ComboboxToggleControlsPacket,
    violations: &mut Vec<M5ComboboxToggleControlsViolation>,
) {
    let comboboxes = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.combobox_examples.iter())
    };
    let toggles = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.toggle_examples.iter())
    };

    // AC1: the first claimed combobox and boolean-control consumers express value source, lock state, and
    // immediate/deferred semantics without contradiction. Clean comboboxes cover the canonical and filtered
    // value sources, clean toggles cover immediate and deferred timing, a value-source-unresolved example
    // degrades, a switch-blur example degrades, a provenance-undisclosed example degrades, and no clean
    // control contradicts itself (no clean unverified-untagged combobox, no clean switch+deferred toggle).
    let clean_value_sources: BTreeSet<String> = comboboxes()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.value_source.clone())
        .collect();
    let value_source_grammar_covered = ["canonical_option", "filtered_subset"]
        .iter()
        .all(|s| clean_value_sources.contains(*s));
    let clean_timings: BTreeSet<String> = toggles()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.apply_timing.clone())
        .collect();
    let timing_grammar_covered = clean_timings.contains("applies_immediately")
        && [
            "deferred_until_save",
            "staged_in_review",
            "requires_confirmation",
        ]
        .iter()
        .any(|t| clean_timings.contains(*t));
    let value_source_unresolved_degrades = comboboxes()
        .any(|ex| ex.degrade_reason == Some(M5ComboboxDegradeReason::ValueSourceUnresolved));
    let switch_blur_degrades = toggles().any(|ex| {
        ex.degrade_reason == Some(M5ToggleDegradeReason::SwitchBlurredWithDeferredCheckbox)
    });
    let provenance_undisclosed_degrades = comboboxes()
        .any(|ex| ex.degrade_reason == Some(M5ComboboxDegradeReason::ValueProvenanceUndisclosed))
        || toggles()
            .any(|ex| ex.degrade_reason == Some(M5ToggleDegradeReason::ValueProvenanceUndisclosed));
    let no_clean_contradiction = comboboxes()
        .all(|ex| !(ex.is_clean() && ex.value_source_is_unverified && !ex.support_class_tagged))
        && toggles()
            .all(|ex| !(ex.is_clean() && ex.semantics_is_switch && ex.apply_timing_is_deferred));
    if !(value_source_grammar_covered
        && timing_grammar_covered
        && value_source_unresolved_degrades
        && switch_blur_degrades
        && provenance_undisclosed_degrades
        && no_clean_contradiction)
    {
        violations.push(M5ComboboxToggleControlsViolation::ValueSourceLockAndTimingNotProven);
    }

    // AC2: keyboard / screen-reader / high-zoom / reduced-motion fixtures preserve toggle/selection truth
    // instead of flattening it into generic disabled styling. At least one clean combobox keeps keyboard
    // navigation stable with a distinct blocked state, at least one clean toggle keeps a distinct blocked
    // state, a locked-hidden example degrades, a keyboard-unstable example degrades, and no clean control
    // hides a locked / read-only state behind generic disabled chrome.
    let clean_combobox_accessible = comboboxes()
        .any(|ex| ex.is_clean() && ex.keyboard_navigation_stable && ex.blocked_state_distinct);
    let clean_toggle_accessible = toggles().any(|ex| ex.is_clean() && ex.blocked_state_distinct);
    let locked_hidden_degrades = comboboxes().any(|ex| {
        ex.degrade_reason == Some(M5ComboboxDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
    }) || toggles().any(|ex| {
        ex.degrade_reason == Some(M5ToggleDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
    });
    let keyboard_unstable_degrades = comboboxes()
        .any(|ex| ex.degrade_reason == Some(M5ComboboxDegradeReason::KeyboardNavigationUnstable));
    let no_clean_hides_lock = comboboxes().all(|ex| {
        !(ex.is_clean() && ex.disposition_requires_distinct_treatment && !ex.blocked_state_distinct)
    }) && toggles().all(|ex| {
        !(ex.is_clean() && ex.disposition_requires_distinct_treatment && !ex.blocked_state_distinct)
    });
    if !(clean_combobox_accessible
        && clean_toggle_accessible
        && locked_hidden_degrades
        && keyboard_unstable_degrades
        && no_clean_hides_lock)
    {
        violations.push(M5ComboboxToggleControlsViolation::AccessibilityAndDistinctStateNotProven);
    }

    // AC3: support/help/export packets can reconstruct the chosen selection/toggle state and why it was or
    // was not editable. At least one clean combobox reconstructs its selected value with resolved
    // provenance and a command trace, at least one clean toggle reconstructs its selected state with
    // resolved timing and a command trace, the provenance grammar covers a user-override and a disclosed
    // non-user origin, and a command-trace-missing example degrades.
    let traceable_combobox = comboboxes().any(|ex| {
        ex.is_clean()
            && ex.selected_value_disclosed
            && ex.value_provenance_resolved
            && ex.command_route_available
    });
    let traceable_toggle = toggles().any(|ex| {
        ex.is_clean()
            && ex.selected_state_disclosed
            && ex.apply_timing_resolved
            && ex.command_route_available
    });
    let clean_provenances: BTreeSet<String> = comboboxes()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.value_provenance.clone())
        .chain(
            toggles()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.value_provenance.clone()),
        )
        .collect();
    let provenance_grammar_covered = clean_provenances.contains("user_override")
        && ["policy_enforced", "imported", "detected"]
            .iter()
            .any(|p| clean_provenances.contains(*p));
    let trace_missing_degrades = comboboxes()
        .any(|ex| ex.degrade_reason == Some(M5ComboboxDegradeReason::CommandTracePathMissing))
        || toggles()
            .any(|ex| ex.degrade_reason == Some(M5ToggleDegradeReason::CommandTracePathMissing));
    if !(traceable_combobox
        && traceable_toggle
        && provenance_grammar_covered
        && trace_missing_degrades)
    {
        violations
            .push(M5ComboboxToggleControlsViolation::SelectionStateAndEditabilityTraceNotProven);
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
    M5CoreControlFamily::Combobox,
    M5CoreControlFamily::ToggleControl,
];
