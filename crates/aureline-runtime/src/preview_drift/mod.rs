//! Preview-to-commit guard for high-risk beta actions.
//!
//! This module owns the export-safe guard object that binds a preview and its
//! approval to the exact target, scope, host boundary, route, policy snapshot,
//! lifecycle state, and representation class that the user reviewed. Apply
//! paths call [`evaluate_preview_commit_guard`] immediately before committing
//! so stale previews fail closed with typed reasons instead of refreshing or
//! widening authority silently.
//!
//! The machine-readable boundary lives at
//! [`/schemas/approvals/preview_commit_guard.schema.json`](../../../../schemas/approvals/preview_commit_guard.schema.json)
//! and the reviewer-facing companion doc lives at
//! [`/docs/trust/m3/preview_drift_and_expiry_beta.md`](../../../../docs/trust/m3/preview_drift_and_expiry_beta.md).

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`PreviewCommitGuard`].
pub const PREVIEW_COMMIT_GUARD_RECORD_KIND: &str = "preview_commit_guard_record";

/// Stable record-kind tag for [`PreviewCommitGuardEvaluation`].
pub const PREVIEW_COMMIT_GUARD_EVALUATION_RECORD_KIND: &str =
    "preview_commit_guard_evaluation_record";

/// Stable record-kind tag for [`PreviewCommitSurfaceProjection`].
pub const PREVIEW_COMMIT_SURFACE_PROJECTION_RECORD_KIND: &str =
    "preview_commit_surface_projection_record";

/// Stable record-kind tag for [`PreviewCommitGuardAuditEvent`].
pub const PREVIEW_COMMIT_GUARD_AUDIT_EVENT_RECORD_KIND: &str =
    "preview_commit_guard_audit_event_record";

/// Stable record-kind tag for [`PreviewCommitGuardSupportExport`].
pub const PREVIEW_COMMIT_GUARD_SUPPORT_EXPORT_RECORD_KIND: &str =
    "preview_commit_guard_support_export_record";

/// Schema version for the preview-commit guard family.
pub const PREVIEW_COMMIT_GUARD_SCHEMA_VERSION: u32 = 1;

/// Action classes that must revalidate preview and approval state before
/// committing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardedActionClass {
    /// Destructive or broad local workspace mutation.
    LocalDestructiveMutation,
    /// Remote host, remote helper, managed workspace, or live infrastructure
    /// mutation.
    RemoteMutation,
    /// External publish action such as pushing, commenting, releasing, or
    /// draining a publish-later queue.
    ExternalPublish,
    /// Provider-linked action whose visible representation and provider object
    /// identity both matter.
    ProviderLinkedMutation,
}

impl GuardedActionClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDestructiveMutation => "local_destructive_mutation",
            Self::RemoteMutation => "remote_mutation",
            Self::ExternalPublish => "external_publish",
            Self::ProviderLinkedMutation => "provider_linked_mutation",
        }
    }
}

/// Representation class the user reviewed before approving an apply path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRepresentationClass {
    /// Source diff or patch representation.
    SourceDiff,
    /// Rendered preview representation.
    RenderedPreview,
    /// Structured metadata representation.
    StructuredMetadata,
    /// Raw text representation.
    RawText,
    /// Provider object projection.
    ProviderObject,
    /// Safe-preview snapshot with explicit copy/export representation labels.
    SafePreviewSnapshot,
}

impl PreviewRepresentationClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceDiff => "source_diff",
            Self::RenderedPreview => "rendered_preview",
            Self::StructuredMetadata => "structured_metadata",
            Self::RawText => "raw_text",
            Self::ProviderObject => "provider_object",
            Self::SafePreviewSnapshot => "safe_preview_snapshot",
        }
    }
}

/// Approval posture observed at preview or apply time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewApprovalState {
    /// Approval ticket exists, is unrevoked, and is within its validity window.
    LiveUnrevoked,
    /// No approval ticket was available.
    Missing,
    /// Approval ticket expired before the spend attempt.
    Expired,
    /// Approval ticket was revoked before the spend attempt.
    Revoked,
    /// A remembered decision exists but no fresh short-lived ticket was minted.
    RememberedDecisionOnly,
}

impl PreviewApprovalState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveUnrevoked => "live_unrevoked",
            Self::Missing => "missing",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::RememberedDecisionOnly => "remembered_decision_only",
        }
    }

    /// True only for spendable approval posture.
    pub const fn is_spendable(self) -> bool {
        matches!(self, Self::LiveUnrevoked)
    }
}

/// Lifecycle posture of the preview/apply object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewLifecycleState {
    /// Preview is open but approval is not yet attached.
    PreviewOpen,
    /// Preview has been reviewed and is waiting on approval.
    AwaitingApproval,
    /// Preview and approval are ready to spend.
    ApprovedReady,
    /// Apply is in progress.
    Applying,
    /// Preview was superseded by a later preview or target refresh.
    Superseded,
    /// Target is unavailable at commit time.
    TargetUnavailable,
    /// Provider or remote control plane disconnected before commit.
    ProviderDisconnected,
}

impl PreviewLifecycleState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewOpen => "preview_open",
            Self::AwaitingApproval => "awaiting_approval",
            Self::ApprovedReady => "approved_ready",
            Self::Applying => "applying",
            Self::Superseded => "superseded",
            Self::TargetUnavailable => "target_unavailable",
            Self::ProviderDisconnected => "provider_disconnected",
        }
    }
}

