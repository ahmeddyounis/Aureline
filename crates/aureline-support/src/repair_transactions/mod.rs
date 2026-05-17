//! Typed repair-transaction preview skeleton, comparison, and support packet.
//!
//! The repair-transaction preview skeleton is the bounded recovery posture a
//! blocked user enters when Project Doctor or Support Center proposes a fix.
//! The skeleton *declares* — by name and reason — what object classes a
//! repair would touch, what its blast radius is, what compensation it
//! requires, and what checkpoint it would capture before mutating durable
//! state. The skeleton is cancellable: a reviewer can compare it against a
//! prior baseline, accept it, or cancel it before any apply path runs.
//!
//! This module mints three typed records that mirror the boundary schema at
//! [`/schemas/support/repair_transaction_preview_skeleton.schema.json`]:
//!
//! - [`RepairPreviewSkeleton`] declares one previewed transaction as a typed
//!   [`RepairBlastRadiusClass`], [`RepairCompensationClass`],
//!   [`RepairAffectedObjectClass`] list,
//!   [`RepairCheckpointDispositionClass`], and
//!   [`RepairPreviewDispositionClass`] row bound to the alpha
//!   transaction id and reversal class so support and audit packets preserve
//!   the same transaction id and reversal class.
//! - [`RepairPreviewComparison`] records one cancellable comparison between
//!   a baseline skeleton and a candidate skeleton across the closed
//!   [`RepairComparisonAxisClass`] vocabulary.
//! - [`RepairPreviewSupportPacket`] folds one skeleton and its comparisons
//!   into a metadata-safe support projection that excludes raw private
//!   material and ambient authority.
//!
//! [`RepairPreviewSkeletonEvaluator::support_packet`] is the entry point a
//! support-export or release-control pipeline consumes verbatim. It refuses
//! to emit a packet when the skeleton declares a destructive reset, drops
//! `user_authored_files` from preserved state, fails to cite a Project
//! Doctor finding, or carries a comparison whose baseline/candidate refs do
//! not match the bound skeleton id.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::repair::{
    ImpactedStateClass, PreservedStateClass, RepairTransactionRecord, TransactionReversalClass,
};

/// Stable record-kind tag for a repair-preview-skeleton record.
pub const REPAIR_PREVIEW_SKELETON_RECORD_KIND: &str = "repair_preview_skeleton_record";

/// Stable record-kind tag for a repair-preview-comparison record.
pub const REPAIR_PREVIEW_COMPARISON_RECORD_KIND: &str = "repair_preview_comparison_record";

/// Stable record-kind tag for the metadata-safe support projection.
pub const REPAIR_PREVIEW_SUPPORT_PACKET_RECORD_KIND: &str =
    "repair_preview_skeleton_support_packet_record";

/// Frozen schema version for repair-preview-skeleton beta records.
pub const REPAIR_PREVIEW_SKELETON_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const REPAIR_PREVIEW_SKELETON_SCHEMA_REF: &str =
    "schemas/support/repair_transaction_preview_skeleton.schema.json";

/// Reviewer doc ref quoted by every emitted packet.
pub const REPAIR_PREVIEW_SKELETON_DOC_REF: &str = "docs/support/m3/repair_transaction_beta.md";

/// Closed blast-radius vocabulary for repair-preview skeletons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairBlastRadiusClass {
    /// Only one object class is touched.
    SingleObjectClass,
    /// Multiple object classes in the same family are touched.
    MultiObjectClassSameFamily,
    /// Object classes across multiple families are touched.
    MultiObjectClassCrossFamily,
    /// No local mutation occurs; the repair only prepares escalation.
    NoLocalBlastEscalationOnly,
}

impl RepairBlastRadiusClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleObjectClass => "single_object_class",
            Self::MultiObjectClassSameFamily => "multi_object_class_same_family",
            Self::MultiObjectClassCrossFamily => "multi_object_class_cross_family",
            Self::NoLocalBlastEscalationOnly => "no_local_blast_escalation_only",
        }
    }
}

/// Closed compensation vocabulary distinct from the alpha reversal class.
///
/// `transaction_reversal_class` says how the prior state is restored after
/// apply. `RepairCompensationClass` says what kind of compensating work
/// reviewers must accept before apply runs at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairCompensationClass {
    /// No compensating action is needed because nothing durable changes.
    NoCompensationNeeded,
    /// Disposable derived state is regenerated from authoritative sources.
    RegenerateFromAuthoritativeSource,
    /// A semantic inverse is applied (for example, quarantine release).
    SemanticInverseCompensation,
    /// Manual follow-up is required (for example, user reinstalls an extension).
    ManualFollowupRequired,
    /// No state is changed; only audit records or escalation prepared.
    AuditOnlyNoStateChange,
}

impl RepairCompensationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCompensationNeeded => "no_compensation_needed",
            Self::RegenerateFromAuthoritativeSource => "regenerate_from_authoritative_source",
            Self::SemanticInverseCompensation => "semantic_inverse_compensation",
            Self::ManualFollowupRequired => "manual_followup_required",
            Self::AuditOnlyNoStateChange => "audit_only_no_state_change",
        }
    }

    /// Returns true when the compensation class requires explicit reviewer
    /// acknowledgement beyond a standard review click.
    pub fn requires_strong_acknowledgement(self) -> bool {
        matches!(
            self,
            Self::SemanticInverseCompensation | Self::ManualFollowupRequired
        )
    }
}

