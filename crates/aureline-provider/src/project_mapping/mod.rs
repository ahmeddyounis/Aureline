//! Board/project/space/repository mapping-review truth for provider-backed
//! issue, review, incident, and publish-later lanes.
//!
//! The account-scope beta ([`crate::account_scope`]) made provider *authority*
//! honest: who Aureline acts as, under which grant, and with what effective
//! write scope. This module makes provider *targeting* honest. Before Aureline
//! proposes or executes a hosted mutation it MUST be able to answer three
//! questions on every claimed beta provider lane:
//!
//! 1. **Who is acting?** Each mapping-review row embeds one
//!    [`AccountSessionBinding`] — the governed account/session object that
//!    names the acting-identity class, the bound account-scope identity row,
//!    the auth source, whether the account was policy-selected, the
//!    first-class provider session state, and the effective write scope.
//! 2. **What target will be touched?** Each row carries a typed
//!    [`MappingResolutionStateClass`] and, when one resolves, a
//!    [`MappingTargetDescriptor`] naming the board, project, space,
//!    repository, or incident queue the comment, issue update, review action,
//!    or incident handoff will land on. Policy-locked and unsupported-remap
//!    cases are first-class, never collapsed into a disabled control.
//! 3. **Is the next action local, queued, or live?** Each row carries a typed
//!    [`PublishPostureClass`] distinguishing a local draft, a queued
//!    publish-later item, a live provider mutation, and a read-only
//!    inspection, with the concrete [`MappingNextActionClass`] that moves the
//!    row forward.
//!
//! When a mapping becomes invalid — a target is archived, a session goes
//! offline, a credential goes stale, or a provider refuses a remap — a
//! [`MappingInvalidationEvent`] records the drop. The drop preserves local
//! drafts, queued transitions, and evidence attachments, and never silently
//! keeps a live mutation posture.
//!
//! The page-level [`TargetMappingBetaPage`] folds review rows and invalidation
//! events into one validator-checked projection over connected, mirror,
//! offline, and enterprise-managed beta profiles across all four lanes.
//! [`TargetMappingBetaSupportExport`] wraps the page in a redaction-safe
//! envelope: raw access tokens, raw provider payloads, and raw provider URLs
//! are excluded; identity, target, posture, and invalidation lineage are
//! preserved verbatim so support and reviewer surfaces can name exactly which
//! account acted and which target a mutation would touch.
//!
//! Reviewer-facing landing page:
//! [`/docs/providers/m3/provider_account_and_mapping_truth.md`](../../../../docs/providers/m3/provider_account_and_mapping_truth.md).
//! The mapping boundary vocabulary lives at
//! [`/schemas/providers/provider_target_mapping.schema.json`](../../../../schemas/providers/provider_target_mapping.schema.json)
//! and the embedded account/session object at
//! [`/schemas/providers/provider_account_scope.schema.json`](../../../../schemas/providers/provider_account_scope.schema.json).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::account_scope::{
    AccountAuthSourceClass, AccountScopeBetaProfileClass, ActingIdentityClass,
};

/// Beta schema version exported with every target-mapping beta record.
pub const TARGET_MAPPING_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every target-mapping beta record.
pub const TARGET_MAPPING_BETA_SHARED_CONTRACT_REF: &str = "providers:target_mapping_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const TARGET_MAPPING_BETA_SOURCE_MATRIX_REF: &str =
    "fixtures/providers/m3/account_scope_and_mapping/corpus_matrix.json";

/// Reviewer-facing doc ref.
pub const TARGET_MAPPING_BETA_DOC_REF: &str =
    "docs/providers/m3/provider_account_and_mapping_truth.md";

/// Mapping-page schema ref.
pub const TARGET_MAPPING_BETA_SCHEMA_REF: &str =
    "schemas/providers/provider_target_mapping.schema.json";

/// Embedded account/session object schema ref.
pub const TARGET_MAPPING_BETA_ACCOUNT_SCOPE_SCHEMA_REF: &str =
    "schemas/providers/provider_account_scope.schema.json";

/// Fixture directory for this beta projection.
pub const TARGET_MAPPING_BETA_FIXTURE_DIR: &str = "fixtures/providers/m3/account_scope_and_mapping";

/// Stable record kind for the embedded account/session binding.
pub const PROVIDER_ACCOUNT_SESSION_BINDING_RECORD_KIND: &str =
    "providers_account_session_binding_record";

/// Stable record kind for [`TargetMappingBetaPage`] payloads.
pub const TARGET_MAPPING_BETA_PAGE_RECORD_KIND: &str = "providers_target_mapping_beta_page_record";

/// Stable record kind for [`MappingReviewRow`] payloads.
pub const TARGET_MAPPING_BETA_ROW_RECORD_KIND: &str =
    "providers_target_mapping_beta_review_row_record";

/// Stable record kind for [`MappingInvalidationEvent`] payloads.
pub const TARGET_MAPPING_BETA_INVALIDATION_EVENT_RECORD_KIND: &str =
    "providers_target_mapping_beta_invalidation_event_record";

/// Stable record kind for [`TargetMappingBetaSummary`] payloads.
pub const TARGET_MAPPING_BETA_SUMMARY_RECORD_KIND: &str =
    "providers_target_mapping_beta_summary_record";

/// Stable record kind for [`TargetMappingBetaDefect`] payloads.
pub const TARGET_MAPPING_BETA_DEFECT_RECORD_KIND: &str =
    "providers_target_mapping_beta_defect_record";

/// Stable record kind for [`TargetMappingBetaSupportExport`] payloads.
pub const TARGET_MAPPING_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_target_mapping_beta_support_export_record";

/// Provider lane a mapping-review row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingLaneClass {
    /// Issue / work-item mutation lane (create, update, comment, transition).
    IssueOrWorkItem,
    /// Review-decision lane (approve, request-changes, merge, close).
    ReviewDecision,
    /// Incident handoff lane (open, escalate, attach evidence, hand off).
    IncidentHandoff,
    /// Publish-later lane (queued drafts awaiting a provider mutation).
    PublishLater,
}

impl MappingLaneClass {
    /// All four lanes in canonical order.
    pub const ALL: [Self; 4] = [
        Self::IssueOrWorkItem,
        Self::ReviewDecision,
        Self::IncidentHandoff,
        Self::PublishLater,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssueOrWorkItem => "issue_or_work_item",
            Self::ReviewDecision => "review_decision",
            Self::IncidentHandoff => "incident_handoff",
            Self::PublishLater => "publish_later",
        }
    }
}

/// Class of action a mapping-review row adjudicates against its target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingActionClass {
    /// Publish a human-authored comment.
    Comment,
    /// Create a new issue or work item.
    IssueCreate,
    /// Update fields on an existing issue or work item.
    IssueUpdate,
    /// Publish a status transition.
    StatusTransition,
    /// Publish a review decision.
    ReviewDecision,
    /// Hand off an incident to a provider queue.
    IncidentHandoff,
    /// Attach evidence to a provider object.
    EvidenceAttachment,
}

impl MappingActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comment => "comment",
            Self::IssueCreate => "issue_create",
            Self::IssueUpdate => "issue_update",
            Self::StatusTransition => "status_transition",
            Self::ReviewDecision => "review_decision",
            Self::IncidentHandoff => "incident_handoff",
            Self::EvidenceAttachment => "evidence_attachment",
        }
    }
}

/// Kind of target a mapping resolves to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKindClass {
    /// A board (column-organized work surface).
    Board,
    /// A project (issue/work-item container).
    Project,
    /// A space (documentation or knowledge container).
    Space,
    /// A source repository.
    Repository,
    /// An incident queue or rotation.
    IncidentQueue,
}

impl TargetKindClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Board => "board",
            Self::Project => "project",
            Self::Space => "space",
            Self::Repository => "repository",
            Self::IncidentQueue => "incident_queue",
        }
    }
}

/// Resolution state of a mapping-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingResolutionStateClass {
    /// Exactly one target resolved; safe to act if posture and session allow.
    ResolvedSingleTarget,
    /// Target is locked by managed policy; the row cannot be remapped here.
    PolicyLockedTarget,
    /// Multiple candidate targets; a selection is required before acting.
    AmbiguousNeedsSelection,
    /// The provider does not support remapping this object's target.
    UnsupportedRemap,
    /// The cached mapping is stale and must be refreshed before acting.
    StaleNeedsRefresh,
    /// The previously resolved target has been invalidated.
    Invalidated,
    /// No mapping is configured for this lane yet.
    NoMappingConfigured,
}

