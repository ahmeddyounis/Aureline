//! Implemented M5 workspace-expiry-banner and local-safe-continuation-card primitives.
//!
//! The frozen [build/remote-boundary component matrix][matrix] names the reusable build / remote /
//! managed-workspace boundary UI components and locks their controlled vocabulary. This module is
//! the fourth and final implement lane over that matrix: it turns the two expiry / fallback
//! components — the **workspace-expiry banner** and the **local-safe-continuation card** — into
//! resolvers that produce export-safe, honest projections instead of a generic disconnect or a
//! silent service loss.
//!
//! Two acceptance criteria drive the resolvers:
//!
//! * **AC1 — expiry events no longer appear as generic disconnects or silent service loss.**
//!   [`resolve_workspace_expiry_banner`] refuses to read as a clean banner unless it names its
//!   exact expiry timing, its triggering owner / source, the affected capabilities, and an
//!   export-before-loss or renew / reopen action. A banner that leaves its timing or its triggering
//!   source unstated degrades to [`M5WorkspaceExpiryBannerDegradeReason::ExpiryTimingUnstated`] /
//!   [`M5WorkspaceExpiryBannerDegradeReason::TriggeringSourceUnstated`] and reads as a generic
//!   disconnect, never a clean pass.
//! * **AC2 — users can see what remains local-safe and what must be reattached or rerun.**
//!   [`resolve_local_safe_continuation_card`] refuses to read as a clean card unless it names its
//!   preserved files / context, its lost live state (terminals, ports, kernels, previews), and the
//!   next safe actions (continue locally, reconnect, or rebuild). A card that hides its preserved or
//!   lost state degrades, and an outage state that offers no local-safe continuation degrades to
//!   [`M5LocalSafeContinuationCardDegradeReason::LocalSafeContinuationUnavailable`].
//! * **Exact continuity is never implied over a material change** — both resolvers degrade to their
//!   `ExactContinuityOverclaimed` reason whenever an expiry or lifecycle change that invalidated the
//!   prior runtime is presented as exact continuity, so a materially different (or gone) workspace
//!   can never masquerade as the one the user last saw.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5BuildRemoteBoundaryDisposition`] boundary-disposition vocabulary and the frozen
//! [`M5BuildRemoteDowngradeTrigger`] downgrade-trigger vocabulary — and bind the expiry class,
//! triggering reason, persistence class, continuity class, and recovery options directly to the
//! shared managed-workspace object model ([`ExpiryClass`], [`TransitionReasonClass`],
//! [`PersistenceClass`], [`ContinuityClass`], and [`RecoveryOptionClass`]), so this lane can never
//! fork its own expiry, continuity, or recovery wording.
//!
//! [matrix]: crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_expiry_continuation_controls,
    seeded_m5_expiry_continuation_controls_expiry_banner_beta_narrowed,
    seeded_m5_expiry_continuation_controls_local_safe_card_preview_narrowed,
    M5_EXPIRY_CONTINUATION_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::{
    M5BuildRemoteAccessibilityRoute, M5BuildRemoteBoundaryDisposition, M5BuildRemoteConsumerSurface,
    M5BuildRemoteDeploymentLine, M5BuildRemoteDowngradeTrigger, M5BuildRemoteQualificationClass,
    M5BuildRemoteRequiredLabel, BOUND_CONTINUITY_CLASSES, BOUND_EXPIRY_CLASSES,
    BOUND_PERSISTENCE_CLASSES, M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
    M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF, M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
    M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
};
use crate::managed_workspace_lifecycle::{
    ContinuityClass, ExpiryClass, PersistenceClass, RecoveryOptionClass, TransitionReasonClass,
    MANAGED_WORKSPACE_LIFECYCLE_DOC_REF,
};

/// Stable record-kind tag carried by [`M5ExpiryContinuationControlsPacket`].
pub const M5_EXPIRY_CONTINUATION_CONTROLS_RECORD_KIND: &str =
    "implement_m5_workspace_expiry_banner_and_local_safe_continuation_card_controls";

/// Schema version for M5 workspace-expiry-banner / local-safe-continuation-card controls records.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls boundary schema.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-expiry-banner-local-safe-continuation-card-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_DOC_REF: &str =
    "docs/remote/m5_workspace_expiry_banner_and_local_safe_continuation_card_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-workspace-expiry-banner-local-safe-continuation-card-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-workspace-expiry-banner-local-safe-continuation-card-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-workspace-expiry-banner-local-safe-continuation-card-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_EXPIRY_CONTINUATION_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workspace-expiry-banner-local-safe-continuation-card-controls";

/// Repo-relative path of the shared managed-workspace lifecycle object model bound by this lane.
pub const M5_EXPIRY_CONTINUATION_OBJECT_MODEL_DOC_REF: &str = MANAGED_WORKSPACE_LIFECYCLE_DOC_REF;

/// Consumer surface an expiry / continuation controls row projects onto. Reuses the frozen matrix
/// consumer-surface taxonomy so no lane invents a parallel surface set.
pub type M5ExpiryContinuationConsumerSurface = M5BuildRemoteConsumerSurface;

/// The canonical triggering owner / source reasons bound from the shared managed-workspace object
/// model. [`TransitionReasonClass`] does not export its own `ALL`, so this lane pins the full set to
/// keep the frozen vocabulary stable and complete.
pub const BOUND_TRANSITION_REASONS: [TransitionReasonClass; 10] = [
    TransitionReasonClass::UserRequestedCreate,
    TransitionReasonClass::UserRequestedResume,
    TransitionReasonClass::UserRequestedSuspend,
    TransitionReasonClass::IdleWindowElapsed,
    TransitionReasonClass::HibernationWindowElapsed,
    TransitionReasonClass::SuccessorImageAvailable,
    TransitionReasonClass::CapsuleDriftDetected,
    TransitionReasonClass::ControlPlaneFailure,
    TransitionReasonClass::ControlPlaneRecovered,
    TransitionReasonClass::ExpiryDeadlineReached,
];

