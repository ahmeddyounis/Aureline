//! Implemented M5 managed-workspace-lifecycle-card and suspend-resume-rebuild-review-sheet
//! primitives.
//!
//! The frozen [build/remote-boundary component matrix][matrix] names the reusable build / remote /
//! managed-workspace boundary UI components and locks their controlled vocabulary. This module is
//! the third implement lane over that matrix: it turns the two managed-workspace components — the
//! **managed-workspace lifecycle card** and the **suspend/resume/rebuild review sheet** — into
//! resolvers that produce export-safe, honest projections instead of operator-only lifecycle prose.
//!
//! Three acceptance criteria drive the resolvers:
//!
//! * **AC1 — a user can tell whether a workspace resumed, rebuilt, recreated, or degraded to
//!   local-safe continuation, and what changed materially.** [`resolve_managed_workspace_lifecycle_card`]
//!   refuses to read as a clean card unless it names its lifecycle state, its persistence class, its
//!   continuity class, and — whenever an expiry window governs the state — its expiry timing. A clean
//!   card always carries one of the ten [`LifecycleStateClass`] states and never implies exact
//!   continuity after a material change or hides local-safe continuation on an outage / expiry state.
//! * **AC2 — lifecycle review sheets appear before destructive or continuity-affecting actions
//!   rather than after the fact.** [`resolve_suspend_resume_rebuild_review_sheet`] degrades to
//!   [`M5SuspendResumeRebuildReviewSheetDegradeReason::ReviewShownAfterCommit`] whenever a sheet
//!   would appear after the action it gates, and never lets a sheet read as clean when its action
//!   class, template / image provenance, changed persistence class, preserved-vs-lost state, or
//!   reattach / rerun consequences are missing.
//! * **Exact continuity is never implied over a material change** —
//!   [`resolve_suspend_resume_rebuild_review_sheet`] degrades to
//!   [`M5SuspendResumeRebuildReviewSheetDegradeReason::ExactContinuityOverclaimed`] (and the card
//!   resolver mirrors it) whenever a resumed / reprovisioned runtime that changed materially is
//!   presented as exact continuity, so a materially different workspace can never masquerade as the
//!   one the user last saw.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5BuildRemoteBoundaryDisposition`] boundary-disposition vocabulary and the frozen
//! [`M5BuildRemoteDowngradeTrigger`] downgrade-trigger vocabulary — and bind the lifecycle state,
//! persistence class, continuity class, expiry class, template / image provenance, recovery options,
//! and caveats directly to the shared managed-workspace object model ([`LifecycleStateClass`],
//! [`PersistenceClass`], [`ContinuityClass`], [`ExpiryClass`], [`ProvenanceClass`],
//! [`RecoveryOptionClass`], and [`CaveatClass`]), so this lane can never fork its own lifecycle,
//! continuity, or provenance wording.
//!
//! [matrix]: crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_managed_lifecycle_controls,
    seeded_m5_managed_lifecycle_controls_lifecycle_card_beta_narrowed,
    seeded_m5_managed_lifecycle_controls_review_sheet_preview_narrowed,
    M5_MANAGED_LIFECYCLE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::{
    M5BuildRemoteAccessibilityRoute, M5BuildRemoteBoundaryDisposition, M5BuildRemoteConsumerSurface,
    M5BuildRemoteDeploymentLine, M5BuildRemoteDowngradeTrigger, M5BuildRemoteQualificationClass,
    M5BuildRemoteRequiredLabel, BOUND_CONTINUITY_CLASSES, BOUND_EXPIRY_CLASSES,
    BOUND_LIFECYCLE_STATES, BOUND_PERSISTENCE_CLASSES,
    M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF, M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
    M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF, M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
};
use crate::managed_workspace_lifecycle::{
    CaveatClass, ContinuityClass, ExpiryClass, LifecycleStateClass, PersistenceClass,
    ProvenanceClass, RecoveryOptionClass, MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
};

/// Stable record-kind tag carried by [`M5ManagedLifecycleControlsPacket`].
pub const M5_MANAGED_LIFECYCLE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_managed_workspace_lifecycle_card_and_suspend_resume_rebuild_review_sheet_controls";

/// Schema version for M5 managed-workspace-lifecycle-card / suspend-resume-rebuild-review-sheet
/// controls records.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_DOC_REF: &str =
    "docs/remote/m5_managed_workspace_lifecycle_card_and_suspend_resume_rebuild_review_sheet_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MANAGED_LIFECYCLE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls";

/// Repo-relative path of the shared managed-workspace lifecycle object model bound by this lane.
pub const M5_MANAGED_LIFECYCLE_OBJECT_MODEL_DOC_REF: &str = MANAGED_WORKSPACE_LIFECYCLE_DOC_REF;

/// Consumer surface a managed-lifecycle controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5ManagedLifecycleConsumerSurface = M5BuildRemoteConsumerSurface;

/// The canonical template/image provenance classes bound from the shared managed-workspace object
/// model. [`ProvenanceClass`] does not export its own `ALL`, so this lane pins the full set to keep
/// the frozen vocabulary stable and complete.
pub const BOUND_PROVENANCE_CLASSES: [ProvenanceClass; 5] = [
    ProvenanceClass::PinnedDigest,
    ProvenanceClass::PinnedTag,
    ProvenanceClass::SuccessorImage,
    ProvenanceClass::DriftedUnpinned,
    ProvenanceClass::Unknown,
];

