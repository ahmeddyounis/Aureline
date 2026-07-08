//! One reusable M5 design-system primitive — the state-distinction explanation helper — so every
//! claimed M5 onboarding / help surface, blocked-action explanation row, settings row, activity row,
//! and workspace-entry surface can explain the most easily confused state distinctions *in place*,
//! with the same words the components themselves expose, instead of forcing the user into external
//! docs or tribal knowledge. The four distinctions this primitive teaches are exactly the ones the
//! acceptance criteria and the frozen taxonomy's precedence rules call out: `current` versus
//! `selected`, `read-only` versus `disabled`, `locked` versus `disabled`, and `pending` versus
//! `loading`. Each explanation names both states, marks that they stay distinct, links back to the
//! canonical shared taxonomy so no surface invents a one-off label, and — for the blocked and
//! limited cases (`read-only`/`disabled`, `locked`/`disabled`, and `pending`/`loading`) — carries a
//! blocked/limited-state copy object that names the consequence, the owner / block reason, and the
//! next safe action, keeping contextual-teaching and blocked-action help aligned with the same state
//! truth the components render.
//!
//! Aureline's frozen shared-component-state-taxonomy component matrix
//! ([`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`])
//! freezes the thirteen canonical state classes and — critically for this primitive — the four
//! state precedence / distinctness rules ([`M5StatePrecedenceRule`]): `locked` over `disabled`,
//! `read-only` over `disabled`, `current` distinct from `selected`, and `pending` distinct from
//! `loading`. This module *implements* explanation helpers over exactly those four rules, so a user
//! reading an onboarding tip, a blocked-action row, a settings row, an activity row, or a
//! workspace-entry surface — on the desktop or through the support export and screen reader alike —
//! always gets the same in-place explanation of why two look-alike states are not the same, rather
//! than one-off copy improvised per feature.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_state_distinction_explanation`] — takes one consumer surface, the confusable state
//!    distinction it needs to explain (one of `current`/`selected`, `read-only`/`disabled`,
//!    `locked`/`disabled`, `pending`/`loading`), the delivery form (an inline chip, an expanded
//!    drawer, or a blocked/limited-state copy object), the recovery-disclosure class and state cause
//!    behind the confusable state, whether a recovery path is available, the high-contrast context,
//!    its opaque stable explanation identity, the opaque shared-taxonomy reference it links back to,
//!    the opaque distinction-copy reference, and the opaque blocked/limited-copy reference, and
//!    produces one [`M5ResolvedStateExplanation`] carrying the derived precedence rule, the primary
//!    and contrasted states the distinction keeps apart, the required non-color cues that carry the
//!    explanation beyond hue, the required disclosures the explanation must publish, and the hard
//!    guarantees that the two states never collapse, the explanation is delivered in place, no
//!    one-off language is invented, the explanation stays aligned with the shared taxonomy, and the
//!    blocked-action copy stays aligned with the same component-state truth. It refuses an
//!    explanation that links to no shared taxonomy (which would invent one-off language), refuses a
//!    blocked/limited-copy delivery for a distinction with no blocked or limited side, refuses a
//!    blocked/limited-copy delivery that carries no copy, refuses a non-blocked delivery that
//!    smuggles in blocked/limited copy, and refuses a recovery-availability claim that contradicts
//!    its recovery-disclosure class.
//!
//! A single parity matrix — [`M5StateExplanationPacket`] — binds one row per claimed M5 consumer
//! surface (the onboarding/help surface, the blocked-action row, the settings row, the activity row,
//! and the workspace-entry surface) to the shared explanation anatomy, the same distinctions,
//! precedence rules, delivery forms, non-color cues, required disclosures, recovery-disclosure
//! classes, state cause classes, export fields, mandatory labels, and non-visual accessibility
//! routes, so the explanation vocabulary and its no-one-off-language, stay-distinct, and
//! blocked-action-alignment rules stay identical across desktop, headless/export, and support
//! consumers.
//!
//! The state class ([`M5SharedComponentStateClass`]), the precedence rule
//! ([`M5StatePrecedenceRule`]), the disclosure trigger ([`M5StateDisclosureTrigger`]), the
//! recovery-disclosure class ([`M5RecoveryDisclosureClass`]), the state cause class
//! ([`M5StateCauseClass`]), the surface family ([`M5ComponentStateSurfaceFamily`]), the deployment
//! line ([`M5ComponentStateDeploymentLine`]), the consumer surface
//! ([`M5ComponentStateConsumerSurface`]), the accessibility route
//! ([`M5ComponentStateAccessibilityRoute`]), the required label
//! ([`M5ComponentStateRequiredLabel`]), the qualification class
//! ([`M5ComponentStateQualificationClass`]), and the downgrade trigger
//! ([`M5ComponentStateDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the explanation helper itself:
//! its claimed consumer surfaces, its confusable distinctions, its delivery forms, its anatomy
//! parts, its non-color cues, and its export fields. No M5 surface invents a second explanation
//! grammar.
//!
//! Raw local paths, credentials, and private endpoints stay outside the export boundary; every
//! explanation identity, taxonomy reference, distinction-copy reference, and blocked/limited-copy
//! reference is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_state_explanation_blocked_action_beta_narrowed, seeded_m5_state_explanation_packet,
    seeded_m5_state_explanation_workspace_entry_preview_narrowed, M5_STATE_EXPLANATION_PACKET_ID,
};