impl MappingResolutionStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedSingleTarget => "resolved_single_target",
            Self::PolicyLockedTarget => "policy_locked_target",
            Self::AmbiguousNeedsSelection => "ambiguous_needs_selection",
            Self::UnsupportedRemap => "unsupported_remap",
            Self::StaleNeedsRefresh => "stale_needs_refresh",
            Self::Invalidated => "invalidated",
            Self::NoMappingConfigured => "no_mapping_configured",
        }
    }

    /// True when this state admits a live provider mutation (a single
    /// authoritative target is bound).
    pub const fn admits_live_mutation(self) -> bool {
        matches!(self, Self::ResolvedSingleTarget | Self::PolicyLockedTarget)
    }
}

/// Publish posture: whether the next action is a local draft, a queued
/// publish-later item, a live provider mutation, or read-only inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishPostureClass {
    /// The action is held as a local draft and has not been queued.
    LocalDraft,
    /// The action is queued for publish-later and will publish on reconnect.
    QueuedPublishLater,
    /// The action will execute as a live provider mutation now.
    LiveProviderMutation,
    /// The row is read-only; no mutation will be proposed.
    ReadOnlyInspection,
}

impl PublishPostureClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraft => "local_draft",
            Self::QueuedPublishLater => "queued_publish_later",
            Self::LiveProviderMutation => "live_provider_mutation",
            Self::ReadOnlyInspection => "read_only_inspection",
        }
    }

    /// True when the posture executes a live provider mutation.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveProviderMutation)
    }

    /// True when the posture parks the action locally or in the queue.
    pub const fn is_deferred(self) -> bool {
        matches!(self, Self::LocalDraft | Self::QueuedPublishLater)
    }
}

/// First-class provider session state observed for the acting identity.
///
/// These states are preserved as distinct lanes rather than collapsed into a
/// generic "unavailable" message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSessionStateClass {
    /// Live session within freshness floor and full effective scope.
    Live,
    /// Session reachable but scope is narrower than the action requires.
    LimitedScope,
    /// Credential is stale and must be refreshed before mutating.
    StaleCredential,
    /// Session is read-only (mirror, viewer grant).
    ReadOnly,
    /// Working from imported snapshots; capture is offline.
    OfflineCapture,
    /// Session can only stage publish-later items, not live mutations.
    PublishLaterOnly,
}

impl ProviderSessionStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::LimitedScope => "limited_scope",
            Self::StaleCredential => "stale_credential",
            Self::ReadOnly => "read_only",
            Self::OfflineCapture => "offline_capture",
            Self::PublishLaterOnly => "publish_later_only",
        }
    }

    /// True when the session can carry a live provider mutation.
    pub const fn admits_live_mutation(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Concrete next-safe action a row offers instead of a silent disabled control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingNextActionClass {
    /// Nothing else is required; the row is ready to act.
    NoneProceed,
    /// Select a target from the candidate set.
    SelectTarget,
    /// Refresh the cached mapping before acting.
    RefreshMapping,
    /// Reconnect the provider session before acting.
    ReconnectSession,
    /// Reapprove the narrowed scope before acting.
    ReapproveScope,
    /// Route the action through a system-browser handoff.
    RouteBrowserHandoff,
    /// Queue the action into publish-later.
    QueuePublishLater,
    /// Contact a workspace admin (policy-locked / suspended).
    ContactAdmin,
    /// Retry or export the queued action.
    RetryExport,
}

impl MappingNextActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneProceed => "none_proceed",
            Self::SelectTarget => "select_target",
            Self::RefreshMapping => "refresh_mapping",
            Self::ReconnectSession => "reconnect_session",
            Self::ReapproveScope => "reapprove_scope",
            Self::RouteBrowserHandoff => "route_browser_handoff",
            Self::QueuePublishLater => "queue_publish_later",
            Self::ContactAdmin => "contact_admin",
            Self::RetryExport => "retry_export",
        }
    }
}

/// Trigger that invalidated a previously resolved mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingInvalidationTriggerClass {
    /// Target was archived at the provider.
    TargetArchived,
    /// Target was deleted at the provider.
    TargetDeleted,
    /// Target was moved or renamed at the provider.
    TargetMovedOrRenamed,
    /// Managed policy locked the row to a different target.
    PolicyRemapLocked,
    /// Provider-declared scope narrowed below the target.
    ScopeNarrowedBelowTarget,
    /// The provider session went offline.
    SessionWentOffline,
    /// The acting credential went stale.
    CredentialWentStale,
    /// The provider refused to remap the object.
    RemapUnsupportedByProvider,
    /// The board/project schema changed under the cached mapping.
    BoardSchemaChanged,
}

impl MappingInvalidationTriggerClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetArchived => "target_archived",
            Self::TargetDeleted => "target_deleted",
            Self::TargetMovedOrRenamed => "target_moved_or_renamed",
            Self::PolicyRemapLocked => "policy_remap_locked",
            Self::ScopeNarrowedBelowTarget => "scope_narrowed_below_target",
            Self::SessionWentOffline => "session_went_offline",
            Self::CredentialWentStale => "credential_went_stale",
            Self::RemapUnsupportedByProvider => "remap_unsupported_by_provider",
            Self::BoardSchemaChanged => "board_schema_changed",
        }
    }
}

/// Governed account/session object embedded on every mapping-review row.
///
/// This object names who Aureline acts as for the row's action. It is the
/// schema-of-record subject for
/// [`/schemas/providers/provider_account_scope.schema.json`]. It never carries
/// raw access tokens, raw delegated-token bodies, or raw provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountSessionBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Acting identity class.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::acting_identity_class`].
    pub acting_identity_class_token: String,
    /// Account-scope identity row ref this binding resolves to.
    pub bound_identity_row_ref: String,
    /// Reviewable account label safe for UI and support export.
    pub account_label: String,
    /// Auth source backing the acting identity.
    pub auth_source: AccountAuthSourceClass,
    /// Stable token for [`Self::auth_source`].
    pub auth_source_token: String,
    /// First-class provider session state.
    pub session_state: ProviderSessionStateClass,
    /// Stable token for [`Self::session_state`].
    pub session_state_token: String,
    /// Export-safe explanation of the session state.
    pub session_state_note: String,
    /// Effective write-scope refs the identity currently holds. Empty when the
    /// session is read-only or offline.
    pub effective_write_scope_refs: Vec<String>,
    /// True when the account was selected by policy rather than by the user.
    pub policy_selected_account: bool,
    /// Beta guardrail: raw access-token material is not present.
    pub raw_token_material_present: bool,
}

/// The board/project/space/repository/incident-queue a mapping resolves to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingTargetDescriptor {
    /// Kind of target.
    pub target_kind: TargetKindClass,
    /// Stable token for [`Self::target_kind`].
    pub target_kind_token: String,
    /// Opaque canonical target ref.
    pub canonical_target_ref: String,
    /// Reviewable target label safe for UI and support export.
    pub target_label: String,
    /// Reviewable container path label (e.g. "Payments org › Backend board").
    pub container_path_label: String,
    /// Reviewable provider-host label.
    pub provider_host_label: String,
}

