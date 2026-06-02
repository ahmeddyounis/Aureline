//! Finalized typed repair-transaction preview, checkpoint, rollback, and
//! compensation flows for the M4 stable lane.
//!
//! This module promotes the alpha repair-transaction compiler, the beta
//! preview-skeleton evaluator, and the recovery-ladder alpha decision path into
//! a single stable contract. It guarantees that every blocked-user recovery
//! flow is:
//!
//! - **Typed** — every preview, checkpoint, rollback, and compensation step
//!   carries a closed-vocabulary class instead of free-form text.
//! - **Truthful** — the evaluator rejects flows that claim exact rollback
//!   without a durable checkpoint, or that claim no blast radius while naming
//!   impacted state classes.
//! - **Narrow** — the blast-radius vocabulary is capped at single-object-class,
//!   same-family, cross-family, or escalation-only; no flow may silently widen
//!   to a destructive reset.
//! - **Versioned** — schema version 1 is frozen; later changes must bump the
//!   version and provide a migration note.
//! - **Export-safe** — the support packet is metadata-only by default; raw
//!   private material and ambient authority are excluded.
//!
//! The module owns four typed records:
//!
//! - [`FinalizedRepairPreviewRecord`] — one row per repair preview that
//!   declares the preview disposition, blast-radius class, confirmation
//!   requirement, and Project Doctor finding lineage.
//! - [`FinalizedRepairCheckpointRecord`] — one row per checkpoint that
//!   declares the checkpoint class (durable, ephemeral, none, refused), the
//!   scoped state classes, and the capture summary.
//! - [`FinalizedRepairRollbackRecord`] — one row per rollback or reversal
//!   that declares the rollback class (exact restore, compensating restore,
//!   regenerate, manual, none), the checkpoint ref consumed, and the outcome.
//! - [`FinalizedRepairCompensationRecord`] — one row per compensation that
//!   declares the compensation class, the reviewer acknowledgement required,
//!   and the follow-up action.
//!
//! The [`FinalizedRepairTransactionFlow`] joins one preview, one checkpoint,
//! one rollback, and one compensation record into a single bounded flow bound
//! to a stable repair transaction id and a recovery-ladder rung ref.
//!
//! The [`FinalizedRepairFlowEvaluator`] validates:
//!
//! - every flow preserves `user_authored_files`,
//! - every flow cites a `doctor.finding.*` ref,
//! - checkpoint class and rollback class are consistent (for example, exact
//!   rollback requires a durable or ephemeral checkpoint),
//! - compensation class matches the previewed reversal class,
//! - preview disposition is one of the closed vocabulary,
//! - no flow declares a destructive reset,
//! - seeded support scenarios cover every required repair-class family.
//!
//! The [`FinalizedRepairFlowSupportPacket`] folds validated flows into a
//! metadata-safe projection that support-export and release-evidence pipelines
//! can consume verbatim.
//!
//! The boundary schema is at
//! `/schemas/support/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.schema.json`.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::recovery_ladder::{RecoveryLadderDecision, RecoveryRungClass};
use crate::repair::{
    ApplyModeClass, ConfirmationClass, ImpactedStateClass, PreservedStateClass, RepairClassFamily,
    RepairTransactionRecord, TransactionReversalClass,
};
use crate::repair_transactions::{
    RepairBlastRadiusClass, RepairCheckpointDispositionClass, RepairCompensationClass,
    RepairPreviewDispositionClass, RepairPreviewSkeleton,
};

/// Stable record-kind tag for a finalized repair-transaction flow.
pub const FINALIZED_REPAIR_FLOW_RECORD_KIND: &str =
    "finalized_repair_transaction_preview_checkpoint_rollback_and_flow_record";

/// Stable record-kind tag for the finalized repair-flow support packet.
pub const FINALIZED_REPAIR_FLOW_SUPPORT_PACKET_RECORD_KIND: &str =
    "finalized_repair_transaction_preview_checkpoint_rollback_and_support_packet_record";

/// Integer schema version for finalized repair-flow records.
pub const FINALIZED_REPAIR_FLOW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const FINALIZED_REPAIR_FLOW_SCHEMA_REF: &str =
    "schemas/support/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const FINALIZED_REPAIR_FLOW_DOC_REF: &str =
    "docs/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const FINALIZED_REPAIR_FLOW_ARTIFACT_REF: &str =
    "artifacts/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const FINALIZED_REPAIR_FLOW_FIXTURE_DIR: &str =
    "fixtures/support/m4/finalize_typed_repair_transaction_preview_checkpoint_rollback_and";

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// Closed finalized checkpoint class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedCheckpointClass {
    /// A durable on-disk checkpoint captured before apply.
    DurablePreApply,
    /// A session-scoped ephemeral checkpoint captured before apply.
    EphemeralPreApply,
    /// No checkpoint because the repair is observe-only or audit-only.
    NoCheckpointNeeded,
    /// Checkpoint capture was refused because the state class cannot be checkpointed.
    CheckpointRefused,
    /// No checkpoint because the flow routes to escalation only.
    NoCheckpointEscalationOnly,
}

impl FinalizedCheckpointClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurablePreApply => "durable_pre_apply",
            Self::EphemeralPreApply => "ephemeral_pre_apply",
            Self::NoCheckpointNeeded => "no_checkpoint_needed",
            Self::CheckpointRefused => "checkpoint_refused",
            Self::NoCheckpointEscalationOnly => "no_checkpoint_escalation_only",
        }
    }

    /// Returns true when this class requires a non-empty checkpoint ref.
    pub fn requires_checkpoint_ref(self) -> bool {
        matches!(self, Self::DurablePreApply | Self::EphemeralPreApply)
    }

    /// Returns true when this class forbids any apply path.
    pub fn blocks_apply(self) -> bool {
        matches!(
            self,
            Self::CheckpointRefused | Self::NoCheckpointEscalationOnly
        )
    }
}