/// Typed reason an apply path was invalidated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewInvalidationReason {
    /// Target set changed after preview.
    TargetSetDrift,
    /// A target identity or fingerprint changed after preview.
    TargetIdentityDrift,
    /// Selected scope, filter, or workset changed after preview.
    SelectedScopeDrift,
    /// Host boundary changed after preview.
    HostBoundaryDrift,
    /// Route or transport binding changed after preview.
    RouteDrift,
    /// Policy epoch or policy snapshot changed after preview.
    PolicyEpochDrift,
    /// Approval ticket changed after preview.
    ApprovalTicketDrift,
    /// Approval ticket was absent at commit time.
    ApprovalTicketMissing,
    /// Approval ticket or preview approval window expired.
    ApprovalTicketExpired,
    /// Approval ticket was revoked before commit.
    ApprovalTicketRevoked,
    /// Remembered decision was present without a fresh ticket.
    RememberedDecisionOnly,
    /// Preview lifecycle changed materially before commit.
    LifecycleDrift,
    /// Reviewed representation class changed before commit.
    RepresentationClassDrift,
    /// Preview freshness window elapsed before commit.
    PreviewExpired,
}

impl PreviewInvalidationReason {
    /// Stable token recorded in schemas, fixtures, CLI output, and audit rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetSetDrift => "target_set_drift",
            Self::TargetIdentityDrift => "target_identity_drift",
            Self::SelectedScopeDrift => "selected_scope_drift",
            Self::HostBoundaryDrift => "host_boundary_drift",
            Self::RouteDrift => "route_drift",
            Self::PolicyEpochDrift => "policy_epoch_drift",
            Self::ApprovalTicketDrift => "approval_ticket_drift",
            Self::ApprovalTicketMissing => "approval_ticket_missing",
            Self::ApprovalTicketExpired => "approval_ticket_expired",
            Self::ApprovalTicketRevoked => "approval_ticket_revoked",
            Self::RememberedDecisionOnly => "remembered_decision_only",
            Self::LifecycleDrift => "lifecycle_drift",
            Self::RepresentationClassDrift => "representation_class_drift",
            Self::PreviewExpired => "preview_expired",
        }
    }

    /// Short explanation suitable for product surfaces and CLI/headless output.
    pub const fn explanation(self) -> &'static str {
        match self {
            Self::TargetSetDrift => "the reviewed target set changed",
            Self::TargetIdentityDrift => "a reviewed target identity changed",
            Self::SelectedScopeDrift => "the reviewed scope or filter changed",
            Self::HostBoundaryDrift => "the host boundary changed",
            Self::RouteDrift => "the route or transport binding changed",
            Self::PolicyEpochDrift => "the policy snapshot changed",
            Self::ApprovalTicketDrift => "the approval ticket binding changed",
            Self::ApprovalTicketMissing => "a fresh approval ticket is missing",
            Self::ApprovalTicketExpired => "the approval ticket expired",
            Self::ApprovalTicketRevoked => "the approval ticket was revoked",
            Self::RememberedDecisionOnly => "a remembered decision did not mint a fresh ticket",
            Self::LifecycleDrift => "the preview lifecycle changed",
            Self::RepresentationClassDrift => "the reviewed representation changed",
            Self::PreviewExpired => "the preview freshness window elapsed",
        }
    }
}

/// Final commit admission decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCommitAdmissionDecision {
    /// Guard matched the current apply context.
    AdmitApply,
    /// Guard drifted and the user must re-review before apply.
    BlockRequireReview,
}

impl PreviewCommitAdmissionDecision {
    /// Stable token recorded in schemas, fixtures, CLI output, and support
    /// exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmitApply => "admit_apply",
            Self::BlockRequireReview => "block_require_review",
        }
    }
}

/// Audit event class emitted by the guard evaluator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCommitAuditEventClass {
    /// The guard matched the current commit context.
    PreviewCommitGuardAdmitted,
    /// The guard invalidated the commit path.
    PreviewCommitGuardInvalidated,
}

impl PreviewCommitAuditEventClass {
    /// Stable token recorded in audit rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewCommitGuardAdmitted => "preview_commit_guard_admitted",
            Self::PreviewCommitGuardInvalidated => "preview_commit_guard_invalidated",
        }
    }
}

/// Target binding captured by the preview guard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewTargetBinding {
    /// Opaque target reference.
    pub target_ref: String,
    /// Export-safe target fingerprint.
    pub target_hash: String,
}

impl PreviewTargetBinding {
    /// Builds a target binding from an already computed export-safe hash.
    pub fn from_hash(target_ref: impl Into<String>, target_hash: impl Into<String>) -> Self {
        Self {
            target_ref: target_ref.into(),
            target_hash: target_hash.into(),
        }
    }

    /// Builds a target binding by hashing export-safe target attributes.
    pub fn new(target_ref: impl Into<String>, fingerprint_parts: &[&str]) -> Self {
        Self {
            target_ref: target_ref.into(),
            target_hash: stable_guard_digest(fingerprint_parts),
        }
    }

    fn token(&self) -> String {
        format!("{}@{}", self.target_ref, self.target_hash)
    }
}

/// Scalar binding captured by the preview guard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScalarBinding {
    /// Opaque binding reference.
    pub binding_ref: String,
    /// Export-safe binding fingerprint.
    pub binding_hash: String,
}

impl PreviewScalarBinding {
    /// Builds a scalar binding from an already computed export-safe hash.
    pub fn from_hash(binding_ref: impl Into<String>, binding_hash: impl Into<String>) -> Self {
        Self {
            binding_ref: binding_ref.into(),
            binding_hash: binding_hash.into(),
        }
    }

