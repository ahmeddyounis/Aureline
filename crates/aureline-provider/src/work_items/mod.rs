//! Provider-backed work-item detail, transition review, and offline handoff contracts.
//!
//! This module is the typed provider-lane consumer for the work-item records
//! frozen under [`/schemas/work_items`](../../../../schemas/work_items). It
//! groups durable detail headers, previewed status-transition packets,
//! transition-review sheets, offline-handoff packets, and provider workflow
//! corpus rows into one beta page so provider, review, Git, workspace, and
//! shell surfaces read the same authority, freshness, local-draft, queued, and
//! offline-capture truth.

pub mod object_rows;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::registry::{ProviderActorClass, ProviderFamily, RedactionClass};

/// Schema version exported by the work-item transition beta page.
pub const WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by provider work-item transition records.
pub const WORK_ITEM_TRANSITION_BETA_SHARED_CONTRACT_REF: &str =
    "providers:work_item_transition_beta:v1";

/// Stable record kind for [`WorkItemTransitionBetaPage`].
pub const WORK_ITEM_TRANSITION_BETA_PAGE_RECORD_KIND: &str =
    "providers_work_item_transition_beta_page_record";

/// Stable record kind for [`WorkItemDetailRecord`].
pub const WORK_ITEM_DETAIL_RECORD_KIND: &str = "work_item_detail_record";

/// Stable record kind for [`StatusTransitionPacketRecord`].
pub const STATUS_TRANSITION_PACKET_RECORD_KIND: &str = "status_transition_packet_record";

/// Stable record kind for [`TransitionReviewSheetRecord`].
pub const TRANSITION_REVIEW_RECORD_KIND: &str = "transition_review_record";

/// Stable record kind for [`OfflineHandoffPacketRecord`].
pub const OFFLINE_HANDOFF_PACKET_RECORD_KIND: &str = "offline_handoff_packet_record";

/// Stable record kind for [`ProviderWorkflowCorpusCase`].
pub const PROVIDER_WORKFLOW_CORPUS_CASE_RECORD_KIND: &str =
    "provider_work_item_workflow_corpus_case_record";

/// Stable record kind for [`WorkItemTransitionBetaValidationReport`].
pub const WORK_ITEM_TRANSITION_BETA_VALIDATION_REPORT_RECORD_KIND: &str =
    "providers_work_item_transition_beta_validation_report";

/// Stable record kind for [`WorkItemTransitionBetaSupportExport`].
pub const WORK_ITEM_TRANSITION_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_work_item_transition_beta_support_export";

/// Work-item row posture rendered by provider-linked lists and details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemRowPostureClass {
    /// Provider truth is authoritative and freshly observed.
    ProviderAuthoritative,
    /// Provider truth is cached or stale and local work must remain draft-only.
    CachedStale,
    /// Provider state can be inspected but not mutated with the current grant.
    ReadOnly,
    /// Provider write is blocked by policy rather than connectivity.
    PolicyBlocked,
    /// Row is a local draft with no provider object yet.
    LocalDraft,
    /// Row is admitted to a publish-later queue.
    Queued,
    /// Row or action was captured offline and is not provider-accepted yet.
    OfflineCaptured,
}

impl WorkItemRowPostureClass {
    /// Stable token recorded on support and fixture records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritative => "provider_authoritative",
            Self::CachedStale => "cached_stale",
            Self::ReadOnly => "read_only",
            Self::PolicyBlocked => "policy_blocked",
            Self::LocalDraft => "local_draft",
            Self::Queued => "queued",
            Self::OfflineCaptured => "offline_captured",
        }
    }
}

/// Authority class for a durable work-item detail row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemAuthorityClass {
    /// Provider row is authoritative and synced locally.
    ProviderAuthoritativeSynced,
    /// Provider remains authoritative, but local work continues as draft while stale.
    ProviderAuthoritativeStaleLocalContinues,
    /// Local draft has not created a provider object.
    LocalDraftNoProviderObject,
    /// Local-authored row has entered publish-later.
    QueuedPublishLocalAuthored,
    /// Work item is inferred only from a review relation.
    LinkedReviewOnlyNoProviderOverlay,
    /// Work item was imported from a handoff packet with no live provider path.
    ImportedHandoffEvidenceOnly,
    /// Cached read-only shadow of provider state.
    CachedReadOnlyShadowInspectOnly,
}

impl WorkItemAuthorityClass {
    /// Stable token recorded on support and fixture records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthoritativeSynced => "provider_authoritative_synced",
            Self::ProviderAuthoritativeStaleLocalContinues => {
                "provider_authoritative_stale_local_continues"
            }
            Self::LocalDraftNoProviderObject => "local_draft_no_provider_object",
            Self::QueuedPublishLocalAuthored => "queued_publish_local_authored",
            Self::LinkedReviewOnlyNoProviderOverlay => "linked_review_only_no_provider_overlay",
            Self::ImportedHandoffEvidenceOnly => "imported_handoff_evidence_only",
            Self::CachedReadOnlyShadowInspectOnly => "cached_read_only_shadow_inspect_only",
        }
    }
}

/// Freshness posture rendered on work-item detail headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemFreshnessClass {
    /// Provider object is live and within its freshness floor.
    LiveAuthoritativeFresh,
    /// Provider object is still inside a grace window.
    WarmWithinGrace,
    /// Provider object is beyond grace but local draft work continues.
    DegradedBeyondGraceLocalContinues,
    /// Provider object cannot currently be verified.
    UnverifiableProviderUnreachable,
    /// Imported snapshot has no refresh path.
    ImportedSnapshotNoRefreshPath,
    /// Local draft has never been published.
    LocalDraftNeverPublished,
}

impl WorkItemFreshnessClass {
    /// Stable token recorded on support and fixture records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAuthoritativeFresh => "live_authoritative_fresh",
            Self::WarmWithinGrace => "warm_within_grace",
            Self::DegradedBeyondGraceLocalContinues => "degraded_beyond_grace_local_continues",
            Self::UnverifiableProviderUnreachable => "unverifiable_provider_unreachable",
            Self::ImportedSnapshotNoRefreshPath => "imported_snapshot_no_refresh_path",
            Self::LocalDraftNeverPublished => "local_draft_never_published",
        }
    }

    /// True when this freshness class is not current provider truth.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::LiveAuthoritativeFresh | Self::WarmWithinGrace)
    }
}

/// Source of a state value shown in a work-item detail row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateValueOriginClass {
    /// Token came directly from provider state.
    ProviderAuthoritativeStateToken,
    /// Token came from local draft authoring.
    LocalDraftPendingPublish,
    /// Token came from an imported handoff packet.
    ImportedHandoffStateTokenNoRefresh,
    /// Token was derived from a linked review state.
    DerivedFromLinkedReviewState,
}

/// State family named by a current or snapshotted state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateFamilyClass {
    /// Work item lifecycle token.
    LifecycleState,
    /// Assignee token or actor row.
    AssigneeState,
    /// Label-set token.
    LabelSetState,
    /// Milestone, sprint, or iteration token.
    MilestoneOrIterationState,
    /// Priority or severity token.
    PriorityOrSeverityState,
    /// Blocking relationship token.
    BlockingRelationshipState,
    /// Review or merge state token.
    ReviewOrMergeState,
    /// Validation-evidence state.
    ValidationEvidenceState,
    /// Publish-preview state.
    PublishPreviewState,
    /// Freshness state.
    FreshnessState,
    /// Trust or redaction state.
    TrustOrRedactionState,
}

/// Object class for a provider-side work item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemObjectClass {
    /// Generic issue or work item.
    IssueOrWorkItem,
    /// Epic or initiative.
    EpicOrInitiative,
    /// User story.
    UserStory,
    /// Task or subtask.
    TaskOrSubtask,
    /// Bug report.
    BugReport,
    /// Incident report.
    IncidentReport,
    /// Change request.
    ChangeRequest,
    /// Request for comment or proposal.
    RfcOrProposal,
    /// Repair-only object class.
    Other,
}

/// Write-authority posture for a work-item detail row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteAuthorityClass {
    /// Provider write can proceed after reviewed publish-now confirmation.
    WriteAdmissibleProviderWriteable,
    /// Provider write can proceed only through browser handoff.
    WriteAdmissibleThroughBrowserHandoffOnly,
    /// Provider write can proceed only through publish-later queueing.
    WriteAdmissibleQueuedPublishOnly,
    /// Local draft write is admitted with no provider path.
    WriteAdmissibleLocalDraftOnlyNoProviderPath,
    /// Provider grant is read-only.
    WriteBlockedProviderReadOnlyScope,
    /// Workspace trust blocks provider write.
    WriteBlockedWorkspaceTrustUnsetOrRestricted,
    /// Provider is unreachable.
    WriteBlockedProviderUnreachable,
    /// Freshness floor is not satisfied.
    WriteBlockedFreshnessFloorUnsatisfied,
    /// Account or project mapping is pending.
    WriteBlockedAccountMappingPending,
    /// Imported evidence has no provider path.
    WriteBlockedImportedEvidenceOnlyNoProviderPath,
    /// Managed admin surface blocks local mutation.
    WriteBlockedManagedAdminOnlySurface,
}

impl WriteAuthorityClass {
    /// True when this class admits a write path of some kind.
    pub const fn admits_write(self) -> bool {
        matches!(
            self,
            Self::WriteAdmissibleProviderWriteable
                | Self::WriteAdmissibleThroughBrowserHandoffOnly
                | Self::WriteAdmissibleQueuedPublishOnly
                | Self::WriteAdmissibleLocalDraftOnlyNoProviderPath
        )
    }

    /// True when provider mutation is blocked or unavailable.
    pub const fn blocks_provider_write(self) -> bool {
        !matches!(self, Self::WriteAdmissibleProviderWriteable)
    }
}

/// Linkage class for issue-to-branch or worktree relations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueToBranchLinkClass {
    /// No branch is linked.
    NotLinkedNoBranch,
    /// Local branch or worktree is linked without provider overlay.
    LinkedLocalBranchOrWorktreeNoProviderOverlay,
    /// Provider branch overlay has been fetched.
    LinkedProviderBranchOverlayFetched,
    /// Browser handoff carries only a branch token.
    LinkedBrowserHandoffBranchTokenOnly,
    /// Imported review bundle supplied the branch snapshot.
    LinkedImportedReviewBundleBranchSnapshot,
    /// Branch resolution is still pending.
    LinkedUnknownBranchResolutionPending,
}

/// Linkage class for review relations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkedReviewClass {
    /// No review is linked.
    NoLinkedReview,
    /// Review workspace is linked with provider overlay.
    LinkedReviewWorkspaceWithProviderOverlay,
    /// Review workspace is linked using local truth only.
    LinkedReviewWorkspaceLocalTruthOnly,
    /// Review pack is linked.
    LinkedReviewPack,
    /// Imported review bundle snapshot is linked.
    LinkedImportedReviewBundleSnapshot,
}

/// Linkage class for change-intent relations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeIntentClass {
    /// No change intent is linked.
    NoChangeIntent,
    /// Provider-authoritative change object is linked.
    ChangeObjectProviderAuthoritative,
    /// Local-draft change object is linked.
    ChangeObjectLocalDraft,
    /// Patch stack is linked.
    PatchStackLinked,
}

/// Linkage class for validation evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationEvidenceClass {
    /// No validation evidence is attached.
    NoValidationEvidenceAttached,
    /// Review evaluation result is linked.
    LinkedReviewEvaluationResult,
    /// Test or check evidence is linked.
    LinkedTestOrCheckEvidence,
    /// Incident or runbook evidence is linked.
    LinkedIncidentOrRunbookEvidence,
}

/// Linkage class for publish-preview records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishPreviewClass {
    /// No publish preview is required.
    NoPublishPreviewRequired,
    /// Provider consequence preview is pinned.
    PublishPreviewPinnedProviderConsequencePreviewRecord,
    /// Publish-later queue preview is pinned.
    PublishPreviewPinnedPublishLaterQueueRecord,
    /// Browser handoff preview is pinned.
    PublishPreviewPinnedBrowserHandoffRecord,
}

/// External-open affordance shown on a work-item detail header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenExternalActionClass {
    /// External open action is available.
    Available,
    /// External open action must route through a typed browser handoff packet.
    BrowserHandoffRequired,
    /// External open is blocked by policy.
    BlockedByPolicy,
    /// No external path exists for imported evidence.
    NotAvailableImportedEvidenceOnly,
}

/// Mutation mode used by work-item transition packets and review rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemMutationMode {
    /// Save locally only.
    LocalDraft,
    /// Publish immediately after review.
    PublishNow,
    /// Route through a system-browser handoff.
    OpenInProvider,
    /// Queue for publish-later.
    DeferredPublish,
    /// Preview or inspect only.
    InspectOnly,
}

impl WorkItemMutationMode {
    /// True when the mode does not publish to a provider now.
    pub const fn is_deferred_or_local(self) -> bool {
        !matches!(self, Self::PublishNow)
    }
}

/// Transition kind carried by a status-transition packet entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionKindClass {
    /// Add or update a provider/local comment.
    AddOrUpdateComment,
    /// Delete a comment.
    DeleteComment,
    /// Assign or unassign an owner.
    AssignOrUnassignOwner,
    /// Change lifecycle state token.
    ChangeLifecycleStateToken,
    /// Change priority or severity token.
    ChangePriorityOrSeverityToken,
    /// Add or remove labels.
    AddOrRemoveLabel,
    /// Link a branch or worktree.
    LinkBranchOrWorktree,
    /// Link a review or change object.
    LinkReviewOrChangeObject,
    /// Attach validation evidence.
    AttachValidationEvidence,
    /// Rename title or metadata label.
    RenameTitleLabel,
    /// Merge or close with a resolution token.
    MergeOrCloseWithResolutionToken,
}

/// Action class for one transition entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionActionClass {
    /// Apply path mutates provider state immediately after review.
    MutateProviderStatePublishNow,
    /// Apply path only saves a local draft.
    SaveLocalDraftOnlyNoProviderPath,
    /// Apply path queues a deferred publish.
    QueueForPublishLaterDeferred,
    /// Apply path opens provider through browser handoff.
    RouteThroughBrowserHandoffOpenInProvider,
    /// No mutation; inspect-only preview.
    InspectOnlyNoMutation,
    /// Transition was captured offline and awaits drain.
    CapturedOfflinePendingDrain,
}

impl TransitionActionClass {
    /// True when this entry claims provider mutation will occur immediately.
    pub const fn mutates_provider_now(self) -> bool {
        matches!(self, Self::MutateProviderStatePublishNow)
    }

    /// True when the entry stays local, queued, handoff, or inspect-only.
    pub const fn is_local_or_deferred(self) -> bool {
        !self.mutates_provider_now()
    }
}

/// Notification side effect projected for a transition entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationSideEffectClass {
    /// Local-only work emits no provider notification.
    NoNotificationLocallyOnly,
    /// Provider publish notifies assignees.
    NotifyAssigneesOnProviderPublish,
    /// Provider publish notifies subscribers.
    NotifySubscribersOnProviderPublish,
    /// Provider publish notifies linked review authors.
    NotifyLinkedReviewAuthorsOnProviderPublish,
    /// Provider publish mentions an explicit actor.
    MentionExplicitActorOnProviderPublish,
    /// Provider publish notifies managed admin.
    NotifyManagedAdminOnProviderPublish,
    /// Notification is blocked pending workspace trust.
    NotificationBlockedPendingWorkspaceTrust,
    /// Notification is unknown until publish is admitted.
    NotificationUnknownUntilPublishAdmittedPendingReview,
    /// Notification was explicitly suppressed by the user.
    NotificationSuppressedByUserChoiceExplicitOptOut,
}

/// Permission scope projected for a transition entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionScopeClass {
    /// Current user may write provider state.
    PermissionAdmissibleUserWritesProvider,
    /// Current user may update assignee only.
    PermissionAdmissibleAssigneeOnly,
    /// Install or app grant admits the write.
    PermissionAdmissibleUnderInstallOrAppGrant,
    /// Browser handoff is the only admitted write path.
    PermissionAdmissibleUnderBrowserHandoffOnly,
    /// Managed admin route is the only admitted write path.
    PermissionAdmissibleUnderManagedAdminOnly,
    /// Local draft is the only admitted path.
    PermissionAdmissibleUnderLocalDraftOnly,
    /// Provider grant is read-only.
    PermissionBlockedProviderReadOnlyScope,
    /// Actor class is forbidden.
    PermissionBlockedActorClassForbidden,
    /// Workspace trust blocks the write.
    PermissionBlockedWorkspaceTrustUnsetOrRestricted,
    /// Actor resolution is pending.
    PermissionUnknownPendingActorResolution,
}

/// Packet-level admissibility for a status transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionAdmissibilityClass {
    /// Transition can publish now.
    AdmissibleNowPublishNow,
    /// Transition can queue for publish-later.
    AdmissibleViaQueueForPublishLater,
    /// Transition can route through browser handoff only.
    AdmissibleViaBrowserHandoffOnly,
    /// Transition can save as a local draft only.
    AdmissibleLocalDraftOnly,
    /// Transition is inspect-only.
    AdmissibleInspectOnlyWhatIf,
    /// Transition is blocked pending prerequisites.
    BlockedPendingPrerequisites,
    /// Transition is blocked by provider read-only scope.
    BlockedProviderReadOnlyScope,
    /// Transition is blocked by actor class.
    BlockedActorClassForbidden,
    /// Transition is blocked by workspace trust.
    BlockedWorkspaceTrustUnsetOrRestricted,
    /// Transition is blocked by freshness floor.
    BlockedFreshnessFloorUnsatisfied,
    /// Transition is blocked by account mapping.
    BlockedAccountMappingPending,
    /// Transition has no provider path because it is imported evidence.
    BlockedImportedEvidenceOnlyNoProviderPath,
}

