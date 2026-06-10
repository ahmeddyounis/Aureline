//! Durable review-workspace headers, local-CI parity, and stable anchor rehydration.
//!
//! This module implements the canonical M5 truth packet for keeping a local
//! review workspace honest across edits, rebases, and reopens. It binds three
//! pillars into one export-safe record:
//!
//! - **Durable review-workspace headers** — each
//!   [`DurableReviewHeaderRow`] carries a workspace's target identity, durable
//!   anchor id, base freshness, approval state, and mutation authority, so the
//!   header surface always shows what is under review and how fresh it is.
//! - **Local-CI parity** — each [`LocalCiParityLaneRow`] records how a local
//!   check result lines up with the CI expectation it mirrors, labeling
//!   divergence rather than hiding it.
//! - **Stable anchor rehydration** — each [`AnchorRehydrationRow`] records the
//!   outcome of re-attaching a durable anchor to its target after an edit,
//!   rebase, reopen, base change, or external sync, relabeling drift rather than
//!   silently dropping the anchor.
//!
//! The packet references upstream review-workspace, review-pack, anchor
//! stability, and pipeline-run contracts by id rather than embedding their
//! content. Raw diff bodies, raw build logs, raw pipeline artifacts, raw
//! provider payloads, credentials, and live provider responses stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/review/implement-durable-review-workspace-headers-local-ci-parity-and-stable-anchor-rehydration.schema.json`](../../../../schemas/review/implement-durable-review-workspace-headers-local-ci-parity-and-stable-anchor-rehydration.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md`](../../../../docs/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/`](../../../../fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DurableReviewHeaderPacket`].
pub const DURABLE_REVIEW_HEADER_RECORD_KIND: &str =
    "durable_review_header_local_ci_parity_and_anchor_rehydration";

/// Schema version for durable review-header records.
pub const DURABLE_REVIEW_HEADER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DURABLE_REVIEW_HEADER_SCHEMA_REF: &str =
    "schemas/review/implement-durable-review-workspace-headers-local-ci-parity-and-stable-anchor-rehydration.schema.json";

/// Repo-relative path of the durable review-header contract doc.
pub const DURABLE_REVIEW_HEADER_DOC_REF: &str =
    "docs/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md";

/// Repo-relative path of the review-workspace anchor contract this lane builds on.
pub const DURABLE_REVIEW_HEADER_REVIEW_WORKSPACE_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the review-pack contract that anchors local-CI parity.
pub const DURABLE_REVIEW_HEADER_REVIEW_PACK_CONTRACT_REF: &str =
    "schemas/review/review_pack.schema.json";

/// Repo-relative path of the anchor-stability contract reused for rehydration.
pub const DURABLE_REVIEW_HEADER_ANCHOR_STABILITY_CONTRACT_REF: &str =
    "schemas/review/review_stabilization.schema.json";

/// Repo-relative path of the pipeline-run contract that local-CI parity mirrors.
pub const DURABLE_REVIEW_HEADER_PIPELINE_RUN_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const DURABLE_REVIEW_HEADER_FIXTURE_DIR: &str =
    "fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration";

/// Repo-relative path of the checked support-export artifact.
pub const DURABLE_REVIEW_HEADER_ARTIFACT_REF: &str =
    "artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DURABLE_REVIEW_HEADER_SUMMARY_REF: &str =
    "artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md";

/// Base-freshness class shown on a durable review-workspace header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderFreshnessClass {
    /// Header is current against its base.
    Current,
    /// Base advanced under the review; the header shows a stale-base label.
    StaleBase,
    /// The diff is outdated relative to the working tree.
    OutdatedDiff,
    /// Local and base histories have diverged.
    Diverged,
    /// Freshness cannot be computed (for example, offline).
    Unknown,
}

impl HeaderFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleBase => "stale_base",
            Self::OutdatedDiff => "outdated_diff",
            Self::Diverged => "diverged",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this class represents a stale or diverged header that must be labeled.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::StaleBase | Self::OutdatedDiff | Self::Diverged)
    }
}

/// Approval state carried by a durable review-workspace header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewApprovalState {
    /// No review verdict has been recorded yet.
    NotReviewed,
    /// The change is approved against the current base.
    Approved,
    /// Changes were requested.
    ChangesRequested,
    /// Approval was invalidated and reset because the base changed.
    ResetOnBaseChange,
}

impl ReviewApprovalState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotReviewed => "not_reviewed",
            Self::Approved => "approved",
            Self::ChangesRequested => "changes_requested",
            Self::ResetOnBaseChange => "reset_on_base_change",
        }
    }
}