    /// Builds a scalar binding by hashing export-safe attributes.
    pub fn new(binding_ref: impl Into<String>, fingerprint_parts: &[&str]) -> Self {
        Self {
            binding_ref: binding_ref.into(),
            binding_hash: stable_guard_digest(fingerprint_parts),
        }
    }

    fn token(&self) -> String {
        format!("{}@{}", self.binding_ref, self.binding_hash)
    }
}

/// Policy snapshot binding captured at preview time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySnapshotBinding {
    /// Opaque policy snapshot reference.
    pub policy_snapshot_ref: String,
    /// Monotonic policy epoch.
    pub policy_epoch: u64,
    /// Export-safe policy snapshot fingerprint.
    pub policy_snapshot_hash: String,
}

impl PolicySnapshotBinding {
    /// Builds a policy snapshot binding by hashing export-safe attributes.
    pub fn new(
        policy_snapshot_ref: impl Into<String>,
        policy_epoch: u64,
        fingerprint_parts: &[&str],
    ) -> Self {
        Self {
            policy_snapshot_ref: policy_snapshot_ref.into(),
            policy_epoch,
            policy_snapshot_hash: stable_guard_digest(fingerprint_parts),
        }
    }

    fn token(&self) -> String {
        format!(
            "{}#{}@{}",
            self.policy_snapshot_ref, self.policy_epoch, self.policy_snapshot_hash
        )
    }
}

/// Approval ticket binding captured at preview time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketBinding {
    /// Opaque approval ticket reference, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Export-safe approval ticket fingerprint.
    pub approval_ticket_hash: String,
    /// Approval posture.
    pub approval_state: PreviewApprovalState,
    /// Ticket expiry timestamp, when bounded by wall-clock freshness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl ApprovalTicketBinding {
    /// Builds a live approval ticket binding.
    pub fn live(
        approval_ticket_ref: impl Into<String>,
        fingerprint_parts: &[&str],
        expires_at: Option<String>,
    ) -> Self {
        Self {
            approval_ticket_ref: Some(approval_ticket_ref.into()),
            approval_ticket_hash: stable_guard_digest(fingerprint_parts),
            approval_state: PreviewApprovalState::LiveUnrevoked,
            expires_at,
        }
    }

    /// Builds a non-spendable approval binding with a typed state.
    pub fn non_spendable(
        approval_state: PreviewApprovalState,
        approval_ticket_ref: Option<String>,
        fingerprint_parts: &[&str],
        expires_at: Option<String>,
    ) -> Self {
        Self {
            approval_ticket_ref,
            approval_ticket_hash: stable_guard_digest(fingerprint_parts),
            approval_state,
            expires_at,
        }
    }

    fn token(&self) -> String {
        format!(
            "{}#{}@{}",
            self.approval_ticket_ref.as_deref().unwrap_or("<none>"),
            self.approval_state.as_str(),
            self.approval_ticket_hash
        )
    }
}

/// Full guard basis captured at preview and approval time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitBasis {
    /// Target bindings reviewed by the user.
    pub target_bindings: Vec<PreviewTargetBinding>,
    /// Selected workset, filter, or explicit scope binding.
    pub selected_scope: PreviewScalarBinding,
    /// Host boundary binding.
    pub host_boundary: PreviewScalarBinding,
    /// Route or transport binding.
    pub route_binding: PreviewScalarBinding,
    /// Policy snapshot binding.
    pub policy_snapshot: PolicySnapshotBinding,
    /// Approval ticket binding.
    pub approval_ticket: ApprovalTicketBinding,
    /// Representation class the user reviewed.
    pub representation_class: PreviewRepresentationClass,
    /// Lifecycle state captured for the preview/apply object.
    pub lifecycle_state: PreviewLifecycleState,
}

impl PreviewCommitBasis {
    /// Builds a guard basis and canonicalizes target ordering by target ref.
    pub fn new(
        target_bindings: Vec<PreviewTargetBinding>,
        selected_scope: PreviewScalarBinding,
        host_boundary: PreviewScalarBinding,
        route_binding: PreviewScalarBinding,
        policy_snapshot: PolicySnapshotBinding,
        approval_ticket: ApprovalTicketBinding,
        representation_class: PreviewRepresentationClass,
        lifecycle_state: PreviewLifecycleState,
    ) -> Self {
        let mut target_bindings = target_bindings;
        target_bindings.sort_by(|left, right| left.target_ref.cmp(&right.target_ref));
        Self {
            target_bindings,
            selected_scope,
            host_boundary,
            route_binding,
            policy_snapshot,
            approval_ticket,
            representation_class,
            lifecycle_state,
        }
    }

    /// Computes the stable basis fingerprint.
    pub fn basis_hash(&self) -> String {
        let mut parts = vec![
            "preview_commit_basis:v1".to_owned(),
            self.selected_scope.token(),
            self.host_boundary.token(),
            self.route_binding.token(),
            self.policy_snapshot.token(),
            self.approval_ticket.token(),
            self.representation_class.as_str().to_owned(),
            self.lifecycle_state.as_str().to_owned(),
        ];
        for target in &self.target_bindings {
            parts.push(target.token());
        }
        stable_guard_digest_owned(&parts)
    }
}

/// Stable guard object persisted with a preview and approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitGuard {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Guard id.
    pub guard_id: String,
    /// Preview object this guard protects.
    pub preview_ref: String,
    /// Command or action reference.
    pub command_id_ref: String,
    /// Guarded action class.
    pub action_class: GuardedActionClass,
    /// Time the guard was created.
    pub created_at: String,
    /// Optional preview freshness expiry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_expires_at: Option<String>,
    /// Captured preview basis.
    pub basis: PreviewCommitBasis,
    /// Stable hash of the captured basis.
    pub basis_hash: String,
}