impl TransitionAdmissibilityClass {
    /// True when a confirm action can perform a local, queued, handoff, or provider write.
    pub const fn is_admissible(self) -> bool {
        matches!(
            self,
            Self::AdmissibleNowPublishNow
                | Self::AdmissibleViaQueueForPublishLater
                | Self::AdmissibleViaBrowserHandoffOnly
                | Self::AdmissibleLocalDraftOnly
                | Self::AdmissibleInspectOnlyWhatIf
        )
    }
}

/// Binding change projected by a transition entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkedArtifactChangeClass {
    /// No linked artifact changes.
    NoArtifactChange,
    /// Bind a review workspace record.
    BindReviewWorkspaceRecordIdRef,
    /// Bind a change object record.
    BindChangeObjectRecordIdRef,
    /// Bind a local branch locator.
    BindBranchLocalLocatorRef,
    /// Bind validation evidence.
    BindValidationEvidenceRecordIdRef,
    /// Bind a provider consequence preview.
    BindProviderConsequencePreviewRecordIdRef,
}

/// User-facing review trigger for a transition sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionTriggerClass {
    /// Status change trigger.
    StatusChangeTrigger,
    /// Assignee change trigger.
    AssigneeChangeTrigger,
    /// Provider comment trigger.
    ProviderCommentTrigger,
    /// Reopen or close trigger.
    ReopenCloseTrigger,
    /// Automation or notification fanout trigger.
    AutomationOrNotificationFanoutTrigger,
    /// Linked review or change-object trigger.
    LinkedReviewOrChangeObjectTrigger,
    /// Validation evidence attach/detach trigger.
    ValidationEvidenceAttachOrDetachTrigger,
    /// Multi-kind batch trigger.
    MultiKindBatchTrigger,
}

/// Authorization class for a transition-review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionReviewAuthorizationClass {
    /// Human actor authored an admissible transition.
    HumanActorSelfAuthoredAdmissible,
    /// Human actor authored an admissible linked-review transition.
    HumanActorWithReviewLinkAdmissible,
    /// Human actor is pending required approvals.
    HumanActorPendingRequiredApprovals,
    /// AI proposal is pending user confirmation.
    AiProposedPendingUserConfirmation,
    /// Managed admin authored an admissible transition.
    ManagedAdminAuthoredAdmissible,
    /// Policy admin authored an admissible transition.
    PolicyAdminAuthoredAdmissible,
    /// Imported evidence is not authoring.
    ImportedEvidenceOnlyNotAuthoring,
}

/// Disposition class for a transition-review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionReviewDispositionClass {
    /// Confirming will publish now.
    AdmissibleNowPublishNow,
    /// Confirming will save a local draft.
    AdmissibleLocalDraftOnly,
    /// Confirming will queue publish-later.
    AdmissibleViaQueueForPublishLater,
    /// Confirming will route through browser handoff.
    AdmissibleViaBrowserHandoffOnly,
    /// Review is inspect-only.
    AdmissibleInspectOnlyWhatIf,
    /// Review is blocked by policy.
    BlockedByPolicy,
    /// Review is blocked by read-only scope.
    BlockedProviderReadOnlyScope,
    /// Review is blocked pending prerequisites.
    BlockedPendingPrerequisites,
    /// Review is blocked by freshness.
    BlockedFreshnessFloorUnsatisfied,
    /// Review was withdrawn before apply.
    WithdrawnBeforeApply,
}

impl TransitionReviewDispositionClass {
    /// True when the disposition allows a confirm path.
    pub const fn is_admissible(self) -> bool {
        matches!(
            self,
            Self::AdmissibleNowPublishNow
                | Self::AdmissibleLocalDraftOnly
                | Self::AdmissibleViaQueueForPublishLater
                | Self::AdmissibleViaBrowserHandoffOnly
                | Self::AdmissibleInspectOnlyWhatIf
        )
    }
}

/// Side-effect fanout class disclosed by a transition review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectFanoutKindClass {
    /// Provider state mutation will fan out.
    ProviderMutationFanout,
    /// Local metadata change will fan out.
    LocalMetadataChangeFanout,
    /// Linked review or change object will update.
    LinkedReviewUpdateFanout,
    /// Notification will be emitted or intentionally deferred.
    NotificationEmissionFanout,
    /// Follow-on automation will be queued.
    QueuedFollowonAutomationFanout,
}

/// Account target class for one fanout row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetAccountClass {
    /// Connected provider account is resolved.
    ConnectedProviderAccountResolved,
    /// Install or app grant is the target.
    ConnectedProviderInstallOrAppGrant,
    /// Delegated user token is the target.
    ConnectedProviderDelegatedUserToken,
    /// Project-scoped grant is the target.
    ConnectedProviderProjectScopedGrant,
    /// Policy-injected service identity is the target.
    ConnectedProviderPolicyInjectedServiceIdentity,
    /// Account mapping is pending.
    AccountMappingBindingPendingUserResolution,
    /// Browser handoff account session is the target.
    BrowserHandoffAccountSessionOnly,
    /// Managed admin account is the target.
    ManagedAdminOnlyAccount,
    /// Local-only row has no provider account.
    LocalOnlyNoProviderAccount,
}

/// Authority source for one fanout row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySourceClass {
    /// Provider authoritative source.
    ProviderAuthoritativeSource,
    /// Local authoritative source with no provider overlay.
    LocalAuthoritativeSourceNoProviderOverlay,
    /// Provider overlay plus local overlay.
    ProviderOverlayWithLocalOverlaySynced,
    /// Cached read-only shadow source.
    CachedReadOnlyShadowSource,
    /// Imported evidence-only source.
    ImportedEvidenceOnlySource,
    /// Managed admin authority source.
    ManagedAdminAuthoritySource,
    /// Policy admin authority source.
    PolicyAdminAuthoritySource,
    /// Pending account-mapping source.
    PendingAccountMappingAuthoritySource,
    /// Local-only no provider authority.
    LocalOnlyNoProviderAuthority,
}

/// Undo or rollback posture for one fanout row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoRollbackPostureClass {
    /// No undo is required for local-only work.
    NoUndoRequiredLocalOnly,
    /// Undo is available within a bounded window.
    UndoAdmissibleWithinWindow,
    /// Rollback requires a compensating action.
    RollbackAdmissibleViaCompensatingAction,
    /// Rollback can revoke before queue drain.
    RollbackAdmissibleViaRevokeBeforeDrain,
    /// Rollback is blocked by irreversibility.
    RollbackBlockedIrreversible,
    /// Rollback is unknown pending provider callback.
    RollbackUnknownPendingProviderCallback,
    /// Imported evidence has no provider path to undo.
    RollbackBlockedImportedEvidenceOnlyNoProviderPath,
}

/// Offline or deferred handling class for one fanout row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineDeferredHandlingClass {
    /// Publish-now row is not offline or deferred.
    NotOfflineNotDeferred,
    /// Row is admitted to deferred publish.
    DeferredPublishAdmittedToQueue,
    /// Row was captured offline and is pending drain.
    DeferredPublishCapturedOfflinePendingDrain,
    /// Row is routed through browser handoff.
    DeferredPublishRoutedThroughBrowserHandoff,
    /// Row is blocked pending prerequisites.
    DeferredPublishBlockedPendingPrerequisites,
    /// Row is inspect-only.
    DeferredPublishInspectOnlyWhatIf,
}

/// Admission reason for an offline-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffAdmissionReasonClass {
    /// Provider was unreachable during capture.
    ProviderUnreachableOfflineCapture,
    /// Provider health was degraded or unavailable.
    ProviderHealthDegradedOrUnavailable,
    /// System browser was blocked on a managed workstation.
    BrowserHandoffBlockedManagedWorkstationNoSystemBrowser,
    /// User chose not to open a browser handoff now.
    BrowserHandoffBlockedUserChoice,
    /// Account mapping awaits user selection.
    AccountMappingPendingUserSelection,
    /// Freshness floor was unsatisfied.
    FreshnessFloorUnsatisfiedRemoteObjectDrifted,
    /// Policy epoch requires re-evaluation.
    PolicyEpochPendingReEvaluationRequired,
    /// Workspace trust is restricted and capture-only.
    WorkspaceTrustUnsetOrRestrictedCaptureOnly,
    /// Step-up authenticator is pending.
    StepUpAuthenticatorPending,
    /// User explicitly deferred into review.
    UserDeferredToReviewPacketExplicitChoice,
    /// Packet was imported from support export with no live provider path.
    ImportedFromSupportExportNoLiveProviderPath,
}

/// Provider acceptance state for an offline-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffProviderAcceptanceClass {
    /// Packet has not been submitted; it is local capture only.
    NotSubmittedLocalCaptureOnly,
    /// Submission occurred but acceptance is unverified.
    SubmittedPendingProviderAcceptUnverified,
    /// Provider acceptance was confirmed after publish-later drain.
    ProviderAcceptConfirmedPublishLaterDrained,
    /// Provider rejected with a typed reason.
    ProviderAcceptRejectedWithTypedReason,
    /// Imported handoff is evidence-only.
    ImportedHandoffEvidenceOnlyNoProviderPath,
}

/// Export route offered by an offline-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffExportRouteClass {
    /// Packet remains local and has no export path.
    LocalOnlyNoExportPath,
    /// Attach by reference to a support bundle.
    SupportBundleAttachmentByReference,
    /// Attach by reference to an incident workspace.
    IncidentWorkspaceAttachmentByReference,
    /// Attach by reference to an object handoff packet.
    ObjectHandoffPacketAttachmentByReference,
    /// Export through companion browser handoff.
    CompanionBrowserHandoffExport,
    /// User can export through clipboard or text.
    ClipboardOrTextExportUserInitiated,
    /// User can export through CLI.
    CliExportCommandUserInitiated,
    /// Export to managed admin handoff.
    ExternalHandoffExportToManagedAdminOnly,
}

/// Retry route watched by an offline-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffRetryRouteClass {
    /// Retry automatically when connectivity is restored.
    AutoRetryOnConnectivityRestored,
    /// Retry automatically when provider health recovers.
    AutoRetryOnProviderHealthRecovered,
    /// Retry automatically when browser availability returns.
    AutoRetryOnBrowserAvailable,
    /// Retry automatically after account reselection.
    AutoRetryOnAccountReselected,
    /// Retry automatically after freshness refresh.
    AutoRetryOnFreshnessRefresh,
    /// Retry automatically after policy epoch stabilizes.
    AutoRetryOnPolicyEpochStable,
    /// Retry automatically after workspace trust is resolved.
    AutoRetryOnWorkspaceTrustResolved,
    /// Retry automatically after step-up clears.
    AutoRetryOnStepUpAuthenticatorCleared,
    /// User must manually confirm retry.
    ManualRetryUserMustConfirmOnly,
    /// Imported evidence has no retry path.
    NoRetryImportedEvidenceOnly,
}

/// Drain lifecycle state for an offline-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDrainStateClass {
    /// Captured and waiting for drain.
    CapturedPendingDrain,
    /// Captured and waiting for user export.
    CapturedPendingExportUserInitiated,
    /// Exported and waiting for external apply.
    ExportedPendingExternalApply,
    /// Drain admitted to publish-later.
    DrainAdmittedToPublishLaterQueue,
    /// Publish-later drain completed.
    DrainedPublishLaterCompleted,
    /// Publish-later drain rejected with typed reason.
    DrainedPublishLaterRejectedWithTypedReason,
    /// User revoked before drain.
    RevokedByUserBeforeDrain,
    /// Superseded by another handoff packet.
    SupersededByHandoffPacket,
}

/// Provider workflow corpus scenario class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderWorkflowCorpusClass {
    /// Stale target identity must be reviewed.
    StaleTargetId,
    /// Revoked credentials block mutation.
    RevokedCredentials,
    /// Conflicting provider update requires review.
    ConflictingUpdates,
    /// Read-only session preserves local drafts.
    ReadOnlySession,
    /// Offline capture preserves a packet.
    OfflineCapture,
    /// Publish-later replay preserves identity and ordering.
    PublishLaterReplay,
}

/// Publish posture included in support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemPublishPostureClass {
    /// Provider state is observed and authoritative.
    PublishedObservedAuthoritative,
    /// Publish-now is pending review or callback.
    PublishNowPendingReview,
    /// Work remains local draft only.
    DraftOnly,
    /// Work is queued for publish-later.
    QueuedForPublishLater,
    /// Work was captured offline and is not provider-accepted.
    OfflineCapturedNotSubmitted,
    /// Imported evidence only.
    ImportedEvidenceOnly,
    /// Read-only inspect path.
    InspectOnly,
}

/// Fixture metadata used by checked-in provider workflow fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionFixtureMetadata {
    /// Fixture name.
    pub name: String,
    /// Redaction-safe scenario summary.
    pub scenario: String,
}

/// Upstream contracts consumed by a work-item transition beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionContractRefs {
    /// Provider-facing work-item detail schema reference.
    pub provider_work_item_detail_schema_ref: String,
    /// Provider-facing offline handoff packet schema reference.
    pub provider_offline_handoff_packet_schema_ref: String,
    /// Canonical status-transition packet schema reference.
    pub status_transition_packet_schema_ref: String,
    /// Canonical transition-review schema reference.
    pub transition_review_schema_ref: String,
    /// Publish-later queue schema reference.
    pub publish_later_queue_schema_ref: String,
    /// Browser-handoff packet schema reference.
    pub browser_handoff_packet_schema_ref: String,
    /// Connected-provider registry schema reference.
    pub connected_provider_registry_schema_ref: String,
}

impl WorkItemTransitionContractRefs {
    fn all_refs(&self) -> [&str; 7] {
        [
            &self.provider_work_item_detail_schema_ref,
            &self.provider_offline_handoff_packet_schema_ref,
            &self.status_transition_packet_schema_ref,
            &self.transition_review_schema_ref,
            &self.publish_later_queue_schema_ref,
            &self.browser_handoff_packet_schema_ref,
            &self.connected_provider_registry_schema_ref,
        ]
    }
}

/// Provider-side identity for one work item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemObjectIdentity {
    /// Provider object class.
    pub object_class: WorkItemObjectClass,
    /// Opaque provider-side id.
    pub provider_side_id: String,
    /// Opaque provider host ref.
    pub provider_host: String,
    /// Opaque tenant, org, project, or space scope ref.
    pub tenant_or_org_scope: String,
    /// Reviewable canonical label safe for UI and support export.
    pub provider_canonical_label: String,
}

/// One state row rendered in the work-item detail header or body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemCurrentStateRow {
    /// State family.
    pub state_family_class: StateFamilyClass,
    /// Provider, local-draft, or imported state token.
    pub state_value: String,
    /// Source of the state token.
    pub state_value_origin_class: StateValueOriginClass,
    /// Optional upstream schema ref.
    #[serde(default)]
    pub source_schema_ref: String,
    /// Optional upstream source field.
    #[serde(default)]
    pub source_field: String,
    /// Optional upstream source record ref.
    #[serde(default)]
    pub source_ref: String,
    /// Redaction-safe state summary.
    pub summary: String,
}

/// Owner, assignee, reporter, reviewer, or watcher row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerOrAssigneeRow {
    /// Role class for the actor.
    pub owner_role_class: OwnerRoleClass,
    /// Opaque actor subject ref.
    pub actor_subject_ref: String,
    /// Provider actor class.
    pub actor_class: ProviderActorClass,
    /// Reviewable actor label.
    pub actor_label: String,
}

/// Owner role on a work item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerRoleClass {
    /// Primary owner.
    PrimaryOwner,
    /// Secondary owner.
    SecondaryOwner,
    /// Assignee.
    Assignee,
    /// Reviewer.
    Reviewer,
    /// Reporter.
    Reporter,
    /// Watcher or subscriber.
    WatcherOrSubscriber,
    /// Managed admin owner.
    ManagedAdminOwner,
}

/// Engineering artifacts related to a work-item detail row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineeringArtifactRelations {
    /// Issue-to-branch linkage class.
    pub issue_to_branch_link_class: IssueToBranchLinkClass,
    /// Local branch or worktree locator ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_branch_local_locator_ref: Option<String>,
    /// Linked review class.
    pub linked_review_class: LinkedReviewClass,
    /// Review workspace ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_review_workspace_record_id_ref: Option<String>,
    /// Change-intent class.
    pub change_intent_class: ChangeIntentClass,
    /// Change object ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_change_object_record_id_ref: Option<String>,
    /// Validation evidence class.
    pub validation_evidence_class: ValidationEvidenceClass,
    /// Validation evidence refs.
    #[serde(default)]
    pub linked_validation_evidence_record_id_refs: Vec<String>,
    /// Linked run or pipeline refs surfaced on relation strips.
    #[serde(default)]
    pub linked_run_record_id_refs: Vec<String>,
    /// Linked incident workspace refs surfaced on relation strips.
    #[serde(default)]
    pub linked_incident_workspace_record_id_refs: Vec<String>,
    /// Publish-preview class.
    pub publish_preview_class: PublishPreviewClass,
    /// Provider consequence preview ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_provider_consequence_preview_record_id_ref: Option<String>,
    /// Publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Offline handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_record_id_ref: Option<String>,
}

impl EngineeringArtifactRelations {
    fn relation_axes_present(&self) -> bool {
        true
    }
}

/// Work-item header action for opening the provider object externally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenExternalAction {
    /// Open-external action class.
    pub action_class: OpenExternalActionClass,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Reviewable action label.
    pub action_label: String,
    /// Guardrail: raw provider URL does not cross this boundary.
    pub raw_url_present: bool,
}

/// Origin disclosure shared by work-item records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemOriginDisclosure {
    /// Opaque host identity ref.
    pub host_identity: String,
    /// Opaque workspace id.
    pub workspace_id: String,
    /// Opaque actor subject ref.
    pub actor_subject: String,
    /// Execution context id.
    pub execution_context_id: String,
    /// Policy epoch ref.
    pub policy_epoch: String,
}

/// Policy context shared by work-item records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemPolicyContext {
    /// Policy epoch ref.
    pub policy_epoch: String,
    /// Trust state token.
    pub trust_state: TrustPosture,
    /// Execution context id.
    pub execution_context_id: String,
    /// Optional policy block ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_block_ref: Option<String>,
}