/// Outcome of re-attaching a durable anchor to its target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorRehydrationState {
    /// Anchor re-attached to the exact original location.
    RehydratedExact,
    /// Anchor re-attached after shifting to track moved content.
    RehydratedShifted,
    /// Anchor drifted off its target and was relabeled rather than dropped.
    DriftedRelabeled,
    /// Anchor target disappeared; the anchor is flagged as orphaned.
    OrphanedFlagged,
    /// Rehydration has not been attempted yet.
    Pending,
}

impl AnchorRehydrationState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RehydratedExact => "rehydrated_exact",
            Self::RehydratedShifted => "rehydrated_shifted",
            Self::DriftedRelabeled => "drifted_relabeled",
            Self::OrphanedFlagged => "orphaned_flagged",
            Self::Pending => "pending",
        }
    }

    /// Whether this state requires an explicit drift label.
    pub const fn requires_drift_label(self) -> bool {
        matches!(self, Self::DriftedRelabeled | Self::OrphanedFlagged)
    }
}

/// Mutation authority a header surface may exercise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationAuthorityClass {
    /// Header is read-only and never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Header may trigger writes that stay individually attributable and reviewable.
    AttributableWrite,
}

impl MutationAuthorityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::AttributableWrite => "attributable_write",
        }
    }
}

/// Verdict comparing a local check result with the CI expectation it mirrors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCiParityVerdict {
    /// Local result matches the CI expectation.
    ParityMatch,
    /// Local result is advisory only; CI has not reported yet.
    LocalOnlyAdvisory,
    /// Local and CI results diverge and the divergence is labeled.
    DivergenceLabeled,
    /// Parity cannot be computed because CI status is unavailable (for example, offline).
    UnavailableOffline,
}

impl LocalCiParityVerdict {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityMatch => "parity_match",
            Self::LocalOnlyAdvisory => "local_only_advisory",
            Self::DivergenceLabeled => "divergence_labeled",
            Self::UnavailableOffline => "unavailable_offline",
        }
    }

    /// Whether this verdict requires a non-empty divergence label.
    pub const fn requires_divergence_label(self) -> bool {
        matches!(self, Self::DivergenceLabeled)
    }
}

/// Enforcement class for a local-CI parity check lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckEnforcementClass {
    /// A required check that gates landing.
    Required,
    /// An advisory check that informs but does not gate.
    Advisory,
    /// An informational signal only.
    Informational,
}

impl CheckEnforcementClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Advisory => "advisory",
            Self::Informational => "informational",
        }
    }
}

/// Event that triggered an anchor rehydration attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RehydrationTrigger {
    /// A local edit moved content under the anchor.
    Edit,
    /// A rebase rewrote the history under the anchor.
    Rebase,
    /// The review workspace was reopened.
    Reopen,
    /// The review base changed.
    BaseChange,
    /// An external sync pulled new content.
    ExternalSync,
}

impl RehydrationTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Edit => "edit",
            Self::Rebase => "rebase",
            Self::Reopen => "reopen",
            Self::BaseChange => "base_change",
            Self::ExternalSync => "external_sync",
        }
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableReviewHeaderDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A durable anchor drifted off its target.
    AnchorDrift,
    /// Local-CI divergence was surfaced without an explicit label.
    ParityDivergenceUnlabeled,
    /// A stale base was surfaced without an explicit label.
    StaleBaseUnlabeled,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified review boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl DurableReviewHeaderDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::AnchorDrift,
        Self::ParityDivergenceUnlabeled,
        Self::StaleBaseUnlabeled,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::AnchorDrift => "anchor_drift",
            Self::ParityDivergenceUnlabeled => "parity_divergence_unlabeled",
            Self::StaleBaseUnlabeled => "stale_base_unlabeled",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableReviewHeaderConsumerSurface {
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Merge-queue panel.
    MergeQueuePanel,
    /// Pipeline viewer.
    PipelineViewer,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl DurableReviewHeaderConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReviewWorkspaceHeader,
        Self::MergeQueuePanel,
        Self::PipelineViewer,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::MergeQueuePanel => "merge_queue_panel",
            Self::PipelineViewer => "pipeline_viewer",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One durable review-workspace header row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableReviewHeaderRow {
    /// Stable header id.
    pub header_id: String,
    /// Human-readable target identity (what is under review).
    pub target_identity_label: String,
    /// Durable anchor id that survives edits, rebases, and reopens.
    pub durable_anchor_id: String,
    /// Base freshness class shown on the header.
    pub base_freshness: HeaderFreshnessClass,
    /// Approval state shown on the header.
    pub approval_state: ReviewApprovalState,
    /// Mutation authority the header may exercise.
    pub mutation_authority: MutationAuthorityClass,
    /// Header fields the surface projects, in display order.
    pub header_fields_shown: Vec<String>,
    /// Source contract refs consumed by this header.
    pub source_contract_refs: Vec<String>,
}

/// One local-CI parity lane row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalCiParityLaneRow {
    /// Stable check id shared by the local run and the CI expectation.
    pub check_id: String,
    /// Enforcement class for this check.
    pub enforcement: CheckEnforcementClass,
    /// Parity verdict comparing the local result with the CI expectation.
    pub verdict: LocalCiParityVerdict,
    /// Human-readable local result label.
    pub local_result_label: String,
    /// Human-readable CI expectation label.
    pub ci_expectation_label: String,
    /// Divergence label; required and non-empty when the verdict is `divergence_labeled`.
    pub divergence_label: String,
}

