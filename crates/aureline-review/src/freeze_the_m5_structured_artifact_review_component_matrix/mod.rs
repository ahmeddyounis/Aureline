//! Frozen M5 structured-artifact review component matrix.
//!
//! This module locks the canonical M5 component truth for nine reusable
//! structured-artifact review surfaces — artifact identity bars, diff-mode
//! switchers, structure rows, merge-decision rows, generated-artifact notices,
//! rendered compare viewers, media-metadata rails, redaction/trust badge sets,
//! and compare-summary cards — into one export-safe packet. Each
//! [`M5ArtifactComponentMatrixRow`] binds a component to its maturity class, the
//! exact canonical-source disclosure it must preserve, its render/schema
//! fidelity-narrowing vocabulary, its compare-only-versus-write-back safety, its
//! render-trust disclosure, its generated-from relation, required evidence packet
//! refs, downgrade triggers, rollback posture, source contracts, and
//! consumer-surface parity.
//!
//! The matrix is the single source of truth for whether every claimed M5 compare,
//! merge, or artifact-review surface may consume one shared component family
//! instead of private widget text or generic file chrome. It references upstream
//! artifact-provenance, cell-aware diff, structure-tree, notebook-merge,
//! generated-artifact, safe-preview, design-snapshot, redaction, and manifest-diff
//! contracts by id rather than embedding their content. Raw artifact bodies, raw
//! render payloads, raw media bytes, credentials, and live provider responses stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-structured-artifact-review-component-matrix.schema.json`](../../../../schemas/ui/m5-structured-artifact-review-component-matrix.schema.json).
//! The contract doc is
//! [`docs/review/m5/freeze_the_m5_structured_artifact_review_component_matrix.md`](../../../../docs/review/m5/freeze_the_m5_structured_artifact_review_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-structured-artifact-review-components/`](../../../../fixtures/ui/m5-structured-artifact-review-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ArtifactComponentMatrixPacket`].
pub const M5_ARTIFACT_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_structured_artifact_review_component_matrix";

/// Schema version for M5 structured-artifact review-component matrix records.
pub const M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-matrix.schema.json";

/// Repo-relative path of the M5 structured-artifact review-component matrix doc.
pub const M5_ARTIFACT_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/review/m5/freeze_the_m5_structured_artifact_review_component_matrix.md";

/// Repo-relative path of the frozen artifact-identity-bar (provenance) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF: &str =
    "schemas/ui/m5-artifact-provenance-bundle-card.schema.json";

/// Repo-relative path of the frozen diff-mode-switcher (cell-aware diff) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF: &str =
    "schemas/notebook/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.schema.json";

/// Repo-relative path of the frozen structure-row (source-tree mapping) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF: &str =
    "schemas/preview/inspect_to_source_tree_mapping.schema.json";

/// Repo-relative path of the frozen merge-decision-row (notebook merge) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF: &str =
    "schemas/notebook/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.schema.json";

/// Repo-relative path of the frozen generated-artifact-notice contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF: &str =
    "schemas/generated/generated-artifact-descriptor.schema.json";

/// Repo-relative path of the frozen rendered-compare-viewer (safe-preview) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF: &str =
    "schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json";

/// Repo-relative path of the frozen media-metadata-rail (design-snapshot) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF: &str =
    "schemas/design-system/m5-design-system-contract-matrix.schema.json";

/// Repo-relative path of the frozen redaction/trust-badge-set contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF: &str =
    "schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json";

/// Repo-relative path of the frozen compare-summary-card (manifest-diff) contract.
pub const M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF: &str =
    "schemas/ui/m5-manifest-diff-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_ARTIFACT_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-structured-artifact-review-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ARTIFACT_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-structured-artifact-review-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_ARTIFACT_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/release/m5-structured-artifact-review-proof/summary.md";

