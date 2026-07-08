//! Merge-readiness panels, merge-queue entries, and stack-dependency chips with
//! provider-managed / repo-policy-managed / Aureline-local-estimate distinction
//! plus stale-base and queue-block reasons.
//!
//! This module narrows the `merge_readiness_panel`, `merge_queue_entry`, and
//! `stack_dependency_chip` components frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`]
//! into an implemented, export-safe panel contract. Every
//! [`MergeReadinessPanel`] makes queue and landing readiness explicit before
//! Aureline offers merge, enqueue, restack, or handoff actions: it names the queue
//! owner, states its blocked reason, labels a stale base, discloses approval
//! recomputation, exposes stack-dependency blocking, and declares auto-merge/queue
//! scope.
//!
//! The core honesty axis is [`QueueGovernance`]: provider-managed queue state,
//! repo-policy-managed queue state, and Aureline local estimate never masquerade
//! as one another. The reader can tell whether a queue result is authoritative,
//! estimated, stale, or blocked — [`QueueResultAuthority`] — from the panel alone,
//! without opening raw provider pages. A panel that is a local estimate may never
//! claim to be authoritative, and an authoritative provider result may never be
//! understated as a mere local estimate.
//!
//! Provider outage and stale-sync degradations are preserved rather than collapsed:
//! a degraded provider still lets ordinary triage continue from the local queue
//! estimate or exported readiness packet via an explicit local-continue path, and an
//! unreachable provider keeps its browser-handoff boundary explicit. Stale sync
//! degrades one panel without collapsing the whole review lane.
//!
//! The same panel contract is reused by the review workspace, review lists,
//! companion queues, handoff packets, CLI/headless output, diagnostics, Help/About,
//! and support exports, so there is no hidden provider-specific meaning. The
//! provider-freshness vocabulary is reused directly from the frozen matrix
//! ([`M5ReviewComponentStaleProviderState`]) so freshness downgrades read the same
//! everywhere.
//!
//! The packet references upstream merge-queue-entry, review-workspace,
//! landing-candidate, patch-stack, and change-lineage contracts by id rather than
//! embedding their content. Raw provider queue responses, credentials, and live
//! provider payloads stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-merge-readiness-panel.schema.json`](../../../../schemas/ui/m5-merge-readiness-panel.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md`](../../../../docs/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-merge-readiness-panels/`](../../../../fixtures/ui/m5-merge-readiness-panels/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix::M5ReviewComponentStaleProviderState;

/// Stable record-kind tag carried by [`MergeReadinessPanelPacket`].
pub const MERGE_READINESS_PANEL_RECORD_KIND: &str =
    "merge_readiness_panel_governance_and_queue_block_truth";

/// Schema version for merge-readiness panel records.
pub const MERGE_READINESS_PANEL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MERGE_READINESS_PANEL_SCHEMA_REF: &str =
    "schemas/ui/m5-merge-readiness-panel.schema.json";

/// Repo-relative path of the merge-readiness panel contract doc.
pub const MERGE_READINESS_PANEL_DOC_REF: &str =
    "docs/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md";

/// Repo-relative path of the frozen component matrix these panels implement.
pub const MERGE_READINESS_PANEL_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the merge-queue-entry contract that supplies queue identity.
pub const MERGE_READINESS_PANEL_MERGE_QUEUE_ENTRY_CONTRACT_REF: &str =
    "schemas/review/merge_queue_entry.schema.json";

/// Repo-relative path of the review-workspace contract that supplies review identity.
pub const MERGE_READINESS_PANEL_REVIEW_WORKSPACE_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the landing-candidate contract that supplies readiness identity.
pub const MERGE_READINESS_PANEL_LANDING_CANDIDATE_CONTRACT_REF: &str =
    "schemas/review/landing_candidate.schema.json";

/// Repo-relative path of the patch-stack contract that anchors stack-dependency chips.
pub const MERGE_READINESS_PANEL_PATCH_STACK_CONTRACT_REF: &str =
    "schemas/vcs/patch_stack.schema.json";