impl PreviewCommitGuard {
    /// Builds a new preview-commit guard.
    pub fn new(
        guard_id: impl Into<String>,
        preview_ref: impl Into<String>,
        command_id_ref: impl Into<String>,
        action_class: GuardedActionClass,
        created_at: impl Into<String>,
        preview_expires_at: Option<String>,
        basis: PreviewCommitBasis,
    ) -> Self {
        let basis_hash = basis.basis_hash();
        Self {
            record_kind: PREVIEW_COMMIT_GUARD_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
            guard_id: guard_id.into(),
            preview_ref: preview_ref.into(),
            command_id_ref: command_id_ref.into(),
            action_class,
            created_at: created_at.into(),
            preview_expires_at,
            basis,
            basis_hash,
        }
    }
}

/// Current apply context compared against a stored [`PreviewCommitGuard`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitContext {
    /// Observation timestamp.
    pub observed_at: String,
    /// Current basis at commit time.
    pub basis: PreviewCommitBasis,
    /// Stable hash of the current basis.
    pub basis_hash: String,
}

impl PreviewCommitContext {
    /// Builds a current apply context.
    pub fn new(observed_at: impl Into<String>, basis: PreviewCommitBasis) -> Self {
        let basis_hash = basis.basis_hash();
        Self {
            observed_at: observed_at.into(),
            basis,
            basis_hash,
        }
    }
}

/// One typed invalidation row emitted by the guard evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewInvalidationRow {
    /// Typed invalidation reason.
    pub reason: PreviewInvalidationReason,
    /// Stable reason token.
    pub reason_token: String,
    /// Stored value token from the preview guard.
    pub stored_value_token: String,
    /// Current value token at commit time.
    pub current_value_token: String,
    /// Short explanation for user-facing and headless output.
    pub explanation: String,
    /// True because every invalidation requires re-review.
    pub requires_re_review: bool,
}

impl PreviewInvalidationRow {
    fn new(
        reason: PreviewInvalidationReason,
        stored_value_token: impl Into<String>,
        current_value_token: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            reason_token: reason.as_str().to_owned(),
            stored_value_token: stored_value_token.into(),
            current_value_token: current_value_token.into(),
            explanation: reason.explanation().to_owned(),
            requires_re_review: true,
        }
    }
}

/// Product-surface projection for a guard evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Guard id.
    pub guard_id: String,
    /// Preview ref.
    pub preview_ref: String,
    /// Severity token.
    pub severity_token: String,
    /// Stable action token for the next safe step.
    pub next_action_token: String,
    /// Human-readable summary.
    pub summary: String,
    /// Reason tokens rendered by product surfaces.
    pub reason_tokens: Vec<String>,
}

/// CLI/headless output projection for a guard evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitCliOutput {
    /// Process-style exit code.
    pub exit_code: u8,
    /// Machine-readable status token.
    pub status_token: String,
    /// Reason tokens suitable for JSON output.
    pub reason_tokens: Vec<String>,
    /// Human-readable message for plain output.
    pub message: String,
}

/// Audit event emitted by the guard evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitGuardAuditEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Audit event id.
    pub audit_event_id: String,
    /// Event class.
    pub event_class: PreviewCommitAuditEventClass,
    /// Stable event token.
    pub event_token: String,
    /// Guard id.
    pub guard_id: String,
    /// Preview ref.
    pub preview_ref: String,
    /// Evaluation id.
    pub evaluation_ref: String,
    /// Event timestamp.
    pub occurred_at: String,
    /// Reason tokens attached to the event.
    pub reason_tokens: Vec<String>,
    /// True because the event carries metadata, tokens, and refs only.
    pub export_safe: bool,
}

/// Full evaluation record for a stored preview guard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitGuardEvaluation {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Evaluation id.
    pub evaluation_id: String,
    /// Guard id.
    pub guard_id: String,
    /// Preview ref.
    pub preview_ref: String,
    /// Command or action reference.
    pub command_id_ref: String,
    /// Guarded action class.
    pub action_class: GuardedActionClass,
    /// Evaluation timestamp.
    pub evaluated_at: String,
    /// Stored basis hash.
    pub stored_basis_hash: String,
    /// Current basis hash.
    pub current_basis_hash: String,
    /// Final admission decision.
    pub admission_decision: PreviewCommitAdmissionDecision,
    /// True when apply must be blocked.
    pub blocks_apply: bool,
    /// True when the next safe route is re-review.
    pub requires_re_review: bool,
    /// False for every invalidation; stale authority is never silently
    /// refreshed.
    pub may_auto_refresh: bool,
    /// Typed invalidation rows.
    pub invalidations: Vec<PreviewInvalidationRow>,
    /// Product-surface projection.
    pub surface_projection: PreviewCommitSurfaceProjection,
    /// CLI/headless projection.
    pub cli_output: PreviewCommitCliOutput,
    /// Audit events emitted for this evaluation.
    pub audit_events: Vec<PreviewCommitGuardAuditEvent>,
}

impl PreviewCommitGuardEvaluation {
    /// Stable reason-token list in evaluation order.
    pub fn reason_tokens(&self) -> Vec<String> {
        self.invalidations
            .iter()
            .map(|row| row.reason_token.clone())
            .collect()
    }
}

