//! Frozen M5 review-workspace, merge-queue, and pipeline-viewer maturity matrix.
//!
//! This module locks the canonical M5 depth qualification for four review, CI,
//! and preview lanes — durable review-workspace anchors, fresh merge-queue
//! truth with attributable rerun/cancel authority, safe-previewed pipeline logs
//! and artifacts, and time-bounded, attributable remote preview routes — into
//! one export-safe packet. Each [`M5ReviewCiPreviewMatrixLaneRow`] binds a lane
//! to its qualification class, required evidence packet refs, downgrade
//! triggers, rollback posture, source contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether these lanes may ship as
//! Stable, Beta, Preview, or must narrow further. It references upstream
//! review-workspace, merge-queue, pipeline-run, and preview-route contracts by
//! id rather than embedding their content. Raw diff bodies, raw build logs, raw
//! pipeline artifacts, raw provider payloads, credentials, and live preview
//! origin responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json`](../../../../schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json).
//! The contract doc is
//! [`docs/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md`](../../../../docs/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/`](../../../../fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ReviewCiPreviewMatrixPacket`].
pub const M5_REVIEW_CI_PREVIEW_MATRIX_RECORD_KIND: &str =
    "freeze_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix";

/// Schema version for M5 review, CI, and preview maturity-matrix records.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_REF: &str =
    "schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json";

/// Repo-relative path of the M5 review, CI, and preview maturity-matrix contract doc.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_DOC_REF: &str =
    "docs/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md";

/// Repo-relative path of the frozen review-workspace boundary contract.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_REVIEW_WORKSPACE_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the frozen merge-queue entry contract.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_MERGE_QUEUE_CONTRACT_REF: &str =
    "schemas/review/merge_queue_entry.schema.json";

/// Repo-relative path of the frozen pipeline-run viewer contract.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_PIPELINE_VIEWER_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the frozen remote preview-route contract.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_REMOTE_PREVIEW_CONTRACT_REF: &str =
    "schemas/runtime/preview_route.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_FIXTURE_DIR: &str =
    "fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_ARTIFACT_REF: &str =
    "artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_REVIEW_CI_PREVIEW_MATRIX_SUMMARY_REF: &str =
    "artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md";

/// One of the four M5 review, CI, and preview lanes governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCiPreviewLane {
    /// Review workspace with durable anchors, stale-base labels, and approval reset.
    ReviewWorkspace,
    /// Merge queue with fresh CI-status truth and attributable rerun/cancel authority.
    MergeQueue,
    /// Pipeline viewer with safe-previewed logs and artifacts.
    PipelineViewer,
    /// Time-bounded, attributable remote preview routes.
    RemotePreview,
}

impl M5ReviewCiPreviewLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewWorkspace,
        Self::MergeQueue,
        Self::PipelineViewer,
        Self::RemotePreview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::MergeQueue => "merge_queue",
            Self::PipelineViewer => "pipeline_viewer",
            Self::RemotePreview => "remote_preview",
        }
    }
}

/// Qualification class for an M5 review, CI, or preview lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCiPreviewQualificationClass {
    /// Lane qualifies for the Stable claim.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental and not claimed.
    Experimental,
    /// Lane is unavailable on this build.
    Unavailable,
    /// Lane is held pending upstream resolution.
    Held,
}

impl M5ReviewCiPreviewQualificationClass {
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

    /// Whether the lane may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence requirement level for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCiPreviewEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this lane's current qualification.
    NotApplicable,
}

impl M5ReviewCiPreviewEvidenceRequirement {
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

/// Downgrade trigger that can narrow a lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCiPreviewDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Merge-queue CI-status truth has gone stale relative to the head it gates.
    MergeQueueStatusStale,
    /// A review-workspace anchor drifted off its target.
    AnchorDrift,
    /// Safe-preview rendering for pipeline logs or artifacts is unavailable.
    SafePreviewUnavailable,
    /// The remote preview route's time bound expired.
    PreviewRouteExpired,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified review/CI/preview boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ReviewCiPreviewDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::MergeQueueStatusStale,
        Self::AnchorDrift,
        Self::SafePreviewUnavailable,
        Self::PreviewRouteExpired,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::MergeQueueStatusStale => "merge_queue_status_stale",
            Self::AnchorDrift => "anchor_drift",
            Self::SafePreviewUnavailable => "safe_preview_unavailable",
            Self::PreviewRouteExpired => "preview_route_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCiPreviewRollbackPosture {
    /// Read-only lane that never mutates workspace, repository, or remote state.
    ReadOnlyNoMutation,
    /// Rerun/cancel actions remain individually attributable and reviewable.
    AttributableRerunOrCancel,
    /// Browser or preview handoff that always preserves a safe return path to the IDE.
    ReturnPathPreserved,
    /// Remote preview route auto-expires at its time bound with no lingering scope.
    TimeBoundedAutoExpire,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the lane's current qualification.
    NotApplicable,
}