/// Repo-relative path of the change-lineage contract that anchors stack relations.
pub const MERGE_READINESS_PANEL_CHANGE_LINEAGE_CONTRACT_REF: &str =
    "schemas/review/change_lineage.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const MERGE_READINESS_PANEL_FIXTURE_DIR: &str = "fixtures/ui/m5-merge-readiness-panels";

/// Repo-relative path of the checked support-export artifact.
pub const MERGE_READINESS_PANEL_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MERGE_READINESS_PANEL_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md";

/// Who governs the queue state a panel reports.
///
/// This is the core honesty axis. Provider-managed, repo-policy-managed, and
/// Aureline-local-estimate queue state must never masquerade as one another: a local
/// estimate must not present itself as authoritative, and an authoritative provider
/// or repo-policy result must not be understated as a mere local estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueGovernance {
    /// The code host's merge queue owns the authoritative queue state.
    ProviderManaged,
    /// A repository merge policy Aureline applies owns the authoritative queue state.
    RepoPolicyManaged,
    /// Aureline computed a local estimate only; the queue result is not authoritative.
    AurelineLocalEstimate,
}

impl QueueGovernance {
    /// Every governance kind, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ProviderManaged,
        Self::RepoPolicyManaged,
        Self::AurelineLocalEstimate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderManaged => "provider_managed",
            Self::RepoPolicyManaged => "repo_policy_managed",
            Self::AurelineLocalEstimate => "aureline_local_estimate",
        }
    }

    /// Whether this governance kind reports an authoritative queue result rather than an estimate.
    ///
    /// Provider-managed and repo-policy-managed queue state is authoritative; a
    /// local estimate is not, and must never be presented as if it were.
    pub const fn is_authoritative_source(self) -> bool {
        matches!(self, Self::ProviderManaged | Self::RepoPolicyManaged)
    }
}

/// Derived label letting a reader tell a queue result apart without opening raw provider pages.
///
/// This is the AC2 axis: from the panel alone a reader can tell whether the queue
/// result is authoritative, estimated, stale, or blocked. It is derived from
/// [`QueueGovernance`], provider freshness, and the readiness state — never asserted
/// directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueResultAuthority {
    /// An authoritative, non-blocked, fresh queue result.
    Authoritative,
    /// A non-blocked estimate computed locally rather than from an authoritative source.
    Estimated,
    /// A result whose provider truth is degraded relative to the head or base it gates.
    Stale,
    /// A blocked result that is not landing until its blocking reason is resolved.
    Blocked,
}

impl QueueResultAuthority {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Estimated => "estimated",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }
}

/// Readiness verdict a merge-readiness panel or merge-queue entry reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeReadinessState {
    /// Ready to land against the current base.
    ReadyToMerge,
    /// Queued and waiting for its turn; not blocked.
    QueuedWaiting,
    /// Blocked because the base advanced and this change is stale against it.
    BlockedOnStaleBase,
    /// Blocked because one or more required checks have not passed.
    BlockedOnFailingChecks,
    /// Blocked because approvals were invalidated and must be recomputed.
    BlockedOnApprovalRecomputation,
    /// Blocked because a stack parent is blocked.
    BlockedOnStackParent,
    /// Blocked by a policy or legal gate.
    BlockedOnPolicy,
    /// Held manually and not advancing.
    HeldManually,
}

