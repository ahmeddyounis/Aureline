//! Frozen M5 docs and code-recall matrix, browser-surface scope, and
//! retrieval-debug contract.
//!
//! This module locks the canonical M5 depth qualification for four
//! docs-and-code-understanding lanes — mirror-aware docs semantic recall, cited
//! codebase explainers, the retrieval-debug inspector, and scoped browser
//! surfaces — into one export-safe packet. Each
//! [`M5DocsRecallMatrixLaneRow`] binds a lane to its qualification class,
//! required evidence packet refs, downgrade triggers, rollback posture, source
//! contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether these lanes may ship as
//! Stable, Beta, Preview, or must narrow further. It references upstream docs,
//! semantic-recall, graph-explainer, retrieval-inspector, and browser-surface
//! contracts by id rather than embedding their content. Raw document bodies,
//! raw source files, raw query text, raw provider payloads, credentials, and
//! live vendor-doc snapshots stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json`](../../../../schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json).
//! The contract doc is
//! [`docs/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract.md`](../../../../docs/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/`](../../../../fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsRecallMatrixPacket`].
pub const M5_DOCS_RECALL_MATRIX_RECORD_KIND: &str =
    "freeze_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract";

/// Schema version for M5 docs and code-recall matrix records.
pub const M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DOCS_RECALL_MATRIX_SCHEMA_REF: &str =
    "schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json";

/// Repo-relative path of the M5 docs and code-recall matrix contract doc.
pub const M5_DOCS_RECALL_MATRIX_DOC_REF: &str =
    "docs/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract.md";

/// Repo-relative path of the frozen mirror-aware semantic-recall boundary contract.
pub const M5_DOCS_RECALL_MATRIX_DOCS_RECALL_CONTRACT_REF: &str =
    "schemas/docs/semantic_recall_boundary_truth.schema.json";

/// Repo-relative path of the frozen cited codebase-explainer contract.
pub const M5_DOCS_RECALL_MATRIX_CODE_EXPLAINER_CONTRACT_REF: &str =
    "schemas/graph/codebase_explainer_packet.schema.json";

/// Repo-relative path of the frozen retrieval-debug inspector contract.
pub const M5_DOCS_RECALL_MATRIX_RETRIEVAL_DEBUG_CONTRACT_REF: &str =
    "schemas/search/retrieval_inspector.schema.json";

/// Repo-relative path of the frozen scoped docs-browser surface contract.
pub const M5_DOCS_RECALL_MATRIX_BROWSER_SURFACE_CONTRACT_REF: &str =
    "schemas/docs/docs_browser_truth_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_RECALL_MATRIX_FIXTURE_DIR: &str =
    "fixtures/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_RECALL_MATRIX_ARTIFACT_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_DOCS_RECALL_MATRIX_SUMMARY_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract.md";

/// One of the four M5 docs-and-code-understanding lanes governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRecallLane {
    /// Mirror-aware docs semantic recall with source/version/freshness truth.
    DocsSemanticRecall,
    /// Cited codebase explainers that preserve source class and confidence.
    CodebaseExplainer,
    /// Retrieval-debug inspector exposing ranking reasons and recall provenance.
    RetrievalDebug,
    /// Narrow, attributable, return-path-safe docs/review/light-edit browser surface.
    ScopedBrowserSurface,
}

impl M5DocsRecallLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DocsSemanticRecall,
        Self::CodebaseExplainer,
        Self::RetrievalDebug,
        Self::ScopedBrowserSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSemanticRecall => "docs_semantic_recall",
            Self::CodebaseExplainer => "codebase_explainer",
            Self::RetrievalDebug => "retrieval_debug",
            Self::ScopedBrowserSurface => "scoped_browser_surface",
        }
    }
}

/// Qualification class for an M5 docs-and-code-recall lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRecallQualificationClass {
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

impl M5DocsRecallQualificationClass {
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
pub enum M5DocsRecallEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this lane's current qualification.
    NotApplicable,
}