/// The canonical recovery options bound from the shared managed-workspace object model.
pub const BOUND_RECOVERY_OPTIONS: [RecoveryOptionClass; 6] = [
    RecoveryOptionClass::Resume,
    RecoveryOptionClass::Reconnect,
    RecoveryOptionClass::Rebuild,
    RecoveryOptionClass::Recreate,
    RecoveryOptionClass::LocalSafeContinue,
    RecoveryOptionClass::ContactOperator,
];

/// The canonical caveats bound from the shared managed-workspace object model.
pub const BOUND_CAVEAT_CLASSES: [CaveatClass; 7] = [
    CaveatClass::PersistenceClassChanged,
    CaveatClass::TemplateChanged,
    CaveatClass::ImageChanged,
    CaveatClass::TargetIdentityChanged,
    CaveatClass::SessionReauthRequired,
    CaveatClass::ScratchStateDiscarded,
    CaveatClass::LocalSafeOnly,
];

/// The suspend / resume / rebuild action a review sheet gates before it is committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedWorkspaceAction {
    /// Suspend / pause the workspace.
    Suspend,
    /// Resume the workspace from a suspended state.
    Resume,
    /// Rebuild the workspace on the current or a successor image.
    Rebuild,
    /// Recreate the workspace from scratch under a new identity.
    Recreate,
    /// Re-establish a dropped connection to a reachable workspace.
    Reconnect,
    /// Tear down an expired workspace and clean up its backing runtime.
    ExpireCleanup,
}

impl M5ManagedWorkspaceAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Suspend,
        Self::Resume,
        Self::Rebuild,
        Self::Recreate,
        Self::Reconnect,
        Self::ExpireCleanup,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Suspend => "suspend",
            Self::Resume => "resume",
            Self::Rebuild => "rebuild",
            Self::Recreate => "recreate",
            Self::Reconnect => "reconnect",
            Self::ExpireCleanup => "expire_cleanup",
        }
    }

    /// Whether this action destroys or materially changes continuity, so its review sheet must appear
    /// before it is committed rather than after the fact.
    pub const fn is_continuity_affecting(self) -> bool {
        matches!(
            self,
            Self::Resume | Self::Rebuild | Self::Recreate | Self::ExpireCleanup
        )
    }
}

/// One mandatory rendered part a lifecycle card or review sheet must be able to show, so no lifecycle,
/// continuity, provenance, or recovery truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedLifecycleAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The current lifecycle state (card).
    LifecycleState,
    /// The persistence class (card).
    PersistenceClass,
    /// The continuity class (card).
    ContinuityClass,
    /// The expiry timing (card).
    ExpiryTiming,
    /// The recovery options offered (card).
    RecoveryOptions,
    /// The local-safe continuation affordance (card).
    LocalSafeContinuation,
    /// The action class the sheet gates (sheet).
    ActionClass,
    /// The template / image provenance (sheet).
    TemplateImageProvenance,
    /// The changed persistence class (sheet).
    ChangedPersistence,
    /// The preserved-vs-lost state (sheet).
    PreservedVsLostState,
    /// The reattach / rerun consequences (sheet).
    ReattachRerunConsequences,
}

