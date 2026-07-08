//! One reusable M5 design-system primitive — the loading / pending / warning-error / degraded
//! state-block contract — so every claimed M5 form, background job row, banner, card, dense row,
//! and review sheet renders its `Loading`, `Pending`, `Warning/Error`, and `Degraded` states the
//! same way, with the semantic distinctions the acceptance criteria demand: background work in
//! progress (`loading`) never reads the same as a user-submitted action awaiting commit
//! (`pending`), a warning never collapses into an error, and a hard error never collapses into a
//! reduced-capability degraded mode. Whenever a state reflects a user-submitted action, a
//! background health regression, or a scoped degraded mode, the contract preserves its submission
//! lineage, names what still works, and names the next safe action instead of a silent, color-only
//! spinner or a generic error toast.
//!
//! Aureline's frozen shared-component-state-taxonomy component matrix
//! ([`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`])
//! names the degraded-state-application contract as one of its four governed component-state
//! families and freezes its controlled vocabulary — the degraded subset of the shared taxonomy
//! (`loading`, `pending`, `warning_error`, `degraded`), the recovery-disclosure classes it can name
//! (`names_consequence`, `names_recovery_action`, `names_freshness`, `names_retry_path`,
//! `names_fallback_scope`, `no_recovery_available`), the state cause classes it can name, plus the
//! surface families, deployment lines, consumer surfaces, non-visual accessibility routes,
//! mandatory labels, qualification classes, and downgrade triggers. This module *implements* that
//! contract as one reusable resolver so a user — reading a form, a job row, a banner, a card, a
//! dense row, or a review sheet, on the desktop or through the support export and screen reader
//! alike — always gets the same explicit loading / pending / warning-error / degraded behavior,
//! instead of one-off spinner-and-toast accidents on individual surfaces.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_degraded_state_application_contract`] — takes one block's kind, the degraded state
//!    it is entering (one of `loading`, `pending`, `warning_error`, `degraded`), the severity that
//!    distinguishes a warning from an error, the recovery-disclosure class and state cause behind
//!    the state, whether a recovery path is available, whether a degraded block retains partial
//!    capability, the high-contrast context, its opaque stable block identity, the opaque shared
//!    state-style token reference that renders it, the opaque submission-lineage reference that
//!    attributes a pending action to the user action that triggered it, and the opaque
//!    consequence / recovery disclosure reference, and produces one
//!    [`M5ResolvedDegradedStateContract`] carrying the derived presentation posture (loading /
//!    pending / warning-error / degraded treatment), the required non-color cues that carry the
//!    state beyond hue, the required disclosures the state must publish (state cause, owner / block
//!    reason, recovery action), and the hard guarantees that `loading` and `pending` never
//!    collapse, `warning` and `error` never collapse, `error` and `degraded` never collapse, a
//!    pending action never masquerades as generic background loading, submission lineage and
//!    what-still-works are preserved, and the state stays keyboard- and screen-reader-explainable.
//!    It refuses to model a submitted pending action with no submission lineage, refuses a
//!    background loading state that falsely claims a user submission, refuses a warning/error state
//!    that has not decided whether it is a warning or an error, refuses a degraded state that has
//!    lost its what-still-works truth, and refuses an explainable state with no consequence /
//!    recovery detail.
//!
//! A single parity matrix — [`M5DegradedStateContractPacket`] — binds one row per claimed M5
//! workflow block (the form, the background job row, the banner, the card, the dense row, and the
//! review sheet) to the shared degraded-state anatomy, the same degraded states, presentation
//! postures, severities, non-color cues, required disclosures, recovery-disclosure classes, state
//! cause classes, export fields, mandatory labels, and non-visual accessibility routes, so the
//! loading / pending / warning-error / degraded vocabulary and its submission-lineage,
//! what-still-works, and next-safe-action rules stay identical across desktop, headless/export, and
//! support consumers.
//!
//! The degraded state class ([`M5SharedComponentStateClass`]), the recovery-disclosure class
//! ([`M5RecoveryDisclosureClass`]), the state cause class ([`M5StateCauseClass`]), the state
//! disclosure trigger ([`M5StateDisclosureTrigger`]), the surface family
//! ([`M5ComponentStateSurfaceFamily`]), the deployment line
//! ([`M5ComponentStateDeploymentLine`]), the consumer surface
//! ([`M5ComponentStateConsumerSurface`]), the accessibility route
//! ([`M5ComponentStateAccessibilityRoute`]), the required label
//! ([`M5ComponentStateRequiredLabel`]), the qualification class
//! ([`M5ComponentStateQualificationClass`]), and the downgrade trigger
//! ([`M5ComponentStateDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the degraded-state rendering
//! itself: its claimed block kinds, its anatomy parts, its derived presentation posture, its
//! warning-vs-error severity, its non-color cues, and its export fields. No M5 workflow block
//! invents a second degraded-state grammar.
//!
//! Raw local paths, credentials, and private endpoints stay outside the export boundary; every
//! block identity, state-style token reference, submission-lineage reference, and consequence /
//! recovery disclosure reference is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_degraded_state_contract_banner_beta_narrowed,
    seeded_m5_degraded_state_contract_packet,
    seeded_m5_degraded_state_contract_review_sheet_preview_narrowed,
    M5_DEGRADED_STATE_CONTRACT_PACKET_ID,
};