/// Workspace trust posture on a work-item record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPosture {
    /// Workspace is trusted.
    Trusted,
    /// Workspace is restricted.
    Restricted,
}

/// Durable detail header for one provider-owned or locally drafted work item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemDetailRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    #[serde(alias = "work_item_detail_schema_version")]
    pub schema_version: u32,
    /// Opaque work-item detail id.
    #[serde(alias = "work_item_detail_id")]
    pub detail_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Connected-provider record ref.
    pub connected_provider_record_id_ref: String,
    /// Provider family for the work item.
    pub provider_family: ProviderFamily,
    /// Reviewable provider label.
    pub provider_label: String,
    /// Opaque project, board, or space ref.
    pub project_or_space_ref: String,
    /// Canonical issue/work-item id visible to the user.
    pub canonical_id: String,
    /// Reviewable title label.
    pub title_label: String,
    /// Provider-side object identity.
    pub target_object_identity: WorkItemObjectIdentity,
    /// Row authority class.
    pub work_item_authority_class: WorkItemAuthorityClass,
    /// User-visible row posture class.
    pub row_posture_class: WorkItemRowPostureClass,
    /// Current state rows.
    pub current_state_rows: Vec<WorkItemCurrentStateRow>,
    /// Owner and assignee rows.
    #[serde(default)]
    pub owner_or_assignee_rows: Vec<OwnerOrAssigneeRow>,
    /// Freshness class.
    pub freshness_class: WorkItemFreshnessClass,
    /// Timestamp at which freshness was observed.
    pub freshness_observed_at: String,
    /// Freshness-floor ref.
    pub freshness_floor_ref: String,
    /// Write authority class.
    pub write_authority_class: WriteAuthorityClass,
    /// Related branch, review, change, evidence, and preview refs.
    pub engineering_artifact_relations: EngineeringArtifactRelations,
    /// Open-external action.
    pub open_external_action: OpenExternalAction,
    /// Acting identity class used by provider mutations.
    pub acting_as: ProviderActorClass,
    /// Optional local-draft ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_draft_ref: Option<String>,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Linked status-transition packet refs.
    #[serde(default)]
    pub linked_status_transition_packet_record_id_refs: Vec<String>,
    /// Linked offline-handoff packet refs.
    #[serde(default)]
    pub linked_offline_handoff_packet_record_id_refs: Vec<String>,
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
    /// Export-safe row summary.
    #[serde(alias = "summary")]
    pub support_export_summary: String,
    /// Timestamp at which the row was captured.
    pub captured_at: String,
}

impl WorkItemDetailRecord {
    /// Returns the publish posture support exports should show for this row.
    pub fn publish_posture(&self) -> WorkItemPublishPostureClass {
        match self.row_posture_class {
            WorkItemRowPostureClass::ProviderAuthoritative => {
                WorkItemPublishPostureClass::PublishedObservedAuthoritative
            }
            WorkItemRowPostureClass::LocalDraft => WorkItemPublishPostureClass::DraftOnly,
            WorkItemRowPostureClass::Queued => WorkItemPublishPostureClass::QueuedForPublishLater,
            WorkItemRowPostureClass::OfflineCaptured => {
                WorkItemPublishPostureClass::OfflineCapturedNotSubmitted
            }
            WorkItemRowPostureClass::ReadOnly => WorkItemPublishPostureClass::InspectOnly,
            WorkItemRowPostureClass::PolicyBlocked | WorkItemRowPostureClass::CachedStale => {
                if self.publish_later_queue_item_ref.is_some() {
                    WorkItemPublishPostureClass::QueuedForPublishLater
                } else if self.local_draft_ref.is_some() {
                    WorkItemPublishPostureClass::DraftOnly
                } else {
                    WorkItemPublishPostureClass::InspectOnly
                }
            }
        }
    }
}

/// One transition entry in a status-transition packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionEntry {
    /// Opaque transition entry id.
    pub transition_entry_id: String,
    /// Transition kind.
    pub transition_kind_class: TransitionKindClass,
    /// Transition action.
    pub transition_action_class: TransitionActionClass,
    /// Notification side effect.
    pub notification_side_effect_class: NotificationSideEffectClass,
    /// Permission scope.
    pub permission_scope_class: PermissionScopeClass,
    /// Entry admissibility.
    pub transition_admissibility_class: TransitionAdmissibilityClass,
    /// Expected provider actor class.
    pub expected_actor_class: ProviderActorClass,
    /// Previous state value.
    #[serde(default)]
    pub previous_state_value: String,
    /// Next state value.
    #[serde(default)]
    pub next_state_value: String,
    /// Linked artifact change.
    pub linked_artifact_change_class: LinkedArtifactChangeClass,
    /// Optional linked artifact ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_artifact_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional provider consequence preview ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_provider_consequence_preview_record_id_ref: Option<String>,
    /// Optional account-mapping binding ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_account_mapping_binding_record_id_ref: Option<String>,
    /// Optional offline-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_record_id_ref: Option<String>,
    /// Redaction-safe transition summary.
    pub summary: String,
}

/// Action affordances rendered by a transition sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionActionAffordances {
    /// Confirm action is visible and enabled.
    pub confirm_action_available: bool,
    /// Export action is visible.
    pub export_action_available: bool,
    /// Cancel action is visible.
    pub cancel_action_available: bool,
}

/// Previewed but not yet applied status-transition packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusTransitionPacketRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    #[serde(alias = "status_transition_packet_schema_version")]
    pub schema_version: u32,
    /// Opaque status-transition packet id.
    #[serde(alias = "status_transition_packet_id")]
    pub packet_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Mutation mode.
    pub mutation_mode: WorkItemMutationMode,
    /// Transition entries.
    pub transition_entries: Vec<TransitionEntry>,
    /// Packet admissibility.
    pub packet_admissibility_class: TransitionAdmissibilityClass,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Optional provider consequence preview ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_provider_consequence_preview_record_id_ref: Option<String>,
    /// Optional account-mapping binding ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_account_mapping_binding_record_id_ref: Option<String>,
    /// Optional offline-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_record_id_ref: Option<String>,
    /// User-visible confirm/export/cancel actions.
    pub action_affordances: TransitionActionAffordances,
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
    /// Export-safe transition summary.
    pub summary: String,
}

/// One side-effect fanout row disclosed before a transition is applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectFanoutRow {
    /// Opaque fanout row id.
    pub fanout_row_id: String,
    /// Fanout kind.
    pub fanout_kind_class: SideEffectFanoutKindClass,
    /// Target account class.
    pub target_account_class: TargetAccountClass,
    /// Authority source class.
    pub authority_source_class: AuthoritySourceClass,
    /// Publish mode.
    pub publish_mode_class: WorkItemMutationMode,
    /// Notification side effect.
    pub notification_side_effect_class: NotificationSideEffectClass,
    /// Undo or rollback posture.
    pub undo_rollback_posture_class: UndoRollbackPostureClass,
    /// Offline or deferred handling class.
    pub offline_deferred_handling_class: OfflineDeferredHandlingClass,
    /// Optional linked review, change, queue, preview, or traceability ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_record_id_ref: Option<String>,
    /// Redaction-safe fanout summary.
    pub summary: String,
}

/// Transition-review sheet rendered before committing a risky transition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionReviewSheetRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    #[serde(alias = "transition_review_schema_version")]
    pub schema_version: u32,
    /// Opaque transition-review id.
    #[serde(alias = "transition_review_id")]
    pub review_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Bound status-transition packet ref.
    pub linked_status_transition_packet_record_id_ref: String,
    /// Trigger class.
    pub transition_trigger_class: TransitionTriggerClass,
    /// Authorization class.
    pub transition_review_authorization_class: TransitionReviewAuthorizationClass,
    /// Disposition class.
    pub transition_review_disposition_class: TransitionReviewDispositionClass,
    /// Target item ref shown in the sheet.
    pub target_item_ref: String,
    /// Provider label shown in the sheet.
    pub provider_label: String,
    /// Current state summary.
    pub current_state_summary: String,
    /// Requested state or changeset summary.
    pub requested_state_summary: String,
    /// Actor authority summary.
    pub actor_authority_summary: String,
    /// Publish mode summary.
    pub publish_mode_class: WorkItemMutationMode,
    /// Permission scope summary.
    pub permission_scope_class: PermissionScopeClass,
    /// Fanout rows.
    pub side_effect_fanout_rows: Vec<SideEffectFanoutRow>,
    /// User-visible confirm/export/cancel actions.
    pub action_affordances: TransitionActionAffordances,
    /// Whether local draft is preserved on publish failure.
    pub local_draft_preserved_on_failure: bool,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Optional offline-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_offline_handoff_packet_record_id_ref: Option<String>,
    /// Optional block reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_summary: Option<String>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Guardrail: no raw provider payload refs cross this boundary.
    pub raw_provider_payload_refs_present: bool,
    /// Authored timestamp.
    pub authored_at: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Export-safe review summary.
    pub summary: String,
}

/// Snapshotted state row in an offline handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotStateRow {
    /// State family class.
    pub state_family_class: StateFamilyClass,
    /// Snapshotted state value.
    pub state_value: String,
    /// Optional upstream source schema.
    #[serde(default)]
    pub source_schema_ref: String,
    /// Optional source field.
    #[serde(default)]
    pub source_field: String,
    /// Optional source ref.
    #[serde(default)]
    pub source_ref: String,
    /// Redaction-safe summary.
    pub summary: String,
}

/// Snapshotted owner or assignee row in an offline handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotOwnerRow {
    /// Owner role.
    pub owner_role_class: OwnerRoleClass,
    /// Opaque actor subject ref.
    pub actor_subject_ref: String,
    /// Provider actor class.
    pub actor_class: ProviderActorClass,
    /// Reviewable actor label.
    pub actor_label: String,
}

/// Relation axis snapshotted by an offline handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotRelationAxisClass {
    /// Issue-to-branch relation.
    IssueToBranchLink,
    /// Linked-review relation.
    LinkedReview,
    /// Change-intent relation.
    ChangeIntent,
    /// Validation-evidence relation.
    ValidationEvidence,
    /// Publish-preview relation.
    PublishPreview,
}

/// Snapshotted engineering relation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotEngineeringRelation {
    /// Relation axis.
    pub relation_axis_class: SnapshotRelationAxisClass,
    /// Class value from the detail relation.
    pub relation_class_value: String,
    /// Optional linked record ref.
    #[serde(default)]
    pub linked_record_id_ref: String,
    /// Redaction-safe summary.
    pub summary: String,
}

/// Offline handoff packet preserving work-item update intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffPacketRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    #[serde(alias = "offline_handoff_packet_schema_version")]
    pub schema_version: u32,
    /// Opaque offline-handoff packet id.
    #[serde(alias = "offline_handoff_packet_id")]
    pub packet_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Bound status-transition packet ref.
    pub linked_status_transition_packet_record_id_ref: String,
    /// Admission reason.
    pub handoff_admission_reason_class: HandoffAdmissionReasonClass,
    /// Provider acceptance class.
    pub handoff_provider_acceptance_class: HandoffProviderAcceptanceClass,
    /// Export routes.
    pub handoff_export_route_classes: Vec<HandoffExportRouteClass>,
    /// Retry routes.
    pub handoff_retry_route_classes: Vec<HandoffRetryRouteClass>,
    /// Drain state.
    pub handoff_drain_state_class: HandoffDrainStateClass,
    /// Snapshotted state rows.
    pub snapshot_state_rows: Vec<SnapshotStateRow>,
    /// Snapshotted owner rows.
    pub snapshot_owner_or_assignee_rows: Vec<SnapshotOwnerRow>,
    /// Snapshotted engineering relations.
    pub snapshot_engineering_relations: Vec<SnapshotEngineeringRelation>,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_publish_later_queue_item_record_id_ref: Option<String>,
    /// Optional provider consequence preview ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_provider_consequence_preview_record_id_ref: Option<String>,
    /// Optional account-mapping binding ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_account_mapping_binding_record_id_ref: Option<String>,
    /// Optional browser-handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_browser_handoff_packet_ref: Option<String>,
    /// Provider callback envelope refs.
    #[serde(default)]
    pub linked_provider_callback_envelope_record_id_refs: Vec<String>,
    /// Optional support bundle ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_support_bundle_record_id_ref: Option<String>,
    /// Optional incident workspace packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_incident_workspace_packet_record_id_ref: Option<String>,
    /// Optional object handoff packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_object_handoff_packet_record_id_ref: Option<String>,
    /// Optional superseded packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supersedes_offline_handoff_packet_record_id_ref: Option<String>,
    /// Optional successor packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by_offline_handoff_packet_record_id_ref: Option<String>,
    /// Optional typed rejection reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acceptance_rejection_reason: Option<String>,
    /// Captured freshness floor ref.
    pub captured_freshness_floor_ref: String,
    /// Redaction manifest ref.
    pub redaction_manifest_ref: String,
    /// Publish target ref.
    pub publish_target_ref: String,
    /// Retry action ref.
    pub retry_action_ref: String,
    /// Export action ref.
    pub export_action_ref: String,
    /// Packet has durable persistence across restart.
    pub packet_survives_restart: bool,
    /// Origin disclosure.
    pub origin_disclosure: WorkItemOriginDisclosure,
    /// Policy context.
    pub policy_context: WorkItemPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Guardrail: no raw provider payload refs cross this boundary.
    pub raw_provider_payload_refs_present: bool,
    /// Captured timestamp.
    pub captured_at: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Export-safe packet summary.
    pub summary: String,
}

/// Corpus row for provider workflow failure and replay drills.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderWorkflowCorpusCase {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Opaque corpus case id.
    pub case_id: String,
    /// Corpus class.
    pub corpus_class: ProviderWorkflowCorpusClass,
    /// Detail row this corpus case exercises.
    pub work_item_detail_record_id_ref: String,
    /// Optional transition packet exercised by the case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_transition_packet_record_id_ref: Option<String>,
    /// Optional offline packet exercised by the case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_handoff_packet_record_id_ref: Option<String>,
    /// Expected visible posture.
    pub expected_visible_posture: WorkItemRowPostureClass,
    /// Export-safe summary.
    pub support_export_summary: String,
}

/// Work-item transition beta page consumed by provider-linked surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionBetaPage {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<WorkItemTransitionFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contract references.
    pub contract_refs: WorkItemTransitionContractRefs,
    /// Durable work-item detail records.
    pub detail_records: Vec<WorkItemDetailRecord>,
    /// Previewed status-transition packets.
    pub transition_packets: Vec<StatusTransitionPacketRecord>,
    /// User-facing transition-review sheets.
    pub transition_reviews: Vec<TransitionReviewSheetRecord>,
    /// Offline handoff packets.
    pub offline_handoff_packets: Vec<OfflineHandoffPacketRecord>,
    /// Provider workflow corpora exercised by the page.
    pub workflow_corpus_cases: Vec<ProviderWorkflowCorpusCase>,
    /// Export-safe page summary.
    pub support_export_summary: String,
}

impl WorkItemTransitionBetaPage {
    /// Validates the page and returns a structured report.
    pub fn validate(&self) -> WorkItemTransitionBetaValidationReport {
        let mut validator = WorkItemTransitionBetaValidator::new(self);
        validator.run();
        validator.finish()
    }

    /// Builds a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> WorkItemTransitionBetaSupportExport {
        WorkItemTransitionBetaSupportExport::from_page(
            format!("{}:support_export", self.page_id),
            self.detail_records
                .first()
                .map(|record| record.captured_at.clone())
                .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string()),
            self,
        )
    }
}

/// Validation report emitted by the beta validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionBetaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Whether no defects were emitted.
    pub passed: bool,
    /// Coverage observed while validating.
    pub coverage: WorkItemTransitionBetaCoverage,
    /// Defects emitted by failed checks.
    pub defects: Vec<WorkItemTransitionBetaDefect>,
}

/// Coverage observed during work-item transition validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorkItemTransitionBetaCoverage {
    /// Row postures covered by detail records.
    pub row_postures: BTreeSet<WorkItemRowPostureClass>,
    /// Authority classes covered by detail records.
    pub authority_classes: BTreeSet<WorkItemAuthorityClass>,
    /// Freshness classes covered by detail records.
    pub freshness_classes: BTreeSet<WorkItemFreshnessClass>,
    /// Write-authority classes covered by detail records.
    pub write_authority_classes: BTreeSet<WriteAuthorityClass>,
    /// Mutation modes covered by transition packets.
    pub mutation_modes: BTreeSet<WorkItemMutationMode>,
    /// Transition action classes covered by transition entries.
    pub transition_action_classes: BTreeSet<TransitionActionClass>,
    /// Transition admissibility classes covered by packets.
    pub transition_admissibility_classes: BTreeSet<TransitionAdmissibilityClass>,
    /// Side-effect fanout kinds covered by review sheets.
    pub side_effect_fanout_kinds: BTreeSet<SideEffectFanoutKindClass>,
    /// Handoff admission reasons covered by offline packets.
    pub handoff_admission_reasons: BTreeSet<HandoffAdmissionReasonClass>,
    /// Handoff provider acceptance classes covered by offline packets.
    pub handoff_provider_acceptance_classes: BTreeSet<HandoffProviderAcceptanceClass>,
    /// Handoff drain states covered by offline packets.
    pub handoff_drain_states: BTreeSet<HandoffDrainStateClass>,
    /// Export routes covered by offline packets.
    pub handoff_export_routes: BTreeSet<HandoffExportRouteClass>,
    /// Retry routes covered by offline packets.
    pub handoff_retry_routes: BTreeSet<HandoffRetryRouteClass>,
    /// Provider workflow corpus cases covered by the page.
    pub workflow_corpus_classes: BTreeSet<ProviderWorkflowCorpusClass>,
}

/// One validation defect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionBetaDefect {
    /// Defect kind.
    pub defect_kind: WorkItemTransitionBetaDefectKind,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
    /// Optional offending record ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_ref: Option<String>,
}