/// One of the nine M5 reusable structured-artifact review components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactComponent {
    /// Artifact identity bar naming artifact class and canonical source.
    ArtifactIdentityBar,
    /// Diff-mode switcher exposing available diff modes and the active one.
    DiffModeSwitcher,
    /// Structure row showing a structured path and its change class.
    StructureRow,
    /// Merge-decision row picking base/ours/theirs with write-back safety.
    MergeDecisionRow,
    /// Generated-artifact notice naming the generated-from relation.
    GeneratedArtifactNotice,
    /// Rendered compare viewer with explicit render-trust class.
    RenderedCompareViewer,
    /// Media-metadata rail exposing metadata visibility for media-like artifacts.
    MediaMetadataRail,
    /// Redaction or trust badge set naming redaction/trust posture.
    RedactionOrTrustBadgeSet,
    /// Compare-summary card rolling up the compare result without flattening it.
    CompareSummaryCard,
}

impl M5ArtifactComponent {
    /// Every component, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ArtifactIdentityBar,
        Self::DiffModeSwitcher,
        Self::StructureRow,
        Self::MergeDecisionRow,
        Self::GeneratedArtifactNotice,
        Self::RenderedCompareViewer,
        Self::MediaMetadataRail,
        Self::RedactionOrTrustBadgeSet,
        Self::CompareSummaryCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactIdentityBar => "artifact_identity_bar",
            Self::DiffModeSwitcher => "diff_mode_switcher",
            Self::StructureRow => "structure_row",
            Self::MergeDecisionRow => "merge_decision_row",
            Self::GeneratedArtifactNotice => "generated_artifact_notice",
            Self::RenderedCompareViewer => "rendered_compare_viewer",
            Self::MediaMetadataRail => "media_metadata_rail",
            Self::RedactionOrTrustBadgeSet => "redaction_or_trust_badge_set",
            Self::CompareSummaryCard => "compare_summary_card",
        }
    }

    /// Canonical upstream source contract this component consumes.
    ///
    /// Every row must list this ref among its `source_contract_refs` so a
    /// component can never be re-homed onto generic file chrome that hides its
    /// canonical source of truth.
    pub const fn canonical_source_contract_ref(self) -> &'static str {
        match self {
            Self::ArtifactIdentityBar => M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF,
            Self::DiffModeSwitcher => M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF,
            Self::StructureRow => M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF,
            Self::MergeDecisionRow => M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF,
            Self::GeneratedArtifactNotice => {
                M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF
            }
            Self::RenderedCompareViewer => {
                M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF
            }
            Self::MediaMetadataRail => M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF,
            Self::RedactionOrTrustBadgeSet => {
                M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF
            }
            Self::CompareSummaryCard => M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF,
        }
    }
}

/// Maturity class for an M5 structured-artifact review component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactComponentMaturityClass {
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

impl M5ArtifactComponentMaturityClass {
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
pub enum M5ArtifactComponentEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this component's current maturity.
    NotApplicable,
}

impl M5ArtifactComponentEvidenceRequirement {
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

/// Render/schema fidelity-narrowing vocabulary that every component must preserve.
///
/// This vocabulary names the structured-render posture explicitly so a
/// schema-unrecognized, untrusted, or redacted artifact is never flattened into a
/// generic raw fallback without explanation, and a raw fallback is always labeled
/// as such rather than presented as faithful structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactFidelityState {
    /// Structured mode renders the artifact faithfully against a known schema.
    StructuredFaithful,
    /// Structured mode is available but only partially covers the artifact.
    StructuredPartial,
    /// No parser/schema recognizes the artifact; structured mode is unavailable.
    SchemaUnrecognized,
    /// A rendered preview exists but its render is not fully trusted.
    RenderUntrusted,
    /// The component falls back to an explicitly labeled raw/export-safe view.
    RawFallback,
    /// Content is redacted or withheld under the export/redaction posture.
    RedactedOrWithheld,
}

impl M5ArtifactFidelityState {
    /// Every fidelity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StructuredFaithful,
        Self::StructuredPartial,
        Self::SchemaUnrecognized,
        Self::RenderUntrusted,
        Self::RawFallback,
        Self::RedactedOrWithheld,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredFaithful => "structured_faithful",
            Self::StructuredPartial => "structured_partial",
            Self::SchemaUnrecognized => "schema_unrecognized",
            Self::RenderUntrusted => "render_untrusted",
            Self::RawFallback => "raw_fallback",
            Self::RedactedOrWithheld => "redacted_or_withheld",
        }
    }
}