/// Closed object-class vocabulary the skeleton may name as affected.
///
/// Mirrors `impacted_state_class` from the alpha contract but renamed at the
/// preview boundary so reviewers see object classes, not raw state stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairAffectedObjectClass {
    /// Disposable derived cache object.
    DerivedCacheObject,
    /// Watcher backlog object.
    WatcherBacklogObject,
    /// Docs-pack mirror snapshot object.
    DocsPackMirrorObject,
    /// Execution-context handle object.
    ExecutionContextObject,
    /// Language-server session handle object.
    LanguageServerSessionObject,
    /// Extension quarantine state object.
    ExtensionQuarantineObject,
    /// Extension install-set object.
    ExtensionInstallSetObject,
    /// Remote helper session handle object.
    RemoteHelperSessionObject,
    /// Remote agent runtime handle object.
    RemoteAgentRuntimeObject,
    /// Policy entitlement handle object.
    PolicyEntitlementObject,
    /// Trust approval ticket object.
    TrustApprovalObject,
    /// Support export store object.
    SupportExportObject,
    /// Doctor audit log entry object.
    DoctorAuditLogObject,
}

impl RepairAffectedObjectClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DerivedCacheObject => "derived_cache_object",
            Self::WatcherBacklogObject => "watcher_backlog_object",
            Self::DocsPackMirrorObject => "docs_pack_mirror_object",
            Self::ExecutionContextObject => "execution_context_object",
            Self::LanguageServerSessionObject => "language_server_session_object",
            Self::ExtensionQuarantineObject => "extension_quarantine_object",
            Self::ExtensionInstallSetObject => "extension_install_set_object",
            Self::RemoteHelperSessionObject => "remote_helper_session_object",
            Self::RemoteAgentRuntimeObject => "remote_agent_runtime_object",
            Self::PolicyEntitlementObject => "policy_entitlement_object",
            Self::TrustApprovalObject => "trust_approval_object",
            Self::SupportExportObject => "support_export_object",
            Self::DoctorAuditLogObject => "doctor_audit_log_object",
        }
    }

    /// Returns the mirrored impacted-state class from the alpha contract.
    pub fn from_impacted_state(state: ImpactedStateClass) -> Self {
        match state {
            ImpactedStateClass::DisposableDerivedCache => Self::DerivedCacheObject,
            ImpactedStateClass::WatcherBacklogState => Self::WatcherBacklogObject,
            ImpactedStateClass::DocsPackMirrorSnapshot => Self::DocsPackMirrorObject,
            ImpactedStateClass::ExecutionContextHandle => Self::ExecutionContextObject,
            ImpactedStateClass::LanguageServerSessionHandle => Self::LanguageServerSessionObject,
            ImpactedStateClass::ExtensionQuarantineState => Self::ExtensionQuarantineObject,
            ImpactedStateClass::ExtensionInstallSet => Self::ExtensionInstallSetObject,
            ImpactedStateClass::RemoteHelperSessionHandle => Self::RemoteHelperSessionObject,
            ImpactedStateClass::RemoteAgentRuntimeHandle => Self::RemoteAgentRuntimeObject,
            ImpactedStateClass::PolicyEntitlementHandle => Self::PolicyEntitlementObject,
            ImpactedStateClass::TrustApprovalTicket => Self::TrustApprovalObject,
            ImpactedStateClass::SupportExportStoreExport => Self::SupportExportObject,
            ImpactedStateClass::DoctorAuditLogEntry => Self::DoctorAuditLogObject,
        }
    }

    /// Returns the object-class family this class belongs to. Used to compute
    /// blast-radius truth.
    pub fn family(self) -> RepairObjectFamily {
        match self {
            Self::DerivedCacheObject
            | Self::WatcherBacklogObject
            | Self::DocsPackMirrorObject => RepairObjectFamily::DisposableCacheFamily,
            Self::ExecutionContextObject | Self::LanguageServerSessionObject => {
                RepairObjectFamily::ExecutionContextFamily
            }
            Self::ExtensionQuarantineObject | Self::ExtensionInstallSetObject => {
                RepairObjectFamily::ExtensionFamily
            }
            Self::RemoteHelperSessionObject | Self::RemoteAgentRuntimeObject => {
                RepairObjectFamily::RemoteRuntimeFamily
            }
            Self::PolicyEntitlementObject | Self::TrustApprovalObject => {
                RepairObjectFamily::TrustPolicyFamily
            }
            Self::SupportExportObject | Self::DoctorAuditLogObject => {
                RepairObjectFamily::AuditFamily
            }
        }
    }
}

/// Coarse family the affected-object class belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairObjectFamily {
    /// Disposable cache, watcher backlog, or docs-pack mirror family.
    DisposableCacheFamily,
    /// Execution-context or language-server handle family.
    ExecutionContextFamily,
    /// Extension quarantine or install-set family.
    ExtensionFamily,
    /// Remote helper or agent runtime family.
    RemoteRuntimeFamily,
    /// Trust or policy handle family.
    TrustPolicyFamily,
    /// Audit-log or support-export family.
    AuditFamily,
}

/// Closed checkpoint-disposition vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairCheckpointDispositionClass {
    /// A durable pre-apply checkpoint is captured and named.
    DurablePreApplyCheckpoint,
    /// A session-scoped ephemeral checkpoint is captured.
    EphemeralPreApplyCheckpoint,
    /// No checkpoint is needed because the repair does not write state.
    NoCheckpointObserveOnly,
    /// No checkpoint exists because the repair only prepares escalation.
    NoCheckpointEscalationOnly,
}

