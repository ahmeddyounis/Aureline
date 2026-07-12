//! Implemented M5 dialog / sheet and consequence-block primitives.
//!
//! The frozen [decision / feedback component matrix][matrix] names Aureline's ubiquitous decision and
//! feedback primitives and locks their controlled vocabulary. This module is the second implement lane
//! over that matrix: it turns the two highest-risk confirmation primitives — the **dialog / sheet** and
//! the **consequence block** — into resolvers that produce export-safe, honest projections, so a user
//! can trust that a high-risk confirmation always names its rationale, scope, and explicit actions,
//! keeps a safe initial focus and a cancel path, returns focus when reopened from status, activity,
//! support, or a deep link, and carries a consequence block that names the affected object, blast
//! radius, and rollback / help posture rather than reducing to generic Yes/No ambiguity — whether it
//! appears in a trust prompt, a review confirmation, a repair flow, an update / install step, or a
//! destructive delete.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement dialogs and sheets with stable title / rationale / consequence anatomy, explicit
//!   action labels, safe initial focus, cancel paths, help / docs hooks, and no generic Yes/No
//!   ambiguity.** [`resolve_dialog`] refuses to read as a clean, trustworthy dialog when the title is
//!   unstated, the surface context is unresolved, the action model is the disallowed generic-yes-no
//!   token, the rationale or scope is unstated, the actions are not explicitly named, the initial focus
//!   is unsafe or unresolved, the cancel path is missing, focus does not return when the dialog is
//!   reopened, the reopen origin is unresolved, or the help / docs hook is missing; it degrades instead.
//! * **Implement reusable consequence blocks that name affected object / scope, blast radius, rollback /
//!   help posture, and partial-success or irreversible notes where relevant.** [`resolve_consequence`]
//!   degrades when the consequence label is unstated, the surface context is unresolved, the disclosure
//!   is the disallowed generic-yes-no token, the affected object is unnamed, the blast radius or
//!   reversibility is unresolved, the rollback / help posture is unstated, the partial-success or
//!   irreversible note is missing, the block reduces to generic Yes/No ambiguity, or the explanation is
//!   reachable only via a screenshot rather than by keyboard, screen reader, and export.
//! * **Preserve focus-return, open-from-notification continuity, and export / help parity when the same
//!   decision surface is reopened from status, activity, support, or deep-link flows.** Both resolvers
//!   carry the surface context and the dialog reopen origin so a broken focus return or a lost reopen
//!   continuity degrades honestly rather than silently.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5DecisionFeedbackDisposition`] state vocabulary, the [`M5DialogActionModel`] dialog-action
//! vocabulary, and the [`M5ConsequenceDisclosure`] consequence-disclosure vocabulary — so trust,
//! review, repair, update, install, and support surfaces can never fork their own state, dialog-action,
//! or consequence-disclosure wording. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::m5_decision_feedback_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_dialog_consequence_controls,
    seeded_m5_dialog_consequence_controls_review_ui_beta_narrowed,
    seeded_m5_dialog_consequence_controls_updates_ui_preview_narrowed,
    M5_DIALOG_CONSEQUENCE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_decision_feedback_component_matrix::{
    M5ConsequenceDisclosure, M5DecisionFeedbackAccessibilityRoute,
    M5DecisionFeedbackConsumerSurface, M5DecisionFeedbackDeploymentLine,
    M5DecisionFeedbackDisposition, M5DecisionFeedbackDowngradeTrigger, M5DecisionFeedbackFamily,
    M5DecisionFeedbackQualificationClass, M5DecisionFeedbackRequiredLabel, M5DialogActionModel,
    M5_CONSEQUENCE_BLOCK_SCHEMA_REF, M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
    M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_DIALOG_SHEET_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5DialogConsequenceControlsPacket`].
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_dialog_sheet_and_consequence_block_controls";

/// Schema version for M5 dialog / consequence controls records.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-dialog-sheet-and-consequence-block-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_DOC_REF: &str =
    "docs/components/m5_dialog_sheet_and_consequence_block_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-dialog-sheet-and-consequence-block-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-dialog-sheet-and-consequence-block-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-dialog-sheet-and-consequence-block-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DIALOG_CONSEQUENCE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-dialog-sheet-and-consequence-block-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5DialogConsequenceConsumerSurface = M5DecisionFeedbackConsumerSurface;

/// Controlled render context — which claimed high-risk M5 surface renders the primitive, so a dialog or
/// consequence block's rationale, scope, and recovery truth stay stable whether it appears in a trust
/// prompt, a review confirmation, a repair confirmation, an update / install step, or a destructive
/// delete. Minted by this lane, tracking the exit-gate confirmation surfaces directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionActionSurfaceContext {
    /// A trust prompt / capability grant.
    TrustPrompt,
    /// A review confirmation.
    ReviewConfirmation,
    /// A repair / recovery confirmation.
    RepairConfirmation,
    /// An update or install confirmation.
    UpdateOrInstall,
    /// A destructive delete confirmation.
    DestructiveDelete,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5DecisionActionSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TrustPrompt,
        Self::ReviewConfirmation,
        Self::RepairConfirmation,
        Self::UpdateOrInstall,
        Self::DestructiveDelete,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustPrompt => "trust_prompt",
            Self::ReviewConfirmation => "review_confirmation",
            Self::RepairConfirmation => "repair_confirmation",
            Self::UpdateOrInstall => "update_or_install",
            Self::DestructiveDelete => "destructive_delete",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled safe initial focus target a dialog / sheet lands on when it opens, so a high-risk dialog
/// never auto-focuses a destructive action: it focuses the least-destructive action, the cancel
/// control, the first input, the rationale heading, or a named primary action. Minted by this lane
/// because the frozen matrix carries the dialog *action model* but not the initial-focus posture the
/// dialog acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogFocusTarget {
    /// Focus lands on the least-destructive action.
    FocusesLeastDestructiveAction,
    /// Focus lands on the cancel control.
    FocusesCancelControl,
    /// Focus lands on the first input.
    FocusesFirstInput,
    /// Focus lands on the rationale heading.
    FocusesRationaleHeading,
    /// Focus lands on a named (non-destructive) primary action.
    FocusesNamedPrimaryAction,
    /// The initial focus target cannot currently be resolved.
    FocusTargetUnknown,
}

impl M5DialogFocusTarget {
    /// Every focus target, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FocusesLeastDestructiveAction,
        Self::FocusesCancelControl,
        Self::FocusesFirstInput,
        Self::FocusesRationaleHeading,
        Self::FocusesNamedPrimaryAction,
        Self::FocusTargetUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusesLeastDestructiveAction => "focuses_least_destructive_action",
            Self::FocusesCancelControl => "focuses_cancel_control",
            Self::FocusesFirstInput => "focuses_first_input",
            Self::FocusesRationaleHeading => "focuses_rationale_heading",
            Self::FocusesNamedPrimaryAction => "focuses_named_primary_action",
            Self::FocusTargetUnknown => "focus_target_unknown",
        }
    }

    /// Whether the focus target is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::FocusTargetUnknown)
    }
}

/// Controlled reopen origin for a dialog / sheet, so a decision surface reopened from status, the
/// activity center, support, or a deep link keeps the same rationale, scope, and focus-return truth as
/// a fresh invocation. Minted by this lane because the frozen matrix carries no reopen-continuity
/// concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogReopenOrigin {
    /// A fresh, first-time invocation.
    FreshInvocation,
    /// Reopened from a status indicator.
    ReopenedFromStatus,
    /// Reopened from the activity center.
    ReopenedFromActivityCenter,
    /// Reopened from a support flow.
    ReopenedFromSupport,
    /// Reopened from a deep link.
    ReopenedFromDeepLink,
    /// The reopen origin cannot currently be resolved.
    OriginUnknown,
}

impl M5DialogReopenOrigin {
    /// Every reopen origin, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FreshInvocation,
        Self::ReopenedFromStatus,
        Self::ReopenedFromActivityCenter,
        Self::ReopenedFromSupport,
        Self::ReopenedFromDeepLink,
        Self::OriginUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshInvocation => "fresh_invocation",
            Self::ReopenedFromStatus => "reopened_from_status",
            Self::ReopenedFromActivityCenter => "reopened_from_activity_center",
            Self::ReopenedFromSupport => "reopened_from_support",
            Self::ReopenedFromDeepLink => "reopened_from_deep_link",
            Self::OriginUnknown => "origin_unknown",
        }
    }

    /// Whether the reopen origin is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::OriginUnknown)
    }

    /// Whether this origin is a reopen (not a fresh first-time invocation).
    pub const fn is_reopen(self) -> bool {
        matches!(
            self,
            Self::ReopenedFromStatus
                | Self::ReopenedFromActivityCenter
                | Self::ReopenedFromSupport
                | Self::ReopenedFromDeepLink
        )
    }
}

/// Controlled blast radius a consequence block names, so a risky action always states how far it
/// reaches: a single object, multiple objects, the whole workspace, the whole deployment, or an
/// irreversible external effect. Minted by this lane because the frozen matrix carries no blast-radius
/// classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConsequenceBlastRadius {
    /// Affects a single named object.
    SingleObject,
    /// Affects multiple named objects.
    MultipleObjects,
    /// Affects the whole workspace.
    WorkspaceWide,
    /// Affects the whole deployment.
    DeploymentWide,
    /// Causes an irreversible external effect.
    IrreversibleExternal,
    /// The blast radius cannot currently be resolved.
    RadiusUnknown,
}

impl M5ConsequenceBlastRadius {
    /// Every blast radius, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleObject,
        Self::MultipleObjects,
        Self::WorkspaceWide,
        Self::DeploymentWide,
        Self::IrreversibleExternal,
        Self::RadiusUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleObject => "single_object",
            Self::MultipleObjects => "multiple_objects",
            Self::WorkspaceWide => "workspace_wide",
            Self::DeploymentWide => "deployment_wide",
            Self::IrreversibleExternal => "irreversible_external",
            Self::RadiusUnknown => "radius_unknown",
        }
    }

    /// Whether the blast radius is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::RadiusUnknown)
    }

    /// Whether this radius reaches beyond a single object and therefore demands extra disclosure.
    pub const fn is_broad(self) -> bool {
        matches!(
            self,
            Self::MultipleObjects
                | Self::WorkspaceWide
                | Self::DeploymentWide
                | Self::IrreversibleExternal
        )
    }
}

/// Controlled reversibility posture a consequence block names, so a risky action always states whether
/// it can be rolled back, needs named rollback steps, may partially succeed, is irreversible, or needs
/// manual recovery. Minted by this lane because the frozen matrix carries no reversibility
/// classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConsequenceReversibility {
    /// Fully reversible with no extra steps.
    FullyReversible,
    /// Reversible through named rollback steps.
    RollbackWithNamedSteps,
    /// May partially succeed, and that is stated.
    PartialSuccessPossible,
    /// Irreversible, and that is stated.
    IrreversibleAndStated,
    /// Requires manual recovery, and that is stated.
    RequiresManualRecovery,
    /// The reversibility posture cannot currently be resolved.
    ReversibilityUnknown,
}

impl M5ConsequenceReversibility {
    /// Every reversibility posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyReversible,
        Self::RollbackWithNamedSteps,
        Self::PartialSuccessPossible,
        Self::IrreversibleAndStated,
        Self::RequiresManualRecovery,
        Self::ReversibilityUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReversible => "fully_reversible",
            Self::RollbackWithNamedSteps => "rollback_with_named_steps",
            Self::PartialSuccessPossible => "partial_success_possible",
            Self::IrreversibleAndStated => "irreversible_and_stated",
            Self::RequiresManualRecovery => "requires_manual_recovery",
            Self::ReversibilityUnknown => "reversibility_unknown",
        }
    }

    /// Whether the reversibility posture is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ReversibilityUnknown)
    }

    /// Whether this posture is irreversible or only partially reversible, so the block must carry a
    /// partial-success or irreversible note.
    pub const fn needs_partial_or_irreversible_note(self) -> bool {
        matches!(
            self,
            Self::PartialSuccessPossible
                | Self::IrreversibleAndStated
                | Self::RequiresManualRecovery
        )
    }
}

/// One mandatory rendered part a dialog or consequence block must be able to show, so no rationale,
/// scope, action, focus, cancel, blast-radius, or rollback fact is left implicit behind generic chrome
/// or a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogConsequenceAnatomyPart {
    /// The primitive's stable identity / permanent title.
    Identity,
    /// The primitive's current typed state disposition.
    State,
    /// The non-visual keyboard route to the primitive.
    KeyboardRoute,
    /// The rationale for why this decision is shown (dialog).
    Rationale,
    /// The named scope the decision applies to (both primitives).
    Scope,
    /// The explicit action labels (dialog).
    ExplicitActions,
    /// The safe initial focus target (dialog).
    SafeInitialFocus,
    /// The cancel / escape path (dialog).
    CancelPath,
    /// The help / docs hook (dialog).
    HelpDocsHook,
    /// The named blast radius (consequence).
    BlastRadius,
    /// The rollback / help posture (consequence).
    RollbackPosture,
}

impl M5DialogConsequenceAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::Rationale,
        Self::Scope,
        Self::ExplicitActions,
        Self::SafeInitialFocus,
        Self::CancelPath,
        Self::HelpDocsHook,
        Self::BlastRadius,
        Self::RollbackPosture,
    ];

    /// The three parts every claimed primitive must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::Rationale => "rationale",
            Self::Scope => "scope",
            Self::ExplicitActions => "explicit_actions",
            Self::SafeInitialFocus => "safe_initial_focus",
            Self::CancelPath => "cancel_path",
            Self::HelpDocsHook => "help_docs_hook",
            Self::BlastRadius => "blast_radius",
            Self::RollbackPosture => "rollback_posture",
        }
    }
}

/// Next safe action a primitive surfaces so a user is never left without a route to read rationale,
/// review the consequence block, choose an explicit action, return focus, or open help.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogConsequenceNextAction {
    /// Read the dialog's rationale and named scope.
    ReviewRationaleAndScope,
    /// Review the consequence block (blast radius, rollback posture).
    ReviewConsequenceBlock,
    /// Choose an explicit, named action.
    ChooseExplicitAction,
    /// Return focus to the invoker that opened the dialog.
    ReturnFocusToInvoker,
    /// Open the help / docs hook.
    OpenHelpOrDocs,
    /// No action is needed; the primitive is clean.
    NoActionNeeded,
}

impl M5DialogConsequenceNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewRationaleAndScope,
        Self::ReviewConsequenceBlock,
        Self::ChooseExplicitAction,
        Self::ReturnFocusToInvoker,
        Self::OpenHelpOrDocs,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewRationaleAndScope => "review_rationale_and_scope",
            Self::ReviewConsequenceBlock => "review_consequence_block",
            Self::ChooseExplicitAction => "choose_explicit_action",
            Self::ReturnFocusToInvoker => "return_focus_to_invoker",
            Self::OpenHelpOrDocs => "open_help_or_docs",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogConsequenceExportField {
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
    /// The dialog action model named by the dialog.
    DialogActionModel,
    /// The consequence disclosure named by the consequence block.
    ConsequenceDisclosure,
    /// The render / surface context named by both primitives.
    SurfaceContext,
    /// The reopen origin named by the dialog.
    ReopenOrigin,
    /// The blast radius named by the consequence block.
    BlastRadius,
    /// The accountable owner role.
    OwnerRole,
}

impl M5DialogConsequenceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::DialogActionModel,
        Self::ConsequenceDisclosure,
        Self::SurfaceContext,
        Self::ReopenOrigin,
        Self::BlastRadius,
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
            Self::DialogActionModel => "dialog_action_model",
            Self::ConsequenceDisclosure => "consequence_disclosure",
            Self::SurfaceContext => "surface_context",
            Self::ReopenOrigin => "reopen_origin",
            Self::BlastRadius => "blast_radius",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a dialog degraded below a clean, trustworthy confirmation state. The degrade-first ladder
/// returns one of these instead of ever letting a generic-yes-no, rationale-less, or unsafe-focus
/// dialog read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DialogDegradeReason {
    /// The dialog title / identity is unstated.
    DialogTitleUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The named action model is the disallowed generic-yes-no token.
    GenericYesNoActionModel,
    /// The rationale is unstated.
    RationaleUnstated,
    /// The named scope is unstated.
    ScopeUnstated,
    /// The actions are not explicitly named.
    ExplicitActionsUnnamed,
    /// The initial focus target is unsafe or unresolved.
    SafeInitialFocusMissing,
    /// The cancel / escape path is missing.
    CancelPathMissing,
    /// Focus does not return to the invoker when the dialog is reopened.
    FocusReturnBrokenOnReopen,
    /// The reopen origin cannot currently be resolved.
    ReopenOriginUnresolved,
    /// The help / docs hook is missing.
    HelpDocsHookMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DialogDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::DialogTitleUnstated,
        Self::SurfaceContextUnresolved,
        Self::GenericYesNoActionModel,
        Self::RationaleUnstated,
        Self::ScopeUnstated,
        Self::ExplicitActionsUnnamed,
        Self::SafeInitialFocusMissing,
        Self::CancelPathMissing,
        Self::FocusReturnBrokenOnReopen,
        Self::ReopenOriginUnresolved,
        Self::HelpDocsHookMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DialogTitleUnstated => "dialog_title_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::GenericYesNoActionModel => "generic_yes_no_action_model",
            Self::RationaleUnstated => "rationale_unstated",
            Self::ScopeUnstated => "scope_unstated",
            Self::ExplicitActionsUnnamed => "explicit_actions_unnamed",
            Self::SafeInitialFocusMissing => "safe_initial_focus_missing",
            Self::CancelPathMissing => "cancel_path_missing",
            Self::FocusReturnBrokenOnReopen => "focus_return_broken_on_reopen",
            Self::ReopenOriginUnresolved => "reopen_origin_unresolved",
            Self::HelpDocsHookMissing => "help_docs_hook_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DialogConsequenceNextAction {
        match self {
            Self::DialogTitleUnstated
            | Self::SurfaceContextUnresolved
            | Self::RationaleUnstated
            | Self::ScopeUnstated => M5DialogConsequenceNextAction::ReviewRationaleAndScope,
            Self::GenericYesNoActionModel | Self::ExplicitActionsUnnamed => {
                M5DialogConsequenceNextAction::ChooseExplicitAction
            }
            Self::SafeInitialFocusMissing
            | Self::CancelPathMissing
            | Self::FocusReturnBrokenOnReopen
            | Self::ReopenOriginUnresolved => M5DialogConsequenceNextAction::ReturnFocusToInvoker,
            Self::HelpDocsHookMissing | Self::ProofStale => {
                M5DialogConsequenceNextAction::OpenHelpOrDocs
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::GenericYesNoActionModel | Self::ExplicitActionsUnnamed => {
                M5DecisionFeedbackDowngradeTrigger::GenericYesNoUsedInHighRiskDialog
            }
            Self::DialogTitleUnstated | Self::RationaleUnstated => {
                M5DecisionFeedbackDowngradeTrigger::RationaleUnstated
            }
            Self::ScopeUnstated => M5DecisionFeedbackDowngradeTrigger::ScopeUnstated,
            Self::SafeInitialFocusMissing | Self::CancelPathMissing | Self::HelpDocsHookMissing => {
                M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated
            }
            Self::SurfaceContextUnresolved
            | Self::FocusReturnBrokenOnReopen
            | Self::ReopenOriginUnresolved => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a consequence block degraded below a clean, blast-radius-named, rollback-honest state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConsequenceDegradeReason {
    /// The consequence label / affected-object summary is unstated.
    ConsequenceLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The named disclosure is the disallowed generic-yes-no token.
    DisclosureModelDisallowed,
    /// The affected object is unnamed.
    AffectedObjectUnnamed,
    /// The blast radius cannot currently be resolved or is unnamed.
    BlastRadiusUnresolved,
    /// The reversibility posture cannot currently be resolved.
    ReversibilityUnresolved,
    /// The rollback / help posture is unstated.
    RollbackPostureUnstated,
    /// A partial-success or irreversible note is required but missing.
    PartialOrIrreversibleNoteMissing,
    /// The block reduces to generic Yes/No ambiguity.
    GenericYesNoAmbiguity,
    /// The explanation is reachable only via a screenshot (not keyboard / screen reader / export).
    ExplanationReachableOnlyViaScreenshot,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ConsequenceDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsequenceLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::DisclosureModelDisallowed,
        Self::AffectedObjectUnnamed,
        Self::BlastRadiusUnresolved,
        Self::ReversibilityUnresolved,
        Self::RollbackPostureUnstated,
        Self::PartialOrIrreversibleNoteMissing,
        Self::GenericYesNoAmbiguity,
        Self::ExplanationReachableOnlyViaScreenshot,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsequenceLabelUnstated => "consequence_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DisclosureModelDisallowed => "disclosure_model_disallowed",
            Self::AffectedObjectUnnamed => "affected_object_unnamed",
            Self::BlastRadiusUnresolved => "blast_radius_unresolved",
            Self::ReversibilityUnresolved => "reversibility_unresolved",
            Self::RollbackPostureUnstated => "rollback_posture_unstated",
            Self::PartialOrIrreversibleNoteMissing => "partial_or_irreversible_note_missing",
            Self::GenericYesNoAmbiguity => "generic_yes_no_ambiguity",
            Self::ExplanationReachableOnlyViaScreenshot => {
                "explanation_reachable_only_via_screenshot"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DialogConsequenceNextAction {
        match self {
            Self::ConsequenceLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::AffectedObjectUnnamed
            | Self::BlastRadiusUnresolved => M5DialogConsequenceNextAction::ReviewConsequenceBlock,
            Self::DisclosureModelDisallowed | Self::GenericYesNoAmbiguity => {
                M5DialogConsequenceNextAction::ChooseExplicitAction
            }
            Self::ReversibilityUnresolved
            | Self::RollbackPostureUnstated
            | Self::PartialOrIrreversibleNoteMissing
            | Self::ExplanationReachableOnlyViaScreenshot
            | Self::ProofStale => M5DialogConsequenceNextAction::OpenHelpOrDocs,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::DisclosureModelDisallowed | Self::GenericYesNoAmbiguity => {
                M5DecisionFeedbackDowngradeTrigger::GenericYesNoUsedInHighRiskDialog
            }
            Self::ConsequenceLabelUnstated
            | Self::AffectedObjectUnnamed
            | Self::BlastRadiusUnresolved => M5DecisionFeedbackDowngradeTrigger::ScopeUnstated,
            Self::ReversibilityUnresolved
            | Self::RollbackPostureUnstated
            | Self::PartialOrIrreversibleNoteMissing => {
                M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated
            }
            Self::SurfaceContextUnresolved | Self::ExplanationReachableOnlyViaScreenshot => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_dialog`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DialogResolutionInput {
    /// Stable identity of the dialog instance.
    pub dialog_id: String,
    /// The dialog title / heading shown; empty means unstated.
    pub dialog_title: String,
    /// The dialog action model (from the frozen matrix vocabulary).
    pub action_model: M5DialogActionModel,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5DecisionActionSurfaceContext,
    /// The safe initial focus target.
    pub focus_target: M5DialogFocusTarget,
    /// The reopen origin (fresh or reopened from status / activity / support / deep link).
    pub reopen_origin: M5DialogReopenOrigin,
    /// True when the rationale for the decision is present.
    pub rationale_present: bool,
    /// True when the affected scope is named.
    pub scope_named: bool,
    /// True when each action is explicitly named (never generic Yes/No).
    pub actions_explicitly_named: bool,
    /// True when the initial focus does not auto-fire a destructive action.
    pub initial_focus_is_safe: bool,
    /// True when a cancel / escape path is present.
    pub cancel_path_present: bool,
    /// True when focus returns to the invoker when the dialog is reopened.
    pub focus_returns_on_reopen: bool,
    /// True when a help / docs hook is present.
    pub help_or_docs_hook_present: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe dialog projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDialog {
    /// Stable identity of the dialog instance.
    pub dialog_id: String,
    /// The dialog title named by the dialog.
    pub dialog_title: String,
    /// The dialog-action-model token named by the dialog.
    pub action_model: String,
    /// Whether the action model names the disallowed generic-yes-no token.
    pub action_model_is_generic_yes_no: bool,
    /// The state-disposition token named by the dialog.
    pub disposition: String,
    /// The render / surface-context token named by the dialog.
    pub surface_context: String,
    /// The focus-target token named by the dialog.
    pub focus_target: String,
    /// The reopen-origin token named by the dialog.
    pub reopen_origin: String,
    /// Whether the reopen origin is a reopen (not a fresh invocation).
    pub is_reopen: bool,
    /// Whether the rationale is present.
    pub rationale_present: bool,
    /// Whether the affected scope is named.
    pub scope_named: bool,
    /// Whether each action is explicitly named.
    pub actions_explicitly_named: bool,
    /// Whether the initial focus is safe.
    pub initial_focus_is_safe: bool,
    /// Whether a cancel / escape path is present.
    pub cancel_path_present: bool,
    /// Whether focus returns to the invoker when the dialog is reopened.
    pub focus_returns_on_reopen: bool,
    /// Whether a help / docs hook is present.
    pub help_or_docs_hook_present: bool,
    /// Degrade reason, if the dialog could not read as a clean, trustworthy confirmation.
    pub degrade_reason: Option<M5DialogDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DialogConsequenceNextAction,
    /// Whether the dialog names rationale, scope, and explicit actions with safe focus (clean dialog
    /// naming every fact).
    pub names_rationale_scope_and_explicit_actions: bool,
}

impl M5ResolvedDialog {
    /// Whether this dialog reads as a clean, trustworthy confirmation.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_consequence`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ConsequenceResolutionInput {
    /// Stable identity of the consequence-block instance.
    pub consequence_id: String,
    /// The affected-object / scope summary shown; empty means unstated.
    pub consequence_label: String,
    /// The consequence disclosure (from the frozen matrix vocabulary).
    pub disclosure: M5ConsequenceDisclosure,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5DecisionActionSurfaceContext,
    /// The named blast radius.
    pub blast_radius: M5ConsequenceBlastRadius,
    /// The reversibility posture.
    pub reversibility: M5ConsequenceReversibility,
    /// True when the affected object is named.
    pub affected_object_named: bool,
    /// True when the blast radius is named.
    pub blast_radius_named: bool,
    /// True when the rollback / help posture is stated.
    pub rollback_or_help_posture_stated: bool,
    /// True when a partial-success or irreversible note is present.
    pub partial_or_irreversible_noted: bool,
    /// True when the block avoids generic Yes/No ambiguity.
    pub avoids_generic_yes_no: bool,
    /// True when the explanation is reachable by keyboard, screen reader, and export (never
    /// screenshot-only).
    pub explanation_reachable_by_keyboard_sr_export: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe consequence-block projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedConsequence {
    /// Stable identity of the consequence-block instance.
    pub consequence_id: String,
    /// The affected-object / scope summary named by the block.
    pub consequence_label: String,
    /// The consequence-disclosure token named by the block.
    pub disclosure: String,
    /// Whether the disclosure names the disallowed generic-yes-no token.
    pub disclosure_is_disallowed: bool,
    /// The state-disposition token named by the block.
    pub disposition: String,
    /// The render / surface-context token named by the block.
    pub surface_context: String,
    /// The blast-radius token named by the block.
    pub blast_radius: String,
    /// Whether the blast radius reaches beyond a single object.
    pub blast_radius_is_broad: bool,
    /// The reversibility-posture token named by the block.
    pub reversibility: String,
    /// Whether the reversibility posture requires a partial-success or irreversible note.
    pub needs_partial_or_irreversible_note: bool,
    /// Whether the affected object is named.
    pub affected_object_named: bool,
    /// Whether the blast radius is named.
    pub blast_radius_named: bool,
    /// Whether the rollback / help posture is stated.
    pub rollback_or_help_posture_stated: bool,
    /// Whether a partial-success or irreversible note is present.
    pub partial_or_irreversible_noted: bool,
    /// Whether the block avoids generic Yes/No ambiguity.
    pub avoids_generic_yes_no: bool,
    /// Whether the explanation is reachable by keyboard / screen reader / export.
    pub explanation_reachable_by_keyboard_sr_export: bool,
    /// Degrade reason, if the block could not read as a clean, blast-radius-named, rollback-honest
    /// state.
    pub degrade_reason: Option<M5ConsequenceDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DialogConsequenceNextAction,
    /// Whether the block names blast radius and rollback / help posture without generic ambiguity
    /// (clean block naming every fact).
    pub names_blast_radius_and_rollback_posture: bool,
}

impl M5ResolvedConsequence {
    /// Whether this consequence block reads as a clean, blast-radius-named, rollback-honest state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5DialogConsequenceResolutionError {
    /// The dialog id was empty.
    EmptyDialogId,
    /// The consequence id was empty.
    EmptyConsequenceId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5DialogConsequenceResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDialogId => "empty_dialog_id",
            Self::EmptyConsequenceId => "empty_consequence_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DialogConsequenceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 dialog / consequence resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DialogConsequenceResolutionError {}

/// Resolves a dialog so it stays an explicit, trustworthy confirmation: the dialog names its title,
/// action model (never generic Yes/No), state disposition, and surface context, states its rationale
/// and named scope, names each action explicitly, keeps a safe initial focus and a cancel path, returns
/// focus to the invoker when reopened, and offers a help / docs hook.
pub fn resolve_dialog(
    input: M5DialogResolutionInput,
) -> Result<M5ResolvedDialog, M5DialogConsequenceResolutionError> {
    if input.dialog_id.trim().is_empty() {
        return Err(M5DialogConsequenceResolutionError::EmptyDialogId);
    }
    if string_is_forbidden(&input.dialog_id) || string_is_forbidden(&input.dialog_title) {
        return Err(M5DialogConsequenceResolutionError::ForbiddenMaterial);
    }

    let action_model_is_generic_yes_no = matches!(
        input.action_model,
        M5DialogActionModel::GenericYesNoDisallowed
    );

    let degrade_reason = if input.dialog_title.trim().is_empty() {
        Some(M5DialogDegradeReason::DialogTitleUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5DialogDegradeReason::SurfaceContextUnresolved)
    } else if action_model_is_generic_yes_no {
        Some(M5DialogDegradeReason::GenericYesNoActionModel)
    } else if !input.rationale_present {
        Some(M5DialogDegradeReason::RationaleUnstated)
    } else if !input.scope_named {
        Some(M5DialogDegradeReason::ScopeUnstated)
    } else if !input.actions_explicitly_named {
        Some(M5DialogDegradeReason::ExplicitActionsUnnamed)
    } else if !input.focus_target.is_resolved() || !input.initial_focus_is_safe {
        Some(M5DialogDegradeReason::SafeInitialFocusMissing)
    } else if !input.cancel_path_present {
        Some(M5DialogDegradeReason::CancelPathMissing)
    } else if !input.focus_returns_on_reopen {
        Some(M5DialogDegradeReason::FocusReturnBrokenOnReopen)
    } else if !input.reopen_origin.is_resolved() {
        Some(M5DialogDegradeReason::ReopenOriginUnresolved)
    } else if !input.help_or_docs_hook_present {
        Some(M5DialogDegradeReason::HelpDocsHookMissing)
    } else if !input.proof_fresh {
        Some(M5DialogDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DialogConsequenceNextAction::ReviewRationaleAndScope,
    };

    Ok(M5ResolvedDialog {
        dialog_id: input.dialog_id,
        dialog_title: input.dialog_title,
        action_model: input.action_model.as_str().to_owned(),
        action_model_is_generic_yes_no,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        focus_target: input.focus_target.as_str().to_owned(),
        reopen_origin: input.reopen_origin.as_str().to_owned(),
        is_reopen: input.reopen_origin.is_reopen(),
        rationale_present: input.rationale_present,
        scope_named: input.scope_named,
        actions_explicitly_named: input.actions_explicitly_named,
        initial_focus_is_safe: input.initial_focus_is_safe,
        cancel_path_present: input.cancel_path_present,
        focus_returns_on_reopen: input.focus_returns_on_reopen,
        help_or_docs_hook_present: input.help_or_docs_hook_present,
        degrade_reason,
        next_action,
        names_rationale_scope_and_explicit_actions: degrade_reason.is_none(),
    })
}

/// Resolves a consequence block so it names its blast radius and rollback / help posture: the block
/// names its affected object, disclosure (never generic Yes/No), state disposition, surface context,
/// blast radius, and reversibility, states its rollback / help posture, carries a partial-success or
/// irreversible note where relevant, avoids generic Yes/No ambiguity, and keeps its explanation
/// reachable by keyboard, screen reader, and export.
pub fn resolve_consequence(
    input: M5ConsequenceResolutionInput,
) -> Result<M5ResolvedConsequence, M5DialogConsequenceResolutionError> {
    if input.consequence_id.trim().is_empty() {
        return Err(M5DialogConsequenceResolutionError::EmptyConsequenceId);
    }
    if string_is_forbidden(&input.consequence_id) || string_is_forbidden(&input.consequence_label) {
        return Err(M5DialogConsequenceResolutionError::ForbiddenMaterial);
    }

    let disclosure_is_disallowed = matches!(
        input.disclosure,
        M5ConsequenceDisclosure::GenericYesNoDisallowed
    );

    let degrade_reason = if input.consequence_label.trim().is_empty() {
        Some(M5ConsequenceDegradeReason::ConsequenceLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ConsequenceDegradeReason::SurfaceContextUnresolved)
    } else if disclosure_is_disallowed {
        Some(M5ConsequenceDegradeReason::DisclosureModelDisallowed)
    } else if !input.affected_object_named {
        Some(M5ConsequenceDegradeReason::AffectedObjectUnnamed)
    } else if !input.blast_radius.is_resolved() || !input.blast_radius_named {
        Some(M5ConsequenceDegradeReason::BlastRadiusUnresolved)
    } else if !input.reversibility.is_resolved() {
        Some(M5ConsequenceDegradeReason::ReversibilityUnresolved)
    } else if !input.rollback_or_help_posture_stated {
        Some(M5ConsequenceDegradeReason::RollbackPostureUnstated)
    } else if input.reversibility.needs_partial_or_irreversible_note()
        && !input.partial_or_irreversible_noted
    {
        Some(M5ConsequenceDegradeReason::PartialOrIrreversibleNoteMissing)
    } else if !input.avoids_generic_yes_no {
        Some(M5ConsequenceDegradeReason::GenericYesNoAmbiguity)
    } else if !input.explanation_reachable_by_keyboard_sr_export {
        Some(M5ConsequenceDegradeReason::ExplanationReachableOnlyViaScreenshot)
    } else if !input.proof_fresh {
        Some(M5ConsequenceDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DialogConsequenceNextAction::ReviewConsequenceBlock,
    };

    Ok(M5ResolvedConsequence {
        consequence_id: input.consequence_id,
        consequence_label: input.consequence_label,
        disclosure: input.disclosure.as_str().to_owned(),
        disclosure_is_disallowed,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        blast_radius: input.blast_radius.as_str().to_owned(),
        blast_radius_is_broad: input.blast_radius.is_broad(),
        reversibility: input.reversibility.as_str().to_owned(),
        needs_partial_or_irreversible_note: input
            .reversibility
            .needs_partial_or_irreversible_note(),
        affected_object_named: input.affected_object_named,
        blast_radius_named: input.blast_radius_named,
        rollback_or_help_posture_stated: input.rollback_or_help_posture_stated,
        partial_or_irreversible_noted: input.partial_or_irreversible_noted,
        avoids_generic_yes_no: input.avoids_generic_yes_no,
        explanation_reachable_by_keyboard_sr_export: input
            .explanation_reachable_by_keyboard_sr_export,
        degrade_reason,
        next_action,
        names_blast_radius_and_rollback_posture: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved dialog and consequence examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DialogConsequenceControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5DialogConsequenceConsumerSurface,
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
    pub anatomy_parts: Vec<M5DialogConsequenceAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5DialogConsequenceExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    /// Resolved dialog examples.
    pub dialog_examples: Vec<M5ResolvedDialog>,
    /// Resolved consequence examples.
    pub consequence_examples: Vec<M5ResolvedConsequence>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a dialog never uses generic Yes/No copy in a high-risk confirmation. MUST be
    /// `false`.
    pub dialog_uses_generic_yes_no_in_high_risk: bool,
    /// Hard invariant: a dialog never fails to return focus when reopened. MUST be `false`.
    pub dialog_focus_fails_to_return_on_reopen: bool,
    /// Hard invariant: a consequence block never omits its named blast radius. MUST be `false`.
    pub consequence_omits_named_blast_radius: bool,
    /// Hard invariant: a consequence block never reduces to generic Yes/No ambiguity. MUST be `false`.
    pub consequence_reduces_to_generic_yes_no: bool,
}

impl M5DialogConsequenceControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DialogConsequenceAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DialogConsequenceAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DialogConsequenceExportField> =
            self.export_fields.iter().copied().collect();
        M5DialogConsequenceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.dialog_uses_generic_yes_no_in_high_risk
            && !self.dialog_focus_fails_to_return_on_reopen
            && !self.consequence_omits_named_blast_radius
            && !self.consequence_reduces_to_generic_yes_no
    }

    /// True when a clean dialog preserves confirmation truth: it is never generic Yes/No, states its
    /// rationale and named scope, names each action explicitly, keeps a safe initial focus and a cancel
    /// path, returns focus on reopen, and offers a help / docs hook.
    fn dialog_is_honest(ex: &M5ResolvedDialog) -> bool {
        !ex.is_clean()
            || (!ex.action_model_is_generic_yes_no
                && ex.rationale_present
                && ex.scope_named
                && ex.actions_explicitly_named
                && ex.initial_focus_is_safe
                && ex.cancel_path_present
                && ex.focus_returns_on_reopen
                && ex.help_or_docs_hook_present)
    }

    /// True when a clean consequence block preserves blast-radius and rollback truth: it never names the
    /// disallowed disclosure, names its affected object and blast radius, states its rollback / help
    /// posture, avoids generic Yes/No, and keeps its explanation reachable off-screenshot.
    fn consequence_is_honest(ex: &M5ResolvedConsequence) -> bool {
        !ex.is_clean()
            || (!ex.disclosure_is_disallowed
                && ex.affected_object_named
                && ex.blast_radius_named
                && ex.rollback_or_help_posture_stated
                && ex.avoids_generic_yes_no
                && ex.explanation_reachable_by_keyboard_sr_export
                && ex.partial_or_irreversible_noted)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.dialog_examples.iter().all(Self::dialog_is_honest)
            && self
                .consequence_examples
                .iter()
                .all(Self::consequence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DialogConsequenceVocabularySet {
    /// State-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Dialog-action-model tokens (bound from the frozen matrix).
    pub dialog_action_models: Vec<String>,
    /// Consequence-disclosure tokens (bound from the frozen matrix).
    pub consequence_disclosures: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Focus-target tokens (minted by this lane).
    pub focus_targets: Vec<String>,
    /// Reopen-origin tokens (minted by this lane).
    pub reopen_origins: Vec<String>,
    /// Blast-radius tokens (minted by this lane).
    pub blast_radii: Vec<String>,
    /// Reversibility tokens (minted by this lane).
    pub reversibilities: Vec<String>,
    /// Dialog degrade-reason tokens.
    pub dialog_degrade_reasons: Vec<String>,
    /// Consequence degrade-reason tokens.
    pub consequence_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5DialogConsequenceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5DecisionFeedbackDisposition::ALL, |v| v.as_str()),
            dialog_action_models: tokens(&M5DialogActionModel::ALL, |v| v.as_str()),
            consequence_disclosures: tokens(&M5ConsequenceDisclosure::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5DecisionActionSurfaceContext::ALL, |v| v.as_str()),
            focus_targets: tokens(&M5DialogFocusTarget::ALL, |v| v.as_str()),
            reopen_origins: tokens(&M5DialogReopenOrigin::ALL, |v| v.as_str()),
            blast_radii: tokens(&M5ConsequenceBlastRadius::ALL, |v| v.as_str()),
            reversibilities: tokens(&M5ConsequenceReversibility::ALL, |v| v.as_str()),
            dialog_degrade_reasons: tokens(&M5DialogDegradeReason::ALL, |v| v.as_str()),
            consequence_degrade_reasons: tokens(&M5ConsequenceDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5DialogConsequenceAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DialogConsequenceNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DialogConsequenceExportField::ALL, |v| v.as_str()),
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
pub struct M5DialogConsequenceGovernanceReview {
    /// The dialog names its title, rationale, and named scope.
    pub dialog_names_title_rationale_and_scope: bool,
    /// The dialog names explicit actions and never uses generic Yes/No copy.
    pub dialog_names_explicit_actions_no_generic_yes_no: bool,
    /// The dialog has a safe initial focus and a cancel path.
    pub dialog_has_safe_initial_focus_and_cancel_path: bool,
    /// The dialog returns focus to the invoker when reopened from notification / status.
    pub dialog_returns_focus_on_reopen_from_notification: bool,
    /// The dialog offers a help / docs hook.
    pub dialog_offers_help_or_docs_hook: bool,
    /// The consequence block names its affected object and blast radius.
    pub consequence_names_affected_object_and_blast_radius: bool,
    /// The consequence block states its rollback / help posture.
    pub consequence_states_rollback_or_help_posture: bool,
    /// The consequence block notes partial-success or irreversibility where relevant.
    pub consequence_notes_partial_success_or_irreversibility: bool,
    /// The consequence block never reduces to generic Yes/No ambiguity.
    pub consequence_never_reduces_to_generic_yes_no: bool,
    /// The consequence block is explainable without screenshots alone.
    pub consequence_explainable_without_screenshots: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DialogConsequenceConsumerProjection {
    /// Review surfaces consume the shared dialog and consequence vocabulary.
    pub review_surfaces_consume_dialog_and_consequence_vocabulary: bool,
    /// Settings surfaces consume the shared dialog vocabulary.
    pub settings_surfaces_consume_dialog_vocabulary: bool,
    /// Update / install surfaces consume the shared dialog and consequence vocabulary.
    pub updates_surfaces_consume_dialog_and_consequence_vocabulary: bool,
    /// Repair surfaces consume the shared consequence vocabulary.
    pub repair_surfaces_consume_consequence_vocabulary: bool,
    /// Dialog and consequence facts trace back to one canonical component contract.
    pub dialog_and_consequence_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical dialog / consequence source.
    pub support_export_reads_single_dialog_consequence_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DialogConsequenceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DialogConsequenceReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DialogConsequenceControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DialogConsequenceControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DialogConsequenceControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DialogConsequenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DialogConsequenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DialogConsequenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DialogConsequenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DialogConsequenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 dialog / consequence controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DialogConsequenceControlsPacket {
    /// Record kind; must equal [`M5_DIALOG_CONSEQUENCE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DialogConsequenceControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DialogConsequenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DialogConsequenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DialogConsequenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DialogConsequenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DialogConsequenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DialogConsequenceControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5DialogConsequenceControlsPacketInput) -> Self {
        Self {
            record_kind: M5_DIALOG_CONSEQUENCE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5DialogConsequenceControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DIALOG_CONSEQUENCE_CONTROLS_RECORD_KIND {
            violations.push(M5DialogConsequenceControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5DialogConsequenceControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DialogConsequenceControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5DialogConsequenceControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 dialog / consequence controls packet serializes"),
        ) {
            violations.push(M5DialogConsequenceControlsViolation::RawMaterialInExport);
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
            .expect("m5 dialog / consequence controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,dialog_examples,consequence_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .dialog_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.consequence_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.dialog_examples.len(),
                row.consequence_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Dialog / Sheet and Consequence-Block Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Dialog action models: {}\n",
            self.vocabulary_set.dialog_action_models.join(", ")
        ));
        out.push_str(&format!(
            "- Blast radii: {}\n",
            self.vocabulary_set.blast_radii.join(", ")
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
                "  - Dialog examples: {} / consequence examples: {}\n",
                row.dialog_examples.len(),
                row.consequence_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5DialogConsequenceControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DialogConsequenceControlsViolation>),
}

impl fmt::Display for M5DialogConsequenceControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 dialog / consequence controls export parse failed: {error}"
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
                    "m5 dialog / consequence controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DialogConsequenceControlsArtifactError {}

/// Validation failures emitted by [`M5DialogConsequenceControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DialogConsequenceControlsViolation {
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
    /// A controls row carries a dishonest clean example (generic-yes-no dialog, rationale-less dialog,
    /// unsafe focus, broken focus return, unnamed blast radius, or a screenshot-only consequence).
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
    /// Dialog rationale, scope, and explicit actions are not proven: clean dialogs do not cover the
    /// action-model grammar, or no generic-yes-no / rationale-missing / scope-missing example degrades,
    /// or a clean dialog is generic-yes-no or missing rationale / scope.
    DialogRationaleScopeActionsNotProven,
    /// Focus and cancel stability is not proven: no clean dialog keeps a safe initial focus, cancel
    /// path, and focus return on reopen, or no safe-focus-missing / cancel-missing / focus-return-broken
    /// example degrades, or a clean dialog loses safe focus / cancel / focus return.
    FocusAndCancelStabilityNotProven,
    /// The consequence block is not proven explainable without screenshots: no clean consequence names
    /// its blast radius and rollback posture reachable off-screenshot, or no blast-radius-unresolved /
    /// screenshot-only example degrades, or no clean dialog and clean consequence keep a canonical trace.
    ConsequenceExplainableWithoutScreenshotsNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DialogConsequenceControlsViolation {
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
            Self::DialogRationaleScopeActionsNotProven => {
                "dialog_rationale_scope_actions_not_proven"
            }
            Self::FocusAndCancelStabilityNotProven => "focus_and_cancel_stability_not_proven",
            Self::ConsequenceExplainableWithoutScreenshotsNotProven => {
                "consequence_explainable_without_screenshots_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_dialog_consequence_controls_export(
) -> Result<M5DialogConsequenceControlsPacket, M5DialogConsequenceControlsArtifactError> {
    let packet: M5DialogConsequenceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-dialog-sheet-and-consequence-block-controls-proof/support_export.json"
    )))
    .map_err(M5DialogConsequenceControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DialogConsequenceControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DIALOG_CONSEQUENCE_CONTROLS_SCHEMA_REF,
        M5_DIALOG_CONSEQUENCE_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_DIALOG_SHEET_SCHEMA_REF,
        M5_CONSEQUENCE_BLOCK_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DialogConsequenceControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5DialogConsequenceControlsViolation::NoControlsRows);
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
            violations.push(M5DialogConsequenceControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DialogConsequenceControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DialogConsequenceControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DIALOG_SHEET_SCHEMA_REF)
            || !refs.contains(M5_CONSEQUENCE_BLOCK_SCHEMA_REF)
        {
            violations.push(M5DialogConsequenceControlsViolation::ComponentSchemaRefMissing);
        }
        if row.dialog_examples.is_empty() || row.consequence_examples.is_empty() {
            violations.push(M5DialogConsequenceControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5DialogConsequenceControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5DialogConsequenceControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.dialog_names_title_rationale_and_scope,
        review.dialog_names_explicit_actions_no_generic_yes_no,
        review.dialog_has_safe_initial_focus_and_cancel_path,
        review.dialog_returns_focus_on_reopen_from_notification,
        review.dialog_offers_help_or_docs_hook,
        review.consequence_names_affected_object_and_blast_radius,
        review.consequence_states_rollback_or_help_posture,
        review.consequence_notes_partial_success_or_irreversibility,
        review.consequence_never_reduces_to_generic_yes_no,
        review.consequence_explainable_without_screenshots,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5DialogConsequenceControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_surfaces_consume_dialog_and_consequence_vocabulary,
        projection.settings_surfaces_consume_dialog_vocabulary,
        projection.updates_surfaces_consume_dialog_and_consequence_vocabulary,
        projection.repair_surfaces_consume_consequence_vocabulary,
        projection.dialog_and_consequence_trace_to_single_component_contract,
        projection.support_export_reads_single_dialog_consequence_source,
    ] {
        if !ok {
            violations.push(M5DialogConsequenceControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DialogConsequenceControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DialogConsequenceControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5DialogConsequenceControlsPacket,
    violations: &mut Vec<M5DialogConsequenceControlsViolation>,
) {
    let dialogs = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.dialog_examples.iter())
    };
    let consequences = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.consequence_examples.iter())
    };

    // AC1: the first claimed M5 dialog / sheet consumers expose rationale, scope, and explicit actions
    // consistently instead of feature-local prose. Clean dialogs cover at least the
    // named-specific-actions / primary-and-cancel / destructive-confirm-named action-model grammar and
    // always state rationale and scope, a generic-yes-no example degrades, a rationale-missing example
    // degrades, a scope-missing example degrades, and no clean dialog is generic-yes-no or missing
    // rationale / scope.
    let clean_action_models: BTreeSet<String> = dialogs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.action_model.clone())
        .collect();
    let action_grammar_covered = [
        "named_specific_actions",
        "primary_and_cancel",
        "destructive_confirm_named",
    ]
    .iter()
    .all(|m| clean_action_models.contains(*m));
    let generic_yes_no_degrades = dialogs()
        .any(|ex| ex.degrade_reason == Some(M5DialogDegradeReason::GenericYesNoActionModel));
    let rationale_missing_degrades =
        dialogs().any(|ex| ex.degrade_reason == Some(M5DialogDegradeReason::RationaleUnstated));
    let scope_missing_degrades =
        dialogs().any(|ex| ex.degrade_reason == Some(M5DialogDegradeReason::ScopeUnstated));
    let no_clean_generic_or_rationaleless_dialog = !dialogs().any(|ex| {
        ex.is_clean()
            && (ex.action_model_is_generic_yes_no || !ex.rationale_present || !ex.scope_named)
    });
    if !(action_grammar_covered
        && generic_yes_no_degrades
        && rationale_missing_degrades
        && scope_missing_degrades
        && no_clean_generic_or_rationaleless_dialog)
    {
        violations.push(M5DialogConsequenceControlsViolation::DialogRationaleScopeActionsNotProven);
    }

    // AC2: default focus, escape / cancel behavior, and focus return remain stable in keyboard-only,
    // screen-reader, and high-zoom scenarios. At least one clean dialog keeps a safe initial focus, a
    // cancel path, and focus return on reopen, a safe-focus-missing example degrades, a cancel-missing
    // example degrades, a focus-return-broken example degrades, and no clean dialog loses safe focus,
    // cancel, or focus return.
    let clean_stable_dialog = dialogs().any(|ex| {
        ex.is_clean()
            && ex.initial_focus_is_safe
            && ex.cancel_path_present
            && ex.focus_returns_on_reopen
    });
    let safe_focus_missing_degrades = dialogs()
        .any(|ex| ex.degrade_reason == Some(M5DialogDegradeReason::SafeInitialFocusMissing));
    let cancel_missing_degrades =
        dialogs().any(|ex| ex.degrade_reason == Some(M5DialogDegradeReason::CancelPathMissing));
    let focus_return_broken_degrades = dialogs()
        .any(|ex| ex.degrade_reason == Some(M5DialogDegradeReason::FocusReturnBrokenOnReopen));
    let no_clean_unstable_dialog = !dialogs().any(|ex| {
        ex.is_clean()
            && (!ex.initial_focus_is_safe || !ex.cancel_path_present || !ex.focus_returns_on_reopen)
    });
    if !(clean_stable_dialog
        && safe_focus_missing_degrades
        && cancel_missing_degrades
        && focus_return_broken_degrades
        && no_clean_unstable_dialog)
    {
        violations.push(M5DialogConsequenceControlsViolation::FocusAndCancelStabilityNotProven);
    }

    // AC3: support / help / export packets can explain the decision surface and its consequence block
    // without screenshots alone. At least one clean consequence names its blast radius and rollback
    // posture reachable off-screenshot, a blast-radius-unresolved example degrades, a screenshot-only
    // example degrades, and at least one clean dialog and one clean consequence both keep a reachable
    // canonical trace.
    let clean_explainable_consequence = consequences().any(|ex| {
        ex.is_clean()
            && ex.blast_radius_named
            && ex.rollback_or_help_posture_stated
            && ex.explanation_reachable_by_keyboard_sr_export
    });
    let blast_radius_unresolved_degrades = consequences()
        .any(|ex| ex.degrade_reason == Some(M5ConsequenceDegradeReason::BlastRadiusUnresolved));
    let screenshot_only_degrades = consequences().any(|ex| {
        ex.degrade_reason == Some(M5ConsequenceDegradeReason::ExplanationReachableOnlyViaScreenshot)
    });
    let traceable_dialog = dialogs().any(|ex| ex.is_clean() && ex.help_or_docs_hook_present);
    let traceable_consequence =
        consequences().any(|ex| ex.is_clean() && ex.explanation_reachable_by_keyboard_sr_export);
    if !(clean_explainable_consequence
        && blast_radius_unresolved_degrades
        && screenshot_only_degrades
        && traceable_dialog
        && traceable_consequence)
    {
        violations.push(
            M5DialogConsequenceControlsViolation::ConsequenceExplainableWithoutScreenshotsNotProven,
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
pub const IMPLEMENTED_FAMILIES: [M5DecisionFeedbackFamily; 2] = [
    M5DecisionFeedbackFamily::DialogSheet,
    M5DecisionFeedbackFamily::ConsequenceBlock,
];
