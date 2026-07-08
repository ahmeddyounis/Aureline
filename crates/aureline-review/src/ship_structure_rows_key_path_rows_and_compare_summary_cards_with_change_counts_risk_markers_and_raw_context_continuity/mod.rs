//! Structure rows, key-path rows, and compare summary cards carrying object
//! identity, change kind, old/new summaries, confidence/schema notes, changed
//! object counts, risk markers, and raw-context continuity.
//!
//! This module narrows the `structure_row` and `compare_summary_card` components
//! frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`] into
//! implemented, export-safe review controls. Every [`StructureRow`] answers, from
//! the component alone, which structured object or key-path changed, whether it
//! was added, removed, modified, metadata-only, or redacted-and-hidden, what the
//! old and new summaries are, what confidence or schema note backs the reading,
//! and how to jump to the underlying raw content. Every [`CompareSummaryCard`]
//! rolls up the compare result — changed-object counts by change kind, scale and
//! risk markers, and a compare-only-versus-write-back safety disclosure — without
//! ever flattening the diff or hiding the ability to inspect the raw artifact.
//!
//! The two controls are paired by artifact reference: every artifact that shows a
//! compare summary card also shows structure rows, so scale and risk are never
//! surfaced in a card divorced from the per-object detail, and per-object detail
//! is never shown without the roll-up scale. Add / remove / modify state stays
//! distinct for structured objects, package-centric deltas, metadata-only
//! changes, and redacted hidden fields, and a redacted field is always shown as
//! hidden rather than silently dropped.
//!
//! The fidelity-narrowing vocabulary ([`M5ArtifactFidelityState`]) and rollback
//! posture ([`M5ArtifactComponentRollbackPosture`]) are reused directly from the
//! frozen matrix so schema state and write-back safety read the same everywhere.
//! The packet references the upstream artifact-component-matrix, source-tree
//! mapping, and manifest-diff contracts by id rather than embedding their
//! content. Raw artifact bodies, raw diffs, credentials, and live provider
//! responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-structure-compare-summary-controls.schema.json`](../../../../schemas/ui/m5-structure-compare-summary-controls.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_structure_rows_and_compare_summary_cards.md`](../../../../docs/review/m5/ship_structure_rows_and_compare_summary_cards.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-structure-compare-summary-controls/`](../../../../fixtures/ui/m5-structure-compare-summary-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_structured_artifact_review_component_matrix::{
    M5ArtifactComponent, M5ArtifactComponentRollbackPosture, M5ArtifactFidelityState,
    M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF,
};

/// Stable record-kind tag carried by [`StructureCompareControlsPacket`].
pub const STRUCTURE_COMPARE_CONTROLS_RECORD_KIND: &str =
    "structure_rows_and_compare_summary_controls";

/// Schema version for structure-row / compare-summary control records.
pub const STRUCTURE_COMPARE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const STRUCTURE_COMPARE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-structure-compare-summary-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const STRUCTURE_COMPARE_CONTROLS_DOC_REF: &str =
    "docs/review/m5/ship_structure_rows_and_compare_summary_cards.md";

/// Repo-relative path of the protected fixture directory.
pub const STRUCTURE_COMPARE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-structure-compare-summary-controls";

/// Repo-relative path of the checked support-export artifact.
pub const STRUCTURE_COMPARE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-structure-compare-summary-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const STRUCTURE_COMPARE_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-structure-compare-summary-controls-proof/summary.md";

/// The change kind carried by a structure row.
///
/// This is a core honesty axis: add / remove / modify state stays distinct, and
/// metadata-only changes and redacted-hidden fields are their own explicit
/// states rather than being folded into a generic "modified" bucket or dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructureChangeKind {
    /// The object was added; only a new-side summary exists.
    Added,
    /// The object was removed; only an old-side summary exists.
    Removed,
    /// The object was modified; both old and new summaries exist.
    Modified,
    /// Only metadata around the object changed; both summaries exist.
    MetadataOnly,
    /// The object changed but its content is redacted and stays hidden.
    RedactedHidden,
}

impl StructureChangeKind {
    /// Every change kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Added,
        Self::Removed,
        Self::Modified,
        Self::MetadataOnly,
        Self::RedactedHidden,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Modified => "modified",
            Self::MetadataOnly => "metadata_only",
            Self::RedactedHidden => "redacted_hidden",
        }
    }

    /// Whether the object's content stays hidden (redacted) on this row.
    pub const fn is_content_hidden(self) -> bool {
        matches!(self, Self::RedactedHidden)
    }
}