impl RepairCheckpointDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurablePreApplyCheckpoint => "durable_pre_apply_checkpoint",
            Self::EphemeralPreApplyCheckpoint => "ephemeral_pre_apply_checkpoint",
            Self::NoCheckpointObserveOnly => "no_checkpoint_observe_only",
            Self::NoCheckpointEscalationOnly => "no_checkpoint_escalation_only",
        }
    }

    /// Returns true when this disposition requires a non-null checkpoint ref.
    pub fn requires_checkpoint_ref(self) -> bool {
        matches!(
            self,
            Self::DurablePreApplyCheckpoint | Self::EphemeralPreApplyCheckpoint
        )
    }
}

/// Closed preview-disposition vocabulary describing the reviewer-facing
/// state of a skeleton.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairPreviewDispositionClass {
    /// Preview is complete; reviewer may cancel or accept before apply.
    CancellablePendingReview,
    /// Preview is compared against a prior baseline before authorization.
    ComparisonWithPriorBaseline,
    /// Preview is authorized for apply (strong confirmation, if any, held).
    AuthorizedForApply,
    /// Preview was cancelled before apply.
    CancelledBeforeApply,
    /// Preview is blocked pending more evidence or policy review.
    BlockedPendingEvidence,
    /// Preview refused local apply and routed to escalation.
    RefusedNoLocalRepair,
}

impl RepairPreviewDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CancellablePendingReview => "cancellable_pending_review",
            Self::ComparisonWithPriorBaseline => "comparison_with_prior_baseline",
            Self::AuthorizedForApply => "authorized_for_apply",
            Self::CancelledBeforeApply => "cancelled_before_apply",
            Self::BlockedPendingEvidence => "blocked_pending_evidence",
            Self::RefusedNoLocalRepair => "refused_no_local_repair",
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
            Self::CancelledBeforeApply | Self::RefusedNoLocalRepair
        )
    }
}

/// Closed comparison-axis vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairComparisonAxisClass {
    /// Comparison surfaces a blast-radius difference.
    BlastRadiusDiff,
    /// Comparison surfaces a compensation difference.
    CompensationDiff,
    /// Comparison surfaces an affected-object difference.
    AffectedObjectDiff,
    /// Comparison surfaces a preserved-state difference.
    PreservedStateDiff,
    /// Comparison surfaces a checkpoint difference.
    CheckpointDiff,
    /// Comparison surfaces a reversal-class difference.
    ReversalClassDiff,
}

impl RepairComparisonAxisClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlastRadiusDiff => "blast_radius_diff",
            Self::CompensationDiff => "compensation_diff",
            Self::AffectedObjectDiff => "affected_object_diff",
            Self::PreservedStateDiff => "preserved_state_diff",
            Self::CheckpointDiff => "checkpoint_diff",
            Self::ReversalClassDiff => "reversal_class_diff",
        }
    }
}

/// Closed cancellation-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairCancellationClass {
    /// Comparison continues without an authorization decision.
    ContinueComparison,
    /// Reviewer cancels the proposed repair before apply.
    CancelBeforeApply,
    /// Reviewer accepts the comparison and forwards to apply review.
    ReadyForApplyReview,
}

impl RepairCancellationClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinueComparison => "continue_comparison",
            Self::CancelBeforeApply => "cancel_before_apply",
            Self::ReadyForApplyReview => "ready_for_apply_review",
        }
    }
}

/// One reviewer-facing affected-object row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairAffectedObjectRow {
    /// Affected object class.
    pub affected_object_class: RepairAffectedObjectClass,
    /// Reviewer-safe change summary.
    pub change_summary: String,
}

/// Reviewer-facing explanation block carried by a skeleton.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewSkeletonExplanation {
    /// What changes if the repair applies.
    pub change_summary: String,
    /// What is preserved across apply and any compensation.
    pub preserved_summary: String,
    /// What compensation costs the reviewer accepts.
    pub compensation_summary: String,
    /// What checkpoint is captured (or why none is needed).
    pub checkpoint_summary: String,
    /// What the reviewer can do next (cancel, compare, authorize).
    pub user_facing_next_step: String,
}

/// One typed repair-preview skeleton.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewSkeleton {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable skeleton identifier.
    pub skeleton_id: String,
    /// Repair transaction ref the skeleton is bound to.
    pub repair_transaction_ref: String,
    /// Reversal class preserved verbatim from the alpha transaction.
    pub reversal_class: TransactionReversalClass,
    /// Blast-radius class for the proposed repair.
    pub blast_radius_class: RepairBlastRadiusClass,
    /// Compensation class for the proposed repair.
    pub compensation_class: RepairCompensationClass,
    /// Checkpoint disposition for the proposed repair.
    pub checkpoint_disposition_class: RepairCheckpointDispositionClass,
    /// Preview disposition (cancellable, comparable, authorized, etc.).
    pub preview_disposition_class: RepairPreviewDispositionClass,
    /// Affected object rows for the proposed repair.
    pub affected_object_rows: Vec<RepairAffectedObjectRow>,
    /// Preserved state classes the repair must not mutate.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Project Doctor finding that justified the repair.
    pub doctor_finding_ref: String,
    /// Optional checkpoint ref (required when the disposition demands one).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checkpoint_ref: Option<String>,
    /// Support packet ref that consumes the skeleton.
    pub support_packet_ref: String,
    /// Whether the skeleton declares a destructive reset.
    pub destructive_resets_present: bool,
    /// Reviewer-facing explanation block.
    pub explanation: RepairPreviewSkeletonExplanation,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One typed comparison between a baseline skeleton and a candidate