/// Closed finalized rollback class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedRollbackClass {
    /// Exact bit-for-bit restore from a captured checkpoint.
    ExactRestoreFromCheckpoint,
    /// Compensating semantic restore when exact restore is unavailable.
    CompensatingRestore,
    /// Regenerate disposable derived state from authoritative sources.
    RegenerateFromAuthoritativeSource,
    /// Manual user-guided recovery path.
    ManualRecoveryPath,
    /// No rollback because the transaction was audit-only or refused.
    NoRollbackNotApplicable,
    /// Rollback failed; evidence was exported instead.
    RollbackFailedExportOnly,
}

impl FinalizedRollbackClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestoreFromCheckpoint => "exact_restore_from_checkpoint",
            Self::CompensatingRestore => "compensating_restore",
            Self::RegenerateFromAuthoritativeSource => "regenerate_from_authoritative_source",
            Self::ManualRecoveryPath => "manual_recovery_path",
            Self::NoRollbackNotApplicable => "no_rollback_not_applicable",
            Self::RollbackFailedExportOnly => "rollback_failed_export_only",
        }
    }

    /// Returns true when this rollback class requires a checkpoint ref.
    pub fn requires_checkpoint_ref(self) -> bool {
        matches!(self, Self::ExactRestoreFromCheckpoint)
    }

    /// Returns the matching transaction reversal class, if any.
    pub fn matching_reversal_class(self) -> Option<TransactionReversalClass> {
        match self {
            Self::ExactRestoreFromCheckpoint => Some(TransactionReversalClass::Exact),
            Self::CompensatingRestore => Some(TransactionReversalClass::Compensating),
            Self::RegenerateFromAuthoritativeSource => Some(TransactionReversalClass::Regenerate),
            Self::ManualRecoveryPath => Some(TransactionReversalClass::Manual),
            Self::NoRollbackNotApplicable => Some(TransactionReversalClass::AuditOnly),
            Self::RollbackFailedExportOnly => None,
        }
    }
}

/// Closed finalized compensation class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedCompensationClass {
    /// No compensation needed because the repair only touches disposable state.
    NoCompensationNeeded,
    /// Regenerate disposable derived state from authoritative sources.
    RegenerateFromAuthoritativeSource,
    /// Apply a semantic inverse (for example, release a quarantine).
    SemanticInverseCompensation,
    /// Manual follow-up required (for example, reinstall an extension by hand).
    ManualFollowupRequired,
    /// Audit-only; no state changed.
    AuditOnlyNoStateChange,
    /// Compensation failed; escalation packet prepared.
    CompensationFailedEscalation,
}

impl FinalizedCompensationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCompensationNeeded => "no_compensation_needed",
            Self::RegenerateFromAuthoritativeSource => "regenerate_from_authoritative_source",
            Self::SemanticInverseCompensation => "semantic_inverse_compensation",
            Self::ManualFollowupRequired => "manual_followup_required",
            Self::AuditOnlyNoStateChange => "audit_only_no_state_change",
            Self::CompensationFailedEscalation => "compensation_failed_escalation",
        }
    }

    /// Returns true when this compensation class requires explicit reviewer
    /// acknowledgement beyond a standard review click.
    pub fn requires_strong_acknowledgement(self) -> bool {
        matches!(
            self,
            Self::SemanticInverseCompensation | Self::ManualFollowupRequired
        )
    }

    /// Returns the matching repair-transaction compensation class.
    pub fn to_preview_compensation(self) -> RepairCompensationClass {
        match self {
            Self::NoCompensationNeeded => RepairCompensationClass::NoCompensationNeeded,
            Self::RegenerateFromAuthoritativeSource => {
                RepairCompensationClass::RegenerateFromAuthoritativeSource
            }
            Self::SemanticInverseCompensation => {
                RepairCompensationClass::SemanticInverseCompensation
            }
            Self::ManualFollowupRequired => RepairCompensationClass::ManualFollowupRequired,
            Self::AuditOnlyNoStateChange => RepairCompensationClass::AuditOnlyNoStateChange,
            Self::CompensationFailedEscalation => RepairCompensationClass::ManualFollowupRequired,
        }
    }
}

/// Closed finalized preview-disposition vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedPreviewDispositionClass {
    /// Preview is complete and pending reviewer authorization.
    PendingReview,
    /// Preview was compared against a prior baseline.
    ComparedWithBaseline,
    /// Preview is authorized for apply.
    AuthorizedForApply,
    /// Preview was cancelled before apply.
    CancelledBeforeApply,
    /// Preview is blocked pending more evidence or policy review.
    BlockedPendingEvidence,
    /// Preview refused local apply and routed to escalation.
    RefusedEscalationOnly,
}

impl FinalizedPreviewDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::ComparedWithBaseline => "compared_with_baseline",
            Self::AuthorizedForApply => "authorized_for_apply",
            Self::CancelledBeforeApply => "cancelled_before_apply",
            Self::BlockedPendingEvidence => "blocked_pending_evidence",
            Self::RefusedEscalationOnly => "refused_escalation_only",
        }
    }

    /// Returns true when this disposition authorizes a downstream apply.
    pub fn authorizes_apply(self) -> bool {
        matches!(self, Self::AuthorizedForApply)
    }

    /// Returns true when this disposition forbids any apply.
    pub fn forbids_apply(self) -> bool {
        matches!(
            self,
            Self::CancelledBeforeApply | Self::RefusedEscalationOnly
        )
    }
}