impl MergeReadinessState {
    /// Every readiness state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ReadyToMerge,
        Self::QueuedWaiting,
        Self::BlockedOnStaleBase,
        Self::BlockedOnFailingChecks,
        Self::BlockedOnApprovalRecomputation,
        Self::BlockedOnStackParent,
        Self::BlockedOnPolicy,
        Self::HeldManually,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToMerge => "ready_to_merge",
            Self::QueuedWaiting => "queued_waiting",
            Self::BlockedOnStaleBase => "blocked_on_stale_base",
            Self::BlockedOnFailingChecks => "blocked_on_failing_checks",
            Self::BlockedOnApprovalRecomputation => "blocked_on_approval_recomputation",
            Self::BlockedOnStackParent => "blocked_on_stack_parent",
            Self::BlockedOnPolicy => "blocked_on_policy",
            Self::HeldManually => "held_manually",
        }
    }

    /// Whether this state is blocked and must carry an explicit blocking reason.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BlockedOnStaleBase
                | Self::BlockedOnFailingChecks
                | Self::BlockedOnApprovalRecomputation
                | Self::BlockedOnStackParent
                | Self::BlockedOnPolicy
        )
    }

    /// Whether this state is a clean ready-to-land verdict.
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::ReadyToMerge)
    }
}

/// Declared scope of an auto-merge / queue action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoMergeScope {
    /// Auto-merge is not enabled for this change.
    NotEnabled,
    /// Auto-merge applies to this entry only.
    ThisEntryOnly,
    /// Auto-merge applies to the whole stack this change belongs to.
    WholeStack,
    /// Auto-merge applies to a batched queue landing.
    QueueBatch,
}

impl AutoMergeScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotEnabled => "not_enabled",
            Self::ThisEntryOnly => "this_entry_only",
            Self::WholeStack => "whole_stack",
            Self::QueueBatch => "queue_batch",
        }
    }
}

/// Relation of a change to its stack, shown on a stack-dependency chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StackDependencyState {
    /// The change is standalone; it belongs to no stack.
    Standalone,
    /// The change is a stack root and its stack is ready ahead of it.
    StackRootReady,
    /// A stack parent is blocked, blocking this change.
    StackParentBlocked,
    /// The change is a stack child waiting on an ancestor still in flight.
    StackChildPending,
}

impl StackDependencyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standalone => "standalone",
            Self::StackRootReady => "stack_root_ready",
            Self::StackParentBlocked => "stack_parent_blocked",
            Self::StackChildPending => "stack_child_pending",
        }
    }

    /// Whether this relation blocks the change and must carry an explicit blocking note.
    pub const fn blocks_this_change(self) -> bool {
        matches!(self, Self::StackParentBlocked)
    }
}

/// A direct action a merge-readiness panel exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeReadinessAction {
    /// Enqueue this change for merge.
    EnqueueForMerge,
    /// Merge now where merging is allowed.
    MergeNow,
    /// Restack this change onto the current base.
    RestackOntoBase,
    /// Recompute approvals for this change.
    RecomputeApprovals,
    /// Requeue after checks rerun against the new base.
    RequeueAfterRerun,
    /// Continue reviewing locally while provider freshness is degraded.
    ContinueLocalReview,
    /// Hand off to the provider in the browser.
    OpenProviderInBrowser,
    /// Export the readiness packet.
    ExportReadinessPacket,
}

impl MergeReadinessAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnqueueForMerge => "enqueue_for_merge",
            Self::MergeNow => "merge_now",
            Self::RestackOntoBase => "restack_onto_base",
            Self::RecomputeApprovals => "recompute_approvals",
            Self::RequeueAfterRerun => "requeue_after_rerun",
            Self::ContinueLocalReview => "continue_local_review",
            Self::OpenProviderInBrowser => "open_provider_in_browser",
            Self::ExportReadinessPacket => "export_readiness_packet",
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
pub enum MergeReadinessDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// A stale base was surfaced without an explicit invalidation label.
    StaleBaseUnlabeled,
    /// An approval recomputation is pending and unresolved.
    ApprovalRecomputePending,
    /// A stack parent is blocked, blocking this change.
    StackParentBlocked,
    /// Merge-queue ownership is unresolved.
    QueueOwnershipUnresolved,
    /// Browser handoff for provider deep links is unavailable.
    BrowserHandoffUnavailable,
    /// Panel trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified merge-readiness boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl MergeReadinessDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::StaleBaseUnlabeled,
        Self::ApprovalRecomputePending,
        Self::StackParentBlocked,
        Self::QueueOwnershipUnresolved,
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
            Self::ApprovalRecomputePending => "approval_recompute_pending",
            Self::StackParentBlocked => "stack_parent_blocked",
            Self::QueueOwnershipUnresolved => "queue_ownership_unresolved",
            Self::BrowserHandoffUnavailable => "browser_handoff_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse this panel contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeReadinessConsumerSurface {
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
    /// Merge-queue drawer.
    MergeQueueDrawer,
}