/// skeleton.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewComparison {
    /// Frozen schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable comparison identifier.
    pub comparison_id: String,
    /// Skeleton id the comparison is bound to.
    pub bound_skeleton_ref: String,
    /// Baseline skeleton ref (often a prior preview of the same transaction).
    pub baseline_skeleton_ref: String,
    /// Candidate skeleton ref (typically equals `bound_skeleton_ref`).
    pub candidate_skeleton_ref: String,
    /// Comparison axes that differ.
    pub differing_axes: Vec<RepairComparisonAxisClass>,
    /// Cancellation class for the comparison.
    pub cancellation_class: RepairCancellationClass,
    /// Reviewer-safe reviewer summary.
    pub reviewer_summary: String,
    /// Capture timestamp.
    pub captured_at: String,
}

impl RepairPreviewComparison {
    /// Returns true when the comparison preserves the bound skeleton id on
    /// both sides.
    pub fn is_consistent(&self) -> bool {
        self.bound_skeleton_ref == self.candidate_skeleton_ref
            && !self.baseline_skeleton_ref.is_empty()
    }
}

/// Metadata-safe support projection joining one skeleton and its
/// comparisons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewSupportPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Doc ref the packet quotes.
    pub doc_ref: String,
    /// Boundary schema ref the packet mirrors.
    pub schema_ref: String,
    /// Skeleton id projected by the packet.
    pub skeleton_id: String,
    /// Repair transaction ref preserved across packet boundaries.
    pub repair_transaction_ref: String,
    /// Reversal class preserved across packet boundaries.
    pub reversal_class: TransactionReversalClass,
    /// Blast-radius class projected by the packet.
    pub blast_radius_class: RepairBlastRadiusClass,
    /// Compensation class projected by the packet.
    pub compensation_class: RepairCompensationClass,
    /// Checkpoint disposition projected by the packet.
    pub checkpoint_disposition_class: RepairCheckpointDispositionClass,
    /// Preview disposition projected by the packet.
    pub preview_disposition_class: RepairPreviewDispositionClass,
    /// Affected object rows.
    pub affected_object_rows: Vec<RepairAffectedObjectRow>,
    /// Preserved state classes.
    pub preserved_state_classes: Vec<PreservedStateClass>,
    /// Project Doctor finding ref.
    pub doctor_finding_ref: String,
    /// Optional checkpoint ref the packet quotes.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub checkpoint_ref: Option<String>,
    /// Comparison rows bundled with the skeleton.
    pub comparison_rows: Vec<RepairPreviewSupportComparisonRow>,
    /// Whether raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Whether ambient authority is excluded.
    pub ambient_authority_excluded: bool,
    /// Whether the projection records a destructive reset.
    pub destructive_resets_present: bool,
}

impl RepairPreviewSupportPacket {
    /// Returns true when the packet preserves the bounded contract.
    pub fn is_export_safe(&self) -> bool {
        self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.destructive_resets_present
            && self
                .preserved_state_classes
                .contains(&PreservedStateClass::UserAuthoredFiles)
            && self.doctor_finding_ref.starts_with("doctor.finding.")
            && self
                .comparison_rows
                .iter()
                .all(RepairPreviewSupportComparisonRow::is_export_safe)
    }
}

/// One comparison row in the support projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairPreviewSupportComparisonRow {
    /// Comparison id.
    pub comparison_id: String,
    /// Baseline skeleton ref.
    pub baseline_skeleton_ref: String,
    /// Candidate skeleton ref (matches the bound skeleton).
    pub candidate_skeleton_ref: String,
    /// Comparison axes that differ.
    pub differing_axes: Vec<RepairComparisonAxisClass>,
    /// Cancellation class.
    pub cancellation_class: RepairCancellationClass,
    /// Reviewer summary.
    pub reviewer_summary: String,
}

impl RepairPreviewSupportComparisonRow {
    /// Returns true when this row preserves the contract.
    pub fn is_export_safe(&self) -> bool {
        !self.baseline_skeleton_ref.is_empty()
            && !self.candidate_skeleton_ref.is_empty()
            && !self.differing_axes.is_empty()
    }
}

impl From<&RepairPreviewComparison> for RepairPreviewSupportComparisonRow {
    fn from(comparison: &RepairPreviewComparison) -> Self {
        Self {
            comparison_id: comparison.comparison_id.clone(),
            baseline_skeleton_ref: comparison.baseline_skeleton_ref.clone(),
            candidate_skeleton_ref: comparison.candidate_skeleton_ref.clone(),
            differing_axes: comparison.differing_axes.clone(),
            cancellation_class: comparison.cancellation_class,
            reviewer_summary: comparison.reviewer_summary.clone(),
        }
    }
}

/// One validation failure emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairPreviewSkeletonViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Reviewer-facing failure message.
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepairPreviewSkeletonValidationReport {
    /// Validation failures.
    pub violations: Vec<RepairPreviewSkeletonViolation>,
}

impl fmt::Display for RepairPreviewSkeletonValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} repair-preview-skeleton violation(s)",
            self.violations.len()
        )
    }
}

impl Error for RepairPreviewSkeletonValidationReport {}

