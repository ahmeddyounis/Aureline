//! Frozen M5 review-request, checks, merge-readiness, and merge-queue component matrix.
//!
//! This module locks the canonical M5 component truth for seven reusable review
//! surfaces — review-request rows, checks-summary cards, pending-review trays,
//! merge-readiness panels, merge-queue entries, stack-dependency chips, and
//! approval-invalidation banners — into one export-safe packet. Each
//! [`M5ReviewComponentMatrixRow`] binds a component to its maturity class, the
//! exact provider-versus-local-estimate distinction it must preserve, its
//! stale-provider downgrade vocabulary, its browser-handoff boundary, its
//! local-continue fallback, required evidence packet refs, downgrade triggers,
//! rollback posture, source contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether every claimed M5 review
//! surface may consume one shared component family instead of private row text or
//! provider-specific badges. It references upstream review-workspace, checks,
//! pending-review, merge-readiness, merge-queue, stack-dependency, and
//! approval-invalidation contracts by id rather than embedding their content. Raw
//! diff bodies, raw check logs, raw provider payloads, credentials, and live
//! provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-review-request-check-queue-component-matrix.schema.json`](../../../../schemas/ui/m5-review-request-check-queue-component-matrix.schema.json).
//! The contract doc is
//! [`docs/review/m5/freeze_the_m5_review_request_check_and_merge_queue_component_matrix.md`](../../../../docs/review/m5/freeze_the_m5_review_request_check_and_merge_queue_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-review-request-check-queue-components/`](../../../../fixtures/ui/m5-review-request-check-queue-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReviewComponentMatrixPacket`].
pub const M5_REVIEW_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_review_request_check_and_merge_queue_component_matrix";

/// Schema version for M5 review-component matrix records.
pub const M5_REVIEW_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_REVIEW_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the M5 review-component matrix contract doc.
pub const M5_REVIEW_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/review/m5/freeze_the_m5_review_request_check_and_merge_queue_component_matrix.md";

/// Repo-relative path of the frozen review-request row (review-workspace) contract.
pub const M5_REVIEW_COMPONENT_MATRIX_REVIEW_REQUEST_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the frozen checks-summary (pipeline-run) contract.
pub const M5_REVIEW_COMPONENT_MATRIX_CHECKS_SUMMARY_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the frozen pending-review-tray (review-surface) contract.
pub const M5_REVIEW_COMPONENT_MATRIX_PENDING_TRAY_CONTRACT_REF: &str =
    "schemas/review/review_surface_record.schema.json";

/// Repo-relative path of the frozen merge-readiness (landing-candidate) contract.
pub const M5_REVIEW_COMPONENT_MATRIX_MERGE_READINESS_CONTRACT_REF: &str =
    "schemas/review/landing_candidate.schema.json";

/// Repo-relative path of the frozen merge-queue entry contract.
pub const M5_REVIEW_COMPONENT_MATRIX_MERGE_QUEUE_CONTRACT_REF: &str =
    "schemas/review/merge_queue_entry.schema.json";

/// Repo-relative path of the frozen stack-dependency (change-lineage) contract.
pub const M5_REVIEW_COMPONENT_MATRIX_STACK_DEPENDENCY_CONTRACT_REF: &str =
    "schemas/review/change_lineage.schema.json";

/// Repo-relative path of the frozen approval-invalidation contract.
pub const M5_REVIEW_COMPONENT_MATRIX_APPROVAL_INVALIDATION_CONTRACT_REF: &str =
    "schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REVIEW_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-review-request-check-queue-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REVIEW_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-review-request-check-queue-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_REVIEW_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/release/m5-review-request-check-queue-proof/summary.md";

/// One of the seven M5 reusable review components governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponent {
    /// Review-request row listing a provider-linked pull/merge request.
    ReviewRequestRow,
    /// Checks-summary card rolling up check-class truth and freshness.
    ChecksSummaryCard,
    /// Pending-review tray listing reviews awaiting the current owner.
    PendingReviewTray,
    /// Merge-readiness panel summarizing blocking state and ownership.
    MergeReadinessPanel,
    /// Merge-queue entry with queue owner and position truth.
    MergeQueueEntry,
    /// Stack-dependency chip showing stack relation and parent blocking.
    StackDependencyChip,
    /// Approval-invalidation banner naming why approvals were recomputed.
    ApprovalInvalidationBanner,
}