/// One mapping-review row for a provider-backed action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingReviewRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque row id.
    pub row_id: String,
    /// Reviewable row label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Lane the row belongs to.
    pub lane: MappingLaneClass,
    /// Stable token for [`Self::lane`].
    pub lane_token: String,
    /// Action class the row adjudicates.
    pub action: MappingActionClass,
    /// Stable token for [`Self::action`].
    pub action_token: String,
    /// Governed account/session binding (who Aureline acts as).
    pub account_session: AccountSessionBinding,
    /// Opaque ref to the provider-linked source object the action originates
    /// from.
    pub provider_linked_row_ref: String,
    /// Mapping resolution state.
    pub resolution_state: MappingResolutionStateClass,
    /// Stable token for [`Self::resolution_state`].
    pub resolution_state_token: String,
    /// Resolved target. Present for resolved/policy-locked rows and for
    /// invalidated/unsupported/stale rows that still show the last target;
    /// absent for ambiguous and unconfigured rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_target: Option<MappingTargetDescriptor>,
    /// Candidate target refs (populated only for ambiguous rows).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidate_target_refs: Vec<String>,
    /// True when the row's target is locked by managed policy.
    pub policy_locked_target: bool,
    /// Publish posture (local draft / queued / live / read-only).
    pub publish_posture: PublishPostureClass,
    /// Stable token for [`Self::publish_posture`].
    pub publish_posture_token: String,
    /// Export-safe explanation of the publish posture.
    pub publish_posture_note: String,
    /// Concrete next-safe action.
    pub next_action: MappingNextActionClass,
    /// Stable token for [`Self::next_action`].
    pub next_action_token: String,
    /// Reviewable next-action label.
    pub next_action_label: String,
    /// Optional opaque remediation-path ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation_path_ref: Option<String>,
    /// Optional opaque publish-later queue item ref for deferred rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_item_ref: Option<String>,
    /// Opaque evidence-attachment refs kept durable with the row.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_attachment_refs: Vec<String>,
    /// Timestamp at which the row was computed.
    pub computed_at: String,
    /// Beta guardrail: raw access-token material is not present.
    pub raw_token_material_present: bool,
    /// Beta guardrail: the row did not silently remap to a different target.
    pub silent_target_remap_taken: bool,
    /// Beta guardrail: the row did not silently widen to a live mutation.
    pub silent_live_mutation_widened: bool,
    /// Beta guardrail: local drafts are preserved through this row.
    pub local_draft_preserved: bool,
    /// Beta guardrail: queued transitions are preserved through this row.
    pub queued_transitions_preserved: bool,
    /// Beta guardrail: evidence attachments are preserved through this row.
    pub evidence_preserved: bool,
    /// Beta guardrail: retry/export remains available for deferred rows.
    pub retry_export_available: bool,
}

/// One mapping-invalidation event that dropped a previously resolved mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingInvalidationEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque event id.
    pub event_id: String,
    /// Profile under which the event is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Row ref affected by the invalidation.
    pub affected_row_ref: String,
    /// Opaque ref to the previously resolved target.
    pub previous_target_ref: String,
    /// Trigger that invalidated the mapping.
    pub trigger: MappingInvalidationTriggerClass,
    /// Stable token for [`Self::trigger`].
    pub trigger_token: String,
    /// Posture the row drops to after invalidation. Never live.
    pub forced_posture: PublishPostureClass,
    /// Stable token for [`Self::forced_posture`].
    pub forced_posture_token: String,
    /// Concrete next-safe action after the invalidation.
    pub next_action: MappingNextActionClass,
    /// Stable token for [`Self::next_action`].
    pub next_action_token: String,
    /// Export-safe rationale for the invalidation.
    pub rationale_summary: String,
    /// Timestamp at which the invalidation was observed.
    pub observed_at: String,
    /// Beta guardrail: invalidation did not silently keep a live mutation.
    pub silent_live_mutation_after_invalidation: bool,
    /// Beta guardrail: local drafts are preserved through the invalidation.
    pub local_draft_preserved: bool,
    /// Beta guardrail: queued transitions are preserved through the
    /// invalidation.
    pub queued_transitions_preserved: bool,
    /// Beta guardrail: evidence attachments are preserved through the
    /// invalidation.
    pub evidence_preserved: bool,
}

/// Defect-kind vocabulary surfaced by the beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetMappingBetaDefectKind {
    /// A record claims raw token material is present.
    RawTokenMaterialPresent,
    /// The row silently remapped to a different target.
    SilentTargetRemapTaken,
    /// The row silently widened to a live mutation posture.
    SilentLiveMutationWidened,
    /// Local drafts were not preserved.
    LocalDraftNotPreserved,
    /// Queued transitions were not preserved.
    QueuedTransitionsNotPreserved,
    /// Evidence attachments were not preserved.
    EvidenceNotPreserved,
    /// Retry/export was unavailable for a deferred row.
    RetryExportUnavailable,
    /// A live mutation posture on an unresolved mapping.
    LiveMutationOnUnresolvedMapping,
    /// A live mutation posture without a selected target.
    LiveMutationWithoutSelectedTarget,
    /// A live mutation posture on a non-live session.
    LiveMutationOnNonLiveSession,
    /// A live mutation posture without effective write scope.
    LiveMutationWithoutEffectiveWriteScope,
    /// A resolved/policy-locked row without a selected target.
    ResolvedWithoutSelectedTarget,
    /// An ambiguous row that already names a single selected target.
    AmbiguousWithSelectedTarget,
    /// An ambiguous row without candidate targets.
    AmbiguousWithoutCandidates,
    /// Candidate targets present on a non-ambiguous row.
    CandidatesOutsideAmbiguous,
    /// `policy_locked_target` disagrees with the resolution state.
    PolicyLockMismatch,
    /// A read-only session that still claims effective write scope.
    ReadOnlySessionWithWriteScope,
    /// A non-actionable resolution without a concrete next action.
    UnresolvedRowMissingNextAction,
    /// A deferred posture without a concrete next action.
    DeferredPostureMissingNextAction,
    /// An invalidation event referencing an unknown row.
    InvalidationEventRowRefUnknown,
    /// An invalidation event that forced a live posture.
    InvalidationEventForcedLivePosture,
    /// An invalidation event that silently kept a live mutation.
    InvalidationEventSilentLiveMutation,
    /// An invalidation event without a concrete next action.
    InvalidationEventMissingNextAction,
    /// An invalidation event that dropped a local draft.
    InvalidationEventLocalDraftNotPreserved,
    /// An invalidation event that dropped queued transitions.
    InvalidationEventQueuedTransitionsNotPreserved,
    /// An invalidation event that dropped evidence.
    InvalidationEventEvidenceNotPreserved,
    /// A required beta profile has no claimed row.
    ProfileCoverageMissing,
    /// A required lane has no claimed row.
    LaneCoverageMissing,
    /// `profile_token` did not match `profile`.
    ProfileTokenDrift,
    /// `lane_token` did not match `lane`.
    LaneTokenDrift,
    /// `action_token` did not match `action`.
    ActionTokenDrift,
    /// `resolution_state_token` did not match `resolution_state`.
    ResolutionStateTokenDrift,
    /// `publish_posture_token` did not match `publish_posture`.
    PublishPostureTokenDrift,
    /// `next_action_token` did not match `next_action`.
    NextActionTokenDrift,
    /// `acting_identity_class_token` did not match `acting_identity_class`.
    ActingIdentityClassTokenDrift,
    /// `auth_source_token` did not match `auth_source`.
    AuthSourceTokenDrift,
    /// `session_state_token` did not match `session_state`.
    SessionStateTokenDrift,
    /// `target_kind_token` did not match `target_kind`.
    TargetKindTokenDrift,
    /// `trigger_token` did not match `trigger`.
    TriggerTokenDrift,
    /// `forced_posture_token` did not match `forced_posture`.
    ForcedPostureTokenDrift,
}

impl TargetMappingBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawTokenMaterialPresent => "raw_token_material_present",
            Self::SilentTargetRemapTaken => "silent_target_remap_taken",
            Self::SilentLiveMutationWidened => "silent_live_mutation_widened",
            Self::LocalDraftNotPreserved => "local_draft_not_preserved",
            Self::QueuedTransitionsNotPreserved => "queued_transitions_not_preserved",
            Self::EvidenceNotPreserved => "evidence_not_preserved",
            Self::RetryExportUnavailable => "retry_export_unavailable",
            Self::LiveMutationOnUnresolvedMapping => "live_mutation_on_unresolved_mapping",
            Self::LiveMutationWithoutSelectedTarget => "live_mutation_without_selected_target",
            Self::LiveMutationOnNonLiveSession => "live_mutation_on_non_live_session",
            Self::LiveMutationWithoutEffectiveWriteScope => {
                "live_mutation_without_effective_write_scope"
            }
            Self::ResolvedWithoutSelectedTarget => "resolved_without_selected_target",
            Self::AmbiguousWithSelectedTarget => "ambiguous_with_selected_target",
            Self::AmbiguousWithoutCandidates => "ambiguous_without_candidates",
            Self::CandidatesOutsideAmbiguous => "candidates_outside_ambiguous",
            Self::PolicyLockMismatch => "policy_lock_mismatch",
            Self::ReadOnlySessionWithWriteScope => "read_only_session_with_write_scope",
            Self::UnresolvedRowMissingNextAction => "unresolved_row_missing_next_action",
            Self::DeferredPostureMissingNextAction => "deferred_posture_missing_next_action",
            Self::InvalidationEventRowRefUnknown => "invalidation_event_row_ref_unknown",
            Self::InvalidationEventForcedLivePosture => "invalidation_event_forced_live_posture",
            Self::InvalidationEventSilentLiveMutation => "invalidation_event_silent_live_mutation",
            Self::InvalidationEventMissingNextAction => "invalidation_event_missing_next_action",
            Self::InvalidationEventLocalDraftNotPreserved => {
                "invalidation_event_local_draft_not_preserved"
            }
            Self::InvalidationEventQueuedTransitionsNotPreserved => {
                "invalidation_event_queued_transitions_not_preserved"
            }
            Self::InvalidationEventEvidenceNotPreserved => {
                "invalidation_event_evidence_not_preserved"
            }
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::LaneCoverageMissing => "lane_coverage_missing",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::LaneTokenDrift => "lane_token_drift",
            Self::ActionTokenDrift => "action_token_drift",
            Self::ResolutionStateTokenDrift => "resolution_state_token_drift",
            Self::PublishPostureTokenDrift => "publish_posture_token_drift",
            Self::NextActionTokenDrift => "next_action_token_drift",
            Self::ActingIdentityClassTokenDrift => "acting_identity_class_token_drift",
            Self::AuthSourceTokenDrift => "auth_source_token_drift",
            Self::SessionStateTokenDrift => "session_state_token_drift",
            Self::TargetKindTokenDrift => "target_kind_token_drift",
            Self::TriggerTokenDrift => "trigger_token_drift",
            Self::ForcedPostureTokenDrift => "forced_posture_token_drift",
        }
    }
}

