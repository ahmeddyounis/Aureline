//! Pending-review trays and approval-invalidation banners with reviewer-scope,
//! restack/rebase drift, publish-later continuity, and reopen-safe follow-up truth.
//!
//! This module narrows the `pending_review_tray` and `approval_invalidation_banner`
//! components frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`]
//! into an implemented, export-safe packet contract. A
//! [`PendingReviewTray`] preserves who still owes action: it names the reviewer
//! scope, lists requested reviewers and their review state, counts unresolved
//! threads, keeps local draft comments and publish-later packets visible, and
//! states the exact next-action verb. An [`ApprovalInvalidationBanner`] preserves
//! when prior approval or readiness truth became invalid: it names the specific
//! invalidation cause (stale base, rebased stack, rewritten series, changed queue
//! state, or policy drift), preserves the prior approval state that was reset, and
//! offers compare, re-review, reopen, and export actions.
//!
//! The core honesty axes are two. First, approval invalidation is kept separate
//! from generic warning and queue-block banners: a banner whose approvals were
//! invalidated must present as an [`ReviewBannerKind::ApprovalInvalidation`] banner
//! carrying its specific cause and the compare/re-review/reopen/export actions, and
//! it may never be collapsed into a generic warning or queue-block pill; conversely
//! a generic-warning or queue-block banner may not masquerade as approval
//! invalidation. Second, local draft comments and offline follow-up packets remain
//! visible even when provider freshness is degraded or unavailable, so a stale or
//! unreachable provider never hides the reviewer's own in-flight work.
//!
//! Provider outage and stale-sync degradations are preserved rather than collapsed:
//! a degraded provider still lets a reviewer continue from local drafts and
//! publish-later packets via an explicit local-continue path, and an unreachable
//! provider keeps its browser-handoff boundary explicit. Stale sync degrades one
//! tray or banner without collapsing the whole review lane.
//!
//! The same tray and banner contracts are reused by the review workspace, review
//! lists, companion queues, handoff packets, CLI/headless output, diagnostics,
//! Help/About, and support exports, so there is no hidden provider-specific
//! meaning. The provider-freshness vocabulary is reused directly from the frozen
//! matrix ([`M5ReviewComponentStaleProviderState`]) so freshness downgrades read
//! the same everywhere.
//!
//! The packet references upstream review-workspace, review-pack, publish-later,
//! approval-invalidation, and landing-candidate contracts by id rather than
//! embedding their content. Raw provider responses, credentials, and live provider
//! payloads stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-pending-review-tray.schema.json`](../../../../schemas/ui/m5-pending-review-tray.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md`](../../../../docs/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-pending-review-trays/`](../../../../fixtures/ui/m5-pending-review-trays/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix::M5ReviewComponentStaleProviderState;

/// Stable record-kind tag carried by [`PendingReviewApprovalPacket`].
pub const PENDING_REVIEW_APPROVAL_RECORD_KIND: &str =
    "pending_review_tray_and_approval_invalidation_banner_truth";

/// Schema version for pending-review / approval-invalidation records.
pub const PENDING_REVIEW_APPROVAL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PENDING_REVIEW_APPROVAL_SCHEMA_REF: &str =
    "schemas/ui/m5-pending-review-tray.schema.json";

/// Repo-relative path of the contract doc.
pub const PENDING_REVIEW_APPROVAL_DOC_REF: &str =
    "docs/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md";

/// Repo-relative path of the frozen component matrix these components implement.
pub const PENDING_REVIEW_APPROVAL_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the review-workspace contract that supplies review identity.
pub const PENDING_REVIEW_APPROVAL_REVIEW_WORKSPACE_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the review-pack contract that supplies thread and comment identity.
pub const PENDING_REVIEW_APPROVAL_REVIEW_PACK_CONTRACT_REF: &str =
    "schemas/review/review_pack.schema.json";

/// Repo-relative path of the publish-later / offline follow-up contract.
pub const PENDING_REVIEW_APPROVAL_PUBLISH_LATER_CONTRACT_REF: &str =
    "schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json";

/// Repo-relative path of the approval-invalidation / stale-base contract.
pub const PENDING_REVIEW_APPROVAL_INVALIDATION_CONTRACT_REF: &str =
    "schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json";

/// Repo-relative path of the landing-candidate contract that anchors readiness identity.
pub const PENDING_REVIEW_APPROVAL_LANDING_CANDIDATE_CONTRACT_REF: &str =
    "schemas/review/landing_candidate.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PENDING_REVIEW_APPROVAL_FIXTURE_DIR: &str = "fixtures/ui/m5-pending-review-trays";

/// Repo-relative path of the checked support-export artifact.
pub const PENDING_REVIEW_APPROVAL_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PENDING_REVIEW_APPROVAL_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth.md";