/// The category of structured object a row describes.
///
/// Kept distinct so package-centric deltas, plain structured objects,
/// metadata-only fields, and redacted fields never blur together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredObjectCategory {
    /// A structured object keyed by a schema path.
    StructuredObject,
    /// A package / dependency delta in a manifest or lockfile.
    PackageDelta,
    /// A metadata-only field around an object.
    MetadataField,
    /// A field whose content is redacted and shown as hidden.
    RedactedField,
}

impl StructuredObjectCategory {
    /// Every category, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StructuredObject,
        Self::PackageDelta,
        Self::MetadataField,
        Self::RedactedField,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredObject => "structured_object",
            Self::PackageDelta => "package_delta",
            Self::MetadataField => "metadata_field",
            Self::RedactedField => "redacted_field",
        }
    }

    /// Whether this category names a redacted field whose content stays hidden.
    pub const fn is_redacted(self) -> bool {
        matches!(self, Self::RedactedField)
    }
}

/// A risk marker a compare summary card may raise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareRiskMarker {
    /// The change volume is large; scale must be surfaced explicitly.
    LargeChangeVolume,
    /// A security-sensitive path changed.
    SecuritySensitivePath,
    /// A generated artifact drifted from its source.
    GeneratedArtifactDrift,
    /// Schema fidelity narrowed for part of the compare.
    SchemaFidelityNarrowed,
    /// Redacted content is present and stays hidden.
    RedactedContentPresent,
    /// Applying the compare would write back irreversibly.
    IrreversibleWriteBack,
}

impl CompareRiskMarker {
    /// Every marker, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LargeChangeVolume,
        Self::SecuritySensitivePath,
        Self::GeneratedArtifactDrift,
        Self::SchemaFidelityNarrowed,
        Self::RedactedContentPresent,
        Self::IrreversibleWriteBack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LargeChangeVolume => "large_change_volume",
            Self::SecuritySensitivePath => "security_sensitive_path",
            Self::GeneratedArtifactDrift => "generated_artifact_drift",
            Self::SchemaFidelityNarrowed => "schema_fidelity_narrowed",
            Self::RedactedContentPresent => "redacted_content_present",
            Self::IrreversibleWriteBack => "irreversible_write_back",
        }
    }
}

/// Severity attached to a risk marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskSeverity {
    /// Informational; no action required.
    Info,
    /// Caution; review before proceeding.
    Caution,
    /// Critical; blocks a safe automatic promotion.
    Critical,
}

impl RiskSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Caution => "caution",
            Self::Critical => "critical",
        }
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructureCompareControlsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// No schema recognizes the artifact class.
    SchemaUnrecognized,
    /// A rendered lens is not trusted.
    RenderUntrusted,
    /// Compare-only safety is enforced; write-back is unavailable.
    CompareOnlyEnforced,
    /// The diff is large enough that the summary may truncate the row sample.
    LargeDiffTruncationRisk,
    /// Content was redacted and narrows visible objects.
    RedactionApplied,
    /// Control trust narrowed.
    TrustNarrowing,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl StructureCompareControlsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::SchemaUnrecognized,
        Self::RenderUntrusted,
        Self::CompareOnlyEnforced,
        Self::LargeDiffTruncationRisk,
        Self::RedactionApplied,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::SchemaUnrecognized => "schema_unrecognized",
            Self::RenderUntrusted => "render_untrusted",
            Self::CompareOnlyEnforced => "compare_only_enforced",
            Self::LargeDiffTruncationRisk => "large_diff_truncation_risk",
            Self::RedactionApplied => "redaction_applied",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse these controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructureCompareControlsConsumerSurface {
    /// Diff / compare view.
    DiffCompareView,
    /// Merge / conflict resolution workspace.
    MergeConflictWorkspace,
    /// Notebook review surface.
    NotebookReview,
    /// Artifact browser (coverage, profile, crash, SBOM, lockfile adjuncts).
    ArtifactBrowser,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl StructureCompareControlsConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DiffCompareView,
        Self::MergeConflictWorkspace,
        Self::NotebookReview,
        Self::ArtifactBrowser,
        Self::CliHeadless,
        Self::SupportExport,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffCompareView => "diff_compare_view",
            Self::MergeConflictWorkspace => "merge_conflict_workspace",
            Self::NotebookReview => "notebook_review",
            Self::ArtifactBrowser => "artifact_browser",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Disclosures a structure row must carry, derived from its change kind.
///
/// Add / remove / modify state stays distinct: an added object only has a new
/// summary, a removed object only has an old summary, a modified or metadata-only
/// object has both, and a redacted-hidden object keeps its content hidden while
/// still naming that a change happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StructureRowDisclosure {
    /// Whether the row must carry an old-side summary.
    pub needs_old_summary: bool,
    /// Whether the row must carry a new-side summary.
    pub needs_new_summary: bool,
    /// Whether the object's content stays hidden (redacted) on this row.
    pub content_hidden: bool,
    /// Whether the row must carry an explicit redaction note.
    pub needs_redaction_note: bool,
}