// The state class, precedence rule, disclosure trigger, recovery-disclosure class, state cause
// class, surface family, deployment line, consumer surface, accessibility route, required label,
// qualification class, and downgrade triggers are frozen once, in the shared-component-state-taxonomy
// component matrix. This primitive reuses them verbatim so it never invents a parallel state
// vocabulary.
pub use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5ComponentStateAccessibilityRoute, M5ComponentStateConsumerSurface,
    M5ComponentStateDeploymentLine, M5ComponentStateDowngradeTrigger,
    M5ComponentStateQualificationClass, M5ComponentStateRequiredLabel,
    M5ComponentStateSurfaceFamily, M5RecoveryDisclosureClass, M5SharedComponentStateClass,
    M5StateCauseClass, M5StateDisclosureTrigger, M5StatePrecedenceRule,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5StateExplanationPacket`].
pub const M5_STATE_EXPLANATION_RECORD_KIND: &str =
    "implement_m5_current_vs_selected_read_only_vs_disabled_locked_vs_disabled_and_pending_vs_loading_state_explanation_helpers_across_claimed_m5_surfaces";

/// Schema version for M5 state-distinction-explanation records.
pub const M5_STATE_EXPLANATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the explanation-helper boundary schema.
pub const M5_STATE_EXPLANATION_SCHEMA_REF: &str =
    "schemas/ui/m5-state-distinction-explanation-helper.schema.json";

/// Repo-relative path of the explanation-helper doc.
pub const M5_STATE_EXPLANATION_DOC_REF: &str =
    "docs/design-system/m5_state_distinction_explanation_helper_primitive.md";

/// Repo-relative path of the frozen shared-component-state-taxonomy component matrix whose
/// precedence rules this primitive teaches.
pub const M5_STATE_EXPLANATION_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json";

/// Repo-relative path of the canonical state-class contract each explanation links its two states
/// back to, so no surface invents a one-off state label.
pub const M5_STATE_EXPLANATION_STATE_CLASS_REF: &str = "schemas/state/state_class.schema.json";

/// Repo-relative path of the feature-availability (why-unavailable) contract the blocked-action
/// explanation row binds its blocked/limited copy against, keeping blocked-action help aligned with
/// the same component-state truth.
pub const M5_STATE_EXPLANATION_BLOCKED_ACTION_REF: &str =
    "schemas/ux/feature_availability_row.schema.json";

/// Repo-relative path of the contextual-tip contract the onboarding / help explanation binds its
/// in-place teaching against, keeping contextual teaching aligned with the same state truth.
pub const M5_STATE_EXPLANATION_CONTEXTUAL_TEACHING_REF: &str =
    "schemas/ui/m5-contextual-tip-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_STATE_EXPLANATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-state-distinction-explanation-helper-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_STATE_EXPLANATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-state-distinction-explanation-helper-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_STATE_EXPLANATION_CSV_REF: &str =
    "artifacts/release/m5-state-distinction-explanation-helper-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_STATE_EXPLANATION_REPORT_REF: &str =
    "artifacts/design/m5-state-distinction-explanation-helper-primitive.md";

/// One claimed M5 consumer surface that hosts the shared state-distinction explanation helper. These
/// are the high-friction surfaces the implementation requirements name — onboarding / help,
/// blocked-action explanation rows, settings, activity rows, and workspace-entry surfaces — so the
/// same explanation grammar works across every claimed consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationConsumerSurface {
    /// An onboarding / contextual-teaching help surface.
    OnboardingHelp,
    /// A blocked-action explanation row.
    BlockedActionRow,
    /// A settings row.
    SettingsRow,
    /// An activity-center row.
    ActivityRow,
    /// A workspace-entry surface.
    WorkspaceEntry,
}

impl M5ExplanationConsumerSurface {
    /// Every claimed consumer surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OnboardingHelp,
        Self::BlockedActionRow,
        Self::SettingsRow,
        Self::ActivityRow,
        Self::WorkspaceEntry,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnboardingHelp => "onboarding_help",
            Self::BlockedActionRow => "blocked_action_row",
            Self::SettingsRow => "settings_row",
            Self::ActivityRow => "activity_row",
            Self::WorkspaceEntry => "workspace_entry",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnboardingHelp => "Onboarding / Help",
            Self::BlockedActionRow => "Blocked-Action Row",
            Self::SettingsRow => "Settings Row",
            Self::ActivityRow => "Activity Row",
            Self::WorkspaceEntry => "Workspace Entry",
        }
    }
}

/// One of the four most easily confused state distinctions this primitive teaches. Each is exactly
/// one of the frozen taxonomy's precedence / distinctness rules, so the explanation never invents a
/// distinction the taxonomy does not already own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConfusableStateDistinction {
    /// `current` (the live route / context owner) versus `selected` (a durable selection).
    CurrentVsSelected,
    /// `read-only` (inspectable but not editable) versus `disabled` (non-actionable).
    ReadOnlyVsDisabled,
    /// `locked` (an explainable policy / trust / ownership lock) versus `disabled`.
    LockedVsDisabled,
    /// `pending` (a submitted user action awaiting commit) versus `loading` (generic background
    /// work).
    PendingVsLoading,
}

impl M5ConfusableStateDistinction {
    /// Every confusable distinction, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentVsSelected,
        Self::ReadOnlyVsDisabled,
        Self::LockedVsDisabled,
        Self::PendingVsLoading,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentVsSelected => "current_vs_selected",
            Self::ReadOnlyVsDisabled => "read_only_vs_disabled",
            Self::LockedVsDisabled => "locked_vs_disabled",
            Self::PendingVsLoading => "pending_vs_loading",
        }
    }

    /// The frozen precedence / distinctness rule this distinction is an explanation of. Reused from
    /// the shared taxonomy so the explanation never invents a rule of its own.
    pub const fn precedence_rule(self) -> M5StatePrecedenceRule {
        match self {
            Self::CurrentVsSelected => M5StatePrecedenceRule::CurrentDistinctFromSelected,
            Self::ReadOnlyVsDisabled => M5StatePrecedenceRule::ReadOnlyOverDisabled,
            Self::LockedVsDisabled => M5StatePrecedenceRule::LockedOverDisabled,
            Self::PendingVsLoading => M5StatePrecedenceRule::PendingDistinctFromLoading,
        }
    }

    /// The primary state — the one the distinction wants a user to recognize (`current`,
    /// `read-only`, `locked`, or `pending`).
    pub const fn primary_state(self) -> M5SharedComponentStateClass {
        match self {
            Self::CurrentVsSelected => M5SharedComponentStateClass::Current,
            Self::ReadOnlyVsDisabled => M5SharedComponentStateClass::ReadOnly,
            Self::LockedVsDisabled => M5SharedComponentStateClass::Locked,
            Self::PendingVsLoading => M5SharedComponentStateClass::Pending,
        }
    }

    /// The contrasted state — the look-alike the primary state must never collapse into (`selected`,
    /// `disabled`, `disabled`, or `loading`).
    pub const fn contrasted_state(self) -> M5SharedComponentStateClass {
        match self {
            Self::CurrentVsSelected => M5SharedComponentStateClass::Selected,
            Self::ReadOnlyVsDisabled => M5SharedComponentStateClass::Disabled,
            Self::LockedVsDisabled => M5SharedComponentStateClass::Disabled,
            Self::PendingVsLoading => M5SharedComponentStateClass::Loading,
        }
    }

    /// True when this distinction has a blocked or limited side — so a blocked/limited-state copy
    /// object applies. `read-only`/`disabled` and `locked`/`disabled` block an action; `pending`
    /// (submitted, awaiting commit) and `loading` limit interaction while in flight.
    /// `current`/`selected` are two positive selection states with no blocked or limited side.
    pub const fn has_blocked_or_limited_side(self) -> bool {
        match self {
            Self::CurrentVsSelected => false,
            Self::ReadOnlyVsDisabled | Self::LockedVsDisabled | Self::PendingVsLoading => true,
        }
    }
}

/// The delivery form of an explanation — how the helper teaches the distinction in place. Derived
/// nothing from the distinction: a surface picks the delivery, and the resolver derives the cues and
/// disclosures each form must carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationDelivery {
    /// A short inline chip that names the primary state and marks that it differs from its
    /// look-alike.
    InlineChip,
    /// An expanded drawer that names both states, marks the distinction, and links back to the
    /// canonical taxonomy.
    ExpandedDrawer,
    /// A blocked/limited-state copy object that names the consequence, the owner / block reason, and
    /// the next safe action for a blocked or limited state.
    BlockedLimitedCopy,
}

impl M5ExplanationDelivery {
    /// Every delivery form, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::InlineChip,
        Self::ExpandedDrawer,
        Self::BlockedLimitedCopy,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineChip => "inline_chip",
            Self::ExpandedDrawer => "expanded_drawer",
            Self::BlockedLimitedCopy => "blocked_limited_copy",
        }
    }

    /// True when this delivery is the blocked/limited-state copy object.
    pub const fn is_blocked_limited_copy(self) -> bool {
        matches!(self, Self::BlockedLimitedCopy)
    }
}

/// One non-color cue an explanation renders so its meaning is never carried by hue alone. Every
/// delivery form publishes at least one of these, enforcing the no-color-only signaling rule and
/// keeping the two look-alike states legible apart without color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationCue {
    /// A text label naming the primary state (for example, "Current").
    PrimaryStateLabel,
    /// A text label naming the contrasted look-alike state (for example, "Selected").
    ContrastedStateLabel,
    /// A non-color marker that the two states stay distinct.
    DistinctionMarker,
    /// A glyph marking the blocked or limited side of the distinction.
    BlockedLimitedGlyph,
    /// A recovery affordance naming the next safe action out of a blocked or limited state.
    RecoveryAffordance,
    /// A reference cue linking the explanation back to the canonical shared taxonomy meaning.
    TaxonomyReferenceCue,
}

impl M5ExplanationCue {
    /// Every non-color cue, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PrimaryStateLabel,
        Self::ContrastedStateLabel,
        Self::DistinctionMarker,
        Self::BlockedLimitedGlyph,
        Self::RecoveryAffordance,
        Self::TaxonomyReferenceCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryStateLabel => "primary_state_label",
            Self::ContrastedStateLabel => "contrasted_state_label",
            Self::DistinctionMarker => "distinction_marker",
            Self::BlockedLimitedGlyph => "blocked_limited_glyph",
            Self::RecoveryAffordance => "recovery_affordance",
            Self::TaxonomyReferenceCue => "taxonomy_reference_cue",
        }
    }
}

/// Controlled explanation anatomy part the shared helper surfaces. The parts in
/// [`M5ExplanationAnatomyPart::MANDATORY`] are required on every surface so the distinction
/// identity, both state labels, the delivery form, and the non-visual keyboard route are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationAnatomyPart {
    /// The typed distinction identity cue.
    DistinctionIdentityCue,
    /// The primary-state label cue.
    PrimaryStateLabelCue,
    /// The contrasted-state label cue.
    ContrastedStateLabelCue,
    /// The delivery-form cue.
    DeliveryFormCue,
    /// The state-cause cue (why the confusable state applies).
    StateCauseCue,
    /// The blocked/limited-copy cue (the blocked-action help body).
    BlockedLimitedCopyCue,
    /// The recovery-action cue (the next safe action out of a blocked / limited state).
    RecoveryActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5ExplanationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DistinctionIdentityCue,
        Self::PrimaryStateLabelCue,
        Self::ContrastedStateLabelCue,
        Self::DeliveryFormCue,
        Self::StateCauseCue,
        Self::BlockedLimitedCopyCue,
        Self::RecoveryActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every surface must render.
    pub const MANDATORY: [Self; 5] = [
        Self::DistinctionIdentityCue,
        Self::PrimaryStateLabelCue,
        Self::ContrastedStateLabelCue,
        Self::DeliveryFormCue,
        Self::KeyboardRouteCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DistinctionIdentityCue => "distinction_identity_cue",
            Self::PrimaryStateLabelCue => "primary_state_label_cue",
            Self::ContrastedStateLabelCue => "contrasted_state_label_cue",
            Self::DeliveryFormCue => "delivery_form_cue",
            Self::StateCauseCue => "state_cause_cue",
            Self::BlockedLimitedCopyCue => "blocked_limited_copy_cue",
            Self::RecoveryActionCue => "recovery_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the explanation export carries so its truth is reconstructable. The fields in
/// [`M5ExplanationExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExplanationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The confusable distinction.
    Distinction,
    /// The frozen precedence rule.
    PrecedenceRule,
    /// The delivery form.
    Delivery,
    /// The primary state.
    PrimaryState,
    /// The contrasted state.
    ContrastedState,
    /// The state cause.
    StateCause,
    /// Whether a blocked/limited-copy object is present.
    BlockedLimitedCopyPresent,
}