impl M5ManagedLifecycleAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::LifecycleState,
        Self::PersistenceClass,
        Self::ContinuityClass,
        Self::ExpiryTiming,
        Self::RecoveryOptions,
        Self::LocalSafeContinuation,
        Self::ActionClass,
        Self::TemplateImageProvenance,
        Self::ChangedPersistence,
        Self::PreservedVsLostState,
        Self::ReattachRerunConsequences,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::LifecycleState => "lifecycle_state",
            Self::PersistenceClass => "persistence_class",
            Self::ContinuityClass => "continuity_class",
            Self::ExpiryTiming => "expiry_timing",
            Self::RecoveryOptions => "recovery_options",
            Self::LocalSafeContinuation => "local_safe_continuation",
            Self::ActionClass => "action_class",
            Self::TemplateImageProvenance => "template_image_provenance",
            Self::ChangedPersistence => "changed_persistence",
            Self::PreservedVsLostState => "preserved_vs_lost_state",
            Self::ReattachRerunConsequences => "reattach_rerun_consequences",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedLifecycleNextAction {
    /// Open the lifecycle details.
    OpenLifecycleDetails,
    /// Review the change before committing a destructive or continuity-affecting action.
    ReviewBeforeCommit,
    /// Reconnect the workspace.
    ReconnectWorkspace,
    /// Continue against the local-safe mirror.
    ContinueLocalSafe,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5ManagedLifecycleNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenLifecycleDetails,
        Self::ReviewBeforeCommit,
        Self::ReconnectWorkspace,
        Self::ContinueLocalSafe,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLifecycleDetails => "open_lifecycle_details",
            Self::ReviewBeforeCommit => "review_before_commit",
            Self::ReconnectWorkspace => "reconnect_workspace",
            Self::ContinueLocalSafe => "continue_local_safe",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a managed-lifecycle controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedLifecycleExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The lifecycle states carried.
    LifecycleStates,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The persistence classes carried.
    PersistenceClasses,
    /// The continuity classes carried.
    ContinuityClasses,
    /// The expiry classes carried.
    ExpiryClasses,
    /// The template / image provenance classes carried.
    ProvenanceClasses,
    /// The recovery options offered.
    RecoveryOptions,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ManagedLifecycleExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::LifecycleStates,
        Self::DegradeReasons,
        Self::Qualification,
        Self::PersistenceClasses,
        Self::ContinuityClasses,
        Self::ExpiryClasses,
        Self::ProvenanceClasses,
        Self::RecoveryOptions,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::LifecycleStates,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::LifecycleStates => "lifecycle_states",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::PersistenceClasses => "persistence_classes",
            Self::ContinuityClasses => "continuity_classes",
            Self::ExpiryClasses => "expiry_classes",
            Self::ProvenanceClasses => "provenance_classes",
            Self::RecoveryOptions => "recovery_options",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a managed-workspace lifecycle card degraded below a clean, fully-legible state. The
/// degrade-first ladder returns one of these instead of ever letting an under-labelled card read as
/// a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManagedWorkspaceLifecycleCardDegradeReason {
    /// The lifecycle state is unstated on the card (AC1 violation).
    LifecycleStateUnstated,
    /// The persistence class is unstated on the card.
    PersistenceClassUnstated,
    /// The continuity class is unstated on the card.
    ContinuityUnstated,
    /// The card claims exact continuity over a material change (guardrail violation).
    ExactContinuityOverclaimed,
    /// An expiry window governs the state but the expiry timing is unstated.
    ExpiryTimingUnstated,
    /// An outage / expiry state hides local-safe continuation or offers no recovery option
    /// (guardrail violation).
    LocalSafeContinuationUnavailable,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ManagedWorkspaceLifecycleCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LifecycleStateUnstated,
        Self::PersistenceClassUnstated,
        Self::ContinuityUnstated,
        Self::ExactContinuityOverclaimed,
        Self::ExpiryTimingUnstated,
        Self::LocalSafeContinuationUnavailable,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleStateUnstated => "lifecycle_state_unstated",
            Self::PersistenceClassUnstated => "persistence_class_unstated",
            Self::ContinuityUnstated => "continuity_unstated",
            Self::ExactContinuityOverclaimed => "exact_continuity_overclaimed",
            Self::ExpiryTimingUnstated => "expiry_timing_unstated",
            Self::LocalSafeContinuationUnavailable => "local_safe_continuation_unavailable",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ManagedLifecycleNextAction {
        match self {
            Self::LifecycleStateUnstated | Self::PersistenceClassUnstated => {
                M5ManagedLifecycleNextAction::OpenLifecycleDetails
            }
            Self::ContinuityUnstated | Self::ExactContinuityOverclaimed => {
                M5ManagedLifecycleNextAction::ReviewBeforeCommit
            }
            Self::LocalSafeContinuationUnavailable => {
                M5ManagedLifecycleNextAction::ContinueLocalSafe
            }
            Self::ExpiryTimingUnstated | Self::ProofStale => {
                M5ManagedLifecycleNextAction::ReviewDiagnostics
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::LifecycleStateUnstated => M5BuildRemoteDowngradeTrigger::LifecycleStateUnstated,
            Self::PersistenceClassUnstated => {
                M5BuildRemoteDowngradeTrigger::PersistenceChangeHidden
            }
            Self::ContinuityUnstated => M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed,
            Self::ExactContinuityOverclaimed => {
                M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed
            }
            Self::ExpiryTimingUnstated => M5BuildRemoteDowngradeTrigger::ExpiryTimingUnstated,
            Self::LocalSafeContinuationUnavailable => {
                M5BuildRemoteDowngradeTrigger::LocalSafeOrCompanionHandoffOverflowOnly
            }
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a suspend / resume / rebuild review sheet degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuspendResumeRebuildReviewSheetDegradeReason {
    /// The action class is unstated on the sheet.
    ActionClassUnstated,
    /// The template / image provenance is unstated.
    ProvenanceUnstated,
    /// A changed persistence class is hidden.
    PersistenceChangeHidden,
    /// The preserved-vs-lost state is unstated.
    PreservedVsLostStateUnstated,
    /// The reattach / rerun consequences are unstated.
    ConsequencesUnstated,
    /// The sheet claims exact continuity over a material change (guardrail violation).
    ExactContinuityOverclaimed,
    /// The review sheet would appear after the action it gates rather than before it (AC2
    /// violation).
    ReviewShownAfterCommit,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SuspendResumeRebuildReviewSheetDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ActionClassUnstated,
        Self::ProvenanceUnstated,
        Self::PersistenceChangeHidden,
        Self::PreservedVsLostStateUnstated,
        Self::ConsequencesUnstated,
        Self::ExactContinuityOverclaimed,
        Self::ReviewShownAfterCommit,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionClassUnstated => "action_class_unstated",
            Self::ProvenanceUnstated => "provenance_unstated",
            Self::PersistenceChangeHidden => "persistence_change_hidden",
            Self::PreservedVsLostStateUnstated => "preserved_vs_lost_state_unstated",
            Self::ConsequencesUnstated => "consequences_unstated",
            Self::ExactContinuityOverclaimed => "exact_continuity_overclaimed",
            Self::ReviewShownAfterCommit => "review_shown_after_commit",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ManagedLifecycleNextAction {
        match self {
            Self::ProofStale => M5ManagedLifecycleNextAction::ReviewDiagnostics,
            _ => M5ManagedLifecycleNextAction::ReviewBeforeCommit,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::PersistenceChangeHidden | Self::PreservedVsLostStateUnstated => {
                M5BuildRemoteDowngradeTrigger::PersistenceChangeHidden
            }
            Self::ExactContinuityOverclaimed => {
                M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed
            }
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
            Self::ActionClassUnstated
            | Self::ProvenanceUnstated
            | Self::ConsequencesUnstated
            | Self::ReviewShownAfterCommit => {
                M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed
            }
        }
    }
}

/// Input to [`resolve_managed_workspace_lifecycle_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ManagedWorkspaceLifecycleCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The workspace label / target identity (empty means unstated).
    pub workspace_label: String,
    /// The lifecycle state the workspace is in.
    pub lifecycle_state: LifecycleStateClass,
    /// True when the lifecycle state is disclosed on the card, never inspector-only.
    pub state_disclosed: bool,
    /// The persistence class backing the workspace.
    pub persistence_class: PersistenceClass,
    /// True when the persistence class is disclosed on the card.
    pub persistence_disclosed: bool,
    /// The claimed continuity relationship to the prior runtime.
    pub continuity_class: ContinuityClass,
    /// True when the continuity class is disclosed on the card.
    pub continuity_disclosed: bool,
    /// The expiry posture governing the state.
    pub expiry_class: ExpiryClass,
    /// True when the expiry timing is disclosed on the card.
    pub expiry_disclosed: bool,
    /// Recovery options offered on the card.
    pub recovery_options: Vec<RecoveryOptionClass>,
    /// True when local-safe continuation is offered inline (not overflow-only).
    pub local_safe_offered: bool,
    /// True when the runtime changed materially relative to the one the user last saw.
    pub material_change_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe managed-workspace lifecycle card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedManagedWorkspaceLifecycleCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The workspace label / target identity named by the card.
    pub workspace_label: String,
    /// The lifecycle-state token the card carries.
    pub lifecycle_state: String,
    /// Human-readable lifecycle-state label.
    pub lifecycle_label: String,
    /// Persistence-class token named by the card.
    pub persistence_class: String,
    /// Continuity-class token named by the card.
    pub continuity_class: String,
    /// Expiry-class token named by the card.
    pub expiry_class: String,
    /// Recovery-option tokens offered by the card.
    pub recovery_options: Vec<String>,
    /// Whether local-safe continuation is offered inline.
    pub local_safe_offered: bool,
    /// Whether the runtime changed materially relative to the one the user last saw.
    pub material_change_present: bool,
    /// Whether this is an outage / expiry / recovery state that must offer local-safe continuation.
    pub is_outage_state: bool,
    /// Degrade reason, if the card could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5ManagedWorkspaceLifecycleCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ManagedLifecycleNextAction,
    /// AC1: whether the lifecycle state is disclosed on the card.
    pub state_disclosed: bool,
    /// AC1: whether the persistence class is disclosed on the card.
    pub persistence_disclosed: bool,
    /// AC1: whether the continuity class is disclosed on the card.
    pub continuity_disclosed: bool,
    /// AC1: whether the expiry timing is disclosed on the card.
    pub expiry_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean card): the card implies exact continuity after a
    /// material change.
    pub implies_exact_continuity_after_material_change: bool,
    /// Guardrail (MUST be `false` on a clean card): an outage / expiry state hides local-safe
    /// continuation.
    pub hides_local_safe_continuation: bool,
}

impl M5ResolvedManagedWorkspaceLifecycleCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this card misrepresents continuity or hides local-safe continuation (a guardrail
    /// violation).
    pub fn misrepresents_continuity_or_local_safe(&self) -> bool {
        self.implies_exact_continuity_after_material_change || self.hides_local_safe_continuation
    }
}

/// Input to [`resolve_suspend_resume_rebuild_review_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SuspendResumeRebuildReviewSheetResolutionInput {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The workspace label / target identity (empty means unstated).
    pub workspace_label: String,
    /// The action the sheet gates.
    pub action: M5ManagedWorkspaceAction,
    /// True when the action class is disclosed on the sheet.
    pub action_disclosed: bool,
    /// The template / image provenance posture.
    pub provenance_class: ProvenanceClass,
    /// True when the template / image provenance is disclosed on the sheet.
    pub provenance_disclosed: bool,
    /// The persistence class the action lands on.
    pub persistence_class: PersistenceClass,
    /// True when the persistence class changed relative to the prior runtime.
    pub persistence_changed: bool,
    /// True when the changed persistence class is disclosed on the sheet.
    pub persistence_change_disclosed: bool,
    /// The claimed continuity relationship to the prior runtime.
    pub continuity_class: ContinuityClass,
    /// True when the preserved state is disclosed on the sheet.
    pub preserved_state_disclosed: bool,
    /// True when the lost state is disclosed on the sheet.
    pub lost_state_disclosed: bool,
    /// True when the reattach / rerun consequences are disclosed on the sheet.
    pub consequences_disclosed: bool,
    /// True when the sheet appears before the action is committed (never after the fact).
    pub shown_before_commit: bool,
    /// Caveats carried by the sheet.
    pub caveats: Vec<CaveatClass>,
    /// True when the runtime changed materially relative to the one the user last saw.
    pub material_change_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe suspend / resume / rebuild review sheet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSuspendResumeRebuildReviewSheet {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The workspace label / target identity named by the sheet.
    pub workspace_label: String,
    /// Action token named by the sheet.
    pub action: String,
    /// Template / image provenance token named by the sheet.
    pub provenance_class: String,
    /// Persistence-class token named by the sheet.
    pub persistence_class: String,
    /// Continuity-class token named by the sheet.
    pub continuity_class: String,
    /// Caveat tokens named by the sheet.
    pub caveats: Vec<String>,
    /// Whether the persistence class changed relative to the prior runtime.
    pub persistence_changed: bool,
    /// Whether the runtime changed materially relative to the one the user last saw.
    pub material_change_present: bool,
    /// AC: whether the action class is disclosed on the sheet.
    pub action_disclosed: bool,
    /// AC: whether the template / image provenance is disclosed on the sheet.
    pub provenance_disclosed: bool,
    /// AC: whether the changed persistence class is disclosed on the sheet.
    pub persistence_change_disclosed: bool,
    /// AC: whether the preserved state is disclosed on the sheet.
    pub preserved_state_disclosed: bool,
    /// AC: whether the lost state is disclosed on the sheet.
    pub lost_state_disclosed: bool,
    /// AC: whether the reattach / rerun consequences are disclosed on the sheet.
    pub consequences_disclosed: bool,
    /// AC2: whether the sheet appears before the action is committed.
    pub shown_before_commit: bool,
    /// Degrade reason, if the sheet could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5SuspendResumeRebuildReviewSheetDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ManagedLifecycleNextAction,
    /// Guardrail (MUST be `false` on a clean sheet): the sheet claims exact continuity over a
    /// material change.
    pub overclaims_exact_continuity: bool,
    /// Guardrail (MUST be `false` on a clean sheet): the sheet would appear after the action rather
    /// than before it.
    pub shown_after_the_fact: bool,
}

impl M5ResolvedSuspendResumeRebuildReviewSheet {
    /// Whether this sheet reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this sheet misrepresents the review — overclaims continuity or appears after the fact
    /// (an AC / guardrail violation).
    pub fn misrepresents_review(&self) -> bool {
        self.overclaims_exact_continuity || self.shown_after_the_fact
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ManagedLifecycleResolutionError {
    /// The card id was empty.
    EmptyCardId,
    /// The sheet id was empty.
    EmptySheetId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ManagedLifecycleResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptySheetId => "empty_sheet_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ManagedLifecycleResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 managed-lifecycle resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ManagedLifecycleResolutionError {}

/// Resolves a managed-workspace lifecycle card, proving AC1: a user can tell whether a workspace
/// resumed, rebuilt, recreated, or degraded to local-safe continuation — with its persistence class,
/// continuity class, and expiry timing — and what changed materially, without opening an operator
/// console.
pub fn resolve_managed_workspace_lifecycle_card(
    input: M5ManagedWorkspaceLifecycleCardResolutionInput,
) -> Result<M5ResolvedManagedWorkspaceLifecycleCard, M5ManagedLifecycleResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5ManagedLifecycleResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id) || string_is_forbidden(&input.workspace_label) {
        return Err(M5ManagedLifecycleResolutionError::ForbiddenMaterial);
    }

    let is_outage_state = input.lifecycle_state.requires_local_safe_continuation();
    let implies_exact_continuity_after_material_change =
        input.material_change_present && input.continuity_class.claims_exact_continuity();
    let hides_local_safe_continuation =
        is_outage_state && (!input.local_safe_offered || input.recovery_options.is_empty());
    let expiry_relevant = !matches!(input.expiry_class, ExpiryClass::None);

    let degrade_reason = if !input.state_disclosed {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::LifecycleStateUnstated)
    } else if !input.persistence_disclosed {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::PersistenceClassUnstated)
    } else if !input.continuity_disclosed {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ContinuityUnstated)
    } else if implies_exact_continuity_after_material_change {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ExactContinuityOverclaimed)
    } else if expiry_relevant && !input.expiry_disclosed {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ExpiryTimingUnstated)
    } else if hides_local_safe_continuation {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::LocalSafeContinuationUnavailable)
    } else if !input.proof_fresh {
        Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None if is_outage_state => M5ManagedLifecycleNextAction::ContinueLocalSafe,
        None => M5ManagedLifecycleNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedManagedWorkspaceLifecycleCard {
        card_id: input.card_id,
        workspace_label: input.workspace_label,
        lifecycle_state: input.lifecycle_state.as_str().to_owned(),
        lifecycle_label: input.lifecycle_state.label().to_owned(),
        persistence_class: input.persistence_class.as_str().to_owned(),
        continuity_class: input.continuity_class.as_str().to_owned(),
        expiry_class: input.expiry_class.as_str().to_owned(),
        recovery_options: input
            .recovery_options
            .iter()
            .map(|option| option.as_str().to_owned())
            .collect(),
        local_safe_offered: input.local_safe_offered,
        material_change_present: input.material_change_present,
        is_outage_state,
        degrade_reason,
        next_action,
        state_disclosed: input.state_disclosed,
        persistence_disclosed: input.persistence_disclosed,
        continuity_disclosed: input.continuity_disclosed,
        expiry_disclosed: input.expiry_disclosed,
        implies_exact_continuity_after_material_change,
        hides_local_safe_continuation,
    })
}