/// Who still owes action on a review, shown on a pending-review tray.
///
/// This preserves the reviewer-scope truth: the reader can tell from the tray alone
/// whether the current owner owes a review, is waiting on other reviewers, is
/// waiting on the author, has requested changes, or has nothing outstanding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerScopeState {
    /// The current owner still owes their own review.
    AwaitingMyReview,
    /// The review is waiting on other requested reviewers.
    AwaitingOtherReviewers,
    /// The review is waiting on the author to revise.
    AwaitingAuthorRevision,
    /// Changes were requested and are outstanding.
    ChangesRequested,
    /// Nothing is outstanding; no reviewer owes action.
    ReadyNoneOutstanding,
}

impl ReviewerScopeState {
    /// Every reviewer-scope state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AwaitingMyReview,
        Self::AwaitingOtherReviewers,
        Self::AwaitingAuthorRevision,
        Self::ChangesRequested,
        Self::ReadyNoneOutstanding,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingMyReview => "awaiting_my_review",
            Self::AwaitingOtherReviewers => "awaiting_other_reviewers",
            Self::AwaitingAuthorRevision => "awaiting_author_revision",
            Self::ChangesRequested => "changes_requested",
            Self::ReadyNoneOutstanding => "ready_none_outstanding",
        }
    }

    /// Whether this scope claims an owner still owes action.
    pub const fn has_outstanding_owner(self) -> bool {
        !matches!(self, Self::ReadyNoneOutstanding)
    }
}

/// Review state of one requested reviewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerReviewState {
    /// A review was requested and is still owed.
    Requested,
    /// The reviewer approved.
    Approved,
    /// The reviewer requested changes.
    ChangesRequested,
    /// The reviewer left comments without a verdict.
    Commented,
    /// The reviewer's request was dismissed.
    Dismissed,
}

impl ReviewerReviewState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Approved => "approved",
            Self::ChangesRequested => "changes_requested",
            Self::Commented => "commented",
            Self::Dismissed => "dismissed",
        }
    }

    /// Whether this reviewer still owes action.
    pub const fn is_outstanding(self) -> bool {
        matches!(self, Self::Requested | Self::ChangesRequested)
    }
}

/// Exact next-action verb a pending-review tray offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingNextAction {
    /// Submit your own review.
    SubmitYourReview,
    /// Publish local draft comments.
    PublishDraftComments,
    /// Resolve unresolved threads.
    ResolveThreads,
    /// Request a re-review.
    RequestReReview,
    /// Address changes that were requested.
    AddressChangesRequested,
    /// Await other reviewers; nothing to do now.
    AwaitReviewers,
    /// Publish or export the offline follow-up packet.
    PublishFollowUpPacket,
}

impl PendingNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SubmitYourReview => "submit_your_review",
            Self::PublishDraftComments => "publish_draft_comments",
            Self::ResolveThreads => "resolve_threads",
            Self::RequestReReview => "request_re_review",
            Self::AddressChangesRequested => "address_changes_requested",
            Self::AwaitReviewers => "await_reviewers",
            Self::PublishFollowUpPacket => "publish_follow_up_packet",
        }
    }
}

/// Which kind of banner a review surface is showing.
///
/// This is the core AC1 honesty axis: approval invalidation is kept separate from a
/// generic warning pill and from a queue-block banner. A banner whose approvals were
/// invalidated must be [`Self::ApprovalInvalidation`]; a generic warning or a
/// queue-block banner may never masquerade as approval invalidation, and approval
/// invalidation may never be collapsed into either of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewBannerKind {
    /// Prior approvals or readiness truth became invalid and must be recomputed.
    ApprovalInvalidation,
    /// A generic, non-invalidating warning.
    GenericWarning,
    /// A queue-block notice that does not invalidate approvals.
    QueueBlock,
}

impl ReviewBannerKind {
    /// Every banner kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ApprovalInvalidation,
        Self::GenericWarning,
        Self::QueueBlock,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApprovalInvalidation => "approval_invalidation",
            Self::GenericWarning => "generic_warning",
            Self::QueueBlock => "queue_block",
        }
    }
}

/// Why prior approval or readiness truth became invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalInvalidationCause {
    /// The base advanced and the change is stale against it.
    StaleBase,
    /// The stack was rebased, moving this change.
    RebasedStack,
    /// The series was rewritten (amended, squashed, or force-pushed).
    RewrittenSeries,
    /// The queue state changed under the existing approvals.
    ChangedQueueState,
    /// A repository or org policy drifted, invalidating prior readiness.
    PolicyDrift,
}

impl ApprovalInvalidationCause {
    /// Every invalidation cause, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StaleBase,
        Self::RebasedStack,
        Self::RewrittenSeries,
        Self::ChangedQueueState,
        Self::PolicyDrift,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleBase => "stale_base",
            Self::RebasedStack => "rebased_stack",
            Self::RewrittenSeries => "rewritten_series",
            Self::ChangedQueueState => "changed_queue_state",
            Self::PolicyDrift => "policy_drift",
        }
    }
}