impl MergeReadinessConsumerSurface {
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
        Self::MergeQueueDrawer,
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
            Self::MergeQueueDrawer => "merge_queue_drawer",
        }
    }
}

/// One merge-queue entry row shown on a panel's queue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeQueueEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable review identity for this entry.
    pub entry_label: String,
    /// Human-readable queue-position label (for example, "#3 in queue").
    pub queue_position_label: String,
    /// Readiness state of this entry.
    pub entry_state: MergeReadinessState,
    /// Whether this entry is the change the panel is about.
    pub is_this_change: bool,
    /// Blocking reason detail; required and non-empty when the entry is blocked.
    pub blocked_reason_detail: String,
}

/// One stack-dependency chip shown on a panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StackDependencyChip {
    /// Stable chip id.
    pub chip_id: String,
    /// Human-readable stack identity label.
    pub stack_id_label: String,
    /// Human-readable stack-position label (for example, "2 of 4").
    pub position_label: String,
    /// Relation of this change to its stack.
    pub relation: StackDependencyState,
    /// Blocking note; required and non-empty when the relation blocks this change.
    pub blocking_note: String,
}

/// Disclosures a panel must carry, derived from its governance, freshness, and readiness.
///
/// This is the resolver output that anchors the honesty invariants: a local
/// estimate never claims to be authoritative, a blocked panel always carries a
/// blocking reason, a degraded provider preserves a local-continue path, and an
/// unreachable provider keeps its browser-handoff boundary explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergeReadinessDisclosure {
    /// The derived queue-result authority the panel must present.
    pub authority: QueueResultAuthority,
    /// Whether the panel may claim an authoritative queue result.
    pub may_claim_authoritative: bool,
    /// Whether the panel must carry an explicit blocking reason.
    pub needs_blocked_reason: bool,
    /// Whether the panel must carry an explicit browser-handoff boundary.
    pub needs_browser_handoff_boundary: bool,
    /// Whether the panel must preserve a local-continue fallback.
    pub needs_local_continue_fallback: bool,
}

/// Resolves the disclosures a panel must carry from its governance, freshness, and readiness.
///
/// The authority is derived, never asserted: a blocked readiness state reads as
/// `blocked`, a degraded provider reads as `stale`, a non-blocked local estimate
/// reads as `estimated`, and only a fresh, non-blocked, authoritative source reads
/// as `authoritative`. A stale, unreachable, conflicting, or local-only provider
/// always forces a local-continue fallback, and an unreachable provider always
/// forces an explicit handoff boundary. Stale sync therefore degrades one panel
/// without collapsing the whole review lane.
pub fn resolve_merge_readiness_disclosure(
    governance: QueueGovernance,
    provider_freshness: M5ReviewComponentStaleProviderState,
    readiness_state: MergeReadinessState,
) -> MergeReadinessDisclosure {
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

    let authority = if readiness_state.is_blocked() {
        QueueResultAuthority::Blocked
    } else if freshness_degraded {
        QueueResultAuthority::Stale
    } else if !governance.is_authoritative_source() {
        QueueResultAuthority::Estimated
    } else {
        QueueResultAuthority::Authoritative
    };

    MergeReadinessDisclosure {
        authority,
        may_claim_authoritative: matches!(authority, QueueResultAuthority::Authoritative),
        needs_blocked_reason: readiness_state.is_blocked(),
        needs_browser_handoff_boundary: freshness_forces_handoff,
        needs_local_continue_fallback: freshness_degraded,
    }
}

