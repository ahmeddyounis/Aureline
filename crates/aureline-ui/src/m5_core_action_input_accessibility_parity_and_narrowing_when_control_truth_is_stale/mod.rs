//! Keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity, and honest automatic
//! claim narrowing for the M5 button / icon-button / split-button / text-field / search-field / combobox
//! / checkbox-radio-switch / segmented-control core action and input controls.
//!
//! This module is the M05-1130 accessibility-and-auto-narrowing capstone over the frozen M5
//! core-action-input component matrix ([`crate::m5_core_action_input_component_matrix`]). Where the freeze
//! matrix defines the reusable button, icon button, split button, text field, search field, combobox,
//! toggle control, and segmented control primitives, and the 1125-1128 implementation lanes resolve their
//! per-surface truth, this lane certifies — per control family — that action / input control claims stay
//! **keyboard-complete, assistive-tech-reachable, high-zoom / reduced-motion-safe, CLI/export-safe, and
//! self-narrowing** rather than presenting an unbound command, an unnamed icon action, a riskier split
//! default, a stale validation anchor, an unstated value source, or a blurred immediate/deferred toggle
//! semantic as still a trusted, ready-to-operate control:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same control identity, interaction state / disposition, command
//!   binding, accessible name, value source, validation anchor, and immediate-versus-deferred toggle
//!   semantics the rich control shows — never a placeholder-as-label, a hover-only affordance, a
//!   color-only emphasis, or a motion-only cue that strands assistive-tech or headless-CLI users.
//!   Structure-heavy families (the split button's alternate menu, the combobox's option list, the
//!   segmented control's segments) additionally bind their structured layout to a flat list / textual
//!   path.
//! - **Export parity.** The support / release / CLI export reconstructs each control's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same identity, state, command
//!   binding, accessible name, value source, validation, and toggle semantics shown in-product so support,
//!   help, and release proof can reconstruct exactly what the user was actually shown without leaking a
//!   raw field value, secret, endpoint, or option payload.
//! - **Honest auto-narrowing.** When a command binding is stale / missing, an icon-only control has no
//!   accessible name, a split button's safe default cannot be confirmed, a validation anchor is stale, an
//!   immediate/deferred toggle semantic is unverified, or a search field can only disclose a partial
//!   retention / privacy posture, the control's claim auto-narrows from `trusted_control` /
//!   `reviewable_control` to a command-binding-unverified / accessible-name-unverified /
//!   default-safety-unverified / validation-unverified / toggle-semantics-unverified / retention-disclosed
//!   projection, discloses the narrowing with a precise trigger and binding dimension, and preserves the
//!   canonical control identity / last-known state. The underlying command / value / validation truth is
//!   never dropped opaquely. A control with every dimension intact must NOT carry a spurious narrowing, and
//!   an unbound-command / unnamed-icon / riskier-split-default / stale-validation / blurred-toggle state
//!   can never keep a trusted, ready-to-operate claim — a loading button never relabels its action, and a
//!   riskier split alternate never quietly becomes the default.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the forms UI, the settings UI, the
//!   search UI, the entry UI, the review UI, the repair UI, the CLI export, the support export, and the
//!   product UI so product, help, and release publication stay aligned on downgrade behavior rather than
//!   drifting in copy — a trusted-looking control can never outrun the command / value / validation
//!   evidence it is being viewed away from.
//!
//! Each [`CoreControlAccessibilityRow`] keys on one
//! [`crate::m5_core_action_input_component_matrix::M5CoreControlFamily`] and reuses that frozen family
//! vocabulary plus the frozen [`M5CoreControlRequiredLabel`], [`M5CoreControlDowngradeTrigger`], and
//! shared [`M5CoreControlConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw field values, option payloads, credentials, secrets, and endpoint
//! refs never cross this boundary; the packet carries only typed class tokens, opaque control refs,
//! booleans, and controlled labels so support, release, and diagnostics exports can reconstruct exactly
//! what an accessible fallback would have shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_core_action_input_component_matrix::{
    M5CoreControlConsumerSurface, M5CoreControlDowngradeTrigger, M5CoreControlFamily,
    M5CoreControlRequiredLabel, M5_CORE_CONTROL_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1130 core-action-input component accessibility parity packet.
