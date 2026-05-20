//! Corpus manifest types for the docs preview / maintenance integrity drill
//! suite.
//!
//! The manifest is the single source of truth for the corpus. Each positive
//! drill names a fixture holding one canonical docs-maintenance record
//! (preview header, suggestion card, finding row, maintenance row, contract,
//! or exported review packet, all owned by `aureline-docs::maintenance`) and
//! pins the truth that record must carry — preview mode and sanitization
//! posture, CommonMark baseline, suggestion trigger and apply posture, finding
//! class / detection / validation mode, suppression attribution, and the
//! branch / release / channel / audience scope. A positive drill MUST validate
//! cleanly (zero findings) and match every pinned `expected_*` field.
//!
//! Each negative drill names a fixture whose record MUST FAIL validation with
//! at least one finding whose `check_id` contains `expected_violation_check_id`
//! (review-packet drifts compare against the seeded contract). This keeps
//! hidden renderer extensions, silent suggestion application, unscoped
//! README / changelog updates, dropped suppression attribution, and
//! wrong-branch / wrong-channel maintenance rejected before any beta
//! docs-authoring claim hardens.

use serde::{Deserialize, Serialize};

/// Filename of the corpus manifest, relative to the corpus directory.
pub const MANIFEST_FILE_NAME: &str = "manifest.json";

/// Path of the corpus directory relative to the repository root.
pub const CORPUS_DIR_REL: &str = "fixtures/docs/m3/docs_maintenance_corpus";

/// Record family a drill fixture deserializes into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillRecordType {
    /// A single [`aureline_docs::DocsPreviewHeader`].
    PreviewHeader,
    /// A single [`aureline_docs::DocsSuggestionCard`].
    SuggestionCard,
    /// A single [`aureline_docs::DocsExampleFindingRow`].
    FindingRow,
    /// A single [`aureline_docs::DocsMaintenanceRow`].
    MaintenanceRow,
    /// A whole [`aureline_docs::DocsMaintenanceContract`].
    Contract,
    /// An exported [`aureline_docs::DocsMaintenanceReviewPacket`], validated
    /// against the seeded contract.
    ReviewPacket,
}

impl DrillRecordType {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewHeader => "preview_header",
            Self::SuggestionCard => "suggestion_card",
            Self::FindingRow => "finding_row",
            Self::MaintenanceRow => "maintenance_row",
            Self::Contract => "contract",
            Self::ReviewPacket => "review_packet",
        }
    }
}

/// Root manifest document for the docs-maintenance drill corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusManifest {
    /// Stable corpus identifier.
    pub corpus_id: String,
    /// Manifest schema version.
    pub schema_version: u32,
    /// Reviewer-facing description.
    pub description: String,
    /// Positive drill specs.
    pub positive_drills: Vec<PositiveDrillSpec>,
    /// Negative drill specs.
    pub negative_drills: Vec<NegativeDrillSpec>,
}

/// Single positive drill spec: the fixture MUST parse, validate cleanly, and
/// satisfy every pinned expectation listed here. Unspecified expectations
/// (`None` / empty) are not asserted, so a focused per-record drill only pins
/// the truth it stands for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositiveDrillSpec {
    /// Stable drill id used by audit / support records.
    pub drill_id: String,
    /// Path to the fixture relative to the corpus directory.
    pub fixture: String,
    /// Record family the fixture deserializes into.
    pub record_type: DrillRecordType,
    /// Reviewer-facing class for the coverage matrix.
    pub drill_class: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,

    /// Expected `DocsPreviewMode` token (`source` / `split` / `rendered`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_preview_mode: Option<String>,
    /// Expected `DocsPreviewSanitizationState` token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_sanitization_state: Option<String>,
    /// Expected CommonMark-baseline flag (preview headers).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_commonmark_baseline: Option<bool>,

    /// Expected `DocsSuggestionTrigger` token (suggestion cards).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_trigger: Option<String>,
    /// Expected `DocsSuggestionApplyPosture` token (suggestion cards).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_apply_posture: Option<String>,

    /// Expected `DocsFindingClass` token (finding rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_finding_class: Option<String>,
    /// Expected `DocsFindingDetectionState` token (finding rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_detection_state: Option<String>,
    /// Expected `DocsExampleValidationMode` token (finding rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_validation_mode: Option<String>,
    /// Expected `DocsFindingSuppressionState` token (finding rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_suppression_state: Option<String>,
    /// When `true`, the finding row MUST carry well-formed suppression
    /// attribution (actor / reason / expiry / evidence).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_suppression_attribution: Option<bool>,

    /// Expected `DocsArtifactKind` token (suggestion / finding / maintenance).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_artifact_kind: Option<String>,
    /// Expected `DocsAudienceScope` token (maintenance rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_audience_scope: Option<String>,
    /// Expected `DocsPublishBoundaryState` token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_publish_boundary_state: Option<String>,
    /// Expected branch scope (maintenance rows / handoff banner).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_branch_scope: Option<String>,
    /// Expected release scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_release_scope: Option<String>,
    /// Expected channel scope (so beta notes cannot pass for stable docs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_channel_scope: Option<String>,
}

/// Single negative drill spec: the fixture MUST FAIL validation with at least
/// one finding whose `check_id` contains `expected_violation_check_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NegativeDrillSpec {
    /// Stable drill id.
    pub drill_id: String,
    /// Fixture path relative to the corpus directory.
    pub fixture: String,
    /// Record family the fixture deserializes into.
    pub record_type: DrillRecordType,
    /// Substring that must appear in a validation finding's `check_id`.
    pub expected_violation_check_id: String,
    /// Sub-axes the drill exercises.
    #[serde(default)]
    pub covers: Vec<String>,
}