/// A direct action an approval-invalidation banner exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalBannerAction {
    /// Compare the change that invalidated prior approvals.
    CompareInvalidatingChange,
    /// Request a re-review.
    RequestReReview,
    /// Reopen a reopen-safe follow-up.
    ReopenFollowUp,
    /// Export the invalidation packet.
    ExportInvalidationPacket,
    /// Continue reviewing locally while provider freshness is degraded.
    ContinueLocalReview,
    /// Hand off to the provider in the browser.
    OpenProviderInBrowser,
}

impl ApprovalBannerAction {
    /// The compare/re-review/reopen/export actions required on an invalidation banner.
    pub const REQUIRED_ON_INVALIDATION: [Self; 4] = [
        Self::CompareInvalidatingChange,
        Self::RequestReReview,
        Self::ReopenFollowUp,
        Self::ExportInvalidationPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareInvalidatingChange => "compare_invalidating_change",
            Self::RequestReReview => "request_re_review",
            Self::ReopenFollowUp => "reopen_follow_up",
            Self::ExportInvalidationPacket => "export_invalidation_packet",
            Self::ContinueLocalReview => "continue_local_review",
            Self::OpenProviderInBrowser => "open_provider_in_browser",
        }
    }

    /// Whether this action stays inside the product rather than forcing raw-provider navigation.
    pub const fn is_in_product(self) -> bool {
        !matches!(self, Self::OpenProviderInBrowser)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingReviewApprovalDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// A stale base was surfaced without an explicit invalidation label.
    StaleBaseUnlabeled,
    /// An approval invalidation is pending and unresolved.
    ApprovalInvalidationPending,
    /// A rewritten or rebased series was surfaced without an invalidation label.
    RewrittenSeriesUnlabeled,
    /// The reviewer scope is unresolved.
    ReviewerScopeUnresolved,
    /// Browser handoff for provider deep links is unavailable.
    BrowserHandoffUnavailable,
    /// Trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl PendingReviewApprovalDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::StaleBaseUnlabeled,
        Self::ApprovalInvalidationPending,
        Self::RewrittenSeriesUnlabeled,
        Self::ReviewerScopeUnresolved,
        Self::BrowserHandoffUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::StaleBaseUnlabeled => "stale_base_unlabeled",
            Self::ApprovalInvalidationPending => "approval_invalidation_pending",
            Self::RewrittenSeriesUnlabeled => "rewritten_series_unlabeled",
            Self::ReviewerScopeUnresolved => "reviewer_scope_unresolved",
            Self::BrowserHandoffUnavailable => "browser_handoff_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse these tray and banner contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingReviewApprovalConsumerSurface {
    /// Review workspace.
    ReviewWorkspace,
    /// Review list.
    ReviewList,
    /// Browser companion queue.
    CompanionQueue,
    /// Browser/provider handoff packet.
    HandoffPacket,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
    /// Notifications / pending-review inbox.
    NotificationsInbox,
}

impl PendingReviewApprovalConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewWorkspace,
        Self::ReviewList,
        Self::CompanionQueue,
        Self::HandoffPacket,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
        Self::NotificationsInbox,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::ReviewList => "review_list",
            Self::CompanionQueue => "companion_queue",
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
            Self::NotificationsInbox => "notifications_inbox",
        }
    }
}

/// One requested reviewer listed on a pending-review tray.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestedReviewer {
    /// Stable reviewer id.
    pub reviewer_id: String,
    /// Human-readable reviewer identity.
    pub reviewer_label: String,
    /// Review state of this reviewer.
    pub review_state: ReviewerReviewState,
    /// Whether this reviewer is a required approver.
    pub is_required: bool,
}

/// One local draft comment that has not been published to the provider.
///
/// Local draft comments remain visible even when provider freshness is degraded or
/// unavailable; they are the reviewer's own in-flight work and never leave the
/// machine until published.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalDraftComment {
    /// Stable draft id.
    pub draft_id: String,
    /// Human-readable thread or location label.
    pub thread_label: String,
    /// Short preview label for the draft.
    pub preview_label: String,
    /// Whether the draft is local-only (never published); always true for drafts.
    pub is_local_only: bool,
}

/// One publish-later / offline follow-up packet attached to a pending-review tray.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishLaterPacket {
    /// Stable packet reference id.
    pub packet_ref_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Number of items queued in this packet.
    pub item_count: u32,
    /// Whether this packet was captured while offline.
    pub is_offline_captured: bool,
}

/// Disclosures a pending-review tray must carry, derived from its provider freshness.
///
/// A degraded provider forces both a local-continue note and keeping local draft
/// comments and publish-later packets visible; an unreachable provider additionally
/// forces an explicit browser-handoff boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingTrayDisclosure {
    /// Whether the tray must carry an explicit local-continue note.
    pub needs_local_continue_note: bool,
    /// Whether the tray must keep local draft comments and publish-later packets visible.
    pub must_keep_local_evidence_visible: bool,
    /// Whether the tray must carry an explicit browser-handoff boundary.
    pub needs_browser_handoff_boundary: bool,
}