impl M5ReviewComponent {
    /// Every component, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReviewRequestRow,
        Self::ChecksSummaryCard,
        Self::PendingReviewTray,
        Self::MergeReadinessPanel,
        Self::MergeQueueEntry,
        Self::StackDependencyChip,
        Self::ApprovalInvalidationBanner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewRequestRow => "review_request_row",
            Self::ChecksSummaryCard => "checks_summary_card",
            Self::PendingReviewTray => "pending_review_tray",
            Self::MergeReadinessPanel => "merge_readiness_panel",
            Self::MergeQueueEntry => "merge_queue_entry",
            Self::StackDependencyChip => "stack_dependency_chip",
            Self::ApprovalInvalidationBanner => "approval_invalidation_banner",
        }
    }
}

/// Maturity class for an M5 review component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentMaturityClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5ReviewComponentMaturityClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence requirement level for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this component's current maturity.
    NotApplicable,
}

impl M5ReviewComponentEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Stale-provider downgrade vocabulary that every component must preserve.
///
/// This vocabulary names the provider-freshness posture explicitly so a stale or
/// unreachable provider is never flattened into a local estimate or hidden behind
/// a generic warning pill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentStaleProviderState {
    /// Provider-backed truth is fresh within the freshness bound.
    ProviderFresh,
    /// Provider truth is refreshing; last-known state is labeled as refreshing.
    ProviderRefreshing,
    /// Provider truth is stale relative to the head or base it gates.
    ProviderStale,
    /// Provider is unreachable; only local estimate is available.
    ProviderUnreachable,
    /// Provider and local truth disagree and the conflict is surfaced.
    ProviderConflict,
    /// Local-only continuation is offered while provider freshness is degraded.
    LocalOnlyContinuation,
}

impl M5ReviewComponentStaleProviderState {
    /// Every stale-provider state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderFresh,
        Self::ProviderRefreshing,
        Self::ProviderStale,
        Self::ProviderUnreachable,
        Self::ProviderConflict,
        Self::LocalOnlyContinuation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderFresh => "provider_fresh",
            Self::ProviderRefreshing => "provider_refreshing",
            Self::ProviderStale => "provider_stale",
            Self::ProviderUnreachable => "provider_unreachable",
            Self::ProviderConflict => "provider_conflict",
            Self::LocalOnlyContinuation => "local_only_continuation",
        }
    }
}

/// Downgrade trigger that can narrow a component below its claimed maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// Approvals were invalidated and must be recomputed.
    ApprovalInvalidated,
    /// A stack parent is blocked, blocking this component's change.
    StackParentBlocked,
    /// Merge-queue ownership is unresolved.
    QueueOwnershipUnresolved,
    /// A check class could not be verified.
    CheckClassUnverified,
    /// Browser handoff for provider deep links is unavailable.
    BrowserHandoffUnavailable,
    /// Component trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified review-component boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ReviewComponentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::ApprovalInvalidated,
        Self::StackParentBlocked,
        Self::QueueOwnershipUnresolved,
        Self::CheckClassUnverified,
        Self::BrowserHandoffUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::ApprovalInvalidated => "approval_invalidated",
            Self::StackParentBlocked => "stack_parent_blocked",
            Self::QueueOwnershipUnresolved => "queue_ownership_unresolved",
            Self::CheckClassUnverified => "check_class_unverified",
            Self::BrowserHandoffUnavailable => "browser_handoff_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentRollbackPosture {
    /// Read-only component that never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Provider mutation stays individually attributable and reviewable.
    ProviderMutationAttributable,
    /// Local continuation is preserved when provider freshness is degraded.
    LocalContinuePreserved,
    /// Browser or provider handoff always preserves a safe return path to the IDE.
    ReturnPathPreserved,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the component's current maturity.
    NotApplicable,
}

