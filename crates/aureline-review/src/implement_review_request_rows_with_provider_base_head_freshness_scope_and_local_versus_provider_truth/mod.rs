//! Review-request rows with provider/base-head/branch freshness, scope, and
//! local-versus-provider truth.
//!
//! This module narrows the `review_request_row` component frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`]
//! into an implemented, export-safe row contract. Every
//! [`ReviewRequestRow`] answers, from the row alone, what the object is, which
//! provider or local object owns it, what scope it covers, how fresh that truth
//! is, and — most importantly — whether the row is a local review estimate, a
//! provider-backed pull/merge request, an offline/exported review packet, or a
//! browser-handoff placeholder. A local estimate never pretends hosted status
//! exists, and a degraded provider is never flattened into a local estimate.
//!
//! The same row contract is reused by review lists, inboxes, switchers, companion
//! queues, and handoff packets, so there is no hidden provider-specific meaning.
//! The provider-freshness vocabulary is reused directly from the frozen matrix
//! ([`M5ReviewComponentStaleProviderState`]) so freshness downgrades read the same
//! everywhere.
//!
//! The packet references upstream review-workspace, merge-queue, change-lineage,
//! and component-matrix contracts by id rather than embedding their content. Raw
//! diff bodies, raw check logs, raw provider payloads, credentials, and live
//! provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-review-request-row.schema.json`](../../../../schemas/ui/m5-review-request-row.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth.md`](../../../../docs/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-review-request-rows/`](../../../../fixtures/ui/m5-review-request-rows/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix::M5ReviewComponentStaleProviderState;

/// Stable record-kind tag carried by [`ReviewRequestRowPacket`].
pub const REVIEW_REQUEST_ROW_RECORD_KIND: &str = "review_request_row_local_versus_provider_truth";

/// Schema version for review-request row records.
pub const REVIEW_REQUEST_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REVIEW_REQUEST_ROW_SCHEMA_REF: &str = "schemas/ui/m5-review-request-row.schema.json";

/// Repo-relative path of the review-request row contract doc.
pub const REVIEW_REQUEST_ROW_DOC_REF: &str =
    "docs/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth.md";

/// Repo-relative path of the frozen component matrix this row implements.
pub const REVIEW_REQUEST_ROW_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the review-workspace contract that supplies object identity.
pub const REVIEW_REQUEST_ROW_REVIEW_WORKSPACE_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the merge-queue entry contract this row links to.
pub const REVIEW_REQUEST_ROW_MERGE_QUEUE_CONTRACT_REF: &str =
    "schemas/review/merge_queue_entry.schema.json";

/// Repo-relative path of the change-lineage contract that supplies stack relation.
pub const REVIEW_REQUEST_ROW_CHANGE_LINEAGE_CONTRACT_REF: &str =
    "schemas/review/change_lineage.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const REVIEW_REQUEST_ROW_FIXTURE_DIR: &str = "fixtures/ui/m5-review-request-rows";

/// Repo-relative path of the checked support-export artifact.
pub const REVIEW_REQUEST_ROW_ARTIFACT_REF: &str =
    "artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REVIEW_REQUEST_ROW_SUMMARY_REF: &str =
    "artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth.md";

/// What backs a review-request row: the local-versus-provider distinction.
///
/// This is the core honesty axis. A row must let the reader tell a local review
/// estimate, a provider-backed pull/merge request, an offline/exported review
/// packet, and a browser-handoff placeholder apart from the row alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRequestBackingKind {
    /// Local-only review estimate; no hosted request exists yet.
    LocalReviewEstimate,
    /// Provider-backed pull/merge request with a real hosted object.
    ProviderBackedRequest,
    /// Offline or exported review packet; cached context, not live hosted truth.
    OfflineExportedPacket,
    /// Browser-handoff placeholder; the row must hand off rather than claim hosted status.
    BrowserHandoffPlaceholder,
}