/// Downgrade trigger that can narrow a component below its claimed maturity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactComponentDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The artifact parser is unavailable on this build.
    ParserUnavailable,
    /// No schema recognizes the artifact class.
    SchemaUnrecognized,
    /// The rendered preview is not trusted.
    RenderUntrusted,
    /// Compare-only safety is enforced; write-back is unavailable.
    CompareOnlyEnforced,
    /// The generated-from source drifted or the generated artifact is stale.
    GeneratedArtifactStale,
    /// Media metadata could not be extracted.
    MediaMetadataUnavailable,
    /// Redaction was applied and narrows visible content.
    RedactionApplied,
    /// Component trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified artifact-review boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ArtifactComponentDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ParserUnavailable,
        Self::SchemaUnrecognized,
        Self::RenderUntrusted,
        Self::CompareOnlyEnforced,
        Self::GeneratedArtifactStale,
        Self::MediaMetadataUnavailable,
        Self::RedactionApplied,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ParserUnavailable => "parser_unavailable",
            Self::SchemaUnrecognized => "schema_unrecognized",
            Self::RenderUntrusted => "render_untrusted",
            Self::CompareOnlyEnforced => "compare_only_enforced",
            Self::GeneratedArtifactStale => "generated_artifact_stale",
            Self::MediaMetadataUnavailable => "media_metadata_unavailable",
            Self::RedactionApplied => "redaction_applied",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback / write-back posture for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactComponentRollbackPosture {
    /// Read-only component that never mutates the artifact, workspace, or repo.
    ReadOnlyNoMutation,
    /// Compare-only component that never silently writes back to the artifact.
    CompareOnlyNoWriteBack,
    /// Write-back stays individually attributable and reviewable.
    WriteBackAttributable,
    /// Generated artifacts regenerate from source rather than accept manual edits.
    RegenerateOnlyNoManualEdit,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the component's current maturity.
    NotApplicable,
}