/// Closed finalized repair-flow class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedRepairFlowClass {
    /// Preview → checkpoint → apply with rollback on failure.
    PreviewCheckpointApplyWithRollback,
    /// Preview → checkpoint → apply with compensating reversal on failure.
    PreviewCheckpointApplyWithCompensation,
    /// Preview → no checkpoint → observe-only audit.
    PreviewObserveOnlyAudit,
    /// Preview → refused → escalation packet prepared.
    PreviewRefusedEscalation,
    /// Comparison with baseline before authorization.
    ComparisonThenAuthorize,
}

impl FinalizedRepairFlowClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewCheckpointApplyWithRollback => "preview_checkpoint_apply_with_rollback",
            Self::PreviewCheckpointApplyWithCompensation => {
                "preview_checkpoint_apply_with_compensation"
            }
            Self::PreviewObserveOnlyAudit => "preview_observe_only_audit",
            Self::PreviewRefusedEscalation => "preview_refused_escalation",
            Self::ComparisonThenAuthorize => "comparison_then_authorize",
        }
    }

    /// Returns the default checkpoint class for this flow.
    pub fn default_checkpoint_class(self) -> FinalizedCheckpointClass {
        match self {
            Self::PreviewCheckpointApplyWithRollback => FinalizedCheckpointClass::DurablePreApply,
            Self::PreviewCheckpointApplyWithCompensation => {
                FinalizedCheckpointClass::DurablePreApply
            }
            Self::PreviewObserveOnlyAudit => FinalizedCheckpointClass::NoCheckpointNeeded,
            Self::PreviewRefusedEscalation => FinalizedCheckpointClass::NoCheckpointEscalationOnly,
            Self::ComparisonThenAuthorize => FinalizedCheckpointClass::DurablePreApply,
        }
    }

    /// Returns the default rollback class for this flow.
    pub fn default_rollback_class(self) -> FinalizedRollbackClass {
        match self {
            Self::PreviewCheckpointApplyWithRollback => {
                FinalizedRollbackClass::ExactRestoreFromCheckpoint
            }
            Self::PreviewCheckpointApplyWithCompensation => {
                FinalizedRollbackClass::CompensatingRestore
            }
            Self::PreviewObserveOnlyAudit => FinalizedRollbackClass::NoRollbackNotApplicable,
            Self::PreviewRefusedEscalation => FinalizedRollbackClass::NoRollbackNotApplicable,
            Self::ComparisonThenAuthorize => FinalizedRollbackClass::ExactRestoreFromCheckpoint,
        }
    }
}

/// Closed seeded-scenario vocabulary that every finalized flow must be able to
/// justify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalizedSeededScenarioClass {
    /// Disposable cache or index repair.
    CacheIndexRepair,
    /// Extension quarantine and bisect.
    ExtensionQuarantineBisect,
    /// Toolchain or execution-context re-resolve.
    ToolchainReresolve,
    /// Remote agent or helper rollback.
    RemoteAgentRollback,
    /// Policy or entitlement refresh.
    PolicyEntitlementRefresh,
    /// Trust reacquire without widening.
    TrustReacquire,
    /// Watcher restart with reseed.
    WatcherRestartReseed,
    /// Docs or mirror pack refresh.
    DocsMirrorRefresh,
    /// Escalation-only; no safe local repair.
    EscalationOnlyNoLocalRepair,
    /// Observe-only; no repair available.
    ObserveOnlyNoRepair,
}

impl FinalizedSeededScenarioClass {
    /// Every required seeded scenario, in declaration order.
    pub const REQUIRED: [Self; 10] = [
        Self::CacheIndexRepair,
        Self::ExtensionQuarantineBisect,
        Self::ToolchainReresolve,
        Self::RemoteAgentRollback,
        Self::PolicyEntitlementRefresh,
        Self::TrustReacquire,
        Self::WatcherRestartReseed,
        Self::DocsMirrorRefresh,
        Self::EscalationOnlyNoLocalRepair,
        Self::ObserveOnlyNoRepair,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CacheIndexRepair => "cache_index_repair",
            Self::ExtensionQuarantineBisect => "extension_quarantine_bisect",
            Self::ToolchainReresolve => "toolchain_reresolve",
            Self::RemoteAgentRollback => "remote_agent_rollback",
            Self::PolicyEntitlementRefresh => "policy_entitlement_refresh",
            Self::TrustReacquire => "trust_reacquire",
            Self::WatcherRestartReseed => "watcher_restart_reseed",
            Self::DocsMirrorRefresh => "docs_mirror_refresh",
            Self::EscalationOnlyNoLocalRepair => "escalation_only_no_local_repair",
            Self::ObserveOnlyNoRepair => "observe_only_no_repair",
        }
    }
}

// ---------------------------------------------------------------------------
// Record types
// ---------------------------------------------------------------------------

