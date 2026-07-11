//! Implemented M5 repair-result-receipt-row and checkpoint-lineage-disclosure primitives.
//!
//! The frozen [workspace-trust / guided-repair component matrix][matrix] names the reusable trust
//! and repair UI components and locks their controlled vocabulary. This module is the guided-repair
//! result lane over that matrix: it turns the **repair-result receipt row** and the
//! **checkpoint-lineage disclosure** into resolvers that produce export-safe, honest projections, so
//! what actually happened after a repair is preserved instead of collapsing into a generic "Fixed"
//! or "Failed".
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render repair-result receipt rows with applied-versus-skipped scope, partial-success markers,
//!   linked checkpoint refs, and compensation / manual-follow-up state.**
//!   [`resolve_repair_result_receipt_row`] refuses to read as a clean, attributable receipt unless it
//!   names its repair id, at least one linked finding id, its applied scope (for any non-failure
//!   outcome), a resolved checkpoint state with a linked checkpoint ref when a checkpoint is present,
//!   a resolved reversal class, and — whenever the outcome needs it — its compensation / manual
//!   follow-up state; it degrades instead. It never collapses distinct
//!   [`M5RepairOutcomeClass`] outcomes into a generic success and never presents a partial success as
//!   complete.
//! * **Expose checkpoint-lineage disclosures so users and support can trace a repair from finding to
//!   preview to apply to result.** [`resolve_checkpoint_lineage_disclosure`] refuses to read as a
//!   traceable lineage unless every stage — finding, preview, checkpoint, apply, and result — is
//!   named, and it never collapses the four stages into a single opaque status.
//! * **Join repair receipts to support export and escalation packets without feature-local
//!   translation.** Every clean receipt keeps a command-backed support-export path reachable and
//!   traces back to one canonical receipt / lineage object, so a support packet can cite a single
//!   canonical receipt instead of re-deriving the outcome per surface.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5WorkspaceTrustRepairDisposition`] trust / repair-disposition vocabulary, the
//! [`M5RepairOutcomeClass`] repair-outcome vocabulary, the [`M5RepairReversalClass`] reversal
//! vocabulary, and the [`M5RepairCheckpointState`] checkpoint vocabulary — so every claimed M5
//! guided-repair surface exposes the same receipt and lineage grammar instead of forking its own
//! outcome copy.
//!
//! [matrix]: crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_repair_receipt_lineage_controls,
    seeded_m5_repair_receipt_lineage_controls_doctor_ui_beta_narrowed,
    seeded_m5_repair_receipt_lineage_controls_safe_mode_ui_preview_narrowed,
    M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix::{
    M5RepairCheckpointState, M5RepairOutcomeClass, M5RepairReversalClass,
    M5WorkspaceTrustRepairAccessibilityRoute, M5WorkspaceTrustRepairComponentFamily,
    M5WorkspaceTrustRepairConsumerSurface, M5WorkspaceTrustRepairDeploymentLine,
    M5WorkspaceTrustRepairDisposition, M5WorkspaceTrustRepairDowngradeTrigger,
    M5WorkspaceTrustRepairQualificationClass, M5WorkspaceTrustRepairRequiredLabel,
    M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF, M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5RepairReceiptLineageControlsPacket`].
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_repair_result_receipt_row_and_checkpoint_lineage_disclosure_controls";

/// Schema version for M5 repair-result-receipt-row / checkpoint-lineage-disclosure controls records.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_DOC_REF: &str =
    "docs/trust/m5_repair_result_receipt_row_and_checkpoint_lineage_disclosure_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5RepairReceiptLineageConsumerSurface = M5WorkspaceTrustRepairConsumerSurface;

/// The next safe review action a receipt row or checkpoint-lineage disclosure surfaces so a user is
/// never left without a route to inspect what happened or to cite the receipt in support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairReceiptReviewAction {
    /// Review the repair-result receipt — the clean default.
    ReviewReceipt,
    /// Inspect the checkpoint-lineage trace behind the receipt.
    InspectCheckpointLineage,
    /// Review the compensation / manual follow-up the outcome still needs.
    ReviewFollowUp,
    /// Review the skipped scope a partial success left behind.
    ReviewSkippedScope,
    /// Open the canonical support-export / escalation packet the receipt joins.
    OpenSupportPacket,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5RepairReceiptReviewAction {
    /// Every review action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewReceipt,
        Self::InspectCheckpointLineage,
        Self::ReviewFollowUp,
        Self::ReviewSkippedScope,
        Self::OpenSupportPacket,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewReceipt => "review_receipt",
            Self::InspectCheckpointLineage => "inspect_checkpoint_lineage",
            Self::ReviewFollowUp => "review_follow_up",
            Self::ReviewSkippedScope => "review_skipped_scope",
            Self::OpenSupportPacket => "open_support_packet",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// One mandatory rendered part a repair-result receipt row or checkpoint-lineage disclosure must be
/// able to show, so no repair-result fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairReceiptLineageAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed repair disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The stable repair id behind the receipt / lineage (both components).
    RepairId,
    /// The linked finding ids the repair addressed (both components).
    LinkedFindingIds,
    /// The controlled repair-outcome class (receipt row).
    OutcomeClass,
    /// The applied-versus-skipped scope of the repair (receipt row).
    AppliedVersusSkippedScope,
    /// The linked checkpoint ref before / at apply (both components).
    CheckpointRef,
    /// The controlled reversal class the outcome carries (receipt row).
    ReversalClass,
    /// The compensation / manual follow-up state the outcome still needs (receipt row).
    FollowUpState,
    /// The finding-to-preview-to-apply-to-result checkpoint lineage (lineage disclosure).
    CheckpointLineage,
    /// The command-backed support-export / escalation link (both components).
    SupportExportLink,
}

impl M5RepairReceiptLineageAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::RepairId,
        Self::LinkedFindingIds,
        Self::OutcomeClass,
        Self::AppliedVersusSkippedScope,
        Self::CheckpointRef,
        Self::ReversalClass,
        Self::FollowUpState,
        Self::CheckpointLineage,
        Self::SupportExportLink,
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
            Self::OutcomeClass => "outcome_class",
            Self::AppliedVersusSkippedScope => "applied_versus_skipped_scope",
            Self::CheckpointRef => "checkpoint_ref",
            Self::ReversalClass => "reversal_class",
            Self::FollowUpState => "follow_up_state",
            Self::CheckpointLineage => "checkpoint_lineage",
            Self::SupportExportLink => "support_export_link",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairReceiptLineageExportField {
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
    /// The repair id named by the components.
    RepairId,
    /// The linked finding ids named by the components.
    LinkedFindingIds,
    /// The repair-outcome class named by the receipt.
    OutcomeClass,
    /// The applied scope named by the receipt.
    AppliedScope,
    /// The skipped scope named by the receipt.
    SkippedScope,
    /// The linked checkpoint ref named by the components.
    CheckpointRef,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RepairReceiptLineageExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::RepairDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::RepairId,
        Self::LinkedFindingIds,
        Self::OutcomeClass,
        Self::AppliedScope,
        Self::SkippedScope,
        Self::CheckpointRef,
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
            Self::OutcomeClass => "outcome_class",
            Self::AppliedScope => "applied_scope",
            Self::SkippedScope => "skipped_scope",
            Self::CheckpointRef => "checkpoint_ref",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a repair-result receipt row degraded below a clean, attributable state. The degrade-first
/// ladder returns one of these instead of ever letting a repair result read as a generic "Fixed".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairResultReceiptRowDegradeReason {
    /// The stable repair id is unstated; the outcome cannot be attributed to a transaction.
    RepairIdUnstated,
    /// No linked finding id is named; the outcome is not tied back to a finding.
    LinkedFindingsUnstated,
    /// A non-failure outcome names no applied scope; the receipt cannot say what changed.
    AppliedScopeUnstated,
    /// The checkpoint state cannot currently be resolved.
    CheckpointStateUnresolved,
    /// A checkpoint is present but no linked checkpoint ref is named.
    CheckpointRefUnstated,
    /// The reversal class cannot currently be resolved.
    ReversalClassUnresolved,
    /// The outcome needs compensation or manual follow-up but that state is left unstated.
    FollowUpStateUnstated,
    /// A partial success reads as a complete success.
    PartialSuccessShownAsComplete,
    /// Distinct outcomes collapsed into a generic success.
    OutcomeCollapsedIntoGenericSuccess,
    /// No command-backed support-export path is reachable; the receipt would need local translation.
    SupportExportPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RepairResultReceiptRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::RepairIdUnstated,
        Self::LinkedFindingsUnstated,
        Self::AppliedScopeUnstated,
        Self::CheckpointStateUnresolved,
        Self::CheckpointRefUnstated,
        Self::ReversalClassUnresolved,
        Self::FollowUpStateUnstated,
        Self::PartialSuccessShownAsComplete,
        Self::OutcomeCollapsedIntoGenericSuccess,
        Self::SupportExportPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairIdUnstated => "repair_id_unstated",
            Self::LinkedFindingsUnstated => "linked_findings_unstated",
            Self::AppliedScopeUnstated => "applied_scope_unstated",
            Self::CheckpointStateUnresolved => "checkpoint_state_unresolved",
            Self::CheckpointRefUnstated => "checkpoint_ref_unstated",
            Self::ReversalClassUnresolved => "reversal_class_unresolved",
            Self::FollowUpStateUnstated => "follow_up_state_unstated",
            Self::PartialSuccessShownAsComplete => "partial_success_shown_as_complete",
            Self::OutcomeCollapsedIntoGenericSuccess => "outcome_collapsed_into_generic_success",
            Self::SupportExportPathMissing => "support_export_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe review action for this reason.
    pub const fn next_action(self) -> M5RepairReceiptReviewAction {
        match self {
            Self::CheckpointStateUnresolved | Self::CheckpointRefUnstated => {
                M5RepairReceiptReviewAction::InspectCheckpointLineage
            }
            Self::FollowUpStateUnstated => M5RepairReceiptReviewAction::ReviewFollowUp,
            Self::PartialSuccessShownAsComplete => M5RepairReceiptReviewAction::ReviewSkippedScope,
            Self::SupportExportPathMissing => M5RepairReceiptReviewAction::OpenSupportPacket,
            _ => M5RepairReceiptReviewAction::ReviewReceipt,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RepairIdUnstated | Self::LinkedFindingsUnstated | Self::AppliedScopeUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::RepairTargetIdsUnstated
            }
            Self::CheckpointStateUnresolved | Self::CheckpointRefUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::CheckpointAbsenceHidden
            }
            Self::PartialSuccessShownAsComplete => {
                M5WorkspaceTrustRepairDowngradeTrigger::PartialSuccessShownAsComplete
            }
            Self::OutcomeCollapsedIntoGenericSuccess => {
                M5WorkspaceTrustRepairDowngradeTrigger::ReversalClassCollapsedIntoGenericSuccess
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            Self::ReversalClassUnresolved
            | Self::FollowUpStateUnstated
            | Self::SupportExportPathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// Reason a checkpoint-lineage disclosure degraded below a clean, traceable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckpointLineageDisclosureDegradeReason {
    /// The stable repair id is unstated; the lineage cannot be tied to a transaction.
    RepairIdUnstated,
    /// The finding stage is unstated; the lineage does not start at a finding.
    FindingLinkUnstated,
    /// The preview stage ref is unstated; the lineage skips the preview.
    PreviewRefUnstated,
    /// The checkpoint state cannot currently be resolved.
    CheckpointStateUnresolved,
    /// The apply stage ref is unstated; the lineage skips the apply.
    ApplyRefUnstated,
    /// The result stage ref is unstated; the lineage does not reach a canonical receipt.
    ResultRefUnstated,
    /// The finding-to-preview-to-apply-to-result stages collapsed into a single opaque status.
    StagesCollapsedIntoSingleStatus,
    /// No command-backed support-export / lineage path is reachable.
    LineagePathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CheckpointLineageDisclosureDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RepairIdUnstated,
        Self::FindingLinkUnstated,
        Self::PreviewRefUnstated,
        Self::CheckpointStateUnresolved,
        Self::ApplyRefUnstated,
        Self::ResultRefUnstated,
        Self::StagesCollapsedIntoSingleStatus,
        Self::LineagePathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairIdUnstated => "repair_id_unstated",
            Self::FindingLinkUnstated => "finding_link_unstated",
            Self::PreviewRefUnstated => "preview_ref_unstated",
            Self::CheckpointStateUnresolved => "checkpoint_state_unresolved",
            Self::ApplyRefUnstated => "apply_ref_unstated",
            Self::ResultRefUnstated => "result_ref_unstated",
            Self::StagesCollapsedIntoSingleStatus => "stages_collapsed_into_single_status",
            Self::LineagePathMissing => "lineage_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe review action for this reason.
    pub const fn next_action(self) -> M5RepairReceiptReviewAction {
        match self {
            Self::CheckpointStateUnresolved => {
                M5RepairReceiptReviewAction::InspectCheckpointLineage
            }
            Self::ResultRefUnstated | Self::LineagePathMissing => {
                M5RepairReceiptReviewAction::OpenSupportPacket
            }
            Self::ProofStale => M5RepairReceiptReviewAction::ReviewReceipt,
            _ => M5RepairReceiptReviewAction::InspectCheckpointLineage,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RepairIdUnstated | Self::FindingLinkUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::RepairTargetIdsUnstated
            }
            Self::CheckpointStateUnresolved => {
                M5WorkspaceTrustRepairDowngradeTrigger::CheckpointAbsenceHidden
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            Self::PreviewRefUnstated
            | Self::ApplyRefUnstated
            | Self::ResultRefUnstated
            | Self::StagesCollapsedIntoSingleStatus
            | Self::LineagePathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
        }
    }
}

/// Maps a repair-outcome class to the single controlled repair disposition, or `None` when no single
/// disposition token honestly applies (a plain success/failure with no reversal posture).
fn disposition_for_outcome(
    outcome: M5RepairOutcomeClass,
) -> Option<M5WorkspaceTrustRepairDisposition> {
    use M5WorkspaceTrustRepairDisposition as D;
    match outcome {
        M5RepairOutcomeClass::RepairAppliedExact => Some(D::ExactReversal),
        M5RepairOutcomeClass::RepairCompensated => Some(D::Compensate),
        M5RepairOutcomeClass::RepairRegenerated => Some(D::Regenerate),
        M5RepairOutcomeClass::RepairPartialSuccess => Some(D::ManualFollowUp),
        M5RepairOutcomeClass::RepairManualRequired => Some(D::ManualFollowUp),
        M5RepairOutcomeClass::RepairFailed => None,
    }
}

/// True when a checkpoint state means a checkpoint is present.
fn checkpoint_is_present(checkpoint: M5RepairCheckpointState) -> bool {
    matches!(
        checkpoint,
        M5RepairCheckpointState::CheckpointAvailable
            | M5RepairCheckpointState::CheckpointPartial
            | M5RepairCheckpointState::CheckpointExternal
    )
}

/// True when a repair-outcome class is a first-class partial success.
fn outcome_is_partial_success(outcome: M5RepairOutcomeClass) -> bool {
    matches!(outcome, M5RepairOutcomeClass::RepairPartialSuccess)
}

/// True when a repair-outcome class is a total failure where nothing changed.
fn outcome_is_failure(outcome: M5RepairOutcomeClass) -> bool {
    matches!(outcome, M5RepairOutcomeClass::RepairFailed)
}

/// True when a repair outcome needs compensation or manual follow-up before it is really finished.
fn outcome_requires_follow_up(outcome: M5RepairOutcomeClass) -> bool {
    matches!(
        outcome,
        M5RepairOutcomeClass::RepairCompensated
            | M5RepairOutcomeClass::RepairPartialSuccess
            | M5RepairOutcomeClass::RepairManualRequired
    )
}

/// Input to [`resolve_repair_result_receipt_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepairResultReceiptRowResolutionInput {
    /// Stable identity of the receipt instance.
    pub receipt_id: String,
    /// The stable repair id the receipt records; empty means unstated.
    pub repair_id: String,
    /// The finding ids the repair addressed.
    pub linked_finding_ids: Vec<String>,
    /// The applied result of the repair transaction.
    pub outcome_class: M5RepairOutcomeClass,
    /// The scope the repair actually applied to.
    pub applied_scope: Vec<String>,
    /// The scope the repair skipped / left behind.
    pub skipped_scope: Vec<String>,
    /// The checkpoint state at apply.
    pub checkpoint_state: M5RepairCheckpointState,
    /// The linked checkpoint ref; empty means unstated.
    pub checkpoint_ref: String,
    /// The reversal class the applied outcome carries.
    pub reversal_class: M5RepairReversalClass,
    /// True when the compensation / manual follow-up state is named on the receipt.
    pub follow_up_stated: bool,
    /// True when the receipt reads the outcome as fully complete.
    pub reads_as_complete: bool,
    /// True when the receipt collapses distinct outcomes into a generic success.
    pub reads_as_generic_success: bool,
    /// True when a command-backed support-export path is reachable, never docs-only.
    pub support_export_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe repair-result receipt row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRepairResultReceiptRow {
    /// Stable identity of the receipt instance.
    pub receipt_id: String,
    /// The stable repair id named by the receipt.
    pub repair_id: String,
    /// The linked finding ids named by the receipt.
    pub linked_finding_ids: Vec<String>,
    /// The repair-outcome token named by the receipt.
    pub outcome_class: String,
    /// Single controlled repair disposition, or `null` when no single disposition applies.
    pub repair_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The scope the repair applied to.
    pub applied_scope: Vec<String>,
    /// The scope the repair skipped / left behind.
    pub skipped_scope: Vec<String>,
    /// Whether this receipt records a first-class partial success.
    pub is_partial_success: bool,
    /// The checkpoint-state token named by the receipt.
    pub checkpoint_state: String,
    /// The linked checkpoint ref named by the receipt.
    pub checkpoint_ref: String,
    /// Whether a checkpoint is present.
    pub checkpoint_present: bool,
    /// The reversal-class token named by the receipt.
    pub reversal_class: String,
    /// Whether the outcome needs compensation or manual follow-up before it is finished.
    pub requires_follow_up: bool,
    /// Whether the compensation / manual follow-up state is named.
    pub follow_up_stated: bool,
    /// Guardrail (MUST be `false` on a clean receipt): distinct outcomes collapse into a generic
    /// success.
    pub collapses_outcome_into_generic_success: bool,
    /// Guardrail (MUST be `false` on a clean receipt): a partial success reads as complete.
    pub presents_partial_as_complete: bool,
    /// Guardrail (MUST be `false` on a clean receipt): a required follow-up is hidden.
    pub hides_follow_up: bool,
    /// Whether a command-backed support-export path is reachable.
    pub support_export_available: bool,
    /// Degrade reason, if the receipt could not read as a clean, attributable outcome.
    pub degrade_reason: Option<M5RepairResultReceiptRowDegradeReason>,
    /// Next safe review action offered.
    pub next_action: M5RepairReceiptReviewAction,
    /// Whether the outcome stays attributable and exportable (clean receipt).
    pub outcome_attributable: bool,
}

impl M5ResolvedRepairResultReceiptRow {
    /// Whether this receipt reads as a clean, attributable outcome.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_checkpoint_lineage_disclosure`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CheckpointLineageDisclosureResolutionInput {
    /// Stable identity of the disclosure instance.
    pub disclosure_id: String,
    /// The stable repair id the lineage traces; empty means unstated.
    pub repair_id: String,
    /// The finding ids the lineage starts from.
    pub linked_finding_ids: Vec<String>,
    /// The preview-stage ref; empty means unstated.
    pub preview_ref: String,
    /// The linked checkpoint ref; empty means unstated.
    pub checkpoint_ref: String,
    /// The checkpoint state at apply.
    pub checkpoint_state: M5RepairCheckpointState,
    /// The apply-stage ref; empty means unstated.
    pub apply_ref: String,
    /// The result-stage (canonical receipt) ref; empty means unstated.
    pub receipt_ref: String,
    /// The applied result of the repair transaction the lineage reached.
    pub outcome_class: M5RepairOutcomeClass,
    /// The reversal class the applied outcome carries.
    pub reversal_class: M5RepairReversalClass,
    /// True when the lineage collapses the four stages into a single opaque status.
    pub reads_as_single_status: bool,
    /// True when a command-backed support-export / lineage path is reachable, never docs-only.
    pub support_export_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe checkpoint-lineage disclosure projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCheckpointLineageDisclosure {
    /// Stable identity of the disclosure instance.
    pub disclosure_id: String,
    /// The stable repair id named by the lineage.
    pub repair_id: String,
    /// The linked finding ids the lineage starts from.
    pub linked_finding_ids: Vec<String>,
    /// The preview-stage ref named by the lineage.
    pub preview_ref: String,
    /// The linked checkpoint ref named by the lineage.
    pub checkpoint_ref: String,
    /// The checkpoint-state token named by the lineage.
    pub checkpoint_state: String,
    /// The apply-stage ref named by the lineage.
    pub apply_ref: String,
    /// The result-stage (canonical receipt) ref named by the lineage.
    pub receipt_ref: String,
    /// The repair-outcome token the lineage reached.
    pub outcome_class: String,
    /// The reversal-class token the lineage carries.
    pub reversal_class: String,
    /// Whether every stage — finding, preview, checkpoint, apply, and result — is present.
    pub lineage_complete: bool,
    /// Guardrail (MUST be `false` on a clean lineage): the four stages collapse into a single status.
    pub collapses_stages_into_single_status: bool,
    /// Guardrail (MUST be `false` on a clean lineage): the lineage is severed from finding or result.
    pub severs_lineage: bool,
    /// Whether a command-backed support-export / lineage path is reachable.
    pub support_export_available: bool,
    /// Degrade reason, if the lineage could not read as a clean, traceable trace.
    pub degrade_reason: Option<M5CheckpointLineageDisclosureDegradeReason>,
    /// Next safe review action offered.
    pub next_action: M5RepairReceiptReviewAction,
    /// Whether the repair traces from finding to result (clean lineage).
    pub lineage_traceable: bool,
}

impl M5ResolvedCheckpointLineageDisclosure {
    /// Whether this lineage reads as a clean, traceable trace.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RepairReceiptLineageResolutionError {
    /// The receipt id was empty.
    EmptyReceiptId,
    /// The disclosure id was empty.
    EmptyDisclosureId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RepairReceiptLineageResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyReceiptId => "empty_receipt_id",
            Self::EmptyDisclosureId => "empty_disclosure_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RepairReceiptLineageResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 repair-receipt-lineage resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RepairReceiptLineageResolutionError {}

/// Resolves a repair-result receipt row, preserving what actually happened: the receipt names its
/// repair id, linked finding ids, outcome class, applied-versus-skipped scope, checkpoint ref,
/// reversal class, and compensation / manual follow-up state, keeps a command-backed support-export
/// path reachable, never collapses distinct outcomes into a generic success, and never presents a
/// partial success as complete.
pub fn resolve_repair_result_receipt_row(
    input: M5RepairResultReceiptRowResolutionInput,
) -> Result<M5ResolvedRepairResultReceiptRow, M5RepairReceiptLineageResolutionError> {
    if input.receipt_id.trim().is_empty() {
        return Err(M5RepairReceiptLineageResolutionError::EmptyReceiptId);
    }
    if string_is_forbidden(&input.receipt_id)
        || string_is_forbidden(&input.repair_id)
        || string_is_forbidden(&input.checkpoint_ref)
        || input
            .linked_finding_ids
            .iter()
            .any(|s| string_is_forbidden(s))
        || input.applied_scope.iter().any(|s| string_is_forbidden(s))
        || input.skipped_scope.iter().any(|s| string_is_forbidden(s))
    {
        return Err(M5RepairReceiptLineageResolutionError::ForbiddenMaterial);
    }

    let checkpoint_present = checkpoint_is_present(input.checkpoint_state);
    let is_partial_success = outcome_is_partial_success(input.outcome_class);
    let is_failure = outcome_is_failure(input.outcome_class);
    let requires_follow_up = outcome_requires_follow_up(input.outcome_class);
    let hides_follow_up = requires_follow_up && !input.follow_up_stated;
    let presents_partial_as_complete = is_partial_success && input.reads_as_complete;

    let degrade_reason = if input.repair_id.trim().is_empty() {
        Some(M5RepairResultReceiptRowDegradeReason::RepairIdUnstated)
    } else if input.linked_finding_ids.is_empty() {
        Some(M5RepairResultReceiptRowDegradeReason::LinkedFindingsUnstated)
    } else if !is_failure && input.applied_scope.is_empty() {
        Some(M5RepairResultReceiptRowDegradeReason::AppliedScopeUnstated)
    } else if matches!(
        input.checkpoint_state,
        M5RepairCheckpointState::CheckpointUnknown
    ) {
        Some(M5RepairResultReceiptRowDegradeReason::CheckpointStateUnresolved)
    } else if checkpoint_present && input.checkpoint_ref.trim().is_empty() {
        Some(M5RepairResultReceiptRowDegradeReason::CheckpointRefUnstated)
    } else if matches!(input.reversal_class, M5RepairReversalClass::ReversalUnknown) {
        Some(M5RepairResultReceiptRowDegradeReason::ReversalClassUnresolved)
    } else if hides_follow_up {
        Some(M5RepairResultReceiptRowDegradeReason::FollowUpStateUnstated)
    } else if presents_partial_as_complete {
        Some(M5RepairResultReceiptRowDegradeReason::PartialSuccessShownAsComplete)
    } else if input.reads_as_generic_success {
        Some(M5RepairResultReceiptRowDegradeReason::OutcomeCollapsedIntoGenericSuccess)
    } else if !input.support_export_available {
        Some(M5RepairResultReceiptRowDegradeReason::SupportExportPathMissing)
    } else if !input.proof_fresh {
        Some(M5RepairResultReceiptRowDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RepairReceiptReviewAction::ReviewReceipt,
    };

    Ok(M5ResolvedRepairResultReceiptRow {
        receipt_id: input.receipt_id,
        repair_id: input.repair_id,
        linked_finding_ids: input.linked_finding_ids,
        outcome_class: input.outcome_class.as_str().to_owned(),
        repair_disposition: disposition_for_outcome(input.outcome_class),
        applied_scope: input.applied_scope,
        skipped_scope: input.skipped_scope,
        is_partial_success,
        checkpoint_state: input.checkpoint_state.as_str().to_owned(),
        checkpoint_ref: input.checkpoint_ref,
        checkpoint_present,
        reversal_class: input.reversal_class.as_str().to_owned(),
        requires_follow_up,
        follow_up_stated: input.follow_up_stated,
        collapses_outcome_into_generic_success: input.reads_as_generic_success,
        presents_partial_as_complete,
        hides_follow_up,
        support_export_available: input.support_export_available,
        degrade_reason,
        next_action,
        outcome_attributable: degrade_reason.is_none(),
    })
}

/// Resolves a checkpoint-lineage disclosure, keeping a repair traceable end to end: the lineage names
/// its repair id, finding ids, preview ref, checkpoint ref, apply ref, and result (canonical receipt)
/// ref, keeps a command-backed support-export path reachable, and never collapses the four stages
/// into a single opaque status.
pub fn resolve_checkpoint_lineage_disclosure(
    input: M5CheckpointLineageDisclosureResolutionInput,
) -> Result<M5ResolvedCheckpointLineageDisclosure, M5RepairReceiptLineageResolutionError> {
    if input.disclosure_id.trim().is_empty() {
        return Err(M5RepairReceiptLineageResolutionError::EmptyDisclosureId);
    }
    if string_is_forbidden(&input.disclosure_id)
        || string_is_forbidden(&input.repair_id)
        || string_is_forbidden(&input.preview_ref)
        || string_is_forbidden(&input.checkpoint_ref)
        || string_is_forbidden(&input.apply_ref)
        || string_is_forbidden(&input.receipt_ref)
        || input
            .linked_finding_ids
            .iter()
            .any(|s| string_is_forbidden(s))
    {
        return Err(M5RepairReceiptLineageResolutionError::ForbiddenMaterial);
    }

    let checkpoint_resolved = !matches!(
        input.checkpoint_state,
        M5RepairCheckpointState::CheckpointUnknown
    );
    let stages_present = !input.repair_id.trim().is_empty()
        && !input.linked_finding_ids.is_empty()
        && !input.preview_ref.trim().is_empty()
        && checkpoint_resolved
        && !input.apply_ref.trim().is_empty()
        && !input.receipt_ref.trim().is_empty();
    let lineage_complete = stages_present && !input.reads_as_single_status;
    let severs_lineage = input.reads_as_single_status || !stages_present;

    let degrade_reason = if input.repair_id.trim().is_empty() {
        Some(M5CheckpointLineageDisclosureDegradeReason::RepairIdUnstated)
    } else if input.linked_finding_ids.is_empty() {
        Some(M5CheckpointLineageDisclosureDegradeReason::FindingLinkUnstated)
    } else if input.preview_ref.trim().is_empty() {
        Some(M5CheckpointLineageDisclosureDegradeReason::PreviewRefUnstated)
    } else if !checkpoint_resolved {
        Some(M5CheckpointLineageDisclosureDegradeReason::CheckpointStateUnresolved)
    } else if input.apply_ref.trim().is_empty() {
        Some(M5CheckpointLineageDisclosureDegradeReason::ApplyRefUnstated)
    } else if input.receipt_ref.trim().is_empty() {
        Some(M5CheckpointLineageDisclosureDegradeReason::ResultRefUnstated)
    } else if input.reads_as_single_status {
        Some(M5CheckpointLineageDisclosureDegradeReason::StagesCollapsedIntoSingleStatus)
    } else if !input.support_export_available {
        Some(M5CheckpointLineageDisclosureDegradeReason::LineagePathMissing)
    } else if !input.proof_fresh {
        Some(M5CheckpointLineageDisclosureDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RepairReceiptReviewAction::InspectCheckpointLineage,
    };

    Ok(M5ResolvedCheckpointLineageDisclosure {
        disclosure_id: input.disclosure_id,
        repair_id: input.repair_id,
        linked_finding_ids: input.linked_finding_ids,
        preview_ref: input.preview_ref,
        checkpoint_ref: input.checkpoint_ref,
        checkpoint_state: input.checkpoint_state.as_str().to_owned(),
        apply_ref: input.apply_ref,
        receipt_ref: input.receipt_ref,
        outcome_class: input.outcome_class.as_str().to_owned(),
        reversal_class: input.reversal_class.as_str().to_owned(),
        lineage_complete,
        collapses_stages_into_single_status: input.reads_as_single_status,
        severs_lineage,
        support_export_available: input.support_export_available,
        degrade_reason,
        next_action,
        lineage_traceable: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved repair-result receipt row and
/// checkpoint-lineage disclosure examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReceiptLineageControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5RepairReceiptLineageConsumerSurface,
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
    pub anatomy_parts: Vec<M5RepairReceiptLineageAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RepairReceiptLineageExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    /// Resolved repair-result receipt row examples.
    pub repair_result_receipt_row_examples: Vec<M5ResolvedRepairResultReceiptRow>,
    /// Resolved checkpoint-lineage disclosure examples.
    pub checkpoint_lineage_disclosure_examples: Vec<M5ResolvedCheckpointLineageDisclosure>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the component schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapse distinct repair outcomes into a generic success.
    pub collapses_outcomes_into_generic_success: bool,
    /// Hard invariant: never hide partial success or the compensation / manual follow-up it needs.
    pub hides_partial_success_or_follow_up: bool,
    /// Hard invariant: never sever a receipt from its checkpoint lineage.
    pub severs_receipt_from_checkpoint_lineage: bool,
    /// Hard invariant: never require feature-local translation to join a support-export packet.
    pub requires_feature_local_translation_for_support_export: bool,
}

impl M5RepairReceiptLineageControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RepairReceiptLineageAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RepairReceiptLineageAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RepairReceiptLineageExportField> =
            self.export_fields.iter().copied().collect();
        M5RepairReceiptLineageExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.collapses_outcomes_into_generic_success
            && !self.hides_partial_success_or_follow_up
            && !self.severs_receipt_from_checkpoint_lineage
            && !self.requires_feature_local_translation_for_support_export
    }

    /// True when every resolved example on this row is honest: no clean receipt collapses its outcome
    /// into a generic success, presents a partial success as complete, hides a required follow-up, or
    /// loses its command-backed support-export path; and no clean lineage collapses its stages, severs
    /// the trace, or loses its command-backed path.
    fn examples_are_honest(&self) -> bool {
        self.repair_result_receipt_row_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_outcome_into_generic_success
                    || ex.presents_partial_as_complete
                    || ex.hides_follow_up
                    || !ex.support_export_available))
        }) && self
            .checkpoint_lineage_disclosure_examples
            .iter()
            .all(|ex| {
                !(ex.is_clean()
                    && (ex.collapses_stages_into_single_status
                        || ex.severs_lineage
                        || !ex.support_export_available))
            })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReceiptLineageVocabularySet {
    /// Repair-disposition tokens (bound from the frozen matrix).
    pub repair_dispositions: Vec<String>,
    /// Repair-outcome tokens (bound from the frozen matrix).
    pub repair_outcomes: Vec<String>,
    /// Reversal-class tokens (bound from the frozen matrix).
    pub reversal_classes: Vec<String>,
    /// Checkpoint-state tokens (bound from the frozen matrix).
    pub checkpoint_states: Vec<String>,
    /// Review-action tokens (minted by this lane).
    pub review_actions: Vec<String>,
    /// Receipt degrade-reason tokens.
    pub receipt_degrade_reasons: Vec<String>,
    /// Lineage degrade-reason tokens.
    pub lineage_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5RepairReceiptLineageVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            repair_dispositions: tokens(&M5WorkspaceTrustRepairDisposition::ALL, |v| v.as_str()),
            repair_outcomes: tokens(&M5RepairOutcomeClass::ALL, |v| v.as_str()),
            reversal_classes: tokens(&M5RepairReversalClass::ALL, |v| v.as_str()),
            checkpoint_states: tokens(&M5RepairCheckpointState::ALL, |v| v.as_str()),
            review_actions: tokens(&M5RepairReceiptReviewAction::ALL, |v| v.as_str()),
            receipt_degrade_reasons: tokens(&M5RepairResultReceiptRowDegradeReason::ALL, |v| {
                v.as_str()
            }),
            lineage_degrade_reasons: tokens(
                &M5CheckpointLineageDisclosureDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5RepairReceiptLineageAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RepairReceiptLineageExportField::ALL, |v| v.as_str()),
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
pub struct M5RepairReceiptLineageGovernanceReview {
    /// Every receipt names its repair id and at least one linked finding id.
    pub receipt_names_repair_id_and_linked_findings: bool,
    /// Every receipt shows its applied-versus-skipped scope.
    pub receipt_shows_applied_versus_skipped_scope: bool,
    /// Partial success and compensation / manual follow-up are visible first-class outcomes.
    pub partial_success_and_follow_up_visible_first_class: bool,
    /// Distinct outcomes are never collapsed into a generic success.
    pub outcome_never_collapsed_into_generic_success: bool,
    /// Checkpoint lineage is traceable from finding to preview to apply to result.
    pub checkpoint_lineage_traceable_finding_to_result: bool,
    /// A receipt joins a support-export / escalation packet without feature-local translation.
    pub receipt_joins_support_export_without_local_translation: bool,
    /// Guided-repair surfaces share one receipt and lineage vocabulary.
    pub receipt_vocabulary_shared_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReceiptLineageConsumerProjection {
    /// Repair receipts across consumers expose the same outcome grammar.
    pub receipts_expose_same_outcome_grammar: bool,
    /// Partial success is legible without hunting through logs.
    pub partial_success_legible_without_logs: bool,
    /// Repair result traces back to one canonical component contract.
    pub repair_result_traces_to_single_component_contract: bool,
    /// Support / export reads a single canonical receipt / lineage source.
    pub support_export_reads_single_receipt_lineage_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReceiptLineageProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReceiptLineageReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RepairReceiptLineageControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RepairReceiptLineageControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5RepairReceiptLineageControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepairReceiptLineageVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepairReceiptLineageGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepairReceiptLineageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepairReceiptLineageProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepairReceiptLineageReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 repair-result-receipt-row / checkpoint-lineage-disclosure controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepairReceiptLineageControlsPacket {
    /// Record kind; must equal [`M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5RepairReceiptLineageControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RepairReceiptLineageVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RepairReceiptLineageGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RepairReceiptLineageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RepairReceiptLineageProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RepairReceiptLineageReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RepairReceiptLineageControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5RepairReceiptLineageControlsPacketInput) -> Self {
        Self {
            record_kind: M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5RepairReceiptLineageControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_RECORD_KIND {
            violations.push(M5RepairReceiptLineageControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5RepairReceiptLineageControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RepairReceiptLineageControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5RepairReceiptLineageControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 repair-receipt-lineage controls packet serializes"),
        ) {
            violations.push(M5RepairReceiptLineageControlsViolation::RawMaterialInExport);
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
            .expect("m5 repair-receipt-lineage controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,receipt_examples,lineage_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .repair_result_receipt_row_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.checkpoint_lineage_disclosure_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.repair_result_receipt_row_examples.len(),
                row.checkpoint_lineage_disclosure_examples.len(),
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
            "# M5 Repair-Result-Receipt-Row and Checkpoint-Lineage-Disclosure Controls\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Repair outcomes: {}\n",
            self.vocabulary_set.repair_outcomes.join(", ")
        ));
        out.push_str(&format!(
            "- Checkpoint states: {}\n",
            self.vocabulary_set.checkpoint_states.join(", ")
        ));
        out.push_str(&format!(
            "- Reversal classes: {}\n",
            self.vocabulary_set.reversal_classes.join(", ")
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
                "  - Receipt examples: {} / lineage examples: {}\n",
                row.repair_result_receipt_row_examples.len(),
                row.checkpoint_lineage_disclosure_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5RepairReceiptLineageControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RepairReceiptLineageControlsViolation>),
}

impl fmt::Display for M5RepairReceiptLineageControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 repair-receipt-lineage controls export parse failed: {error}"
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
                    "m5 repair-receipt-lineage controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RepairReceiptLineageControlsArtifactError {}

/// Validation failures emitted by [`M5RepairReceiptLineageControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RepairReceiptLineageControlsViolation {
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
    /// A controls row does not point at the repair-result-receipt-row component schema.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (collapsed outcome, hidden follow-up, severed
    /// lineage, or lost support-export path).
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
    /// Outcome attributability is not proven: clean receipts do not cover the distinct outcome
    /// classes, or no outcome-collapse / partial-as-complete example degrades, or a clean receipt is
    /// dishonest, or a clean receipt fails to name its repair id / finding / applied scope.
    OutcomeAttributabilityNotProven,
    /// Lineage traceability is not proven: clean lineages are not complete or do not cover a partial
    /// success, or no stage-collapse / missing-stage example degrades, or a clean lineage severs.
    LineageTraceabilityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RepairReceiptLineageControlsViolation {
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
            Self::OutcomeAttributabilityNotProven => "outcome_attributability_not_proven",
            Self::LineageTraceabilityNotProven => "lineage_traceability_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_repair_receipt_lineage_controls_export(
) -> Result<M5RepairReceiptLineageControlsPacket, M5RepairReceiptLineageControlsArtifactError> {
    let packet: M5RepairReceiptLineageControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls-proof/support_export.json"
    )))
    .map_err(M5RepairReceiptLineageControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RepairReceiptLineageControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_SCHEMA_REF,
        M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RepairReceiptLineageControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5RepairReceiptLineageControlsViolation::NoControlsRows);
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
            violations.push(M5RepairReceiptLineageControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5RepairReceiptLineageControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RepairReceiptLineageControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF) {
            violations.push(M5RepairReceiptLineageControlsViolation::ComponentSchemaRefMissing);
        }
        if row.repair_result_receipt_row_examples.is_empty()
            || row.checkpoint_lineage_disclosure_examples.is_empty()
        {
            violations.push(M5RepairReceiptLineageControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5RepairReceiptLineageControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5RepairReceiptLineageControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.receipt_names_repair_id_and_linked_findings,
        review.receipt_shows_applied_versus_skipped_scope,
        review.partial_success_and_follow_up_visible_first_class,
        review.outcome_never_collapsed_into_generic_success,
        review.checkpoint_lineage_traceable_finding_to_result,
        review.receipt_joins_support_export_without_local_translation,
        review.receipt_vocabulary_shared_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5RepairReceiptLineageControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.receipts_expose_same_outcome_grammar,
        projection.partial_success_legible_without_logs,
        projection.repair_result_traces_to_single_component_contract,
        projection.support_export_reads_single_receipt_lineage_source,
    ] {
        if !ok {
            violations.push(M5RepairReceiptLineageControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RepairReceiptLineageControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RepairReceiptLineageControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5RepairReceiptLineageControlsPacket,
    violations: &mut Vec<M5RepairReceiptLineageControlsViolation>,
) {
    let receipts = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.repair_result_receipt_row_examples.iter())
    };
    let lineages = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.checkpoint_lineage_disclosure_examples.iter())
    };

    // AC: repair outcomes stay attributable and exportable, and partial success plus manual follow-up
    // are visible first-class outcomes. Clean receipts cover an exact success, a partial success, and
    // a manual-required outcome so each keeps its own honest word; at least one receipt degrades to an
    // outcome collapsed into a generic success and one to a partial success shown as complete; no clean
    // receipt is dishonest; and every clean receipt names its repair id, at least one finding, applied
    // scope (for any non-failure outcome), and keeps a command-backed support-export path.
    let clean_outcomes: BTreeSet<&str> = receipts()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.outcome_class.as_str())
        .collect();
    let covers_outcomes = [
        M5RepairOutcomeClass::RepairAppliedExact,
        M5RepairOutcomeClass::RepairPartialSuccess,
        M5RepairOutcomeClass::RepairManualRequired,
    ]
    .iter()
    .all(|outcome| clean_outcomes.contains(outcome.as_str()));
    let generic_collapse_degrades = receipts().any(|ex| {
        ex.degrade_reason
            == Some(M5RepairResultReceiptRowDegradeReason::OutcomeCollapsedIntoGenericSuccess)
            && ex.collapses_outcome_into_generic_success
    });
    let partial_as_complete_degrades = receipts().any(|ex| {
        ex.degrade_reason
            == Some(M5RepairResultReceiptRowDegradeReason::PartialSuccessShownAsComplete)
            && ex.presents_partial_as_complete
    });
    let no_clean_dishonest_receipt = receipts().all(|ex| {
        !(ex.is_clean()
            && (ex.collapses_outcome_into_generic_success
                || ex.presents_partial_as_complete
                || ex.hides_follow_up))
    });
    let clean_receipts_named = receipts().filter(|ex| ex.is_clean()).all(|ex| {
        !ex.repair_id.trim().is_empty()
            && !ex.linked_finding_ids.is_empty()
            && (ex.outcome_class == M5RepairOutcomeClass::RepairFailed.as_str()
                || !ex.applied_scope.is_empty())
            && ex.support_export_available
    });
    if !(covers_outcomes
        && generic_collapse_degrades
        && partial_as_complete_degrades
        && no_clean_dishonest_receipt
        && clean_receipts_named)
    {
        violations.push(M5RepairReceiptLineageControlsViolation::OutcomeAttributabilityNotProven);
    }

    // AC: support packets can cite one canonical receipt / lineage object for guided repairs. Clean
    // lineages are complete (finding to preview to apply to result) and cover a partial success; at
    // least one lineage degrades to a collapsed single status and one to a missing stage; no clean
    // lineage severs; and every clean lineage links at least one finding and names a canonical receipt
    // (result) ref.
    let clean_lineage_outcomes: BTreeSet<&str> = lineages()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.outcome_class.as_str())
        .collect();
    let covers_partial_lineage =
        clean_lineage_outcomes.contains(M5RepairOutcomeClass::RepairPartialSuccess.as_str());
    let stage_collapse_degrades = lineages().any(|ex| {
        ex.degrade_reason
            == Some(M5CheckpointLineageDisclosureDegradeReason::StagesCollapsedIntoSingleStatus)
            && ex.collapses_stages_into_single_status
    });
    let missing_stage_degrades = lineages().any(|ex| {
        matches!(
            ex.degrade_reason,
            Some(M5CheckpointLineageDisclosureDegradeReason::FindingLinkUnstated)
                | Some(M5CheckpointLineageDisclosureDegradeReason::PreviewRefUnstated)
                | Some(M5CheckpointLineageDisclosureDegradeReason::ApplyRefUnstated)
                | Some(M5CheckpointLineageDisclosureDegradeReason::ResultRefUnstated)
        )
    });
    let no_clean_severed_lineage = lineages().all(|ex| {
        !(ex.is_clean()
            && (ex.severs_lineage
                || ex.collapses_stages_into_single_status
                || !ex.lineage_complete))
    });
    let clean_lineage_named = lineages().filter(|ex| ex.is_clean()).all(|ex| {
        !ex.linked_finding_ids.is_empty()
            && !ex.receipt_ref.trim().is_empty()
            && ex.support_export_available
    });
    if !(covers_partial_lineage
        && stage_collapse_degrades
        && missing_stage_degrades
        && no_clean_severed_lineage
        && clean_lineage_named)
    {
        violations.push(M5RepairReceiptLineageControlsViolation::LineageTraceabilityNotProven);
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

/// The component family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5WorkspaceTrustRepairComponentFamily; 1] =
    [M5WorkspaceTrustRepairComponentFamily::RepairResultReceiptRow];