/// Resolves the disclosures a structure row must carry from its change kind.
pub fn resolve_structure_row_disclosure(
    change_kind: StructureChangeKind,
) -> StructureRowDisclosure {
    StructureRowDisclosure {
        needs_old_summary: matches!(
            change_kind,
            StructureChangeKind::Removed
                | StructureChangeKind::Modified
                | StructureChangeKind::MetadataOnly
        ),
        needs_new_summary: matches!(
            change_kind,
            StructureChangeKind::Added
                | StructureChangeKind::Modified
                | StructureChangeKind::MetadataOnly
        ),
        content_hidden: change_kind.is_content_hidden(),
        needs_redaction_note: change_kind.is_content_hidden(),
    }
}

/// Disclosures a compare summary card must carry, derived from its counts and scale.
///
/// Scale is always surfaced through the changed-object counts, raw context always
/// stays reachable, and redaction or a producer-asserted large diff always forces
/// an explicit risk marker rather than a silently flattened roll-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompareSummaryDisclosure {
    /// Whether the card must carry a redacted-content risk marker.
    pub needs_redacted_risk_marker: bool,
    /// Whether the card must carry a large-change-volume risk marker.
    pub needs_scale_risk_marker: bool,
    /// Whether the card must keep a raw-context jump action reachable (always true).
    pub raw_context_required: bool,
}

/// Resolves the disclosures a compare summary card must carry.
pub fn resolve_compare_summary_disclosure(
    counts: &StructuredChangeCounts,
    large_diff: bool,
) -> CompareSummaryDisclosure {
    CompareSummaryDisclosure {
        needs_redacted_risk_marker: counts.redacted_hidden > 0,
        needs_scale_risk_marker: large_diff,
        raw_context_required: true,
    }
}

/// Changed-object counts carried by a compare summary card.
///
/// Counts stay broken out by change kind so the roll-up never flattens
/// add / remove / modify / metadata-only / redacted state into a single number,
/// and `total_changed_objects` must equal their sum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredChangeCounts {
    /// Objects added.
    pub added: u32,
    /// Objects removed.
    pub removed: u32,
    /// Objects modified.
    pub modified: u32,
    /// Objects with metadata-only changes.
    pub metadata_only: u32,
    /// Objects changed but redacted and hidden.
    pub redacted_hidden: u32,
    /// Total changed objects; must equal the sum of the per-kind counts.
    pub total_changed_objects: u32,
}

impl StructuredChangeCounts {
    /// The sum of the per-kind counts.
    pub const fn computed_total(&self) -> u32 {
        self.added + self.removed + self.modified + self.metadata_only + self.redacted_hidden
    }

    /// Whether `total_changed_objects` matches the sum of the per-kind counts.
    pub const fn is_consistent(&self) -> bool {
        self.total_changed_objects == self.computed_total()
    }
}

/// A risk marker with its severity and human-readable explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RiskMarkerNote {
    /// The risk marker being raised.
    pub marker: CompareRiskMarker,
    /// Severity attached to the marker.
    pub severity: RiskSeverity,
    /// Human-readable explanation; required and non-empty.
    pub note: String,
}