/// Resolves the disclosures a pending-review tray must carry from its provider freshness.
///
/// A stale, unreachable, conflicting, or local-only provider degrades the tray: it
/// must keep local evidence visible and offer a local-continue path, so the
/// reviewer's own drafts and follow-up packets never vanish. Only an unreachable
/// provider additionally forces an explicit browser-handoff boundary. Stale sync
/// therefore degrades one tray without collapsing the whole review lane.
pub fn resolve_pending_tray_disclosure(
    provider_freshness: M5ReviewComponentStaleProviderState,
) -> PendingTrayDisclosure {
    let freshness_degraded = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderStale
            | M5ReviewComponentStaleProviderState::ProviderUnreachable
            | M5ReviewComponentStaleProviderState::ProviderConflict
            | M5ReviewComponentStaleProviderState::LocalOnlyContinuation
    );
    let freshness_forces_handoff = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderUnreachable
    );

    PendingTrayDisclosure {
        needs_local_continue_note: freshness_degraded,
        must_keep_local_evidence_visible: freshness_degraded,
        needs_browser_handoff_boundary: freshness_forces_handoff,
    }
}

/// Disclosures an approval-invalidation banner must carry.
///
/// This anchors the AC1 separation invariant: a banner whose approvals were
/// invalidated must be an approval-invalidation banner carrying its cause detail,
/// prior approval state, and the required compare/re-review/reopen/export actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApprovalBannerDisclosure {
    /// Whether this banner must present as an approval-invalidation banner.
    pub must_be_invalidation_kind: bool,
    /// Whether this banner must carry an explicit cause detail and prior approval state.
    pub needs_cause_and_prior_state: bool,
    /// Whether this banner must carry the required invalidation actions.
    pub needs_required_actions: bool,
    /// Whether the banner must carry an explicit local-continue note.
    pub needs_local_continue_note: bool,
    /// Whether the banner must carry an explicit browser-handoff boundary.
    pub needs_browser_handoff_boundary: bool,
}

/// Resolves the disclosures an approval-invalidation banner must carry.
///
/// When approvals were invalidated the banner must present as an
/// approval-invalidation banner, carry its cause detail and prior approval state,
/// and offer the required actions. A degraded provider forces a local-continue
/// note; an unreachable provider forces a browser-handoff boundary.
pub fn resolve_approval_banner_disclosure(
    approvals_were_invalidated: bool,
    provider_freshness: M5ReviewComponentStaleProviderState,
) -> ApprovalBannerDisclosure {
    let freshness_degraded = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderStale
            | M5ReviewComponentStaleProviderState::ProviderUnreachable
            | M5ReviewComponentStaleProviderState::ProviderConflict
            | M5ReviewComponentStaleProviderState::LocalOnlyContinuation
    );
    let freshness_forces_handoff = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderUnreachable
    );

    ApprovalBannerDisclosure {
        must_be_invalidation_kind: approvals_were_invalidated,
        needs_cause_and_prior_state: approvals_were_invalidated,
        needs_required_actions: approvals_were_invalidated,
        needs_local_continue_note: freshness_degraded,
        needs_browser_handoff_boundary: freshness_forces_handoff,
    }
}

/// One pending-review tray.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReviewTray {
    /// Stable tray id.
    pub tray_id: String,
    /// Human-readable originating review identity.
    pub review_id_label: String,
    /// Provider-freshness state, reused from the frozen component matrix.
    pub provider_freshness: M5ReviewComponentStaleProviderState,
    /// Who still owes action on this review.
    pub reviewer_scope: ReviewerScopeState,
    /// Human-readable scope summary (never omitted).
    pub scope_summary_label: String,
    /// Requested reviewers and their review state, in display order.
    pub requested_reviewers: Vec<RequestedReviewer>,
    /// Count of unresolved threads on this review.
    pub unresolved_thread_count: u32,
    /// Local draft comments not yet published, in display order.
    pub local_draft_comments: Vec<LocalDraftComment>,
    /// Publish-later / offline follow-up packets, in display order.
    pub publish_later_packets: Vec<PublishLaterPacket>,
    /// Whether local draft comments and publish-later packets are visible on this tray.
    pub local_evidence_visible: bool,
    /// Exact next-action verb.
    pub next_action: PendingNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
    /// Local-continue note; required and non-empty when the disclosure demands it.
    pub local_continue_note: String,
    /// Browser-handoff boundary; required and non-empty when the disclosure demands it.
    pub browser_handoff_boundary: String,
    /// Source contract refs consumed by this tray.
    pub source_contract_refs: Vec<String>,
}

impl PendingReviewTray {
    /// Disclosures this tray must carry, derived from its provider freshness.
    pub fn disclosure(&self) -> PendingTrayDisclosure {
        resolve_pending_tray_disclosure(self.provider_freshness)
    }

    /// Whether this tray carries any local draft comments or publish-later packets.
    pub fn has_local_evidence(&self) -> bool {
        !self.local_draft_comments.is_empty() || !self.publish_later_packets.is_empty()
    }