impl M5ExplanationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ConsumerSurface,
        Self::Distinction,
        Self::PrecedenceRule,
        Self::Delivery,
        Self::PrimaryState,
        Self::ContrastedState,
        Self::StateCause,
        Self::BlockedLimitedCopyPresent,
    ];

    /// The export fields every surface must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::Distinction,
        Self::PrecedenceRule,
        Self::Delivery,
        Self::PrimaryState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::Distinction => "distinction",
            Self::PrecedenceRule => "precedence_rule",
            Self::Delivery => "delivery",
            Self::PrimaryState => "primary_state",
            Self::ContrastedState => "contrasted_state",
            Self::StateCause => "state_cause",
            Self::BlockedLimitedCopyPresent => "blocked_limited_copy_present",
        }
    }
}

/// The four confusable distinctions this primitive teaches, in declaration order.
pub fn confusable_distinctions() -> Vec<M5ConfusableStateDistinction> {
    M5ConfusableStateDistinction::ALL.to_vec()
}

// ---- state-distinction explanation resolver -----------------------------

/// The full input to the state-distinction-explanation resolver for one explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateExplanationInput {
    /// The claimed consumer surface hosting the explanation.
    pub surface: M5ExplanationConsumerSurface,
    /// The confusable distinction the explanation teaches.
    pub distinction: M5ConfusableStateDistinction,
    /// The delivery form (inline chip, expanded drawer, or blocked/limited copy).
    pub delivery: M5ExplanationDelivery,
    /// The recovery-disclosure class the explanation names behind the confusable state.
    pub recovery_class: M5RecoveryDisclosureClass,
    /// The cause of the confusable state (why it applies).
    pub state_cause: M5StateCauseClass,
    /// True when a recovery path out of the blocked / limited state is available.
    pub recovery_available: bool,
    /// True when a high-contrast mode is active, so the explanation stays legible without hue.
    pub high_contrast_active: bool,
    /// The opaque stable explanation identity (must be non-empty).
    pub explanation_identity_ref: String,
    /// The opaque shared-taxonomy reference this explanation links back to (must be non-empty, so
    /// the explanation never invents a one-off label divorced from the taxonomy).
    pub taxonomy_ref: String,
    /// The opaque distinction-copy reference — the chip / drawer copy (must be non-empty).
    pub distinction_copy_ref: String,
    /// The opaque blocked/limited-copy reference (must be non-empty for a blocked/limited-copy
    /// delivery, and must be empty for any other delivery).
    pub blocked_limited_copy_ref: String,
}