impl ReviewRequestBackingKind {
    /// Every backing kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalReviewEstimate,
        Self::ProviderBackedRequest,
        Self::OfflineExportedPacket,
        Self::BrowserHandoffPlaceholder,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReviewEstimate => "local_review_estimate",
            Self::ProviderBackedRequest => "provider_backed_request",
            Self::OfflineExportedPacket => "offline_exported_packet",
            Self::BrowserHandoffPlaceholder => "browser_handoff_placeholder",
        }
    }

    /// Whether a row of this kind legitimately asserts hosted provider-backed status.
    ///
    /// Only a real provider-backed request may claim hosted status; every other
    /// kind must not pretend a hosted object exists.
    pub const fn asserts_hosted_status(self) -> bool {
        matches!(self, Self::ProviderBackedRequest)
    }

    /// Whether a row of this kind must carry an explicit browser-handoff boundary.
    pub const fn needs_browser_handoff(self) -> bool {
        matches!(self, Self::BrowserHandoffPlaceholder)
    }

    /// Whether a row of this kind must preserve a local-only continuation path.
    pub const fn preserves_local_continuation(self) -> bool {
        matches!(
            self,
            Self::LocalReviewEstimate | Self::OfflineExportedPacket
        )
    }
}

/// Base/head or branch freshness class shown on a review-request row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseHeadFreshness {
    /// Head is current against its base.
    Current,
    /// The base advanced under the request; the row shows a stale-base label.
    StaleBase,
    /// The head is outdated relative to its local source.
    OutdatedHead,
    /// Base and head histories have diverged.
    Diverged,
    /// Freshness cannot be computed (for example, offline).
    Unknown,
}

impl BaseHeadFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleBase => "stale_base",
            Self::OutdatedHead => "outdated_head",
            Self::Diverged => "diverged",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this class represents a stale or diverged relation that must be labeled.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleBase | Self::OutdatedHead | Self::Diverged)
    }
}

/// Stack relation shown on a review-request row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRequestStackRelation {
    /// The request stands alone with no stack parent or child.
    Standalone,
    /// The request is the root of a stack.
    StackRoot,
    /// The request is a stack member whose parent is ready.
    StackMemberParentReady,
    /// The request is a stack member whose parent is blocked, blocking this request.
    StackMemberParentBlocked,
}

impl ReviewRequestStackRelation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standalone => "standalone",
            Self::StackRoot => "stack_root",
            Self::StackMemberParentReady => "stack_member_parent_ready",
            Self::StackMemberParentBlocked => "stack_member_parent_blocked",
        }
    }

    /// Whether the row's stack parent is blocked.
    pub const fn parent_blocked(self) -> bool {
        matches!(self, Self::StackMemberParentBlocked)
    }
}

/// Scope a review-request row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRequestScope {
    /// The whole request from base to head.
    FullRequest,
    /// One segment of a stack.
    StackSegment,
    /// A single commit.
    SingleCommit,
    /// A partial selection of files or hunks.
    PartialSelection,
}

impl ReviewRequestScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRequest => "full_request",
            Self::StackSegment => "stack_segment",
            Self::SingleCommit => "single_commit",
            Self::PartialSelection => "partial_selection",
        }
    }
}

/// A direct action a review-request row exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRequestAction {
    /// Open the review inside the Aureline workspace.
    OpenInWorkspace,
    /// Hand off to the provider in the browser.
    OpenProviderInBrowser,
    /// Export a review packet.
    ExportReviewPacket,
    /// Continue the review locally while provider freshness is degraded.
    ContinueLocalReview,
    /// Refresh provider-backed truth.
    RefreshProviderTruth,
}

impl ReviewRequestAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenInWorkspace => "open_in_workspace",
            Self::OpenProviderInBrowser => "open_provider_in_browser",
            Self::ExportReviewPacket => "export_review_packet",
            Self::ContinueLocalReview => "continue_local_review",
            Self::RefreshProviderTruth => "refresh_provider_truth",
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
pub enum ReviewRequestRowDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// Approvals were invalidated and must be recomputed.
    ApprovalInvalidated,
    /// A stack parent is blocked, blocking this row's change.
    StackParentBlocked,
    /// Browser handoff for provider deep links is unavailable.
    BrowserHandoffUnavailable,
    /// Row trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified review-row boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl ReviewRequestRowDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::ApprovalInvalidated,
        Self::StackParentBlocked,
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
            Self::ApprovalInvalidated => "approval_invalidated",
            Self::StackParentBlocked => "stack_parent_blocked",
            Self::BrowserHandoffUnavailable => "browser_handoff_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse this row contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewRequestRowConsumerSurface {
    /// Review list.
    ReviewList,
    /// Review inbox.
    ReviewInbox,
    /// Review switcher.
    ReviewSwitcher,
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
}