    /// Whether any listed reviewer still owes action.
    pub fn has_outstanding_reviewer(&self) -> bool {
        self.requested_reviewers
            .iter()
            .any(|reviewer| reviewer.review_state.is_outstanding())
    }
}

/// One approval-invalidation banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalInvalidationBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// Human-readable originating review identity.
    pub review_id_label: String,
    /// Which kind of banner this is (approval invalidation, generic warning, queue block).
    pub banner_kind: ReviewBannerKind,
    /// Whether prior approvals or readiness truth actually became invalid.
    pub approvals_were_invalidated: bool,
    /// The specific invalidation cause.
    pub invalidation_cause: ApprovalInvalidationCause,
    /// Human-readable headline label.
    pub headline_label: String,
    /// Cause detail (why approvals became invalid); required when invalidated.
    pub cause_detail: String,
    /// Prior approval state that was reset; required when invalidated.
    pub prior_approval_state_label: String,
    /// Whether a reopen-safe follow-up is available.
    pub reopen_safe: bool,
    /// Reopen note; required and non-empty when a reopen-safe follow-up is available.
    pub reopen_note: String,
    /// Provider-freshness state, reused from the frozen component matrix.
    pub provider_freshness: M5ReviewComponentStaleProviderState,
    /// Direct actions the banner exposes, in display order.
    pub actions: Vec<ApprovalBannerAction>,
    /// Local-continue note; required and non-empty when the disclosure demands it.
    pub local_continue_note: String,
    /// Browser-handoff boundary; required and non-empty when the disclosure demands it.
    pub browser_handoff_boundary: String,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
}

impl ApprovalInvalidationBanner {
    /// Disclosures this banner must carry, derived from invalidation and freshness.
    pub fn disclosure(&self) -> ApprovalBannerDisclosure {
        resolve_approval_banner_disclosure(self.approvals_were_invalidated, self.provider_freshness)
    }

    /// Whether the banner exposes at least one in-product action for ordinary triage.
    pub fn has_in_product_action(&self) -> bool {
        self.actions.iter().any(|action| action.is_in_product())
    }