impl M5ReviewCiPreviewRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::AttributableRerunOrCancel => "attributable_rerun_or_cancel",
            Self::ReturnPathPreserved => "return_path_preserved",
            Self::TimeBoundedAutoExpire => "time_bounded_auto_expire",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewCiPreviewConsumerSurface {
    /// Review workspace surface.
    ReviewWorkspace,
    /// Merge-queue panel.
    MergeQueuePanel,
    /// Pipeline viewer.
    PipelineViewer,
    /// Remote preview panel.
    RemotePreviewPanel,
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

impl M5ReviewCiPreviewConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewWorkspace,
        Self::MergeQueuePanel,
        Self::PipelineViewer,
        Self::RemotePreviewPanel,
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
            Self::PipelineViewer => "pipeline_viewer",
            Self::RemotePreviewPanel => "remote_preview_panel",
            Self::BrowserCompanion => "browser_companion",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 review, CI, and preview maturity matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCiPreviewMatrixLaneRow {
    /// Review, CI, or preview lane.
    pub lane: M5ReviewCiPreviewLane,
    /// Qualification class earned by this lane.
    pub qualification: M5ReviewCiPreviewQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5ReviewCiPreviewEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ReviewCiPreviewDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5ReviewCiPreviewRollbackPosture,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<M5ReviewCiPreviewConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCiPreviewMatrixTrustReview {
    /// Review-workspace anchors stay durable across edits, rebases, and reopens.
    pub review_workspace_anchors_durable: bool,
    /// Stale-base and outdated-diff states are labeled, never silently hidden.
    pub stale_base_labels_explicit: bool,
    /// Merge-queue CI-status truth stays fresh relative to the head it gates.
    pub merge_queue_truth_fresh: bool,
    /// Every rerun and cancel action stays individually attributable and reviewable.
    pub rerun_cancel_authority_attributable: bool,
    /// Pipeline logs and artifacts are rendered through the safe-preview boundary.
    pub pipeline_logs_artifacts_safe_previewed: bool,
    /// Remote preview routes stay time-bounded and auto-expire at their bound.
    pub remote_preview_time_bounded: bool,
    /// Remote preview routes stay attributable to their opener and origin.
    pub remote_preview_attributable: bool,
    /// Browser and preview handoffs stay return-path safe.
    pub browser_handoff_return_path_safe: bool,
    /// No provider overlay, browser handoff, or preview server creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCiPreviewMatrixConsumerProjection {
    /// Review workspace shows anchor identity and stale-base truth.
    pub review_workspace_shows_anchor_and_stale_base: bool,
    /// Merge-queue panel shows CI status and freshness truth.
    pub merge_queue_shows_ci_status_and_freshness: bool,
    /// Pipeline viewer shows the safe-preview state of logs and artifacts.
    pub pipeline_viewer_shows_safe_preview_state: bool,
    /// Remote preview panel shows expiry and attribution.
    pub remote_preview_shows_expiry_and_attribution: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_qualification: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_qualification: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Preview / Labs lanes are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_lanes: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCiPreviewMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ReviewCiPreviewMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReviewCiPreviewMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5ReviewCiPreviewMatrixLaneRow>,
    /// Trust review block.
    pub trust_review: M5ReviewCiPreviewMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewCiPreviewMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewCiPreviewMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 review, CI, and preview maturity-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReviewCiPreviewMatrixPacket {
    /// Record kind; must equal [`M5_REVIEW_CI_PREVIEW_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5ReviewCiPreviewMatrixLaneRow>,
    /// Trust review block.
    pub trust_review: M5ReviewCiPreviewMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ReviewCiPreviewMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ReviewCiPreviewMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ReviewCiPreviewMatrixPacket {
    /// Builds an M5 review, CI, and preview maturity-matrix packet from stable-lane input.
    pub fn new(input: M5ReviewCiPreviewMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_REVIEW_CI_PREVIEW_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            lane_rows: input.lane_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 review, CI, and preview maturity-matrix invariants.
    pub fn validate(&self) -> Vec<M5ReviewCiPreviewMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REVIEW_CI_PREVIEW_MATRIX_RECORD_KIND {
            violations.push(M5ReviewCiPreviewMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_VERSION {
            violations.push(M5ReviewCiPreviewMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ReviewCiPreviewMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 review/ci/preview matrix packet serializes"),
        ) {
            violations.push(M5ReviewCiPreviewMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 review/ci/preview matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lanes = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Review, Merge-Queue, and Pipeline Maturity Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_rows.len(),
            stable_lanes
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Lanes\n\n");
        for row in &self.lane_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.lane.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Evidence: {} ({} refs)\n",
                row.evidence_requirement.as_str(),
                row.required_evidence_packet_refs.len()
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 review, CI, and preview matrix export.
#[derive(Debug)]
pub enum M5ReviewCiPreviewMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ReviewCiPreviewMatrixViolation>),
}

impl fmt::Display for M5ReviewCiPreviewMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 review/ci/preview matrix export parse failed: {error}"
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
                    "m5 review/ci/preview matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ReviewCiPreviewMatrixArtifactError {}

/// Validation failures emitted by [`M5ReviewCiPreviewMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ReviewCiPreviewMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required lane is missing from the matrix.
    RequiredLaneMissing,
    /// A lane row is incomplete.
    LaneRowIncomplete,
    /// A lane claiming Stable is missing required evidence packet refs.
    StableLaneMissingEvidence,
    /// A lane has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A lane has no consumer surfaces.
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

impl M5ReviewCiPreviewMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::StableLaneMissingEvidence => "stable_lane_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 review, CI, and preview matrix export.
pub fn current_stable_m5_review_ci_preview_matrix_export(
) -> Result<M5ReviewCiPreviewMatrixPacket, M5ReviewCiPreviewMatrixArtifactError> {
    let packet: M5ReviewCiPreviewMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/support_export.json"
    )))
    .map_err(M5ReviewCiPreviewMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ReviewCiPreviewMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ReviewCiPreviewMatrixPacket,
    violations: &mut Vec<M5ReviewCiPreviewMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REVIEW_CI_PREVIEW_MATRIX_SCHEMA_REF,
        M5_REVIEW_CI_PREVIEW_MATRIX_DOC_REF,
        M5_REVIEW_CI_PREVIEW_MATRIX_REVIEW_WORKSPACE_CONTRACT_REF,
        M5_REVIEW_CI_PREVIEW_MATRIX_MERGE_QUEUE_CONTRACT_REF,
        M5_REVIEW_CI_PREVIEW_MATRIX_PIPELINE_VIEWER_CONTRACT_REF,
        M5_REVIEW_CI_PREVIEW_MATRIX_REMOTE_PREVIEW_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ReviewCiPreviewMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_rows(
    packet: &M5ReviewCiPreviewMatrixPacket,
    violations: &mut Vec<M5ReviewCiPreviewMatrixViolation>,
) {
    let present: BTreeSet<M5ReviewCiPreviewLane> =
        packet.lane_rows.iter().map(|row| row.lane).collect();
    for required in M5ReviewCiPreviewLane::ALL {
        if !present.contains(&required) {
            violations.push(M5ReviewCiPreviewMatrixViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5ReviewCiPreviewMatrixViolation::LaneRowIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5ReviewCiPreviewMatrixViolation::StableLaneMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ReviewCiPreviewMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ReviewCiPreviewMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5ReviewCiPreviewMatrixPacket,
    violations: &mut Vec<M5ReviewCiPreviewMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.review_workspace_anchors_durable,
        review.stale_base_labels_explicit,
        review.merge_queue_truth_fresh,
        review.rerun_cancel_authority_attributable,
        review.pipeline_logs_artifacts_safe_previewed,
        review.remote_preview_time_bounded,
        review.remote_preview_attributable,
        review.browser_handoff_return_path_safe,
        review.no_hidden_write_scope,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5ReviewCiPreviewMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ReviewCiPreviewMatrixPacket,
    violations: &mut Vec<M5ReviewCiPreviewMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_workspace_shows_anchor_and_stale_base,
        projection.merge_queue_shows_ci_status_and_freshness,
        projection.pipeline_viewer_shows_safe_preview_state,
        projection.remote_preview_shows_expiry_and_attribution,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_qualification,
        projection.diagnostics_shows_qualification,
        projection.help_about_shows_qualification,
        projection.preview_labs_label_for_unqualified_lanes,
    ] {
        if !ok {
            violations.push(M5ReviewCiPreviewMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ReviewCiPreviewMatrixPacket,
    violations: &mut Vec<M5ReviewCiPreviewMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ReviewCiPreviewMatrixViolation::ProofFreshnessIncomplete);
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