// The degraded state class, recovery-disclosure class, state cause class, state disclosure trigger,
// surface family, deployment line, consumer surface, accessibility route, required label,
// qualification class, and downgrade triggers are frozen once, in the shared-component-state-taxonomy
// component matrix. This primitive reuses them verbatim so it never invents a parallel
// degraded-state vocabulary.
pub use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5ComponentStateAccessibilityRoute, M5ComponentStateConsumerSurface,
    M5ComponentStateDeploymentLine, M5ComponentStateDowngradeTrigger,
    M5ComponentStateQualificationClass, M5ComponentStateRequiredLabel,
    M5ComponentStateSurfaceFamily, M5RecoveryDisclosureClass, M5SharedComponentStateClass,
    M5SharedComponentStateFamily, M5StateCauseClass, M5StateDisclosureTrigger,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DegradedStateContractPacket`].
pub const M5_DEGRADED_STATE_CONTRACT_RECORD_KIND: &str =
    "implement_m5_loading_pending_warning_error_and_degraded_state_blocks_with_submission_lineage_health_and_recovery_truth_across_claimed_m5_workflows";

/// Schema version for M5 degraded-state-contract records.
pub const M5_DEGRADED_STATE_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the degraded-state-contract boundary schema.
pub const M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF: &str =
    "schemas/ui/m5-loading-pending-degraded-state-contract.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DEGRADED_STATE_CONTRACT_DOC_REF: &str =
    "docs/design-system/m5_loading_pending_degraded_state_contract_primitive.md";

/// Repo-relative path of the frozen shared-component-state-taxonomy component matrix this primitive
/// narrows from.
pub const M5_DEGRADED_STATE_CONTRACT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json";

/// Repo-relative path of the service-health contract the `warning_error` and `degraded` states bind
/// their background-health-regression posture against.
pub const M5_DEGRADED_STATE_CONTRACT_SERVICE_HEALTH_REF: &str =
    "schemas/ops/service_health_card.schema.json";

/// Repo-relative path of the state-class recovery contract the explainable states bind their
/// consequence / recovery disclosure against.
pub const M5_DEGRADED_STATE_CONTRACT_STATE_RECOVERY_REF: &str =
    "schemas/state/state_class_recovery.schema.json";

/// Repo-relative path of the activity-event contract the `pending` state binds its
/// submission-lineage and activity-center attribution against.
pub const M5_DEGRADED_STATE_CONTRACT_ACTIVITY_ROW_REF: &str =
    "schemas/ux/activity_event_row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DEGRADED_STATE_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-loading-pending-degraded-state-contract-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DEGRADED_STATE_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/release/m5-loading-pending-degraded-state-contract-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DEGRADED_STATE_CONTRACT_CSV_REF: &str =
    "artifacts/release/m5-loading-pending-degraded-state-contract-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DEGRADED_STATE_CONTRACT_REPORT_REF: &str =
    "artifacts/design/m5-loading-pending-degraded-state-contract-primitive.md";

/// One claimed M5 workflow block that renders the shared degraded-state contract. These are the
/// blocks the implementation requirements name — forms, background job rows, banners, cards, dense
/// rows, and review sheets — so the same loading / pending / warning-error / degraded grammar works
/// across every claimed workflow surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStateBlockKind {
    /// A form that submits a user action.
    Form,
    /// A background job / task row.
    JobRow,
    /// A status / notice banner.
    Banner,
    /// A summary or content card.
    Card,
    /// A dense collection row.
    Row,
    /// A review / approval sheet.
    ReviewSheet,
}

impl M5DegradedStateBlockKind {
    /// Every claimed block kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Form,
        Self::JobRow,
        Self::Banner,
        Self::Card,
        Self::Row,
        Self::ReviewSheet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Form => "form",
            Self::JobRow => "job_row",
            Self::Banner => "banner",
            Self::Card => "card",
            Self::Row => "row",
            Self::ReviewSheet => "review_sheet",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Form => "Form",
            Self::JobRow => "Background Job Row",
            Self::Banner => "Banner",
            Self::Card => "Card",
            Self::Row => "Dense Row",
            Self::ReviewSheet => "Review Sheet",
        }
    }
}

/// The derived presentation posture of a degraded state — the resolver's verdict about how a
/// block's `loading`, `pending`, `warning_error`, or `degraded` state is rendered. Derived
/// one-to-one from the state so no degraded state collapses into another: background work in
/// progress is always distinguishable from a user-submitted action awaiting commit, and a hard
/// error is always distinguishable from a reduced-capability degraded mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStatePresentation {
    /// The background-work-in-progress loading treatment.
    LoadingTreatment,
    /// The user-submitted-action-awaiting-commit pending treatment.
    PendingTreatment,
    /// The warning / error treatment that names its consequence and recovery.
    WarningErrorTreatment,
    /// The reduced-capability degraded treatment that names what still works.
    DegradedTreatment,
}

impl M5DegradedStatePresentation {
    /// Every presentation posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LoadingTreatment,
        Self::PendingTreatment,
        Self::WarningErrorTreatment,
        Self::DegradedTreatment,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadingTreatment => "loading_treatment",
            Self::PendingTreatment => "pending_treatment",
            Self::WarningErrorTreatment => "warning_error_treatment",
            Self::DegradedTreatment => "degraded_treatment",
        }
    }

    /// The presentation posture for one degraded state, or `None` when the state is not one of the
    /// four governed degraded states.
    pub const fn from_state(state: M5SharedComponentStateClass) -> Option<Self> {
        match state {
            M5SharedComponentStateClass::Loading => Some(Self::LoadingTreatment),
            M5SharedComponentStateClass::Pending => Some(Self::PendingTreatment),
            M5SharedComponentStateClass::WarningError => Some(Self::WarningErrorTreatment),
            M5SharedComponentStateClass::Degraded => Some(Self::DegradedTreatment),
            _ => None,
        }
    }

    /// True when this posture is one of the explainable postures — warning-error or degraded — so
    /// its consequence / recovery detail must be surfaced.
    pub const fn is_explainable(self) -> bool {
        matches!(self, Self::WarningErrorTreatment | Self::DegradedTreatment)
    }
}

/// The severity that resolves a `warning_error` state into a distinct warning or error, and pins
/// the loading / pending states to `informational` and the degraded state to `reduced`, so a
/// warning never reads as an error and an error never reads as a reduced-capability degraded mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStateSeverity {
    /// No warning / error severity — background work or a submitted action in flight.
    Informational,
    /// A warning worth surfacing, but the action can still proceed.
    Warning,
    /// A hard error that blocked the action.
    Error,
    /// A reduced-capability degraded mode.
    Reduced,
}

impl M5DegradedStateSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Informational,
        Self::Warning,
        Self::Error,
        Self::Reduced,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Reduced => "reduced",
        }
    }
}

/// One non-color cue a degraded state renders so its meaning is never carried by hue alone. Every
/// derived presentation posture publishes at least one of these, enforcing the no-color-only
/// signaling rule and keeping the loading / pending, warning / error, and error / degraded
/// distinctions legible without color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStateCue {
    /// A determinate/indeterminate progress indicator carries the background-loading state.
    LoadingProgressIndicator,
    /// An explicit "your action is in flight" attribution carries the pending state, distinct from a
    /// generic loading spinner.
    PendingSubmissionAttribution,
    /// A warning glyph paired with the named consequence carries a warning-severity state.
    WarningConsequenceGlyph,
    /// An error glyph paired with the named consequence carries an error-severity state.
    ErrorConsequenceGlyph,
    /// A reduced-capability glyph paired with the named what-still-works carries the degraded state.
    DegradedReducedCapabilityGlyph,
    /// A recovery affordance names the next safe action out of an explainable state.
    RecoveryAffordance,
}

impl M5DegradedStateCue {
    /// Every non-color cue, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LoadingProgressIndicator,
        Self::PendingSubmissionAttribution,
        Self::WarningConsequenceGlyph,
        Self::ErrorConsequenceGlyph,
        Self::DegradedReducedCapabilityGlyph,
        Self::RecoveryAffordance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadingProgressIndicator => "loading_progress_indicator",
            Self::PendingSubmissionAttribution => "pending_submission_attribution",
            Self::WarningConsequenceGlyph => "warning_consequence_glyph",
            Self::ErrorConsequenceGlyph => "error_consequence_glyph",
            Self::DegradedReducedCapabilityGlyph => "degraded_reduced_capability_glyph",
            Self::RecoveryAffordance => "recovery_affordance",
        }
    }
}

/// Controlled degraded-state anatomy part the shared contract surfaces. The parts in
/// [`M5DegradedStateAnatomyPart::MANDATORY`] are required on every block so the state identity, the
/// presentation posture, the non-color cue set, the state cause, and the non-visual keyboard route
/// are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStateAnatomyPart {
    /// The typed state identity cue.
    StateIdentityCue,
    /// The derived presentation-posture cue.
    PresentationPostureCue,
    /// The non-color cue-set cue.
    NonColorCueSetCue,
    /// The state-cause cue (why the state applies).
    StateCauseCue,
    /// The submission-lineage cue (which user action a pending state belongs to).
    SubmissionLineageCue,
    /// The what-still-works cue (the partial capability a degraded state retains).
    WhatStillWorksCue,
    /// The recovery-action cue (the next safe action out of the state).
    RecoveryActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5DegradedStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StateIdentityCue,
        Self::PresentationPostureCue,
        Self::NonColorCueSetCue,
        Self::StateCauseCue,
        Self::SubmissionLineageCue,
        Self::WhatStillWorksCue,
        Self::RecoveryActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every block must render.
    pub const MANDATORY: [Self; 5] = [
        Self::StateIdentityCue,
        Self::PresentationPostureCue,
        Self::NonColorCueSetCue,
        Self::StateCauseCue,
        Self::KeyboardRouteCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateIdentityCue => "state_identity_cue",
            Self::PresentationPostureCue => "presentation_posture_cue",
            Self::NonColorCueSetCue => "non_color_cue_set_cue",
            Self::StateCauseCue => "state_cause_cue",
            Self::SubmissionLineageCue => "submission_lineage_cue",
            Self::WhatStillWorksCue => "what_still_works_cue",
            Self::RecoveryActionCue => "recovery_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the degraded-state export carries so its truth is reconstructable. The fields in
/// [`M5DegradedStateExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStateExportField {
    /// The block kind.
    BlockKind,
    /// The degraded state.
    DegradedState,
    /// The derived presentation posture.
    Presentation,
    /// The warning-vs-error severity.
    Severity,
    /// The required non-color cues.
    NonColorCues,
    /// The state cause.
    StateCause,
    /// The submission-lineage reference (for a pending state).
    SubmissionLineage,
    /// Whether a recovery path is available.
    RecoveryAvailable,
}

