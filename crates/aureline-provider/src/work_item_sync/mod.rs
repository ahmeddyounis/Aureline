//! Provider work-item link, comment-sync, and publish-review beta contracts.
//!
//! This module finishes the still-light issue/work-item lane by adding stable
//! link, comment-sync, and publish-review semantics on top of the durable
//! [`crate::work_items`] detail records. Every claimed beta row keeps three
//! axes of truth visibly distinct:
//!
//! - **Link state.** Branch, review, change-object, and peer relations bind to
//!   the same work-item object model so a "linked branch," a "linked review,"
//!   and an "offline handoff" cannot drift into private states. Each link
//!   carries its source class, relation state, freshness, write scope,
//!   local-draft state, sync-pending state, and conflict-resolution posture.
//! - **Comment sync state.** Provider-owned comments, local draft comments,
//!   offline capture packets, queued publishes, and failed/conflicted
//!   publishes stay on separate enum lanes — provider acceptance is never
//!   implied by an unconfirmed local draft.
//! - **Publish-review sheets.** Comment create/edit/delete, link/unlink,
//!   status transition plus comment, and retry-after-conflict each render a
//!   typed review row with source class, actor scope, side-effect fanout,
//!   and admissibility/block posture before the user confirms.
//!
//! Support and telemetry exports stay enum-based by default so tracked-work
//! proof never leaks comment text, project names, or provider URLs.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::registry::{ProviderActorClass, ProviderFamily, RedactionClass};
use crate::work_items::{
    TrustPosture, WorkItemFreshnessClass, WorkItemMutationMode, WorkItemOriginDisclosure,
    WorkItemPolicyContext,
};

/// Schema version exported by the work-item sync beta page.
pub const WORK_ITEM_SYNC_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by provider work-item sync records.
pub const WORK_ITEM_SYNC_BETA_SHARED_CONTRACT_REF: &str = "providers:work_item_sync_beta:v1";

/// Stable record kind for [`WorkItemSyncBetaPage`].
pub const WORK_ITEM_SYNC_BETA_PAGE_RECORD_KIND: &str = "providers_work_item_sync_beta_page_record";

/// Stable record kind for [`WorkItemLinkStateRecord`].
pub const WORK_ITEM_LINK_STATE_RECORD_KIND: &str = "work_item_link_state_record";

/// Stable record kind for [`CommentSyncStateRecord`].
pub const COMMENT_SYNC_STATE_RECORD_KIND: &str = "comment_sync_state_record";

/// Stable record kind for [`PublishReviewRecord`].
pub const PUBLISH_REVIEW_RECORD_KIND: &str = "work_item_publish_review_record";

/// Stable record kind for [`WorkItemSyncBetaValidationReport`].
pub const WORK_ITEM_SYNC_BETA_VALIDATION_REPORT_RECORD_KIND: &str =
    "providers_work_item_sync_beta_validation_report_record";

/// Stable record kind for [`WorkItemSyncBetaSupportExport`].
pub const WORK_ITEM_SYNC_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_work_item_sync_beta_support_export_record";

/// Kind of work-item link covered by a [`WorkItemLinkStateRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemLinkKindClass {
    /// Link to a branch or worktree.
    BranchOrWorktreeLink,
    /// Link to a review workspace or review record.
    ReviewWorkspaceLink,
    /// Link to a change object.
    ChangeObjectLink,
    /// Link to a peer work item (blocks, blocked-by, parent, child, related).
    WorkItemPeerLink,
    /// Link to a handoff or evidence packet.
    HandoffPacketLink,
}

impl WorkItemLinkKindClass {
    /// Stable token recorded on support and fixture records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BranchOrWorktreeLink => "branch_or_worktree_link",
            Self::ReviewWorkspaceLink => "review_workspace_link",
            Self::ChangeObjectLink => "change_object_link",
            Self::WorkItemPeerLink => "work_item_peer_link",
            Self::HandoffPacketLink => "handoff_packet_link",
        }
    }
}

/// Source class for one work-item link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkSourceClass {
    /// Link came from a provider overlay that is currently authoritative.
    ProviderAuthoritativeOverlay,
    /// Provider observed the link but a cached shadow is the only local copy.
    ProviderObservedCachedShadow,
    /// User authored the link locally; no provider object yet.
    LocalAuthoredNoProviderObject,
    /// Link was imported from an offline handoff or support export.
    ImportedHandoffEvidenceOnly,
    /// Link routes through a browser handoff packet.
    BrowserHandoffOnly,
    /// Link came from an AI proposal pending user confirmation.
    AiProposedPendingUserConfirmation,
    /// Link came from an imported review bundle snapshot.
    ReviewBundleSnapshot,
}

/// Relation state for one work-item link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkRelationStateClass {
    /// Provider observed and live.
    LinkedActiveProviderConfirmed,
    /// Local draft that has not been published.
    LinkedLocalDraftPendingPublish,
    /// Queued for publish-later.
    LinkedQueuedForPublishLater,
    /// Routed through browser handoff only.
    LinkedThroughBrowserHandoffOnly,
    /// Imported snapshot, no live provider path.
    LinkedImportedSnapshotNoProviderPath,
    /// Unlink requested locally, awaiting publish.
    UnlinkRequestedLocalDraft,
    /// Unlink queued for publish-later.
    UnlinkQueuedForPublishLater,
    /// Provider rejected the link or unlink.
    BrokenProviderRejected,
    /// Provider object the link pointed at can no longer be observed.
    BrokenProviderObjectMissing,
    /// Provider and local copies disagree; user must resolve.
    ConflictRequiresReview,
}

impl LinkRelationStateClass {
    /// True when the relation is one of the unlink/broken/conflict postures.
    pub const fn is_pending_or_broken(self) -> bool {
        matches!(
            self,
            Self::LinkedLocalDraftPendingPublish
                | Self::LinkedQueuedForPublishLater
                | Self::UnlinkRequestedLocalDraft
                | Self::UnlinkQueuedForPublishLater
                | Self::BrokenProviderRejected
                | Self::BrokenProviderObjectMissing
                | Self::ConflictRequiresReview
        )
    }
}

/// Write scope for one work-item link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkWriteScopeClass {
    /// Link change can publish to provider immediately after review.
    WriteAdmissibleProviderWriteable,
    /// Link change can route through browser handoff only.
    WriteAdmissibleThroughBrowserHandoffOnly,
    /// Link change can queue for publish-later.
    WriteAdmissibleQueuedPublishOnly,
    /// Link change saves locally only; no provider path.
    WriteAdmissibleLocalDraftOnlyNoProviderPath,
    /// Provider grant is read-only.
    WriteBlockedProviderReadOnlyScope,
    /// Workspace trust blocks the link change.
    WriteBlockedWorkspaceTrustUnsetOrRestricted,
    /// Provider is unreachable.
    WriteBlockedProviderUnreachable,
    /// Link target is imported evidence only.
    WriteBlockedImportedEvidenceOnlyNoProviderPath,
    /// Account or project mapping is pending.
    WriteBlockedAccountMappingPending,
}

impl LinkWriteScopeClass {
    /// True when the scope admits some local, queued, handoff, or provider write path.
    pub const fn admits_write(self) -> bool {
        matches!(
            self,
            Self::WriteAdmissibleProviderWriteable
                | Self::WriteAdmissibleThroughBrowserHandoffOnly
                | Self::WriteAdmissibleQueuedPublishOnly
                | Self::WriteAdmissibleLocalDraftOnlyNoProviderPath
        )
    }
}

/// Local-draft posture for one work-item link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkLocalDraftStateClass {
    /// No local draft edit is staged.
    NoLocalDraftEdit,
    /// Local draft creates a new link pending publish.
    LocalDraftCreateLinkPendingPublish,
    /// Local draft removes the link pending publish.
    LocalDraftRemoveLinkPendingPublish,
    /// Local draft edit conflicts with provider truth.
    LocalDraftEditConflictsWithProvider,
}

/// Sync-pending posture for one work-item link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkSyncPendingStateClass {
    /// Provider observed and in sync.
    InSyncProviderObserved,
    /// Awaiting a publish of a local draft edit.
    PendingPublishLocalDraft,
    /// Already queued for publish-later.
    PendingPublishQueued,
    /// Awaiting drain of an offline-capture packet.
    PendingDrainOfflineCaptured,
    /// Provider callback envelope is in flight.
    PendingProviderCallbackAfterPublish,
    /// Provider is unreachable; row is stale.
    StaleProviderUnreachable,
    /// Freshness floor is unsatisfied; row is stale.
    StaleFreshnessFloorUnsatisfied,
}

/// Conflict-resolution posture for one work-item link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkConflictResolutionPostureClass {
    /// No conflict has been detected.
    NoConflictDetected,
    /// Provider updated the link after the local draft; awaiting user choice.
    ConflictProviderUpdatedAwaitingUserChoice,
    /// Local edit conflicts with provider truth; awaiting user choice.
    ConflictLocalEditConflictsAwaitingUserChoice,
    /// User chose to keep provider truth and drop the local draft.
    ConflictResolvedKeepProviderTruth,
    /// User chose to keep the local draft and re-publish over provider truth.
    ConflictResolvedKeepLocalDraft,
    /// User chose to merge into a new provider state.
    ConflictResolvedMergeIntoNewProviderState,
    /// Conflict resolution is blocked by policy.
    ConflictBlockedPendingPolicy,
}

/// Origin class for one comment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentOriginClass {
    /// Comment came from authoritative provider truth.
    ProviderAuthoritativeComment,
    /// Comment is a local draft never sent to a provider.
    LocalDraftComment,
    /// Comment is captured in an offline handoff packet.
    OfflineCapturePacketComment,
    /// Comment is imported from a snapshot with no live provider path.
    ImportedSnapshotComment,
    /// Comment came from an AI proposal pending user confirmation.
    AiProposedDraftComment,
}

/// Lifecycle class for one comment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentLifecycleClass {
    /// Provider observed the comment as active.
    ProviderObservedActive,
    /// Provider observed the comment after an edit.
    ProviderObservedEdited,
    /// Provider observed the comment as deleted.
    ProviderObservedDeleted,
    /// Local draft create; never published.
    LocalDraftCreateNeverPublished,
    /// Local draft edit on top of a provider comment.
    LocalDraftEditOnProviderComment,
    /// Local draft delete on top of a provider comment.
    LocalDraftDeleteOnProviderComment,
}

/// Sync state for one comment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentSyncStateClass {
    /// Provider observed and in sync.
    InSyncProviderObserved,
    /// Awaiting publish of a local draft.
    PendingPublishLocalDraft,
    /// Queued for publish-later.
    QueuedPublishDeferred,
    /// Awaiting drain of an offline-capture packet.
    PendingDrainOfflineCaptured,
    /// Provider callback envelope is in flight.
    PendingProviderCallbackAfterPublish,
    /// Publish failed with a typed retry route.
    PublishFailedTypedRetry,
    /// Conflict detected; awaiting user resolution.
    ConflictDetectedAwaitingResolution,
    /// Provider is unreachable; row is stale.
    StaleProviderUnreachable,
}

impl CommentSyncStateClass {
    /// True when the comment is provider-observed and not stale.
    pub const fn is_provider_observed(self) -> bool {
        matches!(self, Self::InSyncProviderObserved)
    }

    /// True when this state implies provider acceptance is not yet confirmed.
    pub const fn implies_unaccepted(self) -> bool {
        !matches!(self, Self::InSyncProviderObserved)
    }
}