/// One finalized repair preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairPreviewRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable preview id.
    pub preview_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Preview disposition.
    pub preview_disposition_class: FinalizedPreviewDispositionClass,
    /// Blast-radius class.
    pub blast_radius_class: RepairBlastRadiusClass,
    /// Confirmation requirement.
    pub confirmation_class: ConfirmationClass,
    /// Whether strong confirmation is required.
    pub strong_confirmation_required: bool,
    /// Project Doctor finding that justified the repair.
    pub doctor_finding_ref: String,
    /// Impacted state classes.
    pub impacted_state_classes: Vec<ImpactedStateClass>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Reviewer-facing preview summary.
    pub preview_summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One finalized repair checkpoint record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairCheckpointRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable checkpoint id.
    pub checkpoint_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Checkpoint class.
    pub checkpoint_class: FinalizedCheckpointClass,
    /// Checkpoint ref when available.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checkpoint_ref: Option<String>,
    /// State classes scoped by the checkpoint.
    pub scoped_state_classes: Vec<ImpactedStateClass>,
    /// Reviewer-facing capture summary.
    pub capture_summary: String,
    /// Whether the checkpoint is export-safe (metadata-only).
    pub export_safe: bool,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One finalized repair rollback record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairRollbackRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable rollback id.
    pub rollback_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Rollback class.
    pub rollback_class: FinalizedRollbackClass,
    /// Checkpoint ref consumed by rollback.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checkpoint_ref_consumed: Option<String>,
    /// Whether the rollback succeeded.
    pub rollback_succeeded: bool,
    /// Reviewer-facing rollback summary.
    pub rollback_summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One finalized repair compensation record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairCompensationRecord {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable compensation id.
    pub compensation_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Compensation class.
    pub compensation_class: FinalizedCompensationClass,
    /// Whether strong acknowledgement is required.
    pub strong_acknowledgement_required: bool,
    /// Reviewer-facing compensation summary.
    pub compensation_summary: String,
    /// Follow-up action summary.
    pub follow_up_action_summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One recovery-ladder proof binding carried by a finalized flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderProofBinding {
    /// Recovery ladder rung class bound to this flow.
    pub rung_class: RecoveryRungClass,
    /// Decision id from the recovery ladder.
    pub decision_id: String,
    /// Whether the ladder approved this flow.
    pub ladder_approved: bool,
    /// Reviewer-facing ladder summary.
    pub ladder_summary: String,
}

/// One seeded support scenario carried by a finalized flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededSupportScenario {
    /// Scenario class.
    pub scenario_class: FinalizedSeededScenarioClass,
    /// Whether this scenario is covered by the flow.
    pub covered: bool,
    /// Reviewer-facing coverage note.
    pub coverage_note: String,
}

/// Top-level finalized repair-transaction flow record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairTransactionFlow {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable flow id.
    pub flow_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Repair flow class.
    pub flow_class: FinalizedRepairFlowClass,
    /// Preview record.
    pub preview: FinalizedRepairPreviewRecord,
    /// Checkpoint record.
    pub checkpoint: FinalizedRepairCheckpointRecord,
    /// Rollback record.
    pub rollback: FinalizedRepairRollbackRecord,
    /// Compensation record.
    pub compensation: FinalizedRepairCompensationRecord,
    /// Recovery-ladder proof bindings.
    pub ladder_bindings: Vec<RecoveryLadderProofBinding>,
    /// Seeded support scenarios.
    pub seeded_scenarios: Vec<SeededSupportScenario>,
    /// Whether the flow declares a destructive reset.
    pub destructive_reset_present: bool,
    /// Exact-build identity ref.
    pub exact_build_identity_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One support-packet row for a finalized flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairFlowSupportRow {
    /// Flow id.
    pub flow_id: String,
    /// Repair transaction ref.
    pub repair_transaction_ref: String,
    /// Flow class.
    pub flow_class: FinalizedRepairFlowClass,
    /// Preview disposition.
    pub preview_disposition_class: FinalizedPreviewDispositionClass,
    /// Checkpoint class.
    pub checkpoint_class: FinalizedCheckpointClass,
    /// Rollback class.
    pub rollback_class: FinalizedRollbackClass,
    /// Compensation class.
    pub compensation_class: FinalizedCompensationClass,
    /// Doctor finding ref.
    pub doctor_finding_ref: String,
    /// Blast-radius class.
    pub blast_radius_class: RepairBlastRadiusClass,
    /// Whether strong confirmation is required.
    pub strong_confirmation_required: bool,
    /// Ladder rung refs.
    pub ladder_rung_refs: Vec<String>,
    /// Seeded scenario tokens covered.
    pub seeded_scenarios_covered: Vec<String>,
    /// Whether a destructive reset is present.
    pub destructive_reset_present: bool,
}

/// Metadata-safe support packet for finalized repair flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairFlowSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Support rows.
    pub rows: Vec<FinalizedRepairFlowSupportRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether every row in the packet is export-safe.
    pub all_rows_export_safe: bool,
}

impl FinalizedRepairFlowSupportPacket {
    /// Returns true when the packet is safe for metadata-only export.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.all_rows_export_safe
            && self.rows.iter().all(|row| !row.destructive_reset_present)
    }
}

/// One validation violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairFlowViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedRepairFlowValidationReport {
    /// Validation failures.
    pub violations: Vec<FinalizedRepairFlowViolation>,
}

impl FinalizedRepairFlowValidationReport {
    /// True when no violations were found.
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for FinalizedRepairFlowValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} finalized-repair-flow violation(s)",
            self.violations.len()
        )
    }
}

impl Error for FinalizedRepairFlowValidationReport {}

// ---------------------------------------------------------------------------
// Load helpers
// ---------------------------------------------------------------------------

/// Deserialize a finalized repair-transaction flow from YAML.
pub fn load_finalized_repair_flow(
    yaml: &str,
) -> Result<FinalizedRepairTransactionFlow, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Deserialize a support packet from YAML.
pub fn load_support_packet(
    yaml: &str,
) -> Result<FinalizedRepairFlowSupportPacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

// ---------------------------------------------------------------------------
// Evaluator
// ---------------------------------------------------------------------------

/// Finalized repair-flow evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct FinalizedRepairFlowEvaluator;