/// One merge-readiness panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReadinessPanel {
    /// Stable panel id.
    pub panel_id: String,
    /// Human-readable originating review identity.
    pub review_id_label: String,
    /// Human-readable queue-owner identity (never omitted).
    pub queue_owner_label: String,
    /// Who governs the queue state this panel reports.
    pub governance: QueueGovernance,
    /// Provider-freshness state, reused from the frozen component matrix.
    pub provider_freshness: M5ReviewComponentStaleProviderState,
    /// Readiness state of this change.
    pub readiness_state: MergeReadinessState,
    /// Whether the panel claims an authoritative queue result; must match the derived authority.
    pub claims_authoritative: bool,
    /// Human-readable headline readiness label.
    pub headline_readiness_label: String,
    /// Blocking reason detail; required and non-empty when the panel is blocked.
    pub blocked_reason_detail: String,
    /// Stale-base note; required and non-empty when blocked on a stale base.
    pub stale_base_note: String,
    /// Approval-recomputation note; required and non-empty when blocked on approval recomputation.
    pub approval_recomputation_note: String,
    /// Declared auto-merge / queue scope.
    pub auto_merge_scope: AutoMergeScope,
    /// Merge-queue entries in this panel's queue, in display order.
    pub queue_entries: Vec<MergeQueueEntry>,
    /// Stack-dependency chips shown on this panel, in display order.
    pub stack_chips: Vec<StackDependencyChip>,
    /// Direct actions the panel exposes, in display order.
    pub actions: Vec<MergeReadinessAction>,
    /// Browser-handoff boundary; required and non-empty when the disclosure demands it.
    pub browser_handoff_boundary: String,
    /// Local-continue fallback; required and non-empty when the disclosure demands it.
    pub local_continue_fallback: String,
    /// Source contract refs consumed by this panel.
    pub source_contract_refs: Vec<String>,
}

impl MergeReadinessPanel {
    /// Disclosures this panel must carry, derived from its governance, freshness, and readiness.
    pub fn disclosure(&self) -> MergeReadinessDisclosure {
        resolve_merge_readiness_disclosure(
            self.governance,
            self.provider_freshness,
            self.readiness_state,
        )
    }