impl M5DocsRecallEvidenceRequirement {
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
pub enum M5DocsRecallDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Pinned, signed mirror is offline or unavailable.
    MirrorOffline,
    /// Source version no longer matches the indexed/pinned version.
    SourceVersionMismatch,
    /// Freshness window for the recall corpus expired.
    FreshnessExpired,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond qualified docs/browser boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5DocsRecallDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::MirrorOffline,
        Self::SourceVersionMismatch,
        Self::FreshnessExpired,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::MirrorOffline => "mirror_offline",
            Self::SourceVersionMismatch => "source_version_mismatch",
            Self::FreshnessExpired => "freshness_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRecallRollbackPosture {
    /// Read-only recall/explainer lane that never mutates workspace state.
    ReadOnlyNoMutation,
    /// Browser handoff that always preserves a safe return path to the IDE.
    ReturnPathPreserved,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the lane's current qualification.
    NotApplicable,
}

impl M5DocsRecallRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::ReturnPathPreserved => "return_path_preserved",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsRecallConsumerSurface {
    /// Docs browser / reader surface.
    DocsBrowser,
    /// Codebase explainer panel.
    CodeExplainerPanel,
    /// Retrieval-debug inspector.
    RetrievalDebugInspector,
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

impl M5DocsRecallConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DocsBrowser,
        Self::CodeExplainerPanel,
        Self::RetrievalDebugInspector,
        Self::BrowserCompanion,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::CodeExplainerPanel => "code_explainer_panel",
            Self::RetrievalDebugInspector => "retrieval_debug_inspector",
            Self::BrowserCompanion => "browser_companion",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 docs and code-recall matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsRecallMatrixLaneRow {
    /// Docs/code-understanding lane.
    pub lane: M5DocsRecallLane,
    /// Qualification class earned by this lane.
    pub qualification: M5DocsRecallQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5DocsRecallEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5DocsRecallDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5DocsRecallRollbackPosture,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<M5DocsRecallConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsRecallMatrixTrustReview {
    /// Docs recall stays mirror-aware (pinned/signed mirror outranks live vendor docs).
    pub docs_recall_mirror_aware: bool,
    /// Codebase explainers stay cited with an explicit source class.
    pub explainers_cited_with_source_class: bool,
    /// Recall and explainer results preserve their confidence class.
    pub confidence_class_preserved: bool,
    /// Open-raw / open-source escapes are preserved on every derived result.
    pub open_raw_open_source_escape_preserved: bool,
    /// Ranking reasons are explicit on every recall result.
    pub ranking_reasons_explicit: bool,
    /// The retrieval-debug inspector stays available for every recall result.
    pub retrieval_debug_available: bool,
    /// Browser surfaces stay narrow and attributable.
    pub browser_surface_narrow_and_attributable: bool,
    /// Browser handoffs stay return-path safe.
    pub browser_handoff_return_path_safe: bool,
    /// No source, mirror, pack, heuristic, or handoff looks more authoritative than proven.
    pub no_source_looks_more_authoritative_than_proven: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsRecallMatrixConsumerProjection {
    /// Docs browser shows provenance and freshness truth.
    pub docs_browser_shows_provenance_and_freshness: bool,
    /// Codebase explainer shows source class and confidence.
    pub code_explainer_shows_source_class_and_confidence: bool,
    /// Retrieval-debug inspector shows ranking reasons.
    pub retrieval_debug_shows_ranking_reasons: bool,
    /// Browser companion shows captured-vs-live state.
    pub browser_companion_shows_captured_vs_live: bool,
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
pub struct M5DocsRecallMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5DocsRecallMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsRecallMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5DocsRecallMatrixLaneRow>,
    /// Trust review block.
    pub trust_review: M5DocsRecallMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsRecallMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsRecallMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 docs and code-recall matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsRecallMatrixPacket {
    /// Record kind; must equal [`M5_DOCS_RECALL_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5DocsRecallMatrixLaneRow>,
    /// Trust review block.
    pub trust_review: M5DocsRecallMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsRecallMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsRecallMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsRecallMatrixPacket {
    /// Builds an M5 docs and code-recall matrix packet from stable-lane input.
    pub fn new(input: M5DocsRecallMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_RECALL_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 docs and code-recall matrix invariants.
    pub fn validate(&self) -> Vec<M5DocsRecallMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_RECALL_MATRIX_RECORD_KIND {
            violations.push(M5DocsRecallMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_RECALL_MATRIX_SCHEMA_VERSION {
            violations.push(M5DocsRecallMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsRecallMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 docs recall matrix packet serializes"),
        ) {
            violations.push(M5DocsRecallMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 docs recall matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lanes = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs and Code-Recall Matrix\n\n");
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

/// Errors emitted when reading the checked-in M5 docs and code-recall matrix export.
#[derive(Debug)]
pub enum M5DocsRecallMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsRecallMatrixViolation>),
}

impl fmt::Display for M5DocsRecallMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 docs recall matrix export parse failed: {error}"
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
                    "m5 docs recall matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsRecallMatrixArtifactError {}

/// Validation failures emitted by [`M5DocsRecallMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsRecallMatrixViolation {
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

impl M5DocsRecallMatrixViolation {
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

/// Reads and validates the checked-in stable M5 docs and code-recall matrix export.
pub fn current_stable_m5_docs_and_code_recall_matrix_export(
) -> Result<M5DocsRecallMatrixPacket, M5DocsRecallMatrixArtifactError> {
    let packet: M5DocsRecallMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/freeze_the_m5_docs_and_code_recall_matrix_browser_surface_scope_and_retrieval_debug_contract/support_export.json"
    )))
    .map_err(M5DocsRecallMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsRecallMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DocsRecallMatrixPacket,
    violations: &mut Vec<M5DocsRecallMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_RECALL_MATRIX_SCHEMA_REF,
        M5_DOCS_RECALL_MATRIX_DOC_REF,
        M5_DOCS_RECALL_MATRIX_DOCS_RECALL_CONTRACT_REF,
        M5_DOCS_RECALL_MATRIX_CODE_EXPLAINER_CONTRACT_REF,
        M5_DOCS_RECALL_MATRIX_RETRIEVAL_DEBUG_CONTRACT_REF,
        M5_DOCS_RECALL_MATRIX_BROWSER_SURFACE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsRecallMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_rows(
    packet: &M5DocsRecallMatrixPacket,
    violations: &mut Vec<M5DocsRecallMatrixViolation>,
) {
    let present: BTreeSet<M5DocsRecallLane> = packet.lane_rows.iter().map(|row| row.lane).collect();
    for required in M5DocsRecallLane::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsRecallMatrixViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5DocsRecallMatrixViolation::LaneRowIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5DocsRecallMatrixViolation::StableLaneMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsRecallMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsRecallMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5DocsRecallMatrixPacket,
    violations: &mut Vec<M5DocsRecallMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.docs_recall_mirror_aware,
        review.explainers_cited_with_source_class,
        review.confidence_class_preserved,
        review.open_raw_open_source_escape_preserved,
        review.ranking_reasons_explicit,
        review.retrieval_debug_available,
        review.browser_surface_narrow_and_attributable,
        review.browser_handoff_return_path_safe,
        review.no_source_looks_more_authoritative_than_proven,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5DocsRecallMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsRecallMatrixPacket,
    violations: &mut Vec<M5DocsRecallMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.docs_browser_shows_provenance_and_freshness,
        projection.code_explainer_shows_source_class_and_confidence,
        projection.retrieval_debug_shows_ranking_reasons,
        projection.browser_companion_shows_captured_vs_live,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_qualification,
        projection.diagnostics_shows_qualification,
        projection.help_about_shows_qualification,
        projection.preview_labs_label_for_unqualified_lanes,
    ] {
        if !ok {
            violations.push(M5DocsRecallMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsRecallMatrixPacket,
    violations: &mut Vec<M5DocsRecallMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsRecallMatrixViolation::ProofFreshnessIncomplete);
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