/// Loads a repair-preview skeleton from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`RepairPreviewSkeleton`].
pub fn load_repair_preview_skeleton(
    yaml: &str,
) -> Result<RepairPreviewSkeleton, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a repair-preview comparison from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text is not shaped like a
/// [`RepairPreviewComparison`].
pub fn load_repair_preview_comparison(
    yaml: &str,
) -> Result<RepairPreviewComparison, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Repair-preview skeleton beta evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct RepairPreviewSkeletonEvaluator;

impl RepairPreviewSkeletonEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates a [`RepairPreviewSkeleton`].
    ///
    /// # Errors
    ///
    /// Returns [`RepairPreviewSkeletonValidationReport`] when the skeleton
    /// drops the user-authored-files preservation, declares a destructive
    /// reset, omits a required checkpoint ref, names a checkpoint ref when
    /// the disposition forbids one, or carries duplicate affected-object
    /// classes.
    pub fn validate_skeleton(
        &self,
        skeleton: &RepairPreviewSkeleton,
    ) -> Result<(), RepairPreviewSkeletonValidationReport> {
        let violations = validate_skeleton(skeleton);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(RepairPreviewSkeletonValidationReport { violations })
        }
    }

    /// Validates a [`RepairPreviewComparison`].
    ///
    /// # Errors
    ///
    /// Returns [`RepairPreviewSkeletonValidationReport`] when the comparison
    /// names no differing axes, leaves the baseline or candidate ref empty,
    /// or fails to bind the candidate ref to the bound skeleton ref.
    pub fn validate_comparison(
        &self,
        comparison: &RepairPreviewComparison,
    ) -> Result<(), RepairPreviewSkeletonValidationReport> {
        let violations = validate_comparison(comparison);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(RepairPreviewSkeletonValidationReport { violations })
        }
    }

    /// Validates a comparison against the bound skeleton.
    ///
    /// # Errors
    ///
    /// Returns [`RepairPreviewSkeletonValidationReport`] when the
    /// comparison's bound or candidate refs do not match the supplied
    /// skeleton id.
    pub fn validate_comparison_against_skeleton(
        &self,
        skeleton: &RepairPreviewSkeleton,
        comparison: &RepairPreviewComparison,
    ) -> Result<(), RepairPreviewSkeletonValidationReport> {
        let mut violations = validate_comparison(comparison);
        if comparison.bound_skeleton_ref != skeleton.skeleton_id {
            push_violation(
                &mut violations,
                "repair_preview.comparison_bound_ref_mismatch",
                &comparison.comparison_id,
                "comparison bound_skeleton_ref must equal the bound skeleton_id",
            );
        }
        if comparison.candidate_skeleton_ref != skeleton.skeleton_id {
            push_violation(
                &mut violations,
                "repair_preview.comparison_candidate_ref_mismatch",
                &comparison.comparison_id,
                "comparison candidate_skeleton_ref must equal the bound skeleton_id",
            );
        }
        if violations.is_empty() {
            Ok(())
        } else {
            Err(RepairPreviewSkeletonValidationReport { violations })
        }
    }

    /// Builds the metadata-safe support packet projection.
    ///
    /// # Errors
    ///
    /// Returns [`RepairPreviewSkeletonValidationReport`] when the skeleton
    /// or any comparison fails validation, or when a comparison binds to a
    /// different skeleton.
    pub fn support_packet(
        &self,
        packet_id: impl Into<String>,
        captured_at: impl Into<String>,
        skeleton: &RepairPreviewSkeleton,
        comparisons: &[RepairPreviewComparison],
    ) -> Result<RepairPreviewSupportPacket, RepairPreviewSkeletonValidationReport> {
        let mut violations = validate_skeleton(skeleton);
        for comparison in comparisons {
            violations.extend(validate_comparison(comparison));
            if comparison.bound_skeleton_ref != skeleton.skeleton_id {
                push_violation(
                    &mut violations,
                    "repair_preview.comparison_bound_ref_mismatch",
                    &comparison.comparison_id,
                    "comparison bound_skeleton_ref must equal the bound skeleton_id",
                );
            }
            if comparison.candidate_skeleton_ref != skeleton.skeleton_id {
                push_violation(
                    &mut violations,
                    "repair_preview.comparison_candidate_ref_mismatch",
                    &comparison.comparison_id,
                    "comparison candidate_skeleton_ref must equal the bound skeleton_id",
                );
            }
        }
        if !violations.is_empty() {
            return Err(RepairPreviewSkeletonValidationReport { violations });
        }

        let comparison_rows = comparisons
            .iter()
            .map(RepairPreviewSupportComparisonRow::from)
            .collect::<Vec<_>>();

        Ok(RepairPreviewSupportPacket {
            record_kind: REPAIR_PREVIEW_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: REPAIR_PREVIEW_SKELETON_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REPAIR_PREVIEW_SKELETON_DOC_REF.to_owned(),
            schema_ref: REPAIR_PREVIEW_SKELETON_SCHEMA_REF.to_owned(),
            skeleton_id: skeleton.skeleton_id.clone(),
            repair_transaction_ref: skeleton.repair_transaction_ref.clone(),
            reversal_class: skeleton.reversal_class,
            blast_radius_class: skeleton.blast_radius_class,
            compensation_class: skeleton.compensation_class,
            checkpoint_disposition_class: skeleton.checkpoint_disposition_class,
            preview_disposition_class: skeleton.preview_disposition_class,
            affected_object_rows: skeleton.affected_object_rows.clone(),
            preserved_state_classes: skeleton.preserved_state_classes.clone(),
            doctor_finding_ref: skeleton.doctor_finding_ref.clone(),
            checkpoint_ref: skeleton.checkpoint_ref.clone(),
            comparison_rows,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
        })
    }

    /// Builds a skeleton row from an alpha [`RepairTransactionRecord`].
    ///
    /// The resulting skeleton preserves the transaction id and reversal
    /// class verbatim and derives the blast-radius class, compensation
    /// class, checkpoint disposition, and affected-object rows from the
    /// transaction's declared shape. Callers may further mutate the
    /// `preview_disposition_class` to reflect cancellation, comparison, or
    /// authorization state.
    pub fn from_alpha_transaction(
        &self,
        transaction: &RepairTransactionRecord,
        skeleton_id: impl Into<String>,
        doctor_finding_ref: impl Into<String>,
        support_packet_ref: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> RepairPreviewSkeleton {
        let affected_object_rows = transaction
            .impacted_state_classes
            .iter()
            .copied()
            .map(|state| RepairAffectedObjectRow {
                affected_object_class: RepairAffectedObjectClass::from_impacted_state(state),
                change_summary: transaction.explanation_fields.change_summary.clone(),
            })
            .collect::<Vec<_>>();

        let blast_radius_class = blast_radius_from_objects(&affected_object_rows);
        let compensation_class = compensation_from_reversal(transaction.transaction_reversal_class);
        let checkpoint_disposition_class = checkpoint_disposition_from_transaction(transaction);
        let preview_disposition_class = preview_disposition_from_transaction(transaction);

        RepairPreviewSkeleton {
            schema_version: REPAIR_PREVIEW_SKELETON_SCHEMA_VERSION,
            record_kind: REPAIR_PREVIEW_SKELETON_RECORD_KIND.to_owned(),
            skeleton_id: skeleton_id.into(),
            repair_transaction_ref: transaction.repair_transaction_id.clone(),
            reversal_class: transaction.transaction_reversal_class,
            blast_radius_class,
            compensation_class,
            checkpoint_disposition_class,
            preview_disposition_class,
            affected_object_rows,
            preserved_state_classes: transaction.preserved_state_classes.clone(),
            doctor_finding_ref: doctor_finding_ref.into(),
            checkpoint_ref: transaction.checkpoint_ref.clone(),
            support_packet_ref: support_packet_ref.into(),
            destructive_resets_present: false,
            explanation: RepairPreviewSkeletonExplanation {
                change_summary: transaction.explanation_fields.change_summary.clone(),
                preserved_summary: transaction
                    .explanation_fields
                    .preserved_work_summary
                    .clone(),
                compensation_summary: compensation_class
                    .summary_for(transaction.transaction_reversal_class),
                checkpoint_summary: checkpoint_disposition_class
                    .summary_for(transaction.checkpoint_ref.as_deref()),
                user_facing_next_step: transaction
                    .explanation_fields
                    .user_facing_next_step
                    .clone(),
            },
            captured_at: captured_at.into(),
        }
    }
}