/// Defect kind emitted by work-item transition validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemTransitionBetaDefectKind {
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
    /// Detail row authority, freshness, or posture is incoherent.
    DetailTruthIncoherent,
    /// Raw provider material crossed a boundary.
    RawProviderMaterialPresent,
    /// Transition packet claims an unsafe or false mutation path.
    TransitionTruthIncoherent,
    /// Transition review sheet omits required consequence disclosure.
    TransitionReviewIncomplete,
    /// Offline packet claims provider acceptance or replay state incorrectly.
    OfflineHandoffTruthIncoherent,
    /// Support/export posture is not redaction safe.
    SupportExportUnsafe,
}

/// Redaction-safe support export for work-item transition beta records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionBetaSupportExport {
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
    /// Detail summaries.
    pub detail_summaries: Vec<WorkItemDetailSupportSummary>,
    /// Transition summaries.
    pub transition_summaries: Vec<WorkItemTransitionSupportSummary>,
    /// Offline packet summaries.
    pub offline_handoff_summaries: Vec<OfflineHandoffSupportSummary>,
    /// Corpus summaries.
    pub workflow_corpus_summaries: Vec<ProviderWorkflowCorpusSupportSummary>,
    /// Redaction class for the export.
    pub redaction_class: RedactionClass,
    /// Guardrail: export contains no raw provider material.
    pub raw_provider_material_excluded: bool,
}

impl WorkItemTransitionBetaSupportExport {
    /// Builds an export-safe projection from a page.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: &WorkItemTransitionBetaPage,
    ) -> Self {
        Self {
            record_kind: WORK_ITEM_TRANSITION_BETA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
            export_id: export_id.into(),
            page_id: page.page_id.clone(),
            generated_at: generated_at.into(),
            detail_summaries: page
                .detail_records
                .iter()
                .map(WorkItemDetailSupportSummary::from)
                .collect(),
            transition_summaries: page
                .transition_packets
                .iter()
                .map(WorkItemTransitionSupportSummary::from)
                .collect(),
            offline_handoff_summaries: page
                .offline_handoff_packets
                .iter()
                .map(OfflineHandoffSupportSummary::from)
                .collect(),
            workflow_corpus_summaries: page
                .workflow_corpus_cases
                .iter()
                .map(ProviderWorkflowCorpusSupportSummary::from)
                .collect(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_provider_material_excluded: true,
        }
    }
}

/// Support-export summary for one detail row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemDetailSupportSummary {
    /// Detail record id.
    pub detail_id: String,
    /// Provider descriptor ref.
    pub provider_descriptor_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Provider-side work-item object class.
    pub object_class: WorkItemObjectClass,
    /// Project or space ref.
    pub project_or_space_ref: String,
    /// Canonical work-item id.
    pub canonical_id: String,
    /// Row posture class.
    pub row_posture_class: WorkItemRowPostureClass,
    /// Authority class.
    pub authority_class: WorkItemAuthorityClass,
    /// Freshness class.
    pub freshness_class: WorkItemFreshnessClass,
    /// Write authority class.
    pub write_authority_class: WriteAuthorityClass,
    /// Actor class Aureline was acting as.
    pub acting_as: ProviderActorClass,
    /// Export-safe sync scope class for the row.
    pub sync_scope_class: object_rows::WorkItemSyncScopeClass,
    /// Export-safe relation-link state for the row.
    pub link_state_class: object_rows::WorkItemLinkStateClass,
    /// Export-safe relation identity refs.
    pub relation_identity_refs: Vec<String>,
    /// Draft/queued/published posture.
    pub publish_posture: WorkItemPublishPostureClass,
    /// Export-safe summary.
    pub summary: String,
}

impl From<&WorkItemDetailRecord> for WorkItemDetailSupportSummary {
    fn from(record: &WorkItemDetailRecord) -> Self {
        Self {
            detail_id: record.detail_id.clone(),
            provider_descriptor_ref: record.provider_descriptor_ref.clone(),
            provider_family: record.provider_family,
            object_class: record.target_object_identity.object_class,
            project_or_space_ref: record.project_or_space_ref.clone(),
            canonical_id: record.canonical_id.clone(),
            row_posture_class: record.row_posture_class,
            authority_class: record.work_item_authority_class,
            freshness_class: record.freshness_class,
            write_authority_class: record.write_authority_class,
            acting_as: record.acting_as,
            sync_scope_class: object_rows::WorkItemSyncScopeClass::from_detail(record),
            link_state_class: object_rows::WorkItemLinkStateClass::from_relations(
                &record.engineering_artifact_relations,
            ),
            relation_identity_refs: object_rows::relation_identity_refs(
                &record.engineering_artifact_relations,
            ),
            publish_posture: record.publish_posture(),
            summary: record.support_export_summary.clone(),
        }
    }
}

/// Support-export summary for one transition packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemTransitionSupportSummary {
    /// Packet id.
    pub packet_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Mutation mode.
    pub mutation_mode: WorkItemMutationMode,
    /// Packet admissibility.
    pub packet_admissibility_class: TransitionAdmissibilityClass,
    /// Transition actions covered by the packet.
    pub transition_action_classes: BTreeSet<TransitionActionClass>,
    /// Export-safe summary.
    pub summary: String,
}

impl From<&StatusTransitionPacketRecord> for WorkItemTransitionSupportSummary {
    fn from(record: &StatusTransitionPacketRecord) -> Self {
        Self {
            packet_id: record.packet_id.clone(),
            work_item_detail_record_id_ref: record.work_item_detail_record_id_ref.clone(),
            mutation_mode: record.mutation_mode,
            packet_admissibility_class: record.packet_admissibility_class,
            transition_action_classes: record
                .transition_entries
                .iter()
                .map(|entry| entry.transition_action_class)
                .collect(),
            summary: record.summary.clone(),
        }
    }
}

/// Support-export summary for one offline handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffSupportSummary {
    /// Packet id.
    pub packet_id: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Admission reason.
    pub handoff_admission_reason_class: HandoffAdmissionReasonClass,
    /// Provider acceptance class.
    pub handoff_provider_acceptance_class: HandoffProviderAcceptanceClass,
    /// Drain state.
    pub handoff_drain_state_class: HandoffDrainStateClass,
    /// Export routes.
    pub handoff_export_route_classes: BTreeSet<HandoffExportRouteClass>,
    /// Retry routes.
    pub handoff_retry_route_classes: BTreeSet<HandoffRetryRouteClass>,
    /// Export-safe summary.
    pub summary: String,
}

impl From<&OfflineHandoffPacketRecord> for OfflineHandoffSupportSummary {
    fn from(record: &OfflineHandoffPacketRecord) -> Self {
        Self {
            packet_id: record.packet_id.clone(),
            work_item_detail_record_id_ref: record.work_item_detail_record_id_ref.clone(),
            handoff_admission_reason_class: record.handoff_admission_reason_class,
            handoff_provider_acceptance_class: record.handoff_provider_acceptance_class,
            handoff_drain_state_class: record.handoff_drain_state_class,
            handoff_export_route_classes: record
                .handoff_export_route_classes
                .iter()
                .copied()
                .collect(),
            handoff_retry_route_classes: record
                .handoff_retry_route_classes
                .iter()
                .copied()
                .collect(),
            summary: record.summary.clone(),
        }
    }
}

/// Support-export summary for one provider workflow corpus case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderWorkflowCorpusSupportSummary {
    /// Corpus case id.
    pub case_id: String,
    /// Corpus class.
    pub corpus_class: ProviderWorkflowCorpusClass,
    /// Expected visible posture.
    pub expected_visible_posture: WorkItemRowPostureClass,
    /// Export-safe summary.
    pub summary: String,
}

impl From<&ProviderWorkflowCorpusCase> for ProviderWorkflowCorpusSupportSummary {
    fn from(record: &ProviderWorkflowCorpusCase) -> Self {
        Self {
            case_id: record.case_id.clone(),
            corpus_class: record.corpus_class,
            expected_visible_posture: record.expected_visible_posture,
            summary: record.support_export_summary.clone(),
        }
    }
}

/// Validates a work-item transition beta page, returning typed defects on failure.
pub fn validate_work_item_transition_beta_page(
    page: &WorkItemTransitionBetaPage,
) -> Result<(), Vec<WorkItemTransitionBetaDefect>> {
    let defects = audit_work_item_transition_beta_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Audits a work-item transition beta page and returns every typed defect.
pub fn audit_work_item_transition_beta_page(
    page: &WorkItemTransitionBetaPage,
) -> Vec<WorkItemTransitionBetaDefect> {
    let mut validator = WorkItemTransitionBetaValidator::new(page);
    validator.run();
    validator.defects
}

struct WorkItemTransitionBetaValidator<'a> {
    page: &'a WorkItemTransitionBetaPage,
    detail_ids: BTreeSet<&'a str>,
    transition_packet_ids: BTreeSet<&'a str>,
    transition_review_ids: BTreeSet<&'a str>,
    offline_handoff_ids: BTreeSet<&'a str>,
    coverage: WorkItemTransitionBetaCoverage,
    defects: Vec<WorkItemTransitionBetaDefect>,
}