    /// Whether this panel exposes at least one in-product action for ordinary triage.
    pub fn has_in_product_action(&self) -> bool {
        self.actions.iter().any(|action| action.is_in_product())
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReadinessTrustReview {
    /// Provider-managed, repo-policy-managed, and local-estimate queue state stay distinct.
    pub provider_local_estimate_distinct: bool,
    /// The queue owner is always explicit.
    pub queue_owner_always_explicit: bool,
    /// A blocked reason is never hidden behind a generic warning pill.
    pub blocked_reason_never_generic_warning: bool,
    /// A stale base is labeled, not hidden.
    pub stale_base_labeled_not_hidden: bool,
    /// Approval recomputation is surfaced explicitly.
    pub approval_recomputation_explicit: bool,
    /// Stack-dependency blocking is surfaced explicitly.
    pub stack_blocking_explicit: bool,
    /// Auto-merge / queue scope is stated explicitly.
    pub auto_merge_scope_explicit: bool,
    /// A provider outage preserves a local-continue path instead of collapsing the panel.
    pub provider_outage_preserves_local_continuation: bool,
    /// Stale sync degrades one panel and never collapses the whole review lane.
    pub stale_sync_never_collapses_review_lane: bool,
    /// Ordinary triage never forces raw-provider navigation.
    pub no_forced_raw_provider_navigation_for_triage: bool,
    /// Downgrade narrows the claim rather than hiding the panel.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified panels automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl MergeReadinessTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.provider_local_estimate_distinct
            && self.queue_owner_always_explicit
            && self.blocked_reason_never_generic_warning
            && self.stale_base_labeled_not_hidden
            && self.approval_recomputation_explicit
            && self.stack_blocking_explicit
            && self.auto_merge_scope_explicit
            && self.provider_outage_preserves_local_continuation
            && self.stale_sync_never_collapses_review_lane
            && self.no_forced_raw_provider_navigation_for_triage
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReadinessConsumerProjection {
    /// The review workspace reuses one panel contract.
    pub review_workspace_reuses_one_panel_contract: bool,
    /// Review lists reuse one panel contract.
    pub review_list_reuses_one_panel_contract: bool,
    /// Companion queues reuse one panel contract.
    pub companion_queue_reuses_one_panel_contract: bool,
    /// The panel distinguishes all governance kinds.
    pub panel_distinguishes_all_governance_kinds: bool,
    /// The queue-result authority is readable without opening raw provider pages.
    pub queue_result_authority_readable_without_raw_provider: bool,
    /// CLI / headless shows panel truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows panel truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows panel truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows panel truth.
    pub help_about_shows_truth: bool,
    /// Export preserves queue and stack identity across reopen paths.
    pub export_preserves_queue_and_stack_identity: bool,
}

impl MergeReadinessConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reuses_one_panel_contract
            && self.review_list_reuses_one_panel_contract
            && self.companion_queue_reuses_one_panel_contract
            && self.panel_distinguishes_all_governance_kinds
            && self.queue_result_authority_readable_without_raw_provider
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.export_preserves_queue_and_stack_identity
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReadinessProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`MergeReadinessPanelPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeReadinessPanelPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Merge-readiness panels.
    pub panels: Vec<MergeReadinessPanel>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MergeReadinessDowngradeTrigger>,
    /// Consumer surfaces that must reuse this panel contract.
    pub consumer_surfaces: Vec<MergeReadinessConsumerSurface>,
    /// Trust review block.
    pub trust_review: MergeReadinessTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MergeReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MergeReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe merge-readiness panel packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeReadinessPanelPacket {
    /// Record kind; must equal [`MERGE_READINESS_PANEL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MERGE_READINESS_PANEL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Merge-readiness panels.
    pub panels: Vec<MergeReadinessPanel>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MergeReadinessDowngradeTrigger>,
    /// Consumer surfaces that must reuse this panel contract.
    pub consumer_surfaces: Vec<MergeReadinessConsumerSurface>,
    /// Trust review block.
    pub trust_review: MergeReadinessTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MergeReadinessConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MergeReadinessProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MergeReadinessPanelPacket {
    /// Builds a merge-readiness panel packet from stable-lane input.
    pub fn new(input: MergeReadinessPanelPacketInput) -> Self {
        Self {
            record_kind: MERGE_READINESS_PANEL_RECORD_KIND.to_owned(),
            schema_version: MERGE_READINESS_PANEL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            panels: input.panels,
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

    /// Validates the merge-readiness panel invariants.
    pub fn validate(&self) -> Vec<MergeReadinessPanelViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MERGE_READINESS_PANEL_RECORD_KIND {
            violations.push(MergeReadinessPanelViolation::WrongRecordKind);
        }
        if self.schema_version != MERGE_READINESS_PANEL_SCHEMA_VERSION {
            violations.push(MergeReadinessPanelViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MergeReadinessPanelViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(MergeReadinessPanelViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(MergeReadinessPanelViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_panels(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(MergeReadinessPanelViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(MergeReadinessPanelViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(MergeReadinessPanelViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("merge-readiness panel packet serializes"),
        ) {
            violations.push(MergeReadinessPanelViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("merge-readiness panel packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked = self
            .panels
            .iter()
            .filter(|panel| panel.readiness_state.is_blocked())
            .count();

        let mut out = String::new();
        out.push_str("# Merge-Readiness Panels: Governance and Queue-Block Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Panels: {} ({} blocked)\n",
            self.panels.len(),
            blocked
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Panels\n\n");
        for panel in &self.panels {
            let disclosure = panel.disclosure();
            out.push_str(&format!(
                "- **{}** [`{}`]: owner `{}`, governance `{}`, readiness `{}`, authority `{}`\n",
                panel.review_id_label,
                panel.governance.as_str(),
                panel.queue_owner_label,
                panel.governance.as_str(),
                panel.readiness_state.as_str(),
                disclosure.authority.as_str(),
            ));
            for entry in &panel.queue_entries {
                out.push_str(&format!(
                    "  - queue `{}` [{}] — {}\n",
                    entry.entry_label,
                    entry.queue_position_label,
                    entry.entry_state.as_str()
                ));
            }
            for chip in &panel.stack_chips {
                out.push_str(&format!(
                    "  - stack `{}` [{}] — {}\n",
                    chip.stack_id_label,
                    chip.position_label,
                    chip.relation.as_str()
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in merge-readiness panel export.
#[derive(Debug)]
pub enum MergeReadinessPanelArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MergeReadinessPanelViolation>),
}

impl fmt::Display for MergeReadinessPanelArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "merge-readiness panel export parse failed: {error}"
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
                    "merge-readiness panel export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MergeReadinessPanelArtifactError {}

/// Validation failures emitted by [`MergeReadinessPanelPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MergeReadinessPanelViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No panels are present.
    PanelsMissing,
    /// A panel is incomplete.
    PanelIncomplete,
    /// A merge-queue entry is incomplete.
    QueueEntryIncomplete,
    /// A stack-dependency chip is incomplete.
    StackChipIncomplete,
    /// A panel misrepresents its queue-result authority relative to its governance and state.
    AuthorityMisrepresented,
    /// A blocked panel is missing its explicit blocking reason.
    BlockedReasonMissing,
    /// A stale-base-blocked panel is missing its explicit stale-base note.
    StaleBaseNoteMissing,
    /// An approval-recomputation-blocked panel is missing its explicit recomputation note.
    ApprovalRecomputationNoteMissing,
    /// A blocked merge-queue entry is missing its explicit blocking reason.
    QueueEntryBlockedReasonMissing,
    /// A stack chip that blocks this change is missing its explicit blocking note.
    StackBlockingNoteMissing,
    /// A panel that must preserve a local-continue fallback is missing it.
    LocalContinueFallbackMissing,
    /// A panel that needs an explicit browser-handoff boundary is missing it.
    BrowserHandoffBoundaryMissing,
    /// A panel forces raw-provider navigation for ordinary triage.
    ForcedRawProviderNavigation,
    /// The panel set does not cover every governance kind.
    GovernanceCoverageMissing,
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

impl MergeReadinessPanelViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::PanelsMissing => "panels_missing",
            Self::PanelIncomplete => "panel_incomplete",
            Self::QueueEntryIncomplete => "queue_entry_incomplete",
            Self::StackChipIncomplete => "stack_chip_incomplete",
            Self::AuthorityMisrepresented => "authority_misrepresented",
            Self::BlockedReasonMissing => "blocked_reason_missing",
            Self::StaleBaseNoteMissing => "stale_base_note_missing",
            Self::ApprovalRecomputationNoteMissing => "approval_recomputation_note_missing",
            Self::QueueEntryBlockedReasonMissing => "queue_entry_blocked_reason_missing",
            Self::StackBlockingNoteMissing => "stack_blocking_note_missing",
            Self::LocalContinueFallbackMissing => "local_continue_fallback_missing",
            Self::BrowserHandoffBoundaryMissing => "browser_handoff_boundary_missing",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::GovernanceCoverageMissing => "governance_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable merge-readiness panel export.
pub fn current_merge_readiness_panel_export(
) -> Result<MergeReadinessPanelPacket, MergeReadinessPanelArtifactError> {
    let packet: MergeReadinessPanelPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth/support_export.json"
    )))
    .map_err(MergeReadinessPanelArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MergeReadinessPanelArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &MergeReadinessPanelPacket,
    violations: &mut Vec<MergeReadinessPanelViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MERGE_READINESS_PANEL_SCHEMA_REF,
        MERGE_READINESS_PANEL_DOC_REF,
        MERGE_READINESS_PANEL_COMPONENT_MATRIX_CONTRACT_REF,
        MERGE_READINESS_PANEL_MERGE_QUEUE_ENTRY_CONTRACT_REF,
        MERGE_READINESS_PANEL_REVIEW_WORKSPACE_CONTRACT_REF,
        MERGE_READINESS_PANEL_LANDING_CANDIDATE_CONTRACT_REF,
        MERGE_READINESS_PANEL_PATCH_STACK_CONTRACT_REF,
        MERGE_READINESS_PANEL_CHANGE_LINEAGE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MergeReadinessPanelViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_panels(
    packet: &MergeReadinessPanelPacket,
    violations: &mut Vec<MergeReadinessPanelViolation>,
) {
    if packet.panels.is_empty() {
        violations.push(MergeReadinessPanelViolation::PanelsMissing);
        return;
    }

    let mut present: BTreeSet<QueueGovernance> = BTreeSet::new();

    for panel in &packet.panels {
        present.insert(panel.governance);

        if panel.panel_id.trim().is_empty()
            || panel.review_id_label.trim().is_empty()
            || panel.queue_owner_label.trim().is_empty()
            || panel.headline_readiness_label.trim().is_empty()
            || panel.actions.is_empty()
            || panel.source_contract_refs.is_empty()
        {
            violations.push(MergeReadinessPanelViolation::PanelIncomplete);
        }

        let disclosure = panel.disclosure();

        if panel.claims_authoritative != disclosure.may_claim_authoritative {
            violations.push(MergeReadinessPanelViolation::AuthorityMisrepresented);
        }
        if disclosure.needs_blocked_reason && panel.blocked_reason_detail.trim().is_empty() {
            violations.push(MergeReadinessPanelViolation::BlockedReasonMissing);
        }
        if panel.readiness_state == MergeReadinessState::BlockedOnStaleBase
            && panel.stale_base_note.trim().is_empty()
        {
            violations.push(MergeReadinessPanelViolation::StaleBaseNoteMissing);
        }
        if panel.readiness_state == MergeReadinessState::BlockedOnApprovalRecomputation
            && panel.approval_recomputation_note.trim().is_empty()
        {
            violations.push(MergeReadinessPanelViolation::ApprovalRecomputationNoteMissing);
        }
        if disclosure.needs_browser_handoff_boundary
            && panel.browser_handoff_boundary.trim().is_empty()
        {
            violations.push(MergeReadinessPanelViolation::BrowserHandoffBoundaryMissing);
        }
        if disclosure.needs_local_continue_fallback
            && panel.local_continue_fallback.trim().is_empty()
        {
            violations.push(MergeReadinessPanelViolation::LocalContinueFallbackMissing);
        }
        if !panel.has_in_product_action() {
            violations.push(MergeReadinessPanelViolation::ForcedRawProviderNavigation);
        }

        for entry in &panel.queue_entries {
            if entry.entry_id.trim().is_empty()
                || entry.entry_label.trim().is_empty()
                || entry.queue_position_label.trim().is_empty()
            {
                violations.push(MergeReadinessPanelViolation::QueueEntryIncomplete);
            }
            if entry.entry_state.is_blocked() && entry.blocked_reason_detail.trim().is_empty() {
                violations.push(MergeReadinessPanelViolation::QueueEntryBlockedReasonMissing);
            }
        }

        for chip in &panel.stack_chips {
            if chip.chip_id.trim().is_empty()
                || chip.stack_id_label.trim().is_empty()
                || chip.position_label.trim().is_empty()
            {
                violations.push(MergeReadinessPanelViolation::StackChipIncomplete);
            }
            if chip.relation.blocks_this_change() && chip.blocking_note.trim().is_empty() {
                violations.push(MergeReadinessPanelViolation::StackBlockingNoteMissing);
            }
        }
    }

    for required in QueueGovernance::ALL {
        if !present.contains(&required) {
            violations.push(MergeReadinessPanelViolation::GovernanceCoverageMissing);
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