/// Resolves a suspend / resume / rebuild review sheet, proving AC2 (the sheet appears before the
/// destructive or continuity-affecting action it gates) and the continuity guardrail (a materially
/// different workspace is never presented as exact continuity).
pub fn resolve_suspend_resume_rebuild_review_sheet(
    input: M5SuspendResumeRebuildReviewSheetResolutionInput,
) -> Result<M5ResolvedSuspendResumeRebuildReviewSheet, M5ManagedLifecycleResolutionError> {
    if input.sheet_id.trim().is_empty() {
        return Err(M5ManagedLifecycleResolutionError::EmptySheetId);
    }
    if string_is_forbidden(&input.sheet_id) || string_is_forbidden(&input.workspace_label) {
        return Err(M5ManagedLifecycleResolutionError::ForbiddenMaterial);
    }

    let overclaims_exact_continuity =
        input.material_change_present && input.continuity_class.claims_exact_continuity();
    let shown_after_the_fact = !input.shown_before_commit;

    let degrade_reason = if !input.action_disclosed {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ActionClassUnstated)
    } else if !input.provenance_disclosed {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ProvenanceUnstated)
    } else if input.persistence_changed && !input.persistence_change_disclosed {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::PersistenceChangeHidden)
    } else if !(input.preserved_state_disclosed && input.lost_state_disclosed) {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::PreservedVsLostStateUnstated)
    } else if !input.consequences_disclosed {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ConsequencesUnstated)
    } else if overclaims_exact_continuity {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ExactContinuityOverclaimed)
    } else if shown_after_the_fact {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ReviewShownAfterCommit)
    } else if !input.proof_fresh {
        Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ManagedLifecycleNextAction::NoActionNeeded,
    };

    Ok(M5ResolvedSuspendResumeRebuildReviewSheet {
        sheet_id: input.sheet_id,
        workspace_label: input.workspace_label,
        action: input.action.as_str().to_owned(),
        provenance_class: input.provenance_class.as_str().to_owned(),
        persistence_class: input.persistence_class.as_str().to_owned(),
        continuity_class: input.continuity_class.as_str().to_owned(),
        caveats: input
            .caveats
            .iter()
            .map(|caveat| caveat.as_str().to_owned())
            .collect(),
        persistence_changed: input.persistence_changed,
        material_change_present: input.material_change_present,
        action_disclosed: input.action_disclosed,
        provenance_disclosed: input.provenance_disclosed,
        persistence_change_disclosed: input.persistence_change_disclosed,
        preserved_state_disclosed: input.preserved_state_disclosed,
        lost_state_disclosed: input.lost_state_disclosed,
        consequences_disclosed: input.consequences_disclosed,
        shown_before_commit: input.shown_before_commit,
        degrade_reason,
        next_action,
        overclaims_exact_continuity,
        shown_after_the_fact,
    })
}

