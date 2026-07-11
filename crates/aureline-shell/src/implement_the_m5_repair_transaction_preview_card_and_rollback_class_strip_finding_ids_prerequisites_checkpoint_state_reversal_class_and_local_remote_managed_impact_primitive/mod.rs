//! Implemented M5 repair-transaction-preview-card and rollback-class-strip primitives.
//!
//! The frozen [workspace-trust / guided-repair component matrix][matrix] names the reusable trust
//! and repair UI components and locks their controlled vocabulary. This module is the guided-repair
//! implement lane over that matrix: it turns the **repair-transaction preview card** and the
//! **rollback-class strip** into resolvers that produce export-safe, honest projections, so a repair
//! preview reads as a typed transaction review rather than a folklore "Fix it" shortcut.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render repair-transaction preview cards with stable repair IDs, linked finding IDs,
//!   preconditions, checkpoint availability, impact scope, and a local / remote / managed target
//!   class.** [`resolve_repair_transaction_preview_card`] refuses to read as a clean, reviewable card
//!   unless it names its repair id, at least one linked finding id, its prerequisites, its impact
//!   scope, a resolved local / remote / managed target class, and a resolved checkpoint state whose
//!   absence is disclosed before apply; it degrades instead.
//! * **Expose rollback-class strips using the controlled exact / compensate / regenerate / manual /
//!   audit-only vocabulary.** [`resolve_rollback_class_strip`] binds each strip to the frozen
//!   [`M5RepairReversalClass`] vocabulary and never collapses distinct reversal classes into a
//!   generic "undo".
//! * **Prevent repair UI from implying reversibility when only compensation or manual follow-up is
//!   available.** A rollback strip degrades to
//!   [`M5RollbackClassStripDegradeReason::ReversibilityOverclaimed`] the moment it reads as reversible
//!   without an exact or regenerate reversal class, and to
//!   [`M5RollbackClassStripDegradeReason::ReversalLimitHidden`] when a non-exact reversal leaves its
//!   limit undisclosed.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5WorkspaceTrustRepairDisposition`] trust / repair-disposition vocabulary, the
//! [`M5RepairReversalClass`] reversal vocabulary, the [`M5RepairCheckpointState`] checkpoint
//! vocabulary, and the [`M5RepairPreviewState`] preview vocabulary — so every claimed M5 guided-repair
//! surface exposes the same transaction-preview grammar and one reversal vocabulary instead of
//! forking its own "Fix it" copy.
//!
//! [matrix]: crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_repair_preview_rollback_controls,
    seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed,
    seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed,
    M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix::{
    M5RepairCheckpointState, M5RepairPreviewState, M5RepairReversalClass,
    M5WorkspaceTrustRepairAccessibilityRoute, M5WorkspaceTrustRepairComponentFamily,
    M5WorkspaceTrustRepairConsumerSurface, M5WorkspaceTrustRepairDeploymentLine,
    M5WorkspaceTrustRepairDisposition, M5WorkspaceTrustRepairDowngradeTrigger,
    M5WorkspaceTrustRepairQualificationClass, M5WorkspaceTrustRepairRequiredLabel,
    M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF, M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF, M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5RepairPreviewRollbackControlsPacket`].
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_RECORD_KIND: &str =
    "implement_m5_repair_transaction_preview_card_and_rollback_class_strip_controls";

/// Schema version for M5 repair-transaction-preview-card / rollback-class-strip controls records.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_DOC_REF: &str =
    "docs/trust/m5_repair_transaction_preview_card_and_rollback_class_strip_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5RepairPreviewRollbackConsumerSurface = M5WorkspaceTrustRepairConsumerSurface;

/// The local / remote / managed impact target a repair transaction mutates, so a preview never hides
/// whether a repair touches the local workspace, a remote host, or a managed workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairTargetClass {
    /// The repair mutates the local workspace.
    LocalWorkspace,
    /// The repair mutates a remote host / SSH target.
    RemoteHost,
    /// The repair mutates a managed / policy-governed workspace.
    ManagedWorkspace,
    /// The repair spans more than one target class.
    MixedTarget,
    /// The repair mutates an external target outside product control.
    ExternalTarget,
    /// The impact target class cannot currently be resolved.
    TargetUnknown,
}

impl M5RepairTargetClass {
    /// Every target class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalWorkspace,
        Self::RemoteHost,
        Self::ManagedWorkspace,
        Self::MixedTarget,
        Self::ExternalTarget,
        Self::TargetUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RemoteHost => "remote_host",
            Self::ManagedWorkspace => "managed_workspace",
            Self::MixedTarget => "mixed_target",
            Self::ExternalTarget => "external_target",
            Self::TargetUnknown => "target_unknown",
        }
    }

    /// Whether this class is one of the three honest local / remote / managed target classes.
    pub const fn is_local_remote_or_managed(self) -> bool {
        matches!(
            self,
            Self::LocalWorkspace | Self::RemoteHost | Self::ManagedWorkspace
        )
    }
}

/// The next safe review action a preview card or rollback strip surfaces so a user is never left
/// without a route to review the transaction before applying it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairReviewAction {
    /// Review the whole repair transaction before applying it — the clean default.
    ReviewTransaction,
    /// Inspect the checkpoint / restore point before applying.
    InspectCheckpoint,
    /// Review the reversal class and its limits.
    ReviewReversalClass,
    /// Review the impact scope the repair will mutate.
    ReviewImpactScope,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5RepairReviewAction {
    /// Every review action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewTransaction,
        Self::InspectCheckpoint,
        Self::ReviewReversalClass,
        Self::ReviewImpactScope,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewTransaction => "review_transaction",
            Self::InspectCheckpoint => "inspect_checkpoint",
            Self::ReviewReversalClass => "review_reversal_class",
            Self::ReviewImpactScope => "review_impact_scope",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// One mandatory rendered part a repair-transaction preview card or rollback-class strip must be able
/// to show, so no repair-transaction fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairPreviewRollbackAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed repair disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The stable repair id behind the transaction (preview card).
    RepairId,
    /// The linked finding ids the repair addresses (preview card).
    LinkedFindingIds,
    /// The preconditions / prerequisites of the repair (preview card).
    Prerequisites,
    /// The checkpoint state before apply (both components).
    CheckpointState,
    /// The impact scope the repair will mutate (preview card).
    ImpactScope,
    /// The local / remote / managed target class (preview card).
    TargetClass,
    /// The controlled reversal class (rollback strip).
    ReversalClass,
    /// The reversal-limit disclosure that keeps compensation / manual from reading as reversible
    /// (rollback strip).
    ReversalLimitDisclosure,
    /// The command-backed path to review the transaction before applying it (both components).
    ReviewCommand,
}

impl M5RepairPreviewRollbackAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::RepairId,
        Self::LinkedFindingIds,
        Self::Prerequisites,
        Self::CheckpointState,
        Self::ImpactScope,
        Self::TargetClass,
        Self::ReversalClass,
        Self::ReversalLimitDisclosure,
        Self::ReviewCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::RepairId => "repair_id",
            Self::LinkedFindingIds => "linked_finding_ids",
            Self::Prerequisites => "prerequisites",
            Self::CheckpointState => "checkpoint_state",
            Self::ImpactScope => "impact_scope",
            Self::TargetClass => "target_class",
            Self::ReversalClass => "reversal_class",
            Self::ReversalLimitDisclosure => "reversal_limit_disclosure",
            Self::ReviewCommand => "review_command",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairPreviewRollbackExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The repair dispositions carried.
    RepairDispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The repair id named by the preview card.
    RepairId,
    /// The linked finding ids named by the preview card.
    LinkedFindingIds,
    /// The checkpoint state named by the components.
    CheckpointState,
    /// The reversal class named by the rollback strip.
    ReversalClass,
    /// The local / remote / managed target class named by the preview card.
    TargetClass,
    /// The impact scope named by the preview card.
    ImpactScope,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RepairPreviewRollbackExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::RepairDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::RepairId,
        Self::LinkedFindingIds,
        Self::CheckpointState,
        Self::ReversalClass,
        Self::TargetClass,
        Self::ImpactScope,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::RepairDispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::RepairDispositions => "repair_dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::RepairId => "repair_id",
            Self::LinkedFindingIds => "linked_finding_ids",
            Self::CheckpointState => "checkpoint_state",
            Self::ReversalClass => "reversal_class",
            Self::TargetClass => "target_class",
            Self::ImpactScope => "impact_scope",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a repair-transaction preview card degraded below a clean, reviewable state. The degrade-
/// first ladder returns one of these instead of ever letting a folklore "Fix it" card read as a
/// clean, ready-to-apply transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairTransactionPreviewCardDegradeReason {
    /// The stable repair id is unstated; a user cannot identify the transaction.
    RepairIdUnstated,
    /// No linked finding id is named; the repair is not tied to a finding.
    LinkedFindingsUnstated,
    /// The preconditions / prerequisites are not stated.
    PrerequisitesUnstated,
    /// The checkpoint state cannot currently be resolved.
    CheckpointStateUnresolved,
    /// A checkpoint is absent but its absence is not disclosed before apply.
    CheckpointAbsenceHidden,
    /// The impact scope is not stated.
    ImpactScopeUnstated,
    /// The local / remote / managed target class cannot currently be resolved.
    TargetClassUnresolved,
    /// The preview is not complete and ready, yet reads as ready to apply.
    PreviewNotReady,
    /// The local / remote / managed target class collapsed into a generic target.
    TargetCollapsedIntoGeneric,
    /// No command-backed review path is reachable; review would be docs- or logs-only.
    ReviewPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RepairTransactionPreviewCardDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::RepairIdUnstated,
        Self::LinkedFindingsUnstated,
        Self::PrerequisitesUnstated,
        Self::CheckpointStateUnresolved,
        Self::CheckpointAbsenceHidden,
        Self::ImpactScopeUnstated,
        Self::TargetClassUnresolved,
        Self::PreviewNotReady,
        Self::TargetCollapsedIntoGeneric,
        Self::ReviewPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairIdUnstated => "repair_id_unstated",
            Self::LinkedFindingsUnstated => "linked_findings_unstated",
            Self::PrerequisitesUnstated => "prerequisites_unstated",
            Self::CheckpointStateUnresolved => "checkpoint_state_unresolved",
            Self::CheckpointAbsenceHidden => "checkpoint_absence_hidden",
            Self::ImpactScopeUnstated => "impact_scope_unstated",
            Self::TargetClassUnresolved => "target_class_unresolved",
            Self::PreviewNotReady => "preview_not_ready",
            Self::TargetCollapsedIntoGeneric => "target_collapsed_into_generic",
            Self::ReviewPathMissing => "review_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe review action for this reason.
    pub const fn next_action(self) -> M5RepairReviewAction {
        match self {
            Self::CheckpointStateUnresolved | Self::CheckpointAbsenceHidden => {
                M5RepairReviewAction::InspectCheckpoint
            }
            Self::ImpactScopeUnstated => M5RepairReviewAction::ReviewImpactScope,
            Self::ProofStale => M5RepairReviewAction::ReviewDiagnostics,
            _ => M5RepairReviewAction::ReviewTransaction,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RepairIdUnstated | Self::LinkedFindingsUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::RepairTargetIdsUnstated
            }
            Self::CheckpointStateUnresolved | Self::CheckpointAbsenceHidden => {
                M5WorkspaceTrustRepairDowngradeTrigger::CheckpointAbsenceHidden
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            _ => M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason a rollback-class strip degraded below a clean, honest state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RollbackClassStripDegradeReason {
    /// The stable repair id is unstated; the strip cannot be tied to a transaction.
    RepairIdUnstated,
    /// The reversal class cannot currently be resolved.
    ReversalClassUnresolved,
    /// The checkpoint state cannot currently be resolved.
    CheckpointStateUnresolved,
    /// A checkpoint is absent but its absence is not disclosed before apply.
    CheckpointAbsenceHidden,
    /// The strip reads as reversible without an exact or regenerate reversal class.
    ReversibilityOverclaimed,
    /// A non-exact reversal leaves its limit undisclosed.
    ReversalLimitHidden,
    /// Distinct reversal classes collapsed into a generic "undo".
    CollapsedIntoGenericUndo,
    /// No command-backed review path is reachable.
    ReviewPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RollbackClassStripDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RepairIdUnstated,
        Self::ReversalClassUnresolved,
        Self::CheckpointStateUnresolved,
        Self::CheckpointAbsenceHidden,
        Self::ReversibilityOverclaimed,
        Self::ReversalLimitHidden,
        Self::CollapsedIntoGenericUndo,
        Self::ReviewPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairIdUnstated => "repair_id_unstated",
            Self::ReversalClassUnresolved => "reversal_class_unresolved",
            Self::CheckpointStateUnresolved => "checkpoint_state_unresolved",
            Self::CheckpointAbsenceHidden => "checkpoint_absence_hidden",
            Self::ReversibilityOverclaimed => "reversibility_overclaimed",
            Self::ReversalLimitHidden => "reversal_limit_hidden",
            Self::CollapsedIntoGenericUndo => "collapsed_into_generic_undo",
            Self::ReviewPathMissing => "review_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe review action for this reason.
    pub const fn next_action(self) -> M5RepairReviewAction {
        match self {
            Self::CheckpointStateUnresolved | Self::CheckpointAbsenceHidden => {
                M5RepairReviewAction::InspectCheckpoint
            }
            Self::ReversalClassUnresolved
            | Self::ReversibilityOverclaimed
            | Self::ReversalLimitHidden
            | Self::CollapsedIntoGenericUndo => M5RepairReviewAction::ReviewReversalClass,
            Self::ProofStale => M5RepairReviewAction::ReviewDiagnostics,
            _ => M5RepairReviewAction::ReviewTransaction,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RepairIdUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::RepairTargetIdsUnstated
            }
            Self::CheckpointStateUnresolved | Self::CheckpointAbsenceHidden => {
                M5WorkspaceTrustRepairDowngradeTrigger::CheckpointAbsenceHidden
            }
            Self::ReversibilityOverclaimed | Self::ReversalLimitHidden => {
                M5WorkspaceTrustRepairDowngradeTrigger::ReversalLimitHidden
            }
            Self::CollapsedIntoGenericUndo => {
                M5WorkspaceTrustRepairDowngradeTrigger::ReversalClassCollapsedIntoGenericSuccess
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            Self::ReversalClassUnresolved | Self::ReviewPathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// Maps a preview state and checkpoint state to the single controlled repair disposition, or `None`
/// when the preview or checkpoint cannot be resolved — an unresolved preview never borrows a
/// preview-ready word.
fn disposition_for_preview(
    preview: M5RepairPreviewState,
    checkpoint: M5RepairCheckpointState,
) -> Option<M5WorkspaceTrustRepairDisposition> {
    use M5WorkspaceTrustRepairDisposition as D;
    if matches!(preview, M5RepairPreviewState::PreviewUnknown)
        || matches!(checkpoint, M5RepairCheckpointState::CheckpointUnknown)
    {
        return None;
    }
    if matches!(
        checkpoint,
        M5RepairCheckpointState::CheckpointMissing | M5RepairCheckpointState::CheckpointExpired
    ) {
        Some(D::CheckpointMissing)
    } else {
        Some(D::PreviewReady)
    }
}

/// Maps a reversal class to the single controlled repair disposition, or `None` when the reversal
/// class cannot be resolved.
fn disposition_for_reversal(
    reversal: M5RepairReversalClass,
) -> Option<M5WorkspaceTrustRepairDisposition> {
    use M5WorkspaceTrustRepairDisposition as D;
    match reversal {
        M5RepairReversalClass::ExactReversal => Some(D::ExactReversal),
        M5RepairReversalClass::CompensatingReversal => Some(D::Compensate),
        M5RepairReversalClass::RegenerateReversal => Some(D::Regenerate),
        M5RepairReversalClass::ManualFollowUp => Some(D::ManualFollowUp),
        M5RepairReversalClass::AuditOnly => Some(D::AuditOnly),
        M5RepairReversalClass::ReversalUnknown => None,
    }
}

/// True when a checkpoint state means a checkpoint is present before apply.
fn checkpoint_is_present(checkpoint: M5RepairCheckpointState) -> bool {
    matches!(
        checkpoint,
        M5RepairCheckpointState::CheckpointAvailable
            | M5RepairCheckpointState::CheckpointPartial
            | M5RepairCheckpointState::CheckpointExternal
    )
}

/// True when a checkpoint state means a checkpoint is absent before apply.
fn checkpoint_is_absent(checkpoint: M5RepairCheckpointState) -> bool {
    matches!(
        checkpoint,
        M5RepairCheckpointState::CheckpointMissing | M5RepairCheckpointState::CheckpointExpired
    )
}

/// True when a reversal class permits an honest "reversible" claim (exact or regenerate). Compensating,
/// manual-follow-up, and audit-only reversals never permit a reversible claim.
fn reversal_permits_reversible_claim(reversal: M5RepairReversalClass) -> bool {
    matches!(
        reversal,
        M5RepairReversalClass::ExactReversal | M5RepairReversalClass::RegenerateReversal
    )
}

/// Input to [`resolve_repair_transaction_preview_card`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepairTransactionPreviewCardResolutionInput {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The stable repair id; empty means unstated.
    pub repair_id: String,
    /// The finding ids the repair addresses.
    pub linked_finding_ids: Vec<String>,
    /// The preconditions / prerequisites the repair depends on.
    pub prerequisites: Vec<String>,
    /// True when the prerequisites were evaluated and are stated on the card.
    pub prerequisites_stated: bool,
    /// The checkpoint state before apply.
    pub checkpoint_state: M5RepairCheckpointState,
    /// True when an absent checkpoint's absence is disclosed before apply.
    pub checkpoint_absence_disclosed: bool,
    /// The impact scope the repair will mutate; empty means unstated.
    pub impact_scope: String,
    /// The local / remote / managed target class.
    pub target_class: M5RepairTargetClass,
    /// The preview state of the transaction.
    pub preview_state: M5RepairPreviewState,
    /// True when the card presents as complete and ready to apply.
    pub reads_as_ready: bool,
    /// True when the card collapses the local / remote / managed target into a generic target.
    pub reads_as_generic_target: bool,
    /// True when a command-backed review path is reachable, never docs-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe repair-transaction preview card projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRepairTransactionPreviewCard {
    /// Stable identity of the card instance.
    pub card_id: String,
    /// The stable repair id named by the card.
    pub repair_id: String,
    /// The linked finding ids named by the card.
    pub linked_finding_ids: Vec<String>,
    /// The prerequisites named by the card.
    pub prerequisites: Vec<String>,
    /// Whether the prerequisites are stated on the card.
    pub prerequisites_stated: bool,
    /// The checkpoint-state token named by the card.
    pub checkpoint_state: String,
    /// Whether a checkpoint is present before apply.
    pub checkpoint_present: bool,
    /// Whether a checkpoint is absent before apply.
    pub checkpoint_absent: bool,
    /// The impact scope named by the card.
    pub impact_scope: String,
    /// The local / remote / managed target-class token named by the card.
    pub target_class: String,
    /// Whether the target class is one of the honest local / remote / managed classes.
    pub is_local_remote_or_managed: bool,
    /// The preview-state token named by the card.
    pub preview_state: String,
    /// Whether the preview is complete and ready to apply.
    pub preview_ready: bool,
    /// Single controlled repair disposition, or `null` when the preview / checkpoint is unresolved.
    pub repair_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// Guardrail (MUST be `false` on a clean card): the target class collapses into a generic target.
    pub collapses_target_into_generic: bool,
    /// Guardrail (MUST be `false` on a clean card): a checkpoint's absence is hidden before apply.
    pub hides_checkpoint_absence: bool,
    /// Guardrail (MUST be `false` on a clean card): an incomplete preview reads as ready to apply.
    pub presents_incomplete_as_ready: bool,
    /// Whether a command-backed review path is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the card could not read as a clean, reviewable transaction.
    pub degrade_reason: Option<M5RepairTransactionPreviewCardDegradeReason>,
    /// Next safe review action offered.
    pub next_action: M5RepairReviewAction,
    /// Whether the card reads as a typed, reviewable transaction (clean card).
    pub transaction_reviewable: bool,
}

impl M5ResolvedRepairTransactionPreviewCard {
    /// Whether this card reads as a clean, reviewable transaction.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_rollback_class_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RollbackClassStripResolutionInput {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The stable repair id the strip describes; empty means unstated.
    pub repair_id: String,
    /// The controlled reversal class.
    pub reversal_class: M5RepairReversalClass,
    /// The checkpoint state before apply.
    pub checkpoint_state: M5RepairCheckpointState,
    /// True when an absent checkpoint's absence is disclosed before apply.
    pub checkpoint_absence_disclosed: bool,
    /// True when the strip reads the change as reversible.
    pub reads_as_reversible: bool,
    /// True when a non-exact reversal's limit is disclosed.
    pub reversal_limit_disclosed: bool,
    /// True when the strip collapses distinct reversal classes into a generic "undo".
    pub reads_as_generic_undo: bool,
    /// True when a command-backed review path is reachable, never docs-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe rollback-class strip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRollbackClassStrip {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The stable repair id named by the strip.
    pub repair_id: String,
    /// The controlled reversal-class token named by the strip.
    pub reversal_class: String,
    /// Single controlled repair disposition, or `null` when the reversal class is unresolved.
    pub repair_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The checkpoint-state token named by the strip.
    pub checkpoint_state: String,
    /// Whether a checkpoint is absent before apply.
    pub checkpoint_absent: bool,
    /// Whether the reversal class is an exact reversal.
    pub is_exact_reversal: bool,
    /// Whether the reversal class permits an honest reversible claim (exact or regenerate).
    pub permits_reversible_claim: bool,
    /// Whether the strip claims the change is reversible.
    pub claims_reversible: bool,
    /// Whether a non-exact reversal's limit is disclosed.
    pub reversal_limit_disclosed: bool,
    /// Guardrail (MUST be `false` on a clean strip): reversibility is implied without exact or
    /// regenerate.
    pub overclaims_reversibility: bool,
    /// Guardrail (MUST be `false` on a clean strip): a non-exact reversal's limit is hidden.
    pub hides_reversal_limit: bool,
    /// Guardrail (MUST be `false` on a clean strip): distinct reversal classes collapse into a
    /// generic undo.
    pub collapses_into_generic_undo: bool,
    /// Guardrail (MUST be `false` on a clean strip): a checkpoint's absence is hidden before apply.
    pub hides_checkpoint_absence: bool,
    /// Whether a command-backed review path is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the strip could not read as a clean, honest state.
    pub degrade_reason: Option<M5RollbackClassStripDegradeReason>,
    /// Next safe review action offered.
    pub next_action: M5RepairReviewAction,
    /// Whether the reversal truth stays explicit (clean strip).
    pub reversal_truth_explicit: bool,
}

impl M5ResolvedRollbackClassStrip {
    /// Whether this strip reads as a clean, honest state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RepairPreviewRollbackResolutionError {
    /// The card id was empty.
    EmptyCardId,
    /// The strip id was empty.
    EmptyStripId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RepairPreviewRollbackResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCardId => "empty_card_id",
            Self::EmptyStripId => "empty_strip_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RepairPreviewRollbackResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 repair-preview-rollback resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RepairPreviewRollbackResolutionError {}

/// Resolves a repair-transaction preview card, making a repair preview a typed transaction review:
/// the card names its repair id, linked finding ids, prerequisites, impact scope, checkpoint state,
/// and local / remote / managed target class, discloses checkpoint absence before apply, and never
/// reads an incomplete preview as ready to apply.
pub fn resolve_repair_transaction_preview_card(
    input: M5RepairTransactionPreviewCardResolutionInput,
) -> Result<M5ResolvedRepairTransactionPreviewCard, M5RepairPreviewRollbackResolutionError> {
    if input.card_id.trim().is_empty() {
        return Err(M5RepairPreviewRollbackResolutionError::EmptyCardId);
    }
    if string_is_forbidden(&input.card_id)
        || string_is_forbidden(&input.repair_id)
        || string_is_forbidden(&input.impact_scope)
        || input
            .linked_finding_ids
            .iter()
            .any(|s| string_is_forbidden(s))
    {
        return Err(M5RepairPreviewRollbackResolutionError::ForbiddenMaterial);
    }

    let checkpoint_present = checkpoint_is_present(input.checkpoint_state);
    let checkpoint_absent = checkpoint_is_absent(input.checkpoint_state);
    let hides_checkpoint_absence = checkpoint_absent && !input.checkpoint_absence_disclosed;
    let preview_ready = matches!(input.preview_state, M5RepairPreviewState::PreviewReady);
    let is_local_remote_or_managed = input.target_class.is_local_remote_or_managed();
    let presents_incomplete_as_ready = !preview_ready && input.reads_as_ready;

    let degrade_reason = if input.repair_id.trim().is_empty() {
        Some(M5RepairTransactionPreviewCardDegradeReason::RepairIdUnstated)
    } else if input.linked_finding_ids.is_empty() {
        Some(M5RepairTransactionPreviewCardDegradeReason::LinkedFindingsUnstated)
    } else if !input.prerequisites_stated {
        Some(M5RepairTransactionPreviewCardDegradeReason::PrerequisitesUnstated)
    } else if matches!(
        input.checkpoint_state,
        M5RepairCheckpointState::CheckpointUnknown
    ) {
        Some(M5RepairTransactionPreviewCardDegradeReason::CheckpointStateUnresolved)
    } else if hides_checkpoint_absence {
        Some(M5RepairTransactionPreviewCardDegradeReason::CheckpointAbsenceHidden)
    } else if input.impact_scope.trim().is_empty() {
        Some(M5RepairTransactionPreviewCardDegradeReason::ImpactScopeUnstated)
    } else if matches!(input.target_class, M5RepairTargetClass::TargetUnknown) {
        Some(M5RepairTransactionPreviewCardDegradeReason::TargetClassUnresolved)
    } else if !preview_ready {
        Some(M5RepairTransactionPreviewCardDegradeReason::PreviewNotReady)
    } else if input.reads_as_generic_target {
        Some(M5RepairTransactionPreviewCardDegradeReason::TargetCollapsedIntoGeneric)
    } else if !input.detail_command_available {
        Some(M5RepairTransactionPreviewCardDegradeReason::ReviewPathMissing)
    } else if !input.proof_fresh {
        Some(M5RepairTransactionPreviewCardDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RepairReviewAction::ReviewTransaction,
    };

    Ok(M5ResolvedRepairTransactionPreviewCard {
        card_id: input.card_id,
        repair_id: input.repair_id,
        linked_finding_ids: input.linked_finding_ids,
        prerequisites: input.prerequisites,
        prerequisites_stated: input.prerequisites_stated,
        checkpoint_state: input.checkpoint_state.as_str().to_owned(),
        checkpoint_present,
        checkpoint_absent,
        impact_scope: input.impact_scope,
        target_class: input.target_class.as_str().to_owned(),
        is_local_remote_or_managed,
        preview_state: input.preview_state.as_str().to_owned(),
        preview_ready,
        repair_disposition: disposition_for_preview(input.preview_state, input.checkpoint_state),
        collapses_target_into_generic: input.reads_as_generic_target,
        hides_checkpoint_absence,
        presents_incomplete_as_ready,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        transaction_reviewable: degrade_reason.is_none(),
    })
}

/// Resolves a rollback-class strip, keeping reversal truth honest: the strip names its repair id and
/// controlled reversal class, discloses checkpoint absence before apply, never implies reversibility
/// without an exact or regenerate reversal, never hides a non-exact reversal's limit, and never
/// collapses distinct reversal classes into a generic undo.
pub fn resolve_rollback_class_strip(
    input: M5RollbackClassStripResolutionInput,
) -> Result<M5ResolvedRollbackClassStrip, M5RepairPreviewRollbackResolutionError> {
    if input.strip_id.trim().is_empty() {
        return Err(M5RepairPreviewRollbackResolutionError::EmptyStripId);
    }
    if string_is_forbidden(&input.strip_id) || string_is_forbidden(&input.repair_id) {
        return Err(M5RepairPreviewRollbackResolutionError::ForbiddenMaterial);
    }

    let is_exact_reversal = matches!(input.reversal_class, M5RepairReversalClass::ExactReversal);
    let reversal_resolved = !matches!(input.reversal_class, M5RepairReversalClass::ReversalUnknown);
    let permits_reversible_claim = reversal_permits_reversible_claim(input.reversal_class);
    let checkpoint_absent = checkpoint_is_absent(input.checkpoint_state);
    let hides_checkpoint_absence = checkpoint_absent && !input.checkpoint_absence_disclosed;
    let overclaims_reversibility = input.reads_as_reversible && !permits_reversible_claim;
    let needs_limit_disclosure = reversal_resolved && !is_exact_reversal;
    let hides_reversal_limit = needs_limit_disclosure && !input.reversal_limit_disclosed;

    let degrade_reason = if input.repair_id.trim().is_empty() {
        Some(M5RollbackClassStripDegradeReason::RepairIdUnstated)
    } else if !reversal_resolved {
        Some(M5RollbackClassStripDegradeReason::ReversalClassUnresolved)
    } else if matches!(
        input.checkpoint_state,
        M5RepairCheckpointState::CheckpointUnknown
    ) {
        Some(M5RollbackClassStripDegradeReason::CheckpointStateUnresolved)
    } else if hides_checkpoint_absence {
        Some(M5RollbackClassStripDegradeReason::CheckpointAbsenceHidden)
    } else if overclaims_reversibility {
        Some(M5RollbackClassStripDegradeReason::ReversibilityOverclaimed)
    } else if hides_reversal_limit {
        Some(M5RollbackClassStripDegradeReason::ReversalLimitHidden)
    } else if input.reads_as_generic_undo {
        Some(M5RollbackClassStripDegradeReason::CollapsedIntoGenericUndo)
    } else if !input.detail_command_available {
        Some(M5RollbackClassStripDegradeReason::ReviewPathMissing)
    } else if !input.proof_fresh {
        Some(M5RollbackClassStripDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RepairReviewAction::ReviewTransaction,
    };

    Ok(M5ResolvedRollbackClassStrip {
        strip_id: input.strip_id,
        repair_id: input.repair_id,
        reversal_class: input.reversal_class.as_str().to_owned(),
        repair_disposition: disposition_for_reversal(input.reversal_class),
        checkpoint_state: input.checkpoint_state.as_str().to_owned(),
        checkpoint_absent,
        is_exact_reversal,
        permits_reversible_claim,
        claims_reversible: input.reads_as_reversible,
        reversal_limit_disclosed: input.reversal_limit_disclosed,
        overclaims_reversibility,
        hides_reversal_limit,
        collapses_into_generic_undo: input.reads_as_generic_undo,
        hides_checkpoint_absence,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        reversal_truth_explicit: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved repair-transaction preview card and
/// rollback-class strip examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairPreviewRollbackControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5RepairPreviewRollbackConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5WorkspaceTrustRepairQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5WorkspaceTrustRepairDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5WorkspaceTrustRepairRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5WorkspaceTrustRepairAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5RepairPreviewRollbackAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RepairPreviewRollbackExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    /// Resolved repair-transaction preview card examples.
    pub repair_transaction_preview_card_examples: Vec<M5ResolvedRepairTransactionPreviewCard>,
    /// Resolved rollback-class strip examples.
    pub rollback_class_strip_examples: Vec<M5ResolvedRollbackClassStrip>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never hide checkpoint absence or reversal limits.
    pub hides_checkpoint_absence_or_reversal_limits: bool,
    /// Hard invariant: never collapse distinct reversal classes into a generic success.
    pub collapses_reversal_classes_into_generic_success: bool,
    /// Hard invariant: never imply reversibility without an exact or regenerate reversal.
    pub implies_reversibility_without_exact_or_regenerate: bool,
    /// Hard invariant: never hide the local / remote / managed target class or impact scope.
    pub hides_target_class_or_impact_scope: bool,
}

impl M5RepairPreviewRollbackControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RepairPreviewRollbackAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RepairPreviewRollbackAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RepairPreviewRollbackExportField> =
            self.export_fields.iter().copied().collect();
        M5RepairPreviewRollbackExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.hides_checkpoint_absence_or_reversal_limits
            && !self.collapses_reversal_classes_into_generic_success
            && !self.implies_reversibility_without_exact_or_regenerate
            && !self.hides_target_class_or_impact_scope
    }

    /// True when every resolved example on this row is honest: no clean card hides checkpoint
    /// absence, collapses its target, or reads an incomplete preview as ready, and no clean strip
    /// overclaims reversibility, hides a reversal limit, or collapses into a generic undo — and no
    /// clean example hides the command-backed review path.
    fn examples_are_honest(&self) -> bool {
        self.repair_transaction_preview_card_examples
            .iter()
            .all(|ex| {
                !(ex.is_clean()
                    && (ex.hides_checkpoint_absence
                        || ex.collapses_target_into_generic
                        || ex.presents_incomplete_as_ready
                        || !ex.detail_command_available))
            })
            && self.rollback_class_strip_examples.iter().all(|ex| {
                !(ex.is_clean()
                    && (ex.overclaims_reversibility
                        || ex.hides_reversal_limit
                        || ex.collapses_into_generic_undo
                        || ex.hides_checkpoint_absence
                        || !ex.detail_command_available))
            })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairPreviewRollbackVocabularySet {
    /// Repair-disposition tokens (bound from the frozen matrix).
    pub repair_dispositions: Vec<String>,
    /// Reversal-class tokens (bound from the frozen matrix).
    pub reversal_classes: Vec<String>,
    /// Checkpoint-state tokens (bound from the frozen matrix).
    pub checkpoint_states: Vec<String>,
    /// Preview-state tokens (bound from the frozen matrix).
    pub preview_states: Vec<String>,
    /// Target-class tokens (minted by this lane).
    pub target_classes: Vec<String>,
    /// Review-action tokens (minted by this lane).
    pub review_actions: Vec<String>,
    /// Card degrade-reason tokens.
    pub card_degrade_reasons: Vec<String>,
    /// Strip degrade-reason tokens.
    pub strip_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5RepairPreviewRollbackVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            repair_dispositions: tokens(&M5WorkspaceTrustRepairDisposition::ALL, |v| v.as_str()),
            reversal_classes: tokens(&M5RepairReversalClass::ALL, |v| v.as_str()),
            checkpoint_states: tokens(&M5RepairCheckpointState::ALL, |v| v.as_str()),
            preview_states: tokens(&M5RepairPreviewState::ALL, |v| v.as_str()),
            target_classes: tokens(&M5RepairTargetClass::ALL, |v| v.as_str()),
            review_actions: tokens(&M5RepairReviewAction::ALL, |v| v.as_str()),
            card_degrade_reasons: tokens(&M5RepairTransactionPreviewCardDegradeReason::ALL, |v| {
                v.as_str()
            }),
            strip_degrade_reasons: tokens(&M5RollbackClassStripDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5RepairPreviewRollbackAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RepairPreviewRollbackExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5WorkspaceTrustRepairConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5RepairPreviewRollbackGovernanceReview {
    /// Every preview card names its repair id and at least one linked finding id.
    pub preview_card_names_repair_id_and_linked_findings: bool,
    /// Every preview card names its prerequisites, impact scope, and target class.
    pub preview_card_names_prerequisites_impact_and_target_class: bool,
    /// Checkpoint presence or absence is visible before apply, not after mutation.
    pub checkpoint_presence_or_absence_visible_before_apply: bool,
    /// Every rollback strip uses the controlled exact / compensate / regenerate / manual / audit-only
    /// vocabulary.
    pub rollback_strip_uses_controlled_reversal_vocabulary: bool,
    /// Reversibility is never implied without an exact or regenerate reversal.
    pub reversibility_never_implied_without_exact_or_regenerate: bool,
    /// A non-exact reversal always discloses its limit.
    pub reversal_limits_always_disclosed: bool,
    /// Guided-repair surfaces share one transaction-preview grammar and one reversal vocabulary.
    pub repair_vocabulary_shared_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairPreviewRollbackConsumerProjection {
    /// Repair previews across consumers expose the same transaction-preview grammar.
    pub repair_previews_expose_same_transaction_grammar: bool,
    /// Reversal class is legible without hunting through docs or logs.
    pub reversal_class_legible_without_docs: bool,
    /// Repair state traces back to one canonical component contract.
    pub repair_state_traces_to_single_component_contract: bool,
    /// Support / export reads a single canonical repair source.
    pub support_export_reads_single_repair_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairPreviewRollbackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairPreviewRollbackReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RepairPreviewRollbackControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepairPreviewRollbackControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5RepairPreviewRollbackControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepairPreviewRollbackVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepairPreviewRollbackGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepairPreviewRollbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepairPreviewRollbackProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepairPreviewRollbackReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 repair-transaction-preview-card / rollback-class-strip controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairPreviewRollbackControlsPacket {
    /// Record kind; must equal [`M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5RepairPreviewRollbackControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepairPreviewRollbackVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepairPreviewRollbackGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepairPreviewRollbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepairPreviewRollbackProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepairPreviewRollbackReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RepairPreviewRollbackControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5RepairPreviewRollbackControlsPacketInput) -> Self {
        Self {
            record_kind: M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5RepairPreviewRollbackControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_RECORD_KIND {
            violations.push(M5RepairPreviewRollbackControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_VERSION {
            violations.push(M5RepairPreviewRollbackControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RepairPreviewRollbackControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5RepairPreviewRollbackControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 repair-preview-rollback controls packet serializes"),
        ) {
            violations.push(M5RepairPreviewRollbackControlsViolation::RawMaterialInExport);
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
            .expect("m5 repair-preview-rollback controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,card_examples,strip_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .repair_transaction_preview_card_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.rollback_class_strip_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.repair_transaction_preview_card_examples.len(),
                row.rollback_class_strip_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Repair-Transaction-Preview-Card and Rollback-Class-Strip Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Reversal classes: {}\n",
            self.vocabulary_set.reversal_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Target classes: {}\n",
            self.vocabulary_set.target_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Checkpoint states: {}\n",
            self.vocabulary_set.checkpoint_states.join(", ")
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
                "  - Card examples: {} / strip examples: {}\n",
                row.repair_transaction_preview_card_examples.len(),
                row.rollback_class_strip_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5RepairPreviewRollbackControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RepairPreviewRollbackControlsViolation>),
}

impl fmt::Display for M5RepairPreviewRollbackControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 repair-preview-rollback controls export parse failed: {error}"
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
                    "m5 repair-preview-rollback controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RepairPreviewRollbackControlsArtifactError {}

/// Validation failures emitted by [`M5RepairPreviewRollbackControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RepairPreviewRollbackControlsViolation {
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
    /// A controls row carries a dishonest clean example (hidden checkpoint absence, overclaimed
    /// reversibility, collapse, or hidden review path).
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
    /// Transaction-preview grammar is not proven: clean cards do not cover the local / remote /
    /// managed target classes and checkpoint presence and absence, or no checkpoint-absence-hidden
    /// example degrades, or a clean card is dishonest.
    TransactionGrammarNotProven,
    /// Reversal truth is not proven: clean strips do not cover exact and a non-reversible class, or no
    /// reversibility-overclaimed / reversal-limit-hidden example degrades, or a clean strip is
    /// dishonest.
    ReversalTruthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RepairPreviewRollbackControlsViolation {
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
            Self::TransactionGrammarNotProven => "transaction_grammar_not_proven",
            Self::ReversalTruthNotProven => "reversal_truth_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_repair_preview_rollback_controls_export(
) -> Result<M5RepairPreviewRollbackControlsPacket, M5RepairPreviewRollbackControlsArtifactError> {
    let packet: M5RepairPreviewRollbackControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/support_export.json"
    )))
    .map_err(M5RepairPreviewRollbackControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RepairPreviewRollbackControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_SCHEMA_REF,
        M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF,
        M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RepairPreviewRollbackControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5RepairPreviewRollbackControlsViolation::NoControlsRows);
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
            violations.push(M5RepairPreviewRollbackControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5RepairPreviewRollbackControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RepairPreviewRollbackControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_REPAIR_TRANSACTION_PREVIEW_CARD_SCHEMA_REF)
            || !refs.contains(M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF)
        {
            violations.push(M5RepairPreviewRollbackControlsViolation::ComponentSchemaRefMissing);
        }
        if row.repair_transaction_preview_card_examples.is_empty()
            || row.rollback_class_strip_examples.is_empty()
        {
            violations.push(M5RepairPreviewRollbackControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5RepairPreviewRollbackControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5RepairPreviewRollbackControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.preview_card_names_repair_id_and_linked_findings,
        review.preview_card_names_prerequisites_impact_and_target_class,
        review.checkpoint_presence_or_absence_visible_before_apply,
        review.rollback_strip_uses_controlled_reversal_vocabulary,
        review.reversibility_never_implied_without_exact_or_regenerate,
        review.reversal_limits_always_disclosed,
        review.repair_vocabulary_shared_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5RepairPreviewRollbackControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.repair_previews_expose_same_transaction_grammar,
        projection.reversal_class_legible_without_docs,
        projection.repair_state_traces_to_single_component_contract,
        projection.support_export_reads_single_repair_source,
    ] {
        if !ok {
            violations.push(M5RepairPreviewRollbackControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RepairPreviewRollbackControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RepairPreviewRollbackControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5RepairPreviewRollbackControlsPacket,
    violations: &mut Vec<M5RepairPreviewRollbackControlsViolation>,
) {
    let cards = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.repair_transaction_preview_card_examples.iter())
    };
    let strips = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.rollback_class_strip_examples.iter())
    };

    // AC: guided-repair flows show one transaction-preview grammar, and checkpoint presence or
    // absence is visible before apply. Clean cards cover the local / remote / managed target classes
    // and both a present and an absent (disclosed) checkpoint; at least one card degrades to
    // checkpoint-absence-hidden; no clean card is dishonest; and every clean card names its repair
    // id, at least one linked finding, prerequisites, and impact scope.
    let clean_targets: BTreeSet<&str> = cards()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.target_class.as_str())
        .collect();
    let covers_target_classes = [
        M5RepairTargetClass::LocalWorkspace,
        M5RepairTargetClass::RemoteHost,
        M5RepairTargetClass::ManagedWorkspace,
    ]
    .iter()
    .all(|class| clean_targets.contains(class.as_str()));
    let clean_checkpoint_present = cards().any(|ex| ex.is_clean() && ex.checkpoint_present);
    let clean_checkpoint_absent = cards().any(|ex| ex.is_clean() && ex.checkpoint_absent);
    let checkpoint_hidden_degrades = cards().any(|ex| {
        ex.degrade_reason
            == Some(M5RepairTransactionPreviewCardDegradeReason::CheckpointAbsenceHidden)
            && ex.hides_checkpoint_absence
    });
    let no_clean_dishonest_card = cards().all(|ex| {
        !(ex.is_clean()
            && (ex.hides_checkpoint_absence
                || ex.collapses_target_into_generic
                || ex.presents_incomplete_as_ready))
    });
    let clean_cards_named = cards().filter(|ex| ex.is_clean()).all(|ex| {
        !ex.repair_id.trim().is_empty()
            && !ex.linked_finding_ids.is_empty()
            && ex.prerequisites_stated
            && !ex.impact_scope.trim().is_empty()
    });
    if !(covers_target_classes
        && clean_checkpoint_present
        && clean_checkpoint_absent
        && checkpoint_hidden_degrades
        && no_clean_dishonest_card
        && clean_cards_named)
    {
        violations.push(M5RepairPreviewRollbackControlsViolation::TransactionGrammarNotProven);
    }

    // AC: one reversal vocabulary, and reversibility is truthful about its limits. Clean strips cover
    // an exact reversal and at least one non-reversible class (compensate / manual / audit-only); at
    // least one strip degrades to reversibility-overclaimed and one to reversal-limit-hidden; no clean
    // strip is dishonest; and every clean non-reversible strip discloses its limit and never claims
    // reversibility.
    let clean_reversals: BTreeSet<&str> = strips()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.reversal_class.as_str())
        .collect();
    let covers_exact = clean_reversals.contains(M5RepairReversalClass::ExactReversal.as_str());
    let covers_non_reversible = [
        M5RepairReversalClass::CompensatingReversal,
        M5RepairReversalClass::ManualFollowUp,
        M5RepairReversalClass::AuditOnly,
    ]
    .iter()
    .any(|class| clean_reversals.contains(class.as_str()));
    let overclaim_degrades = strips().any(|ex| {
        ex.degrade_reason == Some(M5RollbackClassStripDegradeReason::ReversibilityOverclaimed)
            && ex.overclaims_reversibility
    });
    let limit_hidden_degrades = strips().any(|ex| {
        ex.degrade_reason == Some(M5RollbackClassStripDegradeReason::ReversalLimitHidden)
            && ex.hides_reversal_limit
    });
    let no_clean_dishonest_strip = strips().all(|ex| {
        !(ex.is_clean()
            && (ex.overclaims_reversibility
                || ex.hides_reversal_limit
                || ex.collapses_into_generic_undo))
    });
    let non_reversible_clean_honest = strips()
        .filter(|ex| ex.is_clean() && !ex.permits_reversible_claim)
        .all(|ex| !ex.claims_reversible && ex.reversal_limit_disclosed);
    if !(covers_exact
        && covers_non_reversible
        && overclaim_degrades
        && limit_hidden_degrades
        && no_clean_dishonest_strip
        && non_reversible_clean_honest)
    {
        violations.push(M5RepairPreviewRollbackControlsViolation::ReversalTruthNotProven);
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

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5WorkspaceTrustRepairComponentFamily; 2] = [
    M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard,
    M5WorkspaceTrustRepairComponentFamily::RollbackClassStrip,
];