/// Evaluates whether a stored preview guard still admits the commit path.
pub fn evaluate_preview_commit_guard(
    guard: &PreviewCommitGuard,
    current: &PreviewCommitContext,
    evaluation_id: impl Into<String>,
    evaluated_at: impl Into<String>,
) -> PreviewCommitGuardEvaluation {
    let evaluation_id = evaluation_id.into();
    let evaluated_at = evaluated_at.into();
    let mut invalidations = Vec::new();

    compare_target_bindings(
        &guard.basis.target_bindings,
        &current.basis.target_bindings,
        &mut invalidations,
    );
    compare_scalar_binding(
        PreviewInvalidationReason::SelectedScopeDrift,
        &guard.basis.selected_scope,
        &current.basis.selected_scope,
        &mut invalidations,
    );
    compare_scalar_binding(
        PreviewInvalidationReason::HostBoundaryDrift,
        &guard.basis.host_boundary,
        &current.basis.host_boundary,
        &mut invalidations,
    );
    compare_scalar_binding(
        PreviewInvalidationReason::RouteDrift,
        &guard.basis.route_binding,
        &current.basis.route_binding,
        &mut invalidations,
    );
    compare_policy_snapshot(
        &guard.basis.policy_snapshot,
        &current.basis.policy_snapshot,
        &mut invalidations,
    );
    compare_approval_ticket(
        &guard.basis.approval_ticket,
        &current.basis.approval_ticket,
        &evaluated_at,
        &mut invalidations,
    );

    if guard.basis.lifecycle_state != current.basis.lifecycle_state {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::LifecycleDrift,
            guard.basis.lifecycle_state.as_str(),
            current.basis.lifecycle_state.as_str(),
        ));
    }
    if guard.basis.representation_class != current.basis.representation_class {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::RepresentationClassDrift,
            guard.basis.representation_class.as_str(),
            current.basis.representation_class.as_str(),
        ));
    }
    if timestamp_expired(guard.preview_expires_at.as_deref(), &evaluated_at) {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::PreviewExpired,
            guard.preview_expires_at.as_deref().unwrap_or("<none>"),
            &evaluated_at,
        ));
    }

    let admission_decision = if invalidations.is_empty() {
        PreviewCommitAdmissionDecision::AdmitApply
    } else {
        PreviewCommitAdmissionDecision::BlockRequireReview
    };
    let blocks_apply = admission_decision == PreviewCommitAdmissionDecision::BlockRequireReview;
    let reason_tokens: Vec<String> = invalidations
        .iter()
        .map(|row| row.reason_token.clone())
        .collect();
    let surface_projection = surface_projection_for(guard, &reason_tokens, blocks_apply);
    let cli_output = cli_output_for(&reason_tokens, blocks_apply);
    let event_class = if blocks_apply {
        PreviewCommitAuditEventClass::PreviewCommitGuardInvalidated
    } else {
        PreviewCommitAuditEventClass::PreviewCommitGuardAdmitted
    };
    let audit_events = vec![PreviewCommitGuardAuditEvent {
        record_kind: PREVIEW_COMMIT_GUARD_AUDIT_EVENT_RECORD_KIND.to_owned(),
        schema_version: PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
        audit_event_id: format!("audit:{}:{}", evaluation_id, event_class.as_str()),
        event_class,
        event_token: event_class.as_str().to_owned(),
        guard_id: guard.guard_id.clone(),
        preview_ref: guard.preview_ref.clone(),
        evaluation_ref: evaluation_id.clone(),
        occurred_at: evaluated_at.clone(),
        reason_tokens: reason_tokens.clone(),
        export_safe: true,
    }];

    PreviewCommitGuardEvaluation {
        record_kind: PREVIEW_COMMIT_GUARD_EVALUATION_RECORD_KIND.to_owned(),
        schema_version: PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
        evaluation_id,
        guard_id: guard.guard_id.clone(),
        preview_ref: guard.preview_ref.clone(),
        command_id_ref: guard.command_id_ref.clone(),
        action_class: guard.action_class,
        evaluated_at,
        stored_basis_hash: guard.basis_hash.clone(),
        current_basis_hash: current.basis_hash.clone(),
        admission_decision,
        blocks_apply,
        requires_re_review: blocks_apply,
        may_auto_refresh: false,
        invalidations,
        surface_projection,
        cli_output,
        audit_events,
    }
}

/// Support/export packet carrying guard evaluations and audit rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCommitGuardSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Guard evaluations included in this export.
    pub evaluations: Vec<PreviewCommitGuardEvaluation>,
    /// Flattened audit events included in this export.
    pub audit_events: Vec<PreviewCommitGuardAuditEvent>,
    /// Count of blocked evaluations.
    pub blocked_evaluation_count: u32,
    /// True because the export carries metadata, tokens, and refs only.
    pub redaction_safe: bool,
}

impl PreviewCommitGuardSupportExport {
    /// Builds a support export from guard evaluations.
    pub fn from_evaluations(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        evaluations: &[PreviewCommitGuardEvaluation],
    ) -> Self {
        let audit_events = evaluations
            .iter()
            .flat_map(|evaluation| evaluation.audit_events.clone())
            .collect::<Vec<_>>();
        let blocked_evaluation_count = evaluations
            .iter()
            .filter(|evaluation| evaluation.blocks_apply)
            .count() as u32;
        Self {
            record_kind: PREVIEW_COMMIT_GUARD_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            evaluations: evaluations.to_vec(),
            audit_events,
            blocked_evaluation_count,
            redaction_safe: true,
        }
    }