impl M5DegradedStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BlockKind,
        Self::DegradedState,
        Self::Presentation,
        Self::Severity,
        Self::NonColorCues,
        Self::StateCause,
        Self::SubmissionLineage,
        Self::RecoveryAvailable,
    ];

    /// The export fields every block must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::BlockKind,
        Self::DegradedState,
        Self::Presentation,
        Self::NonColorCues,
        Self::StateCause,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockKind => "block_kind",
            Self::DegradedState => "degraded_state",
            Self::Presentation => "presentation",
            Self::Severity => "severity",
            Self::NonColorCues => "non_color_cues",
            Self::StateCause => "state_cause",
            Self::SubmissionLineage => "submission_lineage",
            Self::RecoveryAvailable => "recovery_available",
        }
    }
}

/// The four governed degraded states, in the frozen taxonomy's declaration order. Reused from the
/// degraded-state-application family's canonical partition of the shared taxonomy so this primitive
/// never re-lists a private degraded-state set.
pub fn degraded_states() -> Vec<M5SharedComponentStateClass> {
    M5SharedComponentStateFamily::DegradedStateApplication
        .governed_states()
        .to_vec()
}

// ---- degraded-state resolver --------------------------------------------

/// The full input to the degraded-state-application-contract resolver for one block state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateResolutionInput {
    /// The claimed block kind.
    pub block_kind: M5DegradedStateBlockKind,
    /// The degraded state the block is entering (one of the four governed states).
    pub degraded_state: M5SharedComponentStateClass,
    /// The severity distinguishing a warning from an error (and pinning loading/pending to
    /// `informational` and degraded to `reduced`).
    pub severity: M5DegradedStateSeverity,
    /// The recovery-disclosure class this state names.
    pub recovery_class: M5RecoveryDisclosureClass,
    /// The cause of the state (why it applies).
    pub state_cause: M5StateCauseClass,
    /// True when a recovery path out of the state is available.
    pub recovery_available: bool,
    /// True when a degraded block retains partial capability (required for a degraded state).
    pub retains_partial_capability: bool,
    /// True when a high-contrast mode is active, so the state stays legible without hue.
    pub high_contrast_active: bool,
    /// The opaque stable block identity (must be non-empty).
    pub block_identity_ref: String,
    /// The opaque shared state-style token reference that renders this state (must be non-empty).
    pub state_style_ref: String,
    /// The opaque submission-lineage reference attributing a pending action to the user action that
    /// triggered it (must be non-empty for a pending state, and must be empty for a background
    /// loading state).
    pub submission_lineage_ref: String,
    /// The opaque consequence / recovery disclosure reference (must be non-empty when the state is
    /// explainable: warning-error or degraded).
    pub disclosure_ref: String,
}

