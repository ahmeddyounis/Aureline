//! Implemented M5 text-field and search-field primitives.
//!
//! The frozen [core action / input component matrix][matrix] names Aureline's most reused atomic
//! action and input controls and locks their controlled vocabulary. This module is the text-entry and
//! search-entry implement lane over that matrix: it turns the **text field** and the **search field**
//! into resolvers that produce export-safe, honest projections, so a user never has to infer meaning
//! from placeholder text, never has to guess whether a search input was submitted or retained, and
//! never loses blocked / validation / privacy truth when the same field is reused across a different M5
//! lane.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement permanent labels, hint text, validation / status messaging, clear controls, reveal
//!   controls where relevant, and focus-visible treatment for reusable text / search fields.**
//!   [`resolve_text_field`] refuses to read as a clean, labeled, validation-honest field when the label
//!   is placeholder-only or empty, the surface context or label mode is unresolved, focus-visible
//!   treatment is missing, the validation state is unresolved, a flagging validation carries only vague
//!   copy, a sensitive value is missing its reveal control, a locked / read-only / degraded state hides
//!   behind generic disabled chrome, draft continuity is lost, a validation anchor is lost, or the
//!   canonical command binding / trace path is missing; it degrades instead.
//! * **Support search-icon, clear, submit, scope, cached / live, and blocked / privacy cues on search
//!   fields wherever retention, provider scope, or export behavior materially changes user
//!   expectations.** [`resolve_search_field`] degrades when the label is placeholder-only, the surface
//!   context or label mode is unresolved, the search icon or clear affordance is missing, the submit
//!   model is unresolved, validation copy is vague, the retention posture is unresolved, a retention /
//!   privacy cue that materially changes expectations goes undisclosed, a blocked state hides behind
//!   generic disabled chrome, draft continuity is lost, or the canonical command binding / trace path is
//!   missing.
//! * **Preserve draft / restore continuity and exact validation anchors when text / search fields
//!   survive interruption, retry, import, or reconnect flows.** Both resolvers degrade with a
//!   draft-continuity reason whenever the draft is not preserved across the first interruption, the text
//!   resolver additionally degrades when an exact validation anchor is lost, and both always bind back to
//!   one canonical command.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5CoreControlDisposition`] interaction-state vocabulary, the [`M5FieldLabelMode`] label-mode
//! vocabulary, and the [`M5FieldValidationState`] validation vocabulary — so forms, settings, search,
//! entry, support, and product surfaces can never fork their own labeling, validation, or privacy
//! wording. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_core_action_input_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_text_field_search_field_controls,
    seeded_m5_text_field_search_field_controls_search_ui_preview_narrowed,
    seeded_m5_text_field_search_field_controls_settings_ui_beta_narrowed,
    M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_core_action_input_component_matrix::{
    M5CoreControlAccessibilityRoute, M5CoreControlConsumerSurface, M5CoreControlDeploymentLine,
    M5CoreControlDisposition, M5CoreControlDowngradeTrigger, M5CoreControlFamily,
    M5CoreControlQualificationClass, M5CoreControlRequiredLabel, M5FieldLabelMode,
    M5FieldValidationState, M5_CORE_CONTROL_COMPONENT_DOC_REF,
    M5_CORE_CONTROL_COMPONENT_SCHEMA_REF, M5_SEARCH_FIELD_SCHEMA_REF, M5_TEXT_FIELD_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5TextFieldSearchFieldControlsPacket`].
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_RECORD_KIND: &str =
    "implement_m5_text_field_and_search_field_controls";

/// Schema version for M5 text-field / search-field controls records.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-text-field-search-field-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_DOC_REF: &str =
    "docs/components/m5_text_field_and_search_field_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-text-field-search-field-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-text-field-search-field-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-text-field-search-field-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-text-field-search-field-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5FieldConsumerSurface = M5CoreControlConsumerSurface;

/// Controlled render context — which claimed M5 surface renders the text or search field, so a field's
/// meaning stays stable whether it appears in a forms sheet, a settings row, a search bar, a start-center
/// entry field, or a support flow. Minted by this lane, tracking the exit-gate anchor surfaces directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FieldSurfaceContext {
    /// A forms sheet.
    FormsSheet,
    /// A settings row.
    SettingsRow,
    /// A search bar.
    SearchBar,
    /// The start-center entry field.
    EntryField,
    /// A support / recovery flow.
    SupportFlow,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5FieldSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FormsSheet,
        Self::SettingsRow,
        Self::SearchBar,
        Self::EntryField,
        Self::SupportFlow,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormsSheet => "forms_sheet",
            Self::SettingsRow => "settings_row",
            Self::SearchBar => "search_bar",
            Self::EntryField => "entry_field",
            Self::SupportFlow => "support_flow",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled search submit model — how a search field commits a query, so a user never has to guess
/// whether input was submitted, and a blocked submission is never mistaken for a live one. Minted by this
/// lane because the frozen matrix carries a single-select affordance token but not the submit model the
/// search-field acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SearchSubmitModel {
    /// Submits on an explicit action (Enter / button).
    SubmitExplicit,
    /// Submits as-you-type.
    SubmitAsYouType,
    /// Submits as-you-type but debounced.
    SubmitDebounced,
    /// Submits within a named scope.
    SubmitScoped,
    /// Submission is blocked (policy / privacy), shown distinctly.
    SubmitBlocked,
    /// The submit model cannot currently be resolved.
    SubmitUnknown,
}

impl M5SearchSubmitModel {
    /// Every submit model, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SubmitExplicit,
        Self::SubmitAsYouType,
        Self::SubmitDebounced,
        Self::SubmitScoped,
        Self::SubmitBlocked,
        Self::SubmitUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmitExplicit => "submit_explicit",
            Self::SubmitAsYouType => "submit_as_you_type",
            Self::SubmitDebounced => "submit_debounced",
            Self::SubmitScoped => "submit_scoped",
            Self::SubmitBlocked => "submit_blocked",
            Self::SubmitUnknown => "submit_unknown",
        }
    }

    /// Whether the submit model is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::SubmitUnknown)
    }

    /// Whether submission is blocked and must be shown distinctly, never behind generic disabled chrome.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::SubmitBlocked)
    }
}

/// Controlled search retention posture — how a search field retains or shares a query and its results, so
/// query retention, provider scope, and export sensitivity stay honest cues rather than silent behavior.
/// Minted by this lane and consumed only by the search resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SearchRetentionPosture {
    /// The query is live and not retained.
    LiveNotRetained,
    /// The query history is kept private (not persisted / not shared).
    HistoryPrivate,
    /// Results are served from a disclosed cache.
    CachedResultsDisclosed,
    /// The query is backed by a remote provider whose scope must be disclosed.
    ProviderBackedRemote,
    /// The query touches export-sensitive material whose handling must be disclosed.
    ExportSensitive,
    /// The retention posture cannot currently be resolved.
    RetentionUnknown,
}

impl M5SearchRetentionPosture {
    /// Every retention posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveNotRetained,
        Self::HistoryPrivate,
        Self::CachedResultsDisclosed,
        Self::ProviderBackedRemote,
        Self::ExportSensitive,
        Self::RetentionUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveNotRetained => "live_not_retained",
            Self::HistoryPrivate => "history_private",
            Self::CachedResultsDisclosed => "cached_results_disclosed",
            Self::ProviderBackedRemote => "provider_backed_remote",
            Self::ExportSensitive => "export_sensitive",
            Self::RetentionUnknown => "retention_unknown",
        }
    }

    /// Whether this posture materially changes user expectations and must therefore be disclosed (also
    /// true for the unresolved sentinel, which can never be presented as a plain live search).
    pub const fn needs_disclosure(self) -> bool {
        matches!(
            self,
            Self::CachedResultsDisclosed
                | Self::ProviderBackedRemote
                | Self::ExportSensitive
                | Self::RetentionUnknown
        )
    }

    /// Whether the retention posture is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::RetentionUnknown)
    }
}

/// One mandatory rendered part a text or search field must be able to show, so no label, hint,
/// validation, clear / reveal affordance, scope, retention, or command fact is left implicit behind a
/// placeholder, a tooltip, or a secondary panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FieldAnatomyPart {
    /// The field's stable identity / permanent label.
    Identity,
    /// The field's current typed interaction disposition.
    State,
    /// The non-visual keyboard route to the field.
    KeyboardRoute,
    /// The permanent label text (never placeholder-only).
    LabelText,
    /// The hint / helper text.
    HintText,
    /// The validation / status message.
    ValidationMessage,
    /// The clear affordance (search field).
    ClearAffordance,
    /// The reveal affordance (sensitive text field).
    RevealAffordance,
    /// The search scope cue (search field).
    ScopeCue,
    /// The retention / privacy cue (search field).
    RetentionCue,
    /// The render / surface context.
    SurfaceContext,
    /// The canonical command binding.
    CommandBinding,
}

impl M5FieldAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::LabelText,
        Self::HintText,
        Self::ValidationMessage,
        Self::ClearAffordance,
        Self::RevealAffordance,
        Self::ScopeCue,
        Self::RetentionCue,
        Self::SurfaceContext,
        Self::CommandBinding,
    ];

    /// The three parts every claimed field must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::LabelText => "label_text",
            Self::HintText => "hint_text",
            Self::ValidationMessage => "validation_message",
            Self::ClearAffordance => "clear_affordance",
            Self::RevealAffordance => "reveal_affordance",
            Self::ScopeCue => "scope_cue",
            Self::RetentionCue => "retention_cue",
            Self::SurfaceContext => "surface_context",
            Self::CommandBinding => "command_binding",
        }
    }
}

/// Next safe action a field surfaces so a user is never left without a route to inspect the label,
/// validation, privacy, or command behind a degraded field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FieldNextAction {
    /// Open the command-backed action / command detail.
    OpenCommandDetail,
    /// Inspect the field label / affordances.
    InspectFieldLabel,
    /// Inspect the validation / status message.
    InspectValidation,
    /// Review a locked / read-only / blocked field.
    ReviewBlockedOrLocked,
    /// Review the retention / privacy cue.
    ReviewPrivacyOrRetention,
    /// No action is needed; the field is clean.
    NoActionNeeded,
}

impl M5FieldNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenCommandDetail,
        Self::InspectFieldLabel,
        Self::InspectValidation,
        Self::ReviewBlockedOrLocked,
        Self::ReviewPrivacyOrRetention,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCommandDetail => "open_command_detail",
            Self::InspectFieldLabel => "inspect_field_label",
            Self::InspectValidation => "inspect_validation",
            Self::ReviewBlockedOrLocked => "review_blocked_or_locked",
            Self::ReviewPrivacyOrRetention => "review_privacy_or_retention",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FieldExportField {
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
    /// The field label mode named by the fields.
    LabelMode,
    /// The validation state named by the fields.
    ValidationState,
    /// The retention posture named by the search field.
    RetentionPosture,
    /// The submit model named by the search field.
    SubmitModel,
    /// The render / surface context named by both fields.
    SurfaceContext,
    /// The accountable owner role.
    OwnerRole,
}

impl M5FieldExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::LabelMode,
        Self::ValidationState,
        Self::RetentionPosture,
        Self::SubmitModel,
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
            Self::LabelMode => "label_mode",
            Self::ValidationState => "validation_state",
            Self::RetentionPosture => "retention_posture",
            Self::SubmitModel => "submit_model",
            Self::SurfaceContext => "surface_context",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a text field degraded below a clean, labeled, validation-honest state. The degrade-first
/// ladder returns one of these instead of ever letting a placeholder-only or vague field read as a clean
/// pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextFieldDegradeReason {
    /// The label is placeholder-only or empty; a user must infer meaning from placeholder text.
    LabelIsPlaceholderOnly,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The label mode cannot currently be resolved.
    LabelModeUnresolved,
    /// Focus-visible treatment is missing; the field cannot show keyboard focus.
    FocusVisibleTreatmentMissing,
    /// The validation state cannot currently be resolved.
    ValidationStateUnresolved,
    /// A flagging validation carries only vague copy rather than a specific message.
    VagueValidationCopy,
    /// A sensitive value is missing its reveal control.
    RevealAffordanceMissing,
    /// A locked / read-only / degraded state hides behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// Draft state was not preserved across the first interruption / recovery.
    DraftContinuityLost,
    /// An exact validation anchor was lost across interruption, retry, import, or reconnect.
    ValidationAnchorLost,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the field is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TextFieldDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::LabelIsPlaceholderOnly,
        Self::SurfaceContextUnresolved,
        Self::LabelModeUnresolved,
        Self::FocusVisibleTreatmentMissing,
        Self::ValidationStateUnresolved,
        Self::VagueValidationCopy,
        Self::RevealAffordanceMissing,
        Self::LockedOrDegradedHiddenBehindDisabled,
        Self::DraftContinuityLost,
        Self::ValidationAnchorLost,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabelIsPlaceholderOnly => "label_is_placeholder_only",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LabelModeUnresolved => "label_mode_unresolved",
            Self::FocusVisibleTreatmentMissing => "focus_visible_treatment_missing",
            Self::ValidationStateUnresolved => "validation_state_unresolved",
            Self::VagueValidationCopy => "vague_validation_copy",
            Self::RevealAffordanceMissing => "reveal_affordance_missing",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
            }
            Self::DraftContinuityLost => "draft_continuity_lost",
            Self::ValidationAnchorLost => "validation_anchor_lost",
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5FieldNextAction {
        match self {
            Self::LabelIsPlaceholderOnly
            | Self::SurfaceContextUnresolved
            | Self::LabelModeUnresolved
            | Self::FocusVisibleTreatmentMissing => M5FieldNextAction::InspectFieldLabel,
            Self::ValidationStateUnresolved
            | Self::VagueValidationCopy
            | Self::DraftContinuityLost
            | Self::ValidationAnchorLost => M5FieldNextAction::InspectValidation,
            Self::RevealAffordanceMissing => M5FieldNextAction::ReviewPrivacyOrRetention,
            Self::LockedOrDegradedHiddenBehindDisabled => M5FieldNextAction::ReviewBlockedOrLocked,
            Self::CommandBindingUnstated | Self::CommandTracePathMissing | Self::ProofStale => {
                M5FieldNextAction::OpenCommandDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::LabelIsPlaceholderOnly => M5CoreControlDowngradeTrigger::PlaceholderUsedAsLabel,
            Self::ValidationStateUnresolved
            | Self::VagueValidationCopy
            | Self::ValidationAnchorLost => M5CoreControlDowngradeTrigger::ValidationStateUnstated,
            Self::LockedOrDegradedHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SurfaceContextUnresolved
            | Self::LabelModeUnresolved
            | Self::FocusVisibleTreatmentMissing
            | Self::RevealAffordanceMissing
            | Self::DraftContinuityLost => M5CoreControlDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason a search field degraded below a clean state that keeps clear / submit / privacy / blocked truth
/// explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SearchFieldDegradeReason {
    /// The label is placeholder-only or empty; a user must infer meaning from placeholder text.
    LabelIsPlaceholderOnly,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The label mode cannot currently be resolved.
    LabelModeUnresolved,
    /// The search icon cue is missing.
    SearchIconMissing,
    /// The clear affordance is missing; a user cannot tell how to clear the query.
    ClearAffordanceMissing,
    /// The submit model cannot currently be resolved; a user cannot tell whether input was submitted.
    SubmitModelUnresolved,
    /// A flagging validation carries only vague copy rather than a specific message.
    VagueValidationCopy,
    /// The retention posture cannot currently be resolved.
    RetentionUnresolved,
    /// A retention / privacy cue that materially changes expectations is left undisclosed.
    PrivacyCueMissing,
    /// A blocked submission / state hides behind generic disabled chrome.
    BlockedStateHiddenBehindDisabled,
    /// Draft state was not preserved across the first interruption / recovery.
    DraftContinuityLost,
    /// The canonical command binding is unstated.
    CommandBindingUnstated,
    /// No command-backed path to inspect the field is reachable.
    CommandTracePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SearchFieldDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::LabelIsPlaceholderOnly,
        Self::SurfaceContextUnresolved,
        Self::LabelModeUnresolved,
        Self::SearchIconMissing,
        Self::ClearAffordanceMissing,
        Self::SubmitModelUnresolved,
        Self::VagueValidationCopy,
        Self::RetentionUnresolved,
        Self::PrivacyCueMissing,
        Self::BlockedStateHiddenBehindDisabled,
        Self::DraftContinuityLost,
        Self::CommandBindingUnstated,
        Self::CommandTracePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabelIsPlaceholderOnly => "label_is_placeholder_only",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LabelModeUnresolved => "label_mode_unresolved",
            Self::SearchIconMissing => "search_icon_missing",
            Self::ClearAffordanceMissing => "clear_affordance_missing",
            Self::SubmitModelUnresolved => "submit_model_unresolved",
            Self::VagueValidationCopy => "vague_validation_copy",
            Self::RetentionUnresolved => "retention_unresolved",
            Self::PrivacyCueMissing => "privacy_cue_missing",
            Self::BlockedStateHiddenBehindDisabled => "blocked_state_hidden_behind_disabled",
            Self::DraftContinuityLost => "draft_continuity_lost",
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::CommandTracePathMissing => "command_trace_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5FieldNextAction {
        match self {
            Self::LabelIsPlaceholderOnly
            | Self::SurfaceContextUnresolved
            | Self::LabelModeUnresolved
            | Self::SearchIconMissing
            | Self::ClearAffordanceMissing
            | Self::SubmitModelUnresolved => M5FieldNextAction::InspectFieldLabel,
            Self::VagueValidationCopy | Self::DraftContinuityLost => {
                M5FieldNextAction::InspectValidation
            }
            Self::RetentionUnresolved | Self::PrivacyCueMissing => {
                M5FieldNextAction::ReviewPrivacyOrRetention
            }
            Self::BlockedStateHiddenBehindDisabled => M5FieldNextAction::ReviewBlockedOrLocked,
            Self::CommandBindingUnstated | Self::CommandTracePathMissing | Self::ProofStale => {
                M5FieldNextAction::OpenCommandDetail
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            Self::LabelIsPlaceholderOnly => M5CoreControlDowngradeTrigger::PlaceholderUsedAsLabel,
            Self::VagueValidationCopy => M5CoreControlDowngradeTrigger::ValidationStateUnstated,
            Self::RetentionUnresolved | Self::PrivacyCueMissing => {
                M5CoreControlDowngradeTrigger::ValueSourceUnstated
            }
            Self::BlockedStateHiddenBehindDisabled => {
                M5CoreControlDowngradeTrigger::LockedOrDegradedHiddenBehindDisabled
            }
            Self::CommandBindingUnstated | Self::CommandTracePathMissing => {
                M5CoreControlDowngradeTrigger::CommandBindingUnstated
            }
            Self::ProofStale => M5CoreControlDowngradeTrigger::ProofStale,
            Self::SurfaceContextUnresolved
            | Self::LabelModeUnresolved
            | Self::SearchIconMissing
            | Self::ClearAffordanceMissing
            | Self::SubmitModelUnresolved
            | Self::DraftContinuityLost => M5CoreControlDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// True when a disposition names a locked / read-only / degraded state that must never hide behind
/// generic disabled chrome. Read-only is included because for a field a read-only value must stay
/// distinct from a disabled one.
fn disposition_requires_distinct_treatment(disposition: M5CoreControlDisposition) -> bool {
    matches!(
        disposition,
        M5CoreControlDisposition::Locked
            | M5CoreControlDisposition::ReadOnly
            | M5CoreControlDisposition::Degraded
    )
}

/// True when a label mode is placeholder-only, which is disallowed as the only label.
fn label_mode_is_placeholder_only(mode: M5FieldLabelMode) -> bool {
    matches!(mode, M5FieldLabelMode::PlaceholderOnlyDisallowed)
}

/// True when a label mode is resolved (not the unresolved sentinel).
fn label_mode_is_resolved(mode: M5FieldLabelMode) -> bool {
    !matches!(mode, M5FieldLabelMode::LabelUnresolved)
}

/// True when a validation state is actively flagging (invalid or warning) and therefore needs a specific
/// message rather than vague copy.
fn validation_state_is_flagging(validation: M5FieldValidationState) -> bool {
    matches!(
        validation,
        M5FieldValidationState::InvalidBlocking | M5FieldValidationState::WarningNonblocking
    )
}

/// True when a validation state is resolved (not the unknown sentinel).
fn validation_state_is_resolved(validation: M5FieldValidationState) -> bool {
    !matches!(validation, M5FieldValidationState::ValidationUnknown)
}

/// Input to [`resolve_text_field`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TextFieldResolutionInput {
    /// Stable identity of the text-field instance.
    pub text_field_id: String,
    /// The permanent label shown; empty means placeholder-only.
    pub label: String,
    /// The label mode (from the frozen matrix vocabulary).
    pub label_mode: M5FieldLabelMode,
    /// The validation state (from the frozen matrix vocabulary).
    pub validation: M5FieldValidationState,
    /// True when a flagging validation carries a specific message rather than vague copy.
    pub validation_message_specific: bool,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// The render / surface context.
    pub surface_context: M5FieldSurfaceContext,
    /// True when focus-visible treatment is offered.
    pub focus_visible_offered: bool,
    /// True when this field holds a sensitive value that requires a reveal control.
    pub requires_reveal: bool,
    /// True when a reveal control is offered.
    pub reveal_offered: bool,
    /// True when a locked / read-only / degraded state is shown distinctly, never behind disabled chrome.
    pub blocked_state_distinct: bool,
    /// True when draft state is preserved across the first interruption / recovery.
    pub draft_preserved_across_interruption: bool,
    /// True when an exact validation anchor is preserved across interruption / retry / import / reconnect.
    pub validation_anchor_preserved: bool,
    /// The canonical command ID this field binds back to; empty means unstated.
    pub command_id: String,
    /// True when a command-backed path to inspect the field is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe text-field projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTextField {
    /// Stable identity of the text-field instance.
    pub text_field_id: String,
    /// The permanent label named by the text field.
    pub label: String,
    /// The label-mode token named by the text field.
    pub label_mode: String,
    /// Whether the label is permanent (a non-empty label with a non-placeholder-only mode).
    pub label_is_permanent: bool,
    /// The validation-state token named by the text field.
    pub validation: String,
    /// Whether the validation is actively flagging (invalid or warning).
    pub validation_is_flagging: bool,
    /// Whether a flagging validation carries a specific message.
    pub validation_message_specific: bool,
    /// The interaction-disposition token named by the text field.
    pub disposition: String,
    /// Whether the disposition needs distinct blocked treatment (locked / read-only / degraded).
    pub disposition_requires_distinct_treatment: bool,
    /// The render / surface-context token named by the text field.
    pub surface_context: String,
    /// Whether focus-visible treatment is offered.
    pub focus_visible_offered: bool,
    /// Whether this field holds a sensitive value that requires a reveal control.
    pub requires_reveal: bool,
    /// Whether a reveal control is offered.
    pub reveal_offered: bool,
    /// Whether a locked / read-only / degraded state is shown distinctly, never behind disabled chrome.
    pub blocked_state_distinct: bool,
    /// Whether draft state is preserved across the first interruption / recovery.
    pub draft_preserved_across_interruption: bool,
    /// Whether an exact validation anchor is preserved across interruption / retry / import / reconnect.
    pub validation_anchor_preserved: bool,
    /// The canonical command ID named by the text field.
    pub command_id: String,
    /// Whether a command-backed path to inspect the field is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the text field could not read as a clean, labeled, validation-honest state.
    pub degrade_reason: Option<M5TextFieldDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5FieldNextAction,
    /// Whether the label and validation read honestly at a glance (clean field naming every fact).
    pub label_and_validation_honest_at_a_glance: bool,
}

impl M5ResolvedTextField {
    /// Whether this text field reads as a clean, labeled, validation-honest state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_search_field`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SearchFieldResolutionInput {
    /// Stable identity of the search-field instance.
    pub search_field_id: String,
    /// The permanent label shown; empty means placeholder-only.
    pub label: String,
    /// The label mode (from the frozen matrix vocabulary).
    pub label_mode: M5FieldLabelMode,
    /// The validation state (from the frozen matrix vocabulary).
    pub validation: M5FieldValidationState,
    /// True when a flagging validation carries a specific message rather than vague copy.
    pub validation_message_specific: bool,
    /// The current interaction disposition (from the frozen matrix vocabulary).
    pub disposition: M5CoreControlDisposition,
    /// The render / surface context.
    pub surface_context: M5FieldSurfaceContext,
    /// True when a search-icon cue is offered.
    pub offers_search_icon: bool,
    /// True when a clear affordance is offered.
    pub offers_clear: bool,
    /// The submit model.
    pub submit_model: M5SearchSubmitModel,
    /// The named search scope shown; empty means unscoped.
    pub scope_label: String,
    /// The retention / privacy posture.
    pub retention_posture: M5SearchRetentionPosture,
    /// True when a retention / privacy cue that materially changes expectations is disclosed.
    pub privacy_disclosed: bool,
    /// True when a blocked submission / state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// True when draft state is preserved across the first interruption / recovery.
    pub draft_preserved_across_interruption: bool,
    /// The canonical command ID this field binds back to; empty means unstated.
    pub command_id: String,
    /// True when a command-backed path to inspect the field is reachable, never chrome-only.
    pub command_route_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe search-field projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSearchField {
    /// Stable identity of the search-field instance.
    pub search_field_id: String,
    /// The permanent label named by the search field.
    pub label: String,
    /// The label-mode token named by the search field.
    pub label_mode: String,
    /// Whether the label is permanent (a non-empty label with a non-placeholder-only mode).
    pub label_is_permanent: bool,
    /// The validation-state token named by the search field.
    pub validation: String,
    /// Whether the validation is actively flagging (invalid or warning).
    pub validation_is_flagging: bool,
    /// Whether a flagging validation carries a specific message.
    pub validation_message_specific: bool,
    /// The interaction-disposition token named by the search field.
    pub disposition: String,
    /// Whether the disposition needs distinct blocked treatment (locked / read-only / degraded).
    pub disposition_requires_distinct_treatment: bool,
    /// The render / surface-context token named by the search field.
    pub surface_context: String,
    /// Whether a search-icon cue is offered.
    pub offers_search_icon: bool,
    /// Whether a clear affordance is offered.
    pub offers_clear: bool,
    /// The submit-model token named by the search field.
    pub submit_model: String,
    /// Whether the submit model is resolved (not the unknown sentinel).
    pub submit_model_resolved: bool,
    /// Whether submission is blocked.
    pub submit_is_blocked: bool,
    /// The named search scope; empty means unscoped.
    pub scope_label: String,
    /// The retention-posture token named by the search field.
    pub retention_posture: String,
    /// Whether the retention posture materially changes expectations and must be disclosed.
    pub retention_needs_disclosure: bool,
    /// Whether a retention / privacy cue is disclosed.
    pub privacy_disclosed: bool,
    /// Whether a blocked submission / state is shown distinctly, never behind generic disabled chrome.
    pub blocked_state_distinct: bool,
    /// Whether draft state is preserved across the first interruption / recovery.
    pub draft_preserved_across_interruption: bool,
    /// The canonical command ID named by the search field.
    pub command_id: String,
    /// Whether a command-backed path to inspect the field is reachable.
    pub command_route_available: bool,
    /// Degrade reason, if the search field could not read as a clean clear / submit / privacy state.
    pub degrade_reason: Option<M5SearchFieldDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5FieldNextAction,
    /// Whether the clear / submit / privacy truth reads honestly at a glance (clean field naming facts).
    pub clear_submit_privacy_honest_at_a_glance: bool,
}

impl M5ResolvedSearchField {
    /// Whether this search field reads as a clean clear / submit / privacy state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5FieldResolutionError {
    /// The text-field id was empty.
    EmptyTextFieldId,
    /// The search-field id was empty.
    EmptySearchFieldId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5FieldResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyTextFieldId => "empty_text_field_id",
            Self::EmptySearchFieldId => "empty_search_field_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5FieldResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 text-field / search-field resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5FieldResolutionError {}

/// Resolves a text field so its label and validation read honestly at a glance: the field names its
/// permanent label (never placeholder-only), its label mode, its validation state (with specific copy
/// when flagging), and its interaction disposition, offers focus-visible treatment and any needed reveal
/// control, keeps a locked / read-only / degraded state distinct from generic disabled chrome, preserves
/// draft continuity and exact validation anchors across interruption, and always binds back to one
/// canonical command.
pub fn resolve_text_field(
    input: M5TextFieldResolutionInput,
) -> Result<M5ResolvedTextField, M5FieldResolutionError> {
    if input.text_field_id.trim().is_empty() {
        return Err(M5FieldResolutionError::EmptyTextFieldId);
    }
    if string_is_forbidden(&input.text_field_id)
        || string_is_forbidden(&input.label)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5FieldResolutionError::ForbiddenMaterial);
    }

    let disposition_requires_distinct = disposition_requires_distinct_treatment(input.disposition);
    let validation_is_flagging = validation_state_is_flagging(input.validation);
    let label_is_permanent =
        !input.label.trim().is_empty() && !label_mode_is_placeholder_only(input.label_mode);

    let degrade_reason = if !label_is_permanent {
        Some(M5TextFieldDegradeReason::LabelIsPlaceholderOnly)
    } else if !input.surface_context.is_resolved() {
        Some(M5TextFieldDegradeReason::SurfaceContextUnresolved)
    } else if !label_mode_is_resolved(input.label_mode) {
        Some(M5TextFieldDegradeReason::LabelModeUnresolved)
    } else if !input.focus_visible_offered {
        Some(M5TextFieldDegradeReason::FocusVisibleTreatmentMissing)
    } else if !validation_state_is_resolved(input.validation) {
        Some(M5TextFieldDegradeReason::ValidationStateUnresolved)
    } else if validation_is_flagging && !input.validation_message_specific {
        Some(M5TextFieldDegradeReason::VagueValidationCopy)
    } else if input.requires_reveal && !input.reveal_offered {
        Some(M5TextFieldDegradeReason::RevealAffordanceMissing)
    } else if disposition_requires_distinct && !input.blocked_state_distinct {
        Some(M5TextFieldDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    } else if !input.draft_preserved_across_interruption {
        Some(M5TextFieldDegradeReason::DraftContinuityLost)
    } else if !input.validation_anchor_preserved {
        Some(M5TextFieldDegradeReason::ValidationAnchorLost)
    } else if input.command_id.trim().is_empty() {
        Some(M5TextFieldDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5TextFieldDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5TextFieldDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5FieldNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedTextField {
        text_field_id: input.text_field_id,
        label: input.label,
        label_mode: input.label_mode.as_str().to_owned(),
        label_is_permanent,
        validation: input.validation.as_str().to_owned(),
        validation_is_flagging,
        validation_message_specific: input.validation_message_specific,
        disposition: input.disposition.as_str().to_owned(),
        disposition_requires_distinct_treatment: disposition_requires_distinct,
        surface_context: input.surface_context.as_str().to_owned(),
        focus_visible_offered: input.focus_visible_offered,
        requires_reveal: input.requires_reveal,
        reveal_offered: input.reveal_offered,
        blocked_state_distinct: input.blocked_state_distinct,
        draft_preserved_across_interruption: input.draft_preserved_across_interruption,
        validation_anchor_preserved: input.validation_anchor_preserved,
        command_id: input.command_id,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        label_and_validation_honest_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a search field so its clear / submit / privacy truth reads honestly at a glance: the field
/// names its permanent label, label mode, validation state, and interaction disposition, offers a search
/// icon and a clear affordance, names a resolved submit model so a user never guesses whether input was
/// submitted or retained, discloses any retention / provider / export cue that materially changes
/// expectations, keeps a blocked state distinct from generic disabled chrome, preserves draft continuity
/// across interruption, and always binds back to one canonical command.
pub fn resolve_search_field(
    input: M5SearchFieldResolutionInput,
) -> Result<M5ResolvedSearchField, M5FieldResolutionError> {
    if input.search_field_id.trim().is_empty() {
        return Err(M5FieldResolutionError::EmptySearchFieldId);
    }
    if string_is_forbidden(&input.search_field_id)
        || string_is_forbidden(&input.label)
        || string_is_forbidden(&input.scope_label)
        || string_is_forbidden(&input.command_id)
    {
        return Err(M5FieldResolutionError::ForbiddenMaterial);
    }

    let disposition_requires_distinct = disposition_requires_distinct_treatment(input.disposition);
    let validation_is_flagging = validation_state_is_flagging(input.validation);
    let submit_is_blocked = input.submit_model.is_blocked();
    let retention_needs_disclosure = input.retention_posture.needs_disclosure();
    let label_is_permanent =
        !input.label.trim().is_empty() && !label_mode_is_placeholder_only(input.label_mode);
    let blocked = disposition_requires_distinct || submit_is_blocked;

    let degrade_reason = if !label_is_permanent {
        Some(M5SearchFieldDegradeReason::LabelIsPlaceholderOnly)
    } else if !input.surface_context.is_resolved() {
        Some(M5SearchFieldDegradeReason::SurfaceContextUnresolved)
    } else if !label_mode_is_resolved(input.label_mode) {
        Some(M5SearchFieldDegradeReason::LabelModeUnresolved)
    } else if !input.offers_search_icon {
        Some(M5SearchFieldDegradeReason::SearchIconMissing)
    } else if !input.offers_clear {
        Some(M5SearchFieldDegradeReason::ClearAffordanceMissing)
    } else if !input.submit_model.is_resolved() {
        Some(M5SearchFieldDegradeReason::SubmitModelUnresolved)
    } else if validation_is_flagging && !input.validation_message_specific {
        Some(M5SearchFieldDegradeReason::VagueValidationCopy)
    } else if !input.retention_posture.is_resolved() {
        Some(M5SearchFieldDegradeReason::RetentionUnresolved)
    } else if retention_needs_disclosure && !input.privacy_disclosed {
        Some(M5SearchFieldDegradeReason::PrivacyCueMissing)
    } else if blocked && !input.blocked_state_distinct {
        Some(M5SearchFieldDegradeReason::BlockedStateHiddenBehindDisabled)
    } else if !input.draft_preserved_across_interruption {
        Some(M5SearchFieldDegradeReason::DraftContinuityLost)
    } else if input.command_id.trim().is_empty() {
        Some(M5SearchFieldDegradeReason::CommandBindingUnstated)
    } else if !input.command_route_available {
        Some(M5SearchFieldDegradeReason::CommandTracePathMissing)
    } else if !input.proof_fresh {
        Some(M5SearchFieldDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5FieldNextAction::OpenCommandDetail,
    };

    Ok(M5ResolvedSearchField {
        search_field_id: input.search_field_id,
        label: input.label,
        label_mode: input.label_mode.as_str().to_owned(),
        label_is_permanent,
        validation: input.validation.as_str().to_owned(),
        validation_is_flagging,
        validation_message_specific: input.validation_message_specific,
        disposition: input.disposition.as_str().to_owned(),
        disposition_requires_distinct_treatment: disposition_requires_distinct,
        surface_context: input.surface_context.as_str().to_owned(),
        offers_search_icon: input.offers_search_icon,
        offers_clear: input.offers_clear,
        submit_model: input.submit_model.as_str().to_owned(),
        submit_model_resolved: input.submit_model.is_resolved(),
        submit_is_blocked,
        scope_label: input.scope_label,
        retention_posture: input.retention_posture.as_str().to_owned(),
        retention_needs_disclosure,
        privacy_disclosed: input.privacy_disclosed,
        blocked_state_distinct: input.blocked_state_distinct,
        draft_preserved_across_interruption: input.draft_preserved_across_interruption,
        command_id: input.command_id,
        command_route_available: input.command_route_available,
        degrade_reason,
        next_action,
        clear_submit_privacy_honest_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved text-field and search-field examples it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TextFieldSearchFieldControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5FieldConsumerSurface,
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
    pub anatomy_parts: Vec<M5FieldAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5FieldExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    /// Resolved text-field examples.
    pub text_field_examples: Vec<M5ResolvedTextField>,
    /// Resolved search-field examples.
    pub search_field_examples: Vec<M5ResolvedSearchField>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: placeholder text never replaces the permanent label.
    pub placeholder_text_replaces_label: bool,
    /// Hard invariant: validation copy is never vague when the field is flagging.
    pub vague_validation_copy_used: bool,
    /// Hard invariant: clear / submit / privacy truth is never dropped on a search field.
    pub clear_submit_or_privacy_truth_dropped: bool,
    /// Hard invariant: locked / read-only / degraded semantics never hide behind generic disabled chrome.
    pub locked_or_degraded_semantics_hidden_behind_disabled: bool,
}

impl M5TextFieldSearchFieldControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5FieldAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5FieldAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5FieldExportField> = self.export_fields.iter().copied().collect();
        M5FieldExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.placeholder_text_replaces_label
            && !self.vague_validation_copy_used
            && !self.clear_submit_or_privacy_truth_dropped
            && !self.locked_or_degraded_semantics_hidden_behind_disabled
    }

    /// True when a clean text field keeps its label permanent, its validation copy specific, its reveal
    /// control present where required, its locked / read-only / degraded state distinct, its draft and
    /// validation anchor preserved, and a command trace path reachable.
    fn text_is_honest(ex: &M5ResolvedTextField) -> bool {
        !ex.is_clean()
            || (ex.label_is_permanent
                && (!ex.validation_is_flagging || ex.validation_message_specific)
                && (!ex.requires_reveal || ex.reveal_offered)
                && (!ex.disposition_requires_distinct_treatment || ex.blocked_state_distinct)
                && ex.draft_preserved_across_interruption
                && ex.validation_anchor_preserved
                && ex.command_route_available)
    }

    /// True when a clean search field keeps its label permanent, offers clear and a resolved submit
    /// model, keeps its validation copy specific, discloses any material retention / privacy cue, keeps a
    /// blocked state distinct, preserves draft continuity, and offers a command trace path.
    fn search_is_honest(ex: &M5ResolvedSearchField) -> bool {
        !ex.is_clean()
            || (ex.label_is_permanent
                && ex.offers_clear
                && ex.submit_model_resolved
                && (!ex.validation_is_flagging || ex.validation_message_specific)
                && (!ex.retention_needs_disclosure || ex.privacy_disclosed)
                && (!(ex.disposition_requires_distinct_treatment || ex.submit_is_blocked)
                    || ex.blocked_state_distinct)
                && ex.draft_preserved_across_interruption
                && ex.command_route_available)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.text_field_examples.iter().all(Self::text_is_honest)
            && self
                .search_field_examples
                .iter()
                .all(Self::search_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TextFieldSearchFieldVocabularySet {
    /// Interaction-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Field-label-mode tokens (bound from the frozen matrix).
    pub field_label_modes: Vec<String>,
    /// Field-validation-state tokens (bound from the frozen matrix).
    pub field_validation_states: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Search-submit-model tokens (minted by this lane).
    pub search_submit_models: Vec<String>,
    /// Search-retention-posture tokens (minted by this lane).
    pub search_retention_postures: Vec<String>,
    /// Text-field degrade-reason tokens.
    pub text_field_degrade_reasons: Vec<String>,
    /// Search-field degrade-reason tokens.
    pub search_field_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5TextFieldSearchFieldVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5CoreControlDisposition::ALL, |v| v.as_str()),
            field_label_modes: tokens(&M5FieldLabelMode::ALL, |v| v.as_str()),
            field_validation_states: tokens(&M5FieldValidationState::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5FieldSurfaceContext::ALL, |v| v.as_str()),
            search_submit_models: tokens(&M5SearchSubmitModel::ALL, |v| v.as_str()),
            search_retention_postures: tokens(&M5SearchRetentionPosture::ALL, |v| v.as_str()),
            text_field_degrade_reasons: tokens(&M5TextFieldDegradeReason::ALL, |v| v.as_str()),
            search_field_degrade_reasons: tokens(&M5SearchFieldDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5FieldAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5FieldNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5FieldExportField::ALL, |v| v.as_str()),
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
pub struct M5TextFieldSearchFieldGovernanceReview {
    /// The text field names a permanent label and its validation state.
    pub text_names_permanent_label_and_validation: bool,
    /// The text field never uses placeholder text as the only label.
    pub text_never_uses_placeholder_as_label: bool,
    /// The text field keeps validation copy specific rather than vague.
    pub text_keeps_validation_copy_specific: bool,
    /// The text field preserves draft state and exact validation anchors across interruption.
    pub text_preserves_draft_and_validation_anchors: bool,
    /// Focus-visible treatment is present on the fields.
    pub focus_visible_treatment_present: bool,
    /// The search field exposes clear and submit truth.
    pub search_exposes_clear_and_submit_truth: bool,
    /// The search field discloses retention and privacy cues that materially change expectations.
    pub search_discloses_retention_and_privacy_cues: bool,
    /// The search field keeps a blocked state distinct rather than behind generic disabled chrome.
    pub search_keeps_blocked_state_distinct: bool,
    /// The search field preserves draft state across interruption.
    pub search_preserves_draft_across_interruption: bool,
    /// Locked / read-only / degraded semantics are never hidden behind generic disabled chrome.
    pub locked_and_degraded_never_hidden_behind_disabled: bool,
    /// Both fields bind one canonical command with a command trace path.
    pub both_bind_canonical_command_with_trace: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TextFieldSearchFieldConsumerProjection {
    /// Forms surfaces consume the shared text-field and search-field vocabulary.
    pub forms_surfaces_consume_text_and_search_vocabulary: bool,
    /// Settings surfaces consume the shared text-field vocabulary.
    pub settings_surfaces_consume_text_vocabulary: bool,
    /// Search surfaces consume the shared search-field vocabulary.
    pub search_surfaces_consume_search_vocabulary: bool,
    /// Entry surfaces consume the shared field vocabulary.
    pub entry_surfaces_consume_field_vocabulary: bool,
    /// Label / validation / privacy facts trace back to one canonical component contract.
    pub label_validation_and_privacy_facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical control source.
    pub support_export_reads_single_control_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TextFieldSearchFieldProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the control.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TextFieldSearchFieldReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TextFieldSearchFieldControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TextFieldSearchFieldControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TextFieldSearchFieldControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TextFieldSearchFieldVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TextFieldSearchFieldGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TextFieldSearchFieldConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TextFieldSearchFieldProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TextFieldSearchFieldReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 text-field / search-field controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TextFieldSearchFieldControlsPacket {
    /// Record kind; must equal [`M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TextFieldSearchFieldControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TextFieldSearchFieldVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TextFieldSearchFieldGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TextFieldSearchFieldConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TextFieldSearchFieldProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TextFieldSearchFieldReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TextFieldSearchFieldControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5TextFieldSearchFieldControlsPacketInput) -> Self {
        Self {
            record_kind: M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5TextFieldSearchFieldControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_RECORD_KIND {
            violations.push(M5TextFieldSearchFieldControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_VERSION {
            violations.push(M5TextFieldSearchFieldControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TextFieldSearchFieldControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5TextFieldSearchFieldControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 text-field / search-field controls packet serializes"),
        ) {
            violations.push(M5TextFieldSearchFieldControlsViolation::RawMaterialInExport);
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
            .expect("m5 text-field / search-field controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,text_field_examples,search_field_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .text_field_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.search_field_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.text_field_examples.len(),
                row.search_field_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Text-Field and Search-Field Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Field label modes: {}\n",
            self.vocabulary_set.field_label_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Search retention postures: {}\n",
            self.vocabulary_set.search_retention_postures.join(", ")
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
                "  - Text-field examples: {} / search-field examples: {}\n",
                row.text_field_examples.len(),
                row.search_field_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5TextFieldSearchFieldControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TextFieldSearchFieldControlsViolation>),
}

impl fmt::Display for M5TextFieldSearchFieldControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 text-field / search-field controls export parse failed: {error}"
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
                    "m5 text-field / search-field controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TextFieldSearchFieldControlsArtifactError {}

/// Validation failures emitted by [`M5TextFieldSearchFieldControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TextFieldSearchFieldControlsViolation {
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
    /// A controls row carries a dishonest clean example (placeholder-only label, vague validation,
    /// missing reveal, undisclosed privacy, hidden blocked state, dropped clear, or missing trace).
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
    /// Labeling and validation-copy honesty is not proven: clean fields do not cover the permanent label
    /// modes, or no placeholder-only / vague-validation example degrades, or a clean field is
    /// placeholder-only or carries vague validation copy.
    LabelingAndValidationCopyNotProven,
    /// Clear / submit / privacy / blocked truth is not proven: no clean search field offers clear with a
    /// resolved submit model and discloses privacy across live / provider-backed / export-sensitive
    /// postures, or no privacy-missing / clear-missing example degrades, or a clean search drops clear or
    /// a material privacy cue.
    ClearSubmitPrivacyBlockedTruthNotProven,
    /// Draft and validation continuity is not proven: no clean text field and clean search field preserve
    /// draft continuity with a command trace, or no draft-lost / validation-anchor-lost example degrades.
    DraftAndValidationContinuityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TextFieldSearchFieldControlsViolation {
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
            Self::LabelingAndValidationCopyNotProven => "labeling_and_validation_copy_not_proven",
            Self::ClearSubmitPrivacyBlockedTruthNotProven => {
                "clear_submit_privacy_blocked_truth_not_proven"
            }
            Self::DraftAndValidationContinuityNotProven => {
                "draft_and_validation_continuity_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_text_field_search_field_controls_export(
) -> Result<M5TextFieldSearchFieldControlsPacket, M5TextFieldSearchFieldControlsArtifactError> {
    let packet: M5TextFieldSearchFieldControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-text-field-search-field-controls-proof/support_export.json"
    )))
    .map_err(M5TextFieldSearchFieldControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TextFieldSearchFieldControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_SCHEMA_REF,
        M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_DOC_REF,
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_TEXT_FIELD_SCHEMA_REF,
        M5_SEARCH_FIELD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TextFieldSearchFieldControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5TextFieldSearchFieldControlsViolation::NoControlsRows);
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
            violations.push(M5TextFieldSearchFieldControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TextFieldSearchFieldControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TextFieldSearchFieldControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TEXT_FIELD_SCHEMA_REF) || !refs.contains(M5_SEARCH_FIELD_SCHEMA_REF) {
            violations.push(M5TextFieldSearchFieldControlsViolation::ComponentSchemaRefMissing);
        }
        if row.text_field_examples.is_empty() || row.search_field_examples.is_empty() {
            violations.push(M5TextFieldSearchFieldControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5TextFieldSearchFieldControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5TextFieldSearchFieldControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.text_names_permanent_label_and_validation,
        review.text_never_uses_placeholder_as_label,
        review.text_keeps_validation_copy_specific,
        review.text_preserves_draft_and_validation_anchors,
        review.focus_visible_treatment_present,
        review.search_exposes_clear_and_submit_truth,
        review.search_discloses_retention_and_privacy_cues,
        review.search_keeps_blocked_state_distinct,
        review.search_preserves_draft_across_interruption,
        review.locked_and_degraded_never_hidden_behind_disabled,
        review.both_bind_canonical_command_with_trace,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5TextFieldSearchFieldControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.forms_surfaces_consume_text_and_search_vocabulary,
        projection.settings_surfaces_consume_text_vocabulary,
        projection.search_surfaces_consume_search_vocabulary,
        projection.entry_surfaces_consume_field_vocabulary,
        projection.label_validation_and_privacy_facts_trace_to_single_component_contract,
        projection.support_export_reads_single_control_source,
    ] {
        if !ok {
            violations.push(M5TextFieldSearchFieldControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TextFieldSearchFieldControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TextFieldSearchFieldControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5TextFieldSearchFieldControlsPacket,
    violations: &mut Vec<M5TextFieldSearchFieldControlsViolation>,
) {
    let texts = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.text_field_examples.iter())
    };
    let searches = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.search_field_examples.iter())
    };

    // AC1: no claimed M5 text/search field relies on placeholder-only labeling or vague validation copy.
    // Clean fields cover at least the permanent label modes, a placeholder-only example degrades, a
    // vague-validation example degrades, and no clean field is placeholder-only or carries vague copy.
    let clean_label_modes: BTreeSet<String> = texts()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.label_mode.clone())
        .chain(
            searches()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.label_mode.clone()),
        )
        .collect();
    let label_grammar_covered = ["persistent_label", "floating_label"]
        .iter()
        .all(|m| clean_label_modes.contains(*m));
    let placeholder_only_degrades = texts()
        .any(|ex| ex.degrade_reason == Some(M5TextFieldDegradeReason::LabelIsPlaceholderOnly))
        || searches().any(|ex| {
            ex.degrade_reason == Some(M5SearchFieldDegradeReason::LabelIsPlaceholderOnly)
        });
    let vague_validation_degrades = texts()
        .any(|ex| ex.degrade_reason == Some(M5TextFieldDegradeReason::VagueValidationCopy))
        || searches()
            .any(|ex| ex.degrade_reason == Some(M5SearchFieldDegradeReason::VagueValidationCopy));
    let no_clean_placeholder_or_vague = texts().all(|ex| {
        !(ex.is_clean()
            && (!ex.label_is_permanent
                || (ex.validation_is_flagging && !ex.validation_message_specific)))
    }) && searches().all(|ex| {
        !(ex.is_clean()
            && (!ex.label_is_permanent
                || (ex.validation_is_flagging && !ex.validation_message_specific)))
    });
    if !(label_grammar_covered
        && placeholder_only_degrades
        && vague_validation_degrades
        && no_clean_placeholder_or_vague)
    {
        violations
            .push(M5TextFieldSearchFieldControlsViolation::LabelingAndValidationCopyNotProven);
    }

    // AC2: search-entry consumers expose clear / submit / privacy / blocked truth consistently across
    // local, provider-backed, and export-sensitive surfaces. At least one clean search field offers clear
    // with a resolved submit model and a command binding, clean searches cover the live and a disclosed
    // (cached / provider / export) retention posture, a privacy-cue-missing example degrades, a
    // clear-missing example degrades, and no clean search drops clear or a material privacy cue.
    let clean_search_full = searches().any(|ex| {
        ex.is_clean()
            && ex.offers_clear
            && ex.submit_model_resolved
            && !ex.command_id.trim().is_empty()
    });
    let clean_retention: BTreeSet<String> = searches()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.retention_posture.clone())
        .collect();
    let retention_grammar_covered = clean_retention.contains("live_not_retained")
        && [
            "cached_results_disclosed",
            "provider_backed_remote",
            "export_sensitive",
        ]
        .iter()
        .any(|p| clean_retention.contains(*p));
    let privacy_missing_degrades = searches()
        .any(|ex| ex.degrade_reason == Some(M5SearchFieldDegradeReason::PrivacyCueMissing));
    let clear_missing_degrades = searches()
        .any(|ex| ex.degrade_reason == Some(M5SearchFieldDegradeReason::ClearAffordanceMissing));
    let no_clean_dropped_clear_or_privacy = searches().all(|ex| {
        !(ex.is_clean()
            && (!ex.offers_clear || (ex.retention_needs_disclosure && !ex.privacy_disclosed)))
    });
    if !(clean_search_full
        && retention_grammar_covered
        && privacy_missing_degrades
        && clear_missing_degrades
        && no_clean_dropped_clear_or_privacy)
    {
        violations
            .push(M5TextFieldSearchFieldControlsViolation::ClearSubmitPrivacyBlockedTruthNotProven);
    }

    // AC3: text/search draft state survives the first interruption/recovery without losing source or
    // validation context. At least one clean text field and one clean search field preserve draft
    // continuity with a command trace, a draft-continuity-lost example degrades, and a validation-anchor-
    // lost example degrades.
    let traceable_text = texts().any(|ex| {
        ex.is_clean()
            && ex.command_route_available
            && ex.draft_preserved_across_interruption
            && ex.validation_anchor_preserved
    });
    let traceable_search = searches().any(|ex| {
        ex.is_clean() && ex.command_route_available && ex.draft_preserved_across_interruption
    });
    let draft_lost_degrades = texts()
        .any(|ex| ex.degrade_reason == Some(M5TextFieldDegradeReason::DraftContinuityLost))
        || searches()
            .any(|ex| ex.degrade_reason == Some(M5SearchFieldDegradeReason::DraftContinuityLost));
    let anchor_lost_degrades =
        texts().any(|ex| ex.degrade_reason == Some(M5TextFieldDegradeReason::ValidationAnchorLost));
    if !(traceable_text && traceable_search && draft_lost_degrades && anchor_lost_degrades) {
        violations
            .push(M5TextFieldSearchFieldControlsViolation::DraftAndValidationContinuityNotProven);
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
    M5CoreControlFamily::TextField,
    M5CoreControlFamily::SearchField,
];