impl RepairCompensationClass {
    fn summary_for(self, reversal: TransactionReversalClass) -> String {
        match self {
            Self::NoCompensationNeeded => {
                "No compensation is needed; the repair only touches disposable state."
                    .to_owned()
            }
            Self::RegenerateFromAuthoritativeSource => {
                "Regenerates disposable derived state from authoritative sources.".to_owned()
            }
            Self::SemanticInverseCompensation => {
                "Applies a semantic inverse (for example, releases a quarantine) and \
requires strong reviewer acknowledgement before apply."
                    .to_owned()
            }
            Self::ManualFollowupRequired => {
                "Reversal requires manual follow-up; the reviewer must acknowledge the \
manual recovery path before apply."
                    .to_owned()
            }
            Self::AuditOnlyNoStateChange => format!(
                "No state changes; reversal class {} is audit-only.",
                reversal.as_str()
            ),
        }
    }
}

impl TransactionReversalClass {
    /// Stable snake-case token mirroring the boundary schema enum.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compensating => "compensating",
            Self::Regenerate => "regenerate",
            Self::Manual => "manual",
            Self::AuditOnly => "audit_only",
        }
    }
}

impl RepairCheckpointDispositionClass {
    fn summary_for(self, checkpoint_ref: Option<&str>) -> String {
        match (self, checkpoint_ref) {
            (Self::DurablePreApplyCheckpoint, Some(reference)) => {
                format!("A durable pre-apply checkpoint is available: {reference}.")
            }
            (Self::EphemeralPreApplyCheckpoint, Some(reference)) => {
                format!("An ephemeral pre-apply checkpoint is available: {reference}.")
            }
            (Self::DurablePreApplyCheckpoint, None) => {
                "A durable pre-apply checkpoint is required but not yet captured.".to_owned()
            }
            (Self::EphemeralPreApplyCheckpoint, None) => {
                "An ephemeral pre-apply checkpoint is required but not yet captured.".to_owned()
            }
            (Self::NoCheckpointObserveOnly, _) => {
                "No checkpoint is needed because the repair only observes state.".to_owned()
            }
            (Self::NoCheckpointEscalationOnly, _) => {
                "No checkpoint exists because the repair only prepares escalation.".to_owned()
            }
        }
    }
}