/// The resolved state-distinction-explanation truth for one explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedStateExplanation {
    /// The consumer surface.
    pub surface: M5ExplanationConsumerSurface,
    /// The confusable distinction.
    pub distinction: M5ConfusableStateDistinction,
    /// The delivery form.
    pub delivery: M5ExplanationDelivery,
    /// The frozen precedence rule this explanation teaches, derived from the distinction.
    pub precedence_rule: M5StatePrecedenceRule,
    /// The primary state the distinction wants recognized, derived from the distinction.
    pub primary_state: M5SharedComponentStateClass,
    /// The contrasted look-alike state, derived from the distinction.
    pub contrasted_state: M5SharedComponentStateClass,
    /// The required non-color cues that carry this explanation beyond hue.
    pub required_non_color_cues: Vec<M5ExplanationCue>,
    /// The disclosures this explanation must publish (state cause, owner / block reason, recovery
    /// action, and never a silent style-only change).
    pub required_disclosures: Vec<M5StateDisclosureTrigger>,
    /// The recovery-disclosure class behind the state, preserved exactly from the input.
    pub recovery_class: M5RecoveryDisclosureClass,
    /// The cause of the state, preserved exactly from the input.
    pub state_cause: M5StateCauseClass,
    /// True when a recovery path is available, preserved from the input.
    pub recovery_available: bool,
    /// True when high-contrast is active, preserved from the input.
    pub high_contrast_active: bool,
    /// The opaque stable explanation identity, preserved exactly from the input.
    pub explanation_identity_ref: String,
    /// The opaque shared-taxonomy reference, preserved exactly from the input.
    pub taxonomy_ref: String,
    /// The opaque distinction-copy reference, preserved exactly from the input.
    pub distinction_copy_ref: String,
    /// The opaque blocked/limited-copy reference, preserved exactly from the input.
    pub blocked_limited_copy_ref: String,
    /// True when this distinction has a blocked or limited side.
    pub touches_blocked_or_limited_state: bool,
    /// True when this explanation carries the blocked-action help (a blocked/limited-copy delivery).
    pub carries_blocked_action_help: bool,
    /// The explanation is delivered in place, not deferred to external docs. ALWAYS `true`.
    pub explains_distinction_in_place: bool,
    /// The two look-alike states never collapse into one another. ALWAYS `true`.
    pub states_stay_distinct: bool,
    /// No one-off, per-surface state language is invented. ALWAYS `true`.
    pub no_one_off_language: bool,
    /// The explanation stays aligned with the shared taxonomy. ALWAYS `true`.
    pub aligned_with_shared_taxonomy: bool,
    /// Blocked-action help stays aligned with the same component-state truth. ALWAYS `true`.
    pub blocked_action_help_aligned_with_component_truth: bool,
    /// State meaning is never carried by color alone. ALWAYS `true`.
    pub no_color_only_signaling: bool,
    /// The explanation stays keyboard- and screen-reader-explainable. ALWAYS `true`.
    pub keyboard_and_screen_reader_explainable: bool,
    /// The explanation is driven by the shared contract and its token hooks, not a one-off
    /// implementation choice. ALWAYS `true`.
    pub driven_by_shared_state_contract: bool,
}

/// Errors returned by [`resolve_state_distinction_explanation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5StateExplanationResolutionError {
    /// The explanation identity ref was empty.
    EmptyExplanationIdentity,
    /// The shared-taxonomy ref was empty, so the explanation would float free of the taxonomy and
    /// invent one-off language.
    EmptyTaxonomyRef,
    /// The distinction-copy ref (chip / drawer copy) was empty.
    EmptyDistinctionCopyRef,
    /// A blocked/limited-copy delivery carried no blocked/limited copy.
    BlockedLimitedCopyMissing,
    /// A blocked/limited-copy delivery was used for a distinction with no blocked or limited side.
    BlockedLimitedCopyOnUnblockableDistinction,
    /// A non-blocked delivery smuggled in blocked/limited copy.
    BlockedLimitedCopyOnNonBlockedDelivery,
    /// The recovery-availability flag contradicted the recovery-disclosure class.
    RecoveryClassMismatch,
    /// A descriptor carried forbidden material.
    ForbiddenStateMaterial,
}

impl M5StateExplanationResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyExplanationIdentity => "empty_explanation_identity",
            Self::EmptyTaxonomyRef => "empty_taxonomy_ref",
            Self::EmptyDistinctionCopyRef => "empty_distinction_copy_ref",
            Self::BlockedLimitedCopyMissing => "blocked_limited_copy_missing",
            Self::BlockedLimitedCopyOnUnblockableDistinction => {
                "blocked_limited_copy_on_unblockable_distinction"
            }
            Self::BlockedLimitedCopyOnNonBlockedDelivery => {
                "blocked_limited_copy_on_non_blocked_delivery"
            }
            Self::RecoveryClassMismatch => "recovery_class_mismatch",
            Self::ForbiddenStateMaterial => "forbidden_state_material",
        }
    }
}

impl fmt::Display for M5StateExplanationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "state distinction explanation resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5StateExplanationResolutionError {}