pub const CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CoreControlAccessibilityPacket`].
pub const CORE_ACTION_INPUT_A11Y_RECORD_KIND: &str =
    "m5_core_action_input_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`CoreControlAccessibilityRow`].
pub const CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND: &str =
    "m5_core_action_input_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const CORE_ACTION_INPUT_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-core-action-input-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const CORE_ACTION_INPUT_A11Y_DOC_REF: &str =
    "docs/components/m5_core_action_input_component_accessibility_parity.md";

/// Repo-relative path of the frozen core-action-input component matrix this lane certifies.
pub const CORE_ACTION_INPUT_A11Y_COMPONENT_MATRIX_REF: &str = M5_CORE_CONTROL_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const CORE_ACTION_INPUT_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-core-action-input-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CORE_ACTION_INPUT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-core-action-input-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CORE_ACTION_INPUT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-core-action-input-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CORE_ACTION_INPUT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-core-action-input-component-accessibility-parity.md";

/// The reusable control families that render a dense, structured surface (the split button's alternate
/// menu, the combobox's option list, the segmented control's segments) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual path so the structure is navigable non-visually.
const fn family_is_structure_heavy(family: M5CoreControlFamily) -> bool {
    matches!(
        family,
        M5CoreControlFamily::SplitButton
            | M5CoreControlFamily::Combobox
            | M5CoreControlFamily::SegmentedControl
    )
}

/// The control-truth dimension whose weakening a family primarily discloses. Every row must model at least
/// this dimension so its key weakening axis is covered.
const fn family_primary_dimension(family: M5CoreControlFamily) -> M5CoreControlClaimDimension {
    match family {
        M5CoreControlFamily::Button => M5CoreControlClaimDimension::CommandBindingClarity,
        M5CoreControlFamily::IconButton => M5CoreControlClaimDimension::AccessibleNameClarity,
        M5CoreControlFamily::SplitButton => M5CoreControlClaimDimension::DefaultSafetyClarity,
        M5CoreControlFamily::TextField => M5CoreControlClaimDimension::LabelValidationClarity,
        M5CoreControlFamily::SearchField => M5CoreControlClaimDimension::ClearSubmitPrivacyClarity,
        M5CoreControlFamily::Combobox => M5CoreControlClaimDimension::ValueSourceClarity,
        M5CoreControlFamily::ToggleControl => M5CoreControlClaimDimension::ToggleSemanticsClarity,
        M5CoreControlFamily::SegmentedControl => M5CoreControlClaimDimension::SelectedModeClarity,
    }
}

/// A rendered fallback modality for a core-action-input control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlFallbackModality {
    /// A rich, structured (alternate menu / option list / segments) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5CoreControlFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same control may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs
/// export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5CoreControlRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach for a control's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl CoreControlNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the control meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlExportSummaryState {
    /// The control meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl CoreControlExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl CoreControlNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The control claim ceiling a family asserts: how strong a trusted / ready-to-operate posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a command / accessibility / value / validation
/// / toggle dimension weakens so an unbound command, an unnamed icon action, a riskier split default, a
/// stale validation anchor, an unverified toggle semantic, or a partial retention disclosure can never
/// keep an old `TrustedControl` or `ReviewableControl` label — a loading button never relabels its action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlClaim {
    /// Trusted control: a fully current, command-bound, accessibly-named, value-source-clear,
    /// validation-anchored, toggle-semantics-clear control — the strongest claim, a control Aureline can
    /// present as exactly trusted and ready to operate right now.
    TrustedControl,
    /// Reviewable control: a self-sufficient, reviewable read-only control (a combobox / segmented control
    /// a user can inspect) that is not itself an authoritative, mutation-ready surface.
    ReviewableControl,
    /// Command-binding-unverified projection: the command binding is stale / missing; the control stays a
    /// command-binding-unverified projection with its last-known identity preserved, never a
    /// freshly-bound, ready-to-invoke action.
    CommandBindingUnverifiedProjection,
    /// Accessible-name-unverified projection: an icon-only control has no confirmed accessible name; the
    /// control stays an accessible-name-unverified projection with its last-known glyph / action
    /// preserved, never an unlabeled destructive action shown as safe.
    AccessibleNameUnverifiedProjection,
    /// Default-safety-unverified projection: a split button's safe default cannot be confirmed; the control
    /// stays a default-safety-unverified projection that keeps the safe default explicit, never letting a
    /// riskier alternate become the default.
    DefaultSafetyUnverifiedProjection,
    /// Validation-unverified projection: a field's validation anchor is stale; the control stays a
    /// validation-unverified projection that discloses the last-known validation state, never a
    /// freshly-validated field.
    ValidationUnverifiedProjection,
    /// Toggle-semantics-unverified projection: an immediate-versus-deferred toggle semantic is unverified;
    /// the control stays a toggle-semantics-unverified projection that keeps the last-known toggle
    /// semantics, never blurring a switch with a deferred checkbox.
    ToggleSemanticsUnverifiedProjection,
    /// Retention-disclosed projection: a search field can only disclose a partial / redacted retention /
    /// privacy posture; the control stays a retention-disclosed projection that discloses the partial
    /// retention posture, never a fully-private, no-retention field.
    RetentionDisclosedProjection,
}

impl M5CoreControlClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedControl,
        Self::ReviewableControl,
        Self::CommandBindingUnverifiedProjection,
        Self::AccessibleNameUnverifiedProjection,
        Self::DefaultSafetyUnverifiedProjection,
        Self::ValidationUnverifiedProjection,
        Self::ToggleSemanticsUnverifiedProjection,
        Self::RetentionDisclosedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedControl => 7,
            Self::ReviewableControl => 6,
            Self::CommandBindingUnverifiedProjection => 5,
            Self::AccessibleNameUnverifiedProjection => 4,
            Self::DefaultSafetyUnverifiedProjection => 3,
            Self::ValidationUnverifiedProjection => 2,
            Self::ToggleSemanticsUnverifiedProjection => 1,
            Self::RetentionDisclosedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, ready-to-operate control.
    pub const fn asserts_trusted_control(self) -> bool {
        matches!(self, Self::TrustedControl)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) control.
    pub const fn asserts_self_sufficient_control(self) -> bool {
        matches!(self, Self::TrustedControl | Self::ReviewableControl)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedControl => "trusted_control",
            Self::ReviewableControl => "reviewable_control",
            Self::CommandBindingUnverifiedProjection => "command_binding_unverified_projection",
            Self::AccessibleNameUnverifiedProjection => "accessible_name_unverified_projection",
            Self::DefaultSafetyUnverifiedProjection => "default_safety_unverified_projection",
            Self::ValidationUnverifiedProjection => "validation_unverified_projection",
            Self::ToggleSemanticsUnverifiedProjection => "toggle_semantics_unverified_projection",
            Self::RetentionDisclosedProjection => "retention_disclosed_projection",
        }
    }
}

/// The command / accessibility / value / validation / toggle dimension whose state governs how far a
/// control may claim to be a fully trusted, ready-to-operate surface. The dimensions map 1:1 to the eight
/// frozen control families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlClaimDimension {
    /// Command-binding clarity: is the button's action bound to one canonical command without losing
    /// attribution when loading (button)?
    CommandBindingClarity,
    /// Accessible-name clarity: does the icon-only control carry a confirmed accessible name, especially
    /// for a destructive action (icon button)?
    AccessibleNameClarity,
    /// Default-safety clarity: does the split button keep the safe action as the default rather than a
    /// riskier alternate (split button)?
    DefaultSafetyClarity,
    /// Label / validation clarity: does the text field carry a permanent label and a legible validation
    /// anchor rather than a placeholder-as-label (text field)?
    LabelValidationClarity,
    /// Clear / submit / privacy clarity: does the search field preserve clear / submit / privacy /
    /// retention truth (search field)?
    ClearSubmitPrivacyClarity,
    /// Value-source clarity: does the combobox preserve filterability and source-of-value truth
    /// (combobox)?
    ValueSourceClarity,
    /// Toggle-semantics clarity: is the checkbox / radio / switch immediate-versus-deferred semantic
    /// distinct (toggle control)?
    ToggleSemanticsClarity,
    /// Selected-mode clarity: does the segmented control stay a small mode / view toggle with an explicit
    /// selected mode rather than stealth navigation (segmented control)?
    SelectedModeClarity,
}

impl M5CoreControlClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CommandBindingClarity,
        Self::AccessibleNameClarity,
        Self::DefaultSafetyClarity,
        Self::LabelValidationClarity,
        Self::ClearSubmitPrivacyClarity,
        Self::ValueSourceClarity,
        Self::ToggleSemanticsClarity,
        Self::SelectedModeClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandBindingClarity => "command_binding_clarity",
            Self::AccessibleNameClarity => "accessible_name_clarity",
            Self::DefaultSafetyClarity => "default_safety_clarity",
            Self::LabelValidationClarity => "label_validation_clarity",
            Self::ClearSubmitPrivacyClarity => "clear_submit_privacy_clarity",
            Self::ValueSourceClarity => "value_source_clarity",
            Self::ToggleSemanticsClarity => "toggle_semantics_clarity",
            Self::SelectedModeClarity => "selected_mode_clarity",
        }
    }
}

/// The observed condition of one control-truth dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the control's claim. The stale / missing / unverified states the lane
/// must auto-narrow on as *weakened evidence* — a stale command binding, a missing accessible name, an
/// unconfirmed safe default, a stale validation anchor, and an unverified toggle semantic — are the states
/// that [`Self::cannot_be_shown_trusted`] flags. A partial retention disclosure is an honest
/// disclosed-absence operation (a partial / redacted retention posture shown honestly with an inspectable
/// privacy note), not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlConditionState {
    /// Fully current, command-bound, accessibly-named, value-source-clear, validation-anchored,
    /// toggle-semantics-clear — imposes no ceiling.
    FullyQualified,
    /// The command binding is stale / missing — claim drops to a command-binding-unverified projection.
    CommandBindingStale,
    /// The icon-only control has no confirmed accessible name — claim drops to an
    /// accessible-name-unverified projection.
    AccessibleNameMissing,
    /// The split button's safe default cannot be confirmed — claim drops to a default-safety-unverified
    /// projection.
    DefaultSafetyStale,
    /// The field's validation anchor is stale — claim drops to a validation-unverified projection.
    ValidationAnchorStale,
    /// The immediate-versus-deferred toggle semantic is unverified — claim drops to a
    /// toggle-semantics-unverified projection.
    ToggleSemanticsUnverified,
    /// The search field can only disclose a partial / redacted retention / privacy posture — claim drops
    /// to a retention-disclosed projection.
    RetentionDisclosedPartial,
}

impl M5CoreControlConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::CommandBindingStale,
        Self::AccessibleNameMissing,
        Self::DefaultSafetyStale,
        Self::ValidationAnchorStale,
        Self::ToggleSemanticsUnverified,
        Self::RetentionDisclosedPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// ready-to-operate control and must never be shown as such. A partial retention disclosure is an
    /// honest disclosed-absence operation (a partial / redacted retention posture shown honestly with an
    /// inspectable privacy note), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::CommandBindingStale
                | Self::AccessibleNameMissing
                | Self::DefaultSafetyStale
                | Self::ValidationAnchorStale
                | Self::ToggleSemanticsUnverified
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5CoreControlClaim {
        match self {
            Self::FullyQualified => M5CoreControlClaim::TrustedControl,
            Self::CommandBindingStale => M5CoreControlClaim::CommandBindingUnverifiedProjection,
            Self::AccessibleNameMissing => M5CoreControlClaim::AccessibleNameUnverifiedProjection,
            Self::DefaultSafetyStale => M5CoreControlClaim::DefaultSafetyUnverifiedProjection,
            Self::ValidationAnchorStale => M5CoreControlClaim::ValidationUnverifiedProjection,
            Self::ToggleSemanticsUnverified => {
                M5CoreControlClaim::ToggleSemanticsUnverifiedProjection
            }
            Self::RetentionDisclosedPartial => M5CoreControlClaim::RetentionDisclosedProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5CoreControlDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5CoreControlDowngradeTrigger::ProofStale,
            Self::CommandBindingStale => M5CoreControlDowngradeTrigger::CommandBindingUnstated,
            Self::AccessibleNameMissing => {
                M5CoreControlDowngradeTrigger::IconOnlyDestructiveUnlabeled
            }
            Self::DefaultSafetyStale => {
                M5CoreControlDowngradeTrigger::SplitDefaultedToRiskierAlternate
            }
            Self::ValidationAnchorStale => M5CoreControlDowngradeTrigger::ValidationStateUnstated,
            Self::ToggleSemanticsUnverified => {
                M5CoreControlDowngradeTrigger::SwitchAndDeferredCheckboxBlurred
            }
            Self::RetentionDisclosedPartial => M5CoreControlDowngradeTrigger::ProofStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::CommandBindingStale => "command_binding_stale",
            Self::AccessibleNameMissing => "accessible_name_missing",
            Self::DefaultSafetyStale => "default_safety_stale",
            Self::ValidationAnchorStale => "validation_anchor_stale",
            Self::ToggleSemanticsUnverified => "toggle_semantics_unverified",
            Self::RetentionDisclosedPartial => "retention_disclosed_partial",
        }
    }
}

/// One control-truth dimension's observed condition on a control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5CoreControlClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5CoreControlConditionState,
}

/// An honest claim auto-narrow block. When a control-truth dimension weakens, the control's claim lowers
/// to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// control identity / last-known state rather than silently dropping it — the underlying command / value /
/// validation truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlClaimAutoNarrow {
    /// The claim the control is narrowed to.
    pub narrowed_to: M5CoreControlClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5CoreControlClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5CoreControlDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical control identity and last-known state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying command / value / validation truth is preserved (never dropped) across the
    /// narrowing; must hold so command-binding-unverified, accessible-name-unverified,
    /// default-safety-unverified, validation-unverified, toggle-semantics-unverified, and
    /// retention-disclosed states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl CoreControlClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and command / value /
    /// validation truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a control's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl CoreControlCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5CoreControlRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: CoreControlNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a core-control accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl CoreControlAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one core-control family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlAccessibilityRow {
    /// Record kind; must equal [`CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen control family this row certifies.
    pub component_family: M5CoreControlFamily,
    /// Ref to the frozen per-component schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the control this row represents; stays visible on every surface, so this is never
    /// empty.
    pub component_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5CoreControlFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, state, command binding, accessible
    /// name, value source, validation, and toggle semantics as the rich control; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: CoreControlNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: CoreControlNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: CoreControlNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: CoreControlNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: CoreControlNonVisualReachState,
    /// Whether the export-safe summary preserves control meaning.
    pub export_summary: CoreControlExportSummaryState,
    /// Ref to the export-safe summary object for this control.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: CoreControlCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5CoreControlClaim,
    /// The observed condition of each modeled control-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<CoreControlClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full
    /// claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<CoreControlClaimAutoNarrow>,
    /// Whether the underlying command / value / validation truth is preserved on this control regardless of
    /// narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this control is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5CoreControlRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<CoreControlRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5CoreControlRequiredLabel>,
    /// Semantic consumer surfaces this control is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5CoreControlConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl CoreControlAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5CoreControlClaimDimension,
    ) -> M5CoreControlConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5CoreControlConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5CoreControlClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_condition(&self) -> Option<&CoreControlClaimConditionEntry> {
        let mut binding: Option<(&CoreControlClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5CoreControlClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this control effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5CoreControlClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: an unbound command, an unnamed icon action, a riskier split default, a
    /// stale validation anchor, an unverified toggle semantic, or a partial retention disclosure can no
    /// longer keep an old `TrustedControl` / `ReviewableControl` label. The effective claim never exceeds
    /// the permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is
    /// present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with its
    /// frozen trigger, and preserves canonical identity and truth. When nothing narrows, no spurious narrow
    /// block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: an unbound-command / unnamed-icon / riskier-split-default / stale-validation /
    /// blurred-toggle state never keeps a trusted claim — a loading button never relabels its action. When
    /// such a state is modeled, the effective claim must not assert `TrustedControl`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_control())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a structure-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.component_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the control meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying command / value / validation
    /// truth. The row must assert `truth_preserved`, and any narrow block must preserve truth continuity
    /// too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the control carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, so product / help / release publication stay aligned on the same narrowed
    /// state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5CoreControlRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> CoreControlAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return CoreControlAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            CoreControlAccessibilityStatus::NarrowedDisclosed
        } else {
            CoreControlAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND
            && self.schema_version == CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.component_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1130 core-control accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub structure_heavy_family_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`CoreControlAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreControlAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<CoreControlAccessibilityRow>,
}

/// Checked-in M05-1130 core-control accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<CoreControlAccessibilityRow>,
    pub summary: CoreControlAccessibilitySummary,
}

impl CoreControlAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CoreControlAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            record_kind: CORE_ACTION_INPUT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: CoreControlAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                structure_heavy_family_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5CoreControlFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5CoreControlClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5CoreControlConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5CoreControlClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5CoreControlConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CoreControlAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5CoreControlConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&CoreControlAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                CoreControlAccessibilityStatus::Parity => green += 1,
                CoreControlAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                CoreControlAccessibilityStatus::Stranded => red += 1,
            }
        }

        CoreControlAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(CoreControlAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(CoreControlAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(CoreControlAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(CoreControlAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(CoreControlAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(CoreControlAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CoreControlAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION {
            violations.push(CoreControlAccessibilityViolation::SchemaVersion {
                expected: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CORE_ACTION_INPUT_A11Y_RECORD_KIND {
            violations.push(CoreControlAccessibilityViolation::RecordKind {
                expected: CORE_ACTION_INPUT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CoreControlAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CoreControlAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(CoreControlAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(CoreControlAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // Each row must preserve every mandatory control label.
            if !row.preserves_mandatory_labels() {
                violations.push(CoreControlAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5CoreControlFallbackModality::Structured)
            {
                violations.push(
                    CoreControlAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(CoreControlAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: an unbound-command / unnamed-icon / riskier-split-default /
            // stale-validation / blurred-toggle state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(CoreControlAccessibilityViolation::WeakStateShownAsTrusted {
                    id: row.row_id.clone(),
                });
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(CoreControlAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    CoreControlAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve command / value / validation truth.
            if !row.preserves_truth_continuity() {
                violations.push(CoreControlAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    CoreControlAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(CoreControlAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == CoreControlAccessibilityStatus::Stranded {
                violations.push(CoreControlAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5CoreControlFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(CoreControlAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5CoreControlClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    CoreControlAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5CoreControlConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    CoreControlAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → retention-disclosed) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5CoreControlClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(CoreControlAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one unbound-command / unnamed-icon /
        // riskier-split-default / stale-validation / blurred-toggle row in the packet, so the "cannot-prove
        // never shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(CoreControlAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the forms, settings, search, entry, review,
        // repair, CLI-export, support-export, and product surfaces — so every consumer surface is exercised
        // at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5CoreControlConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    CoreControlAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(CoreControlAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("core-action-input accessibility parity packet serializes"),
        ) {
            violations.push(CoreControlAccessibilityViolation::RawControlMaterialInExport);
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
            .expect("core-action-input accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Core-Action-Input-Control Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5CoreControlFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in core-control accessibility parity export.
pub fn current_m5_core_action_input_component_a11y_export(
) -> Result<CoreControlAccessibilityPacket, CoreControlAccessibilityArtifactError> {
    let packet: CoreControlAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-core-action-input-component-accessibility-parity/support_export.json"
    )))
    .map_err(CoreControlAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CoreControlAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in core-control accessibility parity export.
#[derive(Debug)]
pub enum CoreControlAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CoreControlAccessibilityViolation>),
}

impl fmt::Display for CoreControlAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "core-action-input accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "core-action-input accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for CoreControlAccessibilityArtifactError {}

/// Validation failure for M05-1130 core-control accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreControlAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5CoreControlClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5CoreControlFamily,
    },
    MissingDimensionCoverage {
        dimension: M5CoreControlClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5CoreControlConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5CoreControlClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5CoreControlConsumerSurface,
    },
    SummaryMismatch,
    RawControlMaterialInExport,
}

impl CoreControlAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawControlMaterialInExport => "raw_control_material_in_export",
        }
    }
}

impl fmt::Display for CoreControlAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory control label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable control for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows an unbound-command / unnamed-icon / riskier-split-default / stale-validation / blurred-toggle state as a trusted control"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / reduced-motion / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve command / value / validation truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "control family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no unbound-command / unnamed-icon / riskier-split-default / stale-validation / blurred-toggle row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawControlMaterialInExport => {
                write!(f, "export contains raw control material")
            }
        }
    }
}

impl Error for CoreControlAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const CORE_ACTION_INPUT_A11Y_PACKET_ID: &str =
    "m5-core-action-input-component-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in core-control accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_core_action_input_component_a11y_packet() -> CoreControlAccessibilityPacket {
    CoreControlAccessibilityPacket::new(CoreControlAccessibilityPacketInput {
        packet_id: CORE_ACTION_INPUT_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-12T00:00:00Z".to_owned(),
        matrix_ref: CORE_ACTION_INPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:core-action-input-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5CoreControlRequiredLabel> {
    M5CoreControlRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CoreControlCopyExportParity {
    CoreControlCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5CoreControlClaimDimension,
    state: M5CoreControlConditionState,
) -> CoreControlClaimConditionEntry {
    CoreControlClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5CoreControlConsumerSurface]) -> Vec<M5CoreControlConsumerSurface> {
    let mut out = vec![
        M5CoreControlConsumerSurface::SupportExport,
        M5CoreControlConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: CoreControlNarrowingDisclosureState,
) -> Vec<CoreControlRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        CoreControlRenderingNarrowingDisclosure {
            rendering_surface: M5CoreControlRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        CoreControlRenderingNarrowingDisclosure {
            rendering_surface: M5CoreControlRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_hover_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<CoreControlRenderingNarrowingDisclosure> {
    surface_disclosures(labels, CoreControlNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<CoreControlRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        CoreControlNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5CoreControlRenderingSurface> {
    vec![
        M5CoreControlRenderingSurface::DesktopFull,
        M5CoreControlRenderingSurface::CliHeadless,
        M5CoreControlRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5CoreControlFallbackModality> {
    vec![
        M5CoreControlFallbackModality::List,
        M5CoreControlFallbackModality::Textual,
        M5CoreControlFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5CoreControlFallbackModality> {
    vec![
        M5CoreControlFallbackModality::Structured,
        M5CoreControlFallbackModality::List,
        M5CoreControlFallbackModality::Textual,
        M5CoreControlFallbackModality::Cli,
    ]
}

const REACHABLE: CoreControlNonVisualReachState =
    CoreControlNonVisualReachState::ReachableAndLabeled;
const REDUCED: CoreControlNonVisualReachState =
    CoreControlNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<CoreControlAccessibilityRow> {
    vec![
        // Segmented control (selected-mode fully stated) — the small mode / view toggle keeps its selected
        // mode explicit and stays a mode toggle rather than stealth navigation, so it is a trusted control
        // reachable on every surface with no narrowing (green). Structure-heavy: its segments bind to a
        // flat list / textual path.
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:segmented-control-selected-mode-stated".to_owned(),
            component_family: M5CoreControlFamily::SegmentedControl,
            source_family_schema_ref: M5CoreControlFamily::SegmentedControl
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "entry:segmented-control:0001".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:segmented-control-selected-mode-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "selected_mode",
                "available_modes",
                "keyboard_route",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::SelectedModeClarity,
                M5CoreControlConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "control_identity",
                "selected_mode",
                "available_modes",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::EntryUi,
                M5CoreControlConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.7 — Segmented control".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("segmented-control-selected-mode-stated"),
        },
        // Combobox (value source fully stated) — structure-heavy (a filterable option list); the
        // source-of-value and filterability are fully stated, so it is a reviewable control that binds its
        // option list to a flat list / textual path, but its dense list narrows the screen-reader traversal
        // to a disclosed linear walk (yellow).
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:combobox-value-source-stated".to_owned(),
            component_family: M5CoreControlFamily::Combobox,
            source_family_schema_ref: M5CoreControlFamily::Combobox
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "entry:combobox:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:combobox-value-source-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "value_source",
                "selected_value",
                "filterability",
            ]),
            full_ready_claim: M5CoreControlClaim::ReviewableControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::ValueSourceClarity,
                M5CoreControlConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "value_source",
                "selected_value",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::EntryUi,
                M5CoreControlConsumerSurface::FormsUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.1 — Combobox".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("combobox-value-source-stated"),
        },
        // Button (command binding stale) — the button's command binding is stale / missing, so it
        // auto-narrows to a command-binding-unverified projection that keeps the last-known action identity
        // and label visible without relabeling on loading, never a freshly-bound, ready-to-invoke action
        // (yellow).
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:button-command-binding-stale".to_owned(),
            component_family: M5CoreControlFamily::Button,
            source_family_schema_ref: M5CoreControlFamily::Button
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "forms:button:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:button-command-binding-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "action_label",
                "command_binding",
                "last_known_binding",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::CommandBindingClarity,
                M5CoreControlConditionState::CommandBindingStale,
            )],
            claim_narrow: Some(CoreControlClaimAutoNarrow {
                narrowed_to: M5CoreControlClaim::CommandBindingUnverifiedProjection,
                binding_dimension: M5CoreControlClaimDimension::CommandBindingClarity,
                trigger: M5CoreControlDowngradeTrigger::CommandBindingUnstated,
                narrowed_label:
                    "This button's command binding is stale or unresolved — shown as a command-binding-unverified projection that keeps the last-known action identity and permanent label visible without relabeling on loading, never presenting an unbound button as a freshly-bound, ready-to-invoke action"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "action_label",
                "command_binding",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::FormsUi,
                M5CoreControlConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UX Style Guide §15.1 — Button".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("button-command-binding-stale"),
        },
        // Icon button (accessible name missing) — an icon-only control has no confirmed accessible name, so
        // it auto-narrows to an accessible-name-unverified projection that keeps the last-known glyph and
        // action visible and never leaves an icon-only destructive action unlabeled (yellow).
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:icon-button-accessible-name-missing".to_owned(),
            component_family: M5CoreControlFamily::IconButton,
            source_family_schema_ref: M5CoreControlFamily::IconButton
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "review:icon-button:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:icon-button-accessible-name-missing:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "glyph_role",
                "accessible_name",
                "destructive_state",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::AccessibleNameClarity,
                M5CoreControlConditionState::AccessibleNameMissing,
            )],
            claim_narrow: Some(CoreControlClaimAutoNarrow {
                narrowed_to: M5CoreControlClaim::AccessibleNameUnverifiedProjection,
                binding_dimension: M5CoreControlClaimDimension::AccessibleNameClarity,
                trigger: M5CoreControlDowngradeTrigger::IconOnlyDestructiveUnlabeled,
                narrowed_label:
                    "This icon-only button has no confirmed accessible name — shown as an accessible-name-unverified projection that keeps the last-known glyph role and action visible and flags the destructive state, never presenting an unlabeled icon-only destructive action as a safe, ready-to-invoke control"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "glyph_role",
                "accessible_name",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::ReviewUi,
                M5CoreControlConsumerSurface::RepairUi,
            ]),
            source_refs: vec![
                "UX Style Guide §15.1 — Icon button".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("icon-button-accessible-name-missing"),
        },
        // Split button (safe default unconfirmed) — structure-heavy (a default action plus an alternate
        // menu); the safe default cannot be confirmed, so it auto-narrows to a default-safety-unverified
        // projection that keeps the safe default explicit and the alternate menu inspectable, never letting
        // a riskier alternate become the default (yellow).
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:split-button-default-safety-stale".to_owned(),
            component_family: M5CoreControlFamily::SplitButton,
            source_family_schema_ref: M5CoreControlFamily::SplitButton
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "repair:split-button:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:split-button-default-safety-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "safe_default_action",
                "alternate_actions",
                "default_safety_state",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::DefaultSafetyClarity,
                M5CoreControlConditionState::DefaultSafetyStale,
            )],
            claim_narrow: Some(CoreControlClaimAutoNarrow {
                narrowed_to: M5CoreControlClaim::DefaultSafetyUnverifiedProjection,
                binding_dimension: M5CoreControlClaimDimension::DefaultSafetyClarity,
                trigger: M5CoreControlDowngradeTrigger::SplitDefaultedToRiskierAlternate,
                narrowed_label:
                    "This split button's safe default cannot be confirmed — shown as a default-safety-unverified projection that keeps the safe default action explicit and the alternate menu inspectable, never letting a riskier alternate quietly become the default"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "safe_default_action",
                "alternate_actions",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::RepairUi,
                M5CoreControlConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UX Style Guide §15.1 — Split button".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("split-button-default-safety-stale"),
        },
        // Text field (validation anchor stale) — the field's validation anchor is stale, so it auto-narrows
        // to a validation-unverified projection that keeps the permanent label and last-known validation
        // state visible rather than a placeholder-as-label, never a freshly-validated field (yellow).
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:text-field-validation-anchor-stale".to_owned(),
            component_family: M5CoreControlFamily::TextField,
            source_family_schema_ref: M5CoreControlFamily::TextField
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "settings:text-field:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:text-field-validation-anchor-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "permanent_label",
                "validation_state",
                "last_known_validation",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::LabelValidationClarity,
                M5CoreControlConditionState::ValidationAnchorStale,
            )],
            claim_narrow: Some(CoreControlClaimAutoNarrow {
                narrowed_to: M5CoreControlClaim::ValidationUnverifiedProjection,
                binding_dimension: M5CoreControlClaimDimension::LabelValidationClarity,
                trigger: M5CoreControlDowngradeTrigger::ValidationStateUnstated,
                narrowed_label:
                    "This text field's validation anchor is stale — shown as a validation-unverified projection that keeps the permanent label and last-known validation state visible rather than collapsing into a placeholder-as-label, never presenting a stale field as freshly validated"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "permanent_label",
                "validation_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::SettingsUi,
                M5CoreControlConsumerSurface::FormsUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.1 — Text field".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("text-field-validation-anchor-stale"),
        },
        // Toggle control (immediate/deferred semantic unverified) — the checkbox / radio / switch cannot
        // confirm whether its effect is immediate or deferred, so it auto-narrows to a
        // toggle-semantics-unverified projection that keeps the last-known toggle semantics visible, never
        // blurring a switch with a deferred checkbox (yellow).
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:toggle-control-semantics-unverified".to_owned(),
            component_family: M5CoreControlFamily::ToggleControl,
            source_family_schema_ref: M5CoreControlFamily::ToggleControl
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "settings:toggle-control:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:toggle-control-semantics-unverified:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "toggle_state",
                "immediate_or_deferred_semantic",
                "last_known_semantic",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::ToggleSemanticsClarity,
                M5CoreControlConditionState::ToggleSemanticsUnverified,
            )],
            claim_narrow: Some(CoreControlClaimAutoNarrow {
                narrowed_to: M5CoreControlClaim::ToggleSemanticsUnverifiedProjection,
                binding_dimension: M5CoreControlClaimDimension::ToggleSemanticsClarity,
                trigger: M5CoreControlDowngradeTrigger::SwitchAndDeferredCheckboxBlurred,
                narrowed_label:
                    "This toggle control cannot confirm whether its effect is immediate or deferred — shown as a toggle-semantics-unverified projection that keeps the last-known checkbox / radio / switch semantics visible, never blurring an immediate switch with a deferred checkbox"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "toggle_state",
                "immediate_or_deferred_semantic",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::SettingsUi,
                M5CoreControlConsumerSurface::FormsUi,
            ]),
            source_refs: vec![
                "UX Style Guide §16.2 — Checkbox / radio / switch".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("toggle-control-semantics-unverified"),
        },
        // Search field (retention posture partial) — the search field can only disclose a partial /
        // redacted retention / privacy posture, so it auto-narrows to a retention-disclosed projection that
        // discloses the partial retention posture alongside the clear / submit truth, never hiding the
        // retention in an opaque field (yellow). A partial retention disclosure is an honest
        // disclosed-absence operation, not a trusted overstatement.
        CoreControlAccessibilityRow {
            record_kind: CORE_ACTION_INPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: CORE_ACTION_INPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:search-field-retention-disclosed-partial".to_owned(),
            component_family: M5CoreControlFamily::SearchField,
            source_family_schema_ref: M5CoreControlFamily::SearchField
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "search:search-field:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: CoreControlExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:search-field-retention-disclosed-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_identity",
                "clear_and_submit_truth",
                "retention_posture",
                "partial_or_redacted_note",
            ]),
            full_ready_claim: M5CoreControlClaim::TrustedControl,
            claim_conditions: vec![condition(
                M5CoreControlClaimDimension::ClearSubmitPrivacyClarity,
                M5CoreControlConditionState::RetentionDisclosedPartial,
            )],
            claim_narrow: Some(CoreControlClaimAutoNarrow {
                narrowed_to: M5CoreControlClaim::RetentionDisclosedProjection,
                binding_dimension: M5CoreControlClaimDimension::ClearSubmitPrivacyClarity,
                trigger: M5CoreControlDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This search field can only disclose a partial or redacted retention and privacy posture — shown as a retention-disclosed projection that discloses the partial retention posture alongside the clear / submit truth, never hiding the retention behavior in an opaque field"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_identity",
                "clear_and_submit_truth",
                "retention_posture",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5CoreControlConsumerSurface::SearchUi,
                M5CoreControlConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UX Style Guide §16.1 — Search field".to_owned(),
                CORE_ACTION_INPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("search-field-retention-disclosed-partial"),
        },
    ]
}