/// The canonical recovery options bound from the shared managed-workspace object model.
/// [`RecoveryOptionClass`] does not export its own `ALL`, so this lane pins the full set.
pub const BOUND_RECOVERY_OPTIONS: [RecoveryOptionClass; 6] = [
    RecoveryOptionClass::Resume,
    RecoveryOptionClass::Reconnect,
    RecoveryOptionClass::Rebuild,
    RecoveryOptionClass::Recreate,
    RecoveryOptionClass::LocalSafeContinue,
    RecoveryOptionClass::ContactOperator,
];

/// The expiry-window classes that actually govern a workspace, in canonical order. Excludes
/// [`ExpiryClass::None`] because a banner is only rendered when an expiry window applies.
pub const EXPIRY_WINDOW_CLASSES: [ExpiryClass; 4] = [
    ExpiryClass::IdleWindow,
    ExpiryClass::HibernationWindow,
    ExpiryClass::HardDeadline,
    ExpiryClass::ControlPlaneOutage,
];

/// A live capability that a managed workspace can lose when it expires or degrades to a local-safe
/// mirror. Making the affected / lost capabilities explicit lets an expiry banner and a local-safe
/// continuation card name exactly what stops working, so loss is never silent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceLiveCapability {
    /// Interactive terminals attached to the runtime.
    Terminals,
    /// Forwarded ports / tunnels.
    Ports,
    /// Notebook / language kernels.
    Kernels,
    /// Live preview routes served from the workspace.
    Previews,
    /// Background jobs / long-running tasks.
    BackgroundJobs,
    /// Attached debug sessions.
    DebugSessions,
    /// Managed services provisioned alongside the workspace.
    ManagedServices,
}

impl M5WorkspaceLiveCapability {
    /// Every live capability, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Terminals,
        Self::Ports,
        Self::Kernels,
        Self::Previews,
        Self::BackgroundJobs,
        Self::DebugSessions,
        Self::ManagedServices,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminals => "terminals",
            Self::Ports => "ports",
            Self::Kernels => "kernels",
            Self::Previews => "previews",
            Self::BackgroundJobs => "background_jobs",
            Self::DebugSessions => "debug_sessions",
            Self::ManagedServices => "managed_services",
        }
    }
}

/// A class of preserved file / context that survives an expiry or fallback into a local-safe mirror.
/// Making the preserved context explicit lets a local-safe continuation card name what remains
/// local-safe, so the user is never left guessing what they still have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PreservedContextClass {
    /// The working-tree files as last mirrored locally.
    WorkingTreeFiles,
    /// Unsaved editor edits captured in the local mirror.
    UnsavedEdits,
    /// Durable checkpoints / snapshots.
    Checkpoints,
    /// Notebook input cells (not their live kernel state).
    NotebookInputs,
    /// Recorded command history.
    CommandHistory,
    /// The declared environment / configuration.
    EnvironmentConfig,
}

impl M5PreservedContextClass {
    /// Every preserved-context class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkingTreeFiles,
        Self::UnsavedEdits,
        Self::Checkpoints,
        Self::NotebookInputs,
        Self::CommandHistory,
        Self::EnvironmentConfig,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkingTreeFiles => "working_tree_files",
            Self::UnsavedEdits => "unsaved_edits",
            Self::Checkpoints => "checkpoints",
            Self::NotebookInputs => "notebook_inputs",
            Self::CommandHistory => "command_history",
            Self::EnvironmentConfig => "environment_config",
        }
    }
}

/// An action a workspace-expiry banner offers before the workspace is lost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceExpiryAction {
    /// Export the working state before the runtime is lost.
    ExportBeforeLoss,
    /// Renew / extend the expiry window where allowed.
    Renew,
    /// Reopen the workspace where allowed.
    Reopen,
    /// Continue against the local-safe mirror.
    ContinueLocalSafe,
    /// Escalate to the operator who owns the control plane.
    ContactOperator,
}

impl M5WorkspaceExpiryAction {
    /// Every expiry action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExportBeforeLoss,
        Self::Renew,
        Self::Reopen,
        Self::ContinueLocalSafe,
        Self::ContactOperator,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportBeforeLoss => "export_before_loss",
            Self::Renew => "renew",
            Self::Reopen => "reopen",
            Self::ContinueLocalSafe => "continue_local_safe",
            Self::ContactOperator => "contact_operator",
        }
    }
}

/// One mandatory rendered part an expiry banner or local-safe continuation card must be able to
/// show, so no expiry timing, capability, preserved / lost state, or continuity truth is left
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExpiryContinuationAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The exact expiry timing (banner).
    ExpiryTiming,
    /// The triggering owner / source (banner).
    TriggeringSource,
    /// The affected capabilities (banner).
    AffectedCapabilities,
    /// The export-before-loss / renew / reopen actions (banner).
    ExportRenewActions,
    /// The preserved files / context (card).
    PreservedContext,
    /// The lost live state (card).
    LostLiveState,
    /// The next safe actions (card).
    NextSafeActions,
    /// The local-safe continuation affordance (card).
    LocalSafeContinuation,
    /// The continuity caveat carried by either component.
    ContinuityCaveat,
}