/// Resolves one state-distinction explanation from a consumer surface, the confusable distinction it
/// teaches, and the delivery / recovery / copy context behind it.
///
/// The precedence rule, the primary state, and the contrasted state are derived one-to-one from the
/// distinction, so the explanation always teaches exactly the taxonomy's own rule and never invents
/// a distinction of its own. Each delivery form publishes a non-empty non-color cue set so the
/// explanation is never carried by color alone, and a required-disclosure set so a blocked/limited
/// explanation always names its cause, its owner / block reason, and its recovery action. The
/// resolver refuses an explanation that links to no shared taxonomy (which would invent one-off
/// language), refuses a blocked/limited-copy delivery for a distinction with no blocked or limited
/// side, refuses a blocked/limited-copy delivery that carries no copy, refuses a non-blocked
/// delivery that smuggles in blocked/limited copy, and refuses a recovery-availability claim that
/// contradicts its recovery-disclosure class.
pub fn resolve_state_distinction_explanation(
    input: &M5StateExplanationInput,
) -> Result<M5ResolvedStateExplanation, M5StateExplanationResolutionError> {
    if input.explanation_identity_ref.trim().is_empty() {
        return Err(M5StateExplanationResolutionError::EmptyExplanationIdentity);
    }
    if input.taxonomy_ref.trim().is_empty() {
        return Err(M5StateExplanationResolutionError::EmptyTaxonomyRef);
    }
    if input.distinction_copy_ref.trim().is_empty() {
        return Err(M5StateExplanationResolutionError::EmptyDistinctionCopyRef);
    }
    if value_repr_is_forbidden(&input.explanation_identity_ref)
        || value_repr_is_forbidden(&input.taxonomy_ref)
        || value_repr_is_forbidden(&input.distinction_copy_ref)
        || value_repr_is_forbidden(&input.blocked_limited_copy_ref)
    {
        return Err(M5StateExplanationResolutionError::ForbiddenStateMaterial);
    }

    let has_blocked_copy = !input.blocked_limited_copy_ref.trim().is_empty();
    // A blocked/limited-copy delivery names a blocked or limited state, so it applies only to a
    // distinction with a blocked or limited side, and it must carry the blocked/limited copy. Any
    // other delivery must not smuggle in blocked/limited copy.
    if input.delivery.is_blocked_limited_copy() {
        if !input.distinction.has_blocked_or_limited_side() {
            return Err(
                M5StateExplanationResolutionError::BlockedLimitedCopyOnUnblockableDistinction,
            );
        }
        if !has_blocked_copy {
            return Err(M5StateExplanationResolutionError::BlockedLimitedCopyMissing);
        }
    } else if has_blocked_copy {
        return Err(M5StateExplanationResolutionError::BlockedLimitedCopyOnNonBlockedDelivery);
    }

    // A named recovery-disclosure class means a recovery path is available; `no_recovery_available`
    // means none is. A contradiction between the two is refused.
    let claims_no_recovery = input.recovery_class == M5RecoveryDisclosureClass::NoRecoveryAvailable;
    if input.recovery_available == claims_no_recovery {
        return Err(M5StateExplanationResolutionError::RecoveryClassMismatch);
    }

    let required_non_color_cues = derive_non_color_cues(input.delivery);
    let required_disclosures = derive_required_disclosures(input.delivery);

    Ok(M5ResolvedStateExplanation {
        surface: input.surface,
        distinction: input.distinction,
        delivery: input.delivery,
        precedence_rule: input.distinction.precedence_rule(),
        primary_state: input.distinction.primary_state(),
        contrasted_state: input.distinction.contrasted_state(),
        required_non_color_cues,
        required_disclosures,
        recovery_class: input.recovery_class,
        state_cause: input.state_cause,
        recovery_available: input.recovery_available,
        high_contrast_active: input.high_contrast_active,
        explanation_identity_ref: input.explanation_identity_ref.clone(),
        taxonomy_ref: input.taxonomy_ref.clone(),
        distinction_copy_ref: input.distinction_copy_ref.clone(),
        blocked_limited_copy_ref: input.blocked_limited_copy_ref.clone(),
        touches_blocked_or_limited_state: input.distinction.has_blocked_or_limited_side(),
        carries_blocked_action_help: input.delivery.is_blocked_limited_copy(),
        // The acceptance criteria: the explanation is delivered in place, the two states never
        // collapse, no one-off language is invented, the explanation stays aligned with the shared
        // taxonomy, blocked-action help stays aligned with the same component-state truth, the
        // explanation is never color-only, it stays keyboard- and screen-reader-explainable, and it
        // is driven by the shared contract.
        explains_distinction_in_place: true,
        states_stay_distinct: true,
        no_one_off_language: true,
        aligned_with_shared_taxonomy: true,
        blocked_action_help_aligned_with_component_truth: true,
        no_color_only_signaling: true,
        keyboard_and_screen_reader_explainable: true,
        driven_by_shared_state_contract: true,
    })
}

/// Derives the non-color cue set for a delivery form. Every form publishes at least one non-color
/// cue, so an explanation is never carried by hue alone; the inline chip names the primary state and
/// marks the distinction, the expanded drawer additionally names the contrasted state and links to
/// the taxonomy, and the blocked/limited copy names the blocked side and its recovery affordance.
fn derive_non_color_cues(delivery: M5ExplanationDelivery) -> Vec<M5ExplanationCue> {
    use M5ExplanationCue as Cue;
    use M5ExplanationDelivery as Delivery;

    match delivery {
        Delivery::InlineChip => vec![Cue::PrimaryStateLabel, Cue::DistinctionMarker],
        Delivery::ExpandedDrawer => vec![
            Cue::PrimaryStateLabel,
            Cue::ContrastedStateLabel,
            Cue::DistinctionMarker,
            Cue::TaxonomyReferenceCue,
        ],
        Delivery::BlockedLimitedCopy => vec![
            Cue::PrimaryStateLabel,
            Cue::BlockedLimitedGlyph,
            Cue::RecoveryAffordance,
        ],
    }
}

