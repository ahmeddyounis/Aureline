//! Frozen M5 button, icon-button, split-button, text-field, search-field, combobox,
//! checkbox/radio/switch toggle-control, and segmented-control component matrix.
//!
//! This module locks Aureline's most reused atomic action and input controls into one export-safe
//! packet. Every claimed M5 surface that still ships its own button, icon button, split button, text
//! field, search field, combobox, boolean toggle, or segmented control — across the forms, settings,
//! search, entry (start-center), review, and repair surfaces — is named once here and constrained by
//! the same interaction-state taxonomy (default, hover, focus-visible, pressed, loading, disabled,
//! locked, read-only, degraded), the same button emphasis and icon-label truth, the same split-button
//! default-safety, the same field label-permanence and validation truth, the same search clear/submit/
//! privacy affordances, the same combobox value-source truth, the same checkbox/radio/switch semantics,
//! and the same segmented-mode-versus-navigation distinction regardless of the feature family that
//! renders it.
//!
//! The matrix does not re-implement form workflows, settings precedence, search sessions, or command
//! routing — it is the shared reusable-control-honesty contract those flows consume. The controlled
//! vocabularies are frozen in one self-describing [`M5CoreControlVocabularySet`] rather than minted per
//! feature. The single controlled interaction-state vocabulary consumers bind to — default, hover,
//! focus-visible, pressed, loading, disabled, locked, read-only, and degraded — keeps placeholder text
//! from replacing labels, keeps loading buttons from relabeling the action or losing attribution, keeps
//! icon-only destructive actions from going unlabeled, keeps switches from being blurred with deferred
//! checkboxes, keeps split buttons from defaulting to riskier alternates, and keeps locked and degraded
//! semantics from hiding behind generic disabled chrome. Raw secret values and private endpoints stay
//! outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_core_action_input_component_matrix,
    seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed,
    seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed,
    M5_CORE_CONTROL_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CoreControlComponentMatrixPacket`].
pub const M5_CORE_CONTROL_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_button_icon_button_split_button_text_field_search_field_combobox_toggle_control_and_segmented_control_component_matrix";

/// Schema version for M5 core-action-input component-matrix records.
pub const M5_CORE_CONTROL_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined core-action-input component-matrix schema.
pub const M5_CORE_CONTROL_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-core-action-input-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CORE_CONTROL_COMPONENT_DOC_REF: &str =
    "docs/components/m5_core_action_input_components_contract.md";

/// Repo-relative path of the button canonical component schema.
pub const M5_BUTTON_SCHEMA_REF: &str = "schemas/ui/m5-button.schema.json";

/// Repo-relative path of the icon-button canonical component schema.
pub const M5_ICON_BUTTON_SCHEMA_REF: &str = "schemas/ui/m5-icon-button.schema.json";

/// Repo-relative path of the split-button canonical component schema.
pub const M5_SPLIT_BUTTON_SCHEMA_REF: &str = "schemas/ui/m5-split-button.schema.json";

/// Repo-relative path of the text-field canonical component schema.
pub const M5_TEXT_FIELD_SCHEMA_REF: &str = "schemas/ui/m5-text-field.schema.json";

/// Repo-relative path of the search-field canonical component schema.
pub const M5_SEARCH_FIELD_SCHEMA_REF: &str = "schemas/ui/m5-search-field.schema.json";

/// Repo-relative path of the combobox canonical component schema.
pub const M5_COMBOBOX_SCHEMA_REF: &str = "schemas/ui/m5-combobox.schema.json";

/// Repo-relative path of the checkbox/radio/switch toggle-control canonical component schema.
pub const M5_TOGGLE_CONTROL_SCHEMA_REF: &str = "schemas/ui/m5-toggle-control.schema.json";

/// Repo-relative path of the segmented-control canonical component schema.
pub const M5_SEGMENTED_CONTROL_SCHEMA_REF: &str = "schemas/ui/m5-segmented-control.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CORE_CONTROL_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-core-action-input-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CORE_CONTROL_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-core-action-input-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CORE_CONTROL_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-core-action-input-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CORE_CONTROL_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-core-action-input-component-matrix.md";

/// One of the eight governed core action / input control families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlFamily {
    /// A text button that launches an action with a permanent label and stable emphasis.
    Button,
    /// An icon-only button that never leaves a destructive action unlabeled.
    IconButton,
    /// A split button whose default action stays the safe one, never a riskier alternate.
    SplitButton,
    /// A single-line text field whose label is permanent and whose validation truth is legible.
    TextField,
    /// A search field that preserves clear / submit / privacy / validation truth.
    SearchField,
    /// A combobox that preserves filterability and source-of-value truth.
    Combobox,
    /// A checkbox / radio / switch toggle control whose boolean semantics stay distinct.
    ToggleControl,
    /// A segmented control that stays a small mode / view toggle, never stealth navigation.
    SegmentedControl,
}

impl M5CoreControlFamily {
    /// Every governed control family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Button,
        Self::IconButton,
        Self::SplitButton,
        Self::TextField,
        Self::SearchField,
        Self::Combobox,
        Self::ToggleControl,
        Self::SegmentedControl,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::IconButton => "icon_button",
            Self::SplitButton => "split_button",
            Self::TextField => "text_field",
            Self::SearchField => "search_field",
            Self::Combobox => "combobox",
            Self::ToggleControl => "toggle_control",
            Self::SegmentedControl => "segmented_control",
        }
    }

    /// The canonical per-component schema ref a downstream control points at instead of restating this
    /// control's state / label / value truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::Button => M5_BUTTON_SCHEMA_REF,
            Self::IconButton => M5_ICON_BUTTON_SCHEMA_REF,
            Self::SplitButton => M5_SPLIT_BUTTON_SCHEMA_REF,
            Self::TextField => M5_TEXT_FIELD_SCHEMA_REF,
            Self::SearchField => M5_SEARCH_FIELD_SCHEMA_REF,
            Self::Combobox => M5_COMBOBOX_SCHEMA_REF,
            Self::ToggleControl => M5_TOGGLE_CONTROL_SCHEMA_REF,
            Self::SegmentedControl => M5_SEGMENTED_CONTROL_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled button emphasis.
    pub const fn declares_button_emphasis(self) -> bool {
        matches!(self, Self::Button | Self::IconButton | Self::SplitButton)
    }

    /// `true` when this family must name a controlled icon-label mode.
    pub const fn declares_icon_label_mode(self) -> bool {
        matches!(self, Self::IconButton)
    }

    /// `true` when this family must name a controlled split-button default posture.
    pub const fn declares_split_posture(self) -> bool {
        matches!(self, Self::SplitButton)
    }

    /// `true` when this family must name a controlled field label mode.
    pub const fn declares_field_label_mode(self) -> bool {
        matches!(self, Self::TextField | Self::SearchField | Self::Combobox)
    }

    /// `true` when this family must name a controlled field validation state.
    pub const fn declares_field_validation(self) -> bool {
        matches!(self, Self::TextField | Self::SearchField | Self::Combobox)
    }

    /// `true` when this family must name a controlled search-field affordance.
    pub const fn declares_search_affordance(self) -> bool {
        matches!(self, Self::SearchField)
    }

    /// `true` when this family must name a controlled combobox value source.
    pub const fn declares_combobox_value_source(self) -> bool {
        matches!(self, Self::Combobox)
    }

    /// `true` when this family must name a controlled toggle semantics.
    pub const fn declares_toggle_semantics(self) -> bool {
        matches!(self, Self::ToggleControl)
    }

    /// `true` when this family must name a controlled segmented mode.
    pub const fn declares_segmented_mode(self) -> bool {
        matches!(self, Self::SegmentedControl)
    }
}

/// The single controlled interaction-state vocabulary every forms, settings, search, entry, review, or
/// repair consumer binds to. These are the exact acceptance-criteria tokens that keep `default`,
/// `hover`, `focus-visible`, `pressed`, `loading`, `disabled`, `locked`, `read-only`, and `degraded`
/// meaning the same thing everywhere these controls ship. No feature family invents a parallel word for
/// any of these states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlDisposition {
    /// The resting, interactive state.
    Default,
    /// The pointer is hovering the control.
    Hover,
    /// The control holds visible keyboard focus.
    FocusVisible,
    /// The control is being pressed / activated.
    Pressed,
    /// The control is busy without relabeling the action or losing attribution.
    Loading,
    /// The control is disabled.
    Disabled,
    /// The control is locked (policy / permission), distinct from generic disabled.
    Locked,
    /// The control is read-only (value shown, not editable), distinct from disabled.
    ReadOnly,
    /// The control is degraded (a required signal is unavailable), never hidden behind disabled chrome.
    Degraded,
}

impl M5CoreControlDisposition {
    /// Every interaction-state token, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Default,
        Self::Hover,
        Self::FocusVisible,
        Self::Pressed,
        Self::Loading,
        Self::Disabled,
        Self::Locked,
        Self::ReadOnly,
        Self::Degraded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Hover => "hover",
            Self::FocusVisible => "focus_visible",
            Self::Pressed => "pressed",
            Self::Loading => "loading",
            Self::Disabled => "disabled",
            Self::Locked => "locked",
            Self::ReadOnly => "read_only",
            Self::Degraded => "degraded",
        }
    }

    /// Whether this disposition names a blocked-interaction state that must never be collapsed into one
    /// generic disabled chrome (`disabled`, `locked`, `read_only`).
    pub const fn is_interaction_blocked(self) -> bool {
        matches!(self, Self::Disabled | Self::Locked | Self::ReadOnly)
    }
}

/// Controlled button emphasis — the semantic weight of a button, so a quiet or destructive action is
/// never presented with the same weight as a primary one and emphasis is never encoded by color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ButtonEmphasis {
    /// The primary call to action.
    Primary,
    /// A secondary action.
    Secondary,
    /// A quiet / low-emphasis action.
    Quiet,
    /// A destructive action.
    Destructive,
    /// A ghost / borderless action.
    Ghost,
    /// A link-styled action.
    Link,
}

impl M5ButtonEmphasis {
    /// Every button emphasis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Primary,
        Self::Secondary,
        Self::Quiet,
        Self::Destructive,
        Self::Ghost,
        Self::Link,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Quiet => "quiet",
            Self::Destructive => "destructive",
            Self::Ghost => "ghost",
            Self::Link => "link",
        }
    }

    /// Whether this emphasis names a destructive action that must always carry a label.
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::Destructive)
    }
}

/// Controlled icon-label mode — how an icon-only control carries its accessible name, so an icon-only
/// destructive action is never left unlabeled and a decorative glyph is never mistaken for a control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconLabelMode {
    /// A visible text label sits beside the icon.
    LabeledVisible,
    /// An accessible name is present even though no text is visible.
    AccessibleNameOnly,
    /// The label is available on hover / focus as a tooltip in addition to an accessible name.
    TooltipLabeled,
    /// Text and icon together.
    TextWithIcon,
    /// A decorative-only glyph that is not itself a control (disallowed for actionable icons).
    DecorativeOnly,
    /// The label mode cannot currently be resolved.
    LabelUnresolved,
}

impl M5IconLabelMode {
    /// Every icon-label mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LabeledVisible,
        Self::AccessibleNameOnly,
        Self::TooltipLabeled,
        Self::TextWithIcon,
        Self::DecorativeOnly,
        Self::LabelUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabeledVisible => "labeled_visible",
            Self::AccessibleNameOnly => "accessible_name_only",
            Self::TooltipLabeled => "tooltip_labeled",
            Self::TextWithIcon => "text_with_icon",
            Self::DecorativeOnly => "decorative_only",
            Self::LabelUnresolved => "label_unresolved",
        }
    }
}

/// Controlled split-button default posture — how safe the primary (default-click) action of a split
/// button is, so a split button never defaults to a riskier alternate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SplitDefaultPosture {
    /// The default action is the safe primary one.
    PrimaryDefaultSafe,
    /// Alternates are reachable only by explicit selection, never the default click.
    ExplicitAlternate,
    /// The default action requires an explicit confirm.
    ConfirmRequired,
    /// A destructive alternate is guarded behind a distinct, labeled step.
    DestructiveGuarded,
    /// The whole split button is disabled.
    AllDisabled,
    /// The default posture cannot currently be resolved.
    PostureUnknown,
}

impl M5SplitDefaultPosture {
    /// Every split-button default posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PrimaryDefaultSafe,
        Self::ExplicitAlternate,
        Self::ConfirmRequired,
        Self::DestructiveGuarded,
        Self::AllDisabled,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryDefaultSafe => "primary_default_safe",
            Self::ExplicitAlternate => "explicit_alternate",
            Self::ConfirmRequired => "confirm_required",
            Self::DestructiveGuarded => "destructive_guarded",
            Self::AllDisabled => "all_disabled",
            Self::PostureUnknown => "posture_unknown",
        }
    }
}

/// Controlled field label mode — how an input field carries its permanent label, so placeholder text is
/// never used as the only label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FieldLabelMode {
    /// A persistent visible label.
    PersistentLabel,
    /// A floating label that remains visible once filled.
    FloatingLabel,
    /// A label plus a distinct placeholder hint.
    LabelPlusPlaceholder,
    /// An accessible label only (no visible text), still a real permanent label.
    AriaLabelOnly,
    /// Placeholder-only labeling, which is disallowed.
    PlaceholderOnlyDisallowed,
    /// The label mode cannot currently be resolved.
    LabelUnresolved,
}

impl M5FieldLabelMode {
    /// Every field label mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PersistentLabel,
        Self::FloatingLabel,
        Self::LabelPlusPlaceholder,
        Self::AriaLabelOnly,
        Self::PlaceholderOnlyDisallowed,
        Self::LabelUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistentLabel => "persistent_label",
            Self::FloatingLabel => "floating_label",
            Self::LabelPlusPlaceholder => "label_plus_placeholder",
            Self::AriaLabelOnly => "aria_label_only",
            Self::PlaceholderOnlyDisallowed => "placeholder_only_disallowed",
            Self::LabelUnresolved => "label_unresolved",
        }
    }
}

/// Controlled field validation state — the validation truth of an input, so an invalid or unvalidated
/// value is never presented as valid and a pending async check is never read as complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FieldValidationState {
    /// The value is valid.
    Valid,
    /// The value is invalid and blocks submission.
    InvalidBlocking,
    /// A non-blocking warning applies.
    WarningNonblocking,
    /// An async validation is pending.
    PendingAsync,
    /// The value has not been validated yet.
    NotValidated,
    /// The validation state cannot currently be resolved.
    ValidationUnknown,
}

impl M5FieldValidationState {
    /// Every field validation state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Valid,
        Self::InvalidBlocking,
        Self::WarningNonblocking,
        Self::PendingAsync,
        Self::NotValidated,
        Self::ValidationUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::InvalidBlocking => "invalid_blocking",
            Self::WarningNonblocking => "warning_nonblocking",
            Self::PendingAsync => "pending_async",
            Self::NotValidated => "not_validated",
            Self::ValidationUnknown => "validation_unknown",
        }
    }
}

/// Controlled search-field affordance — the clear / submit / privacy behavior of a search field, so a
/// search field never hides whether it clears, how it submits, or whether its history is private.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SearchFieldAffordance {
    /// Offers an explicit clear affordance.
    Clearable,
    /// Submits on an explicit action (Enter / button).
    SubmitExplicit,
    /// Submits as-you-type.
    SubmitAsYouType,
    /// Keeps its history private (not persisted / not shared).
    HistoryPrivate,
    /// Scopes the search to a named scope.
    ScopedSearch,
    /// The affordance cannot currently be resolved.
    AffordanceUnknown,
}

impl M5SearchFieldAffordance {
    /// Every search-field affordance, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Clearable,
        Self::SubmitExplicit,
        Self::SubmitAsYouType,
        Self::HistoryPrivate,
        Self::ScopedSearch,
        Self::AffordanceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clearable => "clearable",
            Self::SubmitExplicit => "submit_explicit",
            Self::SubmitAsYouType => "submit_as_you_type",
            Self::HistoryPrivate => "history_private",
            Self::ScopedSearch => "scoped_search",
            Self::AffordanceUnknown => "affordance_unknown",
        }
    }
}

/// Controlled combobox value source — where a combobox's committed value comes from, so a free-text or
/// remote / unverified value is never presented as a canonical option and filterability is honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComboboxValueSource {
    /// A canonical option chosen from the list.
    CanonicalOption,
    /// A canonical option chosen from a filtered subset.
    FilteredSubset,
    /// A free-text value the field explicitly allows.
    FreeTextAllowed,
    /// A value backed by a remote source.
    RemoteBacked,
    /// A custom, unverified value.
    CustomUnverified,
    /// The value source cannot currently be resolved.
    SourceUnknown,
}

impl M5ComboboxValueSource {
    /// Every combobox value source, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CanonicalOption,
        Self::FilteredSubset,
        Self::FreeTextAllowed,
        Self::RemoteBacked,
        Self::CustomUnverified,
        Self::SourceUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalOption => "canonical_option",
            Self::FilteredSubset => "filtered_subset",
            Self::FreeTextAllowed => "free_text_allowed",
            Self::RemoteBacked => "remote_backed",
            Self::CustomUnverified => "custom_unverified",
            Self::SourceUnknown => "source_unknown",
        }
    }
}

/// Controlled toggle semantics — which boolean control a toggle actually is, so a switch is never
/// blurred with a deferred checkbox and a radio's exclusivity is never lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToggleSemantics {
    /// A checkbox that applies immediately.
    CheckboxImmediate,
    /// A checkbox whose change is deferred until an explicit save.
    CheckboxDeferred,
    /// A radio in an exclusive group.
    RadioExclusive,
    /// A switch that applies immediately.
    SwitchImmediate,
    /// A tri-state / indeterminate checkbox.
    TristateIndeterminate,
    /// The semantics cannot currently be resolved.
    SemanticsUnknown,
}

impl M5ToggleSemantics {
    /// Every toggle semantics, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CheckboxImmediate,
        Self::CheckboxDeferred,
        Self::RadioExclusive,
        Self::SwitchImmediate,
        Self::TristateIndeterminate,
        Self::SemanticsUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheckboxImmediate => "checkbox_immediate",
            Self::CheckboxDeferred => "checkbox_deferred",
            Self::RadioExclusive => "radio_exclusive",
            Self::SwitchImmediate => "switch_immediate",
            Self::TristateIndeterminate => "tristate_indeterminate",
            Self::SemanticsUnknown => "semantics_unknown",
        }
    }
}

/// Controlled segmented mode — what a segmented control does, so it stays a small mode / view toggle and
/// is never used as stealth top-level navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SegmentedMode {
    /// Toggles a mode.
    ModeToggle,
    /// Switches a view of the same content.
    ViewSwitch,
    /// A single-select over a small set.
    SingleSelectSmallSet,
    /// Mutually exclusive options.
    ExclusiveOptions,
    /// Explicitly not navigation.
    NotNavigation,
    /// The mode cannot currently be resolved.
    ModeUnknown,
}

impl M5SegmentedMode {
    /// Every segmented mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ModeToggle,
        Self::ViewSwitch,
        Self::SingleSelectSmallSet,
        Self::ExclusiveOptions,
        Self::NotNavigation,
        Self::ModeUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeToggle => "mode_toggle",
            Self::ViewSwitch => "view_switch",
            Self::SingleSelectSmallSet => "single_select_small_set",
            Self::ExclusiveOptions => "exclusive_options",
            Self::NotNavigation => "not_navigation",
            Self::ModeUnknown => "mode_unknown",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a core control. No control may invent a parallel
/// surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlSurfaceFamily {
    /// The forms surface.
    Forms,
    /// The settings surface.
    Settings,
    /// The search surface.
    Search,
    /// The start-center entry surface.
    Entry,
    /// The review surface.
    Review,
    /// The repair surface.
    Repair,
    /// The support export.
    SupportExport,
}

impl M5CoreControlSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Forms,
        Self::Settings,
        Self::Search,
        Self::Entry,
        Self::Review,
        Self::Repair,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Forms => "forms",
            Self::Settings => "settings",
            Self::Search => "search",
            Self::Entry => "entry",
            Self::Review => "review",
            Self::Repair => "repair",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a control must survive with the same truth, so a control's state, label, value, or
/// validation truth never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5CoreControlDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a control's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlConsumerSurface {
    /// The forms UI.
    FormsUi,
    /// The settings UI.
    SettingsUi,
    /// The search UI.
    SearchUi,
    /// The start-center entry UI.
    EntryUi,
    /// The review UI.
    ReviewUi,
    /// The repair UI.
    RepairUi,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5CoreControlConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::FormsUi,
        Self::SettingsUi,
        Self::SearchUi,
        Self::EntryUi,
        Self::ReviewUi,
        Self::RepairUi,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FormsUi => "forms_ui",
            Self::SettingsUi => "settings_ui",
            Self::SearchUi => "search_ui",
            Self::EntryUi => "entry_ui",
            Self::ReviewUi => "review_ui",
            Self::RepairUi => "repair_ui",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every control must offer so no action or value truth is hover-only,
/// pointer-only, motion-only, or visually encoded alone. Records the keyboard, screen-reader, high-zoom,
/// reduced-motion, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Legible and usable with reduced motion.
    ReducedMotionSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5CoreControlAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::ReducedMotionSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotionSafe => "reduced_motion_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a core control has degraded below its qualified state. Required on every row so a stale,
/// unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The label source is unavailable.
    LabelSourceUnavailable,
    /// The value source is unavailable.
    ValueSourceUnavailable,
    /// The validation signal is unavailable.
    ValidationSignalUnavailable,
    /// The command binding is unavailable.
    CommandBindingUnavailable,
    /// The state signal is unavailable.
    StateSignalUnavailable,
}

impl M5CoreControlDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::LabelSourceUnavailable,
        Self::ValueSourceUnavailable,
        Self::ValidationSignalUnavailable,
        Self::CommandBindingUnavailable,
        Self::StateSignalUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::LabelSourceUnavailable => "label_source_unavailable",
            Self::ValueSourceUnavailable => "value_source_unavailable",
            Self::ValidationSignalUnavailable => "validation_signal_unavailable",
            Self::CommandBindingUnavailable => "command_binding_unavailable",
            Self::StateSignalUnavailable => "state_signal_unavailable",
        }
    }
}

/// Mandatory label a claimed core control must be able to show. The first three are hard requirements on
/// every control; the remaining three close the acceptance-criteria ambiguity about command binding,
/// value source, and validation / constraint labeling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlRequiredLabel {
    /// The control's stable identity / permanent label.
    Identity,
    /// The control's current typed interaction state.
    State,
    /// The non-visual keyboard route to the control.
    KeyboardRoute,
    /// The command this control binds back to.
    CommandBinding,
    /// The source of the control's value.
    ValueSource,
    /// The validation and constraints behind the control.
    ValidationAndConstraints,
}

impl M5CoreControlRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CommandBinding,
        Self::ValueSource,
        Self::ValidationAndConstraints,
    ];

    /// The three labels every claimed control must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CommandBinding => "command_binding",
            Self::ValueSource => "value_source",
            Self::ValidationAndConstraints => "validation_and_constraints",
        }
    }
}

/// Qualification class for an M5 core-control row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlQualificationClass {
    /// Control qualifies for the Stable claim.
    Stable,
    /// Control is narrowed to Beta.
    Beta,
    /// Control is narrowed to Preview.
    Preview,
    /// Control is experimental and not claimed.
    Experimental,
    /// Control is unavailable on this build.
    Unavailable,
    /// Control is held pending upstream resolution.
    Held,
}

impl M5CoreControlQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the control may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a core control below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlDowngradeTrigger {
    /// Placeholder text was used as the only label.
    PlaceholderUsedAsLabel,
    /// A loading control relabeled the action or changed width enough to lose attribution.
    LoadingRelabeledOrResized,
    /// An icon-only destructive action was left unlabeled.
    IconOnlyDestructiveUnlabeled,
    /// A switch was blurred with a deferred checkbox.
    SwitchAndDeferredCheckboxBlurred,
    /// A split button defaulted to a riskier alternate.
    SplitDefaultedToRiskierAlternate,
    /// Locked or degraded semantics were hidden behind generic disabled chrome.
    LockedOrDegradedHiddenBehindDisabled,
    /// A control left its value source unstated.
    ValueSourceUnstated,
    /// A field left its validation state unstated.
    ValidationStateUnstated,
    /// A control left its command binding unstated.
    CommandBindingUnstated,
    /// A control drifted from the shared interaction-state taxonomy.
    StateTaxonomyDrifted,
    /// Generic chrome wording concealed control truth.
    GenericChromeWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5CoreControlDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PlaceholderUsedAsLabel,
        Self::LoadingRelabeledOrResized,
        Self::IconOnlyDestructiveUnlabeled,
        Self::SwitchAndDeferredCheckboxBlurred,
        Self::SplitDefaultedToRiskierAlternate,
        Self::LockedOrDegradedHiddenBehindDisabled,
        Self::ValueSourceUnstated,
        Self::ValidationStateUnstated,
        Self::CommandBindingUnstated,
        Self::StateTaxonomyDrifted,
        Self::GenericChromeWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaceholderUsedAsLabel => "placeholder_used_as_label",
            Self::LoadingRelabeledOrResized => "loading_relabeled_or_resized",
            Self::IconOnlyDestructiveUnlabeled => "icon_only_destructive_unlabeled",
            Self::SwitchAndDeferredCheckboxBlurred => "switch_and_deferred_checkbox_blurred",
            Self::SplitDefaultedToRiskierAlternate => "split_defaulted_to_riskier_alternate",
            Self::LockedOrDegradedHiddenBehindDisabled => {
                "locked_or_degraded_hidden_behind_disabled"
            }
            Self::ValueSourceUnstated => "value_source_unstated",
            Self::ValidationStateUnstated => "validation_state_unstated",
            Self::CommandBindingUnstated => "command_binding_unstated",
            Self::StateTaxonomyDrifted => "state_taxonomy_drifted",
            Self::GenericChromeWordingUsed => "generic_chrome_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed core-control family bound to the surface-specific truth it must
/// project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlComponentRow {
    /// Governed control family.
    pub component_family: M5CoreControlFamily,
    /// Qualification class earned by this control.
    pub qualification: M5CoreControlQualificationClass,
    /// Owner role accountable for keeping this control governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this control.
    pub surface_families: Vec<M5CoreControlSurfaceFamily>,
    /// Deployment lines this control keeps the same truth across.
    pub deployment_lines: Vec<M5CoreControlDeploymentLine>,
    /// Mandatory labels this control must be able to show (must include the three
    /// [`M5CoreControlRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5CoreControlRequiredLabel>,
    /// Interaction-state dispositions this control can carry (the frozen AC vocabulary; required on
    /// every control).
    pub dispositions: Vec<M5CoreControlDisposition>,
    /// Button emphases this control names (emphasis-bearing families only).
    pub button_emphases: Vec<M5ButtonEmphasis>,
    /// Icon-label modes this control names (icon-button family only).
    pub icon_label_modes: Vec<M5IconLabelMode>,
    /// Split-button default postures this control names (split-button family only).
    pub split_postures: Vec<M5SplitDefaultPosture>,
    /// Field label modes this control names (field-bearing families only).
    pub field_label_modes: Vec<M5FieldLabelMode>,
    /// Field validation states this control names (field-bearing families only).
    pub field_validations: Vec<M5FieldValidationState>,
    /// Search-field affordances this control names (search-field family only).
    pub search_affordances: Vec<M5SearchFieldAffordance>,
    /// Combobox value sources this control names (combobox family only).
    pub combobox_value_sources: Vec<M5ComboboxValueSource>,
    /// Toggle semantics this control names (toggle-control family only).
    pub toggle_semantics: Vec<M5ToggleSemantics>,
    /// Segmented modes this control names (segmented-control family only).
    pub segmented_modes: Vec<M5SegmentedMode>,
    /// Degraded reasons this control can name (required on every control).
    pub degraded_reasons: Vec<M5CoreControlDegradedReason>,
    /// Non-visual accessibility routes this control offers.
    pub accessibility_routes: Vec<M5CoreControlAccessibilityRoute>,
    /// Subsystems that consume this control's projection.
    pub consumer_surfaces: Vec<M5CoreControlConsumerSurface>,
    /// Downgrade triggers that apply to this control.
    pub downgrade_triggers: Vec<M5CoreControlDowngradeTrigger>,
    /// Proof packet refs that keep this control current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this control (must include its own canonical component schema so
    /// downstream controls have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this control never lets placeholder text replace the label. MUST be `false`.
    pub lets_placeholder_text_replace_the_label: bool,
    /// Hard invariant: this control never lets a loading state relabel the action or lose attribution.
    /// MUST be `false`.
    pub lets_loading_relabel_the_action_or_lose_attribution: bool,
    /// Hard invariant: this control never leaves an icon-only destructive action unlabeled. MUST be
    /// `false`.
    pub leaves_icon_only_destructive_action_unlabeled: bool,
    /// Hard invariant: this control never blurs a switch with a deferred checkbox. MUST be `false`.
    pub blurs_switch_with_deferred_checkbox: bool,
    /// Hard invariant: this control never lets a split button default to a riskier alternate. MUST be
    /// `false`.
    pub lets_split_button_default_to_riskier_alternate: bool,
    /// Hard invariant: this control never hides locked or degraded semantics behind generic disabled
    /// chrome. MUST be `false`.
    pub hides_locked_or_degraded_semantics_behind_generic_disabled: bool,
}