/// Typed validation defect for the target-mapping beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetMappingBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: TargetMappingBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, event id, or `"page"`).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl TargetMappingBetaDefect {
    fn new(
        defect_kind: TargetMappingBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TARGET_MAPPING_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the target-mapping beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetMappingBetaSummary {
    /// Stable record kind of the parent page.
    pub page_record_kind: String,
    /// Stable record kind of the summary.
    pub record_kind: String,
    /// Number of mapping-review rows.
    pub review_row_count: usize,
    /// Number of invalidation events.
    pub invalidation_event_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Lane tokens present across the page.
    pub lanes_present: Vec<String>,
    /// Resolution-state tokens present across rows.
    pub resolution_states_present: Vec<String>,
    /// Publish-posture tokens present across rows.
    pub publish_postures_present: Vec<String>,
    /// Session-state tokens present across rows.
    pub session_states_present: Vec<String>,
    /// Counts of rows by publish-posture token.
    pub rows_by_publish_posture: BTreeMap<String, usize>,
    /// Counts of rows by resolution-state token.
    pub rows_by_resolution_state: BTreeMap<String, usize>,
    /// Counts of invalidation events by trigger token.
    pub invalidations_by_trigger: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl TargetMappingBetaSummary {
    /// Builds the summary from rows, invalidation events, and defects.
    pub fn from_records(
        rows: &[MappingReviewRow],
        invalidations: &[MappingInvalidationEvent],
        defects: &[TargetMappingBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut lanes_present: BTreeSet<String> = BTreeSet::new();
        let mut resolution_states_present: BTreeSet<String> = BTreeSet::new();
        let mut publish_postures_present: BTreeSet<String> = BTreeSet::new();
        let mut session_states_present: BTreeSet<String> = BTreeSet::new();
        let mut rows_by_publish_posture: BTreeMap<String, usize> = BTreeMap::new();
        let mut rows_by_resolution_state: BTreeMap<String, usize> = BTreeMap::new();
        let mut invalidations_by_trigger: BTreeMap<String, usize> = BTreeMap::new();

        for row in rows {
            profiles_present.insert(row.profile_token.clone());
            lanes_present.insert(row.lane_token.clone());
            resolution_states_present.insert(row.resolution_state_token.clone());
            publish_postures_present.insert(row.publish_posture_token.clone());
            session_states_present.insert(row.account_session.session_state_token.clone());
            *rows_by_publish_posture
                .entry(row.publish_posture_token.clone())
                .or_insert(0) += 1;
            *rows_by_resolution_state
                .entry(row.resolution_state_token.clone())
                .or_insert(0) += 1;
        }
        for event in invalidations {
            profiles_present.insert(event.profile_token.clone());
            *invalidations_by_trigger
                .entry(event.trigger_token.clone())
                .or_insert(0) += 1;
        }
        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: TARGET_MAPPING_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: TARGET_MAPPING_BETA_SUMMARY_RECORD_KIND.to_owned(),
            review_row_count: rows.len(),
            invalidation_event_count: invalidations.len(),
            profiles_present: profiles_present.into_iter().collect(),
            lanes_present: lanes_present.into_iter().collect(),
            resolution_states_present: resolution_states_present.into_iter().collect(),
            publish_postures_present: publish_postures_present.into_iter().collect(),
            session_states_present: session_states_present.into_iter().collect(),
            rows_by_publish_posture,
            rows_by_resolution_state,
            invalidations_by_trigger,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level target-mapping beta page consumed by admin, support, shell, and
/// reviewer fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetMappingBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Claimed mapping-review rows.
    pub review_rows: Vec<MappingReviewRow>,
    /// Claimed mapping-invalidation events.
    pub invalidation_events: Vec<MappingInvalidationEvent>,
    /// Typed validation defects.
    pub defects: Vec<TargetMappingBetaDefect>,
    /// Aggregate summary.
    pub summary: TargetMappingBetaSummary,
}

/// Support-export wrapper for the target-mapping beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetMappingBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: TargetMappingBetaPage,
    /// Defect-kind tokens present in the page.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw access tokens, raw delegated-token bodies, raw provider
    /// payloads, and raw provider URLs are excluded from the export.
    pub raw_tokens_excluded: bool,
    /// True when account/session lineage is preserved verbatim so support can
    /// name which identity acted.
    pub identity_lineage_preserved: bool,
    /// True when target-mapping lineage (resolution state, target kind,
    /// container path) is preserved verbatim so support can name which target
    /// a mutation would touch.
    pub target_lineage_preserved: bool,
    /// True when posture lineage (local draft / queued / live / read-only) is
    /// preserved verbatim.
    pub posture_lineage_preserved: bool,
    /// True when invalidation lineage (trigger, forced posture, next action) is
    /// preserved verbatim.
    pub invalidation_lineage_preserved: bool,
    /// True when the export proves the no-silent-remap and
    /// no-silent-live-mutation invariants.
    pub fail_closed_invariant: bool,
    /// Reviewable summary of the redaction posture.
    pub redaction_summary: String,
}

impl TargetMappingBetaSupportExport {
    /// Builds a support-export wrapper from a beta page. The beta page never
    /// carries raw token material, raw provider payloads, or raw provider
    /// URLs, so identity, target, posture, and invalidation lineage are
    /// preserved verbatim without further redaction.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: TargetMappingBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: TARGET_MAPPING_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_tokens_excluded: true,
            identity_lineage_preserved: true,
            target_lineage_preserved: true,
            posture_lineage_preserved: true,
            invalidation_lineage_preserved: true,
            fail_closed_invariant: true,
            redaction_summary:
                "Metadata-only target-mapping beta export: account/session identity lineage; \
                 board/project/space/repository target lineage; local-draft, queued-publish, \
                 live-mutation, and read-only posture lineage; and mapping-invalidation trigger \
                 and forced-posture lineage are preserved verbatim. Raw access tokens, raw \
                 delegated-token bodies, raw provider payloads, and raw provider URLs are excluded \
                 because the beta projection never carries them."
                    .to_owned(),
        }
    }
}

/// Validates the target-mapping beta page and returns typed defects on
/// failure.
pub fn validate_target_mapping_beta_page(
    page: &TargetMappingBetaPage,
) -> Result<(), Vec<TargetMappingBetaDefect>> {
    let defects = audit_target_mapping_beta_page(&page.review_rows, &page.invalidation_events);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for a target-mapping beta page.
pub fn audit_target_mapping_beta_page(
    rows: &[MappingReviewRow],
    invalidations: &[MappingInvalidationEvent],
) -> Vec<TargetMappingBetaDefect> {
    let mut defects = Vec::new();

    let row_ids: BTreeSet<&str> = rows.iter().map(|row| row.row_id.as_str()).collect();

    for row in rows {
        audit_row(&mut defects, row);
    }
    for event in invalidations {
        audit_invalidation_event(&mut defects, event, &row_ids);
    }

    let mut observed_profiles: BTreeSet<&str> = BTreeSet::new();
    let mut observed_lanes: BTreeSet<&str> = BTreeSet::new();
    for row in rows {
        observed_profiles.insert(row.profile_token.as_str());
        observed_lanes.insert(row.lane_token.as_str());
    }
    for event in invalidations {
        observed_profiles.insert(event.profile_token.as_str());
    }
    for profile in AccountScopeBetaProfileClass::ALL {
        if !observed_profiles.contains(profile.as_str()) {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::ProfileCoverageMissing,
                "page",
                "profiles",
                format!("missing {} profile coverage", profile.as_str()),
            ));
        }
    }
    for lane in MappingLaneClass::ALL {
        if !observed_lanes.contains(lane.as_str()) {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::LaneCoverageMissing,
                "page",
                "lanes",
                format!("missing {} lane coverage", lane.as_str()),
            ));
        }
    }

    defects
}