/// The resolved degraded-state-application-contract truth for one block state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDegradedStateContract {
    /// The block kind.
    pub block_kind: M5DegradedStateBlockKind,
    /// The degraded state.
    pub degraded_state: M5SharedComponentStateClass,
    /// The derived presentation posture.
    pub presentation: M5DegradedStatePresentation,
    /// The severity, preserved exactly from the input.
    pub severity: M5DegradedStateSeverity,
    /// The required non-color cues that carry this state beyond hue.
    pub required_non_color_cues: Vec<M5DegradedStateCue>,
    /// The disclosures this state must publish (state cause, owner / block reason, recovery action,
    /// and never a silent style-only change).
    pub required_disclosures: Vec<M5StateDisclosureTrigger>,
    /// The recovery-disclosure class behind the state, preserved exactly from the input.
    pub recovery_class: M5RecoveryDisclosureClass,
    /// The cause of the state, preserved exactly from the input.
    pub state_cause: M5StateCauseClass,
    /// True when a recovery path is available, preserved from the input.
    pub recovery_available: bool,
    /// True when a degraded block retains partial capability, preserved from the input.
    pub retains_partial_capability: bool,
    /// True when high-contrast is active, preserved from the input.
    pub high_contrast_active: bool,
    /// The opaque stable block identity, preserved exactly from the input.
    pub block_identity_ref: String,
    /// The opaque shared state-style token reference, preserved exactly from the input.
    pub state_style_ref: String,
    /// The opaque submission-lineage reference, preserved exactly from the input.
    pub submission_lineage_ref: String,
    /// The opaque consequence / recovery disclosure reference, preserved exactly from the input.
    pub disclosure_ref: String,
    /// True when this state is explainable (warning-error or degraded) and therefore must surface
    /// its consequence / recovery detail.
    pub explainable: bool,
    /// True when this state reflects a user-submitted action (the pending state), attributed to its
    /// submission lineage.
    pub user_submitted: bool,
    /// `loading` and `pending` never collapse into one another. ALWAYS `true`.
    pub loading_and_pending_stay_distinct: bool,
    /// `warning` and `error` never collapse into one another. ALWAYS `true`.
    pub warning_and_error_stay_distinct: bool,
    /// `error` and `degraded` never collapse into one another. ALWAYS `true`.
    pub error_and_degraded_stay_distinct: bool,
    /// A pending action never masquerades as generic background loading. ALWAYS `true`.
    pub pending_never_masquerades_as_loading: bool,
    /// Whenever a state is explainable, its consequence / recovery detail is surfaced. ALWAYS
    /// `true`.
    pub names_consequence_and_recovery_when_explainable: bool,
    /// Submission lineage and what-still-works are preserved. ALWAYS `true`.
    pub preserves_submission_lineage_and_capability: bool,
    /// State meaning is never carried by color alone. ALWAYS `true`.
    pub no_color_only_signaling: bool,
    /// The state stays keyboard- and screen-reader-explainable. ALWAYS `true`.
    pub keyboard_and_screen_reader_explainable: bool,
    /// The state semantics are driven by the shared contract and its token hooks, not a one-off
    /// implementation choice. ALWAYS `true`.
    pub driven_by_shared_state_contract: bool,
}

/// Errors returned by [`resolve_degraded_state_application_contract`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DegradedStateResolutionError {
    /// The block identity ref was empty.
    EmptyBlockIdentity,
    /// The state-style token ref was empty.
    EmptyStateStyleRef,
    /// The state was not one of the four governed degraded states.
    NonDegradedState,
    /// A pending state named no submission lineage, so the submitted action would not be
    /// attributable.
    PendingWithoutSubmissionLineage,
    /// A background loading state falsely claimed a submission lineage, masquerading as a
    /// user-submitted pending action.
    LoadingWithSubmissionLineage,
    /// A warning-error state did not decide whether it is a warning or an error.
    WarningErrorSeverityUnset,
    /// A state carried a severity that does not match its degraded state.
    SeverityStateMismatch,
    /// A degraded state lost its what-still-works partial-capability truth.
    DegradedWithoutPartialCapability,
    /// An explainable state carried no consequence / recovery disclosure detail.
    MissingDisclosureDetail,
    /// A descriptor carried forbidden material.
    ForbiddenStateMaterial,
}

impl M5DegradedStateResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBlockIdentity => "empty_block_identity",
            Self::EmptyStateStyleRef => "empty_state_style_ref",
            Self::NonDegradedState => "non_degraded_state",
            Self::PendingWithoutSubmissionLineage => "pending_without_submission_lineage",
            Self::LoadingWithSubmissionLineage => "loading_with_submission_lineage",
            Self::WarningErrorSeverityUnset => "warning_error_severity_unset",
            Self::SeverityStateMismatch => "severity_state_mismatch",
            Self::DegradedWithoutPartialCapability => "degraded_without_partial_capability",
            Self::MissingDisclosureDetail => "missing_disclosure_detail",
            Self::ForbiddenStateMaterial => "forbidden_state_material",
        }
    }
}

impl fmt::Display for M5DegradedStateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "degraded state application contract resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DegradedStateResolutionError {}