/// A structure / key-path row naming an object, its change kind, and its summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureRow {
    /// Frozen component this row implements; must be `structure_row`.
    pub component: M5ArtifactComponent,
    /// Stable row id.
    pub row_id: String,
    /// Stable artifact reference shared with the paired compare summary card.
    pub artifact_ref: String,
    /// Object identity / key-path; required and non-empty.
    pub object_path: String,
    /// The category of structured object this row describes.
    pub object_category: StructuredObjectCategory,
    /// The change kind: added / removed / modified / metadata-only / redacted-hidden.
    pub change_kind: StructureChangeKind,
    /// Old-side summary; required for removed / modified / metadata-only rows.
    pub old_summary: String,
    /// New-side summary; required for added / modified / metadata-only rows.
    pub new_summary: String,
    /// Confidence or schema note backing the reading; required and non-empty.
    pub confidence_or_schema_note: String,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Redaction note; required and non-empty when the row is redacted-hidden.
    pub redaction_note: String,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Row fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl StructureRow {
    /// Disclosures this row must carry, derived from its change kind.
    pub fn disclosure(&self) -> StructureRowDisclosure {
        resolve_structure_row_disclosure(self.change_kind)
    }

    /// Whether the object category and change kind agree about redaction.
    ///
    /// A redacted field must carry a redacted-hidden change kind, and a
    /// redacted-hidden change kind must describe a redacted field, so redacted
    /// content is never shown under a category that implies visible content.
    pub fn category_kind_consistent(&self) -> bool {
        self.object_category.is_redacted() == self.change_kind.is_content_hidden()
    }
}

/// A compare summary card rolling up changed-object counts, scale, and risk.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareSummaryCard {
    /// Frozen component this card implements; must be `compare_summary_card`.
    pub component: M5ArtifactComponent,
    /// Stable card id.
    pub card_id: String,
    /// Stable artifact reference shared with the paired structure rows.
    pub artifact_ref: String,
    /// Human-readable artifact-class label.
    pub artifact_class_label: String,
    /// Changed-object counts, broken out by change kind.
    pub change_counts: StructuredChangeCounts,
    /// Whether the producer marks this as a large diff whose scale must be flagged.
    pub large_diff: bool,
    /// Risk markers raised on this card, each with a required explanation.
    pub risk_markers: Vec<RiskMarkerNote>,
    /// Confidence or schema summary note; required and non-empty.
    pub confidence_or_schema_note: String,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Compare-only-versus-write-back safety disclosure; required and non-empty.
    pub compare_write_back_safety: String,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Card fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
}

impl CompareSummaryCard {
    /// Disclosures this card must carry, derived from its counts and scale.
    pub fn disclosure(&self) -> CompareSummaryDisclosure {
        resolve_compare_summary_disclosure(&self.change_counts, self.large_diff)
    }