impl M5ExpiryContinuationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ExpiryTiming,
        Self::TriggeringSource,
        Self::AffectedCapabilities,
        Self::ExportRenewActions,
        Self::PreservedContext,
        Self::LostLiveState,
        Self::NextSafeActions,
        Self::LocalSafeContinuation,
        Self::ContinuityCaveat,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ExpiryTiming => "expiry_timing",
            Self::TriggeringSource => "triggering_source",
            Self::AffectedCapabilities => "affected_capabilities",
            Self::ExportRenewActions => "export_renew_actions",
            Self::PreservedContext => "preserved_context",
            Self::LostLiveState => "lost_live_state",
            Self::NextSafeActions => "next_safe_actions",
            Self::LocalSafeContinuation => "local_safe_continuation",
            Self::ContinuityCaveat => "continuity_caveat",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExpiryContinuationNextAction {
    /// Export the working state before the runtime is lost.
    ExportBeforeLoss,
    /// Renew or reopen the workspace where allowed.
    RenewOrReopen,
    /// Continue against the local-safe mirror.
    ContinueLocalSafe,
    /// Reconnect the workspace.
    ReconnectWorkspace,
    /// Review diagnostics for the unavailable signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5ExpiryContinuationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExportBeforeLoss,
        Self::RenewOrReopen,
        Self::ContinueLocalSafe,
        Self::ReconnectWorkspace,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportBeforeLoss => "export_before_loss",
            Self::RenewOrReopen => "renew_or_reopen",
            Self::ContinueLocalSafe => "continue_local_safe",
            Self::ReconnectWorkspace => "reconnect_workspace",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field an expiry / continuation controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExpiryContinuationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The expiry classes carried.
    ExpiryClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The triggering reasons carried.
    TriggeringReasons,
    /// The affected / lost capabilities carried.
    AffectedCapabilities,
    /// The preserved-context classes carried.
    PreservedContext,
    /// The lost live-state classes carried.
    LostLiveState,
    /// The continuity classes carried.
    ContinuityClasses,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ExpiryContinuationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::ExpiryClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::TriggeringReasons,
        Self::AffectedCapabilities,
        Self::PreservedContext,
        Self::LostLiveState,
        Self::ContinuityClasses,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::ExpiryClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::ExpiryClasses => "expiry_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::TriggeringReasons => "triggering_reasons",
            Self::AffectedCapabilities => "affected_capabilities",
            Self::PreservedContext => "preserved_context",
            Self::LostLiveState => "lost_live_state",
            Self::ContinuityClasses => "continuity_classes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a workspace-expiry banner degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting an under-labelled banner read as a clean pass
/// (or as a generic disconnect).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceExpiryBannerDegradeReason {
    /// The exact expiry timing is unstated (AC1 violation — reads as a generic disconnect).
    ExpiryTimingUnstated,
    /// The triggering owner / source is unstated (AC1 violation — reads as silent service loss).
    TriggeringSourceUnstated,
    /// The affected capabilities are unstated.
    AffectedCapabilitiesUnstated,
    /// No export-before-loss or renew / reopen action is offered (guardrail — loss without a safe
    /// route).
    ExportOrRenewActionMissing,
    /// The banner claims exact continuity over an expiry / material change (guardrail violation).
    ExactContinuityOverclaimed,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WorkspaceExpiryBannerDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpiryTimingUnstated,
        Self::TriggeringSourceUnstated,
        Self::AffectedCapabilitiesUnstated,
        Self::ExportOrRenewActionMissing,
        Self::ExactContinuityOverclaimed,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpiryTimingUnstated => "expiry_timing_unstated",
            Self::TriggeringSourceUnstated => "triggering_source_unstated",
            Self::AffectedCapabilitiesUnstated => "affected_capabilities_unstated",
            Self::ExportOrRenewActionMissing => "export_or_renew_action_missing",
            Self::ExactContinuityOverclaimed => "exact_continuity_overclaimed",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ExpiryContinuationNextAction {
        match self {
            Self::ExportOrRenewActionMissing => M5ExpiryContinuationNextAction::ExportBeforeLoss,
            Self::ExactContinuityOverclaimed => M5ExpiryContinuationNextAction::ContinueLocalSafe,
            Self::ExpiryTimingUnstated
            | Self::TriggeringSourceUnstated
            | Self::AffectedCapabilitiesUnstated
            | Self::ProofStale => M5ExpiryContinuationNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::ExpiryTimingUnstated => M5BuildRemoteDowngradeTrigger::ExpiryTimingUnstated,
            Self::ExactContinuityOverclaimed => {
                M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed
            }
            Self::ExportOrRenewActionMissing => {
                M5BuildRemoteDowngradeTrigger::LocalSafeOrCompanionHandoffOverflowOnly
            }
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
            Self::TriggeringSourceUnstated | Self::AffectedCapabilitiesUnstated => {
                M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed
            }
        }
    }
}

/// Reason a local-safe continuation card degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LocalSafeContinuationCardDegradeReason {
    /// The preserved files / context are unstated (AC2 violation).
    PreservedContextUnstated,
    /// The lost live state is unstated (AC2 violation).
    LostLiveStateUnstated,
    /// The next safe actions are unstated.
    NextSafeActionsUnstated,
    /// The card offers no local-safe continuation route (guardrail violation).
    LocalSafeContinuationUnavailable,
    /// The card claims exact continuity over a material change (guardrail violation).
    ExactContinuityOverclaimed,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5LocalSafeContinuationCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreservedContextUnstated,
        Self::LostLiveStateUnstated,
        Self::NextSafeActionsUnstated,
        Self::LocalSafeContinuationUnavailable,
        Self::ExactContinuityOverclaimed,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedContextUnstated => "preserved_context_unstated",
            Self::LostLiveStateUnstated => "lost_live_state_unstated",
            Self::NextSafeActionsUnstated => "next_safe_actions_unstated",
            Self::LocalSafeContinuationUnavailable => "local_safe_continuation_unavailable",
            Self::ExactContinuityOverclaimed => "exact_continuity_overclaimed",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ExpiryContinuationNextAction {
        match self {
            Self::PreservedContextUnstated | Self::LostLiveStateUnstated | Self::ProofStale => {
                M5ExpiryContinuationNextAction::ReviewDiagnostics
            }
            Self::NextSafeActionsUnstated
            | Self::LocalSafeContinuationUnavailable
            | Self::ExactContinuityOverclaimed => M5ExpiryContinuationNextAction::ContinueLocalSafe,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::NextSafeActionsUnstated | Self::LocalSafeContinuationUnavailable => {
                M5BuildRemoteDowngradeTrigger::LocalSafeOrCompanionHandoffOverflowOnly
            }
            Self::ExactContinuityOverclaimed => {
                M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed
            }
            Self::ProofStale => M5BuildRemoteDowngradeTrigger::ProofStale,
            Self::PreservedContextUnstated | Self::LostLiveStateUnstated => {
                M5BuildRemoteDowngradeTrigger::GenericStatusWordingUsed
            }
        }
    }
}