    /// Deterministic plaintext summary for support packets.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Preview commit guard support export\n");
        out.push_str(&format!(
            "export_id: {}\ngenerated_at: {}\nblocked_evaluation_count: {}\n",
            self.export_id, self.generated_at, self.blocked_evaluation_count
        ));
        for evaluation in &self.evaluations {
            out.push_str(&format!(
                "evaluation: {} guard={} decision={} reasons={}\n",
                evaluation.evaluation_id,
                evaluation.guard_id,
                evaluation.admission_decision.as_str(),
                evaluation.reason_tokens().join(",")
            ));
        }
        out
    }
}

/// Seeded replay scenarios for destructive, remote, publish, and provider
/// linked guard failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCommitGuardScenario {
    /// Destructive local apply where a target moved after preview.
    DestructiveTargetMoved,
    /// Remote mutation where route and host boundary drifted.
    RemoteRouteHostDrift,
    /// Publish action where approval expired before commit.
    PublishApprovalExpired,
    /// Provider-linked action where the representation class changed.
    ProviderRepresentationChanged,
}

impl PreviewCommitGuardScenario {
    /// All seeded scenarios in fixture order.
    pub const ALL: [Self; 4] = [
        Self::DestructiveTargetMoved,
        Self::RemoteRouteHostDrift,
        Self::PublishApprovalExpired,
        Self::ProviderRepresentationChanged,
    ];

    /// Stable scenario token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DestructiveTargetMoved => "destructive_target_moved",
            Self::RemoteRouteHostDrift => "remote_route_host_drift",
            Self::PublishApprovalExpired => "publish_approval_expired",
            Self::ProviderRepresentationChanged => "provider_representation_changed",
        }
    }
}

/// Builds the seeded guard, current context, and evaluation for one scenario.
pub fn seeded_preview_commit_guard_scenario(
    scenario: PreviewCommitGuardScenario,
) -> (
    PreviewCommitGuard,
    PreviewCommitContext,
    PreviewCommitGuardEvaluation,
) {
    let action_class = match scenario {
        PreviewCommitGuardScenario::DestructiveTargetMoved => {
            GuardedActionClass::LocalDestructiveMutation
        }
        PreviewCommitGuardScenario::RemoteRouteHostDrift => GuardedActionClass::RemoteMutation,
        PreviewCommitGuardScenario::PublishApprovalExpired => GuardedActionClass::ExternalPublish,
        PreviewCommitGuardScenario::ProviderRepresentationChanged => {
            GuardedActionClass::ProviderLinkedMutation
        }
    };
    let stored = seeded_basis(action_class, PreviewRepresentationClass::SourceDiff);
    let guard = PreviewCommitGuard::new(
        format!("preview-guard:{}", scenario.as_str()),
        format!("preview:{}", scenario.as_str()),
        format!("command:{}", action_class.as_str()),
        action_class,
        "2026-05-17T22:00:00Z",
        Some("2026-05-17T22:45:00Z".to_owned()),
        stored,
    );

    let current_basis = match scenario {
        PreviewCommitGuardScenario::DestructiveTargetMoved => {
            let mut basis = guard.basis.clone();
            basis.target_bindings = vec![PreviewTargetBinding::from_hash(
                "target:workspace/src/new.rs",
                stable_guard_digest(&["target", "workspace", "src/new.rs", "rev2"]),
            )];
            PreviewCommitBasis::new(
                basis.target_bindings,
                basis.selected_scope,
                basis.host_boundary,
                basis.route_binding,
                basis.policy_snapshot,
                basis.approval_ticket,
                basis.representation_class,
                basis.lifecycle_state,
            )
        }
        PreviewCommitGuardScenario::RemoteRouteHostDrift => {
            let mut basis = guard.basis.clone();
            basis.host_boundary = PreviewScalarBinding::new(
                "host:remote:successor",
                &["host", "remote", "successor"],
            );
            basis.route_binding =
                PreviewScalarBinding::new("route:ssh:reconnected", &["route", "ssh", "next"]);
            PreviewCommitBasis::new(
                basis.target_bindings,
                basis.selected_scope,
                basis.host_boundary,
                basis.route_binding,
                basis.policy_snapshot,
                basis.approval_ticket,
                basis.representation_class,
                basis.lifecycle_state,
            )
        }
        PreviewCommitGuardScenario::PublishApprovalExpired => {
            let mut basis = guard.basis.clone();
            basis.approval_ticket = ApprovalTicketBinding::non_spendable(
                PreviewApprovalState::Expired,
                Some("approval:publish:1".to_owned()),
                &["approval", "publish", "expired"],
                Some("2026-05-17T22:10:00Z".to_owned()),
            );
            PreviewCommitBasis::new(
                basis.target_bindings,
                basis.selected_scope,
                basis.host_boundary,
                basis.route_binding,
                basis.policy_snapshot,
                basis.approval_ticket,
                basis.representation_class,
                basis.lifecycle_state,
            )
        }
        PreviewCommitGuardScenario::ProviderRepresentationChanged => {
            let mut basis = guard.basis.clone();
            basis.representation_class = PreviewRepresentationClass::ProviderObject;
            PreviewCommitBasis::new(
                basis.target_bindings,
                basis.selected_scope,
                basis.host_boundary,
                basis.route_binding,
                basis.policy_snapshot,
                basis.approval_ticket,
                basis.representation_class,
                basis.lifecycle_state,
            )
        }
    };
    let current = PreviewCommitContext::new("2026-05-17T22:20:00Z", current_basis);
    let evaluation = evaluate_preview_commit_guard(
        &guard,
        &current,
        format!("preview-eval:{}", scenario.as_str()),
        "2026-05-17T22:20:00Z",
    );
    (guard, current, evaluation)
}