/// Publish posture for one comment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentPublishPostureClass {
    /// Provider published and observed.
    ProviderPublishedObserved,
    /// Local draft only; never published.
    LocalDraftNeverPublished,
    /// Queued for publish-later.
    QueuedForPublishLater,
    /// Captured offline; not yet submitted to a provider.
    OfflineCapturedNotSubmitted,
    /// Publish attempt failed; awaiting retry.
    PublishFailedAwaitingRetry,
    /// Imported evidence only.
    ImportedEvidenceOnly,
    /// Inspect-only read of provider state.
    InspectOnlyReadOnly,
}

/// Conflict class for one comment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentConflictClass {
    /// No conflict detected.
    NoConflict,
    /// Provider edited the comment after the local draft started.
    ConflictProviderEditedAfterDraft,
    /// Provider deleted the comment after the local draft started.
    ConflictProviderDeletedAfterDraft,
    /// Duplicate publish was rejected by the provider.
    ConflictDuplicatePublishRejected,
    /// Policy blocked the publish after the draft was authored.
    ConflictPolicyBlockedAfterDraft,
}

impl CommentConflictClass {
    /// True when the comment row currently carries a conflict.
    pub const fn is_conflict(self) -> bool {
        !matches!(self, Self::NoConflict)
    }
}

/// Publish-review action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReviewActionClass {
    /// Create a provider comment.
    CreateProviderComment,
    /// Edit a provider comment.
    EditProviderComment,
    /// Delete a provider comment.
    DeleteProviderComment,
    /// Link a branch or review workspace to the work item.
    LinkBranchOrReview,
    /// Unlink a branch or review workspace from the work item.
    UnlinkBranchOrReview,
    /// Status transition that also publishes a comment.
    StatusTransitionPlusComment,
    /// Retry after a previously detected conflict.
    RetryAfterConflict,
    /// Retry after a previously failed publish.
    RetryAfterFailure,
}

impl PublishReviewActionClass {
    /// True when the action implies a comment-text mutation.
    pub const fn is_comment_action(self) -> bool {
        matches!(
            self,
            Self::CreateProviderComment
                | Self::EditProviderComment
                | Self::DeleteProviderComment
                | Self::StatusTransitionPlusComment
        )
    }

    /// True when the action mutates a branch/review link.
    pub const fn is_link_action(self) -> bool {
        matches!(self, Self::LinkBranchOrReview | Self::UnlinkBranchOrReview)
    }
}

/// Source class for the publish-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReviewSourceClass {
    /// User authored the publish-review row locally.
    LocalDraftAuthored,
    /// AI proposed the row; user must confirm.
    AiProposedPendingConfirmation,
    /// Row came from imported handoff evidence.
    ImportedHandoffEvidence,
    /// Row came from a review bundle snapshot.
    ReviewBundleSnapshot,
    /// Row came from a queue-drain retry.
    QueueDrainRetry,
    /// Row came from a conflict-resolution follow-on.
    ConflictResolutionFollowOn,
}

/// Actor scope for the publish-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReviewActorScopeClass {
    /// Human actor writes provider directly.
    HumanActorWritesProvider,
    /// Human actor with linked review write scope.
    HumanActorWithReviewLinkScope,
    /// Install or app grant scope.
    InstallOrAppGrantScope,
    /// Delegated user token scope.
    DelegatedUserTokenScope,
    /// Project-scoped grant scope.
    ProjectScopedGrantScope,
    /// Browser handoff is the only admitted actor scope.
    BrowserHandoffOnlyActorScope,
    /// Managed admin route is the only admitted actor scope.
    ManagedAdminOnlyActorScope,
    /// Local-only actor scope (no provider account).
    LocalOnlyNoProviderScope,
}

/// Side-effect class disclosed by a publish-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReviewSideEffectClass {
    /// Provider comment mutation fans out.
    ProviderCommentMutationFanout,
    /// Provider link mutation fans out.
    ProviderLinkMutationFanout,
    /// Provider state transition fans out.
    ProviderStateTransitionFanout,
    /// Local metadata change fans out.
    LocalMetadataChangeFanout,
    /// Linked review record updates as a side effect.
    LinkedReviewUpdateFanout,
    /// Notification is emitted (or intentionally deferred).
    NotificationEmissionFanout,
    /// A follow-on automation is queued.
    QueuedFollowonAutomationFanout,
}

/// Disposition class for a publish-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishReviewDispositionClass {
    /// Confirming will publish now.
    AdmissibleNowPublishNow,
    /// Confirming will queue for publish-later.
    AdmissibleViaQueueForPublishLater,
    /// Confirming will route through a browser handoff.
    AdmissibleViaBrowserHandoffOnly,
    /// Confirming will save a local draft only.
    AdmissibleLocalDraftOnly,
    /// Confirming will retry after the conflict is resolved.
    AdmissibleViaRetryAfterConflict,
    /// Pending conflict resolution.
    BlockedPendingConflictResolution,
    /// Provider grant is read-only.
    BlockedProviderReadOnly,
    /// Blocked by policy.
    BlockedByPolicy,
    /// Freshness floor not satisfied.
    BlockedFreshnessFloorUnsatisfied,
    /// Account or project mapping pending.
    BlockedAccountMappingPending,
}

impl PublishReviewDispositionClass {
    /// True when this disposition admits a confirm path.
    pub const fn is_admissible(self) -> bool {
        matches!(
            self,
            Self::AdmissibleNowPublishNow
                | Self::AdmissibleViaQueueForPublishLater
                | Self::AdmissibleViaBrowserHandoffOnly
                | Self::AdmissibleLocalDraftOnly
                | Self::AdmissibleViaRetryAfterConflict
        )
    }
}

/// Action affordances rendered by a publish-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishReviewActionAffordances {
    /// Confirm action is visible and enabled.
    pub confirm_action_available: bool,
    /// Export action is visible.
    pub export_action_available: bool,
    /// Cancel action is visible.
    pub cancel_action_available: bool,
    /// Discard local draft action is visible.
    pub discard_draft_action_available: bool,
}

/// One side-effect fanout row disclosed by a publish-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishReviewSideEffectRow {
    /// Opaque fanout row id.
    pub fanout_row_id: String,
    /// Side-effect class.
    pub side_effect_class: PublishReviewSideEffectClass,
    /// Publish mode for this row.
    pub publish_mode_class: WorkItemMutationMode,
    /// Optional linked record ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_record_id_ref: Option<String>,
    /// Redaction-safe summary.
    pub summary: String,
}

/// Fixture metadata recorded on a sync beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemSyncFixtureMetadata {
    /// Fixture name.
    pub name: String,
    /// Redaction-safe scenario summary.
    pub scenario: String,
}

/// Upstream contracts consumed by the work-item sync beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemSyncContractRefs {
    /// Work-item transition beta contract reference.
    pub work_item_transition_beta_shared_contract_ref: String,
    /// Work-item link state schema reference.
    pub work_item_link_state_schema_ref: String,
    /// Comment sync state schema reference.
    pub comment_sync_state_schema_ref: String,
    /// Publish review schema reference.
    pub publish_review_schema_ref: String,
    /// Publish-later queue schema reference.
    pub publish_later_queue_schema_ref: String,
    /// Browser-handoff packet schema reference.
    pub browser_handoff_packet_schema_ref: String,
    /// Offline-handoff packet schema reference.
    pub offline_handoff_packet_schema_ref: String,
    /// Connected-provider registry schema reference.
    pub connected_provider_registry_schema_ref: String,
}

impl WorkItemSyncContractRefs {
    fn all_refs(&self) -> [&str; 8] {
        [
            &self.work_item_transition_beta_shared_contract_ref,
            &self.work_item_link_state_schema_ref,
            &self.comment_sync_state_schema_ref,
            &self.publish_review_schema_ref,
            &self.publish_later_queue_schema_ref,
            &self.browser_handoff_packet_schema_ref,
            &self.offline_handoff_packet_schema_ref,
            &self.connected_provider_registry_schema_ref,
        ]
    }
}

/// Durable work-item link state record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkStateRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Opaque link id.
    pub link_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Link kind class.
    pub link_kind_class: WorkItemLinkKindClass,
    /// Link source class.
    pub link_source_class: LinkSourceClass,
    /// Link relation state class.
    pub link_relation_state_class: LinkRelationStateClass,
    /// Link freshness class (shared with detail-row freshness vocabulary).
    pub link_freshness_class: WorkItemFreshnessClass,
    /// Write scope for changing this link.
    pub link_write_scope_class: LinkWriteScopeClass,
    /// Local-draft state on the link.
    pub link_local_draft_state_class: LinkLocalDraftStateClass,
    /// Sync-pending state for the link.
    pub link_sync_pending_state_class: LinkSyncPendingStateClass,
    /// Conflict-resolution posture for the link.
    pub link_conflict_resolution_posture_class: LinkConflictResolutionPostureClass,
    /// Optional branch locator ref (branch links only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_branch_local_locator_ref: Option<String>,
    /// Optional review workspace record ref (review links only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_review_workspace_record_id_ref: Option<String>,
    /// Optional change object record ref (change-object links only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_change_object_record_id_ref: Option<String>,
    /// Optional peer work-item detail ref (peer links only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_peer_work_item_detail_record_id_ref: Option<String>,
    /// Optional handoff packet ref (handoff links only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_handoff_packet_record_id_ref: Option<String>,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional offline-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Origin disclosure.
    pub origin_disclosure: WorkItemOriginDisclosure,
    /// Policy context.
    pub policy_context: WorkItemPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Guardrail: no raw provider payload refs cross this boundary.
    pub raw_provider_payload_refs_present: bool,
    /// Guardrail: no raw provider URL crosses this boundary.
    pub raw_provider_url_present: bool,
    /// Captured timestamp.
    pub captured_at: String,
    /// Redaction-safe summary.
    pub support_export_summary: String,
}

/// Durable comment sync state record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentSyncStateRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Opaque comment sync id.
    pub comment_sync_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Optional provider-side comment ref (provider-owned comments only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_comment_record_id_ref: Option<String>,
    /// Optional local draft ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_draft_comment_record_id_ref: Option<String>,
    /// Optional offline capture packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_handoff_packet_record_id_ref: Option<String>,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Comment origin class.
    pub comment_origin_class: CommentOriginClass,
    /// Comment lifecycle class.
    pub comment_lifecycle_class: CommentLifecycleClass,
    /// Comment sync state class.
    pub comment_sync_state_class: CommentSyncStateClass,
    /// Comment publish posture class.
    pub comment_publish_posture_class: CommentPublishPostureClass,
    /// Comment conflict class.
    pub comment_conflict_class: CommentConflictClass,
    /// Acting identity class.
    pub acting_as: ProviderActorClass,
    /// Optional typed retry route token if publish has failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub typed_retry_route_class: Option<CommentRetryRouteClass>,
    /// Origin disclosure.
    pub origin_disclosure: WorkItemOriginDisclosure,
    /// Policy context.
    pub policy_context: WorkItemPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Guardrail: no raw provider comment text crosses this boundary.
    pub raw_comment_text_present: bool,
    /// Guardrail: no raw provider URL crosses this boundary.
    pub raw_provider_url_present: bool,
    /// Captured timestamp.
    pub captured_at: String,
    /// Redaction-safe summary.
    pub support_export_summary: String,
}

/// Typed retry route for a failed comment publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentRetryRouteClass {
    /// Auto retry on connectivity restored.
    AutoRetryOnConnectivityRestored,
    /// Auto retry on provider health recovered.
    AutoRetryOnProviderHealthRecovered,
    /// Auto retry on account reselected.
    AutoRetryOnAccountReselected,
    /// Auto retry on freshness refresh.
    AutoRetryOnFreshnessRefresh,
    /// Auto retry on conflict resolved.
    AutoRetryOnConflictResolved,
    /// Manual retry only.
    ManualRetryUserMustConfirmOnly,
    /// Imported evidence has no retry path.
    NoRetryImportedEvidenceOnly,
}