/// Input to [`resolve_workspace_expiry_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkspaceExpiryBannerResolutionInput {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// The workspace label / target identity (empty means unstated).
    pub workspace_label: String,
    /// The expiry window governing the workspace.
    pub expiry_class: ExpiryClass,
    /// True when the exact expiry timing is disclosed on the banner, never generic.
    pub expiry_disclosed: bool,
    /// The triggering owner / source reason.
    pub triggering_reason: TransitionReasonClass,
    /// True when the triggering owner / source is disclosed on the banner.
    pub triggering_source_disclosed: bool,
    /// The capabilities affected by the expiry.
    pub affected_capabilities: Vec<M5WorkspaceLiveCapability>,
    /// True when the affected capabilities are disclosed on the banner.
    pub capabilities_disclosed: bool,
    /// The actions offered before loss (export-before-loss, renew, reopen, ...).
    pub offered_actions: Vec<M5WorkspaceExpiryAction>,
    /// True when renew / reopen is allowed on this deployment line.
    pub renew_reopen_allowed: bool,
    /// The claimed continuity relationship to the prior runtime.
    pub continuity_class: ContinuityClass,
    /// True when the runtime changed materially / is gone relative to the one the user last saw.
    pub material_change_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe workspace-expiry banner projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWorkspaceExpiryBanner {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// The workspace label / target identity named by the banner.
    pub workspace_label: String,
    /// Expiry-class token named by the banner.
    pub expiry_class: String,
    /// Triggering owner / source token named by the banner.
    pub triggering_reason: String,
    /// Continuity-class token named by the banner.
    pub continuity_class: String,
    /// Affected-capability tokens named by the banner.
    pub affected_capabilities: Vec<String>,
    /// Offered-action tokens named by the banner.
    pub offered_actions: Vec<String>,
    /// Whether renew / reopen is allowed on this deployment line.
    pub renew_reopen_allowed: bool,
    /// Whether the runtime changed materially / is gone relative to the one the user last saw.
    pub material_change_present: bool,
    /// Degrade reason, if the banner could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5WorkspaceExpiryBannerDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ExpiryContinuationNextAction,
    /// AC1: whether the exact expiry timing is disclosed on the banner.
    pub expiry_disclosed: bool,
    /// AC1: whether the triggering owner / source is disclosed on the banner.
    pub triggering_source_disclosed: bool,
    /// Whether the affected capabilities are disclosed on the banner.
    pub capabilities_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean banner): the expiry reads as a generic disconnect or a
    /// silent service loss.
    pub appears_as_generic_disconnect_or_silent_loss: bool,
    /// Guardrail (MUST be `false` on a clean banner): the banner implies exact continuity after an
    /// expiry / material change.
    pub implies_exact_continuity_after_material_change: bool,
}

impl M5ResolvedWorkspaceExpiryBanner {
    /// Whether this banner reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this banner misrepresents the expiry — reads as a generic disconnect / silent loss or
    /// implies exact continuity over a change (a guardrail violation).
    pub fn misrepresents_expiry(&self) -> bool {
        self.appears_as_generic_disconnect_or_silent_loss
            || self.implies_exact_continuity_after_material_change
    }
}

/// Input to [`resolve_local_safe_continuation_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LocalSafeContinuationCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The workspace label / target identity (empty means unstated).
    pub workspace_label: String,
    /// The persistence class backing the local-safe mirror.
    pub persistence_class: PersistenceClass,
    /// The claimed continuity relationship to the prior runtime.
    pub continuity_class: ContinuityClass,
    /// The preserved files / context that remain local-safe.
    pub preserved_context: Vec<M5PreservedContextClass>,
    /// True when the preserved files / context are disclosed on the card.
    pub preserved_disclosed: bool,
    /// The lost live state (terminals, ports, kernels, previews, ...).
    pub lost_live_state: Vec<M5WorkspaceLiveCapability>,
    /// True when the lost live state is disclosed on the card.
    pub lost_disclosed: bool,
    /// The next safe actions offered (continue locally, reconnect, rebuild, ...).
    pub next_actions: Vec<RecoveryOptionClass>,
    /// True when the next safe actions are disclosed on the card.
    pub next_actions_disclosed: bool,
    /// True when the runtime changed materially / is gone relative to the one the user last saw.
    pub material_change_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe local-safe continuation card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLocalSafeContinuationCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The workspace label / target identity named by the card.
    pub workspace_label: String,
    /// Persistence-class token named by the card.
    pub persistence_class: String,
    /// Continuity-class token named by the card.
    pub continuity_class: String,
    /// Preserved-context tokens named by the card.
    pub preserved_context: Vec<String>,
    /// Lost live-state tokens named by the card.
    pub lost_live_state: Vec<String>,
    /// Next-safe-action tokens named by the card.
    pub next_actions: Vec<String>,
    /// Whether the runtime changed materially / is gone relative to the one the user last saw.
    pub material_change_present: bool,
    /// Whether the card offers a continue-locally route.
    pub offers_continue_locally: bool,
    /// Degrade reason, if the card could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5LocalSafeContinuationCardDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ExpiryContinuationNextAction,
    /// AC2: whether the preserved files / context are disclosed on the card.
    pub preserved_disclosed: bool,
    /// AC2: whether the lost live state is disclosed on the card.
    pub lost_disclosed: bool,
    /// Whether the next safe actions are disclosed on the card.
    pub next_actions_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean card): the card offers no local-safe continuation route.
    pub local_safe_continuation_unavailable: bool,
    /// Guardrail (MUST be `false` on a clean card): the card claims exact continuity over a material
    /// change.
    pub overclaims_exact_continuity: bool,
}

