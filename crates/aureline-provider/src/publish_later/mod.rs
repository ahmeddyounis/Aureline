//! Publish-later queue alpha records for connected-provider surfaces.
//!
//! This module consumes the queue vocabulary frozen in
//! `/schemas/providers/publish_later_record.schema.json` and
//! `/schemas/providers/deferred_publish_queue_item.schema.json` by reference.
//! It does not implement a queue drainer; it validates that queued provider
//! mutations keep dependency order, stale-target risk, reauth or rescope needs,
//! and export posture visible.

use serde::{Deserialize, Serialize};

use crate::registry::{
    ActorScope, FreshnessTruth, ProviderSurfaceClass, RedactionClass, StaleTargetRiskClass,
    TargetRef,
};

/// Schema version for publish-later queue alpha records.
pub const PUBLISH_LATER_QUEUE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`PublishLaterQueueAlphaItem`].
pub const PUBLISH_LATER_QUEUE_ALPHA_ITEM_RECORD_KIND: &str =
    "publish_later_queue_alpha_item_record";

/// Publish-later queue item used by the provider registry alpha.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaterQueueAlphaItem {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the queue item.
    pub schema_version: u32,
    /// Opaque queue item id.
    pub queue_item_id: String,
    /// Descriptor this queue item belongs to.
    pub provider_descriptor_ref: String,
    /// Connected provider record reference.
    pub connected_provider_record_ref: String,
    /// Surface class that minted the queue item.
    pub surface_class: ProviderSurfaceClass,
    /// Provider action that will run if the item drains.
    pub action_kind: QueueActionKind,
    /// Provider target ref.
    pub target_ref: TargetRef,
    /// Actor and authority scope.
    pub actor_scope: ActorScope,
    /// Freshness truth admitted with the item.
    pub freshness: FreshnessTruth,
    /// Ordered dependencies required before drain.
    pub dependency_chain: Vec<QueueDependency>,
    /// Stale-target risk at current review time.
    pub stale_target_risk_class: StaleTargetRiskClass,
    /// Reauth requirement for this item.
    pub reauth_requirement: ReauthRequirementClass,
    /// Rescope requirement for this item.
    pub rescope_requirement: RescopeRequirementClass,
    /// Current queue state.
    pub queue_state: QueueState,
    /// Canonical publish-later queue item reference.
    pub linked_publish_later_queue_item_ref: String,
    /// Canonical deferred-publish queue item reference when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_deferred_publish_queue_item_ref: Option<String>,
    /// Local draft reference preserved by the queue item.
    pub local_draft_ref: String,
    /// Export safety posture.
    pub export_safety: ExportSafetyClass,
    /// True only when no raw provider payload refs cross the boundary.
    pub raw_payload_refs_present: bool,
    /// Next safe action a reviewer can take.
    pub next_safe_action: QueueNextSafeActionClass,
    /// Redaction-safe support summary.
    pub support_export_summary: String,
    /// Audit event refs for queue admission and review.
    pub audit_event_refs: Vec<String>,
    /// Queue admission timestamp.
    pub queued_at: String,
    /// Queue expiry timestamp.
    pub expires_at: String,
    /// Redaction posture for the queue item.
    pub redaction_class: RedactionClass,
}

impl PublishLaterQueueAlphaItem {
    /// Returns true when dependencies are zero-based and contiguous.
    pub fn dependency_order_is_strict(&self) -> bool {
        let mut indexes: Vec<usize> = self
            .dependency_chain
            .iter()
            .map(|dependency| dependency.dependency_order_index)
            .collect();
        indexes.sort_unstable();
        indexes
            .iter()
            .enumerate()
            .all(|(expected, actual)| expected == *actual)
    }

    /// Returns true when this row can be included in support bundles.
    pub fn is_export_safe(&self) -> bool {
        self.export_safety == ExportSafetyClass::SupportExportSafe
            && !self.raw_payload_refs_present
            && !self.support_export_summary.trim().is_empty()
            && !self.audit_event_refs.is_empty()
    }

    /// Returns true when either reauth or rescope work is required.
    pub fn requires_reauth_or_rescope(&self) -> bool {
        self.reauth_requirement != ReauthRequirementClass::NotRequired
            || self.rescope_requirement != RescopeRequirementClass::NotRequired
    }
}

/// One ordered dependency required before a queue item may drain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueDependency {
    /// Dependency class.
    pub dependency_class: DependencyClass,
    /// Dependency state.
    pub dependency_state: DependencyState,
    /// Zero-based order index.
    pub dependency_order_index: usize,
    /// Redaction-safe rationale.
    pub rationale_summary: String,
    /// Opaque linked record reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_record_ref: Option<String>,
}

/// Provider action kind represented by a queue row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueActionKind {
    /// Publish or update a pull-request review comment.
    PullRequestReviewCommentPublish,
    /// Create or update an issue or work item.
    IssueUpdate,
    /// Request a CI/check rerun.
    CheckRunRequestRerun,
}

/// Queue state for publish-later alpha items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueState {
    /// Captured offline before prerequisites can be checked.
    CapturedOffline,
    /// Waiting on an ordered dependency.
    PendingDependency,
    /// Waiting on reauthentication.
    PendingReauth,
    /// Waiting on provider-scope repair.
    PendingRescope,
    /// Waiting on target freshness refresh.
    PendingTargetRefresh,
    /// Ready for queue drain.
    ReadyForDrain,
    /// Drained and committed by the provider.
    DrainedCommitted,
    /// Cancelled by the user.
    CancelledByUser,
}

/// Dependency class required before a queue item drains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// Connectivity must be restored.
    ConnectivityRestored,
    /// Approval ticket must be admitted.
    ApprovalTicketAdmitted,
    /// Freshness floor must be satisfied.
    FreshnessFloorSatisfied,
    /// Predecessor queue item must drain first.
    PredecessorQueueItem,
    /// Reauth must complete.
    ReauthCompleted,
    /// Rescope must complete.
    RescopeCompleted,
    /// Conflict must be resolved.
    ConflictResolved,
}

/// Dependency state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyState {
    /// Dependency is unmet.
    Unmet,
    /// Dependency is met.
    Met,
    /// Dependency was waived by review.
    WaivedByReviewer,
}

/// Reauthentication requirement class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReauthRequirementClass {
    /// No reauthentication is required.
    NotRequired,
    /// Human session must be renewed.
    SessionRenewalRequired,
    /// Step-up authentication is required.
    StepUpRequired,
}

/// Rescope requirement class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RescopeRequirementClass {
    /// No rescope is required.
    NotRequired,
    /// Additional provider scopes are required.
    AdditionalScopesRequired,
    /// Tenant or org scope must be realigned.
    TenantOrOrgScopeRealignmentRequired,
}

/// Export safety posture for queue rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSafetyClass {
    /// Row is safe for support export.
    SupportExportSafe,
    /// Row is blocked from support export.
    SupportExportBlocked,
}

/// Next safe action for a queue reviewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueNextSafeActionClass {
    /// Queue item can drain now.
    DrainNow,
    /// Refresh the provider target before review.
    RefreshTargetThenReview,
    /// Reauthenticate before retry.
    ReauthThenRetry,
    /// Request additional scope before retry.
    RescopeThenRetry,
    /// Open provider through typed handoff.
    OpenInProvider,
    /// Export an evidence-safe packet.
    ExportEvidencePacket,
}