/// Derives the required-disclosure set for a delivery form. Every form forbids a silent style-only
/// change; the expanded drawer additionally requires the state cause and the recovery action; the
/// blocked/limited copy requires the state cause, the owner / block reason, and the recovery action,
/// keeping blocked-action help aligned with the component-state truth.
fn derive_required_disclosures(delivery: M5ExplanationDelivery) -> Vec<M5StateDisclosureTrigger> {
    use M5ExplanationDelivery as Delivery;
    use M5StateDisclosureTrigger as Trigger;

    match delivery {
        Delivery::InlineChip => vec![Trigger::SilentStyleOnlyForbidden],
        Delivery::ExpandedDrawer => vec![
            Trigger::StateCauseRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
        Delivery::BlockedLimitedCopy => vec![
            Trigger::StateCauseRequired,
            Trigger::OwnerRequired,
            Trigger::BlockReasonRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked state-distinction-explanation resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateExplanationCase {
    /// The resolver input.
    pub input: M5StateExplanationInput,
    /// The resolved truth. Must equal `resolve_state_distinction_explanation(&input)`.
    pub resolved: M5ResolvedStateExplanation,
}

impl M5StateExplanationCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5StateExplanationInput) -> Self {
        let resolved = resolve_state_distinction_explanation(&input)
            .expect("seed state explanation case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_state_distinction_explanation(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input explanation identity, taxonomy reference,
    /// distinction-copy reference, and blocked/limited-copy reference exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.explanation_identity_ref == self.input.explanation_identity_ref
            && self.resolved.taxonomy_ref == self.input.taxonomy_ref
            && self.resolved.distinction_copy_ref == self.input.distinction_copy_ref
            && self.resolved.blocked_limited_copy_ref == self.input.blocked_limited_copy_ref
    }

    /// True when the resolved case keeps the two states distinct, teaches in place, invents no
    /// one-off language, stays aligned with the taxonomy and with blocked-action help, never signals
    /// by color alone, stays keyboard- and screen-reader-explainable, and is driven by the shared
    /// contract.
    pub fn preserves_guarantees(&self) -> bool {
        !self.resolved.required_non_color_cues.is_empty()
            && !self.resolved.required_disclosures.is_empty()
            && self.resolved.primary_state != self.resolved.contrasted_state
            && self.resolved.explains_distinction_in_place
            && self.resolved.states_stay_distinct
            && self.resolved.no_one_off_language
            && self.resolved.aligned_with_shared_taxonomy
            && self
                .resolved
                .blocked_action_help_aligned_with_component_truth
            && self.resolved.no_color_only_signaling
            && self.resolved.keyboard_and_screen_reader_explainable
            && self.resolved.driven_by_shared_state_contract
    }
}

/// One row in the primitive matrix: one claimed M5 consumer surface bound to the shared explanation
/// anatomy, distinctions, precedence rules, delivery forms, non-color cues, required disclosures,
/// recovery-disclosure classes, state cause classes, export fields, mandatory labels, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplanationSurfaceRow {
    /// Claimed consumer surface.
    pub surface: M5ExplanationConsumerSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5ComponentStateQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this explanation.
    pub surface_families: Vec<M5ComponentStateSurfaceFamily>,
    /// Deployment lines this surface keeps the same truth across.
    pub deployment_lines: Vec<M5ComponentStateDeploymentLine>,
    /// Anatomy parts this surface renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ExplanationAnatomyPart>,
    /// Confusable distinctions this surface can explain.
    pub distinctions: Vec<M5ConfusableStateDistinction>,
    /// Precedence rules this surface teaches.
    pub precedence_rules: Vec<M5StatePrecedenceRule>,
    /// Delivery forms this surface renders.
    pub deliveries: Vec<M5ExplanationDelivery>,
    /// Non-color cues this surface renders.
    pub non_color_cues: Vec<M5ExplanationCue>,
    /// Required disclosures this surface publishes.
    pub required_disclosures: Vec<M5StateDisclosureTrigger>,
    /// Recovery-disclosure classes this surface can name behind a blocked / limited state.
    pub recovery_disclosure_classes: Vec<M5RecoveryDisclosureClass>,
    /// State cause classes this surface can name behind a confusable state.
    pub state_cause_classes: Vec<M5StateCauseClass>,
    /// Export fields this surface carries (must include the mandatory fields).
    pub export_fields: Vec<M5ExplanationExportField>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5ComponentStateAccessibilityRoute>,
    /// Mandatory labels this surface can show (must include the mandatory labels).
    pub required_labels: Vec<M5ComponentStateRequiredLabel>,
    /// Subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5ComponentStateDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked explanation resolutions proving the resolver on this surface.
    pub explanation_examples: Vec<M5StateExplanationCase>,
    /// Hard invariant: this surface never invents a one-off state label. MUST be `false`.
    pub invents_one_off_state_language: bool,
    /// Hard invariant: this surface never contradicts the shared taxonomy. MUST be `false`.
    pub contradicts_shared_taxonomy: bool,
    /// Hard invariant: this surface never collapses the two look-alike states. MUST be `false`.
    pub collapses_the_two_states: bool,
    /// Hard invariant: this surface never misaligns blocked-action help from component-state truth.
    /// MUST be `false`.
    pub misaligns_blocked_action_help: bool,
}

impl M5ExplanationSurfaceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ExplanationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ExplanationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5ExplanationExportField> =
            self.export_fields.iter().copied().collect();
        M5ExplanationExportField::MANDATORY
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
        !self.invents_one_off_state_language
            && !self.contradicts_shared_taxonomy
            && !self.collapses_the_two_states
            && !self.misaligns_blocked_action_help
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplanationVocabularySet {
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Distinction tokens.
    pub distinctions: Vec<String>,
    /// Delivery-form tokens.
    pub deliveries: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Non-color-cue tokens.
    pub non_color_cues: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Precedence-rule tokens (reused from the frozen matrix).
    pub precedence_rules: Vec<String>,
    /// State-class tokens (reused from the frozen matrix).
    pub state_classes: Vec<String>,
    /// Required-disclosure tokens (reused from the frozen matrix).
    pub required_disclosures: Vec<String>,
    /// Recovery-disclosure-class tokens (reused from the frozen matrix).
    pub recovery_disclosure_classes: Vec<String>,
    /// State-cause-class tokens (reused from the frozen matrix).
    pub state_cause_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Shared consumer-surface tokens (reused from the frozen matrix).
    pub shared_consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens (reused from the frozen matrix).
    pub required_labels: Vec<String>,
}

impl M5ExplanationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ExplanationConsumerSurface::ALL, |v| v.as_str()),
            distinctions: tokens(&M5ConfusableStateDistinction::ALL, |v| v.as_str()),
            deliveries: tokens(&M5ExplanationDelivery::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ExplanationAnatomyPart::ALL, |v| v.as_str()),
            non_color_cues: tokens(&M5ExplanationCue::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ExplanationExportField::ALL, |v| v.as_str()),
            precedence_rules: tokens(&M5StatePrecedenceRule::ALL, |v| v.as_str()),
            state_classes: tokens(&M5SharedComponentStateClass::ALL, |v| v.as_str()),
            required_disclosures: tokens(&M5StateDisclosureTrigger::ALL, |v| v.as_str()),
            recovery_disclosure_classes: tokens(&M5RecoveryDisclosureClass::ALL, |v| v.as_str()),
            state_cause_classes: tokens(&M5StateCauseClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ComponentStateSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ComponentStateDeploymentLine::ALL, |v| v.as_str()),
            shared_consumer_surfaces: tokens(&M5ComponentStateConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5ExplanationGovernanceReview {
    /// Surfaces explain their state semantics in place.
    pub surfaces_explain_state_semantics_in_place: bool,
    /// `current` and `selected` never collapse into one another.
    pub current_and_selected_never_collapse: bool,
    /// `read-only` and `disabled` never collapse into one another.
    pub read_only_and_disabled_never_collapse: bool,
    /// `locked` and `disabled` never collapse into one another.
    pub locked_and_disabled_never_collapse: bool,
    /// `pending` and `loading` never collapse into one another.
    pub pending_and_loading_never_collapse: bool,
    /// No one-off, per-surface state language is invented.
    pub no_one_off_language_invented: bool,
    /// Explanations stay aligned with the shared taxonomy.
    pub aligned_with_shared_taxonomy: bool,
    /// Contextual teaching stays aligned with the same component-state truth.
    pub contextual_teaching_aligned_with_component_truth: bool,
    /// Blocked-action help stays aligned with the same component-state truth.
    pub blocked_action_help_aligned_with_component_truth: bool,
    /// State meaning is never carried by color alone.
    pub state_meaning_never_color_only: bool,
    /// Explanations stay keyboard- and screen-reader-explainable.
    pub explanations_keyboard_and_screen_reader_explainable: bool,
    /// Explanations are driven by the shared contract and its token hooks.
    pub explanations_driven_by_shared_contract_and_tokens: bool,
    /// No surface uses one-off, per-surface explanation copy.
    pub no_one_off_per_surface_copy: bool,
    /// Explanations keep the same truth across every deployment line.
    pub explanations_stable_across_deployment_lines: bool,
    /// Explanations keep the same truth across desktop, headless/export, and support consumers.
    pub explanations_stable_across_consumer_surfaces: bool,
    /// Every surface declares a non-visual accessibility route.
    pub every_surface_declares_accessibility_route: bool,
    /// The support / export packet reconstructs explanation truth.
    pub support_export_reconstructs_explanation_truth: bool,
    /// Later M5 rows cannot invent parallel state vocabulary.
    pub later_rows_cannot_invent_parallel_state_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplanationConsumerProjection {
    /// Surfaces consume the shared state vocabulary.
    pub surfaces_consume_state_vocabulary: bool,
    /// The non-color cue-set derivation reads a single canonical source.
    pub cue_set_reads_single_source: bool,
    /// The required-disclosure derivation reads a single canonical source.
    pub disclosure_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop surfaces read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplanationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the explanation helper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplanationReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting explanation audit.
    pub explanation_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5StateExplanationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StateExplanationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub rows: Vec<M5ExplanationSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExplanationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExplanationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExplanationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExplanationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExplanationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 state-distinction-explanation primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateExplanationPacket {
    /// Record kind; must equal [`M5_STATE_EXPLANATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STATE_EXPLANATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub rows: Vec<M5ExplanationSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExplanationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExplanationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExplanationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExplanationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExplanationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5StateExplanationPacket {
    /// Builds an M5 state-distinction-explanation-primitive packet from stable-lane input.
    pub fn new(input: M5StateExplanationPacketInput) -> Self {
        Self {
            record_kind: M5_STATE_EXPLANATION_RECORD_KIND.to_owned(),
            schema_version: M5_STATE_EXPLANATION_SCHEMA_VERSION,
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

    /// Validates the M5 state-distinction-explanation-primitive invariants.
    pub fn validate(&self) -> Vec<M5StateExplanationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_STATE_EXPLANATION_RECORD_KIND {
            violations.push(M5StateExplanationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_STATE_EXPLANATION_SCHEMA_VERSION {
            violations.push(M5StateExplanationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5StateExplanationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_distinction_coverage(self, &mut violations);
        validate_delivery_coverage(self, &mut violations);
        validate_cue_coverage(self, &mut violations);
        validate_disclosure_coverage(self, &mut violations);
        validate_guarantees(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 state explanation primitive packet serializes"),
        ) {
            violations.push(M5StateExplanationViolation::RawMaterialInExport);
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
            .expect("m5 state explanation primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface,qualification,owner,anatomy,distinctions,precedence_rules,deliveries,non_color_cues,required_disclosures,explanation_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.distinctions, |v| v.as_str()),
                join_tokens(&row.precedence_rules, |v| v.as_str()),
                join_tokens(&row.deliveries, |v| v.as_str()),
                join_tokens(&row.non_color_cues, |v| v.as_str()),
                join_tokens(&row.required_disclosures, |v| v.as_str()),
                row.explanation_examples.len(),
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
        out.push_str("# M5 State-Distinction Explanation Helper Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Distinctions: {}\n",
            self.vocabulary_set.distinctions.join(", ")
        ));
        out.push_str(&format!(
            "- Deliveries: {}\n",
            self.vocabulary_set.deliveries.join(", ")
        ));
        out.push_str(&format!(
            "- Non-color cues: {}\n",
            self.vocabulary_set.non_color_cues.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked explanations: {}\n",
                row.explanation_examples.len()
            ));
            for case in &row.explanation_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` via `{}`) → `{}` vs `{}` (non-color cues {}, blocked-action help `{}`, recovery `{}`)\n",
                    case.resolved.explanation_identity_ref,
                    case.resolved.distinction.as_str(),
                    case.resolved.delivery.as_str(),
                    case.resolved.primary_state.as_str(),
                    case.resolved.contrasted_state.as_str(),
                    case.resolved.required_non_color_cues.len(),
                    case.resolved.carries_blocked_action_help,
                    case.resolved.recovery_available,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 state-distinction-explanation-primitive export.
#[derive(Debug)]
pub enum M5StateExplanationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5StateExplanationViolation>),
}

impl fmt::Display for M5StateExplanationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 state explanation primitive export parse failed: {error}"
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
                    "m5 state explanation primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5StateExplanationArtifactError {}

/// Validation failures emitted by [`M5StateExplanationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5StateExplanationViolation {
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
    /// A required consumer surface is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
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
    /// A row declares no worked explanation resolutions.
    ExplanationExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// The worked resolutions do not exercise every confusable distinction.
    DistinctionCoverageUnproven,
    /// The worked resolutions do not exercise every delivery form.
    DeliveryCoverageUnproven,
    /// The worked resolutions do not exercise every non-color cue.
    CueCoverageUnproven,
    /// The worked resolutions do not exercise every required disclosure.
    DisclosureCoverageUnproven,
    /// A worked resolution does not hold the stay-distinct, in-place, no-one-off-language,
    /// taxonomy-alignment, blocked-action-alignment, no-color-only, and keyboard/screen-reader
    /// guarantees.
    GuaranteesUnproven,
    /// A worked resolution does not preserve its exact explanation identity, taxonomy, distinction
    /// copy, and blocked/limited-copy reference.
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

impl M5StateExplanationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExplanationExampleMissing => "explanation_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::DistinctionCoverageUnproven => "distinction_coverage_unproven",
            Self::DeliveryCoverageUnproven => "delivery_coverage_unproven",
            Self::CueCoverageUnproven => "cue_coverage_unproven",
            Self::DisclosureCoverageUnproven => "disclosure_coverage_unproven",
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

/// Reads and validates the checked-in stable M5 state-distinction-explanation-primitive export.
pub fn current_stable_m5_state_explanation_export(
) -> Result<M5StateExplanationPacket, M5StateExplanationArtifactError> {
    let packet: M5StateExplanationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-state-distinction-explanation-helper-primitive-proof/support_export.json"
    )))
    .map_err(M5StateExplanationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5StateExplanationArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_STATE_EXPLANATION_SCHEMA_REF,
        M5_STATE_EXPLANATION_DOC_REF,
        M5_STATE_EXPLANATION_COMPONENT_MATRIX_REF,
        M5_STATE_EXPLANATION_STATE_CLASS_REF,
        M5_STATE_EXPLANATION_BLOCKED_ACTION_REF,
        M5_STATE_EXPLANATION_CONTEXTUAL_TEACHING_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5StateExplanationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5StateExplanationViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let present: BTreeSet<M5ExplanationConsumerSurface> =
        packet.rows.iter().map(|row| row.surface).collect();
    for required in M5ExplanationConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5StateExplanationViolation::RequiredSurfaceMissing);
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
            || row.distinctions.is_empty()
            || row.precedence_rules.is_empty()
            || row.deliveries.is_empty()
            || row.non_color_cues.is_empty()
            || row.required_disclosures.is_empty()
            || row.recovery_disclosure_classes.is_empty()
            || row.state_cause_classes.is_empty()
            || row.export_fields.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5StateExplanationViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5StateExplanationViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5StateExplanationViolation::MandatoryExportMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5StateExplanationViolation::MandatoryLabelMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5StateExplanationViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5StateExplanationViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5StateExplanationViolation::DowngradeTriggersMissing);
        }
        if row.explanation_examples.is_empty() {
            violations.push(M5StateExplanationViolation::ExplanationExampleMissing);
        }
        if row
            .explanation_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5StateExplanationViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5StateExplanationViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5StateExplanationViolation::RowInvariantViolated);
        }
    }
}

/// Every confusable distinction must be exercised by some worked resolution — the requirement that
/// current/selected, read-only/disabled, locked/disabled, and pending/loading are all explained.
fn validate_distinction_coverage(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let exercised: BTreeSet<M5ConfusableStateDistinction> = packet
        .rows
        .iter()
        .flat_map(|row| row.explanation_examples.iter())
        .map(|case| case.resolved.distinction)
        .collect();
    let covered = M5ConfusableStateDistinction::ALL
        .iter()
        .all(|distinction| exercised.contains(distinction));
    if !covered {
        violations.push(M5StateExplanationViolation::DistinctionCoverageUnproven);
    }
}

/// Every delivery form must be exercised by some worked resolution.
fn validate_delivery_coverage(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let exercised: BTreeSet<M5ExplanationDelivery> = packet
        .rows
        .iter()
        .flat_map(|row| row.explanation_examples.iter())
        .map(|case| case.resolved.delivery)
        .collect();
    let covered = M5ExplanationDelivery::ALL
        .iter()
        .all(|delivery| exercised.contains(delivery));
    if !covered {
        violations.push(M5StateExplanationViolation::DeliveryCoverageUnproven);
    }
}

/// Every non-color cue must be exercised by some worked resolution — the acceptance criterion that
/// state meaning never depends on color alone.
fn validate_cue_coverage(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.explanation_examples.iter())
    };
    let covered = M5ExplanationCue::ALL
        .iter()
        .all(|cue| cases().any(|case| case.resolved.required_non_color_cues.contains(cue)));
    if !covered {
        violations.push(M5StateExplanationViolation::CueCoverageUnproven);
    }
}

/// Every required disclosure must be exercised by some worked resolution — the requirement that a
/// blocked / limited explanation always names its cause, owner / block reason, and recovery action.
fn validate_disclosure_coverage(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.explanation_examples.iter())
    };
    let covered = M5StateDisclosureTrigger::ALL
        .iter()
        .all(|trigger| cases().any(|case| case.resolved.required_disclosures.contains(trigger)));
    if !covered {
        violations.push(M5StateExplanationViolation::DisclosureCoverageUnproven);
    }
}

/// Every worked resolution must hold the stay-distinct, in-place, no-one-off-language,
/// taxonomy-alignment, blocked-action-alignment, no-color-only, and keyboard/screen-reader
/// guarantees — the core acceptance criteria.
fn validate_guarantees(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.explanation_examples.iter())
        .all(|case| case.preserves_guarantees());
    if !preserved {
        violations.push(M5StateExplanationViolation::GuaranteesUnproven);
    }
}