/// Publish-review row (sheet) for a comment, link/unlink, or retry action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishReviewRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Opaque publish-review id.
    pub publish_review_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Optional bound work-item link state ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_item_link_state_record_id_ref: Option<String>,
    /// Optional bound comment sync state ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_sync_state_record_id_ref: Option<String>,
    /// Optional bound status-transition packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_transition_packet_record_id_ref: Option<String>,
    /// Publish-review action class.
    pub publish_review_action_class: PublishReviewActionClass,
    /// Source class.
    pub publish_review_source_class: PublishReviewSourceClass,
    /// Actor scope class.
    pub publish_review_actor_scope_class: PublishReviewActorScopeClass,
    /// Disposition class.
    pub publish_review_disposition_class: PublishReviewDispositionClass,
    /// Publish mode class.
    pub publish_mode_class: WorkItemMutationMode,
    /// Acting identity class.
    pub acting_as: ProviderActorClass,
    /// Side-effect fanout rows.
    pub side_effect_rows: Vec<PublishReviewSideEffectRow>,
    /// Action affordances.
    pub action_affordances: PublishReviewActionAffordances,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Optional offline-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_record_id_ref: Option<String>,
    /// Whether a local draft is preserved if publish fails.
    pub local_draft_preserved_on_failure: bool,
    /// Optional block reason summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_summary: Option<String>,
    /// Origin disclosure.
    pub origin_disclosure: WorkItemOriginDisclosure,
    /// Policy context.
    pub policy_context: WorkItemPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Guardrail: no raw provider payload refs cross this boundary.
    pub raw_provider_payload_refs_present: bool,
    /// Authored timestamp.
    pub authored_at: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Redaction-safe summary.
    pub summary: String,
}

/// Work-item sync beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemSyncBetaPage {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<WorkItemSyncFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contract references.
    pub contract_refs: WorkItemSyncContractRefs,
    /// Work-item link state records.
    pub work_item_link_records: Vec<WorkItemLinkStateRecord>,
    /// Comment sync state records.
    pub comment_sync_records: Vec<CommentSyncStateRecord>,
    /// Publish-review records.
    pub publish_reviews: Vec<PublishReviewRecord>,
    /// Redaction-safe page summary.
    pub support_export_summary: String,
}

impl WorkItemSyncBetaPage {
    /// Validates the page and returns a structured report.
    pub fn validate(&self) -> WorkItemSyncBetaValidationReport {
        let mut validator = WorkItemSyncBetaValidator::new(self);
        validator.run();
        validator.finish()
    }

    /// Builds a redaction-safe support-export projection of the page.
    pub fn support_export_projection(&self) -> WorkItemSyncBetaSupportExport {
        WorkItemSyncBetaSupportExport::from_page(
            format!("{}:support_export", self.page_id),
            self.work_item_link_records
                .first()
                .map(|record| record.captured_at.clone())
                .or_else(|| {
                    self.comment_sync_records
                        .first()
                        .map(|record| record.captured_at.clone())
                })
                .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string()),
            self,
        )
    }
}

/// Validation report emitted by the beta validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemSyncBetaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Whether no defects were emitted.
    pub passed: bool,
    /// Coverage observed while validating.
    pub coverage: WorkItemSyncBetaCoverage,
    /// Defects emitted by failed checks.
    pub defects: Vec<WorkItemSyncBetaDefect>,
}

/// Coverage observed during work-item sync validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorkItemSyncBetaCoverage {
    /// Link kinds covered.
    pub link_kinds: BTreeSet<WorkItemLinkKindClass>,
    /// Link source classes covered.
    pub link_source_classes: BTreeSet<LinkSourceClass>,
    /// Link relation states covered.
    pub link_relation_states: BTreeSet<LinkRelationStateClass>,
    /// Link sync-pending states covered.
    pub link_sync_pending_states: BTreeSet<LinkSyncPendingStateClass>,
    /// Link conflict postures covered.
    pub link_conflict_postures: BTreeSet<LinkConflictResolutionPostureClass>,
    /// Comment origin classes covered.
    pub comment_origin_classes: BTreeSet<CommentOriginClass>,
    /// Comment lifecycle classes covered.
    pub comment_lifecycle_classes: BTreeSet<CommentLifecycleClass>,
    /// Comment sync states covered.
    pub comment_sync_states: BTreeSet<CommentSyncStateClass>,
    /// Comment publish postures covered.
    pub comment_publish_postures: BTreeSet<CommentPublishPostureClass>,
    /// Comment conflict classes covered.
    pub comment_conflict_classes: BTreeSet<CommentConflictClass>,
    /// Publish-review action classes covered.
    pub publish_review_action_classes: BTreeSet<PublishReviewActionClass>,
    /// Publish-review disposition classes covered.
    pub publish_review_disposition_classes: BTreeSet<PublishReviewDispositionClass>,
    /// Publish-review side-effect classes covered.
    pub publish_review_side_effect_classes: BTreeSet<PublishReviewSideEffectClass>,
}

/// One validation defect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemSyncBetaDefect {
    /// Defect kind.
    pub defect_kind: WorkItemSyncBetaDefectKind,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
    /// Optional offending record ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_ref: Option<String>,
}

/// Defect kind emitted by work-item sync validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemSyncBetaDefectKind {
    /// Page-level metadata is invalid.
    PageContractInvalid,
    /// Required coverage is missing.
    CoverageMissing,
    /// Duplicate id was found.
    DuplicateId,
    /// Required field is missing.
    RequiredFieldMissing,
    /// A referenced record is missing.
    UnknownRecordReference,
    /// Link row truth is incoherent.
    LinkTruthIncoherent,
    /// Comment-sync row truth is incoherent.
    CommentSyncTruthIncoherent,
    /// Publish-review row truth is incoherent.
    PublishReviewTruthIncoherent,
    /// Raw provider material crossed the boundary.
    RawProviderMaterialPresent,
    /// Support/export posture is unsafe.
    SupportExportUnsafe,
}

/// Redaction-safe support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemSyncBetaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Opaque export id.
    pub export_id: String,
    /// Page id.
    pub page_id: String,
    /// Generated timestamp.
    pub generated_at: String,
    /// Link summaries.
    pub link_summaries: Vec<WorkItemLinkSupportSummary>,
    /// Comment sync summaries.
    pub comment_sync_summaries: Vec<CommentSyncSupportSummary>,
    /// Publish-review summaries.
    pub publish_review_summaries: Vec<PublishReviewSupportSummary>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Guardrail: export contains no raw provider material.
    pub raw_provider_material_excluded: bool,
}

impl WorkItemSyncBetaSupportExport {
    /// Builds an export-safe projection from a page.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: &WorkItemSyncBetaPage,
    ) -> Self {
        Self {
            record_kind: WORK_ITEM_SYNC_BETA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
            export_id: export_id.into(),
            page_id: page.page_id.clone(),
            generated_at: generated_at.into(),
            link_summaries: page
                .work_item_link_records
                .iter()
                .map(WorkItemLinkSupportSummary::from)
                .collect(),
            comment_sync_summaries: page
                .comment_sync_records
                .iter()
                .map(CommentSyncSupportSummary::from)
                .collect(),
            publish_review_summaries: page
                .publish_reviews
                .iter()
                .map(PublishReviewSupportSummary::from)
                .collect(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_provider_material_excluded: true,
        }
    }
}

/// Support summary for one link row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemLinkSupportSummary {
    /// Link id.
    pub link_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Link kind class.
    pub link_kind_class: WorkItemLinkKindClass,
    /// Link source class.
    pub link_source_class: LinkSourceClass,
    /// Link relation state class.
    pub link_relation_state_class: LinkRelationStateClass,
    /// Link write scope class.
    pub link_write_scope_class: LinkWriteScopeClass,
    /// Link local-draft state class.
    pub link_local_draft_state_class: LinkLocalDraftStateClass,
    /// Link sync-pending state class.
    pub link_sync_pending_state_class: LinkSyncPendingStateClass,
    /// Link conflict posture class.
    pub link_conflict_resolution_posture_class: LinkConflictResolutionPostureClass,
    /// Redaction-safe summary.
    pub summary: String,
}

impl From<&WorkItemLinkStateRecord> for WorkItemLinkSupportSummary {
    fn from(record: &WorkItemLinkStateRecord) -> Self {
        Self {
            link_id: record.link_id.clone(),
            work_item_detail_record_id_ref: record.work_item_detail_record_id_ref.clone(),
            link_kind_class: record.link_kind_class,
            link_source_class: record.link_source_class,
            link_relation_state_class: record.link_relation_state_class,
            link_write_scope_class: record.link_write_scope_class,
            link_local_draft_state_class: record.link_local_draft_state_class,
            link_sync_pending_state_class: record.link_sync_pending_state_class,
            link_conflict_resolution_posture_class: record.link_conflict_resolution_posture_class,
            summary: record.support_export_summary.clone(),
        }
    }
}

/// Support summary for one comment-sync row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentSyncSupportSummary {
    /// Comment sync id.
    pub comment_sync_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Comment origin class.
    pub comment_origin_class: CommentOriginClass,
    /// Comment lifecycle class.
    pub comment_lifecycle_class: CommentLifecycleClass,
    /// Comment sync state class.
    pub comment_sync_state_class: CommentSyncStateClass,
    /// Comment publish posture class.
    pub comment_publish_posture_class: CommentPublishPostureClass,
    /// Comment conflict class.
    pub comment_conflict_class: CommentConflictClass,
    /// Acting identity class.
    pub acting_as: ProviderActorClass,
    /// Redaction-safe summary.
    pub summary: String,
}

impl From<&CommentSyncStateRecord> for CommentSyncSupportSummary {
    fn from(record: &CommentSyncStateRecord) -> Self {
        Self {
            comment_sync_id: record.comment_sync_id.clone(),
            work_item_detail_record_id_ref: record.work_item_detail_record_id_ref.clone(),
            comment_origin_class: record.comment_origin_class,
            comment_lifecycle_class: record.comment_lifecycle_class,
            comment_sync_state_class: record.comment_sync_state_class,
            comment_publish_posture_class: record.comment_publish_posture_class,
            comment_conflict_class: record.comment_conflict_class,
            acting_as: record.acting_as,
            summary: record.support_export_summary.clone(),
        }
    }
}

/// Support summary for one publish-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishReviewSupportSummary {
    /// Publish-review id.
    pub publish_review_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Publish-review action class.
    pub publish_review_action_class: PublishReviewActionClass,
    /// Publish-review source class.
    pub publish_review_source_class: PublishReviewSourceClass,
    /// Publish-review actor scope class.
    pub publish_review_actor_scope_class: PublishReviewActorScopeClass,
    /// Publish-review disposition class.
    pub publish_review_disposition_class: PublishReviewDispositionClass,
    /// Publish mode class.
    pub publish_mode_class: WorkItemMutationMode,
    /// Side-effect classes covered by the sheet.
    pub side_effect_classes: BTreeSet<PublishReviewSideEffectClass>,
    /// Redaction-safe summary.
    pub summary: String,
}