fn blast_radius_from_objects(rows: &[RepairAffectedObjectRow]) -> RepairBlastRadiusClass {
    if rows.is_empty() {
        return RepairBlastRadiusClass::NoLocalBlastEscalationOnly;
    }
    let families: BTreeSet<RepairObjectFamily> = rows
        .iter()
        .map(|row| row.affected_object_class.family())
        .collect();
    if rows.len() == 1 {
        RepairBlastRadiusClass::SingleObjectClass
    } else if families.len() == 1 {
        RepairBlastRadiusClass::MultiObjectClassSameFamily
    } else {
        RepairBlastRadiusClass::MultiObjectClassCrossFamily
    }
}

fn compensation_from_reversal(reversal: TransactionReversalClass) -> RepairCompensationClass {
    match reversal {
        TransactionReversalClass::Exact => RepairCompensationClass::NoCompensationNeeded,
        TransactionReversalClass::Regenerate => {
            RepairCompensationClass::RegenerateFromAuthoritativeSource
        }
        TransactionReversalClass::Compensating => {
            RepairCompensationClass::SemanticInverseCompensation
        }
        TransactionReversalClass::Manual => RepairCompensationClass::ManualFollowupRequired,
        TransactionReversalClass::AuditOnly => RepairCompensationClass::AuditOnlyNoStateChange,
    }
}

fn checkpoint_disposition_from_transaction(
    transaction: &RepairTransactionRecord,
) -> RepairCheckpointDispositionClass {
    use crate::repair::ApplyModeClass;
    match transaction.apply_mode_class {
        ApplyModeClass::ApplyWithCheckpoint | ApplyModeClass::ApplyWithRollbackOnFailure => {
            RepairCheckpointDispositionClass::DurablePreApplyCheckpoint
        }
        ApplyModeClass::DryRunPreviewOnly | ApplyModeClass::ApplyObserveOnlyNoWrite => {
            RepairCheckpointDispositionClass::NoCheckpointObserveOnly
        }
        ApplyModeClass::ApplyRefusedEscalationOnly => {
            RepairCheckpointDispositionClass::NoCheckpointEscalationOnly
        }
    }
}

fn preview_disposition_from_transaction(
    transaction: &RepairTransactionRecord,
) -> RepairPreviewDispositionClass {
    use crate::repair::ApplyModeClass;
    match transaction.apply_mode_class {
        ApplyModeClass::ApplyRefusedEscalationOnly => {
            RepairPreviewDispositionClass::RefusedNoLocalRepair
        }
        _ => RepairPreviewDispositionClass::CancellablePendingReview,
    }
}