impl M5ReviewComponentRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::ProviderMutationAttributable => "provider_mutation_attributable",
            Self::LocalContinuePreserved => "local_continue_preserved",
            Self::ReturnPathPreserved => "return_path_preserved",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentConsumerSurface {
    /// Review workspace surface.
    ReviewWorkspace,
    /// Merge-queue panel.
    MergeQueuePanel,
    /// Pending-review tray.
    PendingReviewTray,
    /// Merge-readiness panel.
    MergeReadinessPanel,
    /// Browser companion / handoff follow-up.
    BrowserCompanion,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl M5ReviewComponentConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewWorkspace,
        Self::MergeQueuePanel,
        Self::PendingReviewTray,
        Self::MergeReadinessPanel,
        Self::BrowserCompanion,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::MergeQueuePanel => "merge_queue_panel",
            Self::PendingReviewTray => "pending_review_tray",
            Self::MergeReadinessPanel => "merge_readiness_panel",
            Self::BrowserCompanion => "browser_companion",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 review-component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewComponentMatrixRow {
    /// Review component.
    pub component: M5ReviewComponent,
    /// Maturity class earned by this component.
    pub maturity: M5ReviewComponentMaturityClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Exact provider-versus-local-estimate distinction this component preserves.
    pub provider_local_distinction: String,
    /// Stale-provider downgrade vocabulary this component must preserve.
    pub stale_provider_downgrade_vocab: Vec<M5ReviewComponentStaleProviderState>,
    /// Browser-handoff boundary this component keeps explicit.
    pub browser_handoff_boundary: String,
    /// Local-continue fallback this component preserves when provider freshness degrades.
    pub local_continue_fallback: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5ReviewComponentEvidenceRequirement,
    /// Required evidence packet refs for this maturity.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ReviewComponentDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5ReviewComponentRollbackPosture,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this component.
    pub consumer_surfaces: Vec<M5ReviewComponentConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewComponentMatrixTrustReview {
    /// Provider-managed state is never flattened into a local estimate.
    pub provider_local_estimate_distinct: bool,
    /// Stale-provider downgrades are named, never hidden.
    pub stale_provider_downgrade_explicit: bool,
    /// Approval invalidation is never hidden behind a generic warning pill.
    pub approval_invalidation_never_generic_warning: bool,
    /// Browser handoff stays explicit with a safe return path.
    pub browser_handoff_explicit: bool,
    /// Local-only continuation is preserved when provider freshness is degraded.
    pub local_continue_preserved_on_degraded_freshness: bool,
    /// Stack blocking (parent blocked) stays explicit.
    pub stack_blocking_explicit: bool,
    /// Merge-queue ownership stays explicit.
    pub queue_ownership_explicit: bool,
    /// Check class stays explicit rather than collapsed into a single status.
    pub check_class_explicit: bool,
    /// Ordinary check triage never forces raw-provider navigation.
    pub no_forced_raw_provider_navigation_for_triage: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewComponentMatrixConsumerProjection {
    /// Review-request row shows provider identity and base/head relation.
    pub review_request_row_shows_provider_and_base_head: bool,
    /// Checks-summary card shows check class and freshness.
    pub checks_summary_card_shows_check_class_and_freshness: bool,
    /// Pending-review tray shows owner and local-versus-provider truth.
    pub pending_review_tray_shows_owner_and_local_provider: bool,
    /// Merge-readiness panel shows blocking state and ownership.
    pub merge_readiness_panel_shows_blocking_and_ownership: bool,
    /// Merge-queue entry shows queue owner and position.
    pub merge_queue_entry_shows_queue_owner_and_position: bool,
    /// Stack-dependency chip shows stack relation and blocking.
    pub stack_dependency_chip_shows_stack_relation_and_blocking: bool,
    /// Approval-invalidation banner shows the reason, never a generic warning.
    pub approval_invalidation_banner_shows_reason_not_generic: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_component_truth: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewComponentMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ReviewComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ReviewComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5ReviewComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 review-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewComponentMatrixPacket {
    /// Record kind; must equal [`M5_REVIEW_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REVIEW_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ReviewComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5ReviewComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReviewComponentMatrixPacket {
    /// Builds an M5 review-component matrix packet from stable-component input.
    pub fn new(input: M5ReviewComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_REVIEW_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_REVIEW_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 review-component matrix invariants.
    pub fn validate(&self) -> Vec<M5ReviewComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REVIEW_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5ReviewComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REVIEW_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ReviewComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReviewComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 review-component matrix packet serializes"),
        ) {
            violations.push(M5ReviewComponentMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 review-component matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.maturity.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Review-Request, Checks, and Merge-Queue Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Components: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component.as_str(),
                row.maturity.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Provider/local distinction: {}\n",
                row.provider_local_distinction
            ));
            out.push_str(&format!(
                "  - Browser-handoff boundary: {}\n",
                row.browser_handoff_boundary
            ));
            out.push_str(&format!(
                "  - Local-continue fallback: {}\n",
                row.local_continue_fallback
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 review-component matrix export.
#[derive(Debug)]
pub enum M5ReviewComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReviewComponentMatrixViolation>),
}

impl fmt::Display for M5ReviewComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 review-component matrix export parse failed: {error}"
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
                    "m5 review-component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReviewComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5ReviewComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReviewComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required component is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component claiming Stable is missing required evidence packet refs.
    StableComponentMissingEvidence,
    /// A component has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component does not name its provider-versus-local distinction.
    ProviderLocalDistinctionMissing,
    /// A component does not carry a stale-provider downgrade vocabulary.
    StaleProviderVocabMissing,
    /// A component does not name its browser-handoff boundary.
    BrowserHandoffBoundaryMissing,
    /// A component does not name its local-continue fallback.
    LocalContinueFallbackMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ReviewComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::StableComponentMissingEvidence => "stable_component_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ProviderLocalDistinctionMissing => "provider_local_distinction_missing",
            Self::StaleProviderVocabMissing => "stale_provider_vocab_missing",
            Self::BrowserHandoffBoundaryMissing => "browser_handoff_boundary_missing",
            Self::LocalContinueFallbackMissing => "local_continue_fallback_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 review-component matrix export.
pub fn current_stable_m5_review_component_matrix_export(
) -> Result<M5ReviewComponentMatrixPacket, M5ReviewComponentMatrixArtifactError> {
    let packet: M5ReviewComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-review-request-check-queue-proof/support_export.json"
    )))
    .map_err(M5ReviewComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReviewComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ReviewComponentMatrixPacket,
    violations: &mut Vec<M5ReviewComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REVIEW_COMPONENT_MATRIX_SCHEMA_REF,
        M5_REVIEW_COMPONENT_MATRIX_DOC_REF,
        M5_REVIEW_COMPONENT_MATRIX_REVIEW_REQUEST_CONTRACT_REF,
        M5_REVIEW_COMPONENT_MATRIX_CHECKS_SUMMARY_CONTRACT_REF,
        M5_REVIEW_COMPONENT_MATRIX_PENDING_TRAY_CONTRACT_REF,
        M5_REVIEW_COMPONENT_MATRIX_MERGE_READINESS_CONTRACT_REF,
        M5_REVIEW_COMPONENT_MATRIX_MERGE_QUEUE_CONTRACT_REF,
        M5_REVIEW_COMPONENT_MATRIX_STACK_DEPENDENCY_CONTRACT_REF,
        M5_REVIEW_COMPONENT_MATRIX_APPROVAL_INVALIDATION_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReviewComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_component_rows(
    packet: &M5ReviewComponentMatrixPacket,
    violations: &mut Vec<M5ReviewComponentMatrixViolation>,
) {
    let present: BTreeSet<M5ReviewComponent> = packet
        .component_rows
        .iter()
        .map(|row| row.component)
        .collect();
    for required in M5ReviewComponent::ALL {
        if !present.contains(&required) {
            violations.push(M5ReviewComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::ComponentRowIncomplete);
        }
        if row.maturity.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::StableComponentMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.provider_local_distinction.trim().is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::ProviderLocalDistinctionMissing);
        }
        if row.stale_provider_downgrade_vocab.is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::StaleProviderVocabMissing);
        }
        if row.browser_handoff_boundary.trim().is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::BrowserHandoffBoundaryMissing);
        }
        if row.local_continue_fallback.trim().is_empty() {
            violations.push(M5ReviewComponentMatrixViolation::LocalContinueFallbackMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5ReviewComponentMatrixPacket,
    violations: &mut Vec<M5ReviewComponentMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.provider_local_estimate_distinct,
        review.stale_provider_downgrade_explicit,
        review.approval_invalidation_never_generic_warning,
        review.browser_handoff_explicit,
        review.local_continue_preserved_on_degraded_freshness,
        review.stack_blocking_explicit,
        review.queue_ownership_explicit,
        review.check_class_explicit,
        review.no_forced_raw_provider_navigation_for_triage,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5ReviewComponentMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReviewComponentMatrixPacket,
    violations: &mut Vec<M5ReviewComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_request_row_shows_provider_and_base_head,
        projection.checks_summary_card_shows_check_class_and_freshness,
        projection.pending_review_tray_shows_owner_and_local_provider,
        projection.merge_readiness_panel_shows_blocking_and_ownership,
        projection.merge_queue_entry_shows_queue_owner_and_position,
        projection.stack_dependency_chip_shows_stack_relation_and_blocking,
        projection.approval_invalidation_banner_shows_reason_not_generic,
        projection.cli_headless_shows_component_truth,
        projection.support_export_shows_component_truth,
    ] {
        if !ok {
            violations.push(M5ReviewComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReviewComponentMatrixPacket,
    violations: &mut Vec<M5ReviewComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReviewComponentMatrixViolation::ProofFreshnessIncomplete);
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