impl FinalizedRepairFlowEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a finalized repair-transaction flow.
    pub fn validate_flow(
        &self,
        flow: &FinalizedRepairTransactionFlow,
    ) -> FinalizedRepairFlowValidationReport {
        let mut violations = Vec::new();

        if flow.schema_version != FINALIZED_REPAIR_FLOW_SCHEMA_VERSION {
            push_violation(
                &mut violations,
                "finalized_repair_flow.schema_version",
                &flow.flow_id,
                "flow schema_version must be 1",
            );
        }
        if flow.record_kind != FINALIZED_REPAIR_FLOW_RECORD_KIND {
            push_violation(
                &mut violations,
                "finalized_repair_flow.record_kind",
                &flow.flow_id,
                format!("record_kind must be {FINALIZED_REPAIR_FLOW_RECORD_KIND}"),
            );
        }
        if flow.repair_transaction_ref.trim().is_empty() {
            push_violation(
                &mut violations,
                "finalized_repair_flow.empty_transaction_ref",
                &flow.flow_id,
                "repair_transaction_ref must be non-empty",
            );
        }
        if flow.preview.repair_transaction_ref != flow.repair_transaction_ref {
            push_violation(
                &mut violations,
                "finalized_repair_flow.preview_transaction_mismatch",
                &flow.flow_id,
                "preview repair_transaction_ref must match flow repair_transaction_ref",
            );
        }
        if flow.checkpoint.repair_transaction_ref != flow.repair_transaction_ref {
            push_violation(
                &mut violations,
                "finalized_repair_flow.checkpoint_transaction_mismatch",
                &flow.flow_id,
                "checkpoint repair_transaction_ref must match flow repair_transaction_ref",
            );
        }
        if flow.rollback.repair_transaction_ref != flow.repair_transaction_ref {
            push_violation(
                &mut violations,
                "finalized_repair_flow.rollback_transaction_mismatch",
                &flow.flow_id,
                "rollback repair_transaction_ref must match flow repair_transaction_ref",
            );
        }
        if flow.compensation.repair_transaction_ref != flow.repair_transaction_ref {
            push_violation(
                &mut violations,
                "finalized_repair_flow.compensation_transaction_mismatch",
                &flow.flow_id,
                "compensation repair_transaction_ref must match flow repair_transaction_ref",
            );
        }
        if !flow
            .preview
            .doctor_finding_ref
            .starts_with("doctor.finding.")
        {
            push_violation(
                &mut violations,
                "finalized_repair_flow.doctor_finding_ref_invalid",
                &flow.flow_id,
                "doctor_finding_ref must start with doctor.finding.",
            );
        }
        if !flow
            .preview
            .preserved_state_classes
            .contains(&PreservedStateClass::UserAuthoredFiles)
        {
            push_violation(
                &mut violations,
                "finalized_repair_flow.user_authored_files_not_preserved",
                &flow.flow_id,
                "user_authored_files must be listed in preserved_state_classes",
            );
        }
        if flow.destructive_reset_present {
            push_violation(
                &mut violations,
                "finalized_repair_flow.destructive_reset_present",
                &flow.flow_id,
                "finalized flows must not declare destructive resets",
            );
        }

        // Checkpoint / rollback consistency.
        if flow.checkpoint.checkpoint_class.requires_checkpoint_ref()
            && flow.checkpoint.checkpoint_ref.is_none()
        {
            push_violation(
                &mut violations,
                "finalized_repair_flow.checkpoint_ref_missing",
                &flow.flow_id,
                "checkpoint class requires a checkpoint ref",
            );
        }
        if flow.rollback.rollback_class.requires_checkpoint_ref()
            && flow.rollback.checkpoint_ref_consumed.is_none()
        {
            push_violation(
                &mut violations,
                "finalized_repair_flow.rollback_checkpoint_ref_missing",
                &flow.flow_id,
                "rollback class requires a consumed checkpoint ref",
            );
        }
        if let Some(expected_reversal) = flow.rollback.rollback_class.matching_reversal_class() {
            if flow
                .compensation
                .compensation_class
                .to_preview_compensation()
                != compensation_from_reversal(expected_reversal)
            {
                push_violation(
                    &mut violations,
                    "finalized_repair_flow.compensation_rollback_mismatch",
                    &flow.flow_id,
                    "compensation class does not match rollback reversal class",
                );
            }
        }

        // Flow-class consistency.
        if flow.flow_class == FinalizedRepairFlowClass::PreviewRefusedEscalation
            && flow.preview.preview_disposition_class
                != FinalizedPreviewDispositionClass::RefusedEscalationOnly
        {
            push_violation(
                &mut violations,
                "finalized_repair_flow.disposition_refused_mismatch",
                &flow.flow_id,
                "PreviewRefusedEscalation flow requires RefusedEscalationOnly disposition",
            );
        }
        if flow.flow_class == FinalizedRepairFlowClass::PreviewObserveOnlyAudit
            && flow.checkpoint.checkpoint_class != FinalizedCheckpointClass::NoCheckpointNeeded
        {
            push_violation(
                &mut violations,
                "finalized_repair_flow.observe_only_checkpoint_mismatch",
                &flow.flow_id,
                "PreviewObserveOnlyAudit flow requires NoCheckpointNeeded checkpoint class",
            );
        }

        // Seeded-scenario coverage.
        let covered_scenarios: BTreeSet<FinalizedSeededScenarioClass> = flow
            .seeded_scenarios
            .iter()
            .filter(|s| s.covered)
            .map(|s| s.scenario_class)
            .collect();
        for required in FinalizedSeededScenarioClass::REQUIRED {
            if !covered_scenarios.contains(&required) {
                push_violation(
                    &mut violations,
                    "finalized_repair_flow.missing_seeded_scenario",
                    &flow.flow_id,
                    format!("seeded scenario {} must be covered", required.as_str()),
                );
            }
        }

        // Ladder bindings must not be empty.
        if flow.ladder_bindings.is_empty() {
            push_violation(
                &mut violations,
                "finalized_repair_flow.ladder_bindings_empty",
                &flow.flow_id,
                "at least one recovery-ladder proof binding is required",
            );
        }

        // Exact-build identity ref must be non-empty.
        if flow.exact_build_identity_ref.trim().is_empty() {
            push_violation(
                &mut violations,
                "finalized_repair_flow.empty_build_identity",
                &flow.flow_id,
                "exact_build_identity_ref must be non-empty",
            );
        }

        FinalizedRepairFlowValidationReport { violations }
    }

    /// Builds a metadata-safe support packet from validated flows.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        flows: &[FinalizedRepairTransactionFlow],
    ) -> Result<FinalizedRepairFlowSupportPacket, FinalizedRepairFlowValidationReport> {
        let mut all_violations = Vec::new();
        let mut rows = Vec::new();

        for flow in flows {
            let report = self.validate_flow(flow);
            all_violations.extend(report.violations);

            rows.push(FinalizedRepairFlowSupportRow {
                flow_id: flow.flow_id.clone(),
                repair_transaction_ref: flow.repair_transaction_ref.clone(),
                flow_class: flow.flow_class,
                preview_disposition_class: flow.preview.preview_disposition_class,
                checkpoint_class: flow.checkpoint.checkpoint_class,
                rollback_class: flow.rollback.rollback_class,
                compensation_class: flow.compensation.compensation_class,
                doctor_finding_ref: flow.preview.doctor_finding_ref.clone(),
                blast_radius_class: flow.preview.blast_radius_class,
                strong_confirmation_required: flow.preview.strong_confirmation_required,
                ladder_rung_refs: flow
                    .ladder_bindings
                    .iter()
                    .map(|b| b.decision_id.clone())
                    .collect(),
                seeded_scenarios_covered: flow
                    .seeded_scenarios
                    .iter()
                    .filter(|s| s.covered)
                    .map(|s| s.scenario_class.as_str().to_owned())
                    .collect(),
                destructive_reset_present: flow.destructive_reset_present,
            });
        }

        if !all_violations.is_empty() {
            return Err(FinalizedRepairFlowValidationReport {
                violations: all_violations,
            });
        }

        Ok(FinalizedRepairFlowSupportPacket {
            record_kind: FINALIZED_REPAIR_FLOW_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: FINALIZED_REPAIR_FLOW_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: FINALIZED_REPAIR_FLOW_DOC_REF.to_owned(),
            schema_ref: FINALIZED_REPAIR_FLOW_SCHEMA_REF.to_owned(),
            rows,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            all_rows_export_safe: true,
        })
    }

    /// Derives a finalized flow from an alpha [`RepairTransactionRecord`] and
    /// its beta [`RepairPreviewSkeleton`].
    pub fn from_alpha_beta(
        &self,
        flow_id: impl Into<String>,
        transaction: &RepairTransactionRecord,
        skeleton: &RepairPreviewSkeleton,
        decision: &RecoveryLadderDecision,
        captured_at: impl Into<String>,
    ) -> FinalizedRepairTransactionFlow {
        let captured_at = captured_at.into();
        let flow_id = flow_id.into();
        let transaction_ref = transaction.repair_transaction_id.clone();

        let checkpoint_class = checkpoint_class_from_skeleton(skeleton);
        let rollback_class = rollback_class_from_reversal(transaction.transaction_reversal_class);
        let compensation_class =
            compensation_class_from_preview(transaction.transaction_reversal_class);
        let flow_class = flow_class_from_modes(transaction.apply_mode_class, skeleton);
        let preview_disposition =
            preview_disposition_from_skeleton(skeleton.preview_disposition_class);

        let preview = FinalizedRepairPreviewRecord {
            schema_version: FINALIZED_REPAIR_FLOW_SCHEMA_VERSION,
            record_kind: "finalized_repair_preview_record".to_owned(),
            preview_id: skeleton.skeleton_id.clone(),
            repair_transaction_ref: transaction_ref.clone(),
            preview_disposition_class: preview_disposition,
            blast_radius_class: skeleton.blast_radius_class,
            confirmation_class: confirmation_from_reversal(transaction.transaction_reversal_class),
            strong_confirmation_required: transaction
                .transaction_reversal_class
                .requires_strong_confirmation(),
            doctor_finding_ref: skeleton.doctor_finding_ref.clone(),
            impacted_state_classes: transaction.impacted_state_classes.clone(),
            preserved_state_classes: transaction.preserved_state_classes.clone(),
            preview_summary: skeleton.explanation.change_summary.clone(),
            captured_at: captured_at.clone(),
        };

        let checkpoint = FinalizedRepairCheckpointRecord {
            schema_version: FINALIZED_REPAIR_FLOW_SCHEMA_VERSION,
            record_kind: "finalized_repair_checkpoint_record".to_owned(),
            checkpoint_id: format!("checkpoint:{}", &flow_id),
            repair_transaction_ref: transaction_ref.clone(),
            checkpoint_class,
            checkpoint_ref: skeleton.checkpoint_ref.clone(),
            scoped_state_classes: transaction.impacted_state_classes.clone(),
            capture_summary: skeleton.explanation.checkpoint_summary.clone(),
            export_safe: true,
            captured_at: captured_at.clone(),
        };

        let rollback = FinalizedRepairRollbackRecord {
            schema_version: FINALIZED_REPAIR_FLOW_SCHEMA_VERSION,
            record_kind: "finalized_repair_rollback_record".to_owned(),
            rollback_id: format!("rollback:{}", &flow_id),
            repair_transaction_ref: transaction_ref.clone(),
            rollback_class,
            checkpoint_ref_consumed: skeleton.checkpoint_ref.clone(),
            rollback_succeeded: false,
            rollback_summary: transaction.transaction_reversal_class.summary().to_owned(),
            captured_at: captured_at.clone(),
        };

        let compensation = FinalizedRepairCompensationRecord {
            schema_version: FINALIZED_REPAIR_FLOW_SCHEMA_VERSION,
            record_kind: "finalized_repair_compensation_record".to_owned(),
            compensation_id: format!("compensation:{}", &flow_id),
            repair_transaction_ref: transaction_ref.clone(),
            compensation_class,
            strong_acknowledgement_required: compensation_class.requires_strong_acknowledgement(),
            compensation_summary: skeleton.explanation.compensation_summary.clone(),
            follow_up_action_summary: skeleton.explanation.user_facing_next_step.clone(),
            captured_at: captured_at.clone(),
        };

        let ladder_bindings = vec![RecoveryLadderProofBinding {
            rung_class: decision.rung_class,
            decision_id: decision.decision_id.clone(),
            ladder_approved: true,
            ladder_summary: format!(
                "Ladder approved {} for rung {}",
                flow_id,
                decision.rung_class.as_str()
            ),
        }];

        let seeded_scenarios = all_seeded_scenarios(transaction);

        FinalizedRepairTransactionFlow {
            schema_version: FINALIZED_REPAIR_FLOW_SCHEMA_VERSION,
            record_kind: FINALIZED_REPAIR_FLOW_RECORD_KIND.to_owned(),
            flow_id,
            repair_transaction_ref: transaction_ref,
            flow_class,
            preview,
            checkpoint,
            rollback,
            compensation,
            ladder_bindings,
            seeded_scenarios,
            destructive_reset_present: false,
            exact_build_identity_ref: "build:aureline:support:exact".to_owned(),
            captured_at,
        }
    }
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn checkpoint_class_from_skeleton(skeleton: &RepairPreviewSkeleton) -> FinalizedCheckpointClass {
    match skeleton.checkpoint_disposition_class {
        RepairCheckpointDispositionClass::DurablePreApplyCheckpoint => {
            FinalizedCheckpointClass::DurablePreApply
        }
        RepairCheckpointDispositionClass::EphemeralPreApplyCheckpoint => {
            FinalizedCheckpointClass::EphemeralPreApply
        }
        RepairCheckpointDispositionClass::NoCheckpointObserveOnly => {
            FinalizedCheckpointClass::NoCheckpointNeeded
        }
        RepairCheckpointDispositionClass::NoCheckpointEscalationOnly => {
            FinalizedCheckpointClass::NoCheckpointEscalationOnly
        }
    }
}