/// One stable anchor rehydration row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnchorRehydrationRow {
    /// Durable anchor id being rehydrated.
    pub anchor_id: String,
    /// Event that triggered the rehydration attempt.
    pub trigger: RehydrationTrigger,
    /// Resulting rehydration state.
    pub resulting_state: AnchorRehydrationState,
    /// Drift label; required and non-empty when the state requires it.
    pub drift_label: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableReviewHeaderTrustReview {
    /// Durable anchors rehydrate across edits, rebases, and reopens.
    pub anchors_rehydrate_durably: bool,
    /// Anchor drift is labeled, never silently dropped.
    pub anchor_drift_labeled_not_hidden: bool,
    /// Stale-base and outdated-diff states are labeled explicitly.
    pub stale_base_labeled_explicit: bool,
    /// Approval state resets when the base changes.
    pub approval_resets_on_base_change: bool,
    /// Local-CI parity verdicts are explicit, never implied.
    pub local_ci_parity_explicit: bool,
    /// Local-CI divergence is labeled, never silently hidden.
    pub divergence_labeled_not_hidden: bool,
    /// Header target identity is explicit.
    pub target_identity_explicit: bool,
    /// No header surface creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl DurableReviewHeaderTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.anchors_rehydrate_durably
            && self.anchor_drift_labeled_not_hidden
            && self.stale_base_labeled_explicit
            && self.approval_resets_on_base_change
            && self.local_ci_parity_explicit
            && self.divergence_labeled_not_hidden
            && self.target_identity_explicit
            && self.no_hidden_write_scope
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableReviewHeaderConsumerProjection {
    /// Header shows anchor and target identity.
    pub header_shows_anchor_and_target_identity: bool,
    /// Header shows base freshness truth.
    pub header_shows_base_freshness: bool,
    /// Header shows approval state including reset-on-base-change.
    pub header_shows_approval_state: bool,
    /// Parity surface shows the local-vs-CI verdict.
    pub parity_shows_local_vs_ci_verdict: bool,
    /// Rehydration surface shows drift labels.
    pub rehydration_shows_drift_label: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_truth: bool,
    /// Preview / Labs lanes are labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified: bool,
}

impl DurableReviewHeaderConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.header_shows_anchor_and_target_identity
            && self.header_shows_base_freshness
            && self.header_shows_approval_state
            && self.parity_shows_local_vs_ci_verdict
            && self.rehydration_shows_drift_label
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.preview_labs_label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableReviewHeaderProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`DurableReviewHeaderPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableReviewHeaderPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Durable review-workspace header rows.
    pub headers: Vec<DurableReviewHeaderRow>,
    /// Local-CI parity lane rows.
    pub parity_lanes: Vec<LocalCiParityLaneRow>,
    /// Stable anchor rehydration rows.
    pub rehydration_events: Vec<AnchorRehydrationRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<DurableReviewHeaderDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<DurableReviewHeaderConsumerSurface>,
    /// Trust review block.
    pub trust_review: DurableReviewHeaderTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DurableReviewHeaderConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DurableReviewHeaderProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe durable review-header, local-CI parity, and anchor-rehydration packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableReviewHeaderPacket {
    /// Record kind; must equal [`DURABLE_REVIEW_HEADER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DURABLE_REVIEW_HEADER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Durable review-workspace header rows.
    pub headers: Vec<DurableReviewHeaderRow>,
    /// Local-CI parity lane rows.
    pub parity_lanes: Vec<LocalCiParityLaneRow>,
    /// Stable anchor rehydration rows.
    pub rehydration_events: Vec<AnchorRehydrationRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<DurableReviewHeaderDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<DurableReviewHeaderConsumerSurface>,
    /// Trust review block.
    pub trust_review: DurableReviewHeaderTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DurableReviewHeaderConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DurableReviewHeaderProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DurableReviewHeaderPacket {
    /// Builds a durable review-header packet from stable-lane input.
    pub fn new(input: DurableReviewHeaderPacketInput) -> Self {
        Self {
            record_kind: DURABLE_REVIEW_HEADER_RECORD_KIND.to_owned(),
            schema_version: DURABLE_REVIEW_HEADER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            headers: input.headers,
            parity_lanes: input.parity_lanes,
            rehydration_events: input.rehydration_events,
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

    /// Validates the durable review-header invariants.
    pub fn validate(&self) -> Vec<DurableReviewHeaderViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DURABLE_REVIEW_HEADER_RECORD_KIND {
            violations.push(DurableReviewHeaderViolation::WrongRecordKind);
        }
        if self.schema_version != DURABLE_REVIEW_HEADER_SCHEMA_VERSION {
            violations.push(DurableReviewHeaderViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DurableReviewHeaderViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(DurableReviewHeaderViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(DurableReviewHeaderViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_headers(self, &mut violations);
        validate_parity_lanes(self, &mut violations);
        validate_rehydration(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(DurableReviewHeaderViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(DurableReviewHeaderViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(DurableReviewHeaderViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("durable review-header packet serializes"),
        ) {
            violations.push(DurableReviewHeaderViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("durable review-header packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_headers = self
            .headers
            .iter()
            .filter(|row| row.base_freshness.is_stale())
            .count();
        let labeled_divergences = self
            .parity_lanes
            .iter()
            .filter(|row| row.verdict == LocalCiParityVerdict::DivergenceLabeled)
            .count();
        let drifted_anchors = self
            .rehydration_events
            .iter()
            .filter(|row| row.resulting_state.requires_drift_label())
            .count();

        let mut out = String::new();
        out.push_str("# Durable Review Headers, Local-CI Parity, and Anchor Rehydration\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Headers: {} ({} showing a stale-base or diverged label)\n",
            self.headers.len(),
            stale_headers
        ));
        out.push_str(&format!(
            "- Parity lanes: {} ({} labeled divergences)\n",
            self.parity_lanes.len(),
            labeled_divergences
        ));
        out.push_str(&format!(
            "- Rehydration events: {} ({} drifted or orphaned and relabeled)\n",
            self.rehydration_events.len(),
            drifted_anchors
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Headers\n\n");
        for row in &self.headers {
            out.push_str(&format!(
                "- **{}** → anchor `{}`: base `{}`, approval `{}`, authority `{}`\n",
                row.target_identity_label,
                row.durable_anchor_id,
                row.base_freshness.as_str(),
                row.approval_state.as_str(),
                row.mutation_authority.as_str()
            ));
        }

        out.push_str("\n## Local-CI parity\n\n");
        for row in &self.parity_lanes {
            out.push_str(&format!(
                "- `{}` ({}): {}\n",
                row.check_id,
                row.enforcement.as_str(),
                row.verdict.as_str()
            ));
        }

        out.push_str("\n## Anchor rehydration\n\n");
        for row in &self.rehydration_events {
            out.push_str(&format!(
                "- `{}` on {}: {}\n",
                row.anchor_id,
                row.trigger.as_str(),
                row.resulting_state.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in durable review-header export.
#[derive(Debug)]
pub enum DurableReviewHeaderArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DurableReviewHeaderViolation>),
}

impl fmt::Display for DurableReviewHeaderArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "durable review-header export parse failed: {error}"
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
                    "durable review-header export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DurableReviewHeaderArtifactError {}

/// Validation failures emitted by [`DurableReviewHeaderPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurableReviewHeaderViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No header rows are present.
    HeadersMissing,
    /// A header row is incomplete.
    HeaderRowIncomplete,
    /// No local-CI parity lanes are present.
    ParityLanesMissing,
    /// A parity lane row is incomplete.
    ParityLaneIncomplete,
    /// A labeled divergence is missing its divergence label.
    DivergenceLabelMissing,
    /// No required parity lane is present.
    RequiredParityLaneMissing,
    /// No rehydration rows are present.
    RehydrationEventsMissing,
    /// A rehydration row is incomplete.
    RehydrationRowIncomplete,
    /// A drifted or orphaned anchor is missing its drift label.
    DriftLabelMissing,
    /// A header's durable anchor has no rehydration record.
    HeaderAnchorMissingRehydration,
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

impl DurableReviewHeaderViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::HeadersMissing => "headers_missing",
            Self::HeaderRowIncomplete => "header_row_incomplete",
            Self::ParityLanesMissing => "parity_lanes_missing",
            Self::ParityLaneIncomplete => "parity_lane_incomplete",
            Self::DivergenceLabelMissing => "divergence_label_missing",
            Self::RequiredParityLaneMissing => "required_parity_lane_missing",
            Self::RehydrationEventsMissing => "rehydration_events_missing",
            Self::RehydrationRowIncomplete => "rehydration_row_incomplete",
            Self::DriftLabelMissing => "drift_label_missing",
            Self::HeaderAnchorMissingRehydration => "header_anchor_missing_rehydration",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable durable review-header export.
pub fn current_durable_review_header_export(
) -> Result<DurableReviewHeaderPacket, DurableReviewHeaderArtifactError> {
    let packet: DurableReviewHeaderPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/support_export.json"
    )))
    .map_err(DurableReviewHeaderArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DurableReviewHeaderArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &DurableReviewHeaderPacket,
    violations: &mut Vec<DurableReviewHeaderViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        DURABLE_REVIEW_HEADER_SCHEMA_REF,
        DURABLE_REVIEW_HEADER_DOC_REF,
        DURABLE_REVIEW_HEADER_REVIEW_WORKSPACE_CONTRACT_REF,
        DURABLE_REVIEW_HEADER_REVIEW_PACK_CONTRACT_REF,
        DURABLE_REVIEW_HEADER_ANCHOR_STABILITY_CONTRACT_REF,
        DURABLE_REVIEW_HEADER_PIPELINE_RUN_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DurableReviewHeaderViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_headers(
    packet: &DurableReviewHeaderPacket,
    violations: &mut Vec<DurableReviewHeaderViolation>,
) {
    if packet.headers.is_empty() {
        violations.push(DurableReviewHeaderViolation::HeadersMissing);
        return;
    }

    let rehydrated_anchors: BTreeSet<&str> = packet
        .rehydration_events
        .iter()
        .map(|row| row.anchor_id.as_str())
        .collect();

    for row in &packet.headers {
        if row.header_id.trim().is_empty()
            || row.target_identity_label.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.header_fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(DurableReviewHeaderViolation::HeaderRowIncomplete);
        }
        if !row.durable_anchor_id.trim().is_empty()
            && !rehydrated_anchors.contains(row.durable_anchor_id.as_str())
        {
            violations.push(DurableReviewHeaderViolation::HeaderAnchorMissingRehydration);
        }
    }
}

fn validate_parity_lanes(
    packet: &DurableReviewHeaderPacket,
    violations: &mut Vec<DurableReviewHeaderViolation>,
) {
    if packet.parity_lanes.is_empty() {
        violations.push(DurableReviewHeaderViolation::ParityLanesMissing);
        return;
    }

    let mut has_required = false;
    for row in &packet.parity_lanes {
        if row.check_id.trim().is_empty()
            || row.local_result_label.trim().is_empty()
            || row.ci_expectation_label.trim().is_empty()
        {
            violations.push(DurableReviewHeaderViolation::ParityLaneIncomplete);
        }
        if row.verdict.requires_divergence_label() && row.divergence_label.trim().is_empty() {
            violations.push(DurableReviewHeaderViolation::DivergenceLabelMissing);
        }
        if row.enforcement == CheckEnforcementClass::Required {
            has_required = true;
        }
    }
    if !has_required {
        violations.push(DurableReviewHeaderViolation::RequiredParityLaneMissing);
    }
}

fn validate_rehydration(
    packet: &DurableReviewHeaderPacket,
    violations: &mut Vec<DurableReviewHeaderViolation>,
) {
    if packet.rehydration_events.is_empty() {
        violations.push(DurableReviewHeaderViolation::RehydrationEventsMissing);
        return;
    }

    for row in &packet.rehydration_events {
        if row.anchor_id.trim().is_empty() {
            violations.push(DurableReviewHeaderViolation::RehydrationRowIncomplete);
        }
        if row.resulting_state.requires_drift_label() && row.drift_label.trim().is_empty() {
            violations.push(DurableReviewHeaderViolation::DriftLabelMissing);
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