impl M5ResolvedLocalSafeContinuationCard {
    /// Whether this card reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }

    /// Whether this card misrepresents the continuation — hides local-safe continuation or overclaims
    /// continuity (a guardrail violation).
    pub fn misrepresents_continuation(&self) -> bool {
        self.local_safe_continuation_unavailable || self.overclaims_exact_continuity
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ExpiryContinuationResolutionError {
    /// The banner id was empty.
    EmptyBannerId,
    /// The card id was empty.
    EmptyCardId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ExpiryContinuationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBannerId => "empty_banner_id",
            Self::EmptyCardId => "empty_card_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ExpiryContinuationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 expiry-continuation resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ExpiryContinuationResolutionError {}

/// Resolves a workspace-expiry banner, proving AC1: an expiry event names its exact timing, its
/// triggering owner / source, the affected capabilities, and an export-before-loss or renew / reopen
/// action, so it never reads as a generic disconnect or a silent service loss.
pub fn resolve_workspace_expiry_banner(
    input: M5WorkspaceExpiryBannerResolutionInput,
) -> Result<M5ResolvedWorkspaceExpiryBanner, M5ExpiryContinuationResolutionError> {
    if input.banner_id.trim().is_empty() {
        return Err(M5ExpiryContinuationResolutionError::EmptyBannerId);
    }
    if string_is_forbidden(&input.banner_id) || string_is_forbidden(&input.workspace_label) {
        return Err(M5ExpiryContinuationResolutionError::ForbiddenMaterial);
    }

    let capabilities_stated =
        input.capabilities_disclosed && !input.affected_capabilities.is_empty();
    let appears_as_generic_disconnect_or_silent_loss =
        !input.expiry_disclosed || !input.triggering_source_disclosed;
    let implies_exact_continuity_after_material_change =
        input.material_change_present && input.continuity_class.claims_exact_continuity();

    let degrade_reason = if !input.expiry_disclosed {
        Some(M5WorkspaceExpiryBannerDegradeReason::ExpiryTimingUnstated)
    } else if !input.triggering_source_disclosed {
        Some(M5WorkspaceExpiryBannerDegradeReason::TriggeringSourceUnstated)
    } else if !capabilities_stated {
        Some(M5WorkspaceExpiryBannerDegradeReason::AffectedCapabilitiesUnstated)
    } else if input.offered_actions.is_empty() {
        Some(M5WorkspaceExpiryBannerDegradeReason::ExportOrRenewActionMissing)
    } else if implies_exact_continuity_after_material_change {
        Some(M5WorkspaceExpiryBannerDegradeReason::ExactContinuityOverclaimed)
    } else if !input.proof_fresh {
        Some(M5WorkspaceExpiryBannerDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ExpiryContinuationNextAction::ExportBeforeLoss,
    };

    Ok(M5ResolvedWorkspaceExpiryBanner {
        banner_id: input.banner_id,
        workspace_label: input.workspace_label,
        expiry_class: input.expiry_class.as_str().to_owned(),
        triggering_reason: input.triggering_reason.as_str().to_owned(),
        continuity_class: input.continuity_class.as_str().to_owned(),
        affected_capabilities: input
            .affected_capabilities
            .iter()
            .map(|capability| capability.as_str().to_owned())
            .collect(),
        offered_actions: input
            .offered_actions
            .iter()
            .map(|action| action.as_str().to_owned())
            .collect(),
        renew_reopen_allowed: input.renew_reopen_allowed,
        material_change_present: input.material_change_present,
        degrade_reason,
        next_action,
        expiry_disclosed: input.expiry_disclosed,
        triggering_source_disclosed: input.triggering_source_disclosed,
        capabilities_disclosed: input.capabilities_disclosed,
        appears_as_generic_disconnect_or_silent_loss,
        implies_exact_continuity_after_material_change,
    })
}

/// Resolves a local-safe continuation card, proving AC2: a user can see what remains local-safe
/// (preserved files / context) and what must be reattached or rerun (lost live state), with a next
/// safe action such as continue locally, reconnect, or rebuild.
pub fn resolve_local_safe_continuation_card(
    input: M5LocalSafeContinuationCardResolutionInput,
) -> Result<M5ResolvedLocalSafeContinuationCard, M5ExpiryContinuationResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5ExpiryContinuationResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id) || string_is_forbidden(&input.workspace_label) {
        return Err(M5ExpiryContinuationResolutionError::ForbiddenMaterial);
    }

    let preserved_stated = input.preserved_disclosed && !input.preserved_context.is_empty();
    let lost_stated = input.lost_disclosed && !input.lost_live_state.is_empty();
    let actions_stated = input.next_actions_disclosed && !input.next_actions.is_empty();
    let offers_continue_locally = input
        .next_actions
        .iter()
        .any(|action| matches!(action, RecoveryOptionClass::LocalSafeContinue));
    let local_safe_continuation_unavailable = !offers_continue_locally;
    let overclaims_exact_continuity =
        input.material_change_present && input.continuity_class.claims_exact_continuity();

    let degrade_reason = if !preserved_stated {
        Some(M5LocalSafeContinuationCardDegradeReason::PreservedContextUnstated)
    } else if !lost_stated {
        Some(M5LocalSafeContinuationCardDegradeReason::LostLiveStateUnstated)
    } else if !actions_stated {
        Some(M5LocalSafeContinuationCardDegradeReason::NextSafeActionsUnstated)
    } else if local_safe_continuation_unavailable {
        Some(M5LocalSafeContinuationCardDegradeReason::LocalSafeContinuationUnavailable)
    } else if overclaims_exact_continuity {
        Some(M5LocalSafeContinuationCardDegradeReason::ExactContinuityOverclaimed)
    } else if !input.proof_fresh {
        Some(M5LocalSafeContinuationCardDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ExpiryContinuationNextAction::ContinueLocalSafe,
    };

    Ok(M5ResolvedLocalSafeContinuationCard {
        card_id: input.card_id,
        workspace_label: input.workspace_label,
        persistence_class: input.persistence_class.as_str().to_owned(),
        continuity_class: input.continuity_class.as_str().to_owned(),
        preserved_context: input
            .preserved_context
            .iter()
            .map(|preserved| preserved.as_str().to_owned())
            .collect(),
        lost_live_state: input
            .lost_live_state
            .iter()
            .map(|lost| lost.as_str().to_owned())
            .collect(),
        next_actions: input
            .next_actions
            .iter()
            .map(|action| action.as_str().to_owned())
            .collect(),
        material_change_present: input.material_change_present,
        offers_continue_locally,
        degrade_reason,
        next_action,
        preserved_disclosed: input.preserved_disclosed,
        lost_disclosed: input.lost_disclosed,
        next_actions_disclosed: input.next_actions_disclosed,
        local_safe_continuation_unavailable,
        overclaims_exact_continuity,
    })
}

/// One controls row: one consumer surface bound to the resolved expiry banner and local-safe
/// continuation card examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExpiryContinuationControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ExpiryContinuationConsumerSurface,
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
    pub anatomy_parts: Vec<M5ExpiryContinuationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ExpiryContinuationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    /// Resolved workspace-expiry banner examples.
    pub expiry_banner_examples: Vec<M5ResolvedWorkspaceExpiryBanner>,
    /// Resolved local-safe continuation card examples.
    pub local_safe_card_examples: Vec<M5ResolvedLocalSafeContinuationCard>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never imply exact continuity after an expiry / material change.
    pub implies_exact_continuity_after_material_change: bool,
    /// Hard invariant: never hide local-safe continuation or companion handoff behind overflow-only
    /// affordances.
    pub hides_local_safe_or_companion_handoff_in_overflow_only: bool,
    /// Hard invariant: never let an expiry event read as a generic disconnect or a silent service
    /// loss.
    pub expiry_appears_as_generic_disconnect_or_silent_loss: bool,
    /// Hard invariant: never conceal preserved-vs-lost state or next safe actions behind generic
    /// status wording.
    pub conceals_preserved_vs_lost_state_or_next_safe_actions: bool,
}

impl M5ExpiryContinuationControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ExpiryContinuationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ExpiryContinuationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ExpiryContinuationExportField> =
            self.export_fields.iter().copied().collect();
        M5ExpiryContinuationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.implies_exact_continuity_after_material_change
            && !self.hides_local_safe_or_companion_handoff_in_overflow_only
            && !self.expiry_appears_as_generic_disconnect_or_silent_loss
            && !self.conceals_preserved_vs_lost_state_or_next_safe_actions
    }