fn rollback_class_from_reversal(reversal: TransactionReversalClass) -> FinalizedRollbackClass {
    match reversal {
        TransactionReversalClass::Exact => FinalizedRollbackClass::ExactRestoreFromCheckpoint,
        TransactionReversalClass::Compensating => FinalizedRollbackClass::CompensatingRestore,
        TransactionReversalClass::Regenerate => {
            FinalizedRollbackClass::RegenerateFromAuthoritativeSource
        }
        TransactionReversalClass::Manual => FinalizedRollbackClass::ManualRecoveryPath,
        TransactionReversalClass::AuditOnly => FinalizedRollbackClass::NoRollbackNotApplicable,
    }
}

fn compensation_class_from_preview(
    reversal: TransactionReversalClass,
) -> FinalizedCompensationClass {
    match reversal {
        TransactionReversalClass::Exact => FinalizedCompensationClass::NoCompensationNeeded,
        TransactionReversalClass::Compensating => {
            FinalizedCompensationClass::SemanticInverseCompensation
        }
        TransactionReversalClass::Regenerate => {
            FinalizedCompensationClass::RegenerateFromAuthoritativeSource
        }
        TransactionReversalClass::Manual => FinalizedCompensationClass::ManualFollowupRequired,
        TransactionReversalClass::AuditOnly => FinalizedCompensationClass::AuditOnlyNoStateChange,
    }
}