fn validate_skeleton(skeleton: &RepairPreviewSkeleton) -> Vec<RepairPreviewSkeletonViolation> {
    let mut violations = Vec::new();

    if skeleton.schema_version != REPAIR_PREVIEW_SKELETON_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "repair_preview.schema_version",
            &skeleton.skeleton_id,
            "skeleton schema_version must be 1",
        );
    }
    if skeleton.record_kind != REPAIR_PREVIEW_SKELETON_RECORD_KIND {
        push_violation(
            &mut violations,
            "repair_preview.record_kind",
            &skeleton.skeleton_id,
            "skeleton record_kind must be repair_preview_skeleton_record",
        );
    }
    if skeleton.skeleton_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.skeleton_id_empty",
            &skeleton.skeleton_id,
            "skeleton_id must be non-empty",
        );
    }
    if skeleton.repair_transaction_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.repair_transaction_ref_empty",
            &skeleton.skeleton_id,
            "skeleton must name a repair_transaction_ref",
        );
    }
    if !skeleton.doctor_finding_ref.starts_with("doctor.finding.") {
        push_violation(
            &mut violations,
            "repair_preview.doctor_finding_ref_missing",
            &skeleton.skeleton_id,
            "skeleton must cite a doctor.finding.* ref",
        );
    }
    if skeleton.support_packet_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.support_packet_ref_missing",
            &skeleton.skeleton_id,
            "skeleton must cite a support_packet_ref",
        );
    }
    if !skeleton
        .preserved_state_classes
        .contains(&PreservedStateClass::UserAuthoredFiles)
    {
        push_violation(
            &mut violations,
            "repair_preview.user_authored_files_must_be_preserved",
            &skeleton.skeleton_id,
            "skeleton must preserve user_authored_files",
        );
    }
    if skeleton.destructive_resets_present {
        push_violation(
            &mut violations,
            "repair_preview.destructive_reset_declared",
            &skeleton.skeleton_id,
            "skeleton must not declare a destructive reset",
        );
    }

    if skeleton.checkpoint_disposition_class.requires_checkpoint_ref()
        && skeleton.checkpoint_ref.is_none()
    {
        push_violation(
            &mut violations,
            "repair_preview.checkpoint_ref_required",
            &skeleton.skeleton_id,
            "skeletons with a pre-apply checkpoint disposition must name a checkpoint_ref",
        );
    }
    if !skeleton.checkpoint_disposition_class.requires_checkpoint_ref()
        && skeleton.checkpoint_ref.is_some()
    {
        push_violation(
            &mut violations,
            "repair_preview.checkpoint_ref_unexpected",
            &skeleton.skeleton_id,
            "skeletons without a pre-apply checkpoint disposition must omit checkpoint_ref",
        );
    }

    let preview_authorizes = skeleton.preview_disposition_class.authorizes_apply();
    let preview_forbids = skeleton.preview_disposition_class.forbids_apply();
    if preview_authorizes && skeleton.compensation_class.requires_strong_acknowledgement() {
        // Authorized previews for compensating/manual repairs must travel
        // through a compare-before-apply path first.
        push_violation(
            &mut violations,
            "repair_preview.authorized_requires_comparison",
            &skeleton.skeleton_id,
            "compensating or manual compensation classes must compare before authorizing apply",
        );
    }
    if preview_forbids && !skeleton.affected_object_rows.is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.forbidden_apply_lists_affected_objects",
            &skeleton.skeleton_id,
            "cancelled or escalation-only skeletons must not list affected objects",
        );
    }
    if matches!(
        skeleton.preview_disposition_class,
        RepairPreviewDispositionClass::RefusedNoLocalRepair
    ) && skeleton.checkpoint_disposition_class
        != RepairCheckpointDispositionClass::NoCheckpointEscalationOnly
    {
        push_violation(
            &mut violations,
            "repair_preview.escalation_must_have_no_checkpoint",
            &skeleton.skeleton_id,
            "escalation-only previews must declare no_checkpoint_escalation_only",
        );
    }

    if matches!(
        skeleton.blast_radius_class,
        RepairBlastRadiusClass::NoLocalBlastEscalationOnly
    ) != skeleton.affected_object_rows.is_empty()
    {
        push_violation(
            &mut violations,
            "repair_preview.blast_radius_object_mismatch",
            &skeleton.skeleton_id,
            "no_local_blast_escalation_only requires an empty affected_object_rows list",
        );
    }
    if matches!(
        skeleton.blast_radius_class,
        RepairBlastRadiusClass::SingleObjectClass
    ) && skeleton.affected_object_rows.len() != 1
    {
        push_violation(
            &mut violations,
            "repair_preview.single_blast_requires_single_object",
            &skeleton.skeleton_id,
            "single_object_class blast must list exactly one affected_object_row",
        );
    }
    if matches!(
        skeleton.compensation_class,
        RepairCompensationClass::AuditOnlyNoStateChange
    ) && !skeleton.affected_object_rows.is_empty()
    {
        push_violation(
            &mut violations,
            "repair_preview.audit_only_lists_affected_objects",
            &skeleton.skeleton_id,
            "audit_only_no_state_change compensation must not list affected objects",
        );
    }

    let mut seen = BTreeSet::new();
    for row in &skeleton.affected_object_rows {
        if !seen.insert(row.affected_object_class) {
            push_violation(
                &mut violations,
                "repair_preview.duplicate_affected_object",
                &skeleton.skeleton_id,
                format!(
                    "affected_object_class {:?} appears more than once",
                    row.affected_object_class
                ),
            );
        }
        if row.change_summary.trim().is_empty() {
            push_violation(
                &mut violations,
                "repair_preview.affected_object_summary_empty",
                &skeleton.skeleton_id,
                "affected_object_rows must carry a non-empty change_summary",
            );
        }
    }
    if skeleton.explanation.change_summary.trim().is_empty()
        || skeleton.explanation.preserved_summary.trim().is_empty()
        || skeleton.explanation.compensation_summary.trim().is_empty()
        || skeleton.explanation.checkpoint_summary.trim().is_empty()
        || skeleton.explanation.user_facing_next_step.trim().is_empty()
    {
        push_violation(
            &mut violations,
            "repair_preview.explanation_incomplete",
            &skeleton.skeleton_id,
            "skeleton explanation rows must all be non-empty",
        );
    }

    violations
}

fn validate_comparison(
    comparison: &RepairPreviewComparison,
) -> Vec<RepairPreviewSkeletonViolation> {
    let mut violations = Vec::new();

    if comparison.schema_version != REPAIR_PREVIEW_SKELETON_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "repair_preview.comparison_schema_version",
            &comparison.comparison_id,
            "comparison schema_version must be 1",
        );
    }
    if comparison.record_kind != REPAIR_PREVIEW_COMPARISON_RECORD_KIND {
        push_violation(
            &mut violations,
            "repair_preview.comparison_record_kind",
            &comparison.comparison_id,
            "comparison record_kind must be repair_preview_comparison_record",
        );
    }
    if comparison.comparison_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.comparison_id_empty",
            &comparison.comparison_id,
            "comparison_id must be non-empty",
        );
    }
    if comparison.bound_skeleton_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.comparison_bound_ref_empty",
            &comparison.comparison_id,
            "bound_skeleton_ref must be non-empty",
        );
    }
    if comparison.baseline_skeleton_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.comparison_baseline_ref_empty",
            &comparison.comparison_id,
            "baseline_skeleton_ref must be non-empty",
        );
    }
    if comparison.candidate_skeleton_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.comparison_candidate_ref_empty",
            &comparison.comparison_id,
            "candidate_skeleton_ref must be non-empty",
        );
    }
    if comparison.differing_axes.is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.comparison_axes_empty",
            &comparison.comparison_id,
            "comparison must list at least one differing axis",
        );
    }
    let mut seen = BTreeSet::new();
    for axis in &comparison.differing_axes {
        if !seen.insert(*axis) {
            push_violation(
                &mut violations,
                "repair_preview.comparison_axis_duplicate",
                &comparison.comparison_id,
                format!("differing_axes contains duplicate axis {:?}", axis),
            );
        }
    }
    if comparison.reviewer_summary.trim().is_empty() {
        push_violation(
            &mut violations,
            "repair_preview.comparison_summary_empty",
            &comparison.comparison_id,
            "reviewer_summary must be non-empty",
        );
    }

    violations
}

fn push_violation(
    violations: &mut Vec<RepairPreviewSkeletonViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(RepairPreviewSkeletonViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