/// Every worked resolution must preserve its exact explanation identity, taxonomy, distinction copy,
/// and blocked/limited-copy reference — the invariant that the helper never rewrites what it
/// explains or discloses.
fn validate_identity_preservation(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.explanation_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5StateExplanationViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.surfaces_explain_state_semantics_in_place,
        review.current_and_selected_never_collapse,
        review.read_only_and_disabled_never_collapse,
        review.locked_and_disabled_never_collapse,
        review.pending_and_loading_never_collapse,
        review.no_one_off_language_invented,
        review.aligned_with_shared_taxonomy,
        review.contextual_teaching_aligned_with_component_truth,
        review.blocked_action_help_aligned_with_component_truth,
        review.state_meaning_never_color_only,
        review.explanations_keyboard_and_screen_reader_explainable,
        review.explanations_driven_by_shared_contract_and_tokens,
        review.no_one_off_per_surface_copy,
        review.explanations_stable_across_deployment_lines,
        review.explanations_stable_across_consumer_surfaces,
        review.every_surface_declares_accessibility_route,
        review.support_export_reconstructs_explanation_truth,
        review.later_rows_cannot_invent_parallel_state_vocabulary,
    ] {
        if !ok {
            violations.push(M5StateExplanationViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_state_vocabulary,
        projection.cue_set_reads_single_source,
        projection.disclosure_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5StateExplanationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5StateExplanationViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5StateExplanationPacket,
    violations: &mut Vec<M5StateExplanationViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.explanation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5StateExplanationViolation::ReleasePostureIncomplete);
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