fn compensation_from_reversal(reversal: TransactionReversalClass) -> RepairCompensationClass {
    match reversal {
        TransactionReversalClass::Exact => RepairCompensationClass::NoCompensationNeeded,
        TransactionReversalClass::Compensating => {
            RepairCompensationClass::SemanticInverseCompensation
        }
        TransactionReversalClass::Regenerate => {
            RepairCompensationClass::RegenerateFromAuthoritativeSource
        }
        TransactionReversalClass::Manual => RepairCompensationClass::ManualFollowupRequired,
        TransactionReversalClass::AuditOnly => RepairCompensationClass::AuditOnlyNoStateChange,
    }
}

fn confirmation_from_reversal(reversal: TransactionReversalClass) -> ConfirmationClass {
    if reversal == TransactionReversalClass::AuditOnly {
        ConfirmationClass::NoApplyEscalationOnly
    } else if reversal.requires_strong_confirmation() {
        ConfirmationClass::StrongConfirmationRequired
    } else {
        ConfirmationClass::StandardReview
    }
}

fn flow_class_from_modes(
    apply_mode: ApplyModeClass,
    skeleton: &RepairPreviewSkeleton,
) -> FinalizedRepairFlowClass {
    match apply_mode {
        ApplyModeClass::DryRunPreviewOnly => FinalizedRepairFlowClass::PreviewObserveOnlyAudit,
        ApplyModeClass::ApplyRefusedEscalationOnly => {
            FinalizedRepairFlowClass::PreviewRefusedEscalation
        }
        ApplyModeClass::ApplyObserveOnlyNoWrite => {
            FinalizedRepairFlowClass::PreviewObserveOnlyAudit
        }
        ApplyModeClass::ApplyWithCheckpoint => {
            if skeleton.preview_disposition_class
                == RepairPreviewDispositionClass::ComparisonWithPriorBaseline
            {
                FinalizedRepairFlowClass::ComparisonThenAuthorize
            } else {
                FinalizedRepairFlowClass::PreviewCheckpointApplyWithRollback
            }
        }
        ApplyModeClass::ApplyWithRollbackOnFailure => {
            if skeleton.preview_disposition_class
                == RepairPreviewDispositionClass::ComparisonWithPriorBaseline
            {
                FinalizedRepairFlowClass::ComparisonThenAuthorize
            } else {
                FinalizedRepairFlowClass::PreviewCheckpointApplyWithCompensation
            }
        }
    }
}