    /// Whether the banner carries all required invalidation actions.
    pub fn has_required_invalidation_actions(&self) -> bool {
        ApprovalBannerAction::REQUIRED_ON_INVALIDATION
            .iter()
            .all(|required| self.actions.contains(required))
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReviewApprovalTrustReview {
    /// Approval invalidation is kept separate from generic warning and queue-block banners.
    pub approval_invalidation_kept_separate: bool,
    /// A generic warning never masks an approval invalidation.
    pub generic_warning_never_masks_invalidation: bool,
    /// A queue-block banner never masks an approval invalidation.
    pub queue_block_never_masks_invalidation: bool,
    /// The reviewer scope is always explicit.
    pub reviewer_scope_always_explicit: bool,
    /// The exact next-action verb is always explicit.
    pub next_action_verb_always_explicit: bool,
    /// Local draft comments stay visible when provider freshness is degraded.
    pub local_drafts_visible_under_degraded_provider: bool,
    /// Publish-later packets stay visible when provider freshness is degraded.
    pub publish_later_packets_visible_under_degraded_provider: bool,
    /// Prior approval state is preserved when approvals are invalidated.
    pub prior_approval_state_preserved_on_invalidation: bool,
    /// Reopen-safe follow-up is preserved.
    pub reopen_safe_follow_up_preserved: bool,
    /// Ordinary triage never forces raw-provider navigation.
    pub no_forced_raw_provider_navigation_for_triage: bool,
    /// Downgrade narrows the claim rather than hiding the tray or banner.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified trays and banners automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl PendingReviewApprovalTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.approval_invalidation_kept_separate
            && self.generic_warning_never_masks_invalidation
            && self.queue_block_never_masks_invalidation
            && self.reviewer_scope_always_explicit
            && self.next_action_verb_always_explicit
            && self.local_drafts_visible_under_degraded_provider
            && self.publish_later_packets_visible_under_degraded_provider
            && self.prior_approval_state_preserved_on_invalidation
            && self.reopen_safe_follow_up_preserved
            && self.no_forced_raw_provider_navigation_for_triage
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReviewApprovalConsumerProjection {
    /// The review workspace reuses one tray and banner contract.
    pub review_workspace_reuses_one_contract: bool,
    /// Review lists reuse one tray and banner contract.
    pub review_list_reuses_one_contract: bool,
    /// Companion queues reuse one tray and banner contract.
    pub companion_queue_reuses_one_contract: bool,
    /// The tray distinguishes every reviewer scope.
    pub tray_distinguishes_reviewer_scope: bool,
    /// The banner distinguishes every invalidation cause.
    pub banner_distinguishes_invalidation_cause: bool,
    /// CLI / headless shows tray and banner truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows tray and banner truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows tray and banner truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows tray and banner truth.
    pub help_about_shows_truth: bool,
    /// Export preserves reviewer and invalidation identity across reopen paths.
    pub export_preserves_reviewer_and_invalidation_identity: bool,
}

impl PendingReviewApprovalConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reuses_one_contract
            && self.review_list_reuses_one_contract
            && self.companion_queue_reuses_one_contract
            && self.tray_distinguishes_reviewer_scope
            && self.banner_distinguishes_invalidation_cause
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.export_preserves_reviewer_and_invalidation_identity
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReviewApprovalProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PendingReviewApprovalPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingReviewApprovalPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Pending-review trays.
    pub pending_trays: Vec<PendingReviewTray>,
    /// Approval-invalidation banners.
    pub approval_banners: Vec<ApprovalInvalidationBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PendingReviewApprovalDowngradeTrigger>,
    /// Consumer surfaces that must reuse these contracts.
    pub consumer_surfaces: Vec<PendingReviewApprovalConsumerSurface>,
    /// Trust review block.
    pub trust_review: PendingReviewApprovalTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PendingReviewApprovalConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PendingReviewApprovalProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe pending-review / approval-invalidation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingReviewApprovalPacket {
    /// Record kind; must equal [`PENDING_REVIEW_APPROVAL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PENDING_REVIEW_APPROVAL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Pending-review trays.
    pub pending_trays: Vec<PendingReviewTray>,
    /// Approval-invalidation banners.
    pub approval_banners: Vec<ApprovalInvalidationBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PendingReviewApprovalDowngradeTrigger>,
    /// Consumer surfaces that must reuse these contracts.
    pub consumer_surfaces: Vec<PendingReviewApprovalConsumerSurface>,
    /// Trust review block.
    pub trust_review: PendingReviewApprovalTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PendingReviewApprovalConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PendingReviewApprovalProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PendingReviewApprovalPacket {
    /// Builds a pending-review / approval-invalidation packet from stable-lane input.
    pub fn new(input: PendingReviewApprovalPacketInput) -> Self {
        Self {
            record_kind: PENDING_REVIEW_APPROVAL_RECORD_KIND.to_owned(),
            schema_version: PENDING_REVIEW_APPROVAL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            pending_trays: input.pending_trays,
            approval_banners: input.approval_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the pending-review / approval-invalidation invariants.
    pub fn validate(&self) -> Vec<PendingReviewApprovalViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PENDING_REVIEW_APPROVAL_RECORD_KIND {
            violations.push(PendingReviewApprovalViolation::WrongRecordKind);
        }
        if self.schema_version != PENDING_REVIEW_APPROVAL_SCHEMA_VERSION {
            violations.push(PendingReviewApprovalViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PendingReviewApprovalViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PendingReviewApprovalViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(PendingReviewApprovalViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_trays(self, &mut violations);
        validate_banners(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PendingReviewApprovalViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PendingReviewApprovalViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PendingReviewApprovalViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("pending-review / approval packet serializes"),
        ) {
            violations.push(PendingReviewApprovalViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("pending-review / approval packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let invalidated = self
            .approval_banners
            .iter()
            .filter(|banner| banner.approvals_were_invalidated)
            .count();

        let mut out = String::new();
        out.push_str(
            "# Pending-Review Trays and Approval-Invalidation Banners: Reviewer-Scope and Reopen-Safe Truth\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!("- Pending trays: {}\n", self.pending_trays.len()));
        out.push_str(&format!(
            "- Approval banners: {} ({} invalidating)\n",
            self.approval_banners.len(),
            invalidated
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Pending trays\n\n");
        for tray in &self.pending_trays {
            out.push_str(&format!(
                "- **{}** [`{}`]: scope `{}`, next `{}`, drafts {}, follow-up {}\n",
                tray.review_id_label,
                tray.tray_id,
                tray.reviewer_scope.as_str(),
                tray.next_action.as_str(),
                tray.local_draft_comments.len(),
                tray.publish_later_packets.len(),
            ));
        }

        out.push_str("\n## Approval banners\n\n");
        for banner in &self.approval_banners {
            out.push_str(&format!(
                "- **{}** [`{}`]: kind `{}`, cause `{}`, reopen-safe {}\n",
                banner.review_id_label,
                banner.banner_id,
                banner.banner_kind.as_str(),
                banner.invalidation_cause.as_str(),
                banner.reopen_safe,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in pending-review / approval export.
#[derive(Debug)]
pub enum PendingReviewApprovalArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PendingReviewApprovalViolation>),
}

impl fmt::Display for PendingReviewApprovalArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "pending-review / approval export parse failed: {error}"
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
                    "pending-review / approval export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PendingReviewApprovalArtifactError {}

/// Validation failures emitted by [`PendingReviewApprovalPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PendingReviewApprovalViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No pending-review trays are present.
    PendingTraysMissing,
    /// No approval-invalidation banners are present.
    ApprovalBannersMissing,
    /// A pending-review tray is incomplete.
    TrayIncomplete,
    /// A requested reviewer is incomplete.
    ReviewerIncomplete,
    /// A local draft comment is incomplete.
    LocalDraftIncomplete,
    /// A publish-later packet is incomplete.
    PublishLaterPacketIncomplete,
    /// The reviewer scope misrepresents outstanding action (for example, claims none outstanding
    /// while reviewers or threads still owe action).
    ReviewerScopeMisrepresented,
    /// Local draft comments or publish-later packets are present but hidden.
    LocalDraftsOrFollowUpHidden,
    /// A tray that must preserve a local-continue path is missing its local-continue note.
    TrayLocalContinueNoteMissing,
    /// A tray that needs an explicit browser-handoff boundary is missing it.
    TrayBrowserHandoffBoundaryMissing,
    /// An approval-invalidation banner is incomplete.
    BannerIncomplete,
    /// An approval invalidation is collapsed into a generic warning or queue-block banner, or a
    /// non-invalidating banner masquerades as approval invalidation.
    ApprovalInvalidationNotSeparated,
    /// An invalidation banner is missing its explicit cause detail.
    InvalidationCauseDetailMissing,
    /// An invalidation banner is missing its explicit prior approval state.
    PriorApprovalStateMissing,
    /// An invalidation banner is missing one or more required compare/re-review/reopen/export actions.
    RequiredInvalidationActionsMissing,
    /// A reopen-safe banner is missing its explicit reopen note.
    ReopenNoteMissing,
    /// A banner that must preserve a local-continue path is missing its local-continue note.
    BannerLocalContinueNoteMissing,
    /// A banner that needs an explicit browser-handoff boundary is missing it.
    BannerBrowserHandoffBoundaryMissing,
    /// A banner forces raw-provider navigation for ordinary triage.
    ForcedRawProviderNavigation,
    /// The invalidation banners do not cover every invalidation cause.
    InvalidationCauseCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PendingReviewApprovalViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PendingTraysMissing => "pending_trays_missing",
            Self::ApprovalBannersMissing => "approval_banners_missing",
            Self::TrayIncomplete => "tray_incomplete",
            Self::ReviewerIncomplete => "reviewer_incomplete",
            Self::LocalDraftIncomplete => "local_draft_incomplete",
            Self::PublishLaterPacketIncomplete => "publish_later_packet_incomplete",
            Self::ReviewerScopeMisrepresented => "reviewer_scope_misrepresented",
            Self::LocalDraftsOrFollowUpHidden => "local_drafts_or_follow_up_hidden",
            Self::TrayLocalContinueNoteMissing => "tray_local_continue_note_missing",
            Self::TrayBrowserHandoffBoundaryMissing => "tray_browser_handoff_boundary_missing",
            Self::BannerIncomplete => "banner_incomplete",
            Self::ApprovalInvalidationNotSeparated => "approval_invalidation_not_separated",
            Self::InvalidationCauseDetailMissing => "invalidation_cause_detail_missing",
            Self::PriorApprovalStateMissing => "prior_approval_state_missing",
            Self::RequiredInvalidationActionsMissing => "required_invalidation_actions_missing",
            Self::ReopenNoteMissing => "reopen_note_missing",
            Self::BannerLocalContinueNoteMissing => "banner_local_continue_note_missing",
            Self::BannerBrowserHandoffBoundaryMissing => "banner_browser_handoff_boundary_missing",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::InvalidationCauseCoverageMissing => "invalidation_cause_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable pending-review / approval export.
pub fn current_pending_review_approval_export(
) -> Result<PendingReviewApprovalPacket, PendingReviewApprovalArtifactError> {
    let packet: PendingReviewApprovalPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_pending_review_trays_and_approval_invalidation_banners_with_reviewer_scope_restack_drift_and_reopen_safe_truth/support_export.json"
    )))
    .map_err(PendingReviewApprovalArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PendingReviewApprovalArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &PendingReviewApprovalPacket,
    violations: &mut Vec<PendingReviewApprovalViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PENDING_REVIEW_APPROVAL_SCHEMA_REF,
        PENDING_REVIEW_APPROVAL_DOC_REF,
        PENDING_REVIEW_APPROVAL_COMPONENT_MATRIX_CONTRACT_REF,
        PENDING_REVIEW_APPROVAL_REVIEW_WORKSPACE_CONTRACT_REF,
        PENDING_REVIEW_APPROVAL_REVIEW_PACK_CONTRACT_REF,
        PENDING_REVIEW_APPROVAL_PUBLISH_LATER_CONTRACT_REF,
        PENDING_REVIEW_APPROVAL_INVALIDATION_CONTRACT_REF,
        PENDING_REVIEW_APPROVAL_LANDING_CANDIDATE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PendingReviewApprovalViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_trays(
    packet: &PendingReviewApprovalPacket,
    violations: &mut Vec<PendingReviewApprovalViolation>,
) {
    if packet.pending_trays.is_empty() {
        violations.push(PendingReviewApprovalViolation::PendingTraysMissing);
        return;
    }

    for tray in &packet.pending_trays {
        if tray.tray_id.trim().is_empty()
            || tray.review_id_label.trim().is_empty()
            || tray.scope_summary_label.trim().is_empty()
            || tray.next_action_label.trim().is_empty()
            || tray.source_contract_refs.is_empty()
        {
            violations.push(PendingReviewApprovalViolation::TrayIncomplete);
        }

        for reviewer in &tray.requested_reviewers {
            if reviewer.reviewer_id.trim().is_empty() || reviewer.reviewer_label.trim().is_empty() {
                violations.push(PendingReviewApprovalViolation::ReviewerIncomplete);
            }
        }
        for draft in &tray.local_draft_comments {
            if draft.draft_id.trim().is_empty()
                || draft.thread_label.trim().is_empty()
                || draft.preview_label.trim().is_empty()
            {
                violations.push(PendingReviewApprovalViolation::LocalDraftIncomplete);
            }
        }
        for follow_up in &tray.publish_later_packets {
            if follow_up.packet_ref_id.trim().is_empty() || follow_up.packet_label.trim().is_empty()
            {
                violations.push(PendingReviewApprovalViolation::PublishLaterPacketIncomplete);
            }
        }

        // Reviewer-scope truth: a tray claiming nothing is outstanding must not still
        // owe reviewer action or carry unresolved threads.
        if !tray.reviewer_scope.has_outstanding_owner()
            && (tray.has_outstanding_reviewer() || tray.unresolved_thread_count > 0)
        {
            violations.push(PendingReviewApprovalViolation::ReviewerScopeMisrepresented);
        }

        // AC2: local draft comments and publish-later packets remain visible.
        if tray.has_local_evidence() && !tray.local_evidence_visible {
            violations.push(PendingReviewApprovalViolation::LocalDraftsOrFollowUpHidden);
        }

        let disclosure = tray.disclosure();
        if disclosure.needs_local_continue_note && tray.local_continue_note.trim().is_empty() {
            violations.push(PendingReviewApprovalViolation::TrayLocalContinueNoteMissing);
        }
        if disclosure.needs_browser_handoff_boundary
            && tray.browser_handoff_boundary.trim().is_empty()
        {
            violations.push(PendingReviewApprovalViolation::TrayBrowserHandoffBoundaryMissing);
        }
    }
}

fn validate_banners(
    packet: &PendingReviewApprovalPacket,
    violations: &mut Vec<PendingReviewApprovalViolation>,
) {
    if packet.approval_banners.is_empty() {
        violations.push(PendingReviewApprovalViolation::ApprovalBannersMissing);
        return;
    }

    let mut covered_causes: BTreeSet<ApprovalInvalidationCause> = BTreeSet::new();

    for banner in &packet.approval_banners {
        if banner.banner_id.trim().is_empty()
            || banner.review_id_label.trim().is_empty()
            || banner.headline_label.trim().is_empty()
            || banner.actions.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations.push(PendingReviewApprovalViolation::BannerIncomplete);
        }

        let disclosure = banner.disclosure();

        // AC1 both directions: an invalidating banner must present as approval
        // invalidation, and a non-invalidating banner must not.
        let is_invalidation_kind =
            matches!(banner.banner_kind, ReviewBannerKind::ApprovalInvalidation);
        if disclosure.must_be_invalidation_kind != is_invalidation_kind {
            violations.push(PendingReviewApprovalViolation::ApprovalInvalidationNotSeparated);
        }

        if banner.approvals_were_invalidated {
            covered_causes.insert(banner.invalidation_cause);
        }

        if disclosure.needs_cause_and_prior_state {
            if banner.cause_detail.trim().is_empty() {
                violations.push(PendingReviewApprovalViolation::InvalidationCauseDetailMissing);
            }
            if banner.prior_approval_state_label.trim().is_empty() {
                violations.push(PendingReviewApprovalViolation::PriorApprovalStateMissing);
            }
        }
        if disclosure.needs_required_actions && !banner.has_required_invalidation_actions() {
            violations.push(PendingReviewApprovalViolation::RequiredInvalidationActionsMissing);
        }
        if banner.reopen_safe && banner.reopen_note.trim().is_empty() {
            violations.push(PendingReviewApprovalViolation::ReopenNoteMissing);
        }
        if disclosure.needs_local_continue_note && banner.local_continue_note.trim().is_empty() {
            violations.push(PendingReviewApprovalViolation::BannerLocalContinueNoteMissing);
        }
        if disclosure.needs_browser_handoff_boundary
            && banner.browser_handoff_boundary.trim().is_empty()
        {
            violations.push(PendingReviewApprovalViolation::BannerBrowserHandoffBoundaryMissing);
        }
        if !banner.has_in_product_action() {
            violations.push(PendingReviewApprovalViolation::ForcedRawProviderNavigation);
        }
    }

    for required in ApprovalInvalidationCause::ALL {
        if !covered_causes.contains(&required) {
            violations.push(PendingReviewApprovalViolation::InvalidationCauseCoverageMissing);
            return;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