impl From<&PublishReviewRecord> for PublishReviewSupportSummary {
    fn from(record: &PublishReviewRecord) -> Self {
        Self {
            publish_review_id: record.publish_review_id.clone(),
            work_item_detail_record_id_ref: record.work_item_detail_record_id_ref.clone(),
            publish_review_action_class: record.publish_review_action_class,
            publish_review_source_class: record.publish_review_source_class,
            publish_review_actor_scope_class: record.publish_review_actor_scope_class,
            publish_review_disposition_class: record.publish_review_disposition_class,
            publish_mode_class: record.publish_mode_class,
            side_effect_classes: record
                .side_effect_rows
                .iter()
                .map(|row| row.side_effect_class)
                .collect(),
            summary: record.summary.clone(),
        }
    }
}

/// Validates a page and returns typed defects on failure.
pub fn validate_work_item_sync_beta_page(
    page: &WorkItemSyncBetaPage,
) -> Result<(), Vec<WorkItemSyncBetaDefect>> {
    let defects = audit_work_item_sync_beta_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Audits a page and returns every defect.
pub fn audit_work_item_sync_beta_page(page: &WorkItemSyncBetaPage) -> Vec<WorkItemSyncBetaDefect> {
    let mut validator = WorkItemSyncBetaValidator::new(page);
    validator.run();
    validator.defects
}

struct WorkItemSyncBetaValidator<'a> {
    page: &'a WorkItemSyncBetaPage,
    link_ids: BTreeSet<&'a str>,
    comment_sync_ids: BTreeSet<&'a str>,
    publish_review_ids: BTreeSet<&'a str>,
    coverage: WorkItemSyncBetaCoverage,
    defects: Vec<WorkItemSyncBetaDefect>,
}