impl M5CoreControlComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CoreControlRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CoreControlRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_placeholder_text_replace_the_label
            && !self.lets_loading_relabel_the_action_or_lose_attribution
            && !self.leaves_icon_only_destructive_action_unlabeled
            && !self.blurs_switch_with_deferred_checkbox
            && !self.lets_split_button_default_to_riskier_alternate
            && !self.hides_locked_or_degraded_semantics_behind_generic_disabled
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlVocabularySet {
    /// Control-family tokens.
    pub component_families: Vec<String>,
    /// Interaction-state disposition tokens.
    pub dispositions: Vec<String>,
    /// Button-emphasis tokens.
    pub button_emphases: Vec<String>,
    /// Icon-label-mode tokens.
    pub icon_label_modes: Vec<String>,
    /// Split-default-posture tokens.
    pub split_postures: Vec<String>,
    /// Field-label-mode tokens.
    pub field_label_modes: Vec<String>,
    /// Field-validation-state tokens.
    pub field_validations: Vec<String>,
    /// Search-field-affordance tokens.
    pub search_affordances: Vec<String>,
    /// Combobox-value-source tokens.
    pub combobox_value_sources: Vec<String>,
    /// Toggle-semantics tokens.
    pub toggle_semantics: Vec<String>,
    /// Segmented-mode tokens.
    pub segmented_modes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5CoreControlVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5CoreControlFamily::ALL, |v| v.as_str()),
            dispositions: tokens(&M5CoreControlDisposition::ALL, |v| v.as_str()),
            button_emphases: tokens(&M5ButtonEmphasis::ALL, |v| v.as_str()),
            icon_label_modes: tokens(&M5IconLabelMode::ALL, |v| v.as_str()),
            split_postures: tokens(&M5SplitDefaultPosture::ALL, |v| v.as_str()),
            field_label_modes: tokens(&M5FieldLabelMode::ALL, |v| v.as_str()),
            field_validations: tokens(&M5FieldValidationState::ALL, |v| v.as_str()),
            search_affordances: tokens(&M5SearchFieldAffordance::ALL, |v| v.as_str()),
            combobox_value_sources: tokens(&M5ComboboxValueSource::ALL, |v| v.as_str()),
            toggle_semantics: tokens(&M5ToggleSemantics::ALL, |v| v.as_str()),
            segmented_modes: tokens(&M5SegmentedMode::ALL, |v| v.as_str()),
            surface_families: tokens(&M5CoreControlSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5CoreControlDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5CoreControlConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5CoreControlAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5CoreControlDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5CoreControlRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5CoreControlDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5CoreControlGovernanceReview {
    /// Button states stay semantically stable across surfaces.
    pub button_states_stay_semantically_stable: bool,
    /// Icon buttons never leave a destructive action unlabeled.
    pub icon_button_never_unlabeled_when_destructive: bool,
    /// Split-button default action stays the safe one.
    pub split_button_default_stays_safe: bool,
    /// Text-field labels are permanent, never placeholder-only.
    pub text_field_labels_are_permanent_not_placeholder_only: bool,
    /// Search fields preserve clear / submit / privacy truth.
    pub search_field_preserves_clear_submit_and_privacy_truth: bool,
    /// Comboboxes preserve filterability and value-source truth.
    pub combobox_preserves_filterability_and_value_source_truth: bool,
    /// Checkbox / radio / switch semantics stay distinct.
    pub toggle_control_semantics_stay_distinct: bool,
    /// Segmented controls stay mode toggles, never navigation.
    pub segmented_control_stays_mode_toggle_not_navigation: bool,
    /// The interaction-state taxonomy means the same thing everywhere.
    pub state_taxonomy_means_the_same_everywhere: bool,
    /// Loading never relabels the action or loses attribution.
    pub loading_never_relabels_or_loses_attribution: bool,
    /// Placeholder text never replaces a label.
    pub placeholder_never_replaces_label: bool,
    /// Locked and degraded semantics are never hidden behind generic disabled chrome.
    pub locked_and_degraded_never_hidden_behind_disabled: bool,
    /// Every control binds back to one command or value source.
    pub every_control_binds_to_one_command_or_value_source: bool,
    /// Every control keeps the same truth across every deployment line.
    pub every_control_declares_deployment_lines: bool,
    /// Every control declares a non-visual accessibility route.
    pub every_control_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel control vocabulary.
    pub later_rows_cannot_invent_parallel_control_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlConsumerProjection {
    /// Forms and settings consume the shared control vocabulary.
    pub forms_and_settings_consume_shared_control_vocabulary: bool,
    /// Search and entry consume the shared field vocabulary.
    pub search_and_entry_consume_shared_field_vocabulary: bool,
    /// Review consumes the shared action and value vocabulary.
    pub review_consumes_shared_action_and_value_vocabulary: bool,
    /// Repair consumes the shared control vocabulary.
    pub repair_consumes_shared_control_vocabulary: bool,
    /// Boolean controls consume the shared toggle vocabulary.
    pub boolean_controls_consume_shared_toggle_vocabulary: bool,
    /// Support / export reads a single canonical control source.
    pub support_export_reads_single_control_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the control.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the core-control lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting core-control audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every control.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every control.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CoreControlComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CoreControlComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Control rows.
    pub component_rows: Vec<M5CoreControlComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CoreControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CoreControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CoreControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CoreControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CoreControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 core-action-input component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CoreControlComponentMatrixPacket {
    /// Record kind; must equal [`M5_CORE_CONTROL_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CORE_CONTROL_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Control rows.
    pub component_rows: Vec<M5CoreControlComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CoreControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CoreControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CoreControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CoreControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CoreControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CoreControlComponentMatrixPacket {
    /// Builds an M5 core-action-input component matrix packet from stable-lane input.
    pub fn new(input: M5CoreControlComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CORE_CONTROL_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CORE_CONTROL_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 core-action-input component matrix invariants.
    pub fn validate(&self) -> Vec<M5CoreControlComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CORE_CONTROL_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5CoreControlComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CORE_CONTROL_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5CoreControlComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CoreControlComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 core-action-input component matrix serializes"),
        ) {
            violations.push(M5CoreControlComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 core-action-input component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Button, Icon-Button, Split-Button, Text-Field, Search-Field, Combobox, Toggle-Control, and Segmented-Control Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Control families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Interaction states: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Button emphases: {}\n",
            self.vocabulary_set.button_emphases.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Control families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 core-control matrix export.
#[derive(Debug)]
pub enum M5CoreControlComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CoreControlComponentMatrixViolation>),
}

impl fmt::Display for M5CoreControlComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 core-action-input component matrix export parse failed: {error}"
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
                    "m5 core-action-input component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CoreControlComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5CoreControlComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CoreControlComponentMatrixViolation {
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
    /// A required governed control family is missing from the matrix.
    RequiredComponentMissing,
    /// A control row is incomplete.
    ComponentRowIncomplete,
    /// A control row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A control row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A control declares no interaction-state dispositions.
    DispositionMissing,
    /// An emphasis-bearing control declares no button emphases.
    ButtonEmphasisMissing,
    /// The icon-button control declares no icon-label modes.
    IconLabelModeMissing,
    /// The split-button control declares no split-default postures.
    SplitPostureMissing,
    /// A field-bearing control declares no field label modes.
    FieldLabelModeMissing,
    /// A field-bearing control declares no field validation states.
    FieldValidationMissing,
    /// The search-field control declares no search affordances.
    SearchAffordanceMissing,
    /// The combobox control declares no value sources.
    ComboboxValueSourceMissing,
    /// The toggle control declares no toggle semantics.
    ToggleSemanticsMissing,
    /// The segmented control declares no segmented modes.
    SegmentedModeMissing,
    /// A control declares no degraded reasons.
    DegradedReasonMissing,
    /// A control declares no surface families.
    SurfaceFamilyMissing,
    /// A control declares no deployment lines.
    DeploymentLineMissing,
    /// A control declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A control declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A control declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A control claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A control violates a hard invariant (placeholder-as-label, loading relabels or loses
    /// attribution, icon-only destructive unlabeled, switch blurred with deferred checkbox, split
    /// defaults to a riskier alternate, or locked/degraded hidden behind disabled).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CoreControlComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::DispositionMissing => "disposition_missing",
            Self::ButtonEmphasisMissing => "button_emphasis_missing",
            Self::IconLabelModeMissing => "icon_label_mode_missing",
            Self::SplitPostureMissing => "split_posture_missing",
            Self::FieldLabelModeMissing => "field_label_mode_missing",
            Self::FieldValidationMissing => "field_validation_missing",
            Self::SearchAffordanceMissing => "search_affordance_missing",
            Self::ComboboxValueSourceMissing => "combobox_value_source_missing",
            Self::ToggleSemanticsMissing => "toggle_semantics_missing",
            Self::SegmentedModeMissing => "segmented_mode_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 core-control matrix export.
pub fn current_stable_m5_core_action_input_component_matrix_export(
) -> Result<M5CoreControlComponentMatrixPacket, M5CoreControlComponentMatrixArtifactError> {
    let packet: M5CoreControlComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-core-action-input-proof/support_export.json"
    )))
    .map_err(M5CoreControlComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CoreControlComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
        M5_CORE_CONTROL_COMPONENT_DOC_REF,
        M5_BUTTON_SCHEMA_REF,
        M5_ICON_BUTTON_SCHEMA_REF,
        M5_SPLIT_BUTTON_SCHEMA_REF,
        M5_TEXT_FIELD_SCHEMA_REF,
        M5_SEARCH_FIELD_SCHEMA_REF,
        M5_COMBOBOX_SCHEMA_REF,
        M5_TOGGLE_CONTROL_SCHEMA_REF,
        M5_SEGMENTED_CONTROL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CoreControlComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CoreControlComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    let present: BTreeSet<M5CoreControlFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5CoreControlFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5CoreControlComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5CoreControlComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5CoreControlComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5CoreControlComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_button_emphasis() && row.button_emphases.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::ButtonEmphasisMissing);
        }
        if family.declares_icon_label_mode() && row.icon_label_modes.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::IconLabelModeMissing);
        }
        if family.declares_split_posture() && row.split_postures.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::SplitPostureMissing);
        }
        if family.declares_field_label_mode() && row.field_label_modes.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::FieldLabelModeMissing);
        }
        if family.declares_field_validation() && row.field_validations.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::FieldValidationMissing);
        }
        if family.declares_search_affordance() && row.search_affordances.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::SearchAffordanceMissing);
        }
        if family.declares_combobox_value_source() && row.combobox_value_sources.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::ComboboxValueSourceMissing);
        }
        if family.declares_toggle_semantics() && row.toggle_semantics.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::ToggleSemanticsMissing);
        }
        if family.declares_segmented_mode() && row.segmented_modes.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::SegmentedModeMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CoreControlComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CoreControlComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.button_states_stay_semantically_stable,
        review.icon_button_never_unlabeled_when_destructive,
        review.split_button_default_stays_safe,
        review.text_field_labels_are_permanent_not_placeholder_only,
        review.search_field_preserves_clear_submit_and_privacy_truth,
        review.combobox_preserves_filterability_and_value_source_truth,
        review.toggle_control_semantics_stay_distinct,
        review.segmented_control_stays_mode_toggle_not_navigation,
        review.state_taxonomy_means_the_same_everywhere,
        review.loading_never_relabels_or_loses_attribution,
        review.placeholder_never_replaces_label,
        review.locked_and_degraded_never_hidden_behind_disabled,
        review.every_control_binds_to_one_command_or_value_source,
        review.every_control_declares_deployment_lines,
        review.every_control_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_control_vocabulary,
    ] {
        if !ok {
            violations.push(M5CoreControlComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.forms_and_settings_consume_shared_control_vocabulary,
        projection.search_and_entry_consume_shared_field_vocabulary,
        projection.review_consumes_shared_action_and_value_vocabulary,
        projection.repair_consumes_shared_control_vocabulary,
        projection.boolean_controls_consume_shared_toggle_vocabulary,
        projection.support_export_reads_single_control_source,
    ] {
        if !ok {
            violations.push(M5CoreControlComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CoreControlComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CoreControlComponentMatrixPacket,
    violations: &mut Vec<M5CoreControlComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CoreControlComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses forms / control words; what is rejected is a raw secret *value* shape —
/// a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