/// Resolves one degraded-state-application contract from a block's kind, the degraded state it is
/// entering, and the severity / recovery / submission-lineage context behind it.
///
/// The presentation posture is derived one-to-one from the state so no state collapses into
/// another: `loading` renders the background-work treatment, `pending` renders the
/// submitted-action-awaiting-commit treatment, `warning_error` renders the warning / error
/// treatment, and `degraded` renders the reduced-capability treatment. Each posture publishes a
/// non-empty non-color cue set so the state is never carried by color alone, and a
/// required-disclosure set so an explainable state always names its cause, its owner / block reason,
/// and its recovery action. The resolver refuses a pending state that names no submission lineage,
/// refuses a background loading state that falsely claims a submission (which would masquerade as
/// pending), refuses a warning-error state that has not decided whether it is a warning or an error,
/// refuses a degraded state that has lost its what-still-works truth, and refuses an explainable
/// state that carries no consequence / recovery detail.
pub fn resolve_degraded_state_application_contract(
    input: &M5DegradedStateResolutionInput,
) -> Result<M5ResolvedDegradedStateContract, M5DegradedStateResolutionError> {
    if input.block_identity_ref.trim().is_empty() {
        return Err(M5DegradedStateResolutionError::EmptyBlockIdentity);
    }
    if input.state_style_ref.trim().is_empty() {
        return Err(M5DegradedStateResolutionError::EmptyStateStyleRef);
    }
    if value_repr_is_forbidden(&input.block_identity_ref)
        || value_repr_is_forbidden(&input.state_style_ref)
        || value_repr_is_forbidden(&input.submission_lineage_ref)
        || value_repr_is_forbidden(&input.disclosure_ref)
    {
        return Err(M5DegradedStateResolutionError::ForbiddenStateMaterial);
    }

    let presentation = M5DegradedStatePresentation::from_state(input.degraded_state)
        .ok_or(M5DegradedStateResolutionError::NonDegradedState)?;

    let state = input.degraded_state;

    // The severity distinguishes a warning from an error, and pins loading/pending to
    // `informational` and degraded to `reduced`, so warning, error, and degraded never collapse.
    match state {
        M5SharedComponentStateClass::WarningError => {
            if !matches!(
                input.severity,
                M5DegradedStateSeverity::Warning | M5DegradedStateSeverity::Error
            ) {
                return Err(M5DegradedStateResolutionError::WarningErrorSeverityUnset);
            }
        }
        M5SharedComponentStateClass::Loading | M5SharedComponentStateClass::Pending => {
            if input.severity != M5DegradedStateSeverity::Informational {
                return Err(M5DegradedStateResolutionError::SeverityStateMismatch);
            }
        }
        M5SharedComponentStateClass::Degraded => {
            if input.severity != M5DegradedStateSeverity::Reduced {
                return Err(M5DegradedStateResolutionError::SeverityStateMismatch);
            }
        }
        _ => {}
    }

    let has_lineage = !input.submission_lineage_ref.trim().is_empty();
    // A pending action is attributable to the user action that triggered it, and a background
    // loading state never claims a user submission — so pending never masquerades as loading.
    if state == M5SharedComponentStateClass::Pending && !has_lineage {
        return Err(M5DegradedStateResolutionError::PendingWithoutSubmissionLineage);
    }
    if state == M5SharedComponentStateClass::Loading && has_lineage {
        return Err(M5DegradedStateResolutionError::LoadingWithSubmissionLineage);
    }
    // A degraded state preserves its what-still-works partial capability.
    if state == M5SharedComponentStateClass::Degraded && !input.retains_partial_capability {
        return Err(M5DegradedStateResolutionError::DegradedWithoutPartialCapability);
    }
    // An explainable state always carries consequence / recovery detail.
    if presentation.is_explainable() && input.disclosure_ref.trim().is_empty() {
        return Err(M5DegradedStateResolutionError::MissingDisclosureDetail);
    }

    let required_non_color_cues = derive_non_color_cues(presentation, input.severity);
    let required_disclosures = derive_required_disclosures(presentation);
    let user_submitted = state == M5SharedComponentStateClass::Pending;

    Ok(M5ResolvedDegradedStateContract {
        block_kind: input.block_kind,
        degraded_state: state,
        presentation,
        severity: input.severity,
        required_non_color_cues,
        required_disclosures,
        recovery_class: input.recovery_class,
        state_cause: input.state_cause,
        recovery_available: input.recovery_available,
        retains_partial_capability: input.retains_partial_capability,
        high_contrast_active: input.high_contrast_active,
        block_identity_ref: input.block_identity_ref.clone(),
        state_style_ref: input.state_style_ref.clone(),
        submission_lineage_ref: input.submission_lineage_ref.clone(),
        disclosure_ref: input.disclosure_ref.clone(),
        explainable: presentation.is_explainable(),
        user_submitted,
        // The acceptance criteria: loading and pending never collapse, warning and error never
        // collapse, error and degraded never collapse, pending never masquerades as loading,
        // consequence / recovery is surfaced when explainable, submission lineage and
        // what-still-works are preserved, the state is never color-only, the state stays keyboard-
        // and screen-reader-explainable, and the semantics are driven by the shared contract.
        loading_and_pending_stay_distinct: true,
        warning_and_error_stay_distinct: true,
        error_and_degraded_stay_distinct: true,
        pending_never_masquerades_as_loading: true,
        names_consequence_and_recovery_when_explainable: true,
        preserves_submission_lineage_and_capability: true,
        no_color_only_signaling: true,
        keyboard_and_screen_reader_explainable: true,
        driven_by_shared_state_contract: true,
    })
}

/// Derives the non-color cue set for a presentation posture. Every posture publishes at least one
/// non-color cue, so state meaning is never carried by hue alone; the warning-error posture picks
/// its glyph from the severity so a warning and an error never share a cue, and every explainable
/// posture additionally publishes a recovery affordance.
fn derive_non_color_cues(
    presentation: M5DegradedStatePresentation,
    severity: M5DegradedStateSeverity,
) -> Vec<M5DegradedStateCue> {
    use M5DegradedStateCue as Cue;
    use M5DegradedStatePresentation as Posture;

    match presentation {
        Posture::LoadingTreatment => vec![Cue::LoadingProgressIndicator],
        Posture::PendingTreatment => vec![Cue::PendingSubmissionAttribution],
        Posture::WarningErrorTreatment => {
            let glyph = if matches!(severity, M5DegradedStateSeverity::Error) {
                Cue::ErrorConsequenceGlyph
            } else {
                Cue::WarningConsequenceGlyph
            };
            vec![glyph, Cue::RecoveryAffordance]
        }
        Posture::DegradedTreatment => {
            vec![Cue::DegradedReducedCapabilityGlyph, Cue::RecoveryAffordance]
        }
    }
}