impl ReviewRequestRowConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewList,
        Self::ReviewInbox,
        Self::ReviewSwitcher,
        Self::CompanionQueue,
        Self::HandoffPacket,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewList => "review_list",
            Self::ReviewInbox => "review_inbox",
            Self::ReviewSwitcher => "review_switcher",
            Self::CompanionQueue => "companion_queue",
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Disclosures a row must carry, derived from its backing kind and provider freshness.
///
/// This is the resolver output that anchors the honesty invariants: a local or
/// placeholder row never asserts hosted status, a degraded provider preserves a
/// local-continue path, and an unreachable or placeholder row keeps its
/// browser-handoff boundary explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewRequestDisclosure {
    /// Whether the row asserts hosted provider-backed status.
    pub asserts_hosted_status: bool,
    /// Whether the row must carry an explicit browser-handoff boundary.
    pub needs_browser_handoff_boundary: bool,
    /// Whether the row must preserve a local-continue fallback.
    pub needs_local_continue_fallback: bool,
}

/// Resolves the disclosures a row must carry from its backing kind and provider freshness.
///
/// The distinction is derived, never asserted directly: hosted status follows the
/// backing kind alone, so a local estimate or an offline packet can never claim a
/// hosted object exists. A stale, unreachable, conflicting, or local-only-continued
/// provider always forces a local-continue fallback, and an unreachable provider or
/// a browser-handoff placeholder always forces an explicit handoff boundary.
pub fn resolve_review_request_row_disclosure(
    backing: ReviewRequestBackingKind,
    provider_freshness: M5ReviewComponentStaleProviderState,
) -> ReviewRequestDisclosure {
    let freshness_forces_local_continue = matches!(
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
    ReviewRequestDisclosure {
        asserts_hosted_status: backing.asserts_hosted_status(),
        needs_browser_handoff_boundary: backing.needs_browser_handoff() || freshness_forces_handoff,
        needs_local_continue_fallback: backing.preserves_local_continuation()
            || freshness_forces_local_continue,
    }
}

/// One review-request row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRequestRow {
    /// Stable row id.
    pub row_id: String,
    /// What backs the row: the local-versus-provider distinction.
    pub backing_kind: ReviewRequestBackingKind,
    /// Human-readable provider or local-object identity (who owns the object).
    pub provider_identity_label: String,
    /// Human-readable object id (PR/MR number or local bundle id).
    pub object_id_label: String,
    /// Human-readable base ref label.
    pub base_ref_label: String,
    /// Human-readable head ref label.
    pub head_ref_label: String,
    /// Base/head or branch freshness class.
    pub base_head_freshness: BaseHeadFreshness,
    /// Stack relation shown on the row.
    pub stack_relation: ReviewRequestStackRelation,
    /// Scope the row covers.
    pub scope: ReviewRequestScope,
    /// Provider-freshness state, reused from the frozen component matrix.
    pub provider_freshness: M5ReviewComponentStaleProviderState,
    /// Whether the row claims hosted provider-backed status; must match the backing kind.
    pub claims_provider_backed: bool,
    /// Browser-handoff boundary; required and non-empty when the disclosure demands it.
    pub browser_handoff_boundary: String,
    /// Local-continue fallback; required and non-empty when the disclosure demands it.
    pub local_continue_fallback: String,
    /// Direct actions the row exposes, in display order.
    pub actions: Vec<ReviewRequestAction>,
    /// Row fields the surface projects, in display order.
    pub row_fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl ReviewRequestRow {
    /// Disclosures this row must carry, derived from its backing kind and freshness.
    pub fn disclosure(&self) -> ReviewRequestDisclosure {
        resolve_review_request_row_disclosure(self.backing_kind, self.provider_freshness)
    }

    /// Whether this row exposes at least one in-product action for ordinary triage.
    pub fn has_in_product_action(&self) -> bool {
        self.actions.iter().any(|action| action.is_in_product())
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRequestRowTrustReview {
    /// A provider-managed request is never flattened into a local estimate.
    pub provider_local_estimate_distinct: bool,
    /// A local estimate never claims hosted provider-backed status.
    pub local_estimate_never_claims_hosted: bool,
    /// An offline/exported review packet is distinguishable from live hosted truth.
    pub offline_exported_packet_distinct: bool,
    /// Provider freshness is explicit, never implied.
    pub provider_freshness_explicit: bool,
    /// Base/head or branch relation is explicit.
    pub base_head_relation_explicit: bool,
    /// Stack relation and parent blocking stay explicit.
    pub stack_relation_explicit: bool,
    /// Browser handoff stays explicit with a safe return path.
    pub browser_handoff_explicit: bool,
    /// Local-only continuation is preserved when provider freshness is degraded.
    pub local_continue_preserved_on_degraded_freshness: bool,
    /// Ordinary triage never forces raw-provider navigation.
    pub no_forced_raw_provider_navigation_for_triage: bool,
    /// One row contract is reused with no hidden provider-specific meaning.
    pub one_row_contract_no_hidden_provider_meaning: bool,
    /// Downgrade narrows the claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ReviewRequestRowTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.provider_local_estimate_distinct
            && self.local_estimate_never_claims_hosted
            && self.offline_exported_packet_distinct
            && self.provider_freshness_explicit
            && self.base_head_relation_explicit
            && self.stack_relation_explicit
            && self.browser_handoff_explicit
            && self.local_continue_preserved_on_degraded_freshness
            && self.no_forced_raw_provider_navigation_for_triage
            && self.one_row_contract_no_hidden_provider_meaning
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRequestRowConsumerProjection {
    /// Review lists reuse one row contract.
    pub review_list_reuses_one_row_contract: bool,
    /// Review inboxes reuse one row contract.
    pub inbox_reuses_one_row_contract: bool,
    /// Review switchers reuse one row contract.
    pub switcher_reuses_one_row_contract: bool,
    /// Companion queues reuse one row contract.
    pub companion_queue_reuses_one_row_contract: bool,
    /// Handoff packets reuse one row contract.
    pub handoff_packet_reuses_one_row_contract: bool,
    /// The row distinguishes local estimate, provider-backed, and offline/exported.
    pub row_distinguishes_local_provider_offline: bool,
    /// CLI / headless shows row truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows row truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows row truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows row truth.
    pub help_about_shows_truth: bool,
}

impl ReviewRequestRowConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_list_reuses_one_row_contract
            && self.inbox_reuses_one_row_contract
            && self.switcher_reuses_one_row_contract
            && self.companion_queue_reuses_one_row_contract
            && self.handoff_packet_reuses_one_row_contract
            && self.row_distinguishes_local_provider_offline
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRequestRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ReviewRequestRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewRequestRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review-request rows.
    pub rows: Vec<ReviewRequestRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewRequestRowDowngradeTrigger>,
    /// Consumer surfaces that must reuse this row contract.
    pub consumer_surfaces: Vec<ReviewRequestRowConsumerSurface>,
    /// Trust review block.
    pub trust_review: ReviewRequestRowTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewRequestRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewRequestRowProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe review-request row packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewRequestRowPacket {
    /// Record kind; must equal [`REVIEW_REQUEST_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REVIEW_REQUEST_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Review-request rows.
    pub rows: Vec<ReviewRequestRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewRequestRowDowngradeTrigger>,
    /// Consumer surfaces that must reuse this row contract.
    pub consumer_surfaces: Vec<ReviewRequestRowConsumerSurface>,
    /// Trust review block.
    pub trust_review: ReviewRequestRowTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewRequestRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewRequestRowProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ReviewRequestRowPacket {
    /// Builds a review-request row packet from stable-lane input.
    pub fn new(input: ReviewRequestRowPacketInput) -> Self {
        Self {
            record_kind: REVIEW_REQUEST_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_REQUEST_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            rows: input.rows,
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

    /// Validates the review-request row invariants.
    pub fn validate(&self) -> Vec<ReviewRequestRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REVIEW_REQUEST_ROW_RECORD_KIND {
            violations.push(ReviewRequestRowViolation::WrongRecordKind);
        }
        if self.schema_version != REVIEW_REQUEST_ROW_SCHEMA_VERSION {
            violations.push(ReviewRequestRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ReviewRequestRowViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ReviewRequestRowViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ReviewRequestRowViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ReviewRequestRowViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ReviewRequestRowViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ReviewRequestRowViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review-request row packet serializes"),
        ) {
            violations.push(ReviewRequestRowViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("review-request row packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let provider_backed = self
            .rows
            .iter()
            .filter(|row| row.backing_kind == ReviewRequestBackingKind::ProviderBackedRequest)
            .count();
        let local_estimates = self
            .rows
            .iter()
            .filter(|row| row.backing_kind == ReviewRequestBackingKind::LocalReviewEstimate)
            .count();
        let stale_rows = self
            .rows
            .iter()
            .filter(|row| row.base_head_freshness.is_stale())
            .count();

        let mut out = String::new();
        out.push_str("# Review-Request Rows: Provider/Local-vs-Provider Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Rows: {} ({} provider-backed, {} local estimates, {} showing a stale relation)\n",
            self.rows.len(),
            provider_backed,
            local_estimates,
            stale_rows
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} vs {} — base/head `{}`, stack `{}`, scope `{}`, provider freshness `{}`\n",
                row.object_id_label,
                row.backing_kind.as_str(),
                row.provider_identity_label,
                row.head_ref_label,
                row.base_head_freshness.as_str(),
                row.stack_relation.as_str(),
                row.scope.as_str(),
                row.provider_freshness.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in review-request row export.
#[derive(Debug)]
pub enum ReviewRequestRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewRequestRowViolation>),
}

impl fmt::Display for ReviewRequestRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "review-request row export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "review-request row export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ReviewRequestRowArtifactError {}

/// Validation failures emitted by [`ReviewRequestRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewRequestRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No rows are present.
    RowsMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row misrepresents hosted status relative to its backing kind.
    HostedStatusMisrepresented,
    /// A row that needs an explicit browser-handoff boundary is missing it.
    BrowserHandoffBoundaryMissing,
    /// A row that must preserve a local-continue fallback is missing it.
    LocalContinueFallbackMissing,
    /// A provider-backed row forces raw-provider navigation for ordinary triage.
    ForcedRawProviderNavigation,
    /// The row set does not cover local estimate, provider-backed, and offline/exported kinds.
    BackingKindCoverageMissing,
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

impl ReviewRequestRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsMissing => "rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::HostedStatusMisrepresented => "hosted_status_misrepresented",
            Self::BrowserHandoffBoundaryMissing => "browser_handoff_boundary_missing",
            Self::LocalContinueFallbackMissing => "local_continue_fallback_missing",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::BackingKindCoverageMissing => "backing_kind_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable review-request row export.
pub fn current_review_request_row_export(
) -> Result<ReviewRequestRowPacket, ReviewRequestRowArtifactError> {
    let packet: ReviewRequestRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth/support_export.json"
    )))
    .map_err(ReviewRequestRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewRequestRowArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ReviewRequestRowPacket,
    violations: &mut Vec<ReviewRequestRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REVIEW_REQUEST_ROW_SCHEMA_REF,
        REVIEW_REQUEST_ROW_DOC_REF,
        REVIEW_REQUEST_ROW_COMPONENT_MATRIX_CONTRACT_REF,
        REVIEW_REQUEST_ROW_REVIEW_WORKSPACE_CONTRACT_REF,
        REVIEW_REQUEST_ROW_MERGE_QUEUE_CONTRACT_REF,
        REVIEW_REQUEST_ROW_CHANGE_LINEAGE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ReviewRequestRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(packet: &ReviewRequestRowPacket, violations: &mut Vec<ReviewRequestRowViolation>) {
    if packet.rows.is_empty() {
        violations.push(ReviewRequestRowViolation::RowsMissing);
        return;
    }

    let mut present: BTreeSet<ReviewRequestBackingKind> = BTreeSet::new();

    for row in &packet.rows {
        present.insert(row.backing_kind);

        if row.row_id.trim().is_empty()
            || row.provider_identity_label.trim().is_empty()
            || row.object_id_label.trim().is_empty()
            || row.base_ref_label.trim().is_empty()
            || row.head_ref_label.trim().is_empty()
            || row.actions.is_empty()
            || row.row_fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(ReviewRequestRowViolation::RowIncomplete);
        }

        let disclosure = row.disclosure();

        if row.claims_provider_backed != disclosure.asserts_hosted_status {
            violations.push(ReviewRequestRowViolation::HostedStatusMisrepresented);
        }
        if disclosure.needs_browser_handoff_boundary
            && row.browser_handoff_boundary.trim().is_empty()
        {
            violations.push(ReviewRequestRowViolation::BrowserHandoffBoundaryMissing);
        }
        if disclosure.needs_local_continue_fallback && row.local_continue_fallback.trim().is_empty()
        {
            violations.push(ReviewRequestRowViolation::LocalContinueFallbackMissing);
        }
        if row.backing_kind == ReviewRequestBackingKind::ProviderBackedRequest
            && !row.has_in_product_action()
        {
            violations.push(ReviewRequestRowViolation::ForcedRawProviderNavigation);
        }
    }

    for required in [
        ReviewRequestBackingKind::LocalReviewEstimate,
        ReviewRequestBackingKind::ProviderBackedRequest,
        ReviewRequestBackingKind::OfflineExportedPacket,
    ] {
        if !present.contains(&required) {
            violations.push(ReviewRequestRowViolation::BackingKindCoverageMissing);
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