fn audit_row(defects: &mut Vec<TargetMappingBetaDefect>, row: &MappingReviewRow) {
    let id = row.row_id.as_str();

    // Token-drift checks.
    if row.profile_token != row.profile.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ProfileTokenDrift,
            id,
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if row.lane_token != row.lane.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::LaneTokenDrift,
            id,
            "lane_token",
            "lane_token must match lane",
        ));
    }
    if row.action_token != row.action.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ActionTokenDrift,
            id,
            "action_token",
            "action_token must match action",
        ));
    }
    if row.resolution_state_token != row.resolution_state.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ResolutionStateTokenDrift,
            id,
            "resolution_state_token",
            "resolution_state_token must match resolution_state",
        ));
    }
    if row.publish_posture_token != row.publish_posture.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::PublishPostureTokenDrift,
            id,
            "publish_posture_token",
            "publish_posture_token must match publish_posture",
        ));
    }
    if row.next_action_token != row.next_action.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::NextActionTokenDrift,
            id,
            "next_action_token",
            "next_action_token must match next_action",
        ));
    }
    audit_account_session(defects, id, &row.account_session);
    if let Some(target) = &row.selected_target {
        if target.target_kind_token != target.target_kind.as_str() {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::TargetKindTokenDrift,
                id,
                "selected_target.target_kind_token",
                "target_kind_token must match target_kind",
            ));
        }
    }

    // Guardrails.
    if row.raw_token_material_present {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::RawTokenMaterialPresent,
            id,
            "raw_token_material_present",
            "claimed beta row must not carry raw token material",
        ));
    }
    if row.silent_target_remap_taken {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::SilentTargetRemapTaken,
            id,
            "silent_target_remap_taken",
            "claimed beta row must not silently remap to a different target",
        ));
    }
    if row.silent_live_mutation_widened {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::SilentLiveMutationWidened,
            id,
            "silent_live_mutation_widened",
            "claimed beta row must not silently widen to a live mutation",
        ));
    }
    if !row.local_draft_preserved {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::LocalDraftNotPreserved,
            id,
            "local_draft_preserved",
            "claimed beta row must preserve local drafts",
        ));
    }
    if !row.queued_transitions_preserved {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::QueuedTransitionsNotPreserved,
            id,
            "queued_transitions_preserved",
            "claimed beta row must preserve queued transitions",
        ));
    }
    if !row.evidence_preserved {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::EvidenceNotPreserved,
            id,
            "evidence_preserved",
            "claimed beta row must preserve evidence attachments",
        ));
    }
    if !row.retry_export_available {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::RetryExportUnavailable,
            id,
            "retry_export_available",
            "claimed beta row must keep retry/export available",
        ));
    }

    // Live-posture safety.
    if row.publish_posture.is_live() {
        if !row.resolution_state.admits_live_mutation() {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::LiveMutationOnUnresolvedMapping,
                id,
                "publish_posture",
                "live mutation requires a resolved or policy-locked single target",
            ));
        }
        if row.selected_target.is_none() {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::LiveMutationWithoutSelectedTarget,
                id,
                "selected_target",
                "live mutation must name the target it will touch",
            ));
        }
        if !row.account_session.session_state.admits_live_mutation() {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::LiveMutationOnNonLiveSession,
                id,
                "account_session.session_state",
                "live mutation requires a live provider session",
            ));
        }
        if row.account_session.effective_write_scope_refs.is_empty() {
            defects.push(TargetMappingBetaDefect::new(
                TargetMappingBetaDefectKind::LiveMutationWithoutEffectiveWriteScope,
                id,
                "account_session.effective_write_scope_refs",
                "live mutation requires effective write scope",
            ));
        }
    }

    // Resolution / target consistency.
    match row.resolution_state {
        MappingResolutionStateClass::ResolvedSingleTarget
        | MappingResolutionStateClass::PolicyLockedTarget => {
            if row.selected_target.is_none() {
                defects.push(TargetMappingBetaDefect::new(
                    TargetMappingBetaDefectKind::ResolvedWithoutSelectedTarget,
                    id,
                    "selected_target",
                    "resolved or policy-locked row must name a selected target",
                ));
            }
        }
        MappingResolutionStateClass::AmbiguousNeedsSelection => {
            if row.selected_target.is_some() {
                defects.push(TargetMappingBetaDefect::new(
                    TargetMappingBetaDefectKind::AmbiguousWithSelectedTarget,
                    id,
                    "selected_target",
                    "ambiguous row must not pre-commit to a single target",
                ));
            }
            if row.candidate_target_refs.is_empty() {
                defects.push(TargetMappingBetaDefect::new(
                    TargetMappingBetaDefectKind::AmbiguousWithoutCandidates,
                    id,
                    "candidate_target_refs",
                    "ambiguous row must list candidate targets",
                ));
            }
        }
        _ => {}
    }
    if row.resolution_state != MappingResolutionStateClass::AmbiguousNeedsSelection
        && !row.candidate_target_refs.is_empty()
    {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::CandidatesOutsideAmbiguous,
            id,
            "candidate_target_refs",
            "candidate targets are only valid on an ambiguous row",
        ));
    }
    let policy_locked_state =
        row.resolution_state == MappingResolutionStateClass::PolicyLockedTarget;
    if policy_locked_state != row.policy_locked_target {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::PolicyLockMismatch,
            id,
            "policy_locked_target",
            "policy_locked_target must agree with the policy-locked resolution state",
        ));
    }

    // Session / scope consistency.
    if row.account_session.session_state == ProviderSessionStateClass::ReadOnly
        && !row.account_session.effective_write_scope_refs.is_empty()
    {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ReadOnlySessionWithWriteScope,
            id,
            "account_session.effective_write_scope_refs",
            "read-only session must not claim effective write scope",
        ));
    }

    // Next-action obligations.
    if !row.resolution_state.admits_live_mutation()
        && row.next_action == MappingNextActionClass::NoneProceed
    {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::UnresolvedRowMissingNextAction,
            id,
            "next_action",
            "a row that is not a resolved single target must name a concrete next action",
        ));
    }
    if row.publish_posture.is_deferred() && row.next_action == MappingNextActionClass::NoneProceed {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::DeferredPostureMissingNextAction,
            id,
            "next_action",
            "a deferred posture must name a concrete next action",
        ));
    }
}

fn audit_account_session(
    defects: &mut Vec<TargetMappingBetaDefect>,
    subject_id: &str,
    session: &AccountSessionBinding,
) {
    if session.acting_identity_class_token != session.acting_identity_class.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ActingIdentityClassTokenDrift,
            subject_id.to_owned(),
            "account_session.acting_identity_class_token",
            "acting_identity_class_token must match acting_identity_class",
        ));
    }
    if session.auth_source_token != session.auth_source.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::AuthSourceTokenDrift,
            subject_id.to_owned(),
            "account_session.auth_source_token",
            "auth_source_token must match auth_source",
        ));
    }
    if session.session_state_token != session.session_state.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::SessionStateTokenDrift,
            subject_id.to_owned(),
            "account_session.session_state_token",
            "session_state_token must match session_state",
        ));
    }
    if session.raw_token_material_present {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::RawTokenMaterialPresent,
            subject_id.to_owned(),
            "account_session.raw_token_material_present",
            "claimed beta account/session binding must not carry raw token material",
        ));
    }
}