impl<'a> WorkItemTransitionBetaValidator<'a> {
    fn new(page: &'a WorkItemTransitionBetaPage) -> Self {
        Self {
            page,
            detail_ids: BTreeSet::new(),
            transition_packet_ids: BTreeSet::new(),
            transition_review_ids: BTreeSet::new(),
            offline_handoff_ids: BTreeSet::new(),
            coverage: WorkItemTransitionBetaCoverage::default(),
            defects: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_details();
        self.validate_transition_packets();
        self.validate_transition_reviews();
        self.validate_offline_handoffs();
        self.validate_corpus_cases();
        self.validate_required_coverage();
    }

    fn finish(self) -> WorkItemTransitionBetaValidationReport {
        WorkItemTransitionBetaValidationReport {
            record_kind: WORK_ITEM_TRANSITION_BETA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed: self.defects.is_empty(),
            coverage: self.coverage,
            defects: self.defects,
        }
    }

    fn validate_page_header(&mut self) {
        self.expect(
            self.page.record_kind == WORK_ITEM_TRANSITION_BETA_PAGE_RECORD_KIND,
            WorkItemTransitionBetaDefectKind::PageContractInvalid,
            "work_item_transition_beta.page_record_kind",
            "page.record_kind must be providers_work_item_transition_beta_page_record",
            Some(&self.page.page_id),
        );
        self.expect(
            self.page.schema_version == WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
            WorkItemTransitionBetaDefectKind::PageContractInvalid,
            "work_item_transition_beta.page_schema_version",
            "page.schema_version must match the crate constant",
            Some(&self.page.page_id),
        );
        self.expect(
            self.page.shared_contract_ref == WORK_ITEM_TRANSITION_BETA_SHARED_CONTRACT_REF,
            WorkItemTransitionBetaDefectKind::PageContractInvalid,
            "work_item_transition_beta.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
            Some(&self.page.page_id),
        );
        self.expect_non_empty(
            &self.page.page_id,
            "work_item_transition_beta.page_id_missing",
            Some(&self.page.page_id),
        );
        self.expect_non_empty(
            &self.page.support_export_summary,
            "work_item_transition_beta.page_summary_missing",
            Some(&self.page.page_id),
        );
        for contract_ref in self.page.contract_refs.all_refs() {
            self.expect_non_empty(
                contract_ref,
                "work_item_transition_beta.contract_ref_missing",
                Some(&self.page.page_id),
            );
        }
        self.expect(
            !self.page.detail_records.is_empty(),
            WorkItemTransitionBetaDefectKind::RequiredFieldMissing,
            "work_item_transition_beta.details_missing",
            "page must carry at least one work-item detail record",
            Some(&self.page.page_id),
        );
    }

    fn validate_details(&mut self) {
        for detail in &self.page.detail_records {
            self.expect(
                detail.record_kind == WORK_ITEM_DETAIL_RECORD_KIND,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.detail_record_kind",
                "detail.record_kind must be work_item_detail_record",
                Some(&detail.detail_id),
            );
            self.expect(
                detail.schema_version == WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.detail_schema_version",
                "detail.schema_version must match the crate constant",
                Some(&detail.detail_id),
            );
            let unique = self.detail_ids.insert(detail.detail_id.as_str());
            self.expect(
                unique,
                WorkItemTransitionBetaDefectKind::DuplicateId,
                "work_item_transition_beta.detail_duplicate_id",
                "detail ids must be unique",
                Some(&detail.detail_id),
            );
            for (value, check) in [
                (
                    &detail.detail_id,
                    "work_item_transition_beta.detail_id_missing",
                ),
                (
                    &detail.provider_descriptor_ref,
                    "work_item_transition_beta.detail_provider_descriptor_missing",
                ),
                (
                    &detail.connected_provider_record_id_ref,
                    "work_item_transition_beta.detail_connected_provider_missing",
                ),
                (
                    &detail.provider_label,
                    "work_item_transition_beta.detail_provider_label_missing",
                ),
                (
                    &detail.project_or_space_ref,
                    "work_item_transition_beta.detail_project_space_missing",
                ),
                (
                    &detail.canonical_id,
                    "work_item_transition_beta.detail_canonical_id_missing",
                ),
                (
                    &detail.title_label,
                    "work_item_transition_beta.detail_title_missing",
                ),
                (
                    &detail.freshness_floor_ref,
                    "work_item_transition_beta.detail_freshness_floor_missing",
                ),
                (
                    &detail.support_export_summary,
                    "work_item_transition_beta.detail_summary_missing",
                ),
            ] {
                self.expect_non_empty(value, check, Some(&detail.detail_id));
            }
            self.expect(
                !detail.current_state_rows.is_empty(),
                WorkItemTransitionBetaDefectKind::RequiredFieldMissing,
                "work_item_transition_beta.detail_state_rows_missing",
                "detail rows must include current provider/local state rows",
                Some(&detail.detail_id),
            );
            self.expect(
                !detail.owner_or_assignee_rows.is_empty(),
                WorkItemTransitionBetaDefectKind::RequiredFieldMissing,
                "work_item_transition_beta.detail_owner_rows_missing",
                "detail rows must include owner or assignee rows",
                Some(&detail.detail_id),
            );
            self.expect(
                detail
                    .engineering_artifact_relations
                    .relation_axes_present(),
                WorkItemTransitionBetaDefectKind::RequiredFieldMissing,
                "work_item_transition_beta.detail_relation_axes_missing",
                "detail rows must carry all engineering-artifact relation axes",
                Some(&detail.detail_id),
            );
            self.expect(
                !detail.raw_provider_payload_refs_present && !detail.raw_provider_url_present,
                WorkItemTransitionBetaDefectKind::RawProviderMaterialPresent,
                "work_item_transition_beta.detail_raw_provider_material_present",
                "detail rows must not carry raw provider payloads or URLs",
                Some(&detail.detail_id),
            );
            self.expect(
                !detail.open_external_action.raw_url_present,
                WorkItemTransitionBetaDefectKind::RawProviderMaterialPresent,
                "work_item_transition_beta.detail_open_external_raw_url",
                "open-external actions must not carry raw provider URLs",
                Some(&detail.detail_id),
            );
            if detail.open_external_action.action_class
                == OpenExternalActionClass::BrowserHandoffRequired
            {
                self.expect(
                    non_empty_opt(&detail.open_external_action.browser_handoff_packet_ref),
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_open_external_handoff_ref_missing",
                    "browser-handoff open-external actions must cite a handoff packet",
                    Some(&detail.detail_id),
                );
            }
            self.validate_detail_coherence(detail);
            self.coverage.row_postures.insert(detail.row_posture_class);
            self.coverage
                .authority_classes
                .insert(detail.work_item_authority_class);
            self.coverage
                .freshness_classes
                .insert(detail.freshness_class);
            self.coverage
                .write_authority_classes
                .insert(detail.write_authority_class);
        }
    }

    fn validate_detail_coherence(&mut self, detail: &WorkItemDetailRecord) {
        match detail.row_posture_class {
            WorkItemRowPostureClass::ProviderAuthoritative => {
                self.expect(
                    detail.work_item_authority_class
                        == WorkItemAuthorityClass::ProviderAuthoritativeSynced,
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_provider_authoritative_authority",
                    "provider-authoritative rows must use provider_authoritative_synced",
                    Some(&detail.detail_id),
                );
                self.expect(
                    detail.freshness_class == WorkItemFreshnessClass::LiveAuthoritativeFresh,
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_provider_authoritative_freshness",
                    "provider-authoritative rows must be live-authoritative-fresh",
                    Some(&detail.detail_id),
                );
            }
            WorkItemRowPostureClass::CachedStale => {
                self.expect(
                    detail.freshness_class.is_degraded()
                        || detail.freshness_class == WorkItemFreshnessClass::WarmWithinGrace,
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_cached_stale_freshness",
                    "cached/stale rows must disclose non-fresh or grace-window freshness",
                    Some(&detail.detail_id),
                );
                self.expect(
                    detail.write_authority_class.blocks_provider_write(),
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_cached_stale_write",
                    "cached/stale rows cannot claim immediate provider-write authority",
                    Some(&detail.detail_id),
                );
            }
            WorkItemRowPostureClass::ReadOnly => {
                self.expect(
                    detail.write_authority_class
                        == WriteAuthorityClass::WriteBlockedProviderReadOnlyScope
                        || detail.work_item_authority_class
                            == WorkItemAuthorityClass::CachedReadOnlyShadowInspectOnly,
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_read_only_scope",
                    "read-only rows must name read-only write scope or cached shadow authority",
                    Some(&detail.detail_id),
                );
            }
            WorkItemRowPostureClass::PolicyBlocked => {
                self.expect(
                    detail.policy_context.policy_block_ref.is_some()
                        || matches!(
                            detail.write_authority_class,
                            WriteAuthorityClass::WriteBlockedWorkspaceTrustUnsetOrRestricted
                                | WriteAuthorityClass::WriteBlockedManagedAdminOnlySurface
                        ),
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_policy_block_ref_missing",
                    "policy-blocked rows must cite a policy block or managed/trust boundary",
                    Some(&detail.detail_id),
                );
            }
            WorkItemRowPostureClass::LocalDraft => {
                self.expect(
                    detail.work_item_authority_class
                        == WorkItemAuthorityClass::LocalDraftNoProviderObject
                        && detail.freshness_class
                            == WorkItemFreshnessClass::LocalDraftNeverPublished
                        && detail.write_authority_class
                            == WriteAuthorityClass::WriteAdmissibleLocalDraftOnlyNoProviderPath
                        && detail.local_draft_ref.is_some(),
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_local_draft_truth",
                    "local-draft rows must be never-published local draft with a local_draft_ref",
                    Some(&detail.detail_id),
                );
            }
            WorkItemRowPostureClass::Queued => {
                self.expect(
                    detail.work_item_authority_class
                        == WorkItemAuthorityClass::QueuedPublishLocalAuthored
                        && detail.publish_later_queue_item_ref.is_some(),
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_queued_truth",
                    "queued rows must use queued authority and cite a publish-later queue item",
                    Some(&detail.detail_id),
                );
            }
            WorkItemRowPostureClass::OfflineCaptured => {
                self.expect(
                    !detail
                        .linked_offline_handoff_packet_record_id_refs
                        .is_empty(),
                    WorkItemTransitionBetaDefectKind::DetailTruthIncoherent,
                    "work_item_transition_beta.detail_offline_handoff_ref_missing",
                    "offline-captured rows must cite an offline-handoff packet",
                    Some(&detail.detail_id),
                );
            }
        }
    }

    fn validate_transition_packets(&mut self) {
        for packet in &self.page.transition_packets {
            self.expect(
                packet.record_kind == STATUS_TRANSITION_PACKET_RECORD_KIND,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.transition_record_kind",
                "transition packet record_kind must be status_transition_packet_record",
                Some(&packet.packet_id),
            );
            self.expect(
                packet.schema_version == WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.transition_schema_version",
                "transition packet schema_version must match the crate constant",
                Some(&packet.packet_id),
            );
            let unique = self.transition_packet_ids.insert(packet.packet_id.as_str());
            self.expect(
                unique,
                WorkItemTransitionBetaDefectKind::DuplicateId,
                "work_item_transition_beta.transition_duplicate_id",
                "transition packet ids must be unique",
                Some(&packet.packet_id),
            );
            self.expect(
                self.detail_ids
                    .contains(packet.work_item_detail_record_id_ref.as_str()),
                WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                "work_item_transition_beta.transition_unknown_detail_ref",
                "transition packet must bind an existing detail record",
                Some(&packet.packet_id),
            );
            self.expect(
                !packet.transition_entries.is_empty(),
                WorkItemTransitionBetaDefectKind::RequiredFieldMissing,
                "work_item_transition_beta.transition_entries_missing",
                "transition packet must carry at least one transition entry",
                Some(&packet.packet_id),
            );
            self.expect(
                !packet.raw_provider_payload_refs_present,
                WorkItemTransitionBetaDefectKind::RawProviderMaterialPresent,
                "work_item_transition_beta.transition_raw_provider_material_present",
                "transition packets must not carry raw provider payload refs",
                Some(&packet.packet_id),
            );
            self.expect(
                packet.action_affordances.export_action_available
                    && packet.action_affordances.cancel_action_available,
                WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                "work_item_transition_beta.transition_export_cancel_missing",
                "transition sheets must expose export and cancel actions",
                Some(&packet.packet_id),
            );
            if packet.packet_admissibility_class.is_admissible() {
                self.expect(
                    packet.action_affordances.confirm_action_available,
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_confirm_missing",
                    "admissible transition packets must expose a confirm action",
                    Some(&packet.packet_id),
                );
            }
            self.validate_transition_mode_refs(packet);
            for entry in &packet.transition_entries {
                self.validate_transition_entry(packet, entry);
                self.coverage
                    .transition_action_classes
                    .insert(entry.transition_action_class);
            }
            self.coverage.mutation_modes.insert(packet.mutation_mode);
            self.coverage
                .transition_admissibility_classes
                .insert(packet.packet_admissibility_class);
        }
    }

    fn validate_transition_mode_refs(&mut self, packet: &StatusTransitionPacketRecord) {
        match packet.mutation_mode {
            WorkItemMutationMode::PublishNow => {
                self.expect(
                    non_empty_opt(&packet.linked_provider_consequence_preview_record_id_ref),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_publish_now_preview_missing",
                    "publish-now transitions must cite a provider consequence preview",
                    Some(&packet.packet_id),
                );
            }
            WorkItemMutationMode::OpenInProvider => {
                self.expect(
                    non_empty_opt(&packet.linked_browser_handoff_packet_ref),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_browser_handoff_missing",
                    "open-in-provider transitions must cite a browser-handoff packet",
                    Some(&packet.packet_id),
                );
            }
            WorkItemMutationMode::DeferredPublish => {
                if packet.packet_admissibility_class
                    == TransitionAdmissibilityClass::AdmissibleViaQueueForPublishLater
                {
                    self.expect(
                        non_empty_opt(&packet.linked_publish_later_queue_item_record_id_ref),
                        WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                        "work_item_transition_beta.transition_queue_ref_missing",
                        "queued transitions must cite a publish-later queue item",
                        Some(&packet.packet_id),
                    );
                }
            }
            WorkItemMutationMode::LocalDraft | WorkItemMutationMode::InspectOnly => {
                self.expect(
                    packet.linked_provider_consequence_preview_record_id_ref.is_none(),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_local_preview_claim",
                    "local-draft and inspect-only transitions must not imply provider consequence preview",
                    Some(&packet.packet_id),
                );
            }
        }
    }

    fn validate_transition_entry(
        &mut self,
        packet: &StatusTransitionPacketRecord,
        entry: &TransitionEntry,
    ) {
        self.expect_non_empty(
            &entry.transition_entry_id,
            "work_item_transition_beta.transition_entry_id_missing",
            Some(&packet.packet_id),
        );
        self.expect_non_empty(
            &entry.summary,
            "work_item_transition_beta.transition_entry_summary_missing",
            Some(&packet.packet_id),
        );
        if entry.transition_kind_class == TransitionKindClass::ChangeLifecycleStateToken {
            self.expect(
                !entry.previous_state_value.trim().is_empty()
                    && !entry.next_state_value.trim().is_empty(),
                WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                "work_item_transition_beta.transition_lifecycle_state_missing",
                "lifecycle transitions must disclose previous and next provider/local state tokens",
                Some(&packet.packet_id),
            );
        }
        if packet.mutation_mode == WorkItemMutationMode::LocalDraft {
            self.expect(
                !entry.transition_action_class.mutates_provider_now(),
                WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                "work_item_transition_beta.transition_silent_provider_mutation_local_draft",
                "local-draft transition packets must not contain provider mutation entries",
                Some(&packet.packet_id),
            );
        }
        match entry.transition_action_class {
            TransitionActionClass::MutateProviderStatePublishNow => {
                self.expect(
                    packet.mutation_mode == WorkItemMutationMode::PublishNow
                        && matches!(
                            entry.permission_scope_class,
                            PermissionScopeClass::PermissionAdmissibleUserWritesProvider
                                | PermissionScopeClass::PermissionAdmissibleAssigneeOnly
                                | PermissionScopeClass::PermissionAdmissibleUnderInstallOrAppGrant
                        ),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_publish_now_scope",
                    "publish-now provider mutations must use publish_now mode and write-admissible permission scope",
                    Some(&packet.packet_id),
                );
            }
            TransitionActionClass::QueueForPublishLaterDeferred => {
                self.expect(
                    packet.mutation_mode == WorkItemMutationMode::DeferredPublish
                        && non_empty_opt(&entry.linked_publish_later_queue_item_record_id_ref),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_queue_entry_ref_missing",
                    "queue transition entries must cite the publish-later queue item",
                    Some(&packet.packet_id),
                );
            }
            TransitionActionClass::RouteThroughBrowserHandoffOpenInProvider => {
                self.expect(
                    packet.mutation_mode == WorkItemMutationMode::OpenInProvider
                        && non_empty_opt(&entry.linked_browser_handoff_packet_ref),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_browser_entry_ref_missing",
                    "browser-handoff transition entries must cite the browser-handoff packet",
                    Some(&packet.packet_id),
                );
            }
            TransitionActionClass::CapturedOfflinePendingDrain => {
                self.expect(
                    packet.mutation_mode == WorkItemMutationMode::DeferredPublish
                        && non_empty_opt(&entry.linked_offline_handoff_packet_record_id_ref),
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_offline_entry_ref_missing",
                    "offline-captured transition entries must cite the offline-handoff packet",
                    Some(&packet.packet_id),
                );
            }
            TransitionActionClass::SaveLocalDraftOnlyNoProviderPath
            | TransitionActionClass::InspectOnlyNoMutation => {
                self.expect(
                    entry.notification_side_effect_class
                        == NotificationSideEffectClass::NoNotificationLocallyOnly,
                    WorkItemTransitionBetaDefectKind::TransitionTruthIncoherent,
                    "work_item_transition_beta.transition_local_notification_claim",
                    "local or inspect-only entries must not claim provider notifications",
                    Some(&packet.packet_id),
                );
            }
        }
    }

    fn validate_transition_reviews(&mut self) {
        for review in &self.page.transition_reviews {
            self.expect(
                review.record_kind == TRANSITION_REVIEW_RECORD_KIND,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.review_record_kind",
                "transition review record_kind must be transition_review_record",
                Some(&review.review_id),
            );
            self.expect(
                review.schema_version == WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.review_schema_version",
                "transition review schema_version must match the crate constant",
                Some(&review.review_id),
            );
            let unique = self.transition_review_ids.insert(review.review_id.as_str());
            self.expect(
                unique,
                WorkItemTransitionBetaDefectKind::DuplicateId,
                "work_item_transition_beta.review_duplicate_id",
                "transition review ids must be unique",
                Some(&review.review_id),
            );
            self.expect(
                self.detail_ids
                    .contains(review.work_item_detail_record_id_ref.as_str()),
                WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                "work_item_transition_beta.review_unknown_detail_ref",
                "transition review must bind an existing detail record",
                Some(&review.review_id),
            );
            self.expect(
                self.transition_packet_ids.contains(
                    review
                        .linked_status_transition_packet_record_id_ref
                        .as_str(),
                ),
                WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                "work_item_transition_beta.review_unknown_transition_packet_ref",
                "transition review must bind an existing status-transition packet",
                Some(&review.review_id),
            );
            for (value, check) in [
                (
                    &review.target_item_ref,
                    "work_item_transition_beta.review_target_missing",
                ),
                (
                    &review.provider_label,
                    "work_item_transition_beta.review_provider_missing",
                ),
                (
                    &review.current_state_summary,
                    "work_item_transition_beta.review_current_state_missing",
                ),
                (
                    &review.requested_state_summary,
                    "work_item_transition_beta.review_requested_state_missing",
                ),
                (
                    &review.actor_authority_summary,
                    "work_item_transition_beta.review_actor_authority_missing",
                ),
                (
                    &review.summary,
                    "work_item_transition_beta.review_summary_missing",
                ),
            ] {
                self.expect_non_empty(value, check, Some(&review.review_id));
            }
            self.expect(
                !review.side_effect_fanout_rows.is_empty(),
                WorkItemTransitionBetaDefectKind::TransitionReviewIncomplete,
                "work_item_transition_beta.review_fanout_missing",
                "transition reviews must disclose side effects",
                Some(&review.review_id),
            );
            self.expect(
                review.action_affordances.export_action_available
                    && review.action_affordances.cancel_action_available,
                WorkItemTransitionBetaDefectKind::TransitionReviewIncomplete,
                "work_item_transition_beta.review_export_cancel_missing",
                "transition reviews must expose export and cancel actions",
                Some(&review.review_id),
            );
            if review.transition_review_disposition_class.is_admissible() {
                self.expect(
                    review.action_affordances.confirm_action_available,
                    WorkItemTransitionBetaDefectKind::TransitionReviewIncomplete,
                    "work_item_transition_beta.review_confirm_missing",
                    "admissible transition reviews must expose confirm",
                    Some(&review.review_id),
                );
            }
            self.expect(
                review.local_draft_preserved_on_failure,
                WorkItemTransitionBetaDefectKind::TransitionReviewIncomplete,
                "work_item_transition_beta.review_local_draft_not_preserved",
                "transition reviews must preserve local draft state on publish failure",
                Some(&review.review_id),
            );
            self.expect(
                !review.raw_provider_payload_refs_present,
                WorkItemTransitionBetaDefectKind::RawProviderMaterialPresent,
                "work_item_transition_beta.review_raw_provider_material_present",
                "transition reviews must not carry raw provider payload refs",
                Some(&review.review_id),
            );
            for row in &review.side_effect_fanout_rows {
                self.validate_fanout_row(review, row);
                self.coverage
                    .side_effect_fanout_kinds
                    .insert(row.fanout_kind_class);
            }
        }
    }

    fn validate_fanout_row(
        &mut self,
        review: &TransitionReviewSheetRecord,
        row: &SideEffectFanoutRow,
    ) {
        self.expect_non_empty(
            &row.fanout_row_id,
            "work_item_transition_beta.fanout_row_id_missing",
            Some(&review.review_id),
        );
        self.expect_non_empty(
            &row.summary,
            "work_item_transition_beta.fanout_summary_missing",
            Some(&review.review_id),
        );
        if row.fanout_kind_class == SideEffectFanoutKindClass::ProviderMutationFanout {
            self.expect(
                !matches!(
                    row.publish_mode_class,
                    WorkItemMutationMode::LocalDraft | WorkItemMutationMode::InspectOnly
                ) && row.target_account_class != TargetAccountClass::LocalOnlyNoProviderAccount,
                WorkItemTransitionBetaDefectKind::TransitionReviewIncomplete,
                "work_item_transition_beta.fanout_provider_mutation_local_only",
                "provider mutation fanout cannot target local-only account or local/inspect publish mode",
                Some(&review.review_id),
            );
        }
        if row.fanout_kind_class == SideEffectFanoutKindClass::LinkedReviewUpdateFanout {
            self.expect(
                non_empty_opt(&row.linked_record_id_ref),
                WorkItemTransitionBetaDefectKind::TransitionReviewIncomplete,
                "work_item_transition_beta.fanout_linked_review_ref_missing",
                "linked-review fanout rows must cite the linked record",
                Some(&review.review_id),
            );
        }
    }

    fn validate_offline_handoffs(&mut self) {
        for packet in &self.page.offline_handoff_packets {
            self.expect(
                packet.record_kind == OFFLINE_HANDOFF_PACKET_RECORD_KIND,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.offline_record_kind",
                "offline handoff record_kind must be offline_handoff_packet_record",
                Some(&packet.packet_id),
            );
            self.expect(
                packet.schema_version == WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.offline_schema_version",
                "offline handoff schema_version must match the crate constant",
                Some(&packet.packet_id),
            );
            let unique = self.offline_handoff_ids.insert(packet.packet_id.as_str());
            self.expect(
                unique,
                WorkItemTransitionBetaDefectKind::DuplicateId,
                "work_item_transition_beta.offline_duplicate_id",
                "offline handoff packet ids must be unique",
                Some(&packet.packet_id),
            );
            self.expect(
                self.detail_ids
                    .contains(packet.work_item_detail_record_id_ref.as_str()),
                WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                "work_item_transition_beta.offline_unknown_detail_ref",
                "offline handoff packets must bind an existing detail record",
                Some(&packet.packet_id),
            );
            self.expect(
                self.transition_packet_ids.contains(
                    packet
                        .linked_status_transition_packet_record_id_ref
                        .as_str(),
                ),
                WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                "work_item_transition_beta.offline_unknown_transition_ref",
                "offline handoff packets must bind an existing status-transition packet",
                Some(&packet.packet_id),
            );
            self.expect(
                !packet.snapshot_state_rows.is_empty()
                    && !packet.snapshot_owner_or_assignee_rows.is_empty()
                    && snapshot_axes_complete(&packet.snapshot_engineering_relations),
                WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                "work_item_transition_beta.offline_snapshot_incomplete",
                "offline handoff packets must preserve state, owner, and all relation axes",
                Some(&packet.packet_id),
            );
            self.expect(
                !packet.handoff_export_route_classes.is_empty()
                    && !packet.handoff_retry_route_classes.is_empty(),
                WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                "work_item_transition_beta.offline_routes_missing",
                "offline handoff packets must expose retry and export routes",
                Some(&packet.packet_id),
            );
            for (value, check) in [
                (
                    &packet.captured_freshness_floor_ref,
                    "work_item_transition_beta.offline_freshness_floor_missing",
                ),
                (
                    &packet.redaction_manifest_ref,
                    "work_item_transition_beta.offline_redaction_manifest_missing",
                ),
                (
                    &packet.publish_target_ref,
                    "work_item_transition_beta.offline_publish_target_missing",
                ),
                (
                    &packet.retry_action_ref,
                    "work_item_transition_beta.offline_retry_action_missing",
                ),
                (
                    &packet.export_action_ref,
                    "work_item_transition_beta.offline_export_action_missing",
                ),
                (
                    &packet.summary,
                    "work_item_transition_beta.offline_summary_missing",
                ),
            ] {
                self.expect_non_empty(value, check, Some(&packet.packet_id));
            }
            self.expect(
                packet.packet_survives_restart,
                WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                "work_item_transition_beta.offline_not_durable",
                "offline handoff packets must survive restarts",
                Some(&packet.packet_id),
            );
            self.expect(
                !packet.raw_provider_payload_refs_present,
                WorkItemTransitionBetaDefectKind::RawProviderMaterialPresent,
                "work_item_transition_beta.offline_raw_provider_material_present",
                "offline handoff packets must not carry raw provider payload refs",
                Some(&packet.packet_id),
            );
            self.validate_offline_acceptance(packet);
            self.coverage
                .handoff_admission_reasons
                .insert(packet.handoff_admission_reason_class);
            self.coverage
                .handoff_provider_acceptance_classes
                .insert(packet.handoff_provider_acceptance_class);
            self.coverage
                .handoff_drain_states
                .insert(packet.handoff_drain_state_class);
            self.coverage
                .handoff_export_routes
                .extend(packet.handoff_export_route_classes.iter().copied());
            self.coverage
                .handoff_retry_routes
                .extend(packet.handoff_retry_route_classes.iter().copied());
        }
    }

    fn validate_offline_acceptance(&mut self, packet: &OfflineHandoffPacketRecord) {
        match packet.handoff_provider_acceptance_class {
            HandoffProviderAcceptanceClass::NotSubmittedLocalCaptureOnly => {
                self.expect(
                    packet
                        .linked_provider_callback_envelope_record_id_refs
                        .is_empty(),
                    WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                    "work_item_transition_beta.offline_not_submitted_has_callback",
                    "not-submitted offline packets must not cite provider callback acceptance",
                    Some(&packet.packet_id),
                );
            }
            HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained => {
                self.expect(
                    packet.handoff_drain_state_class
                        == HandoffDrainStateClass::DrainedPublishLaterCompleted
                        && non_empty_opt(&packet.linked_publish_later_queue_item_record_id_ref)
                        && !packet.linked_provider_callback_envelope_record_id_refs.is_empty(),
                    WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                    "work_item_transition_beta.offline_accept_without_callback",
                    "provider acceptance requires completed drain, queue item, and callback envelope",
                    Some(&packet.packet_id),
                );
            }
            HandoffProviderAcceptanceClass::ProviderAcceptRejectedWithTypedReason => {
                self.expect(
                    packet.handoff_drain_state_class
                        == HandoffDrainStateClass::DrainedPublishLaterRejectedWithTypedReason
                        && packet
                            .acceptance_rejection_reason
                            .as_deref()
                            .is_some_and(|reason| !reason.trim().is_empty()),
                    WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                    "work_item_transition_beta.offline_reject_reason_missing",
                    "provider rejection requires rejected drain state and typed rejection reason",
                    Some(&packet.packet_id),
                );
            }
            HandoffProviderAcceptanceClass::ImportedHandoffEvidenceOnlyNoProviderPath => {
                self.expect(
                    packet.handoff_retry_route_classes
                        == vec![HandoffRetryRouteClass::NoRetryImportedEvidenceOnly]
                        && packet.handoff_admission_reason_class
                            == HandoffAdmissionReasonClass::ImportedFromSupportExportNoLiveProviderPath,
                    WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                    "work_item_transition_beta.offline_imported_retry_truth",
                    "imported evidence-only packets must use imported admission and no-retry route",
                    Some(&packet.packet_id),
                );
            }
            HandoffProviderAcceptanceClass::SubmittedPendingProviderAcceptUnverified => {
                self.expect(
                    !packet
                        .linked_provider_callback_envelope_record_id_refs
                        .is_empty(),
                    WorkItemTransitionBetaDefectKind::OfflineHandoffTruthIncoherent,
                    "work_item_transition_beta.offline_submitted_callback_missing",
                    "submitted-unverified packets must cite the callback envelope under review",
                    Some(&packet.packet_id),
                );
            }
        }
    }

    fn validate_corpus_cases(&mut self) {
        for case in &self.page.workflow_corpus_cases {
            self.expect(
                case.record_kind == PROVIDER_WORKFLOW_CORPUS_CASE_RECORD_KIND,
                WorkItemTransitionBetaDefectKind::PageContractInvalid,
                "work_item_transition_beta.corpus_record_kind",
                "workflow corpus record_kind must match the crate constant",
                Some(&case.case_id),
            );
            self.expect(
                self.detail_ids
                    .contains(case.work_item_detail_record_id_ref.as_str()),
                WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                "work_item_transition_beta.corpus_unknown_detail_ref",
                "workflow corpus rows must bind an existing detail record",
                Some(&case.case_id),
            );
            if let Some(packet_ref) = &case.status_transition_packet_record_id_ref {
                self.expect(
                    self.transition_packet_ids.contains(packet_ref.as_str()),
                    WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                    "work_item_transition_beta.corpus_unknown_transition_ref",
                    "workflow corpus transition refs must bind existing transition packets",
                    Some(&case.case_id),
                );
            }
            if let Some(packet_ref) = &case.offline_handoff_packet_record_id_ref {
                self.expect(
                    self.offline_handoff_ids.contains(packet_ref.as_str()),
                    WorkItemTransitionBetaDefectKind::UnknownRecordReference,
                    "work_item_transition_beta.corpus_unknown_offline_ref",
                    "workflow corpus offline refs must bind existing offline packets",
                    Some(&case.case_id),
                );
            }
            self.expect_non_empty(
                &case.support_export_summary,
                "work_item_transition_beta.corpus_summary_missing",
                Some(&case.case_id),
            );
            self.coverage
                .workflow_corpus_classes
                .insert(case.corpus_class);
        }
    }

    fn validate_required_coverage(&mut self) {
        for posture in [
            WorkItemRowPostureClass::ProviderAuthoritative,
            WorkItemRowPostureClass::CachedStale,
            WorkItemRowPostureClass::ReadOnly,
            WorkItemRowPostureClass::PolicyBlocked,
            WorkItemRowPostureClass::LocalDraft,
            WorkItemRowPostureClass::Queued,
            WorkItemRowPostureClass::OfflineCaptured,
        ] {
            self.expect(
                self.coverage.row_postures.contains(&posture),
                WorkItemTransitionBetaDefectKind::CoverageMissing,
                "work_item_transition_beta.coverage_row_posture_missing",
                "page must cover provider-authoritative, cached/stale, read-only, policy-blocked, local-draft, queued, and offline-captured states",
                Some(posture.as_str()),
            );
        }
        for mode in [
            WorkItemMutationMode::PublishNow,
            WorkItemMutationMode::DeferredPublish,
            WorkItemMutationMode::OpenInProvider,
            WorkItemMutationMode::LocalDraft,
        ] {
            self.expect(
                self.coverage.mutation_modes.contains(&mode),
                WorkItemTransitionBetaDefectKind::CoverageMissing,
                "work_item_transition_beta.coverage_mutation_mode_missing",
                "page must cover publish-now, deferred-publish, open-in-provider, and local-draft transition modes",
                Some("mutation_mode"),
            );
        }
        for action in [
            TransitionActionClass::MutateProviderStatePublishNow,
            TransitionActionClass::QueueForPublishLaterDeferred,
            TransitionActionClass::RouteThroughBrowserHandoffOpenInProvider,
            TransitionActionClass::SaveLocalDraftOnlyNoProviderPath,
            TransitionActionClass::CapturedOfflinePendingDrain,
        ] {
            self.expect(
                self.coverage.transition_action_classes.contains(&action),
                WorkItemTransitionBetaDefectKind::CoverageMissing,
                "work_item_transition_beta.coverage_transition_action_missing",
                "page must cover provider mutation, queue, browser handoff, local draft, and offline-captured transition actions",
                Some("transition_action"),
            );
        }
        for corpus in [
            ProviderWorkflowCorpusClass::StaleTargetId,
            ProviderWorkflowCorpusClass::RevokedCredentials,
            ProviderWorkflowCorpusClass::ConflictingUpdates,
            ProviderWorkflowCorpusClass::ReadOnlySession,
            ProviderWorkflowCorpusClass::OfflineCapture,
            ProviderWorkflowCorpusClass::PublishLaterReplay,
        ] {
            self.expect(
                self.coverage.workflow_corpus_classes.contains(&corpus),
                WorkItemTransitionBetaDefectKind::CoverageMissing,
                "work_item_transition_beta.coverage_workflow_corpus_missing",
                "page must cover stale target IDs, revoked credentials, conflicting updates, read-only sessions, offline capture, and publish-later replay",
                Some("workflow_corpus"),
            );
        }
    }

    fn expect_non_empty(&mut self, value: &str, check_id: &str, record_ref: Option<&str>) {
        self.expect(
            !value.trim().is_empty(),
            WorkItemTransitionBetaDefectKind::RequiredFieldMissing,
            check_id,
            "required value must be non-empty",
            record_ref,
        );
    }

    fn expect(
        &mut self,
        passed: bool,
        defect_kind: WorkItemTransitionBetaDefectKind,
        check_id: &str,
        message: &str,
        record_ref: Option<&str>,
    ) {
        if !passed {
            self.defects.push(WorkItemTransitionBetaDefect {
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

fn snapshot_axes_complete(relations: &[SnapshotEngineeringRelation]) -> bool {
    let axes: BTreeSet<_> = relations
        .iter()
        .map(|relation| relation.relation_axis_class)
        .collect();
    [
        SnapshotRelationAxisClass::IssueToBranchLink,
        SnapshotRelationAxisClass::LinkedReview,
        SnapshotRelationAxisClass::ChangeIntent,
        SnapshotRelationAxisClass::ValidationEvidence,
        SnapshotRelationAxisClass::PublishPreview,
    ]
    .into_iter()
    .all(|axis| axes.contains(&axis))
}

/// Builds a seeded beta page covering provider work-item transition and handoff lanes.
pub fn seeded_work_item_transition_beta_page() -> WorkItemTransitionBetaPage {
    let detail_records = seeded_detail_records();
    let transition_packets = seeded_transition_packets();
    let transition_reviews = seeded_transition_reviews();
    let offline_handoff_packets = seeded_offline_handoff_packets();
    let workflow_corpus_cases = seeded_workflow_corpus_cases();
    WorkItemTransitionBetaPage {
        fixture_metadata: Some(WorkItemTransitionFixtureMetadata {
            name: "provider_work_item_transition_beta".to_string(),
            scenario: "Provider work-item rows, transition review sheets, and offline handoff packets cover authoritative, stale, read-only, policy-blocked, local-draft, queued, and offline-captured lanes without claiming provider success for local packets.".to_string(),
        }),
        record_kind: WORK_ITEM_TRANSITION_BETA_PAGE_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
        shared_contract_ref: WORK_ITEM_TRANSITION_BETA_SHARED_CONTRACT_REF.to_string(),
        page_id: "providers.work_item_transition_beta.page".to_string(),
        contract_refs: WorkItemTransitionContractRefs {
            provider_work_item_detail_schema_ref: "schemas/providers/work_item_detail.schema.json"
                .to_string(),
            provider_offline_handoff_packet_schema_ref:
                "schemas/providers/offline_handoff_packet.schema.json".to_string(),
            status_transition_packet_schema_ref:
                "schemas/work_items/status_transition_packet.schema.json".to_string(),
            transition_review_schema_ref: "schemas/work_items/transition_review.schema.json"
                .to_string(),
            publish_later_queue_schema_ref: "schemas/providers/publish_later_queue_alpha.schema.json"
                .to_string(),
            browser_handoff_packet_schema_ref: "schemas/providers/browser_handoff_packet.schema.json"
                .to_string(),
            connected_provider_registry_schema_ref:
                "schemas/providers/connected_provider_registry.schema.json".to_string(),
        },
        detail_records,
        transition_packets,
        transition_reviews,
        offline_handoff_packets,
        workflow_corpus_cases,
        support_export_summary: "Provider-backed work-item detail rows keep provider authority, queued/local draft state, transition fanout, and offline handoff packets separate for support and release review.".to_string(),
    }
}

fn seeded_detail_records() -> Vec<WorkItemDetailRecord> {
    vec![
        detail_record(
            "work_items:detail:provider-authoritative",
            WorkItemRowPostureClass::ProviderAuthoritative,
            WorkItemAuthorityClass::ProviderAuthoritativeSynced,
            WorkItemFreshnessClass::LiveAuthoritativeFresh,
            WriteAuthorityClass::WriteAdmissibleProviderWriteable,
            "AUR-241",
            "provider:issue:241",
            Some("work_items:transition_packet:publish-now"),
            None,
            None,
            None,
        ),
        detail_record(
            "work_items:detail:cached-stale",
            WorkItemRowPostureClass::CachedStale,
            WorkItemAuthorityClass::ProviderAuthoritativeStaleLocalContinues,
            WorkItemFreshnessClass::DegradedBeyondGraceLocalContinues,
            WriteAuthorityClass::WriteBlockedProviderUnreachable,
            "AUR-242",
            "provider:issue:242",
            Some("work_items:transition_packet:offline-captured"),
            Some("work_items:offline_handoff:provider-unreachable"),
            Some("local_draft:issue:242"),
            None,
        ),
        detail_record(
            "work_items:detail:read-only",
            WorkItemRowPostureClass::ReadOnly,
            WorkItemAuthorityClass::CachedReadOnlyShadowInspectOnly,
            WorkItemFreshnessClass::WarmWithinGrace,
            WriteAuthorityClass::WriteBlockedProviderReadOnlyScope,
            "AUR-243",
            "provider:issue:243",
            None,
            None,
            None,
            None,
        ),
        detail_record(
            "work_items:detail:policy-blocked",
            WorkItemRowPostureClass::PolicyBlocked,
            WorkItemAuthorityClass::ProviderAuthoritativeStaleLocalContinues,
            WorkItemFreshnessClass::WarmWithinGrace,
            WriteAuthorityClass::WriteBlockedWorkspaceTrustUnsetOrRestricted,
            "AUR-244",
            "provider:issue:244",
            Some("work_items:transition_packet:policy-blocked"),
            None,
            Some("local_draft:issue:244"),
            None,
        ),
        detail_record(
            "work_items:detail:local-draft",
            WorkItemRowPostureClass::LocalDraft,
            WorkItemAuthorityClass::LocalDraftNoProviderObject,
            WorkItemFreshnessClass::LocalDraftNeverPublished,
            WriteAuthorityClass::WriteAdmissibleLocalDraftOnlyNoProviderPath,
            "TASK-17",
            "local:work_item:17",
            Some("work_items:transition_packet:local-draft"),
            None,
            Some("local_draft:work_item:17"),
            None,
        ),
        detail_record(
            "work_items:detail:queued",
            WorkItemRowPostureClass::Queued,
            WorkItemAuthorityClass::QueuedPublishLocalAuthored,
            WorkItemFreshnessClass::LocalDraftNeverPublished,
            WriteAuthorityClass::WriteAdmissibleQueuedPublishOnly,
            "AUR-245",
            "provider:issue:245",
            Some("work_items:transition_packet:deferred"),
            None,
            Some("local_draft:issue:245"),
            Some("providers:queue_item:issue:245"),
        ),
        detail_record(
            "work_items:detail:offline-captured",
            WorkItemRowPostureClass::OfflineCaptured,
            WorkItemAuthorityClass::ImportedHandoffEvidenceOnly,
            WorkItemFreshnessClass::ImportedSnapshotNoRefreshPath,
            WriteAuthorityClass::WriteBlockedImportedEvidenceOnlyNoProviderPath,
            "INC-246",
            "provider:issue:246",
            None,
            Some("work_items:offline_handoff:imported-evidence"),
            None,
            None,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn detail_record(
    detail_id: &str,
    row_posture_class: WorkItemRowPostureClass,
    authority_class: WorkItemAuthorityClass,
    freshness_class: WorkItemFreshnessClass,
    write_authority_class: WriteAuthorityClass,
    canonical_id: &str,
    provider_side_id: &str,
    transition_ref: Option<&str>,
    offline_ref: Option<&str>,
    local_draft_ref: Option<&str>,
    queue_ref: Option<&str>,
) -> WorkItemDetailRecord {
    let policy_block_ref = (row_posture_class == WorkItemRowPostureClass::PolicyBlocked)
        .then(|| "policy:block:work-item-write".to_string());
    let object_class = match detail_id {
        "work_items:detail:local-draft" => WorkItemObjectClass::TaskOrSubtask,
        "work_items:detail:offline-captured" => WorkItemObjectClass::IncidentReport,
        _ => WorkItemObjectClass::IssueOrWorkItem,
    };
    WorkItemDetailRecord {
        record_kind: WORK_ITEM_DETAIL_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
        detail_id: detail_id.to_string(),
        provider_descriptor_ref: "provider_descriptor.issue.primary".to_string(),
        connected_provider_record_id_ref: "provider:connected:issue-primary".to_string(),
        provider_family: ProviderFamily::IssueTracker,
        provider_label: "Issue tracker".to_string(),
        project_or_space_ref: "provider:project:aureline".to_string(),
        canonical_id: canonical_id.to_string(),
        title_label: format!("Reviewable work-item title for {canonical_id}"),
        target_object_identity: WorkItemObjectIdentity {
            object_class,
            provider_side_id: provider_side_id.to_string(),
            provider_host: "provider:host:issue-primary".to_string(),
            tenant_or_org_scope: "provider:tenant:aureline".to_string(),
            provider_canonical_label: canonical_id.to_string(),
        },
        work_item_authority_class: authority_class,
        row_posture_class,
        current_state_rows: vec![
            WorkItemCurrentStateRow {
                state_family_class: StateFamilyClass::LifecycleState,
                state_value: match row_posture_class {
                    WorkItemRowPostureClass::LocalDraft => "draft_new",
                    WorkItemRowPostureClass::Queued => "queued_for_triage",
                    WorkItemRowPostureClass::OfflineCaptured => "imported_snapshot",
                    WorkItemRowPostureClass::ProviderAuthoritative
                        if object_class == WorkItemObjectClass::IssueOrWorkItem =>
                    {
                        "provider_in_progress"
                    }
                    WorkItemRowPostureClass::CachedStale => "provider_blocked_waiting_validation",
                    WorkItemRowPostureClass::ReadOnly => "provider_ready_for_review",
                    WorkItemRowPostureClass::PolicyBlocked => "provider_ready_for_publish",
                    _ => "provider_triaged",
                }
                .to_string(),
                state_value_origin_class: match row_posture_class {
                    WorkItemRowPostureClass::LocalDraft | WorkItemRowPostureClass::Queued => {
                        StateValueOriginClass::LocalDraftPendingPublish
                    }
                    WorkItemRowPostureClass::OfflineCaptured => {
                        StateValueOriginClass::ImportedHandoffStateTokenNoRefresh
                    }
                    _ => StateValueOriginClass::ProviderAuthoritativeStateToken,
                },
                source_schema_ref: "schemas/work_items/work_item_detail.schema.json".to_string(),
                source_field: "current_state_rows.state_value".to_string(),
                source_ref: provider_side_id.to_string(),
                summary: "Lifecycle state token remains provider/local/import exact.".to_string(),
            },
            WorkItemCurrentStateRow {
                state_family_class: StateFamilyClass::ReviewOrMergeState,
                state_value: "review_workspace_open".to_string(),
                state_value_origin_class: StateValueOriginClass::DerivedFromLinkedReviewState,
                source_schema_ref: "schemas/vcs/review_workspace.schema.json".to_string(),
                source_field: "review_workspace_lifecycle_state".to_string(),
                source_ref: "vcs:review_workspace:work-item".to_string(),
                summary: "Linked review workspace state is summarized by reference.".to_string(),
            },
        ],
        owner_or_assignee_rows: vec![OwnerOrAssigneeRow {
            owner_role_class: OwnerRoleClass::Assignee,
            actor_subject_ref: "actor:human:opaque:assignee".to_string(),
            actor_class: ProviderActorClass::HumanAccount,
            actor_label: "Reviewable assignee label".to_string(),
        }],
        freshness_class,
        freshness_observed_at: "2026-05-18T09:00:00Z".to_string(),
        freshness_floor_ref: "providers:freshness_floor:issue-primary".to_string(),
        write_authority_class,
        engineering_artifact_relations: EngineeringArtifactRelations {
            issue_to_branch_link_class: IssueToBranchLinkClass::LinkedProviderBranchOverlayFetched,
            linked_branch_local_locator_ref: Some("workspace:branch:work-item".to_string()),
            linked_review_class: LinkedReviewClass::LinkedReviewWorkspaceWithProviderOverlay,
            linked_review_workspace_record_id_ref: Some(
                "vcs:review_workspace:work-item".to_string(),
            ),
            change_intent_class: ChangeIntentClass::ChangeObjectProviderAuthoritative,
            linked_change_object_record_id_ref: Some("vcs:change_object:work-item".to_string()),
            validation_evidence_class: ValidationEvidenceClass::LinkedReviewEvaluationResult,
            linked_validation_evidence_record_id_refs: vec![
                "vcs:review_evaluation_result:work-item".to_string(),
            ],
            linked_run_record_id_refs: vec!["ci:run:work-item:preview".to_string()],
            linked_incident_workspace_record_id_refs: vec![
                "incident:workspace:work-item:preview".to_string()
            ],
            publish_preview_class: if queue_ref.is_some() {
                PublishPreviewClass::PublishPreviewPinnedPublishLaterQueueRecord
            } else {
                PublishPreviewClass::PublishPreviewPinnedProviderConsequencePreviewRecord
            },
            linked_provider_consequence_preview_record_id_ref: Some(
                "providers:consequence_preview:work-item".to_string(),
            ),
            linked_publish_later_queue_item_record_id_ref: queue_ref.map(str::to_string),
            linked_offline_handoff_packet_record_id_ref: offline_ref.map(str::to_string),
        },
        open_external_action: OpenExternalAction {
            action_class: match row_posture_class {
                WorkItemRowPostureClass::PolicyBlocked => OpenExternalActionClass::BlockedByPolicy,
                WorkItemRowPostureClass::OfflineCaptured => {
                    OpenExternalActionClass::NotAvailableImportedEvidenceOnly
                }
                _ => OpenExternalActionClass::BrowserHandoffRequired,
            },
            browser_handoff_packet_ref: (!matches!(
                row_posture_class,
                WorkItemRowPostureClass::PolicyBlocked | WorkItemRowPostureClass::OfflineCaptured
            ))
            .then(|| "browser_handoff:issue-primary:open".to_string()),
            action_label: "Open provider object through reviewed handoff".to_string(),
            raw_url_present: false,
        },
        acting_as: ProviderActorClass::HumanAccount,
        local_draft_ref: local_draft_ref.map(str::to_string),
        publish_later_queue_item_ref: queue_ref.map(str::to_string),
        linked_status_transition_packet_record_id_refs: transition_ref
            .map(|value| vec![value.to_string()])
            .unwrap_or_default(),
        linked_offline_handoff_packet_record_id_refs: offline_ref
            .map(|value| vec![value.to_string()])
            .unwrap_or_default(),
        origin_disclosure: origin("exec:work-item-detail"),
        policy_context: WorkItemPolicyContext {
            policy_epoch: "policy:epoch:2026-05-18".to_string(),
            trust_state: if row_posture_class == WorkItemRowPostureClass::PolicyBlocked {
                TrustPosture::Restricted
            } else {
                TrustPosture::Trusted
            },
            execution_context_id: "exec:work-item-detail".to_string(),
            policy_block_ref,
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_provider_payload_refs_present: false,
        raw_provider_url_present: false,
        support_export_summary: format!(
            "{canonical_id} renders as {} with explicit provider/local publish posture.",
            row_posture_class.as_str()
        ),
        captured_at: "2026-05-18T09:00:00Z".to_string(),
    }
}

fn seeded_transition_packets() -> Vec<StatusTransitionPacketRecord> {
    vec![
        transition_packet(
            "work_items:transition_packet:publish-now",
            "work_items:detail:provider-authoritative",
            WorkItemMutationMode::PublishNow,
            TransitionActionClass::MutateProviderStatePublishNow,
            TransitionAdmissibilityClass::AdmissibleNowPublishNow,
            Some("providers:consequence_preview:publish-now"),
            None,
            None,
            None,
            "Publish-now status transition changes provider lifecycle token after review.",
        ),
        transition_packet(
            "work_items:transition_packet:deferred",
            "work_items:detail:queued",
            WorkItemMutationMode::DeferredPublish,
            TransitionActionClass::QueueForPublishLaterDeferred,
            TransitionAdmissibilityClass::AdmissibleViaQueueForPublishLater,
            Some("providers:consequence_preview:deferred"),
            Some("providers:queue_item:issue:245"),
            None,
            None,
            "Deferred comment update queues for publish-later with notification fanout unknown until drain.",
        ),
        transition_packet(
            "work_items:transition_packet:open-in-provider",
            "work_items:detail:provider-authoritative",
            WorkItemMutationMode::OpenInProvider,
            TransitionActionClass::RouteThroughBrowserHandoffOpenInProvider,
            TransitionAdmissibilityClass::AdmissibleViaBrowserHandoffOnly,
            Some("providers:consequence_preview:browser"),
            None,
            Some("browser_handoff:issue-primary:status"),
            None,
            "Browser-handoff status transition keeps the provider mutation outside in-product authority.",
        ),
        transition_packet(
            "work_items:transition_packet:local-draft",
            "work_items:detail:local-draft",
            WorkItemMutationMode::LocalDraft,
            TransitionActionClass::SaveLocalDraftOnlyNoProviderPath,
            TransitionAdmissibilityClass::AdmissibleLocalDraftOnly,
            None,
            None,
            None,
            None,
            "Local draft title update has no provider call or notification fanout.",
        ),
        transition_packet(
            "work_items:transition_packet:offline-captured",
            "work_items:detail:cached-stale",
            WorkItemMutationMode::DeferredPublish,
            TransitionActionClass::CapturedOfflinePendingDrain,
            TransitionAdmissibilityClass::BlockedPendingPrerequisites,
            Some("providers:consequence_preview:offline"),
            Some("providers:queue_item:issue:242"),
            None,
            Some("work_items:offline_handoff:provider-unreachable"),
            "Offline-captured transition preserves intent until connectivity and freshness return.",
        ),
        transition_packet(
            "work_items:transition_packet:policy-blocked",
            "work_items:detail:policy-blocked",
            WorkItemMutationMode::DeferredPublish,
            TransitionActionClass::CapturedOfflinePendingDrain,
            TransitionAdmissibilityClass::BlockedWorkspaceTrustUnsetOrRestricted,
            Some("providers:consequence_preview:policy"),
            None,
            None,
            Some("work_items:offline_handoff:browser-blocked"),
            "Policy-blocked transition captures a packet rather than claiming provider mutation.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn transition_packet(
    packet_id: &str,
    detail_ref: &str,
    mode: WorkItemMutationMode,
    action: TransitionActionClass,
    admissibility: TransitionAdmissibilityClass,
    preview_ref: Option<&str>,
    queue_ref: Option<&str>,
    browser_ref: Option<&str>,
    offline_ref: Option<&str>,
    summary: &str,
) -> StatusTransitionPacketRecord {
    let permission_scope = match action {
        TransitionActionClass::MutateProviderStatePublishNow => {
            PermissionScopeClass::PermissionAdmissibleUserWritesProvider
        }
        TransitionActionClass::QueueForPublishLaterDeferred => {
            PermissionScopeClass::PermissionAdmissibleUnderInstallOrAppGrant
        }
        TransitionActionClass::RouteThroughBrowserHandoffOpenInProvider => {
            PermissionScopeClass::PermissionAdmissibleUnderBrowserHandoffOnly
        }
        TransitionActionClass::SaveLocalDraftOnlyNoProviderPath => {
            PermissionScopeClass::PermissionAdmissibleUnderLocalDraftOnly
        }
        TransitionActionClass::CapturedOfflinePendingDrain => {
            PermissionScopeClass::PermissionBlockedWorkspaceTrustUnsetOrRestricted
        }
        TransitionActionClass::InspectOnlyNoMutation => {
            PermissionScopeClass::PermissionBlockedProviderReadOnlyScope
        }
    };
    let notification = match action {
        TransitionActionClass::SaveLocalDraftOnlyNoProviderPath
        | TransitionActionClass::InspectOnlyNoMutation => {
            NotificationSideEffectClass::NoNotificationLocallyOnly
        }
        TransitionActionClass::QueueForPublishLaterDeferred
        | TransitionActionClass::CapturedOfflinePendingDrain
        | TransitionActionClass::RouteThroughBrowserHandoffOpenInProvider => {
            NotificationSideEffectClass::NotificationUnknownUntilPublishAdmittedPendingReview
        }
        TransitionActionClass::MutateProviderStatePublishNow => {
            NotificationSideEffectClass::NotifyAssigneesOnProviderPublish
        }
    };
    StatusTransitionPacketRecord {
        record_kind: STATUS_TRANSITION_PACKET_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
        packet_id: packet_id.to_string(),
        work_item_detail_record_id_ref: detail_ref.to_string(),
        mutation_mode: mode,
        transition_entries: vec![TransitionEntry {
            transition_entry_id: format!("{packet_id}:entry:01"),
            transition_kind_class: TransitionKindClass::ChangeLifecycleStateToken,
            transition_action_class: action,
            notification_side_effect_class: notification,
            permission_scope_class: permission_scope,
            transition_admissibility_class: admissibility,
            expected_actor_class: ProviderActorClass::HumanAccount,
            previous_state_value: "provider_triaged".to_string(),
            next_state_value: "provider_in_review".to_string(),
            linked_artifact_change_class: LinkedArtifactChangeClass::NoArtifactChange,
            linked_artifact_record_id_ref: None,
            linked_browser_handoff_packet_ref: browser_ref.map(str::to_string),
            linked_publish_later_queue_item_record_id_ref: queue_ref.map(str::to_string),
            linked_provider_consequence_preview_record_id_ref: preview_ref.map(str::to_string),
            linked_account_mapping_binding_record_id_ref: None,
            linked_offline_handoff_packet_record_id_ref: offline_ref.map(str::to_string),
            summary: summary.to_string(),
        }],
        packet_admissibility_class: admissibility,
        linked_publish_later_queue_item_record_id_ref: queue_ref.map(str::to_string),
        linked_browser_handoff_packet_ref: browser_ref.map(str::to_string),
        linked_provider_consequence_preview_record_id_ref: if mode
            == WorkItemMutationMode::LocalDraft
        {
            None
        } else {
            preview_ref.map(str::to_string)
        },
        linked_account_mapping_binding_record_id_ref: None,
        linked_offline_handoff_packet_record_id_ref: offline_ref.map(str::to_string),
        action_affordances: TransitionActionAffordances {
            confirm_action_available: admissibility.is_admissible(),
            export_action_available: true,
            cancel_action_available: true,
        },
        origin_disclosure: origin("exec:transition-packet"),
        policy_context: WorkItemPolicyContext {
            policy_epoch: "policy:epoch:2026-05-18".to_string(),
            trust_state: if matches!(
                admissibility,
                TransitionAdmissibilityClass::BlockedWorkspaceTrustUnsetOrRestricted
            ) {
                TrustPosture::Restricted
            } else {
                TrustPosture::Trusted
            },
            execution_context_id: "exec:transition-packet".to_string(),
            policy_block_ref: matches!(
                admissibility,
                TransitionAdmissibilityClass::BlockedWorkspaceTrustUnsetOrRestricted
            )
            .then(|| "policy:block:work-item-transition".to_string()),
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_provider_payload_refs_present: false,
        authored_at: "2026-05-18T09:05:00Z".to_string(),
        expires_at: "2026-05-18T10:05:00Z".to_string(),
        summary: summary.to_string(),
    }
}

fn seeded_transition_reviews() -> Vec<TransitionReviewSheetRecord> {
    vec![
        transition_review(
            "work_items:transition_review:publish-now",
            "work_items:detail:provider-authoritative",
            "work_items:transition_packet:publish-now",
            TransitionReviewDispositionClass::AdmissibleNowPublishNow,
            WorkItemMutationMode::PublishNow,
            PermissionScopeClass::PermissionAdmissibleUserWritesProvider,
            vec![
                fanout(
                    "fanout:publish-now:provider",
                    SideEffectFanoutKindClass::ProviderMutationFanout,
                    TargetAccountClass::ConnectedProviderAccountResolved,
                    AuthoritySourceClass::ProviderAuthoritativeSource,
                    WorkItemMutationMode::PublishNow,
                    NotificationSideEffectClass::NotifyAssigneesOnProviderPublish,
                    UndoRollbackPostureClass::RollbackAdmissibleViaCompensatingAction,
                    OfflineDeferredHandlingClass::NotOfflineNotDeferred,
                    None,
                ),
                fanout(
                    "fanout:publish-now:notification",
                    SideEffectFanoutKindClass::NotificationEmissionFanout,
                    TargetAccountClass::ConnectedProviderAccountResolved,
                    AuthoritySourceClass::ProviderAuthoritativeSource,
                    WorkItemMutationMode::PublishNow,
                    NotificationSideEffectClass::NotifyAssigneesOnProviderPublish,
                    UndoRollbackPostureClass::RollbackUnknownPendingProviderCallback,
                    OfflineDeferredHandlingClass::NotOfflineNotDeferred,
                    None,
                ),
            ],
            true,
            None,
            None,
        ),
        transition_review(
            "work_items:transition_review:open-in-provider",
            "work_items:detail:provider-authoritative",
            "work_items:transition_packet:open-in-provider",
            TransitionReviewDispositionClass::AdmissibleViaBrowserHandoffOnly,
            WorkItemMutationMode::OpenInProvider,
            PermissionScopeClass::PermissionAdmissibleUnderBrowserHandoffOnly,
            vec![
                fanout(
                    "fanout:open-in-provider:provider",
                    SideEffectFanoutKindClass::ProviderMutationFanout,
                    TargetAccountClass::BrowserHandoffAccountSessionOnly,
                    AuthoritySourceClass::ProviderAuthoritativeSource,
                    WorkItemMutationMode::OpenInProvider,
                    NotificationSideEffectClass::NotificationUnknownUntilPublishAdmittedPendingReview,
                    UndoRollbackPostureClass::RollbackUnknownPendingProviderCallback,
                    OfflineDeferredHandlingClass::DeferredPublishBlockedPendingPrerequisites,
                    Some("providers:browser_handoff:issue:241"),
                ),
                fanout(
                    "fanout:open-in-provider:review",
                    SideEffectFanoutKindClass::LinkedReviewUpdateFanout,
                    TargetAccountClass::LocalOnlyNoProviderAccount,
                    AuthoritySourceClass::LocalAuthoritativeSourceNoProviderOverlay,
                    WorkItemMutationMode::LocalDraft,
                    NotificationSideEffectClass::NoNotificationLocallyOnly,
                    UndoRollbackPostureClass::NoUndoRequiredLocalOnly,
                    OfflineDeferredHandlingClass::NotOfflineNotDeferred,
                    Some("vcs:review_workspace:work-item"),
                ),
            ],
            true,
            None,
            Some("providers:browser_handoff:issue:241"),
        ),
        transition_review(
            "work_items:transition_review:local-draft",
            "work_items:detail:local-draft",
            "work_items:transition_packet:local-draft",
            TransitionReviewDispositionClass::AdmissibleLocalDraftOnly,
            WorkItemMutationMode::LocalDraft,
            PermissionScopeClass::PermissionAdmissibleUnderLocalDraftOnly,
            vec![fanout(
                "fanout:local-draft:metadata",
                SideEffectFanoutKindClass::LocalMetadataChangeFanout,
                TargetAccountClass::LocalOnlyNoProviderAccount,
                AuthoritySourceClass::LocalOnlyNoProviderAuthority,
                WorkItemMutationMode::LocalDraft,
                NotificationSideEffectClass::NoNotificationLocallyOnly,
                UndoRollbackPostureClass::NoUndoRequiredLocalOnly,
                OfflineDeferredHandlingClass::NotOfflineNotDeferred,
                None,
            )],
            true,
            None,
            None,
        ),
        transition_review(
            "work_items:transition_review:deferred",
            "work_items:detail:queued",
            "work_items:transition_packet:deferred",
            TransitionReviewDispositionClass::AdmissibleViaQueueForPublishLater,
            WorkItemMutationMode::DeferredPublish,
            PermissionScopeClass::PermissionAdmissibleUnderInstallOrAppGrant,
            vec![
                fanout(
                    "fanout:deferred:provider",
                    SideEffectFanoutKindClass::ProviderMutationFanout,
                    TargetAccountClass::ConnectedProviderInstallOrAppGrant,
                    AuthoritySourceClass::ProviderOverlayWithLocalOverlaySynced,
                    WorkItemMutationMode::DeferredPublish,
                    NotificationSideEffectClass::NotificationUnknownUntilPublishAdmittedPendingReview,
                    UndoRollbackPostureClass::RollbackAdmissibleViaRevokeBeforeDrain,
                    OfflineDeferredHandlingClass::DeferredPublishAdmittedToQueue,
                    Some("providers:queue_item:issue:245"),
                ),
                fanout(
                    "fanout:deferred:review",
                    SideEffectFanoutKindClass::LinkedReviewUpdateFanout,
                    TargetAccountClass::LocalOnlyNoProviderAccount,
                    AuthoritySourceClass::LocalAuthoritativeSourceNoProviderOverlay,
                    WorkItemMutationMode::LocalDraft,
                    NotificationSideEffectClass::NoNotificationLocallyOnly,
                    UndoRollbackPostureClass::NoUndoRequiredLocalOnly,
                    OfflineDeferredHandlingClass::NotOfflineNotDeferred,
                    Some("vcs:review_workspace:work-item"),
                ),
                fanout(
                    "fanout:deferred:automation",
                    SideEffectFanoutKindClass::QueuedFollowonAutomationFanout,
                    TargetAccountClass::ConnectedProviderInstallOrAppGrant,
                    AuthoritySourceClass::ProviderOverlayWithLocalOverlaySynced,
                    WorkItemMutationMode::DeferredPublish,
                    NotificationSideEffectClass::NotificationUnknownUntilPublishAdmittedPendingReview,
                    UndoRollbackPostureClass::RollbackAdmissibleViaRevokeBeforeDrain,
                    OfflineDeferredHandlingClass::DeferredPublishAdmittedToQueue,
                    Some("automation:followon:issue:245"),
                ),
            ],
            true,
            Some("providers:queue_item:issue:245"),
            None,
        ),
        transition_review(
            "work_items:transition_review:policy-blocked",
            "work_items:detail:policy-blocked",
            "work_items:transition_packet:policy-blocked",
            TransitionReviewDispositionClass::BlockedByPolicy,
            WorkItemMutationMode::DeferredPublish,
            PermissionScopeClass::PermissionBlockedWorkspaceTrustUnsetOrRestricted,
            vec![fanout(
                "fanout:policy:provider-blocked",
                SideEffectFanoutKindClass::ProviderMutationFanout,
                TargetAccountClass::AccountMappingBindingPendingUserResolution,
                AuthoritySourceClass::PolicyAdminAuthoritySource,
                WorkItemMutationMode::DeferredPublish,
                NotificationSideEffectClass::NotificationBlockedPendingWorkspaceTrust,
                UndoRollbackPostureClass::RollbackAdmissibleViaRevokeBeforeDrain,
                OfflineDeferredHandlingClass::DeferredPublishBlockedPendingPrerequisites,
                Some("policy:block:work-item-transition"),
            )],
            false,
            None,
            None,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn transition_review(
    review_id: &str,
    detail_ref: &str,
    packet_ref: &str,
    disposition: TransitionReviewDispositionClass,
    mode: WorkItemMutationMode,
    permission_scope: PermissionScopeClass,
    side_effect_fanout_rows: Vec<SideEffectFanoutRow>,
    confirm_available: bool,
    queue_ref: Option<&str>,
    browser_ref: Option<&str>,
) -> TransitionReviewSheetRecord {
    TransitionReviewSheetRecord {
        record_kind: TRANSITION_REVIEW_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
        review_id: review_id.to_string(),
        work_item_detail_record_id_ref: detail_ref.to_string(),
        linked_status_transition_packet_record_id_ref: packet_ref.to_string(),
        transition_trigger_class: TransitionTriggerClass::StatusChangeTrigger,
        transition_review_authorization_class: if disposition.is_admissible() {
            TransitionReviewAuthorizationClass::HumanActorSelfAuthoredAdmissible
        } else {
            TransitionReviewAuthorizationClass::HumanActorPendingRequiredApprovals
        },
        transition_review_disposition_class: disposition,
        target_item_ref: detail_ref.to_string(),
        provider_label: "Issue tracker".to_string(),
        current_state_summary: "Current lifecycle token: provider_triaged.".to_string(),
        requested_state_summary: "Requested lifecycle token: provider_in_review.".to_string(),
        actor_authority_summary: "Aureline acts as a human account or explicit queue grant.".to_string(),
        publish_mode_class: mode,
        permission_scope_class: permission_scope,
        side_effect_fanout_rows,
        action_affordances: TransitionActionAffordances {
            confirm_action_available: confirm_available,
            export_action_available: true,
            cancel_action_available: true,
        },
        local_draft_preserved_on_failure: true,
        linked_publish_later_queue_item_record_id_ref: queue_ref.map(str::to_string),
        linked_browser_handoff_packet_ref: browser_ref.map(str::to_string),
        linked_offline_handoff_packet_record_id_ref: None,
        block_reason_summary: (!disposition.is_admissible())
            .then(|| "Policy or trust prerequisites must clear before provider mutation.".to_string()),
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_provider_payload_refs_present: false,
        authored_at: "2026-05-18T09:06:00Z".to_string(),
        expires_at: "2026-05-18T10:06:00Z".to_string(),
        summary: "Transition review sheet discloses target state, side effects, permission scope, queue/local posture, and confirm/export/cancel actions.".to_string(),
    }
}

#[allow(clippy::too_many_arguments)]
fn fanout(
    fanout_row_id: &str,
    fanout_kind_class: SideEffectFanoutKindClass,
    target_account_class: TargetAccountClass,
    authority_source_class: AuthoritySourceClass,
    publish_mode_class: WorkItemMutationMode,
    notification_side_effect_class: NotificationSideEffectClass,
    undo_rollback_posture_class: UndoRollbackPostureClass,
    offline_deferred_handling_class: OfflineDeferredHandlingClass,
    linked_record_id_ref: Option<&str>,
) -> SideEffectFanoutRow {
    SideEffectFanoutRow {
        fanout_row_id: fanout_row_id.to_string(),
        fanout_kind_class,
        target_account_class,
        authority_source_class,
        publish_mode_class,
        notification_side_effect_class,
        undo_rollback_posture_class,
        offline_deferred_handling_class,
        linked_record_id_ref: linked_record_id_ref.map(str::to_string),
        summary: "Side-effect row names provider/local/review/notification/automation consequence before apply.".to_string(),
    }
}

fn seeded_offline_handoff_packets() -> Vec<OfflineHandoffPacketRecord> {
    vec![
        offline_packet(
            "work_items:offline_handoff:provider-unreachable",
            "work_items:detail:cached-stale",
            "work_items:transition_packet:offline-captured",
            HandoffAdmissionReasonClass::ProviderUnreachableOfflineCapture,
            HandoffProviderAcceptanceClass::NotSubmittedLocalCaptureOnly,
            HandoffDrainStateClass::CapturedPendingDrain,
            vec![
                HandoffExportRouteClass::SupportBundleAttachmentByReference,
                HandoffExportRouteClass::ClipboardOrTextExportUserInitiated,
            ],
            vec![
                HandoffRetryRouteClass::AutoRetryOnConnectivityRestored,
                HandoffRetryRouteClass::ManualRetryUserMustConfirmOnly,
            ],
            Some("providers:queue_item:issue:242"),
            vec![],
            None,
        ),
        offline_packet(
            "work_items:offline_handoff:browser-blocked",
            "work_items:detail:policy-blocked",
            "work_items:transition_packet:policy-blocked",
            HandoffAdmissionReasonClass::BrowserHandoffBlockedManagedWorkstationNoSystemBrowser,
            HandoffProviderAcceptanceClass::NotSubmittedLocalCaptureOnly,
            HandoffDrainStateClass::CapturedPendingExportUserInitiated,
            vec![
                HandoffExportRouteClass::CliExportCommandUserInitiated,
                HandoffExportRouteClass::ExternalHandoffExportToManagedAdminOnly,
            ],
            vec![
                HandoffRetryRouteClass::AutoRetryOnBrowserAvailable,
                HandoffRetryRouteClass::ManualRetryUserMustConfirmOnly,
            ],
            None,
            vec![],
            None,
        ),
        offline_packet(
            "work_items:offline_handoff:drained-accepted",
            "work_items:detail:cached-stale",
            "work_items:transition_packet:offline-captured",
            HandoffAdmissionReasonClass::ProviderUnreachableOfflineCapture,
            HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained,
            HandoffDrainStateClass::DrainedPublishLaterCompleted,
            vec![HandoffExportRouteClass::SupportBundleAttachmentByReference],
            vec![HandoffRetryRouteClass::ManualRetryUserMustConfirmOnly],
            Some("providers:queue_item:issue:242"),
            vec!["providers:callback_envelope:issue:242:accepted".to_string()],
            None,
        ),
        offline_packet(
            "work_items:offline_handoff:drained-rejected",
            "work_items:detail:cached-stale",
            "work_items:transition_packet:offline-captured",
            HandoffAdmissionReasonClass::ProviderUnreachableOfflineCapture,
            HandoffProviderAcceptanceClass::ProviderAcceptRejectedWithTypedReason,
            HandoffDrainStateClass::DrainedPublishLaterRejectedWithTypedReason,
            vec![HandoffExportRouteClass::SupportBundleAttachmentByReference],
            vec![HandoffRetryRouteClass::ManualRetryUserMustConfirmOnly],
            Some("providers:queue_item:issue:242"),
            vec!["providers:callback_envelope:issue:242:rejected".to_string()],
            Some("provider_rejected_due_to_conflicting_update"),
        ),
        offline_packet(
            "work_items:offline_handoff:imported-evidence",
            "work_items:detail:offline-captured",
            "work_items:transition_packet:offline-captured",
            HandoffAdmissionReasonClass::ImportedFromSupportExportNoLiveProviderPath,
            HandoffProviderAcceptanceClass::ImportedHandoffEvidenceOnlyNoProviderPath,
            HandoffDrainStateClass::ExportedPendingExternalApply,
            vec![HandoffExportRouteClass::LocalOnlyNoExportPath],
            vec![HandoffRetryRouteClass::NoRetryImportedEvidenceOnly],
            None,
            vec![],
            None,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn offline_packet(
    packet_id: &str,
    detail_ref: &str,
    transition_ref: &str,
    admission: HandoffAdmissionReasonClass,
    acceptance: HandoffProviderAcceptanceClass,
    drain_state: HandoffDrainStateClass,
    export_routes: Vec<HandoffExportRouteClass>,
    retry_routes: Vec<HandoffRetryRouteClass>,
    queue_ref: Option<&str>,
    callback_refs: Vec<String>,
    rejection_reason: Option<&str>,
) -> OfflineHandoffPacketRecord {
    OfflineHandoffPacketRecord {
        record_kind: OFFLINE_HANDOFF_PACKET_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_TRANSITION_BETA_SCHEMA_VERSION,
        packet_id: packet_id.to_string(),
        work_item_detail_record_id_ref: detail_ref.to_string(),
        linked_status_transition_packet_record_id_ref: transition_ref.to_string(),
        handoff_admission_reason_class: admission,
        handoff_provider_acceptance_class: acceptance,
        handoff_export_route_classes: export_routes,
        handoff_retry_route_classes: retry_routes,
        handoff_drain_state_class: drain_state,
        snapshot_state_rows: vec![SnapshotStateRow {
            state_family_class: StateFamilyClass::LifecycleState,
            state_value: "provider_triaged".to_string(),
            source_schema_ref: "schemas/work_items/work_item_detail.schema.json".to_string(),
            source_field: "current_state_rows.state_value".to_string(),
            source_ref: detail_ref.to_string(),
            summary: "Lifecycle token snapshotted when the handoff packet was captured.".to_string(),
        }],
        snapshot_owner_or_assignee_rows: vec![SnapshotOwnerRow {
            owner_role_class: OwnerRoleClass::Assignee,
            actor_subject_ref: "actor:human:opaque:assignee".to_string(),
            actor_class: ProviderActorClass::HumanAccount,
            actor_label: "Reviewable assignee label".to_string(),
        }],
        snapshot_engineering_relations: vec![
            snapshot_relation(
                SnapshotRelationAxisClass::IssueToBranchLink,
                "linked_provider_branch_overlay_fetched",
                "workspace:branch:work-item",
            ),
            snapshot_relation(
                SnapshotRelationAxisClass::LinkedReview,
                "linked_review_workspace_with_provider_overlay",
                "vcs:review_workspace:work-item",
            ),
            snapshot_relation(
                SnapshotRelationAxisClass::ChangeIntent,
                "change_object_provider_authoritative",
                "vcs:change_object:work-item",
            ),
            snapshot_relation(
                SnapshotRelationAxisClass::ValidationEvidence,
                "linked_review_evaluation_result",
                "vcs:review_evaluation_result:work-item",
            ),
            snapshot_relation(
                SnapshotRelationAxisClass::PublishPreview,
                "publish_preview_pinned_provider_consequence_preview_record",
                "providers:consequence_preview:work-item",
            ),
        ],
        linked_publish_later_queue_item_record_id_ref: queue_ref.map(str::to_string),
        linked_provider_consequence_preview_record_id_ref: Some(
            "providers:consequence_preview:offline".to_string(),
        ),
        linked_account_mapping_binding_record_id_ref: None,
        linked_browser_handoff_packet_ref: None,
        linked_provider_callback_envelope_record_id_refs: callback_refs,
        linked_support_bundle_record_id_ref: Some("support:bundle:work-item-offline".to_string()),
        linked_incident_workspace_packet_record_id_ref: None,
        linked_object_handoff_packet_record_id_ref: None,
        supersedes_offline_handoff_packet_record_id_ref: None,
        superseded_by_offline_handoff_packet_record_id_ref: None,
        acceptance_rejection_reason: rejection_reason.map(str::to_string),
        captured_freshness_floor_ref: "providers:freshness_floor:issue-primary".to_string(),
        redaction_manifest_ref: "redaction:manifest:work-item-offline".to_string(),
        publish_target_ref: detail_ref.to_string(),
        retry_action_ref: format!("{packet_id}:retry"),
        export_action_ref: format!("{packet_id}:export"),
        packet_survives_restart: true,
        origin_disclosure: origin("exec:offline-handoff"),
        policy_context: WorkItemPolicyContext {
            policy_epoch: "policy:epoch:2026-05-18".to_string(),
            trust_state: TrustPosture::Trusted,
            execution_context_id: "exec:offline-handoff".to_string(),
            policy_block_ref: None,
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        raw_provider_payload_refs_present: false,
        captured_at: "2026-05-18T09:10:00Z".to_string(),
        expires_at: "2026-05-19T09:10:00Z".to_string(),
        summary: "Offline handoff packet preserves intended transition, evidence refs, redaction manifest, publish target, retry action, and export action without implying provider acceptance.".to_string(),
    }
}

fn snapshot_relation(
    relation_axis_class: SnapshotRelationAxisClass,
    relation_class_value: &str,
    linked_record_id_ref: &str,
) -> SnapshotEngineeringRelation {
    SnapshotEngineeringRelation {
        relation_axis_class,
        relation_class_value: relation_class_value.to_string(),
        linked_record_id_ref: linked_record_id_ref.to_string(),
        summary: "Engineering relation snapshotted by reference.".to_string(),
    }
}

fn seeded_workflow_corpus_cases() -> Vec<ProviderWorkflowCorpusCase> {
    vec![
        corpus_case(
            "provider_workflow:case:stale-target",
            ProviderWorkflowCorpusClass::StaleTargetId,
            "work_items:detail:cached-stale",
            Some("work_items:transition_packet:offline-captured"),
            Some("work_items:offline_handoff:provider-unreachable"),
            WorkItemRowPostureClass::CachedStale,
        ),
        corpus_case(
            "provider_workflow:case:revoked-credentials",
            ProviderWorkflowCorpusClass::RevokedCredentials,
            "work_items:detail:policy-blocked",
            Some("work_items:transition_packet:policy-blocked"),
            Some("work_items:offline_handoff:browser-blocked"),
            WorkItemRowPostureClass::PolicyBlocked,
        ),
        corpus_case(
            "provider_workflow:case:conflicting-update",
            ProviderWorkflowCorpusClass::ConflictingUpdates,
            "work_items:detail:cached-stale",
            Some("work_items:transition_packet:offline-captured"),
            Some("work_items:offline_handoff:drained-rejected"),
            WorkItemRowPostureClass::CachedStale,
        ),
        corpus_case(
            "provider_workflow:case:read-only-session",
            ProviderWorkflowCorpusClass::ReadOnlySession,
            "work_items:detail:read-only",
            None,
            None,
            WorkItemRowPostureClass::ReadOnly,
        ),
        corpus_case(
            "provider_workflow:case:offline-capture",
            ProviderWorkflowCorpusClass::OfflineCapture,
            "work_items:detail:offline-captured",
            Some("work_items:transition_packet:offline-captured"),
            Some("work_items:offline_handoff:imported-evidence"),
            WorkItemRowPostureClass::OfflineCaptured,
        ),
        corpus_case(
            "provider_workflow:case:publish-later-replay",
            ProviderWorkflowCorpusClass::PublishLaterReplay,
            "work_items:detail:queued",
            Some("work_items:transition_packet:deferred"),
            Some("work_items:offline_handoff:drained-accepted"),
            WorkItemRowPostureClass::Queued,
        ),
    ]
}

fn corpus_case(
    case_id: &str,
    corpus_class: ProviderWorkflowCorpusClass,
    detail_ref: &str,
    transition_ref: Option<&str>,
    offline_ref: Option<&str>,
    posture: WorkItemRowPostureClass,
) -> ProviderWorkflowCorpusCase {
    ProviderWorkflowCorpusCase {
        record_kind: PROVIDER_WORKFLOW_CORPUS_CASE_RECORD_KIND.to_string(),
        case_id: case_id.to_string(),
        corpus_class,
        work_item_detail_record_id_ref: detail_ref.to_string(),
        status_transition_packet_record_id_ref: transition_ref.map(str::to_string),
        offline_handoff_packet_record_id_ref: offline_ref.map(str::to_string),
        expected_visible_posture: posture,
        support_export_summary: "Provider workflow corpus case preserves target, actor, local/queued/offline posture, and safe follow-up path.".to_string(),
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