/// One controls row: one consumer surface bound to the resolved lifecycle card and review sheet
/// examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedLifecycleControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ManagedLifecycleConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5BuildRemoteQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5BuildRemoteDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5BuildRemoteRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5BuildRemoteAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5ManagedLifecycleAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ManagedLifecycleExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    /// Resolved managed-workspace lifecycle card examples.
    pub lifecycle_card_examples: Vec<M5ResolvedManagedWorkspaceLifecycleCard>,
    /// Resolved suspend / resume / rebuild review sheet examples.
    pub review_sheet_examples: Vec<M5ResolvedSuspendResumeRebuildReviewSheet>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never imply exact continuity after a material change.
    pub implies_exact_continuity_after_material_change: bool,
    /// Hard invariant: never hide local-safe continuation or companion handoff behind overflow-only
    /// affordances.
    pub hides_local_safe_or_companion_handoff_in_overflow_only: bool,
    /// Hard invariant: never let a review sheet appear after the destructive / continuity-affecting
    /// action it gates.
    pub review_sheet_appears_after_the_fact: bool,
    /// Hard invariant: never conceal lifecycle or continuity truth behind generic status wording.
    pub conceals_lifecycle_or_continuity_in_generic_status_wording: bool,
}

impl M5ManagedLifecycleControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ManagedLifecycleAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ManagedLifecycleAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ManagedLifecycleExportField> =
            self.export_fields.iter().copied().collect();
        M5ManagedLifecycleExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.implies_exact_continuity_after_material_change
            && !self.hides_local_safe_or_companion_handoff_in_overflow_only
            && !self.review_sheet_appears_after_the_fact
            && !self.conceals_lifecycle_or_continuity_in_generic_status_wording
    }

    /// True when every resolved example on this row is honest: no clean card misrepresents
    /// continuity or hides local-safe continuation, and no clean sheet overclaims continuity or
    /// appears after the fact.
    fn examples_are_honest(&self) -> bool {
        self.lifecycle_card_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.misrepresents_continuity_or_local_safe()))
            && self
                .review_sheet_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.misrepresents_review()))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedLifecycleVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Lifecycle-state tokens (bound from the managed-workspace object model).
    pub lifecycle_states: Vec<String>,
    /// Persistence-class tokens (bound from the managed-workspace object model).
    pub persistence_classes: Vec<String>,
    /// Continuity-class tokens (bound from the managed-workspace object model).
    pub continuity_classes: Vec<String>,
    /// Expiry-class tokens (bound from the managed-workspace object model).
    pub expiry_classes: Vec<String>,
    /// Provenance-class tokens (bound from the managed-workspace object model).
    pub provenance_classes: Vec<String>,
    /// Recovery-option tokens (bound from the managed-workspace object model).
    pub recovery_options: Vec<String>,
    /// Caveat-class tokens (bound from the managed-workspace object model).
    pub caveat_classes: Vec<String>,
    /// Workspace-action tokens.
    pub workspace_actions: Vec<String>,
    /// Card degrade-reason tokens.
    pub card_degrade_reasons: Vec<String>,
    /// Sheet degrade-reason tokens.
    pub sheet_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ManagedLifecycleVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` / bound arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5BuildRemoteBoundaryDisposition::ALL, |v| v.as_str()),
            lifecycle_states: tokens(&BOUND_LIFECYCLE_STATES, |v| v.as_str()),
            persistence_classes: tokens(&BOUND_PERSISTENCE_CLASSES, |v| v.as_str()),
            continuity_classes: tokens(&BOUND_CONTINUITY_CLASSES, |v| v.as_str()),
            expiry_classes: tokens(&BOUND_EXPIRY_CLASSES, |v| v.as_str()),
            provenance_classes: tokens(&BOUND_PROVENANCE_CLASSES, |v| v.as_str()),
            recovery_options: tokens(&BOUND_RECOVERY_OPTIONS, |v| v.as_str()),
            caveat_classes: tokens(&BOUND_CAVEAT_CLASSES, |v| v.as_str()),
            workspace_actions: tokens(&M5ManagedWorkspaceAction::ALL, |v| v.as_str()),
            card_degrade_reasons: tokens(&M5ManagedWorkspaceLifecycleCardDegradeReason::ALL, |v| {
                v.as_str()
            }),
            sheet_degrade_reasons: tokens(
                &M5SuspendResumeRebuildReviewSheetDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5ManagedLifecycleAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ManagedLifecycleNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ManagedLifecycleExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BuildRemoteConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5ManagedLifecycleGovernanceReview {
    /// The card always names its lifecycle state and persistence class.
    pub card_names_lifecycle_state_and_persistence_class: bool,
    /// The card always names its continuity class and expiry timing.
    pub card_names_continuity_and_expiry_timing: bool,
    /// The lifecycle state is always explicit, never operator-console-only.
    pub lifecycle_state_always_explicit: bool,
    /// The review sheet always names its action class and template / image provenance.
    pub review_sheet_names_action_and_provenance: bool,
    /// The review sheet always names its preserved-vs-lost state and reattach / rerun consequences.
    pub review_sheet_names_preserved_vs_lost_and_consequences: bool,
    /// The review sheet always appears before a destructive or continuity-affecting action.
    pub review_sheet_appears_before_destructive_action: bool,
    /// A material change never implies exact continuity.
    pub material_change_never_implies_exact_continuity: bool,
    /// Local-safe continuation is never hidden behind overflow-only affordances.
    pub local_safe_continuation_never_overflow_only: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix and managed-workspace vocabulary rather than inventing
    /// parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedLifecycleConsumerProjection {
    /// Run / test / debug surfaces consume the shared lifecycle vocabulary.
    pub run_test_debug_surfaces_consume_lifecycle_vocabulary: bool,
    /// Preview surfaces consume the shared lifecycle vocabulary.
    pub preview_surfaces_consume_lifecycle_vocabulary: bool,
    /// Companion surfaces reuse the same lifecycle cards and review language.
    pub companion_surfaces_reuse_lifecycle_cards_and_review_language: bool,
    /// Incident / ops surfaces consume the shared lifecycle vocabulary.
    pub incident_ops_consumes_lifecycle_vocabulary: bool,
    /// Support / export reads a single canonical lifecycle source.
    pub support_export_reads_single_lifecycle_source: bool,
    /// Lifecycle language stays consistent across every surface.
    pub lifecycle_language_consistent_across_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedLifecycleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedLifecycleReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ManagedLifecycleControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ManagedLifecycleControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ManagedLifecycleControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ManagedLifecycleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ManagedLifecycleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ManagedLifecycleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ManagedLifecycleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ManagedLifecycleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 managed-workspace-lifecycle-card / suspend-resume-rebuild-review-sheet controls
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ManagedLifecycleControlsPacket {
    /// Record kind; must equal [`M5_MANAGED_LIFECYCLE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ManagedLifecycleControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ManagedLifecycleVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ManagedLifecycleGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ManagedLifecycleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ManagedLifecycleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ManagedLifecycleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ManagedLifecycleControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ManagedLifecycleControlsPacketInput) -> Self {
        Self {
            record_kind: M5_MANAGED_LIFECYCLE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ManagedLifecycleControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MANAGED_LIFECYCLE_CONTROLS_RECORD_KIND {
            violations.push(M5ManagedLifecycleControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ManagedLifecycleControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ManagedLifecycleControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ManagedLifecycleControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 managed-lifecycle controls packet serializes"),
        ) {
            violations.push(M5ManagedLifecycleControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 managed-lifecycle controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,card_examples,sheet_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .lifecycle_card_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.review_sheet_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.lifecycle_card_examples.len(),
                row.review_sheet_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Managed-Workspace-Lifecycle-Card and Suspend-Resume-Rebuild-Review-Sheet Controls\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Lifecycle states: {}\n",
            self.vocabulary_set.lifecycle_states.join(", ")
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
                "  - Card examples: {} / sheet examples: {}\n",
                row.lifecycle_card_examples.len(),
                row.review_sheet_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ManagedLifecycleControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ManagedLifecycleControlsViolation>),
}

impl fmt::Display for M5ManagedLifecycleControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 managed-lifecycle controls export parse failed: {error}"
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
                    "m5 managed-lifecycle controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ManagedLifecycleControlsArtifactError {}

/// Validation failures emitted by [`M5ManagedLifecycleControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ManagedLifecycleControlsViolation {
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
    /// A controls row carries a dishonest clean example (implied exact continuity, hidden local-safe
    /// continuation, overclaimed continuity, or review shown after the fact).
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
    /// AC1 is not proven: clean cards do not cover every lifecycle state, no
    /// state/continuity/local-safe-unavailable card degrades, or a clean card implies exact
    /// continuity after a material change / hides local-safe continuation.
    Ac1NotProven,
    /// AC2 / continuity guardrail is not proven: no review-shown-after-commit or
    /// exact-continuity-overclaimed sheet degrades, no clean sheet is shown before commit with full
    /// disclosure, or a clean sheet overclaims continuity / appears after the fact.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ManagedLifecycleControlsViolation {
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
            Self::Ac1NotProven => "ac1_not_proven",
            Self::Ac2NotProven => "ac2_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_managed_lifecycle_controls_export(
) -> Result<M5ManagedLifecycleControlsPacket, M5ManagedLifecycleControlsArtifactError> {
    let packet: M5ManagedLifecycleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls-proof/support_export.json"
    )))
    .map_err(M5ManagedLifecycleControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ManagedLifecycleControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MANAGED_LIFECYCLE_CONTROLS_SCHEMA_REF,
        M5_MANAGED_LIFECYCLE_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF,
        M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ManagedLifecycleControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ManagedLifecycleControlsViolation::NoControlsRows);
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
            violations.push(M5ManagedLifecycleControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ManagedLifecycleControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ManagedLifecycleControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_MANAGED_WORKSPACE_LIFECYCLE_CARD_SCHEMA_REF)
            || !refs.contains(M5_SUSPEND_RESUME_REBUILD_REVIEW_SHEET_SCHEMA_REF)
        {
            violations.push(M5ManagedLifecycleControlsViolation::ComponentSchemaRefMissing);
        }
        if row.lifecycle_card_examples.is_empty() || row.review_sheet_examples.is_empty() {
            violations.push(M5ManagedLifecycleControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ManagedLifecycleControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ManagedLifecycleControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.card_names_lifecycle_state_and_persistence_class,
        review.card_names_continuity_and_expiry_timing,
        review.lifecycle_state_always_explicit,
        review.review_sheet_names_action_and_provenance,
        review.review_sheet_names_preserved_vs_lost_and_consequences,
        review.review_sheet_appears_before_destructive_action,
        review.material_change_never_implies_exact_continuity,
        review.local_safe_continuation_never_overflow_only,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ManagedLifecycleControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.run_test_debug_surfaces_consume_lifecycle_vocabulary,
        projection.preview_surfaces_consume_lifecycle_vocabulary,
        projection.companion_surfaces_reuse_lifecycle_cards_and_review_language,
        projection.incident_ops_consumes_lifecycle_vocabulary,
        projection.support_export_reads_single_lifecycle_source,
        projection.lifecycle_language_consistent_across_surfaces,
    ] {
        if !ok {
            violations.push(M5ManagedLifecycleControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ManagedLifecycleControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ManagedLifecycleControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ManagedLifecycleControlsPacket,
    violations: &mut Vec<M5ManagedLifecycleControlsViolation>,
) {
    let card_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.lifecycle_card_examples.iter())
    };
    let sheet_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.review_sheet_examples.iter())
    };

    // AC1: a user can tell whether a workspace resumed / rebuilt / recreated / degraded to
    // local-safe continuation and what changed materially — clean cards cover every lifecycle state,
    // a state-unstated card degrades, a continuity-unstated card degrades, a local-safe-unavailable
    // card degrades, and no clean card implies exact continuity after a material change or hides
    // local-safe continuation.
    let clean_states: BTreeSet<&str> = card_examples()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.lifecycle_state.as_str())
        .collect();
    let covers_all_states = BOUND_LIFECYCLE_STATES
        .iter()
        .all(|state| clean_states.contains(state.as_str()));
    let state_unstated_degrades = card_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5ManagedWorkspaceLifecycleCardDegradeReason::LifecycleStateUnstated)
    });
    let continuity_unstated_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5ManagedWorkspaceLifecycleCardDegradeReason::ContinuityUnstated)
    });
    let local_safe_unavailable_degrades = card_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5ManagedWorkspaceLifecycleCardDegradeReason::LocalSafeContinuationUnavailable)
    });
    let no_clean_card_misrepresents =
        card_examples().all(|ex| !(ex.is_clean() && ex.misrepresents_continuity_or_local_safe()));
    if !(covers_all_states
        && state_unstated_degrades
        && continuity_unstated_degrades
        && local_safe_unavailable_degrades
        && no_clean_card_misrepresents)
    {
        violations.push(M5ManagedLifecycleControlsViolation::Ac1NotProven);
    }

    // AC2 + continuity guardrail: review sheets appear before destructive / continuity-affecting
    // actions and never imply exact continuity over a material change — at least one
    // review-shown-after-commit sheet degrades, at least one exact-continuity-overclaimed sheet
    // degrades, at least one clean sheet is shown before commit with full disclosure, and no clean
    // sheet overclaims continuity or appears after the fact.
    let review_after_commit_degrades = sheet_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ReviewShownAfterCommit)
            && ex.shown_after_the_fact
    });
    let continuity_overclaim_degrades = sheet_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5SuspendResumeRebuildReviewSheetDegradeReason::ExactContinuityOverclaimed)
            && ex.overclaims_exact_continuity
    });
    let clean_sheet_shown_before_commit =
        sheet_examples().any(|ex| ex.is_clean() && ex.shown_before_commit);
    let no_clean_sheet_misrepresents =
        sheet_examples().all(|ex| !(ex.is_clean() && ex.misrepresents_review()));
    if !(review_after_commit_degrades
        && continuity_overclaim_degrades
        && clean_sheet_shown_before_commit
        && no_clean_sheet_misrepresents)
    {
        violations.push(M5ManagedLifecycleControlsViolation::Ac2NotProven);
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