/// Derives the required-disclosure set for a presentation posture. Every posture forbids a silent
/// style-only change; the pending posture requires the state cause (the submitted action); every
/// explainable posture additionally requires the state cause and the recovery action; the
/// warning-error posture requires the owner and the block reason.
fn derive_required_disclosures(
    presentation: M5DegradedStatePresentation,
) -> Vec<M5StateDisclosureTrigger> {
    use M5DegradedStatePresentation as Posture;
    use M5StateDisclosureTrigger as Trigger;

    match presentation {
        Posture::LoadingTreatment => vec![Trigger::SilentStyleOnlyForbidden],
        Posture::PendingTreatment => {
            vec![
                Trigger::StateCauseRequired,
                Trigger::SilentStyleOnlyForbidden,
            ]
        }
        Posture::WarningErrorTreatment => vec![
            Trigger::StateCauseRequired,
            Trigger::OwnerRequired,
            Trigger::BlockReasonRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
        Posture::DegradedTreatment => vec![
            Trigger::StateCauseRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked degraded-state resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateResolutionCase {
    /// The resolver input.
    pub input: M5DegradedStateResolutionInput,
    /// The resolved truth. Must equal `resolve_degraded_state_application_contract(&input)`.
    pub resolved: M5ResolvedDegradedStateContract,
}

impl M5DegradedStateResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DegradedStateResolutionInput) -> Self {
        let resolved = resolve_degraded_state_application_contract(&input)
            .expect("seed degraded state contract case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_degraded_state_application_contract(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input block identity, state-style reference,
    /// submission-lineage reference, and disclosure reference exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.block_identity_ref == self.input.block_identity_ref
            && self.resolved.state_style_ref == self.input.state_style_ref
            && self.resolved.submission_lineage_ref == self.input.submission_lineage_ref
            && self.resolved.disclosure_ref == self.input.disclosure_ref
    }

    /// True when the resolved case keeps loading/pending, warning/error, and error/degraded
    /// distinct, never lets a pending action read as loading, preserves submission lineage and
    /// what-still-works, never signals by color alone, names consequence / recovery when
    /// explainable, stays keyboard- and screen-reader-explainable, and is driven by the shared
    /// contract.
    pub fn preserves_guarantees(&self) -> bool {
        !self.resolved.required_non_color_cues.is_empty()
            && !self.resolved.required_disclosures.is_empty()
            && self.resolved.loading_and_pending_stay_distinct
            && self.resolved.warning_and_error_stay_distinct
            && self.resolved.error_and_degraded_stay_distinct
            && self.resolved.pending_never_masquerades_as_loading
            && self
                .resolved
                .names_consequence_and_recovery_when_explainable
            && self.resolved.preserves_submission_lineage_and_capability
            && self.resolved.no_color_only_signaling
            && self.resolved.keyboard_and_screen_reader_explainable
            && self.resolved.driven_by_shared_state_contract
    }
}

/// One row in the primitive matrix: one claimed M5 workflow block bound to the shared
/// degraded-state anatomy, degraded states, presentation postures, severities, non-color cues,
/// required disclosures, recovery-disclosure classes, state cause classes, export fields, mandatory
/// labels, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateBlockRow {
    /// Claimed block kind.
    pub block_kind: M5DegradedStateBlockKind,
    /// Qualification class earned by this block.
    pub qualification: M5ComponentStateQualificationClass,
    /// Owner role accountable for keeping this block governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this block.
    pub surface_families: Vec<M5ComponentStateSurfaceFamily>,
    /// Deployment lines this block keeps the same truth across.
    pub deployment_lines: Vec<M5ComponentStateDeploymentLine>,
    /// Anatomy parts this block renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5DegradedStateAnatomyPart>,
    /// Degraded states this block distinguishes.
    pub degraded_states: Vec<M5SharedComponentStateClass>,
    /// Presentation postures this block distinguishes.
    pub presentations: Vec<M5DegradedStatePresentation>,
    /// Severities this block distinguishes.
    pub severities: Vec<M5DegradedStateSeverity>,
    /// Non-color cues this block renders.
    pub non_color_cues: Vec<M5DegradedStateCue>,
    /// Required disclosures this block publishes.
    pub required_disclosures: Vec<M5StateDisclosureTrigger>,
    /// Recovery-disclosure classes this block can name behind an explainable state.
    pub recovery_disclosure_classes: Vec<M5RecoveryDisclosureClass>,
    /// State cause classes this block can name behind a non-default state.
    pub state_cause_classes: Vec<M5StateCauseClass>,
    /// Export fields this block carries (must include the mandatory fields).
    pub export_fields: Vec<M5DegradedStateExportField>,
    /// Non-visual accessibility routes this block offers.
    pub accessibility_routes: Vec<M5ComponentStateAccessibilityRoute>,
    /// Mandatory labels this block can show (must include the mandatory labels).
    pub required_labels: Vec<M5ComponentStateRequiredLabel>,
    /// Subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Downgrade triggers that apply to this block.
    pub downgrade_triggers: Vec<M5ComponentStateDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked degraded-state resolutions proving the resolver on this block.
    pub state_examples: Vec<M5DegradedStateResolutionCase>,
    /// Hard invariant: this block never presents `pending` as generic `loading`. MUST be `false`.
    pub presents_pending_as_generic_loading: bool,
    /// Hard invariant: this block never collapses `warning` and `error`. MUST be `false`.
    pub collapses_warning_and_error: bool,
    /// Hard invariant: this block never omits consequence or recovery on an explainable state. MUST
    /// be `false`.
    pub omits_consequence_or_recovery: bool,
    /// Hard invariant: this block never invents a private state name. MUST be `false`.
    pub invents_private_state_name: bool,
}

impl M5DegradedStateBlockRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DegradedStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DegradedStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5DegradedStateExportField> =
            self.export_fields.iter().copied().collect();
        M5DegradedStateExportField::MANDATORY
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
        !self.presents_pending_as_generic_loading
            && !self.collapses_warning_and_error
            && !self.omits_consequence_or_recovery
            && !self.invents_private_state_name
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateVocabularySet {
    /// Block-kind tokens.
    pub block_kinds: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Presentation-posture tokens.
    pub presentations: Vec<String>,
    /// Severity tokens.
    pub severities: Vec<String>,
    /// Non-color-cue tokens.
    pub non_color_cues: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Degraded-state tokens (reused from the frozen matrix).
    pub degraded_states: Vec<String>,
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
    /// Consumer-surface tokens (reused from the frozen matrix).
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens (reused from the frozen matrix).
    pub required_labels: Vec<String>,
}

impl M5DegradedStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            block_kinds: tokens(&M5DegradedStateBlockKind::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DegradedStateAnatomyPart::ALL, |v| v.as_str()),
            presentations: tokens(&M5DegradedStatePresentation::ALL, |v| v.as_str()),
            severities: tokens(&M5DegradedStateSeverity::ALL, |v| v.as_str()),
            non_color_cues: tokens(&M5DegradedStateCue::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DegradedStateExportField::ALL, |v| v.as_str()),
            degraded_states: degraded_states()
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            required_disclosures: tokens(&M5StateDisclosureTrigger::ALL, |v| v.as_str()),
            recovery_disclosure_classes: tokens(&M5RecoveryDisclosureClass::ALL, |v| v.as_str()),
            state_cause_classes: tokens(&M5StateCauseClass::ALL, |v| v.as_str()),
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
pub struct M5DegradedStateGovernanceReview {
    /// Blocks distinguish loading, pending, warning/error, and degraded explicitly.
    pub blocks_distinguish_loading_pending_warning_error_degraded: bool,
    /// `loading` and `pending` never collapse into one another.
    pub loading_and_pending_never_collapse: bool,
    /// `warning` and `error` never collapse into one another.
    pub warning_and_error_never_collapse: bool,
    /// `error` and `degraded` never collapse into one another.
    pub error_and_degraded_never_collapse: bool,
    /// A pending action is attributed to the user action that triggered it.
    pub pending_attributed_to_user_action: bool,
    /// Consequence / recovery detail is surfaced whenever a state is explainable.
    pub consequence_and_recovery_surfaced_when_explainable: bool,
    /// Submission lineage and what-still-works are preserved.
    pub submission_lineage_and_capability_preserved: bool,
    /// State meaning is never carried by color alone.
    pub state_meaning_never_color_only: bool,
    /// States stay keyboard- and screen-reader-explainable.
    pub states_keyboard_and_screen_reader_explainable: bool,
    /// State semantics are driven by the shared contract and its token hooks.
    pub states_driven_by_shared_contract_and_tokens: bool,
    /// No block uses one-off, per-surface degraded-state styling.
    pub no_one_off_per_surface_styling: bool,
    /// Degraded states keep the same truth across every deployment line.
    pub states_stable_across_deployment_lines: bool,
    /// Degraded states keep the same truth across desktop, headless/export, and support consumers.
    pub states_stable_across_consumer_surfaces: bool,
    /// Every block declares a non-visual accessibility route.
    pub every_block_declares_accessibility_route: bool,
    /// The support / export packet reconstructs degraded-state truth for the activity center.
    pub support_export_reconstructs_state_truth: bool,
    /// Later M5 rows cannot invent parallel degraded-state vocabulary.
    pub later_rows_cannot_invent_parallel_state_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateConsumerProjection {
    /// Blocks consume the shared degraded-state vocabulary.
    pub blocks_consume_state_vocabulary: bool,
    /// The presentation-posture resolver reads a single canonical source.
    pub presentation_reads_single_source: bool,
    /// The required-disclosure derivation reads a single canonical source.
    pub disclosure_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop blocks read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the degraded-state contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting degraded-state audit.
    pub degraded_state_audit_ref: String,
    /// True when support / export parity is required for every block.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every block.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DegradedStateContractPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DegradedStateContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Block rows.
    pub rows: Vec<M5DegradedStateBlockRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DegradedStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DegradedStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DegradedStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DegradedStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DegradedStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 degraded-state-contract primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DegradedStateContractPacket {
    /// Record kind; must equal [`M5_DEGRADED_STATE_CONTRACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DEGRADED_STATE_CONTRACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Block rows.
    pub rows: Vec<M5DegradedStateBlockRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DegradedStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DegradedStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DegradedStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DegradedStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DegradedStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DegradedStateContractPacket {
    /// Builds an M5 degraded-state-contract-primitive packet from stable-lane input.
    pub fn new(input: M5DegradedStateContractPacketInput) -> Self {
        Self {
            record_kind: M5_DEGRADED_STATE_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: M5_DEGRADED_STATE_CONTRACT_SCHEMA_VERSION,
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

    /// Validates the M5 degraded-state-contract-primitive invariants.
    pub fn validate(&self) -> Vec<M5DegradedStateContractViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DEGRADED_STATE_CONTRACT_RECORD_KIND {
            violations.push(M5DegradedStateContractViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DEGRADED_STATE_CONTRACT_SCHEMA_VERSION {
            violations.push(M5DegradedStateContractViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DegradedStateContractViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_degraded_state_coverage(self, &mut violations);
        validate_presentation_coverage(self, &mut violations);
        validate_severity_coverage(self, &mut violations);
        validate_cue_coverage(self, &mut violations);
        validate_disclosure_coverage(self, &mut violations);
        validate_guarantees(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 degraded state contract primitive packet serializes"),
        ) {
            violations.push(M5DegradedStateContractViolation::RawMaterialInExport);
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
            .expect("m5 degraded state contract primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per block kind.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "block_kind,qualification,owner,anatomy,degraded_states,presentations,severities,non_color_cues,required_disclosures,state_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.block_kind.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_state_tokens(&row.degraded_states),
                join_tokens(&row.presentations, |v| v.as_str()),
                join_tokens(&row.severities, |v| v.as_str()),
                join_tokens(&row.non_color_cues, |v| v.as_str()),
                join_tokens(&row.required_disclosures, |v| v.as_str()),
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
        out.push_str(
            "# M5 Loading / Pending / Warning-Error / Degraded State-Block Contract Primitive\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Blocks: {} ({} stable)\n",
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
            "- Degraded states: {}\n",
            self.vocabulary_set.degraded_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Blocks\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.block_kind.label(),
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
                    "    - `{}` (`{}` / `{}`) → `{}` (non-color cues {}, submitted `{}`, explainable `{}`, recovery `{}`)\n",
                    case.resolved.block_identity_ref,
                    case.resolved.degraded_state.as_str(),
                    case.resolved.severity.as_str(),
                    case.resolved.presentation.as_str(),
                    case.resolved.required_non_color_cues.len(),
                    case.resolved.user_submitted,
                    case.resolved.explainable,
                    case.resolved.recovery_available,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 degraded-state-contract-primitive export.
#[derive(Debug)]
pub enum M5DegradedStateContractArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DegradedStateContractViolation>),
}

impl fmt::Display for M5DegradedStateContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 degraded state contract primitive export parse failed: {error}"
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
                    "m5 degraded state contract primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DegradedStateContractArtifactError {}

/// Validation failures emitted by [`M5DegradedStateContractPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DegradedStateContractViolation {
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
    /// A required block kind is missing from the matrix.
    RequiredBlockMissing,
    /// A block row is incomplete.
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
    StableBlockMissingProof,
    /// The worked resolutions do not exercise every degraded state.
    DegradedStateCoverageUnproven,
    /// The worked resolutions do not exercise every presentation posture.
    PresentationCoverageUnproven,
    /// The worked resolutions do not exercise every severity.
    SeverityCoverageUnproven,
    /// The worked resolutions do not exercise every non-color cue.
    CueCoverageUnproven,
    /// The worked resolutions do not exercise every required disclosure.
    DisclosureCoverageUnproven,
    /// A worked resolution does not hold the loading/pending, warning/error, error/degraded
    /// distinctness, submission-lineage, no-color-only, and keyboard/screen-reader guarantees.
    GuaranteesUnproven,
    /// A worked resolution does not preserve its exact block identity, state-style, submission
    /// lineage, and disclosure reference.
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

impl M5DegradedStateContractViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredBlockMissing => "required_block_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StateExampleMissing => "state_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableBlockMissingProof => "stable_block_missing_proof",
            Self::DegradedStateCoverageUnproven => "degraded_state_coverage_unproven",
            Self::PresentationCoverageUnproven => "presentation_coverage_unproven",
            Self::SeverityCoverageUnproven => "severity_coverage_unproven",
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

/// Reads and validates the checked-in stable M5 degraded-state-contract-primitive export.
pub fn current_stable_m5_degraded_state_contract_export(
) -> Result<M5DegradedStateContractPacket, M5DegradedStateContractArtifactError> {
    let packet: M5DegradedStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-loading-pending-degraded-state-contract-primitive-proof/support_export.json"
    )))
    .map_err(M5DegradedStateContractArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DegradedStateContractArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF,
        M5_DEGRADED_STATE_CONTRACT_DOC_REF,
        M5_DEGRADED_STATE_CONTRACT_COMPONENT_MATRIX_REF,
        M5_DEGRADED_STATE_CONTRACT_SERVICE_HEALTH_REF,
        M5_DEGRADED_STATE_CONTRACT_STATE_RECOVERY_REF,
        M5_DEGRADED_STATE_CONTRACT_ACTIVITY_ROW_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DegradedStateContractViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DegradedStateContractViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let present: BTreeSet<M5DegradedStateBlockKind> =
        packet.rows.iter().map(|row| row.block_kind).collect();
    for required in M5DegradedStateBlockKind::ALL {
        if !present.contains(&required) {
            violations.push(M5DegradedStateContractViolation::RequiredBlockMissing);
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
            || row.degraded_states.is_empty()
            || row.presentations.is_empty()
            || row.severities.is_empty()
            || row.non_color_cues.is_empty()
            || row.required_disclosures.is_empty()
            || row.recovery_disclosure_classes.is_empty()
            || row.state_cause_classes.is_empty()
            || row.export_fields.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5DegradedStateContractViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DegradedStateContractViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5DegradedStateContractViolation::MandatoryExportMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5DegradedStateContractViolation::MandatoryLabelMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5DegradedStateContractViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DegradedStateContractViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DegradedStateContractViolation::DowngradeTriggersMissing);
        }
        if row.state_examples.is_empty() {
            violations.push(M5DegradedStateContractViolation::StateExampleMissing);
        }
        if row
            .state_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DegradedStateContractViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DegradedStateContractViolation::StableBlockMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DegradedStateContractViolation::RowInvariantViolated);
        }
    }
}

/// Every degraded state must be exercised by some worked resolution — the implementation
/// requirement that loading, pending, warning/error, and degraded states are all wired explicitly.
fn validate_degraded_state_coverage(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let exercised: BTreeSet<M5SharedComponentStateClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.degraded_state)
        .collect();
    let covered = degraded_states()
        .iter()
        .all(|state| exercised.contains(state));
    if !covered {
        violations.push(M5DegradedStateContractViolation::DegradedStateCoverageUnproven);
    }
}

/// Every presentation posture must be exercised by some worked resolution.
fn validate_presentation_coverage(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let exercised: BTreeSet<M5DegradedStatePresentation> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.presentation)
        .collect();
    let covered = M5DegradedStatePresentation::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5DegradedStateContractViolation::PresentationCoverageUnproven);
    }
}

/// Every severity must be exercised by some worked resolution — the acceptance criterion that a
/// warning is distinct from an error and an error is distinct from a reduced-capability degraded
/// mode.
fn validate_severity_coverage(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let exercised: BTreeSet<M5DegradedStateSeverity> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.severity)
        .collect();
    let covered = M5DegradedStateSeverity::ALL
        .iter()
        .all(|severity| exercised.contains(severity));
    if !covered {
        violations.push(M5DegradedStateContractViolation::SeverityCoverageUnproven);
    }
}

/// Every non-color cue must be exercised by some worked resolution — the acceptance criterion that
/// state meaning never depends on color alone.
fn validate_cue_coverage(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.state_examples.iter());
    let covered = M5DegradedStateCue::ALL
        .iter()
        .all(|cue| cases().any(|case| case.resolved.required_non_color_cues.contains(cue)));
    if !covered {
        violations.push(M5DegradedStateContractViolation::CueCoverageUnproven);
    }
}

/// Every required disclosure must be exercised by some worked resolution — the requirement that an
/// explainable state always names its cause, owner / block reason, and recovery action.
fn validate_disclosure_coverage(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.state_examples.iter());
    let covered = M5StateDisclosureTrigger::ALL
        .iter()
        .all(|trigger| cases().any(|case| case.resolved.required_disclosures.contains(trigger)));
    if !covered {
        violations.push(M5DegradedStateContractViolation::DisclosureCoverageUnproven);
    }
}

/// Every worked resolution must hold the loading/pending, warning/error, error/degraded
/// distinctness, submission-lineage, no-color-only, and keyboard/screen-reader guarantees — the
/// core acceptance criteria.
fn validate_guarantees(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .all(|case| case.preserves_guarantees());
    if !preserved {
        violations.push(M5DegradedStateContractViolation::GuaranteesUnproven);
    }
}

/// Every worked resolution must preserve its exact block identity, state-style, submission lineage,
/// and disclosure reference — the invariant that the contract never rewrites what it renders or
/// discloses.
fn validate_identity_preservation(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5DegradedStateContractViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.blocks_distinguish_loading_pending_warning_error_degraded,
        review.loading_and_pending_never_collapse,
        review.warning_and_error_never_collapse,
        review.error_and_degraded_never_collapse,
        review.pending_attributed_to_user_action,
        review.consequence_and_recovery_surfaced_when_explainable,
        review.submission_lineage_and_capability_preserved,
        review.state_meaning_never_color_only,
        review.states_keyboard_and_screen_reader_explainable,
        review.states_driven_by_shared_contract_and_tokens,
        review.no_one_off_per_surface_styling,
        review.states_stable_across_deployment_lines,
        review.states_stable_across_consumer_surfaces,
        review.every_block_declares_accessibility_route,
        review.support_export_reconstructs_state_truth,
        review.later_rows_cannot_invent_parallel_state_vocabulary,
    ] {
        if !ok {
            violations.push(M5DegradedStateContractViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.blocks_consume_state_vocabulary,
        projection.presentation_reads_single_source,
        projection.disclosure_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5DegradedStateContractViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DegradedStateContractViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DegradedStateContractPacket,
    violations: &mut Vec<M5DegradedStateContractViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.degraded_state_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DegradedStateContractViolation::ReleasePostureIncomplete);
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

/// Joins degraded-state tokens for a CSV cell with a `|` separator.
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