    /// Whether the card raises the given risk marker.
    pub fn has_marker(&self, marker: CompareRiskMarker) -> bool {
        self.risk_markers.iter().any(|note| note.marker == marker)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureCompareControlsTrustReview {
    /// Object identity is always explicit on a structure row.
    pub object_identity_always_explicit: bool,
    /// Add / remove / modify state stays distinct per object.
    pub change_kind_distinct_per_object: bool,
    /// Old / new summaries are preserved for the change kinds that carry them.
    pub old_new_summary_preserved: bool,
    /// A raw-context jump action is always reachable from both controls.
    pub raw_context_always_reachable: bool,
    /// Scale is surfaced through counts without flattening the diff.
    pub scale_surfaced_without_flattening: bool,
    /// Risk markers always carry an explanation.
    pub risk_markers_explained: bool,
    /// Redacted content is shown as hidden and never leaked into a summary.
    pub redacted_content_never_leaked: bool,
    /// Confidence or schema notes stay explicit.
    pub confidence_or_schema_note_explicit: bool,
    /// Compare-only artifacts are never silently promoted to writable state.
    pub compare_only_never_silently_writable: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified controls automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl StructureCompareControlsTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_identity_always_explicit
            && self.change_kind_distinct_per_object
            && self.old_new_summary_preserved
            && self.raw_context_always_reachable
            && self.scale_surfaced_without_flattening
            && self.risk_markers_explained
            && self.redacted_content_never_leaked
            && self.confidence_or_schema_note_explicit
            && self.compare_only_never_silently_writable
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureCompareControlsConsumerProjection {
    /// Structure row shows object identity and change kind.
    pub structure_row_shows_object_and_change_kind: bool,
    /// Compare card shows counts and risk markers.
    pub compare_card_shows_counts_and_risk: bool,
    /// Raw context is reachable from both the row and the card.
    pub raw_context_reachable_from_both: bool,
    /// Redacted fields are shown as hidden rather than dropped.
    pub redacted_fields_shown_as_hidden_not_dropped: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_truth: bool,
}

impl StructureCompareControlsConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.structure_row_shows_object_and_change_kind
            && self.compare_card_shows_counts_and_risk
            && self.raw_context_reachable_from_both
            && self.redacted_fields_shown_as_hidden_not_dropped
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.help_about_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureCompareControlsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`StructureCompareControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureCompareControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Structure / key-path rows.
    pub structure_rows: Vec<StructureRow>,
    /// Compare summary cards.
    pub compare_summary_cards: Vec<CompareSummaryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<StructureCompareControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<StructureCompareControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: StructureCompareControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: StructureCompareControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StructureCompareControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe structure-row / compare-summary controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureCompareControlsPacket {
    /// Record kind; must equal [`STRUCTURE_COMPARE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STRUCTURE_COMPARE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Structure / key-path rows.
    pub structure_rows: Vec<StructureRow>,
    /// Compare summary cards.
    pub compare_summary_cards: Vec<CompareSummaryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<StructureCompareControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<StructureCompareControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: StructureCompareControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: StructureCompareControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: StructureCompareControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl StructureCompareControlsPacket {
    /// Builds a structure-row / compare-summary controls packet from stable-lane input.
    pub fn new(input: StructureCompareControlsPacketInput) -> Self {
        Self {
            record_kind: STRUCTURE_COMPARE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: STRUCTURE_COMPARE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            structure_rows: input.structure_rows,
            compare_summary_cards: input.compare_summary_cards,
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

    /// Validates the structure-row / compare-summary controls invariants.
    pub fn validate(&self) -> Vec<StructureCompareControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != STRUCTURE_COMPARE_CONTROLS_RECORD_KIND {
            violations.push(StructureCompareControlsViolation::WrongRecordKind);
        }
        if self.schema_version != STRUCTURE_COMPARE_CONTROLS_SCHEMA_VERSION {
            violations.push(StructureCompareControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(StructureCompareControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(StructureCompareControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(StructureCompareControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_structure_rows(self, &mut violations);
        validate_compare_summary_cards(self, &mut violations);
        validate_pairing(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(StructureCompareControlsViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(StructureCompareControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(StructureCompareControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("structure compare controls packet serializes"),
        ) {
            violations.push(StructureCompareControlsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("structure compare controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let redacted_rows = self
            .structure_rows
            .iter()
            .filter(|row| row.change_kind.is_content_hidden())
            .count();
        let total_changed: u32 = self
            .compare_summary_cards
            .iter()
            .map(|card| card.change_counts.total_changed_objects)
            .sum();

        let mut out = String::new();
        out.push_str("# Structure Rows & Compare Summary Cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Structure rows: {} ({} redacted-hidden)\n",
            self.structure_rows.len(),
            redacted_rows
        ));
        out.push_str(&format!(
            "- Compare summary cards: {} ({} changed objects total)\n",
            self.compare_summary_cards.len(),
            total_changed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Structure rows\n\n");
        for row in &self.structure_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} ({}) — {}\n",
                row.object_path,
                row.artifact_ref,
                row.change_kind.as_str(),
                row.object_category.as_str(),
                row.confidence_or_schema_note
            ));
        }

        out.push_str("\n## Compare summary cards\n\n");
        for card in &self.compare_summary_cards {
            let markers = card
                .risk_markers
                .iter()
                .map(|note| format!("{}={}", note.marker.as_str(), note.severity.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "- **{}** [`{}`]: {} changed (+{}/-{}/~{}/meta {}/redacted {}) — risk [{}]\n",
                card.artifact_class_label,
                card.artifact_ref,
                card.change_counts.total_changed_objects,
                card.change_counts.added,
                card.change_counts.removed,
                card.change_counts.modified,
                card.change_counts.metadata_only,
                card.change_counts.redacted_hidden,
                markers
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in structure/compare-controls export.
#[derive(Debug)]
pub enum StructureCompareControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StructureCompareControlsViolation>),
}

impl fmt::Display for StructureCompareControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "structure compare controls export parse failed: {error}"
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
                    "structure compare controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for StructureCompareControlsArtifactError {}

/// Validation failures emitted by [`StructureCompareControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructureCompareControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No structure rows are present.
    StructureRowsMissing,
    /// A structure row is incomplete.
    StructureRowIncomplete,
    /// A structure row carries the wrong frozen component class.
    StructureRowWrongComponentClass,
    /// A structure row does not name its object identity / key-path.
    ObjectIdentityMissing,
    /// A structure row is missing an old / new summary its change kind requires.
    ChangeSummaryMissing,
    /// A redacted-hidden row leaks content into an old / new summary.
    HiddenContentLeaked,
    /// A redacted-hidden row does not carry an explicit redaction note.
    RedactionNoteMissing,
    /// A structure row does not carry a confidence or schema note.
    ConfidenceNoteMissing,
    /// A structure row or compare card does not carry a raw-context jump action.
    RawContextActionMissing,
    /// A row's object category and change kind disagree about redaction.
    CategoryChangeKindInconsistent,
    /// The structure rows do not cover the added, removed, and modified change kinds.
    StructureChangeKindCoverageMissing,
    /// No compare summary cards are present.
    CompareSummaryCardsMissing,
    /// A compare summary card is incomplete.
    CompareSummaryCardIncomplete,
    /// A compare summary card carries the wrong frozen component class.
    CompareSummaryCardWrongComponentClass,
    /// A compare summary card's total does not match its per-kind counts.
    ChangeCountsInconsistent,
    /// A compare summary card rolls up zero changed objects.
    EmptyCompareSummary,
    /// A large-diff card does not carry a large-change-volume risk marker.
    ScaleRiskMarkerMissing,
    /// A card with redacted objects does not carry a redacted-content risk marker.
    RedactedRiskMarkerMissing,
    /// A risk marker does not carry an explanation.
    RiskMarkerNoteMissing,
    /// A compare summary card does not name its compare-only-versus-write-back safety.
    CompareWriteBackSafetyMissing,
    /// A compare summary card does not carry a confidence or schema note.
    CompareSummaryConfidenceNoteMissing,
    /// Structure rows and compare summary cards are not paired by artifact reference.
    ComparePairingIncomplete,
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

impl StructureCompareControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::StructureRowsMissing => "structure_rows_missing",
            Self::StructureRowIncomplete => "structure_row_incomplete",
            Self::StructureRowWrongComponentClass => "structure_row_wrong_component_class",
            Self::ObjectIdentityMissing => "object_identity_missing",
            Self::ChangeSummaryMissing => "change_summary_missing",
            Self::HiddenContentLeaked => "hidden_content_leaked",
            Self::RedactionNoteMissing => "redaction_note_missing",
            Self::ConfidenceNoteMissing => "confidence_note_missing",
            Self::RawContextActionMissing => "raw_context_action_missing",
            Self::CategoryChangeKindInconsistent => "category_change_kind_inconsistent",
            Self::StructureChangeKindCoverageMissing => "structure_change_kind_coverage_missing",
            Self::CompareSummaryCardsMissing => "compare_summary_cards_missing",
            Self::CompareSummaryCardIncomplete => "compare_summary_card_incomplete",
            Self::CompareSummaryCardWrongComponentClass => {
                "compare_summary_card_wrong_component_class"
            }
            Self::ChangeCountsInconsistent => "change_counts_inconsistent",
            Self::EmptyCompareSummary => "empty_compare_summary",
            Self::ScaleRiskMarkerMissing => "scale_risk_marker_missing",
            Self::RedactedRiskMarkerMissing => "redacted_risk_marker_missing",
            Self::RiskMarkerNoteMissing => "risk_marker_note_missing",
            Self::CompareWriteBackSafetyMissing => "compare_write_back_safety_missing",
            Self::CompareSummaryConfidenceNoteMissing => "compare_summary_confidence_note_missing",
            Self::ComparePairingIncomplete => "compare_pairing_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable structure/compare-controls export.
pub fn current_structure_compare_controls_export(
) -> Result<StructureCompareControlsPacket, StructureCompareControlsArtifactError> {
    let packet: StructureCompareControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-structure-compare-summary-controls-proof/support_export.json"
    )))
    .map_err(StructureCompareControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StructureCompareControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &StructureCompareControlsPacket,
    violations: &mut Vec<StructureCompareControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        STRUCTURE_COMPARE_CONTROLS_SCHEMA_REF,
        STRUCTURE_COMPARE_CONTROLS_DOC_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_STRUCTURE_ROW_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_COMPARE_SUMMARY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(StructureCompareControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_structure_rows(
    packet: &StructureCompareControlsPacket,
    violations: &mut Vec<StructureCompareControlsViolation>,
) {
    if packet.structure_rows.is_empty() {
        violations.push(StructureCompareControlsViolation::StructureRowsMissing);
        return;
    }

    let mut kinds: BTreeSet<StructureChangeKind> = BTreeSet::new();

    for row in &packet.structure_rows {
        kinds.insert(row.change_kind);

        if row.row_id.trim().is_empty()
            || row.artifact_ref.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(StructureCompareControlsViolation::StructureRowIncomplete);
        }
        if row.component != M5ArtifactComponent::StructureRow {
            violations.push(StructureCompareControlsViolation::StructureRowWrongComponentClass);
        }
        if row.object_path.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::ObjectIdentityMissing);
        }
        if row.confidence_or_schema_note.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::ConfidenceNoteMissing);
        }
        if row.raw_context_action.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::RawContextActionMissing);
        }

        let disclosure = row.disclosure();

        if disclosure.needs_old_summary && row.old_summary.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::ChangeSummaryMissing);
        }
        if disclosure.needs_new_summary && row.new_summary.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::ChangeSummaryMissing);
        }
        if disclosure.content_hidden
            && (!row.old_summary.trim().is_empty() || !row.new_summary.trim().is_empty())
        {
            violations.push(StructureCompareControlsViolation::HiddenContentLeaked);
        }
        if disclosure.needs_redaction_note && row.redaction_note.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::RedactionNoteMissing);
        }
        if !row.category_kind_consistent() {
            violations.push(StructureCompareControlsViolation::CategoryChangeKindInconsistent);
        }
    }

    for required in [
        StructureChangeKind::Added,
        StructureChangeKind::Removed,
        StructureChangeKind::Modified,
    ] {
        if !kinds.contains(&required) {
            violations.push(StructureCompareControlsViolation::StructureChangeKindCoverageMissing);
            break;
        }
    }
}

fn validate_compare_summary_cards(
    packet: &StructureCompareControlsPacket,
    violations: &mut Vec<StructureCompareControlsViolation>,
) {
    if packet.compare_summary_cards.is_empty() {
        violations.push(StructureCompareControlsViolation::CompareSummaryCardsMissing);
        return;
    }

    for card in &packet.compare_summary_cards {
        if card.card_id.trim().is_empty()
            || card.artifact_ref.trim().is_empty()
            || card.artifact_class_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(StructureCompareControlsViolation::CompareSummaryCardIncomplete);
        }
        if card.component != M5ArtifactComponent::CompareSummaryCard {
            violations
                .push(StructureCompareControlsViolation::CompareSummaryCardWrongComponentClass);
        }
        if card.confidence_or_schema_note.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::CompareSummaryConfidenceNoteMissing);
        }
        if card.compare_write_back_safety.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::CompareWriteBackSafetyMissing);
        }
        if card.raw_context_action.trim().is_empty() {
            violations.push(StructureCompareControlsViolation::RawContextActionMissing);
        }
        if !card.change_counts.is_consistent() {
            violations.push(StructureCompareControlsViolation::ChangeCountsInconsistent);
        }
        if card.change_counts.computed_total() == 0 {
            violations.push(StructureCompareControlsViolation::EmptyCompareSummary);
        }

        let disclosure = card.disclosure();
        if disclosure.needs_scale_risk_marker
            && !card.has_marker(CompareRiskMarker::LargeChangeVolume)
        {
            violations.push(StructureCompareControlsViolation::ScaleRiskMarkerMissing);
        }
        if disclosure.needs_redacted_risk_marker
            && !card.has_marker(CompareRiskMarker::RedactedContentPresent)
        {
            violations.push(StructureCompareControlsViolation::RedactedRiskMarkerMissing);
        }

        for note in &card.risk_markers {
            if note.note.trim().is_empty() {
                violations.push(StructureCompareControlsViolation::RiskMarkerNoteMissing);
            }
        }
    }
}

fn validate_pairing(
    packet: &StructureCompareControlsPacket,
    violations: &mut Vec<StructureCompareControlsViolation>,
) {
    let row_refs: BTreeSet<&str> = packet
        .structure_rows
        .iter()
        .map(|row| row.artifact_ref.as_str())
        .collect();
    let card_refs: BTreeSet<&str> = packet
        .compare_summary_cards
        .iter()
        .map(|card| card.artifact_ref.as_str())
        .collect();
    if row_refs != card_refs {
        violations.push(StructureCompareControlsViolation::ComparePairingIncomplete);
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