    /// True when every resolved example on this row is honest: no clean banner misrepresents the
    /// expiry, and no clean card hides local-safe continuation or overclaims continuity.
    fn examples_are_honest(&self) -> bool {
        self.expiry_banner_examples
            .iter()
            .all(|ex| !(ex.is_clean() && ex.misrepresents_expiry()))
            && self
                .local_safe_card_examples
                .iter()
                .all(|ex| !(ex.is_clean() && ex.misrepresents_continuation()))
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExpiryContinuationVocabularySet {
    /// Boundary-disposition tokens (bound from the frozen matrix).
    pub boundary_dispositions: Vec<String>,
    /// Expiry-class tokens (bound from the managed-workspace object model).
    pub expiry_classes: Vec<String>,
    /// Triggering owner / source reason tokens (bound from the managed-workspace object model).
    pub triggering_reasons: Vec<String>,
    /// Persistence-class tokens (bound from the managed-workspace object model).
    pub persistence_classes: Vec<String>,
    /// Continuity-class tokens (bound from the managed-workspace object model).
    pub continuity_classes: Vec<String>,
    /// Recovery-option tokens (bound from the managed-workspace object model).
    pub recovery_options: Vec<String>,
    /// Live-capability tokens.
    pub live_capabilities: Vec<String>,
    /// Preserved-context tokens.
    pub preserved_context_classes: Vec<String>,
    /// Expiry-action tokens.
    pub expiry_actions: Vec<String>,
    /// Banner degrade-reason tokens.
    pub banner_degrade_reasons: Vec<String>,
    /// Card degrade-reason tokens.
    pub card_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ExpiryContinuationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` / bound arrays.
    pub fn canonical() -> Self {
        Self {
            boundary_dispositions: tokens(&M5BuildRemoteBoundaryDisposition::ALL, |v| v.as_str()),
            expiry_classes: tokens(&BOUND_EXPIRY_CLASSES, |v| v.as_str()),
            triggering_reasons: tokens(&BOUND_TRANSITION_REASONS, |v| v.as_str()),
            persistence_classes: tokens(&BOUND_PERSISTENCE_CLASSES, |v| v.as_str()),
            continuity_classes: tokens(&BOUND_CONTINUITY_CLASSES, |v| v.as_str()),
            recovery_options: tokens(&BOUND_RECOVERY_OPTIONS, |v| v.as_str()),
            live_capabilities: tokens(&M5WorkspaceLiveCapability::ALL, |v| v.as_str()),
            preserved_context_classes: tokens(&M5PreservedContextClass::ALL, |v| v.as_str()),
            expiry_actions: tokens(&M5WorkspaceExpiryAction::ALL, |v| v.as_str()),
            banner_degrade_reasons: tokens(&M5WorkspaceExpiryBannerDegradeReason::ALL, |v| {
                v.as_str()
            }),
            card_degrade_reasons: tokens(&M5LocalSafeContinuationCardDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ExpiryContinuationAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ExpiryContinuationNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ExpiryContinuationExportField::ALL, |v| v.as_str()),
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
pub struct M5ExpiryContinuationGovernanceReview {
    /// The banner always names its exact expiry timing and triggering owner / source.
    pub banner_names_expiry_timing_and_triggering_source: bool,
    /// The banner always names its affected capabilities and its offered actions.
    pub banner_names_affected_capabilities_and_actions: bool,
    /// An expiry event never reads as a generic disconnect or a silent service loss.
    pub expiry_never_appears_as_generic_disconnect: bool,
    /// The card always names its preserved files / context and its lost live state.
    pub card_names_preserved_and_lost_live_state: bool,
    /// The card always names its next safe actions.
    pub card_names_next_safe_actions: bool,
    /// Local-safe continuation is never hidden behind overflow-only affordances.
    pub local_safe_continuation_never_overflow_only: bool,
    /// A material change / expiry never implies exact continuity.
    pub material_change_never_implies_exact_continuity: bool,
    /// An export-before-loss action is always available before the runtime is lost.
    pub export_before_loss_action_always_available: bool,
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
pub struct M5ExpiryContinuationConsumerProjection {
    /// Shell surfaces consume the shared expiry / continuation vocabulary.
    pub shell_surfaces_consume_expiry_vocabulary: bool,
    /// Preview surfaces consume the shared expiry / continuation vocabulary.
    pub preview_surfaces_consume_expiry_vocabulary: bool,
    /// Companion surfaces reuse the same expiry banner and local-safe continuation cards.
    pub companion_surfaces_reuse_expiry_banner_and_continuation_cards: bool,
    /// Incident / ops surfaces consume the shared expiry / continuation vocabulary.
    pub incident_ops_consumes_expiry_vocabulary: bool,
    /// Support / export reads a single canonical expiry / continuation source.
    pub support_export_reads_single_expiry_source: bool,
    /// Expiry and fallback language stays consistent across every surface.
    pub expiry_and_fallback_language_consistent_across_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExpiryContinuationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExpiryContinuationReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting boundary audit for the lane.
    pub boundary_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ExpiryContinuationControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ExpiryContinuationControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ExpiryContinuationControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExpiryContinuationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExpiryContinuationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExpiryContinuationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExpiryContinuationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExpiryContinuationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 workspace-expiry-banner / local-safe-continuation-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExpiryContinuationControlsPacket {
    /// Record kind; must equal [`M5_EXPIRY_CONTINUATION_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5ExpiryContinuationControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ExpiryContinuationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ExpiryContinuationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ExpiryContinuationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ExpiryContinuationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ExpiryContinuationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ExpiryContinuationControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5ExpiryContinuationControlsPacketInput) -> Self {
        Self {
            record_kind: M5_EXPIRY_CONTINUATION_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ExpiryContinuationControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_EXPIRY_CONTINUATION_CONTROLS_RECORD_KIND {
            violations.push(M5ExpiryContinuationControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_VERSION {
            violations.push(M5ExpiryContinuationControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ExpiryContinuationControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ExpiryContinuationControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 expiry-continuation controls packet serializes"),
        ) {
            violations.push(M5ExpiryContinuationControlsViolation::RawMaterialInExport);
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
            .expect("m5 expiry-continuation controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,banner_examples,card_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .expiry_banner_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.local_safe_card_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.expiry_banner_examples.len(),
                row.local_safe_card_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Workspace-Expiry-Banner and Local-Safe-Continuation-Card Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Expiry windows: {}\n",
            self.vocabulary_set.expiry_classes.join(", ")
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
                "  - Banner examples: {} / card examples: {}\n",
                row.expiry_banner_examples.len(),
                row.local_safe_card_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5ExpiryContinuationControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ExpiryContinuationControlsViolation>),
}

impl fmt::Display for M5ExpiryContinuationControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 expiry-continuation controls export parse failed: {error}"
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
                    "m5 expiry-continuation controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ExpiryContinuationControlsArtifactError {}

/// Validation failures emitted by [`M5ExpiryContinuationControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ExpiryContinuationControlsViolation {
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
    /// A controls row carries a dishonest clean example (generic disconnect, implied exact
    /// continuity, hidden local-safe continuation, or overclaimed continuity).
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
    /// AC1 is not proven: clean banners do not cover every expiry window, no
    /// timing/source/export-action banner degrades, or a clean banner reads as a generic disconnect
    /// / implies exact continuity after a material change.
    Ac1NotProven,
    /// AC2 is not proven: no preserved/lost/local-safe-unavailable card degrades, no clean card
    /// shows both preserved and lost state, or a clean card hides local-safe continuation /
    /// overclaims continuity.
    Ac2NotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ExpiryContinuationControlsViolation {
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
pub fn current_stable_m5_expiry_continuation_controls_export(
) -> Result<M5ExpiryContinuationControlsPacket, M5ExpiryContinuationControlsArtifactError> {
    let packet: M5ExpiryContinuationControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workspace-expiry-banner-local-safe-continuation-card-controls-proof/support_export.json"
    )))
    .map_err(M5ExpiryContinuationControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExpiryContinuationControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_EXPIRY_CONTINUATION_CONTROLS_SCHEMA_REF,
        M5_EXPIRY_CONTINUATION_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF,
        M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ExpiryContinuationControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5ExpiryContinuationControlsViolation::NoControlsRows);
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
            violations.push(M5ExpiryContinuationControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ExpiryContinuationControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ExpiryContinuationControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_WORKSPACE_EXPIRY_BANNER_SCHEMA_REF)
            || !refs.contains(M5_LOCAL_SAFE_CONTINUATION_CARD_SCHEMA_REF)
        {
            violations.push(M5ExpiryContinuationControlsViolation::ComponentSchemaRefMissing);
        }
        if row.expiry_banner_examples.is_empty() || row.local_safe_card_examples.is_empty() {
            violations.push(M5ExpiryContinuationControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ExpiryContinuationControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ExpiryContinuationControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.banner_names_expiry_timing_and_triggering_source,
        review.banner_names_affected_capabilities_and_actions,
        review.expiry_never_appears_as_generic_disconnect,
        review.card_names_preserved_and_lost_live_state,
        review.card_names_next_safe_actions,
        review.local_safe_continuation_never_overflow_only,
        review.material_change_never_implies_exact_continuity,
        review.export_before_loss_action_always_available,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ExpiryContinuationControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_expiry_vocabulary,
        projection.preview_surfaces_consume_expiry_vocabulary,
        projection.companion_surfaces_reuse_expiry_banner_and_continuation_cards,
        projection.incident_ops_consumes_expiry_vocabulary,
        projection.support_export_reads_single_expiry_source,
        projection.expiry_and_fallback_language_consistent_across_surfaces,
    ] {
        if !ok {
            violations.push(M5ExpiryContinuationControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ExpiryContinuationControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ExpiryContinuationControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ExpiryContinuationControlsPacket,
    violations: &mut Vec<M5ExpiryContinuationControlsViolation>,
) {
    let banner_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.expiry_banner_examples.iter())
    };
    let card_examples = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.local_safe_card_examples.iter())
    };

    // AC1: expiry events no longer appear as generic disconnects or silent service loss — clean
    // banners cover every governing expiry window, a timing-unstated banner degrades, a
    // source-unstated banner degrades, an export-action-missing banner degrades, an
    // exact-continuity-overclaimed banner degrades, and no clean banner reads as a generic
    // disconnect or implies exact continuity after a material change.
    let clean_windows: BTreeSet<&str> = banner_examples()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.expiry_class.as_str())
        .collect();
    let covers_all_windows = EXPIRY_WINDOW_CLASSES
        .iter()
        .all(|window| clean_windows.contains(window.as_str()));
    let timing_unstated_degrades = banner_examples().any(|ex| {
        ex.degrade_reason == Some(M5WorkspaceExpiryBannerDegradeReason::ExpiryTimingUnstated)
    });
    let source_unstated_degrades = banner_examples().any(|ex| {
        ex.degrade_reason == Some(M5WorkspaceExpiryBannerDegradeReason::TriggeringSourceUnstated)
    });
    let export_action_missing_degrades = banner_examples().any(|ex| {
        ex.degrade_reason == Some(M5WorkspaceExpiryBannerDegradeReason::ExportOrRenewActionMissing)
    });
    let banner_overclaim_degrades = banner_examples().any(|ex| {
        ex.degrade_reason == Some(M5WorkspaceExpiryBannerDegradeReason::ExactContinuityOverclaimed)
            && ex.implies_exact_continuity_after_material_change
    });
    let no_clean_banner_misrepresents =
        banner_examples().all(|ex| !(ex.is_clean() && ex.misrepresents_expiry()));
    if !(covers_all_windows
        && timing_unstated_degrades
        && source_unstated_degrades
        && export_action_missing_degrades
        && banner_overclaim_degrades
        && no_clean_banner_misrepresents)
    {
        violations.push(M5ExpiryContinuationControlsViolation::Ac1NotProven);
    }

    // AC2: users can see what remains local-safe and what must be reattached or rerun — a
    // preserved-unstated card degrades, a lost-unstated card degrades, a local-safe-unavailable card
    // degrades, an exact-continuity-overclaimed card degrades, at least one clean card names both
    // preserved and lost state, and no clean card hides local-safe continuation or overclaims
    // continuity.
    let preserved_unstated_degrades = card_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5LocalSafeContinuationCardDegradeReason::PreservedContextUnstated)
    });
    let lost_unstated_degrades = card_examples().any(|ex| {
        ex.degrade_reason == Some(M5LocalSafeContinuationCardDegradeReason::LostLiveStateUnstated)
    });
    let local_safe_unavailable_degrades = card_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5LocalSafeContinuationCardDegradeReason::LocalSafeContinuationUnavailable)
    });
    let card_overclaim_degrades = card_examples().any(|ex| {
        ex.degrade_reason
            == Some(M5LocalSafeContinuationCardDegradeReason::ExactContinuityOverclaimed)
            && ex.overclaims_exact_continuity
    });
    let clean_card_shows_preserved_and_lost =
        card_examples().any(|ex| ex.is_clean() && ex.preserved_disclosed && ex.lost_disclosed);
    let no_clean_card_misrepresents =
        card_examples().all(|ex| !(ex.is_clean() && ex.misrepresents_continuation()));
    if !(preserved_unstated_degrades
        && lost_unstated_degrades
        && local_safe_unavailable_degrades
        && card_overclaim_degrades
        && clean_card_shows_preserved_and_lost
        && no_clean_card_misrepresents)
    {
        violations.push(M5ExpiryContinuationControlsViolation::Ac2NotProven);
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