impl<'a> WorkItemSyncBetaValidator<'a> {
    fn new(page: &'a WorkItemSyncBetaPage) -> Self {
        Self {
            page,
            link_ids: BTreeSet::new(),
            comment_sync_ids: BTreeSet::new(),
            publish_review_ids: BTreeSet::new(),
            coverage: WorkItemSyncBetaCoverage::default(),
            defects: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_links();
        self.validate_comment_syncs();
        self.validate_publish_reviews();
        self.validate_required_coverage();
    }

    fn finish(self) -> WorkItemSyncBetaValidationReport {
        WorkItemSyncBetaValidationReport {
            record_kind: WORK_ITEM_SYNC_BETA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed: self.defects.is_empty(),
            coverage: self.coverage,
            defects: self.defects,
        }
    }

    fn validate_page_header(&mut self) {
        self.expect(
            self.page.record_kind == WORK_ITEM_SYNC_BETA_PAGE_RECORD_KIND,
            WorkItemSyncBetaDefectKind::PageContractInvalid,
            "work_item_sync_beta.page_record_kind",
            "page.record_kind must be providers_work_item_sync_beta_page_record",
            Some(&self.page.page_id),
        );
        self.expect(
            self.page.schema_version == WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
            WorkItemSyncBetaDefectKind::PageContractInvalid,
            "work_item_sync_beta.page_schema_version",
            "page.schema_version must match the crate constant",
            Some(&self.page.page_id),
        );
        self.expect(
            self.page.shared_contract_ref == WORK_ITEM_SYNC_BETA_SHARED_CONTRACT_REF,
            WorkItemSyncBetaDefectKind::PageContractInvalid,
            "work_item_sync_beta.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
            Some(&self.page.page_id),
        );
        self.expect_non_empty(
            &self.page.page_id,
            "work_item_sync_beta.page_id_missing",
            Some(&self.page.page_id),
        );
        self.expect_non_empty(
            &self.page.support_export_summary,
            "work_item_sync_beta.page_summary_missing",
            Some(&self.page.page_id),
        );
        for contract_ref in self.page.contract_refs.all_refs() {
            self.expect_non_empty(
                contract_ref,
                "work_item_sync_beta.contract_ref_missing",
                Some(&self.page.page_id),
            );
        }
        self.expect(
            !self.page.work_item_link_records.is_empty(),
            WorkItemSyncBetaDefectKind::RequiredFieldMissing,
            "work_item_sync_beta.links_missing",
            "page must carry at least one work-item link state record",
            Some(&self.page.page_id),
        );
        self.expect(
            !self.page.comment_sync_records.is_empty(),
            WorkItemSyncBetaDefectKind::RequiredFieldMissing,
            "work_item_sync_beta.comments_missing",
            "page must carry at least one comment sync state record",
            Some(&self.page.page_id),
        );
        self.expect(
            !self.page.publish_reviews.is_empty(),
            WorkItemSyncBetaDefectKind::RequiredFieldMissing,
            "work_item_sync_beta.publish_reviews_missing",
            "page must carry at least one publish-review record",
            Some(&self.page.page_id),
        );
    }

    fn validate_links(&mut self) {
        for record in &self.page.work_item_link_records {
            self.expect(
                record.record_kind == WORK_ITEM_LINK_STATE_RECORD_KIND,
                WorkItemSyncBetaDefectKind::PageContractInvalid,
                "work_item_sync_beta.link_record_kind",
                "link.record_kind must be work_item_link_state_record",
                Some(&record.link_id),
            );
            self.expect(
                record.schema_version == WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
                WorkItemSyncBetaDefectKind::PageContractInvalid,
                "work_item_sync_beta.link_schema_version",
                "link.schema_version must match the crate constant",
                Some(&record.link_id),
            );
            let unique = self.link_ids.insert(record.link_id.as_str());
            self.expect(
                unique,
                WorkItemSyncBetaDefectKind::DuplicateId,
                "work_item_sync_beta.link_duplicate_id",
                "link ids must be unique",
                Some(&record.link_id),
            );
            for (value, check) in [
                (&record.link_id, "work_item_sync_beta.link_id_missing"),
                (
                    &record.work_item_detail_record_id_ref,
                    "work_item_sync_beta.link_detail_ref_missing",
                ),
                (
                    &record.provider_descriptor_ref,
                    "work_item_sync_beta.link_provider_descriptor_missing",
                ),
                (
                    &record.support_export_summary,
                    "work_item_sync_beta.link_summary_missing",
                ),
            ] {
                self.expect_non_empty(value, check, Some(&record.link_id));
            }
            self.expect(
                !record.raw_provider_payload_refs_present && !record.raw_provider_url_present,
                WorkItemSyncBetaDefectKind::RawProviderMaterialPresent,
                "work_item_sync_beta.link_raw_provider_material_present",
                "link rows must not carry raw provider payloads or URLs",
                Some(&record.link_id),
            );
            self.validate_link_relations(record);
            self.coverage.link_kinds.insert(record.link_kind_class);
            self.coverage
                .link_source_classes
                .insert(record.link_source_class);
            self.coverage
                .link_relation_states
                .insert(record.link_relation_state_class);
            self.coverage
                .link_sync_pending_states
                .insert(record.link_sync_pending_state_class);
            self.coverage
                .link_conflict_postures
                .insert(record.link_conflict_resolution_posture_class);
        }
    }

    fn validate_link_relations(&mut self, record: &WorkItemLinkStateRecord) {
        match record.link_kind_class {
            WorkItemLinkKindClass::BranchOrWorktreeLink => self.expect(
                non_empty_opt(&record.linked_branch_local_locator_ref),
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_branch_ref_missing",
                "branch links must cite a branch locator ref",
                Some(&record.link_id),
            ),
            WorkItemLinkKindClass::ReviewWorkspaceLink => self.expect(
                non_empty_opt(&record.linked_review_workspace_record_id_ref),
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_review_ref_missing",
                "review links must cite a review workspace ref",
                Some(&record.link_id),
            ),
            WorkItemLinkKindClass::ChangeObjectLink => self.expect(
                non_empty_opt(&record.linked_change_object_record_id_ref),
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_change_ref_missing",
                "change-object links must cite a change object ref",
                Some(&record.link_id),
            ),
            WorkItemLinkKindClass::WorkItemPeerLink => self.expect(
                non_empty_opt(&record.linked_peer_work_item_detail_record_id_ref),
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_peer_ref_missing",
                "peer links must cite a peer work-item detail ref",
                Some(&record.link_id),
            ),
            WorkItemLinkKindClass::HandoffPacketLink => self.expect(
                non_empty_opt(&record.linked_handoff_packet_record_id_ref)
                    || non_empty_opt(&record.linked_offline_handoff_packet_record_id_ref)
                    || non_empty_opt(&record.linked_browser_handoff_packet_ref),
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_handoff_ref_missing",
                "handoff links must cite a handoff packet ref",
                Some(&record.link_id),
            ),
        }
        match record.link_relation_state_class {
            LinkRelationStateClass::LinkedActiveProviderConfirmed => self.expect(
                matches!(
                    record.link_source_class,
                    LinkSourceClass::ProviderAuthoritativeOverlay
                        | LinkSourceClass::ProviderObservedCachedShadow
                ) && record.link_local_draft_state_class
                    == LinkLocalDraftStateClass::NoLocalDraftEdit
                    && record.link_conflict_resolution_posture_class
                        == LinkConflictResolutionPostureClass::NoConflictDetected,
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_active_truth_incoherent",
                "provider-confirmed links must use provider source, no local draft, and no conflict",
                Some(&record.link_id),
            ),
            LinkRelationStateClass::LinkedLocalDraftPendingPublish
            | LinkRelationStateClass::LinkedQueuedForPublishLater => {
                self.expect(
                    record.link_local_draft_state_class
                        == LinkLocalDraftStateClass::LocalDraftCreateLinkPendingPublish,
                    WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                    "work_item_sync_beta.link_local_draft_truth",
                    "pending-publish links must carry a local-draft-create state",
                    Some(&record.link_id),
                );
                self.expect(
                    record.link_write_scope_class.admits_write(),
                    WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                    "work_item_sync_beta.link_local_draft_write_blocked",
                    "pending-publish links must claim an admissible write scope",
                    Some(&record.link_id),
                );
            }
            LinkRelationStateClass::UnlinkRequestedLocalDraft
            | LinkRelationStateClass::UnlinkQueuedForPublishLater => {
                self.expect(
                    record.link_local_draft_state_class
                        == LinkLocalDraftStateClass::LocalDraftRemoveLinkPendingPublish,
                    WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                    "work_item_sync_beta.link_unlink_truth",
                    "unlink rows must carry a local-draft-remove state",
                    Some(&record.link_id),
                );
            }
            LinkRelationStateClass::ConflictRequiresReview => self.expect(
                record.link_conflict_resolution_posture_class
                    != LinkConflictResolutionPostureClass::NoConflictDetected,
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_conflict_truth",
                "conflict-requires-review rows must declare a non-noop conflict posture",
                Some(&record.link_id),
            ),
            LinkRelationStateClass::BrokenProviderRejected
            | LinkRelationStateClass::BrokenProviderObjectMissing => self.expect(
                !record.link_write_scope_class.admits_write()
                    || record.link_write_scope_class
                        == LinkWriteScopeClass::WriteAdmissibleLocalDraftOnlyNoProviderPath,
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_broken_write_truth",
                "broken links cannot claim an immediate provider write path",
                Some(&record.link_id),
            ),
            LinkRelationStateClass::LinkedImportedSnapshotNoProviderPath => self.expect(
                record.link_source_class == LinkSourceClass::ImportedHandoffEvidenceOnly
                    && record.link_write_scope_class
                        == LinkWriteScopeClass::WriteBlockedImportedEvidenceOnlyNoProviderPath,
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_imported_truth",
                "imported-snapshot links must declare imported source and imported write scope",
                Some(&record.link_id),
            ),
            LinkRelationStateClass::LinkedThroughBrowserHandoffOnly => self.expect(
                non_empty_opt(&record.linked_browser_handoff_packet_ref),
                WorkItemSyncBetaDefectKind::LinkTruthIncoherent,
                "work_item_sync_beta.link_browser_handoff_truth",
                "browser-handoff-only links must cite a browser-handoff packet",
                Some(&record.link_id),
            ),
        }
    }

    fn validate_comment_syncs(&mut self) {
        for record in &self.page.comment_sync_records {
            self.expect(
                record.record_kind == COMMENT_SYNC_STATE_RECORD_KIND,
                WorkItemSyncBetaDefectKind::PageContractInvalid,
                "work_item_sync_beta.comment_record_kind",
                "comment.record_kind must be comment_sync_state_record",
                Some(&record.comment_sync_id),
            );
            self.expect(
                record.schema_version == WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
                WorkItemSyncBetaDefectKind::PageContractInvalid,
                "work_item_sync_beta.comment_schema_version",
                "comment.schema_version must match the crate constant",
                Some(&record.comment_sync_id),
            );
            let unique = self
                .comment_sync_ids
                .insert(record.comment_sync_id.as_str());
            self.expect(
                unique,
                WorkItemSyncBetaDefectKind::DuplicateId,
                "work_item_sync_beta.comment_duplicate_id",
                "comment sync ids must be unique",
                Some(&record.comment_sync_id),
            );
            for (value, check) in [
                (
                    &record.comment_sync_id,
                    "work_item_sync_beta.comment_id_missing",
                ),
                (
                    &record.work_item_detail_record_id_ref,
                    "work_item_sync_beta.comment_detail_ref_missing",
                ),
                (
                    &record.support_export_summary,
                    "work_item_sync_beta.comment_summary_missing",
                ),
            ] {
                self.expect_non_empty(value, check, Some(&record.comment_sync_id));
            }
            self.expect(
                !record.raw_comment_text_present && !record.raw_provider_url_present,
                WorkItemSyncBetaDefectKind::RawProviderMaterialPresent,
                "work_item_sync_beta.comment_raw_material_present",
                "comment rows must not carry raw comment text or provider URLs",
                Some(&record.comment_sync_id),
            );
            self.validate_comment_truth(record);
            self.coverage
                .comment_origin_classes
                .insert(record.comment_origin_class);
            self.coverage
                .comment_lifecycle_classes
                .insert(record.comment_lifecycle_class);
            self.coverage
                .comment_sync_states
                .insert(record.comment_sync_state_class);
            self.coverage
                .comment_publish_postures
                .insert(record.comment_publish_posture_class);
            self.coverage
                .comment_conflict_classes
                .insert(record.comment_conflict_class);
        }
    }

    fn validate_comment_truth(&mut self, record: &CommentSyncStateRecord) {
        match record.comment_origin_class {
            CommentOriginClass::ProviderAuthoritativeComment => self.expect(
                record.provider_comment_record_id_ref.is_some(),
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_provider_ref_missing",
                "provider-owned comments must cite a provider comment record ref",
                Some(&record.comment_sync_id),
            ),
            CommentOriginClass::LocalDraftComment => self.expect(
                record.local_draft_comment_record_id_ref.is_some()
                    && record.comment_publish_posture_class
                        != CommentPublishPostureClass::ProviderPublishedObserved,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_local_draft_truth",
                "local-draft comments must cite a local draft and cannot claim provider publish",
                Some(&record.comment_sync_id),
            ),
            CommentOriginClass::OfflineCapturePacketComment => self.expect(
                record.offline_handoff_packet_record_id_ref.is_some(),
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_offline_ref_missing",
                "offline-capture comments must cite an offline-handoff packet",
                Some(&record.comment_sync_id),
            ),
            CommentOriginClass::ImportedSnapshotComment => self.expect(
                record.comment_publish_posture_class
                    == CommentPublishPostureClass::ImportedEvidenceOnly,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_imported_posture",
                "imported-snapshot comments must use imported_evidence_only posture",
                Some(&record.comment_sync_id),
            ),
            CommentOriginClass::AiProposedDraftComment => self.expect(
                record.local_draft_comment_record_id_ref.is_some()
                    && record.comment_publish_posture_class
                        == CommentPublishPostureClass::LocalDraftNeverPublished,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_ai_draft_truth",
                "AI-proposed draft comments must cite a local draft and be never_published",
                Some(&record.comment_sync_id),
            ),
        }
        match record.comment_sync_state_class {
            CommentSyncStateClass::InSyncProviderObserved => self.expect(
                matches!(
                    record.comment_lifecycle_class,
                    CommentLifecycleClass::ProviderObservedActive
                        | CommentLifecycleClass::ProviderObservedEdited
                        | CommentLifecycleClass::ProviderObservedDeleted
                ) && record.comment_conflict_class == CommentConflictClass::NoConflict,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_in_sync_truth",
                "in-sync comments must use a provider-observed lifecycle and no conflict",
                Some(&record.comment_sync_id),
            ),
            CommentSyncStateClass::QueuedPublishDeferred => self.expect(
                record
                    .linked_publish_later_queue_item_record_id_ref
                    .is_some()
                    && record.comment_publish_posture_class
                        == CommentPublishPostureClass::QueuedForPublishLater,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_queued_truth",
                "queued comments must cite a publish-later queue item and queued posture",
                Some(&record.comment_sync_id),
            ),
            CommentSyncStateClass::PendingDrainOfflineCaptured => self.expect(
                record.offline_handoff_packet_record_id_ref.is_some()
                    && record.comment_publish_posture_class
                        == CommentPublishPostureClass::OfflineCapturedNotSubmitted,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_offline_drain_truth",
                "offline-captured pending-drain comments must cite an offline-handoff packet",
                Some(&record.comment_sync_id),
            ),
            CommentSyncStateClass::PublishFailedTypedRetry => self.expect(
                record.typed_retry_route_class.is_some()
                    && record.comment_publish_posture_class
                        == CommentPublishPostureClass::PublishFailedAwaitingRetry,
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_failed_retry_truth",
                "publish-failed comments must declare a typed retry route and failed posture",
                Some(&record.comment_sync_id),
            ),
            CommentSyncStateClass::ConflictDetectedAwaitingResolution => self.expect(
                record.comment_conflict_class.is_conflict(),
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_conflict_truth",
                "conflict-detected comments must name a non-noop conflict class",
                Some(&record.comment_sync_id),
            ),
            CommentSyncStateClass::PendingPublishLocalDraft => self.expect(
                record.local_draft_comment_record_id_ref.is_some(),
                WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent,
                "work_item_sync_beta.comment_pending_local_truth",
                "pending-publish local comments must cite a local draft",
                Some(&record.comment_sync_id),
            ),
            CommentSyncStateClass::PendingProviderCallbackAfterPublish
            | CommentSyncStateClass::StaleProviderUnreachable => {}
        }
    }

    fn validate_publish_reviews(&mut self) {
        for record in &self.page.publish_reviews {
            self.expect(
                record.record_kind == PUBLISH_REVIEW_RECORD_KIND,
                WorkItemSyncBetaDefectKind::PageContractInvalid,
                "work_item_sync_beta.publish_review_record_kind",
                "publish_review.record_kind must be work_item_publish_review_record",
                Some(&record.publish_review_id),
            );
            self.expect(
                record.schema_version == WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
                WorkItemSyncBetaDefectKind::PageContractInvalid,
                "work_item_sync_beta.publish_review_schema_version",
                "publish_review.schema_version must match the crate constant",
                Some(&record.publish_review_id),
            );
            let unique = self
                .publish_review_ids
                .insert(record.publish_review_id.as_str());
            self.expect(
                unique,
                WorkItemSyncBetaDefectKind::DuplicateId,
                "work_item_sync_beta.publish_review_duplicate_id",
                "publish-review ids must be unique",
                Some(&record.publish_review_id),
            );
            for (value, check) in [
                (
                    &record.publish_review_id,
                    "work_item_sync_beta.publish_review_id_missing",
                ),
                (
                    &record.work_item_detail_record_id_ref,
                    "work_item_sync_beta.publish_review_detail_ref_missing",
                ),
                (
                    &record.summary,
                    "work_item_sync_beta.publish_review_summary_missing",
                ),
            ] {
                self.expect_non_empty(value, check, Some(&record.publish_review_id));
            }
            self.expect(
                !record.raw_provider_payload_refs_present,
                WorkItemSyncBetaDefectKind::RawProviderMaterialPresent,
                "work_item_sync_beta.publish_review_raw_provider_material_present",
                "publish-review rows must not carry raw provider payload refs",
                Some(&record.publish_review_id),
            );
            self.expect(
                !record.side_effect_rows.is_empty(),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_side_effects_missing",
                "publish-review rows must disclose side effects",
                Some(&record.publish_review_id),
            );
            self.expect(
                record.action_affordances.export_action_available
                    && record.action_affordances.cancel_action_available,
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_export_cancel_missing",
                "publish-review rows must expose export and cancel actions",
                Some(&record.publish_review_id),
            );
            self.expect(
                record.local_draft_preserved_on_failure,
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_local_draft_not_preserved",
                "publish-review rows must preserve local draft state on publish failure",
                Some(&record.publish_review_id),
            );
            if record.publish_review_disposition_class.is_admissible() {
                self.expect(
                    record.action_affordances.confirm_action_available,
                    WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                    "work_item_sync_beta.publish_review_confirm_missing",
                    "admissible publish-review rows must expose a confirm action",
                    Some(&record.publish_review_id),
                );
            }
            self.validate_publish_review_truth(record);
            for row in &record.side_effect_rows {
                self.expect_non_empty(
                    &row.fanout_row_id,
                    "work_item_sync_beta.publish_review_fanout_id_missing",
                    Some(&record.publish_review_id),
                );
                self.expect_non_empty(
                    &row.summary,
                    "work_item_sync_beta.publish_review_fanout_summary_missing",
                    Some(&record.publish_review_id),
                );
                self.coverage
                    .publish_review_side_effect_classes
                    .insert(row.side_effect_class);
            }
            self.coverage
                .publish_review_action_classes
                .insert(record.publish_review_action_class);
            self.coverage
                .publish_review_disposition_classes
                .insert(record.publish_review_disposition_class);
        }
    }

    fn validate_publish_review_truth(&mut self, record: &PublishReviewRecord) {
        if record.publish_review_action_class.is_link_action() {
            self.expect(
                record.work_item_link_state_record_id_ref.is_some(),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_link_ref_missing",
                "link/unlink publish-review rows must cite a work-item link state record",
                Some(&record.publish_review_id),
            );
            if let Some(link_ref) = &record.work_item_link_state_record_id_ref {
                self.expect(
                    self.link_ids.contains(link_ref.as_str()),
                    WorkItemSyncBetaDefectKind::UnknownRecordReference,
                    "work_item_sync_beta.publish_review_link_unknown",
                    "publish-review link ref must bind an existing link record",
                    Some(&record.publish_review_id),
                );
            }
        }
        if record.publish_review_action_class.is_comment_action() {
            self.expect(
                record.comment_sync_state_record_id_ref.is_some(),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_comment_ref_missing",
                "comment publish-review rows must cite a comment sync state record",
                Some(&record.publish_review_id),
            );
            if let Some(comment_ref) = &record.comment_sync_state_record_id_ref {
                self.expect(
                    self.comment_sync_ids.contains(comment_ref.as_str()),
                    WorkItemSyncBetaDefectKind::UnknownRecordReference,
                    "work_item_sync_beta.publish_review_comment_unknown",
                    "publish-review comment ref must bind an existing comment record",
                    Some(&record.publish_review_id),
                );
            }
        }
        if record.publish_review_action_class
            == PublishReviewActionClass::StatusTransitionPlusComment
        {
            self.expect(
                non_empty_opt(&record.status_transition_packet_record_id_ref),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_transition_ref_missing",
                "status-transition-plus-comment rows must cite a status-transition packet",
                Some(&record.publish_review_id),
            );
        }
        match record.publish_review_disposition_class {
            PublishReviewDispositionClass::AdmissibleNowPublishNow => self.expect(
                record.publish_mode_class == WorkItemMutationMode::PublishNow,
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_publish_now_mode",
                "publish-now rows must use publish_now mode",
                Some(&record.publish_review_id),
            ),
            PublishReviewDispositionClass::AdmissibleViaQueueForPublishLater => self.expect(
                record.publish_mode_class == WorkItemMutationMode::DeferredPublish
                    && non_empty_opt(&record.linked_publish_later_queue_item_record_id_ref),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_queue_truth",
                "queued publish-review rows must use deferred_publish mode and cite a queue item",
                Some(&record.publish_review_id),
            ),
            PublishReviewDispositionClass::AdmissibleViaBrowserHandoffOnly => self.expect(
                record.publish_mode_class == WorkItemMutationMode::OpenInProvider
                    && non_empty_opt(&record.linked_browser_handoff_packet_ref),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_browser_truth",
                "browser-handoff publish-review rows must cite a browser-handoff packet",
                Some(&record.publish_review_id),
            ),
            PublishReviewDispositionClass::AdmissibleLocalDraftOnly => self.expect(
                record.publish_mode_class == WorkItemMutationMode::LocalDraft,
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_local_truth",
                "local-draft publish-review rows must use local_draft mode",
                Some(&record.publish_review_id),
            ),
            PublishReviewDispositionClass::AdmissibleViaRetryAfterConflict => self.expect(
                matches!(
                    record.publish_review_action_class,
                    PublishReviewActionClass::RetryAfterConflict
                        | PublishReviewActionClass::RetryAfterFailure
                ),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_retry_truth",
                "retry dispositions must use a retry-after-conflict or retry-after-failure action",
                Some(&record.publish_review_id),
            ),
            PublishReviewDispositionClass::BlockedPendingConflictResolution
            | PublishReviewDispositionClass::BlockedProviderReadOnly
            | PublishReviewDispositionClass::BlockedByPolicy
            | PublishReviewDispositionClass::BlockedFreshnessFloorUnsatisfied
            | PublishReviewDispositionClass::BlockedAccountMappingPending => self.expect(
                non_empty_opt(&record.block_reason_summary),
                WorkItemSyncBetaDefectKind::PublishReviewTruthIncoherent,
                "work_item_sync_beta.publish_review_block_reason_missing",
                "blocked publish-review rows must carry a typed block reason summary",
                Some(&record.publish_review_id),
            ),
        }
    }

    fn validate_required_coverage(&mut self) {
        for kind in [
            WorkItemLinkKindClass::BranchOrWorktreeLink,
            WorkItemLinkKindClass::ReviewWorkspaceLink,
        ] {
            self.expect(
                self.coverage.link_kinds.contains(&kind),
                WorkItemSyncBetaDefectKind::CoverageMissing,
                "work_item_sync_beta.coverage_link_kind_missing",
                "page must cover branch and review link kinds",
                Some(kind.as_str()),
            );
        }
        for state in [
            LinkRelationStateClass::LinkedActiveProviderConfirmed,
            LinkRelationStateClass::LinkedLocalDraftPendingPublish,
            LinkRelationStateClass::LinkedQueuedForPublishLater,
            LinkRelationStateClass::UnlinkRequestedLocalDraft,
            LinkRelationStateClass::ConflictRequiresReview,
        ] {
            self.expect(
                self.coverage.link_relation_states.contains(&state),
                WorkItemSyncBetaDefectKind::CoverageMissing,
                "work_item_sync_beta.coverage_link_relation_missing",
                "page must cover active, local-draft, queued, unlink, and conflict link states",
                Some("link_relation_state"),
            );
        }
        for origin in [
            CommentOriginClass::ProviderAuthoritativeComment,
            CommentOriginClass::LocalDraftComment,
            CommentOriginClass::OfflineCapturePacketComment,
        ] {
            self.expect(
                self.coverage.comment_origin_classes.contains(&origin),
                WorkItemSyncBetaDefectKind::CoverageMissing,
                "work_item_sync_beta.coverage_comment_origin_missing",
                "page must cover provider, local, and offline comment origins",
                Some("comment_origin"),
            );
        }
        for state in [
            CommentSyncStateClass::InSyncProviderObserved,
            CommentSyncStateClass::PendingPublishLocalDraft,
            CommentSyncStateClass::QueuedPublishDeferred,
            CommentSyncStateClass::PendingDrainOfflineCaptured,
            CommentSyncStateClass::PublishFailedTypedRetry,
            CommentSyncStateClass::ConflictDetectedAwaitingResolution,
        ] {
            self.expect(
                self.coverage.comment_sync_states.contains(&state),
                WorkItemSyncBetaDefectKind::CoverageMissing,
                "work_item_sync_beta.coverage_comment_sync_state_missing",
                "page must cover in-sync, pending, queued, offline, failed, and conflict comment states",
                Some("comment_sync_state"),
            );
        }
        for action in [
            PublishReviewActionClass::CreateProviderComment,
            PublishReviewActionClass::EditProviderComment,
            PublishReviewActionClass::DeleteProviderComment,
            PublishReviewActionClass::LinkBranchOrReview,
            PublishReviewActionClass::UnlinkBranchOrReview,
            PublishReviewActionClass::StatusTransitionPlusComment,
            PublishReviewActionClass::RetryAfterConflict,
        ] {
            self.expect(
                self.coverage.publish_review_action_classes.contains(&action),
                WorkItemSyncBetaDefectKind::CoverageMissing,
                "work_item_sync_beta.coverage_publish_review_action_missing",
                "page must cover create/edit/delete comment, link/unlink, status-transition-plus-comment, and retry-after-conflict actions",
                Some("publish_review_action"),
            );
        }
    }

    fn expect_non_empty(&mut self, value: &str, check_id: &str, record_ref: Option<&str>) {
        self.expect(
            !value.trim().is_empty(),
            WorkItemSyncBetaDefectKind::RequiredFieldMissing,
            check_id,
            "required value must be non-empty",
            record_ref,
        );
    }

    fn expect(
        &mut self,
        passed: bool,
        defect_kind: WorkItemSyncBetaDefectKind,
        check_id: &str,
        message: &str,
        record_ref: Option<&str>,
    ) {
        if !passed {
            self.defects.push(WorkItemSyncBetaDefect {
                defect_kind,
                check_id: check_id.to_string(),
                message: message.to_string(),
                record_ref: record_ref.map(str::to_string),
            });
        }
    }
}

fn non_empty_opt(value: &Option<String>) -> bool {
    value
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
}

/// Returns a seeded beta page covering provider/local/offline/queued/conflict
/// link, comment-sync, and publish-review lanes.
pub fn seeded_work_item_sync_beta_page() -> WorkItemSyncBetaPage {
    let work_item_link_records = seeded_link_records();
    let comment_sync_records = seeded_comment_sync_records();
    let publish_reviews = seeded_publish_reviews();
    WorkItemSyncBetaPage {
        fixture_metadata: Some(WorkItemSyncFixtureMetadata {
            name: "provider_work_item_sync_beta".to_string(),
            scenario: "Provider link state, comment-sync state, and publish-review sheets cover provider-authoritative, local-draft, queued, offline, failed, and conflicted lanes so tracked-work proof never collapses provider truth, local drafts, and handoff packets into one state.".to_string(),
        }),
        record_kind: WORK_ITEM_SYNC_BETA_PAGE_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
        shared_contract_ref: WORK_ITEM_SYNC_BETA_SHARED_CONTRACT_REF.to_string(),
        page_id: "providers.work_item_sync_beta.page".to_string(),
        contract_refs: WorkItemSyncContractRefs {
            work_item_transition_beta_shared_contract_ref:
                "providers:work_item_transition_beta:v1".to_string(),
            work_item_link_state_schema_ref:
                "schemas/providers/work_item_link_state.schema.json".to_string(),
            comment_sync_state_schema_ref:
                "schemas/providers/comment_sync_state.schema.json".to_string(),
            publish_review_schema_ref: "schemas/providers/publish_review.schema.json".to_string(),
            publish_later_queue_schema_ref:
                "schemas/providers/publish_later_queue_alpha.schema.json".to_string(),
            browser_handoff_packet_schema_ref:
                "schemas/providers/browser_handoff_packet.schema.json".to_string(),
            offline_handoff_packet_schema_ref:
                "schemas/providers/offline_handoff_packet.schema.json".to_string(),
            connected_provider_registry_schema_ref:
                "schemas/providers/connected_provider_registry.schema.json".to_string(),
        },
        work_item_link_records,
        comment_sync_records,
        publish_reviews,
        support_export_summary: "Provider-backed work-item link, comment-sync, and publish-review rows keep provider truth, local drafts, queued publishes, offline handoffs, and conflicts on separate enum lanes for support and release review.".to_string(),
    }
}

fn seeded_link_records() -> Vec<WorkItemLinkStateRecord> {
    vec![
        link_record(
            "work_item_sync:link:branch-provider-confirmed",
            "work_items:detail:provider-authoritative",
            WorkItemLinkKindClass::BranchOrWorktreeLink,
            LinkSourceClass::ProviderAuthoritativeOverlay,
            LinkRelationStateClass::LinkedActiveProviderConfirmed,
            WorkItemFreshnessClass::LiveAuthoritativeFresh,
            LinkWriteScopeClass::WriteAdmissibleProviderWriteable,
            LinkLocalDraftStateClass::NoLocalDraftEdit,
            LinkSyncPendingStateClass::InSyncProviderObserved,
            LinkConflictResolutionPostureClass::NoConflictDetected,
            BranchedRefs {
                branch: Some("workspace:branch:work-item"),
                review: None,
                change: None,
                peer: None,
                handoff: None,
                queue: None,
                offline: None,
                browser: None,
            },
            TrustPosture::Trusted,
            "Provider-confirmed branch link stays bound to the same work-item object model.",
        ),
        link_record(
            "work_item_sync:link:review-local-draft-pending",
            "work_items:detail:local-draft",
            WorkItemLinkKindClass::ReviewWorkspaceLink,
            LinkSourceClass::LocalAuthoredNoProviderObject,
            LinkRelationStateClass::LinkedLocalDraftPendingPublish,
            WorkItemFreshnessClass::LocalDraftNeverPublished,
            LinkWriteScopeClass::WriteAdmissibleLocalDraftOnlyNoProviderPath,
            LinkLocalDraftStateClass::LocalDraftCreateLinkPendingPublish,
            LinkSyncPendingStateClass::PendingPublishLocalDraft,
            LinkConflictResolutionPostureClass::NoConflictDetected,
            BranchedRefs {
                branch: None,
                review: Some("vcs:review_workspace:work-item"),
                change: None,
                peer: None,
                handoff: None,
                queue: None,
                offline: None,
                browser: None,
            },
            TrustPosture::Trusted,
            "Local-draft review link is pending publish without claiming provider truth.",
        ),
        link_record(
            "work_item_sync:link:review-queued",
            "work_items:detail:queued",
            WorkItemLinkKindClass::ReviewWorkspaceLink,
            LinkSourceClass::LocalAuthoredNoProviderObject,
            LinkRelationStateClass::LinkedQueuedForPublishLater,
            WorkItemFreshnessClass::LocalDraftNeverPublished,
            LinkWriteScopeClass::WriteAdmissibleQueuedPublishOnly,
            LinkLocalDraftStateClass::LocalDraftCreateLinkPendingPublish,
            LinkSyncPendingStateClass::PendingPublishQueued,
            LinkConflictResolutionPostureClass::NoConflictDetected,
            BranchedRefs {
                branch: None,
                review: Some("vcs:review_workspace:work-item-queued"),
                change: None,
                peer: None,
                handoff: None,
                queue: Some("providers:queue_item:link:queued"),
                offline: None,
                browser: None,
            },
            TrustPosture::Trusted,
            "Queued review link awaits publish-later drain instead of provider confirmation.",
        ),
        link_record(
            "work_item_sync:link:branch-unlink-pending",
            "work_items:detail:cached-stale",
            WorkItemLinkKindClass::BranchOrWorktreeLink,
            LinkSourceClass::LocalAuthoredNoProviderObject,
            LinkRelationStateClass::UnlinkRequestedLocalDraft,
            WorkItemFreshnessClass::WarmWithinGrace,
            LinkWriteScopeClass::WriteAdmissibleLocalDraftOnlyNoProviderPath,
            LinkLocalDraftStateClass::LocalDraftRemoveLinkPendingPublish,
            LinkSyncPendingStateClass::PendingPublishLocalDraft,
            LinkConflictResolutionPostureClass::NoConflictDetected,
            BranchedRefs {
                branch: Some("workspace:branch:work-item-stale"),
                review: None,
                change: None,
                peer: None,
                handoff: None,
                queue: None,
                offline: None,
                browser: None,
            },
            TrustPosture::Trusted,
            "Local unlink request awaits publish; provider state remains the truth until then.",
        ),
        link_record(
            "work_item_sync:link:branch-conflict",
            "work_items:detail:cached-stale",
            WorkItemLinkKindClass::BranchOrWorktreeLink,
            LinkSourceClass::ProviderAuthoritativeOverlay,
            LinkRelationStateClass::ConflictRequiresReview,
            WorkItemFreshnessClass::DegradedBeyondGraceLocalContinues,
            LinkWriteScopeClass::WriteBlockedProviderUnreachable,
            LinkLocalDraftStateClass::LocalDraftEditConflictsWithProvider,
            LinkSyncPendingStateClass::StaleProviderUnreachable,
            LinkConflictResolutionPostureClass::ConflictLocalEditConflictsAwaitingUserChoice,
            BranchedRefs {
                branch: Some("workspace:branch:work-item-conflict"),
                review: None,
                change: None,
                peer: None,
                handoff: None,
                queue: None,
                offline: None,
                browser: None,
            },
            TrustPosture::Trusted,
            "Branch link conflict: local edit disagrees with provider truth and awaits user choice.",
        ),
        link_record(
            "work_item_sync:link:imported-handoff",
            "work_items:detail:offline-captured",
            WorkItemLinkKindClass::HandoffPacketLink,
            LinkSourceClass::ImportedHandoffEvidenceOnly,
            LinkRelationStateClass::LinkedImportedSnapshotNoProviderPath,
            WorkItemFreshnessClass::ImportedSnapshotNoRefreshPath,
            LinkWriteScopeClass::WriteBlockedImportedEvidenceOnlyNoProviderPath,
            LinkLocalDraftStateClass::NoLocalDraftEdit,
            LinkSyncPendingStateClass::StaleFreshnessFloorUnsatisfied,
            LinkConflictResolutionPostureClass::NoConflictDetected,
            BranchedRefs {
                branch: None,
                review: None,
                change: None,
                peer: None,
                handoff: Some("work_items:offline_handoff:imported-evidence"),
                queue: None,
                offline: Some("work_items:offline_handoff:imported-evidence"),
                browser: None,
            },
            TrustPosture::Trusted,
            "Imported handoff link has no live provider path and stays evidence-only.",
        ),
    ]
}

struct BranchedRefs {
    branch: Option<&'static str>,
    review: Option<&'static str>,
    change: Option<&'static str>,
    peer: Option<&'static str>,
    handoff: Option<&'static str>,
    queue: Option<&'static str>,
    offline: Option<&'static str>,
    browser: Option<&'static str>,
}

#[allow(clippy::too_many_arguments)]
fn link_record(
    link_id: &str,
    detail_ref: &str,
    kind: WorkItemLinkKindClass,
    source: LinkSourceClass,
    relation: LinkRelationStateClass,
    freshness: WorkItemFreshnessClass,
    write_scope: LinkWriteScopeClass,
    local_draft: LinkLocalDraftStateClass,
    sync_pending: LinkSyncPendingStateClass,
    conflict: LinkConflictResolutionPostureClass,
    refs: BranchedRefs,
    trust_state: TrustPosture,
    summary: &str,
) -> WorkItemLinkStateRecord {
    WorkItemLinkStateRecord {
        record_kind: WORK_ITEM_LINK_STATE_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
        link_id: link_id.to_string(),
        work_item_detail_record_id_ref: detail_ref.to_string(),
        provider_descriptor_ref: "provider_descriptor.issue.primary".to_string(),
        provider_family: ProviderFamily::IssueTracker,
        link_kind_class: kind,
        link_source_class: source,
        link_relation_state_class: relation,
        link_freshness_class: freshness,
        link_write_scope_class: write_scope,
        link_local_draft_state_class: local_draft,
        link_sync_pending_state_class: sync_pending,
        link_conflict_resolution_posture_class: conflict,
        linked_branch_local_locator_ref: refs.branch.map(str::to_string),
        linked_review_workspace_record_id_ref: refs.review.map(str::to_string),
        linked_change_object_record_id_ref: refs.change.map(str::to_string),
        linked_peer_work_item_detail_record_id_ref: refs.peer.map(str::to_string),
        linked_handoff_packet_record_id_ref: refs.handoff.map(str::to_string),
        linked_publish_later_queue_item_record_id_ref: refs.queue.map(str::to_string),
        linked_offline_handoff_packet_record_id_ref: refs.offline.map(str::to_string),
        linked_browser_handoff_packet_ref: refs.browser.map(str::to_string),
        origin_disclosure: origin("exec:work-item-sync-link"),
        policy_context: WorkItemPolicyContext {
            policy_epoch: "policy:epoch:2026-05-18".to_string(),
            trust_state,
            execution_context_id: "exec:work-item-sync-link".to_string(),
            policy_block_ref: None,
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_provider_payload_refs_present: false,
        raw_provider_url_present: false,
        captured_at: "2026-05-18T09:15:00Z".to_string(),
        support_export_summary: summary.to_string(),
    }
}

fn seeded_comment_sync_records() -> Vec<CommentSyncStateRecord> {
    vec![
        comment_record(
            "work_item_sync:comment:provider-active",
            "work_items:detail:provider-authoritative",
            CommentRefs {
                provider: Some("provider:comment:active"),
                local: None,
                offline: None,
                queue: None,
                browser: None,
            },
            CommentOriginClass::ProviderAuthoritativeComment,
            CommentLifecycleClass::ProviderObservedActive,
            CommentSyncStateClass::InSyncProviderObserved,
            CommentPublishPostureClass::ProviderPublishedObserved,
            CommentConflictClass::NoConflict,
            None,
            "Provider-owned comment is observed authoritative and in sync.",
        ),
        comment_record(
            "work_item_sync:comment:local-draft-pending",
            "work_items:detail:local-draft",
            CommentRefs {
                provider: None,
                local: Some("local_draft:comment:pending"),
                offline: None,
                queue: None,
                browser: None,
            },
            CommentOriginClass::LocalDraftComment,
            CommentLifecycleClass::LocalDraftCreateNeverPublished,
            CommentSyncStateClass::PendingPublishLocalDraft,
            CommentPublishPostureClass::LocalDraftNeverPublished,
            CommentConflictClass::NoConflict,
            None,
            "Local draft comment is pending publish and is not claimed as provider truth.",
        ),
        comment_record(
            "work_item_sync:comment:queued",
            "work_items:detail:queued",
            CommentRefs {
                provider: None,
                local: Some("local_draft:comment:queued"),
                offline: None,
                queue: Some("providers:queue_item:comment:queued"),
                browser: None,
            },
            CommentOriginClass::LocalDraftComment,
            CommentLifecycleClass::LocalDraftCreateNeverPublished,
            CommentSyncStateClass::QueuedPublishDeferred,
            CommentPublishPostureClass::QueuedForPublishLater,
            CommentConflictClass::NoConflict,
            None,
            "Queued comment awaits publish-later drain; provider acceptance is unverified.",
        ),
        comment_record(
            "work_item_sync:comment:offline-captured",
            "work_items:detail:cached-stale",
            CommentRefs {
                provider: None,
                local: Some("local_draft:comment:offline"),
                offline: Some("work_items:offline_handoff:provider-unreachable"),
                queue: None,
                browser: None,
            },
            CommentOriginClass::OfflineCapturePacketComment,
            CommentLifecycleClass::LocalDraftCreateNeverPublished,
            CommentSyncStateClass::PendingDrainOfflineCaptured,
            CommentPublishPostureClass::OfflineCapturedNotSubmitted,
            CommentConflictClass::NoConflict,
            None,
            "Offline-captured comment preserves intent without claiming provider acceptance.",
        ),
        comment_record(
            "work_item_sync:comment:publish-failed",
            "work_items:detail:cached-stale",
            CommentRefs {
                provider: None,
                local: Some("local_draft:comment:retry"),
                offline: None,
                queue: Some("providers:queue_item:comment:retry"),
                browser: None,
            },
            CommentOriginClass::LocalDraftComment,
            CommentLifecycleClass::LocalDraftCreateNeverPublished,
            CommentSyncStateClass::PublishFailedTypedRetry,
            CommentPublishPostureClass::PublishFailedAwaitingRetry,
            CommentConflictClass::ConflictDuplicatePublishRejected,
            Some(CommentRetryRouteClass::AutoRetryOnProviderHealthRecovered),
            "Comment publish failed with typed retry route; local draft is preserved.",
        ),
        comment_record(
            "work_item_sync:comment:conflict",
            "work_items:detail:cached-stale",
            CommentRefs {
                provider: Some("provider:comment:conflict-base"),
                local: Some("local_draft:comment:conflict-edit"),
                offline: None,
                queue: None,
                browser: None,
            },
            CommentOriginClass::LocalDraftComment,
            CommentLifecycleClass::LocalDraftEditOnProviderComment,
            CommentSyncStateClass::ConflictDetectedAwaitingResolution,
            CommentPublishPostureClass::LocalDraftNeverPublished,
            CommentConflictClass::ConflictProviderEditedAfterDraft,
            None,
            "Comment conflict: provider edited after the local draft; awaiting user resolution.",
        ),
    ]
}

struct CommentRefs {
    provider: Option<&'static str>,
    local: Option<&'static str>,
    offline: Option<&'static str>,
    queue: Option<&'static str>,
    browser: Option<&'static str>,
}

#[allow(clippy::too_many_arguments)]
fn comment_record(
    comment_sync_id: &str,
    detail_ref: &str,
    refs: CommentRefs,
    origin_class: CommentOriginClass,
    lifecycle: CommentLifecycleClass,
    sync_state: CommentSyncStateClass,
    publish_posture: CommentPublishPostureClass,
    conflict: CommentConflictClass,
    retry_route: Option<CommentRetryRouteClass>,
    summary: &str,
) -> CommentSyncStateRecord {
    CommentSyncStateRecord {
        record_kind: COMMENT_SYNC_STATE_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
        comment_sync_id: comment_sync_id.to_string(),
        work_item_detail_record_id_ref: detail_ref.to_string(),
        provider_comment_record_id_ref: refs.provider.map(str::to_string),
        local_draft_comment_record_id_ref: refs.local.map(str::to_string),
        offline_handoff_packet_record_id_ref: refs.offline.map(str::to_string),
        linked_publish_later_queue_item_record_id_ref: refs.queue.map(str::to_string),
        linked_browser_handoff_packet_ref: refs.browser.map(str::to_string),
        comment_origin_class: origin_class,
        comment_lifecycle_class: lifecycle,
        comment_sync_state_class: sync_state,
        comment_publish_posture_class: publish_posture,
        comment_conflict_class: conflict,
        acting_as: ProviderActorClass::HumanAccount,
        typed_retry_route_class: retry_route,
        origin_disclosure: origin("exec:work-item-sync-comment"),
        policy_context: WorkItemPolicyContext {
            policy_epoch: "policy:epoch:2026-05-18".to_string(),
            trust_state: TrustPosture::Trusted,
            execution_context_id: "exec:work-item-sync-comment".to_string(),
            policy_block_ref: None,
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_comment_text_present: false,
        raw_provider_url_present: false,
        captured_at: "2026-05-18T09:20:00Z".to_string(),
        support_export_summary: summary.to_string(),
    }
}

fn seeded_publish_reviews() -> Vec<PublishReviewRecord> {
    vec![
        publish_review(
            "work_item_sync:publish_review:create-comment",
            "work_items:detail:provider-authoritative",
            None,
            Some("work_item_sync:comment:provider-active"),
            None,
            PublishReviewActionClass::CreateProviderComment,
            PublishReviewSourceClass::LocalDraftAuthored,
            PublishReviewActorScopeClass::HumanActorWritesProvider,
            PublishReviewDispositionClass::AdmissibleNowPublishNow,
            WorkItemMutationMode::PublishNow,
            vec![
                side_effect(
                    "fanout:create-comment:provider",
                    PublishReviewSideEffectClass::ProviderCommentMutationFanout,
                    WorkItemMutationMode::PublishNow,
                    None,
                    "Provider receives a new comment after confirm.",
                ),
                side_effect(
                    "fanout:create-comment:notification",
                    PublishReviewSideEffectClass::NotificationEmissionFanout,
                    WorkItemMutationMode::PublishNow,
                    None,
                    "Subscribers receive a provider notification after publish.",
                ),
            ],
            None,
            None,
            None,
            None,
            "Create-comment publish-review row exposes provider mutation and notification fanout.",
        ),
        publish_review(
            "work_item_sync:publish_review:edit-comment",
            "work_items:detail:provider-authoritative",
            None,
            Some("work_item_sync:comment:provider-active"),
            None,
            PublishReviewActionClass::EditProviderComment,
            PublishReviewSourceClass::LocalDraftAuthored,
            PublishReviewActorScopeClass::HumanActorWritesProvider,
            PublishReviewDispositionClass::AdmissibleNowPublishNow,
            WorkItemMutationMode::PublishNow,
            vec![side_effect(
                "fanout:edit-comment:provider",
                PublishReviewSideEffectClass::ProviderCommentMutationFanout,
                WorkItemMutationMode::PublishNow,
                None,
                "Provider receives an edited comment after confirm.",
            )],
            None,
            None,
            None,
            None,
            "Edit-comment publish-review row preserves local draft on failure.",
        ),
        publish_review(
            "work_item_sync:publish_review:delete-comment",
            "work_items:detail:provider-authoritative",
            None,
            Some("work_item_sync:comment:provider-active"),
            None,
            PublishReviewActionClass::DeleteProviderComment,
            PublishReviewSourceClass::LocalDraftAuthored,
            PublishReviewActorScopeClass::HumanActorWritesProvider,
            PublishReviewDispositionClass::AdmissibleNowPublishNow,
            WorkItemMutationMode::PublishNow,
            vec![side_effect(
                "fanout:delete-comment:provider",
                PublishReviewSideEffectClass::ProviderCommentMutationFanout,
                WorkItemMutationMode::PublishNow,
                None,
                "Provider deletes the comment after confirm.",
            )],
            None,
            None,
            None,
            None,
            "Delete-comment publish-review row leaves local draft preserved on failure.",
        ),
        publish_review(
            "work_item_sync:publish_review:link-review",
            "work_items:detail:local-draft",
            Some("work_item_sync:link:review-local-draft-pending"),
            None,
            None,
            PublishReviewActionClass::LinkBranchOrReview,
            PublishReviewSourceClass::LocalDraftAuthored,
            PublishReviewActorScopeClass::LocalOnlyNoProviderScope,
            PublishReviewDispositionClass::AdmissibleLocalDraftOnly,
            WorkItemMutationMode::LocalDraft,
            vec![side_effect(
                "fanout:link-review:local",
                PublishReviewSideEffectClass::ProviderLinkMutationFanout,
                WorkItemMutationMode::LocalDraft,
                Some("vcs:review_workspace:work-item"),
                "Local link change updates the local work-item record only.",
            )],
            None,
            None,
            None,
            None,
            "Link-review publish-review row saves the link as a local draft only.",
        ),
        publish_review(
            "work_item_sync:publish_review:unlink-branch",
            "work_items:detail:cached-stale",
            Some("work_item_sync:link:branch-unlink-pending"),
            None,
            None,
            PublishReviewActionClass::UnlinkBranchOrReview,
            PublishReviewSourceClass::LocalDraftAuthored,
            PublishReviewActorScopeClass::InstallOrAppGrantScope,
            PublishReviewDispositionClass::AdmissibleViaQueueForPublishLater,
            WorkItemMutationMode::DeferredPublish,
            vec![side_effect(
                "fanout:unlink-branch:queue",
                PublishReviewSideEffectClass::ProviderLinkMutationFanout,
                WorkItemMutationMode::DeferredPublish,
                Some("providers:queue_item:link:queued"),
                "Unlink queues for publish-later drain.",
            )],
            Some("providers:queue_item:link:queued"),
            None,
            None,
            None,
            "Unlink-branch publish-review row routes through the publish-later queue.",
        ),
        publish_review(
            "work_item_sync:publish_review:transition-plus-comment",
            "work_items:detail:provider-authoritative",
            None,
            Some("work_item_sync:comment:provider-active"),
            Some("work_items:transition_packet:publish-now"),
            PublishReviewActionClass::StatusTransitionPlusComment,
            PublishReviewSourceClass::LocalDraftAuthored,
            PublishReviewActorScopeClass::HumanActorWritesProvider,
            PublishReviewDispositionClass::AdmissibleNowPublishNow,
            WorkItemMutationMode::PublishNow,
            vec![
                side_effect(
                    "fanout:transition-plus-comment:state",
                    PublishReviewSideEffectClass::ProviderStateTransitionFanout,
                    WorkItemMutationMode::PublishNow,
                    Some("work_items:transition_packet:publish-now"),
                    "Provider state transitions and a comment is added in one confirm.",
                ),
                side_effect(
                    "fanout:transition-plus-comment:comment",
                    PublishReviewSideEffectClass::ProviderCommentMutationFanout,
                    WorkItemMutationMode::PublishNow,
                    None,
                    "Provider receives a new comment alongside the transition.",
                ),
            ],
            None,
            None,
            None,
            None,
            "Status-transition-plus-comment publish-review row binds transition packet and comment record.",
        ),
        publish_review(
            "work_item_sync:publish_review:retry-after-conflict",
            "work_items:detail:cached-stale",
            None,
            Some("work_item_sync:comment:conflict"),
            None,
            PublishReviewActionClass::RetryAfterConflict,
            PublishReviewSourceClass::ConflictResolutionFollowOn,
            PublishReviewActorScopeClass::HumanActorWritesProvider,
            PublishReviewDispositionClass::AdmissibleViaRetryAfterConflict,
            WorkItemMutationMode::PublishNow,
            vec![side_effect(
                "fanout:retry-after-conflict:provider",
                PublishReviewSideEffectClass::ProviderCommentMutationFanout,
                WorkItemMutationMode::PublishNow,
                None,
                "Retry republishes the comment after the conflict resolves.",
            )],
            None,
            None,
            None,
            None,
            "Retry-after-conflict publish-review row repackages the local draft after user resolution.",
        ),
    ]
}

fn side_effect(
    fanout_row_id: &str,
    side_effect_class: PublishReviewSideEffectClass,
    publish_mode_class: WorkItemMutationMode,
    linked_record_id_ref: Option<&str>,
    summary: &str,
) -> PublishReviewSideEffectRow {
    PublishReviewSideEffectRow {
        fanout_row_id: fanout_row_id.to_string(),
        side_effect_class,
        publish_mode_class,
        linked_record_id_ref: linked_record_id_ref.map(str::to_string),
        summary: summary.to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
fn publish_review(
    publish_review_id: &str,
    detail_ref: &str,
    link_ref: Option<&str>,
    comment_ref: Option<&str>,
    transition_ref: Option<&str>,
    action: PublishReviewActionClass,
    source: PublishReviewSourceClass,
    actor_scope: PublishReviewActorScopeClass,
    disposition: PublishReviewDispositionClass,
    mode: WorkItemMutationMode,
    side_effect_rows: Vec<PublishReviewSideEffectRow>,
    queue_ref: Option<&str>,
    browser_ref: Option<&str>,
    offline_ref: Option<&str>,
    block_reason: Option<&str>,
    summary: &str,
) -> PublishReviewRecord {
    PublishReviewRecord {
        record_kind: PUBLISH_REVIEW_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
        publish_review_id: publish_review_id.to_string(),
        work_item_detail_record_id_ref: detail_ref.to_string(),
        work_item_link_state_record_id_ref: link_ref.map(str::to_string),
        comment_sync_state_record_id_ref: comment_ref.map(str::to_string),
        status_transition_packet_record_id_ref: transition_ref.map(str::to_string),
        publish_review_action_class: action,
        publish_review_source_class: source,
        publish_review_actor_scope_class: actor_scope,
        publish_review_disposition_class: disposition,
        publish_mode_class: mode,
        acting_as: ProviderActorClass::HumanAccount,
        side_effect_rows,
        action_affordances: PublishReviewActionAffordances {
            confirm_action_available: disposition.is_admissible(),
            export_action_available: true,
            cancel_action_available: true,
            discard_draft_action_available: true,
        },
        linked_publish_later_queue_item_record_id_ref: queue_ref.map(str::to_string),
        linked_browser_handoff_packet_ref: browser_ref.map(str::to_string),
        linked_offline_handoff_packet_record_id_ref: offline_ref.map(str::to_string),
        local_draft_preserved_on_failure: true,
        block_reason_summary: block_reason.map(str::to_string),
        origin_disclosure: origin("exec:work-item-sync-publish-review"),
        policy_context: WorkItemPolicyContext {
            policy_epoch: "policy:epoch:2026-05-18".to_string(),
            trust_state: TrustPosture::Trusted,
            execution_context_id: "exec:work-item-sync-publish-review".to_string(),
            policy_block_ref: None,
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_provider_payload_refs_present: false,
        authored_at: "2026-05-18T09:25:00Z".to_string(),
        expires_at: "2026-05-18T10:25:00Z".to_string(),
        summary: summary.to_string(),
    }
}

fn origin(execution_context_id: &str) -> WorkItemOriginDisclosure {
    WorkItemOriginDisclosure {
        host_identity: "aureline:host:primary".to_string(),
        workspace_id: "workspace:primary".to_string(),
        actor_subject: "actor:human:opaque:primary".to_string(),
        execution_context_id: execution_context_id.to_string(),
        policy_epoch: "policy:epoch:2026-05-18".to_string(),
    }
}
