//! Implemented M5 toast and loading-state primitives.
//!
//! The frozen [decision / feedback component matrix][matrix] names Aureline's ubiquitous decision and
//! feedback primitives and locks their controlled vocabulary. This module is the fourth implement lane
//! over that matrix: it turns the two transient-acknowledgement and loading-honesty primitives — the
//! **toast** and the **loading state** — into resolvers that produce export-safe, honest projections, so
//! a user can trust that an acknowledgement never becomes the only durable truth and a loading cue never
//! overclaims readiness: a toast acknowledges work briefly, carries at most one bounded action, and
//! points back to the authoritative durable object whenever the outcome still matters after the toast is
//! gone, and a loading state distinguishes a skeleton, retained previous content, a stable placeholder,
//! partial results streaming, and a blocked-waiting state rather than blanking a useful pane or spinning
//! full-screen where partial capability exists — whether it appears in the shell / entry pane, a review
//! workspace, a settings area, a help area, or a support area.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement toasts for short-lived acknowledgement only, with one bounded action where appropriate
//!   and a durable backlink to the authoritative object when the outcome matters after dismissal.**
//!   [`resolve_toast`] refuses to read as a clean, honest acknowledgement when the label is unstated, the
//!   surface context is unresolved, the toast durability is the disallowed toast-only-truth token, the
//!   acknowledgement scope is unresolved, the acknowledgement is not short-lived, the outcome matters but
//!   the durable backlink is missing, the backlink target is unresolved, a present action is not bounded,
//!   the toast is used as the only durable truth, the explanation cannot be reconstructed from the export,
//!   or the proof packet is stale; it degrades instead.
//! * **Implement loading-state primitives that distinguish skeleton, retained-previous-content, stable
//!   placeholder, partial-results-streaming, and blocked-waiting states rather than defaulting to one
//!   spinner treatment.** [`resolve_loading_state`] degrades when the label is unstated, the surface
//!   context is unresolved, the loading fidelity is the disallowed full-screen-spinner token, the loading
//!   treatment is unresolved, the readiness posture is unresolved, a useful pane is blanked, partial
//!   content is not preserved, readiness is overclaimed while data is warming or blocked, the loading
//!   purpose is unstated, the explanation cannot be reconstructed from the export, or the proof packet is
//!   stale.
//! * **Preserve no-toast-only and no-full-screen-spinner rules in the first reusable consumers covering
//!   shell, entry, settings, review, help, and support flows.** Both resolvers carry the single
//!   [`M5LoadingTreatment`] loading vocabulary and the shared [`M5TransientSurfaceContext`] so the
//!   acknowledgement and loading wording stays stable across local, remote, managed, and export-sensitive
//!   panes.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5DecisionFeedbackDisposition`] state vocabulary, the [`M5ToastDurability`] toast-durability
//! vocabulary, and the [`M5LoadingFidelity`] loading-fidelity vocabulary — so shell, review, settings,
//! help, and support surfaces can never fork their own state, durability, or loading wording. Raw secret
//! values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_decision_feedback_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_toast_loading_controls, seeded_m5_toast_loading_controls_review_ui_beta_narrowed,
    seeded_m5_toast_loading_controls_support_ui_preview_narrowed,
    M5_TOAST_LOADING_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_decision_feedback_component_matrix::{
    M5DecisionFeedbackAccessibilityRoute, M5DecisionFeedbackConsumerSurface,
    M5DecisionFeedbackDeploymentLine, M5DecisionFeedbackDisposition,
    M5DecisionFeedbackDowngradeTrigger, M5DecisionFeedbackFamily,
    M5DecisionFeedbackQualificationClass, M5DecisionFeedbackRequiredLabel, M5LoadingFidelity,
    M5ToastDurability, M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
    M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_LOADING_STATE_SCHEMA_REF, M5_TOAST_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ToastLoadingControlsPacket`].
pub const M5_TOAST_LOADING_CONTROLS_RECORD_KIND: &str =
    "implement_m5_toast_and_loading_state_controls";

/// Schema version for M5 toast / loading-state controls records.
pub const M5_TOAST_LOADING_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_TOAST_LOADING_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-toast-and-loading-state-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_TOAST_LOADING_CONTROLS_DOC_REF: &str =
    "docs/components/m5_toast_and_loading_state_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TOAST_LOADING_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-toast-and-loading-state-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_TOAST_LOADING_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-toast-and-loading-state-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TOAST_LOADING_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-toast-and-loading-state-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TOAST_LOADING_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-toast-and-loading-state-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5ToastLoadingConsumerSurface = M5DecisionFeedbackConsumerSurface;

/// Controlled render context — which claimed M5 pane renders the primitive, so a toast's acknowledgement
/// truth and a loading state's readiness truth stay stable whether they appear in the shell / entry pane,
/// a review workspace, a settings area, a help area, or a support area. Minted by this lane, tracking the
/// first claimed M5 panes directly. Shared by both resolvers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TransientSurfaceContext {
    /// The shell / start-center entry pane.
    ShellEntry,
    /// A review workspace pane.
    ReviewWorkspace,
    /// A settings area pane.
    SettingsArea,
    /// A help area pane.
    HelpArea,
    /// A support area pane.
    SupportArea,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5TransientSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellEntry,
        Self::ReviewWorkspace,
        Self::SettingsArea,
        Self::HelpArea,
        Self::SupportArea,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellEntry => "shell_entry",
            Self::ReviewWorkspace => "review_workspace",
            Self::SettingsArea => "settings_area",
            Self::HelpArea => "help_area",
            Self::SupportArea => "support_area",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled acknowledgement scope a toast names, so a transient acknowledgement always states what it
/// is confirming rather than reading as an unlabeled flash. Minted by this lane because the frozen matrix
/// carries the toast *durability* but not the acknowledgement scope the toast acceptance criteria require
/// by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastAcknowledgementScope {
    /// A transient confirmation of a completed action.
    TransientConfirmation,
    /// Background work has been handed off and continues elsewhere.
    BackgroundHandoff,
    /// A reversible action was performed and can be undone.
    ReversibleActionAck,
    /// A non-blocking notice that needs no immediate action.
    NonBlockingNotice,
    /// An outcome whose durable record lives in the authoritative object.
    DurableOutcomeAck,
    /// The acknowledgement scope cannot currently be resolved.
    ScopeUnknown,
}

impl M5ToastAcknowledgementScope {
    /// Every acknowledgement scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TransientConfirmation,
        Self::BackgroundHandoff,
        Self::ReversibleActionAck,
        Self::NonBlockingNotice,
        Self::DurableOutcomeAck,
        Self::ScopeUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransientConfirmation => "transient_confirmation",
            Self::BackgroundHandoff => "background_handoff",
            Self::ReversibleActionAck => "reversible_action_ack",
            Self::NonBlockingNotice => "non_blocking_notice",
            Self::DurableOutcomeAck => "durable_outcome_ack",
            Self::ScopeUnknown => "scope_unknown",
        }
    }

    /// Whether the acknowledgement scope is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ScopeUnknown)
    }
}

/// Controlled durable-object backlink target a toast points back to, so an acknowledgement whose outcome
/// matters after dismissal always routes to the authoritative object rather than vanishing. Minted by
/// this lane so the durable-linkage grammar stays stable across local, remote, managed, and
/// export-sensitive panes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastBacklinkTarget {
    /// The durable activity center.
    ActivityCenter,
    /// A review queue item.
    ReviewQueue,
    /// A settings / capability record.
    SettingsRecord,
    /// A support record / ticket.
    SupportRecord,
    /// The notification center.
    NotificationCenter,
    /// The backlink target cannot currently be resolved.
    TargetUnknown,
}

impl M5ToastBacklinkTarget {
    /// Every backlink target, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActivityCenter,
        Self::ReviewQueue,
        Self::SettingsRecord,
        Self::SupportRecord,
        Self::NotificationCenter,
        Self::TargetUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::ReviewQueue => "review_queue",
            Self::SettingsRecord => "settings_record",
            Self::SupportRecord => "support_record",
            Self::NotificationCenter => "notification_center",
            Self::TargetUnknown => "target_unknown",
        }
    }

    /// Whether the backlink target is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::TargetUnknown)
    }
}

/// Controlled loading treatment a loading state names, so an in-progress pane is always classified with
/// one shared vocabulary — a skeleton, retained previous content, a stable placeholder, partial results
/// streaming, or a blocked-waiting state — rather than defaulting to one spinner treatment. Minted by
/// this lane so the loading vocabulary stays consistent across local, remote, managed, and
/// export-sensitive panes; this is the impl-requirement's five distinguished states plus the unknown
/// sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LoadingTreatment {
    /// A skeleton that preserves the layout while first data warms.
    Skeleton,
    /// Retained previous content shown while a refresh runs.
    RetainedPreviousContent,
    /// A stable placeholder that reserves space without implying data.
    StablePlaceholder,
    /// Partial results streaming in as they arrive.
    PartialResultsStreaming,
    /// A blocked-waiting state that needs an action to proceed.
    BlockedWaiting,
    /// The loading treatment cannot currently be resolved.
    TreatmentUnknown,
}

impl M5LoadingTreatment {
    /// Every loading treatment, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Skeleton,
        Self::RetainedPreviousContent,
        Self::StablePlaceholder,
        Self::PartialResultsStreaming,
        Self::BlockedWaiting,
        Self::TreatmentUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Skeleton => "skeleton",
            Self::RetainedPreviousContent => "retained_previous_content",
            Self::StablePlaceholder => "stable_placeholder",
            Self::PartialResultsStreaming => "partial_results_streaming",
            Self::BlockedWaiting => "blocked_waiting",
            Self::TreatmentUnknown => "treatment_unknown",
        }
    }

    /// Whether the loading treatment is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::TreatmentUnknown)
    }
}

/// Controlled readiness posture a loading state names, so a warming, partial, blocked, or complete pane
/// never overclaims readiness. Minted by this lane because the frozen matrix carries the loading
/// *fidelity* but not the readiness posture the loading acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LoadingReadinessPosture {
    /// Warming and not ready; no data yet.
    WarmingNotReady,
    /// Partially ready; some data is usable.
    PartiallyReady,
    /// Blocked; an action is needed before it can proceed.
    BlockedNeedsAction,
    /// Ready and complete.
    ReadyComplete,
    /// Stalled but retryable.
    StalledRetryable,
    /// The readiness posture cannot currently be resolved.
    PostureUnknown,
}

impl M5LoadingReadinessPosture {
    /// Every readiness posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WarmingNotReady,
        Self::PartiallyReady,
        Self::BlockedNeedsAction,
        Self::ReadyComplete,
        Self::StalledRetryable,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmingNotReady => "warming_not_ready",
            Self::PartiallyReady => "partially_ready",
            Self::BlockedNeedsAction => "blocked_needs_action",
            Self::ReadyComplete => "ready_complete",
            Self::StalledRetryable => "stalled_retryable",
            Self::PostureUnknown => "posture_unknown",
        }
    }

    /// Whether the readiness posture is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::PostureUnknown)
    }
}

/// One mandatory rendered part a toast or loading state must be able to show, so no acknowledgement,
/// durable-backlink, loading-treatment, readiness, or purpose fact is left implicit behind generic chrome
/// or a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastLoadingAnatomyPart {
    /// The primitive's stable identity / permanent title.
    Identity,
    /// The primitive's current typed state disposition.
    State,
    /// The non-visual keyboard route to the primitive.
    KeyboardRoute,
    /// The named acknowledgement scope (toast).
    AcknowledgementScope,
    /// The durable-object backlink (toast).
    DurableBacklink,
    /// The one bounded action, where present (toast).
    BoundedAction,
    /// The named loading treatment (loading state).
    LoadingTreatment,
    /// The named readiness posture (loading state).
    ReadinessPosture,
    /// Whether useful partial content is preserved (loading state).
    PartialContentPreserved,
    /// What the pane is loading and why (loading state).
    LoadingPurpose,
    /// The next step once the pane is ready (loading state).
    NextStepWhenReady,
}

impl M5ToastLoadingAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::AcknowledgementScope,
        Self::DurableBacklink,
        Self::BoundedAction,
        Self::LoadingTreatment,
        Self::ReadinessPosture,
        Self::PartialContentPreserved,
        Self::LoadingPurpose,
        Self::NextStepWhenReady,
    ];

    /// The three parts every claimed primitive must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::AcknowledgementScope => "acknowledgement_scope",
            Self::DurableBacklink => "durable_backlink",
            Self::BoundedAction => "bounded_action",
            Self::LoadingTreatment => "loading_treatment",
            Self::ReadinessPosture => "readiness_posture",
            Self::PartialContentPreserved => "partial_content_preserved",
            Self::LoadingPurpose => "loading_purpose",
            Self::NextStepWhenReady => "next_step_when_ready",
        }
    }
}

/// Next safe action a primitive surfaces so a user is never left without a route to open the durable
/// object, review an acknowledged outcome, wait for ready content, act on a blocked loading state, or
/// read the loading purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastLoadingNextAction {
    /// Open the durable object the toast points back to.
    OpenDurableObject,
    /// Review the acknowledged outcome.
    ReviewAcknowledgedOutcome,
    /// Wait for the ready content to arrive.
    WaitForReadyContent,
    /// Act on a blocked loading state.
    ActOnBlockedLoading,
    /// Read the loading purpose / readiness.
    ReadLoadingPurpose,
    /// No action is needed; the primitive is clean.
    NoActionNeeded,
}

impl M5ToastLoadingNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDurableObject,
        Self::ReviewAcknowledgedOutcome,
        Self::WaitForReadyContent,
        Self::ActOnBlockedLoading,
        Self::ReadLoadingPurpose,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDurableObject => "open_durable_object",
            Self::ReviewAcknowledgedOutcome => "review_acknowledged_outcome",
            Self::WaitForReadyContent => "wait_for_ready_content",
            Self::ActOnBlockedLoading => "act_on_blocked_loading",
            Self::ReadLoadingPurpose => "read_loading_purpose",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastLoadingExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The state dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The toast durability named by the toast.
    ToastDurability,
    /// The loading fidelity named by the loading state.
    LoadingFidelity,
    /// The render / surface context named by both primitives.
    SurfaceContext,
    /// The acknowledgement scope named by the toast.
    AcknowledgementScope,
    /// The loading treatment named by the loading state.
    LoadingTreatment,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ToastLoadingExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::ToastDurability,
        Self::LoadingFidelity,
        Self::SurfaceContext,
        Self::AcknowledgementScope,
        Self::LoadingTreatment,
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
            Self::ToastDurability => "toast_durability",
            Self::LoadingFidelity => "loading_fidelity",
            Self::SurfaceContext => "surface_context",
            Self::AcknowledgementScope => "acknowledgement_scope",
            Self::LoadingTreatment => "loading_treatment",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a toast degraded below a clean, short-lived, durably-linked acknowledgement. The degrade-first
/// ladder returns one of these instead of ever letting a toast-only-truth or backlink-missing toast read
/// as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToastDegradeReason {
    /// The toast label / identity is unstated.
    ToastLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The toast durability is the disallowed toast-only-truth token.
    DurabilityToastOnlyDisallowed,
    /// The acknowledgement scope cannot currently be resolved.
    AcknowledgementScopeUnresolved,
    /// The acknowledgement is not short-lived (long-running work shown as a toast).
    AcknowledgementNotShortLived,
    /// The outcome matters after dismissal but the durable backlink is missing.
    DurableBacklinkMissing,
    /// A durable backlink is present but its target cannot be resolved.
    BacklinkTargetUnresolved,
    /// A present action is not bounded to a single safe action.
    ActionNotBounded,
    /// The toast is used as the only durable truth for reviewable work.
    ToastOnlyTruthUsed,
    /// The explanation cannot be reconstructed from the support export.
    NotReconstructableFromExport,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ToastDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ToastLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::DurabilityToastOnlyDisallowed,
        Self::AcknowledgementScopeUnresolved,
        Self::AcknowledgementNotShortLived,
        Self::DurableBacklinkMissing,
        Self::BacklinkTargetUnresolved,
        Self::ActionNotBounded,
        Self::ToastOnlyTruthUsed,
        Self::NotReconstructableFromExport,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ToastLabelUnstated => "toast_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DurabilityToastOnlyDisallowed => "durability_toast_only_disallowed",
            Self::AcknowledgementScopeUnresolved => "acknowledgement_scope_unresolved",
            Self::AcknowledgementNotShortLived => "acknowledgement_not_short_lived",
            Self::DurableBacklinkMissing => "durable_backlink_missing",
            Self::BacklinkTargetUnresolved => "backlink_target_unresolved",
            Self::ActionNotBounded => "action_not_bounded",
            Self::ToastOnlyTruthUsed => "toast_only_truth_used",
            Self::NotReconstructableFromExport => "not_reconstructable_from_export",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ToastLoadingNextAction {
        match self {
            Self::ToastLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::AcknowledgementScopeUnresolved
            | Self::AcknowledgementNotShortLived
            | Self::ActionNotBounded => M5ToastLoadingNextAction::ReviewAcknowledgedOutcome,
            Self::DurabilityToastOnlyDisallowed
            | Self::DurableBacklinkMissing
            | Self::BacklinkTargetUnresolved
            | Self::ToastOnlyTruthUsed
            | Self::NotReconstructableFromExport
            | Self::ProofStale => M5ToastLoadingNextAction::OpenDurableObject,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::DurabilityToastOnlyDisallowed
            | Self::AcknowledgementNotShortLived
            | Self::ToastOnlyTruthUsed => {
                M5DecisionFeedbackDowngradeTrigger::DurableWorkShownAsToastOnly
            }
            Self::AcknowledgementScopeUnresolved => {
                M5DecisionFeedbackDowngradeTrigger::ScopeUnstated
            }
            Self::DurableBacklinkMissing
            | Self::BacklinkTargetUnresolved
            | Self::ActionNotBounded => M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated,
            Self::ToastLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::NotReconstructableFromExport => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a loading state degraded below a clean, partial-preserving, readiness-honest pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LoadingStateDegradeReason {
    /// The loading label / identity is unstated.
    LoadingLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The loading fidelity is the disallowed full-screen-spinner token.
    FidelityFullScreenSpinnerDisallowed,
    /// The loading treatment cannot currently be resolved.
    LoadingTreatmentUnresolved,
    /// The readiness posture cannot currently be resolved.
    ReadinessPostureUnresolved,
    /// A useful pane was blanked while partial content was available.
    UsefulPaneBlanked,
    /// Useful partial content was not preserved.
    PartialContentNotPreserved,
    /// Readiness was overclaimed while data is warming or blocked.
    ReadinessOverclaimed,
    /// What the pane is loading and why is unstated.
    PurposeUnstated,
    /// The explanation cannot be reconstructed from the support export.
    NotReconstructableFromExport,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5LoadingStateDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::LoadingLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::FidelityFullScreenSpinnerDisallowed,
        Self::LoadingTreatmentUnresolved,
        Self::ReadinessPostureUnresolved,
        Self::UsefulPaneBlanked,
        Self::PartialContentNotPreserved,
        Self::ReadinessOverclaimed,
        Self::PurposeUnstated,
        Self::NotReconstructableFromExport,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoadingLabelUnstated => "loading_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::FidelityFullScreenSpinnerDisallowed => "fidelity_full_screen_spinner_disallowed",
            Self::LoadingTreatmentUnresolved => "loading_treatment_unresolved",
            Self::ReadinessPostureUnresolved => "readiness_posture_unresolved",
            Self::UsefulPaneBlanked => "useful_pane_blanked",
            Self::PartialContentNotPreserved => "partial_content_not_preserved",
            Self::ReadinessOverclaimed => "readiness_overclaimed",
            Self::PurposeUnstated => "purpose_unstated",
            Self::NotReconstructableFromExport => "not_reconstructable_from_export",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ToastLoadingNextAction {
        match self {
            Self::LoadingLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::LoadingTreatmentUnresolved
            | Self::ReadinessPostureUnresolved
            | Self::PurposeUnstated => M5ToastLoadingNextAction::ReadLoadingPurpose,
            Self::UsefulPaneBlanked
            | Self::PartialContentNotPreserved
            | Self::ReadinessOverclaimed
            | Self::FidelityFullScreenSpinnerDisallowed => {
                M5ToastLoadingNextAction::WaitForReadyContent
            }
            Self::NotReconstructableFromExport | Self::ProofStale => {
                M5ToastLoadingNextAction::ActOnBlockedLoading
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::FidelityFullScreenSpinnerDisallowed => {
                M5DecisionFeedbackDowngradeTrigger::FullScreenSpinnerWhenPartialCapable
            }
            Self::UsefulPaneBlanked | Self::PartialContentNotPreserved => {
                M5DecisionFeedbackDowngradeTrigger::UsefulPaneBlankedDuringLoading
            }
            Self::LoadingTreatmentUnresolved => M5DecisionFeedbackDowngradeTrigger::ScopeUnstated,
            Self::ReadinessPostureUnresolved | Self::ReadinessOverclaimed => {
                M5DecisionFeedbackDowngradeTrigger::StateTaxonomyDrifted
            }
            Self::PurposeUnstated => M5DecisionFeedbackDowngradeTrigger::RationaleUnstated,
            Self::LoadingLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::NotReconstructableFromExport => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_toast`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ToastResolutionInput {
    /// Stable identity of the toast instance.
    pub toast_id: String,
    /// The toast label / message shown; empty means unstated.
    pub toast_label: String,
    /// The toast durability (from the frozen matrix vocabulary).
    pub toast_durability: M5ToastDurability,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5TransientSurfaceContext,
    /// The acknowledgement scope named by the toast.
    pub acknowledgement_scope: M5ToastAcknowledgementScope,
    /// The durable-object backlink target.
    pub backlink_target: M5ToastBacklinkTarget,
    /// True when the toast acknowledges transiently (short-lived), not as long-running work.
    pub acknowledges_transiently: bool,
    /// True when the outcome still matters after the toast is dismissed.
    pub outcome_matters_after_dismissal: bool,
    /// True when a durable backlink to the authoritative object is present.
    pub durable_backlink_present: bool,
    /// True when a bounded action is present on the toast.
    pub bounded_action_present: bool,
    /// True when the present action is bounded to a single safe action.
    pub action_is_bounded: bool,
    /// True when the toast avoids being the only durable truth for reviewable work.
    pub avoids_toast_only_truth: bool,
    /// True when the toast explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe toast projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedToast {
    /// Stable identity of the toast instance.
    pub toast_id: String,
    /// The toast label named by the toast.
    pub toast_label: String,
    /// The toast-durability token named by the toast.
    pub toast_durability: String,
    /// Whether the durability is the disallowed toast-only-truth token.
    pub durability_is_toast_only: bool,
    /// The state-disposition token named by the toast.
    pub disposition: String,
    /// The render / surface-context token named by the toast.
    pub surface_context: String,
    /// The acknowledgement-scope token named by the toast.
    pub acknowledgement_scope: String,
    /// The backlink-target token named by the toast.
    pub backlink_target: String,
    /// Whether the toast acknowledges transiently.
    pub acknowledges_transiently: bool,
    /// Whether the outcome matters after dismissal.
    pub outcome_matters_after_dismissal: bool,
    /// Whether a durable backlink is present.
    pub durable_backlink_present: bool,
    /// Whether a bounded action is present.
    pub bounded_action_present: bool,
    /// Whether the present action is bounded.
    pub action_is_bounded: bool,
    /// Whether the toast avoids being the only durable truth.
    pub avoids_toast_only_truth: bool,
    /// Whether the toast explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// Degrade reason, if the toast could not read as a clean, durably-linked acknowledgement.
    pub degrade_reason: Option<M5ToastDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ToastLoadingNextAction,
    /// Whether the toast acknowledges without becoming the only durable truth (clean acknowledgement).
    pub acknowledges_without_becoming_only_truth: bool,
}

impl M5ResolvedToast {
    /// Whether this toast reads as a clean, short-lived, durably-linked acknowledgement.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_loading_state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LoadingStateResolutionInput {
    /// Stable identity of the loading-state instance.
    pub loading_state_id: String,
    /// The loading label / heading shown; empty means unstated.
    pub loading_label: String,
    /// The loading fidelity (from the frozen matrix vocabulary).
    pub loading_fidelity: M5LoadingFidelity,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5TransientSurfaceContext,
    /// The loading treatment named by the loading state.
    pub loading_treatment: M5LoadingTreatment,
    /// The readiness posture named by the loading state.
    pub readiness_posture: M5LoadingReadinessPosture,
    /// True when useful partial content is available to show.
    pub partial_content_available: bool,
    /// True when the available partial content is preserved.
    pub partial_content_preserved: bool,
    /// True when a useful pane is blanked during loading.
    pub pane_blanked: bool,
    /// True when readiness is overclaimed while data is warming or blocked.
    pub overclaims_readiness: bool,
    /// True when what the pane is loading and why is stated.
    pub purpose_stated: bool,
    /// True when the loading explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe loading-state projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLoadingState {
    /// Stable identity of the loading-state instance.
    pub loading_state_id: String,
    /// The loading label named by the pane.
    pub loading_label: String,
    /// The loading-fidelity token named by the pane.
    pub loading_fidelity: String,
    /// Whether the fidelity is the disallowed full-screen-spinner token.
    pub fidelity_is_full_screen_spinner: bool,
    /// The state-disposition token named by the pane.
    pub disposition: String,
    /// The render / surface-context token named by the pane.
    pub surface_context: String,
    /// The loading-treatment token named by the pane.
    pub loading_treatment: String,
    /// The readiness-posture token named by the pane.
    pub readiness_posture: String,
    /// Whether useful partial content is available.
    pub partial_content_available: bool,
    /// Whether the available partial content is preserved.
    pub partial_content_preserved: bool,
    /// Whether a useful pane is blanked during loading.
    pub pane_blanked: bool,
    /// Whether readiness is overclaimed.
    pub overclaims_readiness: bool,
    /// Whether what the pane is loading and why is stated.
    pub purpose_stated: bool,
    /// Whether the loading explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// Degrade reason, if the pane could not read as a clean, partial-preserving, readiness-honest state.
    pub degrade_reason: Option<M5LoadingStateDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ToastLoadingNextAction,
    /// Whether the pane preserves partial content and readiness honesty (clean loading state).
    pub preserves_partial_and_readiness_honesty: bool,
}

impl M5ResolvedLoadingState {
    /// Whether this loading state reads as a clean, partial-preserving, readiness-honest pane.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ToastLoadingResolutionError {
    /// The toast id was empty.
    EmptyToastId,
    /// The loading-state id was empty.
    EmptyLoadingStateId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ToastLoadingResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyToastId => "empty_toast_id",
            Self::EmptyLoadingStateId => "empty_loading_state_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ToastLoadingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 toast / loading-state resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ToastLoadingResolutionError {}

/// Resolves a toast so it stays a short-lived acknowledgement: the toast names its label, durability
/// (never toast-only truth), state disposition, surface context, and acknowledgement scope, acknowledges
/// transiently, points back to a durable object whenever the outcome matters after dismissal, keeps any
/// present action bounded, avoids becoming the only durable truth, and stays reconstructable from the
/// export.
pub fn resolve_toast(
    input: M5ToastResolutionInput,
) -> Result<M5ResolvedToast, M5ToastLoadingResolutionError> {
    if input.toast_id.trim().is_empty() {
        return Err(M5ToastLoadingResolutionError::EmptyToastId);
    }
    if string_is_forbidden(&input.toast_id) || string_is_forbidden(&input.toast_label) {
        return Err(M5ToastLoadingResolutionError::ForbiddenMaterial);
    }

    let durability_is_toast_only = matches!(
        input.toast_durability,
        M5ToastDurability::ToastOnlyTruthDisallowed
    );

    let degrade_reason = if input.toast_label.trim().is_empty() {
        Some(M5ToastDegradeReason::ToastLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ToastDegradeReason::SurfaceContextUnresolved)
    } else if durability_is_toast_only {
        Some(M5ToastDegradeReason::DurabilityToastOnlyDisallowed)
    } else if !input.acknowledgement_scope.is_resolved() {
        Some(M5ToastDegradeReason::AcknowledgementScopeUnresolved)
    } else if !input.acknowledges_transiently {
        Some(M5ToastDegradeReason::AcknowledgementNotShortLived)
    } else if input.outcome_matters_after_dismissal && !input.durable_backlink_present {
        Some(M5ToastDegradeReason::DurableBacklinkMissing)
    } else if input.durable_backlink_present && !input.backlink_target.is_resolved() {
        Some(M5ToastDegradeReason::BacklinkTargetUnresolved)
    } else if input.bounded_action_present && !input.action_is_bounded {
        Some(M5ToastDegradeReason::ActionNotBounded)
    } else if !input.avoids_toast_only_truth {
        Some(M5ToastDegradeReason::ToastOnlyTruthUsed)
    } else if !input.reconstructable_from_export {
        Some(M5ToastDegradeReason::NotReconstructableFromExport)
    } else if !input.proof_fresh {
        Some(M5ToastDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ToastLoadingNextAction::ReviewAcknowledgedOutcome,
    };

    Ok(M5ResolvedToast {
        toast_id: input.toast_id,
        toast_label: input.toast_label,
        toast_durability: input.toast_durability.as_str().to_owned(),
        durability_is_toast_only,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        acknowledgement_scope: input.acknowledgement_scope.as_str().to_owned(),
        backlink_target: input.backlink_target.as_str().to_owned(),
        acknowledges_transiently: input.acknowledges_transiently,
        outcome_matters_after_dismissal: input.outcome_matters_after_dismissal,
        durable_backlink_present: input.durable_backlink_present,
        bounded_action_present: input.bounded_action_present,
        action_is_bounded: input.action_is_bounded,
        avoids_toast_only_truth: input.avoids_toast_only_truth,
        reconstructable_from_export: input.reconstructable_from_export,
        degrade_reason,
        next_action,
        acknowledges_without_becoming_only_truth: degrade_reason.is_none(),
    })
}

/// Resolves a loading state so it preserves partial content and readiness honesty: the pane names its
/// label, loading fidelity (never full-screen spinner where partial capability exists), state
/// disposition, surface context, loading treatment, and readiness posture, preserves useful partial
/// content without blanking a useful pane, never overclaims readiness while data is warming or blocked,
/// states its purpose, and stays reconstructable from the export.
pub fn resolve_loading_state(
    input: M5LoadingStateResolutionInput,
) -> Result<M5ResolvedLoadingState, M5ToastLoadingResolutionError> {
    if input.loading_state_id.trim().is_empty() {
        return Err(M5ToastLoadingResolutionError::EmptyLoadingStateId);
    }
    if string_is_forbidden(&input.loading_state_id) || string_is_forbidden(&input.loading_label) {
        return Err(M5ToastLoadingResolutionError::ForbiddenMaterial);
    }

    let fidelity_is_full_screen_spinner = matches!(
        input.loading_fidelity,
        M5LoadingFidelity::FullScreenSpinnerDisallowed
    );

    let degrade_reason = if input.loading_label.trim().is_empty() {
        Some(M5LoadingStateDegradeReason::LoadingLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5LoadingStateDegradeReason::SurfaceContextUnresolved)
    } else if fidelity_is_full_screen_spinner {
        Some(M5LoadingStateDegradeReason::FidelityFullScreenSpinnerDisallowed)
    } else if !input.loading_treatment.is_resolved() {
        Some(M5LoadingStateDegradeReason::LoadingTreatmentUnresolved)
    } else if !input.readiness_posture.is_resolved() {
        Some(M5LoadingStateDegradeReason::ReadinessPostureUnresolved)
    } else if input.partial_content_available && input.pane_blanked {
        Some(M5LoadingStateDegradeReason::UsefulPaneBlanked)
    } else if input.partial_content_available && !input.partial_content_preserved {
        Some(M5LoadingStateDegradeReason::PartialContentNotPreserved)
    } else if input.overclaims_readiness {
        Some(M5LoadingStateDegradeReason::ReadinessOverclaimed)
    } else if !input.purpose_stated {
        Some(M5LoadingStateDegradeReason::PurposeUnstated)
    } else if !input.reconstructable_from_export {
        Some(M5LoadingStateDegradeReason::NotReconstructableFromExport)
    } else if !input.proof_fresh {
        Some(M5LoadingStateDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ToastLoadingNextAction::WaitForReadyContent,
    };

    Ok(M5ResolvedLoadingState {
        loading_state_id: input.loading_state_id,
        loading_label: input.loading_label,
        loading_fidelity: input.loading_fidelity.as_str().to_owned(),
        fidelity_is_full_screen_spinner,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        loading_treatment: input.loading_treatment.as_str().to_owned(),
        readiness_posture: input.readiness_posture.as_str().to_owned(),
        partial_content_available: input.partial_content_available,
        partial_content_preserved: input.partial_content_preserved,
        pane_blanked: input.pane_blanked,
        overclaims_readiness: input.overclaims_readiness,
        purpose_stated: input.purpose_stated,
        reconstructable_from_export: input.reconstructable_from_export,
        degrade_reason,
        next_action,
        preserves_partial_and_readiness_honesty: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved toast and loading-state examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToastLoadingControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ToastLoadingConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5DecisionFeedbackQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5DecisionFeedbackDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5DecisionFeedbackRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5DecisionFeedbackAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ToastLoadingAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ToastLoadingExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    /// Resolved toast examples.
    pub toast_examples: Vec<M5ResolvedToast>,
    /// Resolved loading-state examples.
    pub loading_state_examples: Vec<M5ResolvedLoadingState>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a toast never represents durable work as toast-only truth. MUST be `false`.
    pub toast_represents_durable_work_as_toast_only: bool,
    /// Hard invariant: a toast never omits its durable backlink when the outcome matters. MUST be
    /// `false`.
    pub toast_lacks_durable_backlink_when_outcome_matters: bool,
    /// Hard invariant: a loading state never blanks a useful pane. MUST be `false`.
    pub loading_blanks_useful_pane: bool,
    /// Hard invariant: a loading state never uses a full-screen spinner where partial capability exists.
    /// MUST be `false`.
    pub loading_uses_full_screen_spinner_when_partial_capable: bool,
}

impl M5ToastLoadingControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ToastLoadingAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ToastLoadingAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ToastLoadingExportField> =
            self.export_fields.iter().copied().collect();
        M5ToastLoadingExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.toast_represents_durable_work_as_toast_only
            && !self.toast_lacks_durable_backlink_when_outcome_matters
            && !self.loading_blanks_useful_pane
            && !self.loading_uses_full_screen_spinner_when_partial_capable
    }

    /// True when a clean toast preserves acknowledgement truth: it is never toast-only, acknowledges
    /// transiently, points back to a durable object whenever the outcome matters, keeps any present
    /// action bounded, avoids being the only durable truth, and stays reconstructable from the export.
    fn toast_is_honest(ex: &M5ResolvedToast) -> bool {
        !ex.is_clean()
            || (!ex.durability_is_toast_only
                && ex.acknowledges_transiently
                && (!ex.outcome_matters_after_dismissal || ex.durable_backlink_present)
                && (!ex.bounded_action_present || ex.action_is_bounded)
                && ex.avoids_toast_only_truth
                && ex.reconstructable_from_export)
    }

    /// True when a clean loading state preserves partial and readiness truth: it never uses a full-screen
    /// spinner, preserves useful partial content without blanking a useful pane, never overclaims
    /// readiness, states its purpose, and stays reconstructable from the export.
    fn loading_state_is_honest(ex: &M5ResolvedLoadingState) -> bool {
        !ex.is_clean()
            || (!ex.fidelity_is_full_screen_spinner
                && (!ex.partial_content_available
                    || (ex.partial_content_preserved && !ex.pane_blanked))
                && !ex.overclaims_readiness
                && ex.purpose_stated
                && ex.reconstructable_from_export)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.toast_examples.iter().all(Self::toast_is_honest)
            && self
                .loading_state_examples
                .iter()
                .all(Self::loading_state_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToastLoadingVocabularySet {
    /// State-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Toast-durability tokens (bound from the frozen matrix).
    pub toast_durabilities: Vec<String>,
    /// Loading-fidelity tokens (bound from the frozen matrix).
    pub loading_fidelities: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Acknowledgement-scope tokens (minted by this lane).
    pub acknowledgement_scopes: Vec<String>,
    /// Backlink-target tokens (minted by this lane).
    pub backlink_targets: Vec<String>,
    /// Loading-treatment tokens (minted by this lane).
    pub loading_treatments: Vec<String>,
    /// Readiness-posture tokens (minted by this lane).
    pub readiness_postures: Vec<String>,
    /// Toast degrade-reason tokens.
    pub toast_degrade_reasons: Vec<String>,
    /// Loading-state degrade-reason tokens.
    pub loading_state_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ToastLoadingVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5DecisionFeedbackDisposition::ALL, |v| v.as_str()),
            toast_durabilities: tokens(&M5ToastDurability::ALL, |v| v.as_str()),
            loading_fidelities: tokens(&M5LoadingFidelity::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5TransientSurfaceContext::ALL, |v| v.as_str()),
            acknowledgement_scopes: tokens(&M5ToastAcknowledgementScope::ALL, |v| v.as_str()),
            backlink_targets: tokens(&M5ToastBacklinkTarget::ALL, |v| v.as_str()),
            loading_treatments: tokens(&M5LoadingTreatment::ALL, |v| v.as_str()),
            readiness_postures: tokens(&M5LoadingReadinessPosture::ALL, |v| v.as_str()),
            toast_degrade_reasons: tokens(&M5ToastDegradeReason::ALL, |v| v.as_str()),
            loading_state_degrade_reasons: tokens(&M5LoadingStateDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ToastLoadingAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ToastLoadingNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ToastLoadingExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5DecisionFeedbackConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5ToastLoadingGovernanceReview {
    /// The toast acknowledges transiently and names its acknowledgement scope.
    pub toast_acknowledges_transiently_with_named_scope: bool,
    /// The toast points back to a durable object when the outcome matters after dismissal.
    pub toast_points_back_to_durable_object_when_outcome_matters: bool,
    /// The toast keeps any present action bounded.
    pub toast_keeps_present_action_bounded: bool,
    /// The toast never becomes the only durable truth.
    pub toast_never_only_durable_truth: bool,
    /// The loading state distinguishes the five loading treatments.
    pub loading_state_distinguishes_treatments: bool,
    /// The loading state preserves useful partial content.
    pub loading_state_preserves_partial_content: bool,
    /// The loading state never blanks a useful pane.
    pub loading_state_never_blanks_useful_pane: bool,
    /// The loading state never uses a full-screen spinner where partial capability exists.
    pub loading_state_never_full_screen_spinner_when_partial_capable: bool,
    /// The loading state never overclaims readiness.
    pub loading_state_never_overclaims_readiness: bool,
    /// Both primitives are reconstructable from the support export.
    pub both_reconstructable_from_export: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToastLoadingConsumerProjection {
    /// Shell / entry surfaces consume the shared toast and loading-state vocabulary.
    pub shell_surfaces_consume_toast_and_loading_vocabulary: bool,
    /// Review surfaces consume the shared toast and loading-state vocabulary.
    pub review_surfaces_consume_toast_and_loading_vocabulary: bool,
    /// Settings surfaces consume the shared loading-state vocabulary.
    pub settings_surfaces_consume_loading_vocabulary: bool,
    /// Help surfaces consume the shared loading-state vocabulary.
    pub help_surfaces_consume_loading_vocabulary: bool,
    /// Toast and loading-state facts trace back to one canonical component contract.
    pub toast_and_loading_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical toast / loading-state source.
    pub support_export_reads_single_toast_loading_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToastLoadingProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToastLoadingReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ToastLoadingControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ToastLoadingControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ToastLoadingControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ToastLoadingVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ToastLoadingGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ToastLoadingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ToastLoadingProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ToastLoadingReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 toast / loading-state controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToastLoadingControlsPacket {
    /// Record kind; must equal [`M5_TOAST_LOADING_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TOAST_LOADING_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ToastLoadingControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ToastLoadingVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ToastLoadingGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ToastLoadingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ToastLoadingProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ToastLoadingReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ToastLoadingControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ToastLoadingControlsPacketInput) -> Self {
        Self {
            record_kind: M5_TOAST_LOADING_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_TOAST_LOADING_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ToastLoadingControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TOAST_LOADING_CONTROLS_RECORD_KIND {
            violations.push(M5ToastLoadingControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TOAST_LOADING_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ToastLoadingControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ToastLoadingControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ToastLoadingControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 toast / loading-state controls packet serializes"),
        ) {
            violations.push(M5ToastLoadingControlsViolation::RawMaterialInExport);
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
            .expect("m5 toast / loading-state controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,toast_examples,loading_state_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .toast_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.loading_state_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.toast_examples.len(),
                row.loading_state_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Toast and Loading-State Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Toast durabilities: {}\n",
            self.vocabulary_set.toast_durabilities.join(", ")
        ));
        out.push_str(&format!(
            "- Loading treatments: {}\n",
            self.vocabulary_set.loading_treatments.join(", ")
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
                "  - Toast examples: {} / loading-state examples: {}\n",
                row.toast_examples.len(),
                row.loading_state_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ToastLoadingControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ToastLoadingControlsViolation>),
}

impl fmt::Display for M5ToastLoadingControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 toast / loading-state controls export parse failed: {error}"
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
                    "m5 toast / loading-state controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ToastLoadingControlsArtifactError {}

/// Validation failures emitted by [`M5ToastLoadingControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ToastLoadingControlsViolation {
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
    /// A controls row carries a dishonest clean example (toast-only truth, backlink-missing toast,
    /// unbounded-action toast, blanked-pane loading, full-screen-spinner loading, readiness-overclaim
    /// loading, or a non-reconstructable primitive).
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
    /// Durable-backlink-when-outcome-matters is not proven: a clean toast whose outcome matters omits a
    /// durable backlink, or a clean toast is the only durable truth, or no backlink-missing example
    /// degrades, or no toast-only-truth example degrades, or no clean toast proves the durable backlink.
    DurableBacklinkWhenOutcomeMattersNotProven,
    /// Partial-content and readiness honesty is not proven: a clean loading state blanks a useful pane or
    /// drops partial content or overclaims readiness, or the loading treatments are not all covered, or
    /// no blanked-pane / full-screen-spinner / readiness-overclaim example degrades.
    PartialContentAndReadinessHonestyNotProven,
    /// The primitives are not proven reconstructable from the export: no clean toast and clean loading
    /// state stay reachable off-screenshot, or no not-reconstructable toast / loading-state example
    /// degrades.
    ReconstructableFromExportNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ToastLoadingControlsViolation {
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
            Self::DurableBacklinkWhenOutcomeMattersNotProven => {
                "durable_backlink_when_outcome_matters_not_proven"
            }
            Self::PartialContentAndReadinessHonestyNotProven => {
                "partial_content_and_readiness_honesty_not_proven"
            }
            Self::ReconstructableFromExportNotProven => "reconstructable_from_export_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_toast_loading_controls_export(
) -> Result<M5ToastLoadingControlsPacket, M5ToastLoadingControlsArtifactError> {
    let packet: M5ToastLoadingControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-toast-and-loading-state-controls-proof/support_export.json"
    )))
    .map_err(M5ToastLoadingControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ToastLoadingControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TOAST_LOADING_CONTROLS_SCHEMA_REF,
        M5_TOAST_LOADING_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_TOAST_SCHEMA_REF,
        M5_LOADING_STATE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ToastLoadingControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ToastLoadingControlsViolation::NoControlsRows);
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
            violations.push(M5ToastLoadingControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ToastLoadingControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ToastLoadingControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TOAST_SCHEMA_REF) || !refs.contains(M5_LOADING_STATE_SCHEMA_REF) {
            violations.push(M5ToastLoadingControlsViolation::ComponentSchemaRefMissing);
        }
        if row.toast_examples.is_empty() || row.loading_state_examples.is_empty() {
            violations.push(M5ToastLoadingControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ToastLoadingControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ToastLoadingControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.toast_acknowledges_transiently_with_named_scope,
        review.toast_points_back_to_durable_object_when_outcome_matters,
        review.toast_keeps_present_action_bounded,
        review.toast_never_only_durable_truth,
        review.loading_state_distinguishes_treatments,
        review.loading_state_preserves_partial_content,
        review.loading_state_never_blanks_useful_pane,
        review.loading_state_never_full_screen_spinner_when_partial_capable,
        review.loading_state_never_overclaims_readiness,
        review.both_reconstructable_from_export,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ToastLoadingControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_toast_and_loading_vocabulary,
        projection.review_surfaces_consume_toast_and_loading_vocabulary,
        projection.settings_surfaces_consume_loading_vocabulary,
        projection.help_surfaces_consume_loading_vocabulary,
        projection.toast_and_loading_trace_to_single_component_contract,
        projection.support_export_reads_single_toast_loading_source,
    ] {
        if !ok {
            violations.push(M5ToastLoadingControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ToastLoadingControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ToastLoadingControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ToastLoadingControlsPacket,
    violations: &mut Vec<M5ToastLoadingControlsViolation>,
) {
    let toasts = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.toast_examples.iter())
    };
    let loadings = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.loading_state_examples.iter())
    };

    // AC1: the first claimed M5 toasts always point back to a durable object when the result matters after
    // dismissal. Every clean toast whose outcome matters carries a durable backlink, no clean toast is the
    // only durable truth, at least one clean toast proves the durable backlink, a backlink-missing toast
    // degrades, and a toast-only-truth toast degrades.
    let clean_toasts_backlink_when_matters = !toasts().any(|ex| {
        ex.is_clean() && ex.outcome_matters_after_dismissal && !ex.durable_backlink_present
    });
    let clean_toasts_avoid_toast_only =
        !toasts().any(|ex| ex.is_clean() && !ex.avoids_toast_only_truth);
    let clean_durable_backlink_proved = toasts().any(|ex| {
        ex.is_clean() && ex.outcome_matters_after_dismissal && ex.durable_backlink_present
    });
    let backlink_missing_degrades =
        toasts().any(|ex| ex.degrade_reason == Some(M5ToastDegradeReason::DurableBacklinkMissing));
    let toast_only_truth_degrades = toasts().any(|ex| {
        ex.degrade_reason == Some(M5ToastDegradeReason::ToastOnlyTruthUsed)
            || ex.degrade_reason == Some(M5ToastDegradeReason::DurabilityToastOnlyDisallowed)
    });
    if !(clean_toasts_backlink_when_matters
        && clean_toasts_avoid_toast_only
        && clean_durable_backlink_proved
        && backlink_missing_degrades
        && toast_only_truth_degrades)
    {
        violations
            .push(M5ToastLoadingControlsViolation::DurableBacklinkWhenOutcomeMattersNotProven);
    }

    // AC2: loading states preserve useful partial content and do not overclaim readiness while data is
    // warming or blocked. Every clean loading state that has partial content preserves it without blanking
    // a useful pane, no clean loading state overclaims readiness, clean loading states cover the skeleton /
    // retained-previous-content / stable-placeholder / partial-results-streaming / blocked-waiting
    // treatment grammar, a blanked-pane example degrades, a full-screen-spinner example degrades, and a
    // readiness-overclaim example degrades.
    let clean_loadings_preserve_partial = !loadings().any(|ex| {
        ex.is_clean()
            && ex.partial_content_available
            && (!ex.partial_content_preserved || ex.pane_blanked)
    });
    let clean_loadings_no_overclaim =
        !loadings().any(|ex| ex.is_clean() && ex.overclaims_readiness);
    let clean_treatments: BTreeSet<String> = loadings()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.loading_treatment.clone())
        .collect();
    let treatment_grammar_covered = [
        "skeleton",
        "retained_previous_content",
        "stable_placeholder",
        "partial_results_streaming",
        "blocked_waiting",
    ]
    .iter()
    .all(|t| clean_treatments.contains(*t));
    let blanked_pane_degrades = loadings()
        .any(|ex| ex.degrade_reason == Some(M5LoadingStateDegradeReason::UsefulPaneBlanked));
    let full_screen_spinner_degrades = loadings().any(|ex| {
        ex.degrade_reason == Some(M5LoadingStateDegradeReason::FidelityFullScreenSpinnerDisallowed)
    });
    let readiness_overclaim_degrades = loadings()
        .any(|ex| ex.degrade_reason == Some(M5LoadingStateDegradeReason::ReadinessOverclaimed));
    if !(clean_loadings_preserve_partial
        && clean_loadings_no_overclaim
        && treatment_grammar_covered
        && blanked_pane_degrades
        && full_screen_spinner_degrades
        && readiness_overclaim_degrades)
    {
        violations
            .push(M5ToastLoadingControlsViolation::PartialContentAndReadinessHonestyNotProven);
    }

    // AC3: release / help / support packets can explain why a toast appeared or a loading state persisted
    // without losing the underlying object identity. At least one clean toast stays reconstructable
    // off-screenshot with a durable backlink, at least one clean loading state stays reconstructable
    // off-screenshot, a not-reconstructable toast degrades, and a not-reconstructable loading state
    // degrades.
    let clean_reconstructable_toast = toasts()
        .any(|ex| ex.is_clean() && ex.reconstructable_from_export && ex.durable_backlink_present);
    let clean_reconstructable_loading =
        loadings().any(|ex| ex.is_clean() && ex.reconstructable_from_export);
    let toast_not_reconstructable_degrades = toasts()
        .any(|ex| ex.degrade_reason == Some(M5ToastDegradeReason::NotReconstructableFromExport));
    let loading_not_reconstructable_degrades = loadings().any(|ex| {
        ex.degrade_reason == Some(M5LoadingStateDegradeReason::NotReconstructableFromExport)
    });
    if !(clean_reconstructable_toast
        && clean_reconstructable_loading
        && toast_not_reconstructable_degrades
        && loading_not_reconstructable_degrades)
    {
        violations.push(M5ToastLoadingControlsViolation::ReconstructableFromExportNotProven);
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
pub const IMPLEMENTED_FAMILIES: [M5DecisionFeedbackFamily; 2] = [
    M5DecisionFeedbackFamily::Toast,
    M5DecisionFeedbackFamily::LoadingState,
];