fn audit_invalidation_event(
    defects: &mut Vec<TargetMappingBetaDefect>,
    event: &MappingInvalidationEvent,
    row_ids: &BTreeSet<&str>,
) {
    let id = event.event_id.as_str();
    if event.profile_token != event.profile.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ProfileTokenDrift,
            id,
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if event.trigger_token != event.trigger.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::TriggerTokenDrift,
            id,
            "trigger_token",
            "trigger_token must match trigger",
        ));
    }
    if event.forced_posture_token != event.forced_posture.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::ForcedPostureTokenDrift,
            id,
            "forced_posture_token",
            "forced_posture_token must match forced_posture",
        ));
    }
    if event.next_action_token != event.next_action.as_str() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::NextActionTokenDrift,
            id,
            "next_action_token",
            "next_action_token must match next_action",
        ));
    }
    if !row_ids.contains(event.affected_row_ref.as_str()) {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventRowRefUnknown,
            id,
            "affected_row_ref",
            "affected_row_ref must reference a review row on the page",
        ));
    }
    if event.forced_posture.is_live() {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventForcedLivePosture,
            id,
            "forced_posture",
            "invalidation must not force a live mutation posture",
        ));
    }
    if event.silent_live_mutation_after_invalidation {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventSilentLiveMutation,
            id,
            "silent_live_mutation_after_invalidation",
            "invalidation must not silently keep a live mutation",
        ));
    }
    if event.next_action == MappingNextActionClass::NoneProceed {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventMissingNextAction,
            id,
            "next_action",
            "invalidation must name a concrete next action",
        ));
    }
    if !event.local_draft_preserved {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventLocalDraftNotPreserved,
            id,
            "local_draft_preserved",
            "invalidation must preserve local drafts",
        ));
    }
    if !event.queued_transitions_preserved {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventQueuedTransitionsNotPreserved,
            id,
            "queued_transitions_preserved",
            "invalidation must preserve queued transitions",
        ));
    }
    if !event.evidence_preserved {
        defects.push(TargetMappingBetaDefect::new(
            TargetMappingBetaDefectKind::InvalidationEventEvidenceNotPreserved,
            id,
            "evidence_preserved",
            "invalidation must preserve evidence attachments",
        ));
    }
}

/// Builds the seeded target-mapping beta page covering connected, mirror,
/// offline, and enterprise-managed profiles across all four provider lanes.
pub fn seeded_target_mapping_beta_page() -> TargetMappingBetaPage {
    let review_rows = seed_review_rows();
    let invalidation_events = seed_invalidation_events();
    let defects = audit_target_mapping_beta_page(&review_rows, &invalidation_events);
    let summary =
        TargetMappingBetaSummary::from_records(&review_rows, &invalidation_events, &defects);
    TargetMappingBetaPage {
        record_kind: TARGET_MAPPING_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
        shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: TARGET_MAPPING_BETA_SOURCE_MATRIX_REF.to_owned(),
        review_rows,
        invalidation_events,
        defects,
        summary,
    }
}

#[allow(clippy::too_many_arguments)]
fn session(
    acting_identity_class: ActingIdentityClass,
    bound_identity_row_ref: &str,
    account_label: &str,
    auth_source: AccountAuthSourceClass,
    session_state: ProviderSessionStateClass,
    session_state_note: &str,
    effective_write_scope_refs: &[&str],
    policy_selected_account: bool,
) -> AccountSessionBinding {
    AccountSessionBinding {
        record_kind: PROVIDER_ACCOUNT_SESSION_BINDING_RECORD_KIND.to_owned(),
        schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
        shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
        acting_identity_class,
        acting_identity_class_token: acting_identity_class.as_str().to_owned(),
        bound_identity_row_ref: bound_identity_row_ref.to_owned(),
        account_label: account_label.to_owned(),
        auth_source,
        auth_source_token: auth_source.as_str().to_owned(),
        session_state,
        session_state_token: session_state.as_str().to_owned(),
        session_state_note: session_state_note.to_owned(),
        effective_write_scope_refs: effective_write_scope_refs
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        policy_selected_account,
        raw_token_material_present: false,
    }
}

fn target(
    target_kind: TargetKindClass,
    canonical_target_ref: &str,
    target_label: &str,
    container_path_label: &str,
    provider_host_label: &str,
) -> MappingTargetDescriptor {
    MappingTargetDescriptor {
        target_kind,
        target_kind_token: target_kind.as_str().to_owned(),
        canonical_target_ref: canonical_target_ref.to_owned(),
        target_label: target_label.to_owned(),
        container_path_label: container_path_label.to_owned(),
        provider_host_label: provider_host_label.to_owned(),
    }
}