fn seeded_basis(
    action_class: GuardedActionClass,
    representation_class: PreviewRepresentationClass,
) -> PreviewCommitBasis {
    let action_token = action_class.as_str();
    PreviewCommitBasis::new(
        vec![PreviewTargetBinding::new(
            "target:workspace/src/old.rs",
            &["target", action_token, "workspace", "src/old.rs", "rev1"],
        )],
        PreviewScalarBinding::new(
            "scope:selected-review",
            &["scope", action_token, "selected-review"],
        ),
        PreviewScalarBinding::new("host:local", &["host", action_token, "local"]),
        PreviewScalarBinding::new("route:primary", &["route", action_token, "primary"]),
        PolicySnapshotBinding::new(
            "policy:org:7",
            7,
            &["policy", action_token, "org", "epoch7"],
        ),
        ApprovalTicketBinding::live(
            format!("approval:{}:1", action_token),
            &["approval", action_token, "live"],
            Some("2026-05-17T22:30:00Z".to_owned()),
        ),
        representation_class,
        PreviewLifecycleState::ApprovedReady,
    )
}

fn compare_target_bindings(
    stored: &[PreviewTargetBinding],
    current: &[PreviewTargetBinding],
    invalidations: &mut Vec<PreviewInvalidationRow>,
) {
    let stored_refs: Vec<&str> = stored
        .iter()
        .map(|target| target.target_ref.as_str())
        .collect();
    let current_refs: Vec<&str> = current
        .iter()
        .map(|target| target.target_ref.as_str())
        .collect();
    if stored_refs != current_refs {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::TargetSetDrift,
            target_set_token(stored),
            target_set_token(current),
        ));
        return;
    }
    for (stored_target, current_target) in stored.iter().zip(current) {
        if stored_target.target_hash != current_target.target_hash {
            invalidations.push(PreviewInvalidationRow::new(
                PreviewInvalidationReason::TargetIdentityDrift,
                stored_target.token(),
                current_target.token(),
            ));
        }
    }
}

fn compare_scalar_binding(
    reason: PreviewInvalidationReason,
    stored: &PreviewScalarBinding,
    current: &PreviewScalarBinding,
    invalidations: &mut Vec<PreviewInvalidationRow>,
) {
    if stored != current {
        invalidations.push(PreviewInvalidationRow::new(
            reason,
            stored.token(),
            current.token(),
        ));
    }
}

fn compare_policy_snapshot(
    stored: &PolicySnapshotBinding,
    current: &PolicySnapshotBinding,
    invalidations: &mut Vec<PreviewInvalidationRow>,
) {
    if stored != current {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::PolicyEpochDrift,
            stored.token(),
            current.token(),
        ));
    }
}

fn compare_approval_ticket(
    stored: &ApprovalTicketBinding,
    current: &ApprovalTicketBinding,
    evaluated_at: &str,
    invalidations: &mut Vec<PreviewInvalidationRow>,
) {
    let mut expired_reported = false;
    if !stored.approval_state.is_spendable() {
        let reason = reason_for_approval_state(stored.approval_state);
        expired_reported |= reason == PreviewInvalidationReason::ApprovalTicketExpired;
        invalidations.push(PreviewInvalidationRow::new(
            reason,
            stored.token(),
            current.token(),
        ));
    }
    if !current.approval_state.is_spendable() {
        let reason = reason_for_approval_state(current.approval_state);
        expired_reported |= reason == PreviewInvalidationReason::ApprovalTicketExpired;
        invalidations.push(PreviewInvalidationRow::new(
            reason,
            stored.token(),
            current.token(),
        ));
    }
    if !expired_reported
        && (timestamp_expired(stored.expires_at.as_deref(), evaluated_at)
            || timestamp_expired(current.expires_at.as_deref(), evaluated_at))
    {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::ApprovalTicketExpired,
            stored.expires_at.as_deref().unwrap_or("<none>"),
            current.expires_at.as_deref().unwrap_or("<none>"),
        ));
    }
    if stored.approval_ticket_ref != current.approval_ticket_ref
        || stored.approval_ticket_hash != current.approval_ticket_hash
    {
        invalidations.push(PreviewInvalidationRow::new(
            PreviewInvalidationReason::ApprovalTicketDrift,
            stored.token(),
            current.token(),
        ));
    }
}

fn reason_for_approval_state(state: PreviewApprovalState) -> PreviewInvalidationReason {
    match state {
        PreviewApprovalState::LiveUnrevoked => PreviewInvalidationReason::ApprovalTicketDrift,
        PreviewApprovalState::Missing => PreviewInvalidationReason::ApprovalTicketMissing,
        PreviewApprovalState::Expired => PreviewInvalidationReason::ApprovalTicketExpired,
        PreviewApprovalState::Revoked => PreviewInvalidationReason::ApprovalTicketRevoked,
        PreviewApprovalState::RememberedDecisionOnly => {
            PreviewInvalidationReason::RememberedDecisionOnly
        }
    }
}

fn surface_projection_for(
    guard: &PreviewCommitGuard,
    reason_tokens: &[String],
    blocks_apply: bool,
) -> PreviewCommitSurfaceProjection {
    let (severity_token, next_action_token, summary) = if blocks_apply {
        (
            "blocking",
            "review_again",
            format!(
                "Apply blocked for {}; review again because {}.",
                guard.preview_ref,
                reason_tokens.join(", ")
            ),
        )
    } else {
        (
            "info",
            "apply",
            format!("Apply admitted for {}.", guard.preview_ref),
        )
    };
    PreviewCommitSurfaceProjection {
        record_kind: PREVIEW_COMMIT_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
        guard_id: guard.guard_id.clone(),
        preview_ref: guard.preview_ref.clone(),
        severity_token: severity_token.to_owned(),
        next_action_token: next_action_token.to_owned(),
        summary,
        reason_tokens: reason_tokens.to_vec(),
    }
}