fn preview_disposition_from_skeleton(
    disposition: RepairPreviewDispositionClass,
) -> FinalizedPreviewDispositionClass {
    match disposition {
        RepairPreviewDispositionClass::CancellablePendingReview => {
            FinalizedPreviewDispositionClass::PendingReview
        }
        RepairPreviewDispositionClass::ComparisonWithPriorBaseline => {
            FinalizedPreviewDispositionClass::ComparedWithBaseline
        }
        RepairPreviewDispositionClass::AuthorizedForApply => {
            FinalizedPreviewDispositionClass::AuthorizedForApply
        }
        RepairPreviewDispositionClass::CancelledBeforeApply => {
            FinalizedPreviewDispositionClass::CancelledBeforeApply
        }
        RepairPreviewDispositionClass::BlockedPendingEvidence => {
            FinalizedPreviewDispositionClass::BlockedPendingEvidence
        }
        RepairPreviewDispositionClass::RefusedNoLocalRepair => {
            FinalizedPreviewDispositionClass::RefusedEscalationOnly
        }
    }
}

fn all_seeded_scenarios(transaction: &RepairTransactionRecord) -> Vec<SeededSupportScenario> {
    use crate::repair::RepairClassFamily;

    let covered: BTreeSet<FinalizedSeededScenarioClass> = match transaction.repair_class_family {
        RepairClassFamily::DisposableStateRebuild => {
            [FinalizedSeededScenarioClass::CacheIndexRepair]
                .into_iter()
                .collect()
        }
        RepairClassFamily::ExtensionIsolation | RepairClassFamily::ExtensionRollbackReinstall => {
            [FinalizedSeededScenarioClass::ExtensionQuarantineBisect]
                .into_iter()
                .collect()
        }
        RepairClassFamily::ExecutionContextReresolve => {
            [FinalizedSeededScenarioClass::ToolchainReresolve]
                .into_iter()
                .collect()
        }
        RepairClassFamily::RemoteRuntimeRepair => {
            [FinalizedSeededScenarioClass::RemoteAgentRollback]
                .into_iter()
                .collect()
        }
        RepairClassFamily::PolicyEntitlementRefresh => {
            [FinalizedSeededScenarioClass::PolicyEntitlementRefresh]
                .into_iter()
                .collect()
        }
        RepairClassFamily::GuidedExportEscalation => {
            [FinalizedSeededScenarioClass::EscalationOnlyNoLocalRepair]
                .into_iter()
                .collect()
        }
        RepairClassFamily::ObserveOnlyNoRepair => {
            [FinalizedSeededScenarioClass::ObserveOnlyNoRepair]
                .into_iter()
                .collect()
        }
    };

    FinalizedSeededScenarioClass::REQUIRED
        .into_iter()
        .map(|scenario| SeededSupportScenario {
            scenario_class: scenario,
            covered: covered.contains(&scenario),
            coverage_note: if covered.contains(&scenario) {
                format!(
                    "Covered by {} flow.",
                    transaction.repair_class_family.as_str()
                )
            } else {
                "Not covered by this repair class family.".to_owned()
            },
        })
        .collect()
}

// ---------------------------------------------------------------------------
// AsStr impls for RepairClassFamily (needed by all_seeded_scenarios)
// ---------------------------------------------------------------------------

impl RepairClassFamily {
    fn as_str(self) -> &'static str {
        match self {
            Self::DisposableStateRebuild => "disposable_state_rebuild",
            Self::ExtensionIsolation => "extension_isolation",
            Self::ExtensionRollbackReinstall => "extension_rollback_reinstall",
            Self::ExecutionContextReresolve => "execution_context_reresolve",
            Self::RemoteRuntimeRepair => "remote_runtime_repair",
            Self::PolicyEntitlementRefresh => "policy_entitlement_refresh",
            Self::GuidedExportEscalation => "guided_export_escalation",
            Self::ObserveOnlyNoRepair => "observe_only_no_repair",
        }
    }
}

impl RecoveryRungClass {
    fn as_str(self) -> &'static str {
        match self {
            Self::SafeMode => "safe_mode",
            Self::RuntimeExtensionQuarantine => "runtime_extension_quarantine",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::CacheIndexRepair => "cache_index_repair",
        }
    }
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

fn push_violation(
    violations: &mut Vec<FinalizedRepairFlowViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(FinalizedRepairFlowViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