fn seed_review_rows() -> Vec<MappingReviewRow> {
    vec![
        // 1. Connected, issue/work-item, live issue update on a resolved board.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:connected:issue-update-live".to_owned(),
            display_label: "Issue field update on Backend board (live)".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            lane: MappingLaneClass::IssueOrWorkItem,
            lane_token: MappingLaneClass::IssueOrWorkItem.as_str().to_owned(),
            action: MappingActionClass::IssueUpdate,
            action_token: MappingActionClass::IssueUpdate.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::ConnectedAccount,
                "account-scope-beta:connected-account:connected:human-dev",
                "Signed-in human developer account",
                AccountAuthSourceClass::HumanSession,
                ProviderSessionStateClass::Live,
                "Live system-browser session within freshness floor.",
                &["scope:project:payments/backend:issue_or_work_item_mutation"],
                false,
            ),
            provider_linked_row_ref: "provider-linked-row:work-item:payments/backend:WI-321"
                .to_owned(),
            resolution_state: MappingResolutionStateClass::ResolvedSingleTarget,
            resolution_state_token: MappingResolutionStateClass::ResolvedSingleTarget
                .as_str()
                .to_owned(),
            selected_target: Some(target(
                TargetKindClass::Board,
                "target:board:payments/backend:active",
                "Backend active board",
                "Payments org › Backend project › Active board",
                "Public code host (payments org)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::LiveProviderMutation,
            publish_posture_token: PublishPostureClass::LiveProviderMutation
                .as_str()
                .to_owned(),
            publish_posture_note: "Live update will move WI-321 on the Backend active board now."
                .to_owned(),
            next_action: MappingNextActionClass::NoneProceed,
            next_action_token: MappingNextActionClass::NoneProceed.as_str().to_owned(),
            next_action_label: "Ready to publish".to_owned(),
            remediation_path_ref: None,
            queue_item_ref: None,
            evidence_attachment_refs: vec![],
            computed_at: "2026-05-18T09:00:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 2. Connected, review, live review decision on a policy-locked repo.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:connected:review-policy-locked".to_owned(),
            display_label: "Review decision on payments/backend (policy-locked target)".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            lane: MappingLaneClass::ReviewDecision,
            lane_token: MappingLaneClass::ReviewDecision.as_str().to_owned(),
            action: MappingActionClass::ReviewDecision,
            action_token: MappingActionClass::ReviewDecision.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::ConnectedAccount,
                "account-scope-beta:connected-account:connected:human-dev",
                "Signed-in human developer account",
                AccountAuthSourceClass::HumanSession,
                ProviderSessionStateClass::Live,
                "Live session; review scope held.",
                &["scope:repo:payments/backend:review_decision_publish"],
                false,
            ),
            provider_linked_row_ref: "provider-linked-row:pr:payments/backend:1234".to_owned(),
            resolution_state: MappingResolutionStateClass::PolicyLockedTarget,
            resolution_state_token: MappingResolutionStateClass::PolicyLockedTarget
                .as_str()
                .to_owned(),
            selected_target: Some(target(
                TargetKindClass::Repository,
                "target:repository:payments/backend",
                "payments/backend",
                "Payments org › payments/backend",
                "Public code host (payments org)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: true,
            publish_posture: PublishPostureClass::LiveProviderMutation,
            publish_posture_token: PublishPostureClass::LiveProviderMutation
                .as_str()
                .to_owned(),
            publish_posture_note:
                "Live review decision on payments/backend; the target repository is policy-locked \
                 and cannot be remapped."
                    .to_owned(),
            next_action: MappingNextActionClass::NoneProceed,
            next_action_token: MappingNextActionClass::NoneProceed.as_str().to_owned(),
            next_action_label: "Ready to publish".to_owned(),
            remediation_path_ref: None,
            queue_item_ref: None,
            evidence_attachment_refs: vec![],
            computed_at: "2026-05-18T09:05:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 3. Mirror-only, issue/work-item, stale credential -> local draft.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:mirror_only:comment-stale-credential".to_owned(),
            display_label: "Comment held as local draft (stale credential)".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            lane: MappingLaneClass::IssueOrWorkItem,
            lane_token: MappingLaneClass::IssueOrWorkItem.as_str().to_owned(),
            action: MappingActionClass::Comment,
            action_token: MappingActionClass::Comment.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::ConnectedAccount,
                "account-scope-beta:connected-account:mirror_only:human-reviewer",
                "Signed-in reviewer on mirror-only profile",
                AccountAuthSourceClass::HumanSession,
                ProviderSessionStateClass::StaleCredential,
                "Mirror credential is stale; reconnect before publishing.",
                &["scope:project:payments/backend:human_authored_comment"],
                false,
            ),
            provider_linked_row_ref: "provider-linked-row:work-item:payments/backend:WI-322"
                .to_owned(),
            resolution_state: MappingResolutionStateClass::StaleNeedsRefresh,
            resolution_state_token: MappingResolutionStateClass::StaleNeedsRefresh
                .as_str()
                .to_owned(),
            selected_target: Some(target(
                TargetKindClass::Board,
                "target:board:payments/backend:triage",
                "Backend triage board",
                "Payments org › Backend project › Triage board",
                "Enterprise mirror (payments org)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::LocalDraft,
            publish_posture_token: PublishPostureClass::LocalDraft.as_str().to_owned(),
            publish_posture_note:
                "Comment is held as a local draft; the mirror credential must be refreshed before \
                 it can publish to the triage board."
                    .to_owned(),
            next_action: MappingNextActionClass::ReconnectSession,
            next_action_token: MappingNextActionClass::ReconnectSession.as_str().to_owned(),
            next_action_label: "Reconnect mirror to publish".to_owned(),
            remediation_path_ref: Some(
                "remediation-path:reconnect:mirror:payments/backend".to_owned(),
            ),
            queue_item_ref: Some(
                "publish-later-queue-item:comment:payments/backend:WI-322".to_owned(),
            ),
            evidence_attachment_refs: vec![],
            computed_at: "2026-05-18T09:10:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 4. Offline, incident handoff, offline capture -> queued publish-later.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:offline:incident-handoff-queued".to_owned(),
            display_label: "Incident handoff queued for publish-later (offline capture)".to_owned(),
            profile: AccountScopeBetaProfileClass::Offline,
            profile_token: AccountScopeBetaProfileClass::Offline.as_str().to_owned(),
            lane: MappingLaneClass::IncidentHandoff,
            lane_token: MappingLaneClass::IncidentHandoff.as_str().to_owned(),
            action: MappingActionClass::IncidentHandoff,
            action_token: MappingActionClass::IncidentHandoff.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::DelegatedCredential,
                "account-scope-beta:delegated-credential:offline:release-signer",
                "Delegated incident-responder credential",
                AccountAuthSourceClass::DelegatedCredential,
                ProviderSessionStateClass::OfflineCapture,
                "Working from imported snapshots; capture is offline.",
                &["scope:incident_queue:fleet-0001:incident_handoff"],
                false,
            ),
            provider_linked_row_ref: "provider-linked-row:incident:fleet-0001:INC-77".to_owned(),
            resolution_state: MappingResolutionStateClass::ResolvedSingleTarget,
            resolution_state_token: MappingResolutionStateClass::ResolvedSingleTarget
                .as_str()
                .to_owned(),
            selected_target: Some(target(
                TargetKindClass::IncidentQueue,
                "target:incident_queue:fleet-0001:sev1",
                "Fleet SEV1 incident queue",
                "Fleet-0001 › Incident response › SEV1 queue",
                "Incident provider (fleet-0001)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::QueuedPublishLater,
            publish_posture_token: PublishPostureClass::QueuedPublishLater.as_str().to_owned(),
            publish_posture_note:
                "Incident handoff is queued for publish-later and will publish to the SEV1 queue \
                 on reconnect."
                    .to_owned(),
            next_action: MappingNextActionClass::ReconnectSession,
            next_action_token: MappingNextActionClass::ReconnectSession.as_str().to_owned(),
            next_action_label: "Reconnect to drain queue".to_owned(),
            remediation_path_ref: Some("remediation-path:reconnect:offline:fleet-0001".to_owned()),
            queue_item_ref: Some("publish-later-queue-item:incident:fleet-0001:INC-77".to_owned()),
            evidence_attachment_refs: vec![
                "evidence:incident:fleet-0001:INC-77:timeline".to_owned(),
                "evidence:incident:fleet-0001:INC-77:logs".to_owned(),
            ],
            computed_at: "2026-05-18T09:15:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 5. Enterprise-managed, publish-later, limited scope -> ambiguous.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:enterprise_managed:issue-create-ambiguous".to_owned(),
            display_label: "Issue create needs a project selection (ambiguous)".to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            lane: MappingLaneClass::PublishLater,
            lane_token: MappingLaneClass::PublishLater.as_str().to_owned(),
            action: MappingActionClass::IssueCreate,
            action_token: MappingActionClass::IssueCreate.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::InstallationGrant,
                "account-scope-beta:installation-grant:enterprise_managed:managed-bot",
                "Enterprise-managed work-item grant",
                AccountAuthSourceClass::PolicyInjectedService,
                ProviderSessionStateClass::LimitedScope,
                "Policy-selected grant; scope is limited to managed projects.",
                &["scope:project:tenant-001/platform:issue_or_work_item_mutation"],
                true,
            ),
            provider_linked_row_ref: "provider-linked-row:draft-issue:tenant-001:DRAFT-9"
                .to_owned(),
            resolution_state: MappingResolutionStateClass::AmbiguousNeedsSelection,
            resolution_state_token: MappingResolutionStateClass::AmbiguousNeedsSelection
                .as_str()
                .to_owned(),
            selected_target: None,
            candidate_target_refs: vec![
                "target:project:tenant-001/platform".to_owned(),
                "target:project:tenant-001/fleet".to_owned(),
            ],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::LocalDraft,
            publish_posture_token: PublishPostureClass::LocalDraft.as_str().to_owned(),
            publish_posture_note:
                "Draft issue is held locally until a managed project target is selected.".to_owned(),
            next_action: MappingNextActionClass::SelectTarget,
            next_action_token: MappingNextActionClass::SelectTarget.as_str().to_owned(),
            next_action_label: "Select managed project".to_owned(),
            remediation_path_ref: Some(
                "remediation-path:select-target:tenant-001:DRAFT-9".to_owned(),
            ),
            queue_item_ref: Some(
                "publish-later-queue-item:draft-issue:tenant-001:DRAFT-9".to_owned(),
            ),
            evidence_attachment_refs: vec![],
            computed_at: "2026-05-18T09:20:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 6. Connected, review, unsupported remap on a read-only session.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:connected:review-unsupported-remap".to_owned(),
            display_label: "Review target cannot be remapped (unsupported)".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            lane: MappingLaneClass::ReviewDecision,
            lane_token: MappingLaneClass::ReviewDecision.as_str().to_owned(),
            action: MappingActionClass::ReviewDecision,
            action_token: MappingActionClass::ReviewDecision.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::ConnectedAccount,
                "account-scope-beta:connected-account:mirror_only:human-reviewer",
                "Signed-in reviewer (viewer grant)",
                AccountAuthSourceClass::HumanSession,
                ProviderSessionStateClass::ReadOnly,
                "Viewer grant; this session cannot mutate review state.",
                &[],
                false,
            ),
            provider_linked_row_ref: "provider-linked-row:pr:payments/frontend:5678".to_owned(),
            resolution_state: MappingResolutionStateClass::UnsupportedRemap,
            resolution_state_token: MappingResolutionStateClass::UnsupportedRemap
                .as_str()
                .to_owned(),
            selected_target: Some(target(
                TargetKindClass::Repository,
                "target:repository:payments/frontend",
                "payments/frontend",
                "Payments org › payments/frontend",
                "Public code host (payments org)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::ReadOnlyInspection,
            publish_posture_token: PublishPostureClass::ReadOnlyInspection.as_str().to_owned(),
            publish_posture_note:
                "The provider does not support moving this pull request to a different repository; \
                 the row is read-only here."
                    .to_owned(),
            next_action: MappingNextActionClass::ContactAdmin,
            next_action_token: MappingNextActionClass::ContactAdmin.as_str().to_owned(),
            next_action_label: "Contact admin to relocate".to_owned(),
            remediation_path_ref: Some(
                "remediation-path:admin-review:remap:payments/frontend:5678".to_owned(),
            ),
            queue_item_ref: None,
            evidence_attachment_refs: vec![],
            computed_at: "2026-05-18T09:25:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 7. Mirror-only, incident handoff, invalidated target -> local draft.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:mirror_only:incident-invalidated".to_owned(),
            display_label: "Incident queue archived (mapping invalidated)".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            lane: MappingLaneClass::IncidentHandoff,
            lane_token: MappingLaneClass::IncidentHandoff.as_str().to_owned(),
            action: MappingActionClass::IncidentHandoff,
            action_token: MappingActionClass::IncidentHandoff.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::ConnectedAccount,
                "account-scope-beta:connected-account:mirror_only:human-reviewer",
                "Signed-in incident responder",
                AccountAuthSourceClass::HumanSession,
                ProviderSessionStateClass::Live,
                "Live session; the previously mapped incident queue was archived.",
                &["scope:incident_queue:fleet-0001:incident_handoff"],
                false,
            ),
            provider_linked_row_ref: "provider-linked-row:incident:fleet-0001:INC-78".to_owned(),
            resolution_state: MappingResolutionStateClass::Invalidated,
            resolution_state_token: MappingResolutionStateClass::Invalidated.as_str().to_owned(),
            selected_target: Some(target(
                TargetKindClass::IncidentQueue,
                "target:incident_queue:fleet-0001:legacy",
                "Fleet legacy incident queue (archived)",
                "Fleet-0001 › Incident response › Legacy queue",
                "Incident provider (fleet-0001)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::LocalDraft,
            publish_posture_token: PublishPostureClass::LocalDraft.as_str().to_owned(),
            publish_posture_note:
                "The mapped incident queue was archived; the handoff is held as a local draft \
                 until a new queue is selected."
                    .to_owned(),
            next_action: MappingNextActionClass::SelectTarget,
            next_action_token: MappingNextActionClass::SelectTarget.as_str().to_owned(),
            next_action_label: "Select a new incident queue".to_owned(),
            remediation_path_ref: Some(
                "remediation-path:select-target:incident:fleet-0001:INC-78".to_owned(),
            ),
            queue_item_ref: None,
            evidence_attachment_refs: vec![
                "evidence:incident:fleet-0001:INC-78:timeline".to_owned()
            ],
            computed_at: "2026-05-18T09:30:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
        // 8. Enterprise-managed, publish-later, publish-later-only session.
        MappingReviewRow {
            record_kind: TARGET_MAPPING_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_id: "target-mapping-beta:row:enterprise_managed:evidence-publish-later".to_owned(),
            display_label: "Evidence attachment queued (publish-later only)".to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            lane: MappingLaneClass::PublishLater,
            lane_token: MappingLaneClass::PublishLater.as_str().to_owned(),
            action: MappingActionClass::EvidenceAttachment,
            action_token: MappingActionClass::EvidenceAttachment.as_str().to_owned(),
            account_session: session(
                ActingIdentityClass::InstallationGrant,
                "account-scope-beta:installation-grant:enterprise_managed:managed-bot",
                "Enterprise-managed evidence grant",
                AccountAuthSourceClass::PolicyInjectedService,
                ProviderSessionStateClass::PublishLaterOnly,
                "Policy-selected grant can only stage publish-later items.",
                &["scope:space:tenant-001/runbooks:evidence_attachment"],
                true,
            ),
            provider_linked_row_ref: "provider-linked-row:evidence:tenant-001:EV-12".to_owned(),
            resolution_state: MappingResolutionStateClass::ResolvedSingleTarget,
            resolution_state_token: MappingResolutionStateClass::ResolvedSingleTarget
                .as_str()
                .to_owned(),
            selected_target: Some(target(
                TargetKindClass::Space,
                "target:space:tenant-001/runbooks",
                "Tenant runbooks space",
                "Tenant-001 › Knowledge › Runbooks space",
                "Enterprise docs host (tenant 001)",
            )),
            candidate_target_refs: vec![],
            policy_locked_target: false,
            publish_posture: PublishPostureClass::QueuedPublishLater,
            publish_posture_token: PublishPostureClass::QueuedPublishLater.as_str().to_owned(),
            publish_posture_note:
                "Evidence attachment is queued for publish-later to the runbooks space; this grant \
                 cannot publish live."
                    .to_owned(),
            next_action: MappingNextActionClass::ReconnectSession,
            next_action_token: MappingNextActionClass::ReconnectSession.as_str().to_owned(),
            next_action_label: "Reconnect with a live grant to publish".to_owned(),
            remediation_path_ref: Some("remediation-path:reconnect:tenant-001:runbooks".to_owned()),
            queue_item_ref: Some("publish-later-queue-item:evidence:tenant-001:EV-12".to_owned()),
            evidence_attachment_refs: vec![
                "evidence:runbook:tenant-001:EV-12:postmortem".to_owned()
            ],
            computed_at: "2026-05-18T09:35:00Z".to_owned(),
            raw_token_material_present: false,
            silent_target_remap_taken: false,
            silent_live_mutation_widened: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
            retry_export_available: true,
        },
    ]
}

fn seed_invalidation_events() -> Vec<MappingInvalidationEvent> {
    vec![
        MappingInvalidationEvent {
            record_kind: TARGET_MAPPING_BETA_INVALIDATION_EVENT_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            event_id: "target-mapping-beta:invalidation:incident-queue-archived".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            affected_row_ref: "target-mapping-beta:row:mirror_only:incident-invalidated".to_owned(),
            previous_target_ref: "target:incident_queue:fleet-0001:legacy".to_owned(),
            trigger: MappingInvalidationTriggerClass::TargetArchived,
            trigger_token: MappingInvalidationTriggerClass::TargetArchived
                .as_str()
                .to_owned(),
            forced_posture: PublishPostureClass::LocalDraft,
            forced_posture_token: PublishPostureClass::LocalDraft.as_str().to_owned(),
            next_action: MappingNextActionClass::SelectTarget,
            next_action_token: MappingNextActionClass::SelectTarget.as_str().to_owned(),
            rationale_summary:
                "The legacy SEV1 incident queue was archived at the provider; the handoff dropped \
                 to a local draft and now needs a new queue."
                    .to_owned(),
            observed_at: "2026-05-18T09:29:00Z".to_owned(),
            silent_live_mutation_after_invalidation: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
        },
        MappingInvalidationEvent {
            record_kind: TARGET_MAPPING_BETA_INVALIDATION_EVENT_RECORD_KIND.to_owned(),
            schema_version: TARGET_MAPPING_BETA_SCHEMA_VERSION,
            shared_contract_ref: TARGET_MAPPING_BETA_SHARED_CONTRACT_REF.to_owned(),
            event_id: "target-mapping-beta:invalidation:mirror-credential-stale".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            affected_row_ref: "target-mapping-beta:row:mirror_only:comment-stale-credential"
                .to_owned(),
            previous_target_ref: "target:board:payments/backend:triage".to_owned(),
            trigger: MappingInvalidationTriggerClass::CredentialWentStale,
            trigger_token: MappingInvalidationTriggerClass::CredentialWentStale
                .as_str()
                .to_owned(),
            forced_posture: PublishPostureClass::LocalDraft,
            forced_posture_token: PublishPostureClass::LocalDraft.as_str().to_owned(),
            next_action: MappingNextActionClass::ReconnectSession,
            next_action_token: MappingNextActionClass::ReconnectSession.as_str().to_owned(),
            rationale_summary:
                "The mirror credential went stale; the comment dropped to a local draft and the \
                 board mapping must be refreshed on reconnect."
                    .to_owned(),
            observed_at: "2026-05-18T09:09:00Z".to_owned(),
            silent_live_mutation_after_invalidation: false,
            local_draft_preserved: true,
            queued_transitions_preserved: true,
            evidence_preserved: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_clean() {
        let page = seeded_target_mapping_beta_page();
        assert!(
            page.defects.is_empty(),
            "seeded page must validate clean: {:#?}",
            page.defects
        );
        validate_target_mapping_beta_page(&page).expect("seeded page validates");
    }

    #[test]
    fn seeded_page_covers_all_profiles_and_lanes() {
        let page = seeded_target_mapping_beta_page();
        for profile in AccountScopeBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|p| p == profile.as_str()));
        }
        for lane in MappingLaneClass::ALL {
            assert!(page
                .summary
                .lanes_present
                .iter()
                .any(|l| l == lane.as_str()));
        }
    }
}