fn cli_output_for(reason_tokens: &[String], blocks_apply: bool) -> PreviewCommitCliOutput {
    if blocks_apply {
        PreviewCommitCliOutput {
            exit_code: 78,
            status_token: "blocked_require_review".to_owned(),
            reason_tokens: reason_tokens.to_vec(),
            message: format!(
                "apply blocked; review again required ({})",
                reason_tokens.join(",")
            ),
        }
    } else {
        PreviewCommitCliOutput {
            exit_code: 0,
            status_token: "admitted".to_owned(),
            reason_tokens: Vec::new(),
            message: "apply admitted".to_owned(),
        }
    }
}

fn target_set_token(targets: &[PreviewTargetBinding]) -> String {
    targets
        .iter()
        .map(PreviewTargetBinding::token)
        .collect::<Vec<_>>()
        .join(",")
}

fn timestamp_expired(expires_at: Option<&str>, evaluated_at: &str) -> bool {
    expires_at
        .map(|expires_at| expires_at <= evaluated_at)
        .unwrap_or(false)
}

fn stable_guard_digest(parts: &[&str]) -> String {
    let mut hash = FNV_OFFSET_BASIS;
    for part in parts {
        fnv_update(&mut hash, part.as_bytes());
        fnv_update(&mut hash, &[0xff]);
    }
    format!("fnv1a64:{hash:016x}")
}

fn stable_guard_digest_owned(parts: &[String]) -> String {
    let mut hash = FNV_OFFSET_BASIS;
    for part in parts {
        fnv_update(&mut hash, part.as_bytes());
        fnv_update(&mut hash, &[0xff]);
    }
    format!("fnv1a64:{hash:016x}")
}

fn fnv_update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_guard_and_context() -> (PreviewCommitGuard, PreviewCommitContext) {
        let basis = seeded_basis(
            GuardedActionClass::LocalDestructiveMutation,
            PreviewRepresentationClass::SourceDiff,
        );
        let guard = PreviewCommitGuard::new(
            "preview-guard:fresh",
            "preview:fresh",
            "command:local_destructive_mutation",
            GuardedActionClass::LocalDestructiveMutation,
            "2026-05-17T22:00:00Z",
            Some("2026-05-17T22:45:00Z".to_owned()),
            basis.clone(),
        );
        let context = PreviewCommitContext::new("2026-05-17T22:10:00Z", basis);
        (guard, context)
    }

    #[test]
    fn fresh_guard_admits_apply() {
        let (guard, current) = fresh_guard_and_context();
        let evaluation =
            evaluate_preview_commit_guard(&guard, &current, "eval:fresh", "2026-05-17T22:10:00Z");
        assert_eq!(
            evaluation.admission_decision,
            PreviewCommitAdmissionDecision::AdmitApply
        );
        assert!(!evaluation.blocks_apply);
        assert!(evaluation.invalidations.is_empty());
        assert_eq!(evaluation.cli_output.exit_code, 0);
    }

    #[test]
    fn target_hash_change_invalidates_apply() {
        let (guard, mut current) = fresh_guard_and_context();
        current.basis.target_bindings[0].target_hash = stable_guard_digest(&["target", "changed"]);
        current.basis_hash = current.basis.basis_hash();
        let evaluation = evaluate_preview_commit_guard(
            &guard,
            &current,
            "eval:target-drift",
            "2026-05-17T22:10:00Z",
        );
        assert!(evaluation.blocks_apply);
        assert!(evaluation
            .reason_tokens()
            .contains(&"target_identity_drift".to_owned()));
        assert_eq!(evaluation.cli_output.exit_code, 78);
        assert!(!evaluation.may_auto_refresh);
    }

    #[test]
    fn expired_approval_invalidates_apply() {
        let (guard, current) = fresh_guard_and_context();
        let evaluation = evaluate_preview_commit_guard(
            &guard,
            &current,
            "eval:approval-expired",
            "2026-05-17T22:40:00Z",
        );
        assert!(evaluation.blocks_apply);
        assert!(evaluation
            .reason_tokens()
            .contains(&"approval_ticket_expired".to_owned()));
    }

    #[test]
    fn remembered_decision_without_fresh_ticket_invalidates_apply() {
        let (guard, mut current) = fresh_guard_and_context();
        current.basis.approval_ticket.approval_state = PreviewApprovalState::RememberedDecisionOnly;
        current.basis_hash = current.basis.basis_hash();
        let evaluation = evaluate_preview_commit_guard(
            &guard,
            &current,
            "eval:remembered-decision",
            "2026-05-17T22:10:00Z",
        );
        assert!(evaluation.blocks_apply);
        assert!(evaluation
            .reason_tokens()
            .contains(&"remembered_decision_only".to_owned()));
    }

    #[test]
    fn support_export_flattens_audit_events() {
        let (_, _, evaluation) =
            seeded_preview_commit_guard_scenario(PreviewCommitGuardScenario::RemoteRouteHostDrift);
        let export = PreviewCommitGuardSupportExport::from_evaluations(
            "support:preview-guard:test",
            "2026-05-17T22:30:00Z",
            &[evaluation],
        );
        assert_eq!(export.blocked_evaluation_count, 1);
        assert_eq!(export.audit_events.len(), 1);
        assert!(export.render_plaintext().contains("route_drift"));
        assert!(export.redaction_safe);
    }
}