impl M5ArtifactComponentRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyNoMutation => "read_only_no_mutation",
            Self::CompareOnlyNoWriteBack => "compare_only_no_write_back",
            Self::WriteBackAttributable => "write_back_attributable",
            Self::RegenerateOnlyNoManualEdit => "regenerate_only_no_manual_edit",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ArtifactComponentConsumerSurface {
    /// Review workspace surface.
    ReviewWorkspace,
    /// Diff / compare view.
    DiffCompareView,
    /// Merge / conflict resolution workspace.
    MergeConflictWorkspace,
    /// Notebook review surface.
    NotebookReview,
    /// Artifact browser (coverage, profile, crash, SBOM, lockfile adjuncts).
    ArtifactBrowser,
    /// Browser companion / handoff follow-up.
    BrowserCompanion,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl M5ArtifactComponentConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewWorkspace,
        Self::DiffCompareView,
        Self::MergeConflictWorkspace,
        Self::NotebookReview,
        Self::ArtifactBrowser,
        Self::BrowserCompanion,
        Self::CliHeadless,
        Self::SupportExport,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::DiffCompareView => "diff_compare_view",
            Self::MergeConflictWorkspace => "merge_conflict_workspace",
            Self::NotebookReview => "notebook_review",
            Self::ArtifactBrowser => "artifact_browser",
            Self::BrowserCompanion => "browser_companion",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 structured-artifact review-component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactComponentMatrixRow {
    /// Structured-artifact review component.
    pub component: M5ArtifactComponent,
    /// Maturity class earned by this component.
    pub maturity: M5ArtifactComponentMaturityClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Exact artifact-class and canonical-source disclosure this component keeps.
    pub canonical_source_disclosure: String,
    /// Render/schema fidelity-narrowing vocabulary this component must preserve.
    pub fidelity_narrowing_vocab: Vec<M5ArtifactFidelityState>,
    /// Compare-only-versus-write-back safety this component keeps explicit.
    pub compare_write_back_safety: String,
    /// Render-trust disclosure this component keeps explicit.
    pub render_trust_disclosure: String,
    /// Generated-from / source-of-truth relation this component keeps explicit.
    pub generated_from_relation: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5ArtifactComponentEvidenceRequirement,
    /// Required evidence packet refs for this maturity.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ArtifactComponentDowngradeTrigger>,
    /// Rollback / write-back posture.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this component.
    pub consumer_surfaces: Vec<M5ArtifactComponentConsumerSurface>,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactComponentMatrixTrustReview {
    /// Artifact class and canonical source are always explicit.
    pub canonical_source_always_explicit: bool,
    /// Compare-only artifacts are never silently promoted to writable state.
    pub compare_only_never_silently_writable: bool,
    /// Structured modes are never flattened into raw fallback without explanation.
    pub structured_mode_never_flattened_without_explanation: bool,
    /// Generated-from / source-of-truth relations are never hidden.
    pub generated_from_relation_never_hidden: bool,
    /// Render trust stays explicit rather than implied.
    pub render_trust_explicit: bool,
    /// Metadata visibility stays explicit for media-like artifacts.
    pub metadata_visibility_explicit: bool,
    /// Raw / export-safe fallbacks stay explicit when fidelity narrows.
    pub raw_export_safe_fallback_explicit: bool,
    /// Redaction / export posture stays explicit.
    pub redaction_posture_explicit: bool,
    /// Parser / schema state stays explicit.
    pub parser_schema_state_explicit: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactComponentMatrixConsumerProjection {
    /// Artifact identity bar shows artifact class and canonical source.
    pub artifact_identity_bar_shows_class_and_canonical_source: bool,
    /// Diff-mode switcher shows available modes and the active mode.
    pub diff_mode_switcher_shows_available_modes_and_active: bool,
    /// Structure row shows the structured path and its change class.
    pub structure_row_shows_path_and_change_class: bool,
    /// Merge-decision row shows the chosen side and write-back safety.
    pub merge_decision_row_shows_side_and_write_back_safety: bool,
    /// Generated-artifact notice shows the generated-from relation.
    pub generated_artifact_notice_shows_generated_from_relation: bool,
    /// Rendered compare viewer shows its render-trust class.
    pub rendered_compare_viewer_shows_render_trust: bool,
    /// Media-metadata rail shows metadata visibility.
    pub media_metadata_rail_shows_metadata_visibility: bool,
    /// Redaction / trust badge set shows redaction posture.
    pub redaction_or_trust_badge_set_shows_redaction_posture: bool,
    /// Compare-summary card shows the summary without flattening it.
    pub compare_summary_card_shows_summary_without_flattening: bool,
    /// CLI / headless shows component truth.
    pub cli_headless_shows_component_truth: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactComponentMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ArtifactComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ArtifactComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ArtifactComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5ArtifactComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ArtifactComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ArtifactComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 structured-artifact review-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactComponentMatrixPacket {
    /// Record kind; must equal [`M5_ARTIFACT_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ArtifactComponentMatrixRow>,
    /// Trust review block.
    pub trust_review: M5ArtifactComponentMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ArtifactComponentMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ArtifactComponentMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ArtifactComponentMatrixPacket {
    /// Builds an M5 structured-artifact review-component matrix packet.
    pub fn new(input: M5ArtifactComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_ARTIFACT_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 structured-artifact review-component matrix invariants.
    pub fn validate(&self) -> Vec<M5ArtifactComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ARTIFACT_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5ArtifactComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ArtifactComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ArtifactComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 artifact-component matrix packet serializes"),
        ) {
            violations.push(M5ArtifactComponentMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 artifact-component matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.maturity.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Structured-Artifact Review Component Matrix\n\n");
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
                "  - Canonical source: {}\n",
                row.canonical_source_disclosure
            ));
            out.push_str(&format!(
                "  - Compare/write-back safety: {}\n",
                row.compare_write_back_safety
            ));
            out.push_str(&format!(
                "  - Render trust: {}\n",
                row.render_trust_disclosure
            ));
            out.push_str(&format!(
                "  - Generated-from relation: {}\n",
                row.generated_from_relation
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 artifact-component matrix export.
#[derive(Debug)]
pub enum M5ArtifactComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ArtifactComponentMatrixViolation>),
}

impl fmt::Display for M5ArtifactComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 artifact-component matrix export parse failed: {error}"
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
                    "m5 artifact-component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ArtifactComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5ArtifactComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ArtifactComponentMatrixViolation {
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
    /// A component's row does not list its canonical source contract ref.
    ComponentSourceContractMismatch,
    /// A component claiming Stable is missing required evidence packet refs.
    StableComponentMissingEvidence,
    /// A component has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component does not name its artifact-class / canonical-source disclosure.
    CanonicalSourceDisclosureMissing,
    /// A component does not carry a render/schema fidelity-narrowing vocabulary.
    FidelityNarrowingVocabMissing,
    /// A component does not name its compare-only-versus-write-back safety.
    CompareWriteBackSafetyMissing,
    /// A component does not name its render-trust disclosure.
    RenderTrustDisclosureMissing,
    /// A component does not name its generated-from relation.
    GeneratedFromRelationMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ArtifactComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::ComponentSourceContractMismatch => "component_source_contract_mismatch",
            Self::StableComponentMissingEvidence => "stable_component_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::CanonicalSourceDisclosureMissing => "canonical_source_disclosure_missing",
            Self::FidelityNarrowingVocabMissing => "fidelity_narrowing_vocab_missing",
            Self::CompareWriteBackSafetyMissing => "compare_write_back_safety_missing",
            Self::RenderTrustDisclosureMissing => "render_trust_disclosure_missing",
            Self::GeneratedFromRelationMissing => "generated_from_relation_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 artifact-component matrix export.
pub fn current_stable_m5_artifact_component_matrix_export(
) -> Result<M5ArtifactComponentMatrixPacket, M5ArtifactComponentMatrixArtifactError> {
    let packet: M5ArtifactComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-structured-artifact-review-proof/support_export.json"
    )))
    .map_err(M5ArtifactComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ArtifactComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ArtifactComponentMatrixPacket,
    violations: &mut Vec<M5ArtifactComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_DOC_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ArtifactComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_component_rows(
    packet: &M5ArtifactComponentMatrixPacket,
    violations: &mut Vec<M5ArtifactComponentMatrixViolation>,
) {
    let present: BTreeSet<M5ArtifactComponent> = packet
        .component_rows
        .iter()
        .map(|row| row.component)
        .collect();
    for required in M5ArtifactComponent::ALL {
        if !present.contains(&required) {
            violations.push(M5ArtifactComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|contract| contract == row.component.canonical_source_contract_ref())
        {
            violations.push(M5ArtifactComponentMatrixViolation::ComponentSourceContractMismatch);
        }
        if row.maturity.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::StableComponentMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.canonical_source_disclosure.trim().is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::CanonicalSourceDisclosureMissing);
        }
        if row.fidelity_narrowing_vocab.is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::FidelityNarrowingVocabMissing);
        }
        if row.compare_write_back_safety.trim().is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::CompareWriteBackSafetyMissing);
        }
        if row.render_trust_disclosure.trim().is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::RenderTrustDisclosureMissing);
        }
        if row.generated_from_relation.trim().is_empty() {
            violations.push(M5ArtifactComponentMatrixViolation::GeneratedFromRelationMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5ArtifactComponentMatrixPacket,
    violations: &mut Vec<M5ArtifactComponentMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.canonical_source_always_explicit,
        review.compare_only_never_silently_writable,
        review.structured_mode_never_flattened_without_explanation,
        review.generated_from_relation_never_hidden,
        review.render_trust_explicit,
        review.metadata_visibility_explicit,
        review.raw_export_safe_fallback_explicit,
        review.redaction_posture_explicit,
        review.parser_schema_state_explicit,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5ArtifactComponentMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ArtifactComponentMatrixPacket,
    violations: &mut Vec<M5ArtifactComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.artifact_identity_bar_shows_class_and_canonical_source,
        projection.diff_mode_switcher_shows_available_modes_and_active,
        projection.structure_row_shows_path_and_change_class,
        projection.merge_decision_row_shows_side_and_write_back_safety,
        projection.generated_artifact_notice_shows_generated_from_relation,
        projection.rendered_compare_viewer_shows_render_trust,
        projection.media_metadata_rail_shows_metadata_visibility,
        projection.redaction_or_trust_badge_set_shows_redaction_posture,
        projection.compare_summary_card_shows_summary_without_flattening,
        projection.cli_headless_shows_component_truth,
        projection.support_export_shows_component_truth,
    ] {
        if !ok {
            violations.push(M5ArtifactComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ArtifactComponentMatrixPacket,
    violations: &mut Vec<M5ArtifactComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ArtifactComponentMatrixViolation::ProofFreshnessIncomplete);
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
