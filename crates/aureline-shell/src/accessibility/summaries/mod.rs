//! Non-visual semantic summaries and drill-down navigation for the claimed M5
//! custom-rendered surfaces whose meaning would otherwise depend on visual density.
//!
//! Where the per-surface descriptors
//! ([`crate::accessibility`]) bind a custom surface to its semantic roles, label
//! model, and OS bridge mapping, and the event-class coverage catalog
//! ([`crate::accessibility::events`]) materializes *which dynamic events* each
//! workflow narrates, this module materializes *how each custom surface explains its
//! own structure and current fidelity non-visually*. One [`M5SurfaceSummary`] row per
//! claimed custom surface — custom-rendered editors, terminal/log canvases, dense
//! tables/trees, logs/traces, charts, and rich review/artifact viewers — binds a
//! stable surface id to the SAME object identity the visual surface carries, a
//! quantified structure summary (so the description is never a vague one-liner), a set
//! of object-linked drill-down routes a professional follows by keyboard alone, an
//! export-safe text alternative plus metadata view for surfaces whose visual state
//! materially affects decisions, and the current preview/cached/generated/approximate/
//! sampled/buffered presentation state so provisional truth stays visible in the
//! non-visual representation rather than only in the visual chrome.
//!
//! The catalog is the single M5 source for non-visual custom-surface *summary* truth:
//! editor, terminal, data, observability, review, and docs surfaces project these
//! summaries rather than improvising per-surface prose; diagnostics, support exports,
//! docs/help, and assistive-tech conformance packets reuse the same rows so a
//! non-visual-summary regression is debuggable from the support export alone. The
//! guardrail is enforced structurally: every surface carries a quantified structure
//! plus at least two object-linked, keyboard-reachable drill-down routes, never a
//! vague one-line summary; every visual-decision surface provides a text alternative
//! and an export-safe metadata view; and every summary stays linked to the same object
//! identity and freshness/fidelity state as the visual surface. When a surface's bridge
//! or proof state goes stale the claimed summary auto-narrows rather than implying
//! silent screen-reader completeness.
//!
//! The controlled state vocabularies — semantic role class, non-visual fidelity,
//! qualification class, downgrade trigger, consumer surface, proof/release posture —
//! are reused verbatim from the frozen dynamic-surface matrix, and the durable
//! fallback surface tokens from the announcement grammar, rather than minting parallel
//! tokens. Only the summary-shaped vocabularies this lane adds (surface kind, summary
//! producer, presentation state, drill-down kind, and text-alternative kind) are
//! minted here and frozen in a self-describing [`M5NonVisualSummaryVocabularySet`].
//! Raw provider payloads, credentials, secret material, screenshots, and untranslated
//! free-text prose stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/a11y/m5-nonvisual-summaries.schema.json`](../../../../../schemas/a11y/m5-nonvisual-summaries.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-custom-surface-summaries.md`](../../../../../docs/a11y/m5-custom-surface-summaries.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-nonvisual-summaries/`](../../../../../fixtures/a11y/m5-nonvisual-summaries/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_nonvisual_summary_catalog,
    seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed,
    seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed,
    M5_NONVISUAL_SUMMARY_CATALOG_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The announcement grammar owns the canonical durable-fallback-surface vocabulary;
// route every summary's durable fallback through it rather than minting synonyms.
use crate::announcement_grammar as grammar;
// The frozen matrix owns the shared state vocabularies, qualification classes,
// downgrade triggers, consumer surfaces, and proof/release posture.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use grammar::{M5DurableFallbackRef, M5DurableFallbackSurface};
pub use matrix::{
    A11yNonVisualFidelity, A11ySemanticRoleClass, M5DynamicSurfaceA11yConsumerSurface,
    M5DynamicSurfaceA11yDowngradeTrigger, M5DynamicSurfaceA11yProofFreshness,
    M5DynamicSurfaceA11yQualificationClass, M5DynamicSurfaceA11yReleasePosture,
    M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5NonVisualSummaryCatalogPacket`].
pub const M5_NONVISUAL_SUMMARY_RECORD_KIND: &str = "m5_nonvisual_summary_catalog";

/// Schema version for M5 non-visual summary catalogs.
pub const M5_NONVISUAL_SUMMARY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_NONVISUAL_SUMMARY_SCHEMA_REF: &str = "schemas/a11y/m5-nonvisual-summaries.schema.json";

/// Repo-relative path of the M5 non-visual summary contract doc.
pub const M5_NONVISUAL_SUMMARY_DOC_REF: &str = "docs/a11y/m5-custom-surface-summaries.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that governs
/// this lane's shared controlled vocabularies and qualification classes.
pub const M5_NONVISUAL_SUMMARY_MATRIX_REF: &str =
    "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the live-announcement grammar that owns the durable-fallback
/// surface vocabulary these summaries resolve their fallbacks against.
pub const M5_NONVISUAL_SUMMARY_ANNOUNCEMENT_GRAMMAR_REF: &str =
    "schemas/a11y/m5-announcement-grammar.schema.json";

/// Repo-relative path of the per-surface accessibility descriptors these summaries
/// share their object identity and bridge state with.
pub const M5_NONVISUAL_SUMMARY_SURFACE_DESCRIPTOR_REF: &str =
    "schemas/a11y/m5-surface-descriptors.schema.json";

/// Repo-relative path of the frozen screen-reader announcement / live-region contract.
pub const M5_NONVISUAL_SUMMARY_SCREEN_READER_CONTRACT_REF: &str =
    "docs/accessibility/screen_reader_and_live_region_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_NONVISUAL_SUMMARY_FIXTURE_DIR: &str = "fixtures/a11y/m5-nonvisual-summaries";

/// Repo-relative path of the checked support-export artifact.
pub const M5_NONVISUAL_SUMMARY_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-nonvisual-summary-proof/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_NONVISUAL_SUMMARY_SUMMARY_REF: &str =
    "artifacts/a11y/m5-nonvisual-summary-proof/nonvisual-summary-proof.md";

/// Stable prefix every summary-owned message id carries (structure, dimensions,
/// drill-down routes, and text alternatives).
pub const M5_SUMMARY_MESSAGE_ID_PREFIX: &str = "summary.";

/// Minimum number of drill-down routes a claimed summary must carry.
///
/// The guardrail bars replacing detailed drill-down navigation with a vague one-line
/// summary, so every surface must offer at least this many object-linked routes.
pub const M5_SUMMARY_MIN_DRILLDOWNS: usize = 2;

/// Kind of custom-rendered surface that earns a non-visual summary.
///
/// These are exactly the claimed M5 surfaces whose meaning depends on visual density:
/// custom editors, terminal/log canvases, dense tables/trees, logs/traces, charts, and
/// rich review/artifact viewers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummarySurfaceKind {
    /// Custom-rendered code editor.
    CustomEditor,
    /// Terminal / shell-integration canvas.
    TerminalCanvas,
    /// Dense data grid / table.
    DataGrid,
    /// Tree / outline.
    TreeOutline,
    /// Streaming log surface.
    LogStream,
    /// Trace / span timeline.
    TraceTimeline,
    /// Chart / visualization.
    Chart,
    /// Rich review / diff viewer.
    ReviewDiff,
    /// Image / design / rich artifact viewer.
    ArtifactViewer,
}

impl M5SummarySurfaceKind {
    /// Every governed surface kind, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CustomEditor,
        Self::TerminalCanvas,
        Self::DataGrid,
        Self::TreeOutline,
        Self::LogStream,
        Self::TraceTimeline,
        Self::Chart,
        Self::ReviewDiff,
        Self::ArtifactViewer,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomEditor => "custom_editor",
            Self::TerminalCanvas => "terminal_canvas",
            Self::DataGrid => "data_grid",
            Self::TreeOutline => "tree_outline",
            Self::LogStream => "log_stream",
            Self::TraceTimeline => "trace_timeline",
            Self::Chart => "chart",
            Self::ReviewDiff => "review_diff",
            Self::ArtifactViewer => "artifact_viewer",
        }
    }

    /// True when the surface's visual state materially affects decisions, so it must
    /// carry a text alternative and an export-safe metadata view rather than relying on
    /// pixels alone (charts, traces, rich diffs, and image/design artifact viewers).
    pub const fn requires_text_alternative(self) -> bool {
        matches!(
            self,
            Self::TraceTimeline | Self::Chart | Self::ReviewDiff | Self::ArtifactViewer
        )
    }
}

/// Producer crate that originates a custom surface getting a non-visual summary.
///
/// These are the first real custom-surface consumers needed to prove the pattern; each
/// maps to a claimed M5 surface whose truth would otherwise be pixel-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummaryProducer {
    /// Custom-rendered editor (`aureline-editor`).
    Editor,
    /// Terminal / log canvas (`aureline-terminal`).
    Terminal,
    /// Dense data grid / tree (`aureline-data`).
    Data,
    /// Logs / traces / charts (`aureline-observability`).
    Observability,
    /// Review / diff surface (`aureline-review`).
    Review,
    /// Docs / artifact rendering (`aureline-docs`).
    Docs,
}

impl M5SummaryProducer {
    /// Every producer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Editor,
        Self::Terminal,
        Self::Data,
        Self::Observability,
        Self::Review,
        Self::Docs,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Terminal => "terminal",
            Self::Data => "data",
            Self::Observability => "observability",
            Self::Review => "review",
            Self::Docs => "docs",
        }
    }
}

/// Current presentation state of a custom surface's content.
///
/// `authoritative` is complete, current truth. Every other token names a provisional
/// state the spec requires to stay visible in the non-visual representation rather than
/// only in the visual chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummaryPresentationState {
    /// Complete, current truth.
    Authoritative,
    /// A preview render that may differ from the committed result.
    Preview,
    /// Served from a cache that may lag the source.
    Cached,
    /// Auto/AI-generated content, not user-authored.
    Generated,
    /// Approximate or estimated values.
    Approximate,
    /// A sampled / downsampled subset of the full data.
    Sampled,
    /// Buffered / streaming content with more data still pending.
    Buffered,
}

impl M5SummaryPresentationState {
    /// Every presentation state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Authoritative,
        Self::Preview,
        Self::Cached,
        Self::Generated,
        Self::Approximate,
        Self::Sampled,
        Self::Buffered,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Preview => "preview",
            Self::Cached => "cached",
            Self::Generated => "generated",
            Self::Approximate => "approximate",
            Self::Sampled => "sampled",
            Self::Buffered => "buffered",
        }
    }

    /// True when this state is provisional (anything other than authoritative truth).
    pub const fn is_provisional(self) -> bool {
        !matches!(self, Self::Authoritative)
    }
}

/// Kind of drill-down navigation route a summary offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummaryDrillDownKind {
    /// Enumerate the surface's top-level structure (rows/columns/nodes/regions/series).
    EnumerateStructure,
    /// Open the focused item's full detail.
    OpenItemDetail,
    /// Jump to a semantically named region (error block, diff hunk, marker).
    JumpToRegion,
    /// Describe a data series / column / channel non-visually.
    DescribeSeries,
    /// Read the export-safe text alternative for a visual artifact.
    ReadTextAlternative,
    /// Open the export-safe metadata view.
    OpenMetadataView,
}

impl M5SummaryDrillDownKind {
    /// Every drill-down kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EnumerateStructure,
        Self::OpenItemDetail,
        Self::JumpToRegion,
        Self::DescribeSeries,
        Self::ReadTextAlternative,
        Self::OpenMetadataView,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnumerateStructure => "enumerate_structure",
            Self::OpenItemDetail => "open_item_detail",
            Self::JumpToRegion => "jump_to_region",
            Self::DescribeSeries => "describe_series",
            Self::ReadTextAlternative => "read_text_alternative",
            Self::OpenMetadataView => "open_metadata_view",
        }
    }
}

/// Kind of text alternative a surface exposes for visual-only state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SummaryTextAlternativeKind {
    /// A text-native surface needs no image/chart alternative.
    NotApplicable,
    /// Description of a chart / trace timeline (axes, series, trends, critical path).
    ChartDescription,
    /// Alt text for an image artifact.
    ImageAltText,
    /// Export-safe metadata view for a design / rich artifact.
    DesignSpecMetadata,
    /// Summary of a rich review diff.
    DiffSummary,
}

impl M5SummaryTextAlternativeKind {
    /// Every text-alternative kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotApplicable,
        Self::ChartDescription,
        Self::ImageAltText,
        Self::DesignSpecMetadata,
        Self::DiffSummary,
    ];

    /// Stable token recorded in the catalog.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::ChartDescription => "chart_description",
            Self::ImageAltText => "image_alt_text",
            Self::DesignSpecMetadata => "design_spec_metadata",
            Self::DiffSummary => "diff_summary",
        }
    }

    /// True when this kind supplies a real text alternative for visual state.
    pub const fn is_applicable(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// One quantified structural dimension of a surface (e.g. "rows", "columns",
/// "depth", "series"), carried by a stable message id rather than a pixel measurement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SummaryDimension {
    /// Dimension name (a stable, translatable label key fragment).
    pub name: String,
    /// Stable message id carrying this dimension's live count; prefixed
    /// [`M5_SUMMARY_MESSAGE_ID_PREFIX`].
    pub dimension_message_id: String,
}

/// Quantified structure summary for one surface.
///
/// The structure is never a vague one-liner: it names the surface's semantic role and
/// enumerates the quantified dimensions a professional needs to understand its shape
/// without vision or pointer hover.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SummaryStructure {
    /// Stable message id for the structure summary; prefixed
    /// [`M5_SUMMARY_MESSAGE_ID_PREFIX`].
    pub structure_message_id: String,
    /// Semantic role class of the surface's structure (matrix-owned vocabulary).
    pub role_class: A11ySemanticRoleClass,
    /// Quantified structural dimensions; at least one is required.
    pub dimensions: Vec<M5SummaryDimension>,
}

/// One object-linked, keyboard-reachable drill-down route into a surface's structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SummaryDrillDown {
    /// Stable drill-down id, unique within the catalog.
    pub drilldown_id: String,
    /// Kind of drill-down route.
    pub kind: M5SummaryDrillDownKind,
    /// Human-readable route label.
    pub label: String,
    /// Stable message id for the route; prefixed [`M5_SUMMARY_MESSAGE_ID_PREFIX`].
    pub route_message_id: String,
    /// Object-identity ref the route lands on, keeping the drill-down object-linked.
    pub target_identity_ref: String,
    /// True when the route is reachable by keyboard alone (no pointer hover).
    pub keyboard_reachable: bool,
}

/// Export-safe text alternative plus metadata view for a surface's visual state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SummaryTextAlternative {
    /// Kind of text alternative.
    pub kind: M5SummaryTextAlternativeKind,
    /// True when this surface provides a text alternative for visual-only state.
    pub provided: bool,
    /// Stable message id for the alt text; prefixed [`M5_SUMMARY_MESSAGE_ID_PREFIX`]
    /// when provided, empty when not applicable.
    pub alt_text_message_id: String,
    /// Export-safe metadata field names exposed in the non-visual metadata view; empty
    /// when not applicable.
    pub export_metadata_fields: Vec<String>,
}

/// One non-visual summary row for a claimed M5 custom surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceSummary {
    /// Stable summary id, unique within the catalog.
    pub summary_id: String,
    /// Governed surface kind.
    pub surface_kind: M5SummarySurfaceKind,
    /// Human-readable surface label.
    pub label: String,
    /// Owner role accountable for keeping this summary current.
    pub owner_role: String,
    /// Object identity this summary is bound to — the SAME identity as the visual
    /// surface, so the non-visual representation can never drift from the object.
    pub object_identity_ref: String,
    /// Qualification class earned by this summary.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Non-visual fidelity the summary currently delivers (matrix-owned).
    pub non_visual_fidelity: A11yNonVisualFidelity,
    /// Current presentation state (preview/cached/generated/approximate/sampled/
    /// buffered/authoritative), kept visible in the non-visual representation.
    pub presentation_state: M5SummaryPresentationState,
    /// Producer crates that originate this surface.
    pub producers: Vec<M5SummaryProducer>,
    /// Quantified structure summary.
    pub structure: M5SummaryStructure,
    /// Object-linked drill-down routes; at least [`M5_SUMMARY_MIN_DRILLDOWNS`].
    pub drilldowns: Vec<M5SummaryDrillDown>,
    /// Text alternative plus export-safe metadata view.
    pub text_alternative: M5SummaryTextAlternative,
    /// Durable fallback surface that preserves this summary's identity (grammar-owned).
    pub durable_fallback: M5DurableFallbackRef,
    /// Downgrade triggers that can narrow this summary below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this summary current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this summary.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that project this summary's truth.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

/// Self-describing controlled-vocabulary set for the summary-shaped tokens this lane
/// mints (the state tokens live in the matrix; the durable-fallback tokens live in the
/// grammar).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NonVisualSummaryVocabularySet {
    /// Surface-kind tokens.
    pub surface_kinds: Vec<String>,
    /// Summary-producer tokens.
    pub summary_producers: Vec<String>,
    /// Presentation-state tokens.
    pub presentation_states: Vec<String>,
    /// Drill-down-kind tokens.
    pub drilldown_kinds: Vec<String>,
    /// Text-alternative-kind tokens.
    pub text_alternative_kinds: Vec<String>,
}

impl M5NonVisualSummaryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_kinds: M5SummarySurfaceKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            summary_producers: M5SummaryProducer::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            presentation_states: M5SummaryPresentationState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            drilldown_kinds: M5SummaryDrillDownKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            text_alternative_kinds: M5SummaryTextAlternativeKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block for the non-visual summary lane.
///
/// Every flag is a hard invariant; all must hold for the catalog to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NonVisualSummaryConformanceReview {
    /// Claimed surfaces explain their structure without vision or pointer hover.
    pub surfaces_explain_structure_without_vision_or_hover: bool,
    /// Drill-down navigation stays actionable, not a vague one-line summary.
    pub drilldowns_remain_actionable_not_vague_one_liners: bool,
    /// Summaries stay linked to the same object identity as the visual surface.
    pub summaries_linked_to_same_object_identity_as_visual: bool,
    /// Preview/cached/generated/approximate/sampled/buffered states stay visible.
    pub provisional_states_visible_in_non_visual_representation: bool,
    /// Visual-decision surfaces provide a text alternative and a metadata view.
    pub visual_artifacts_provide_text_alternative_and_metadata_view: bool,
    /// Chart/trace and artifact viewers no longer require visual interpretation alone.
    pub chart_and_artifact_viewers_no_longer_require_visual_interpretation_alone: bool,
    /// No summary sources its truth from a pixel-only render or pointer-only cue.
    pub no_pixel_only_or_pointer_only_truth_source: bool,
    /// Claimed surfaces auto-narrow when bridge or proof state goes stale.
    pub claimed_surfaces_auto_narrow_when_bridge_or_proof_stale: bool,
    /// Downgrade narrows the claim rather than hiding the surface.
    pub downgrade_narrows_instead_of_hides: bool,
}

/// Consumer projection block: who projects the non-visual summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NonVisualSummaryConsumerProjection {
    /// Editor projects its custom-editor summary.
    pub editor_consumes_summaries: bool,
    /// Terminal projects its terminal/log canvas summary.
    pub terminal_consumes_summaries: bool,
    /// Data grid and tree project their dense-collection summaries.
    pub data_grid_and_tree_consume_summaries: bool,
    /// Observability projects its log/trace/chart summaries.
    pub observability_logs_traces_charts_consume_summaries: bool,
    /// Review and artifact viewers project their rich-viewer summaries.
    pub review_and_artifact_viewers_consume_summaries: bool,
    /// Docs / help reuse the summaries.
    pub docs_help_reuse_summaries: bool,
    /// Support export reuses the summaries.
    pub support_export_reuses_summaries: bool,
    /// Assistive-tech conformance packets reuse the summaries.
    pub at_conformance_packets_reuse_summaries: bool,
}

/// Constructor input for [`M5NonVisualSummaryCatalogPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5NonVisualSummaryCatalogPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Surface summary rows.
    pub summaries: Vec<M5SurfaceSummary>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Summary-shaped controlled-vocabulary set.
    pub summary_vocabulary_set: M5NonVisualSummaryVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5NonVisualSummaryConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NonVisualSummaryConsumerProjection,
    /// Proof freshness block (reused from the matrix lane).
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture (reused from the matrix lane).
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 non-visual summary catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NonVisualSummaryCatalogPacket {
    /// Record kind; must equal [`M5_NONVISUAL_SUMMARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_NONVISUAL_SUMMARY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalog label.
    pub catalog_label: String,
    /// Surface summary rows.
    pub summaries: Vec<M5SurfaceSummary>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Summary-shaped controlled-vocabulary set.
    pub summary_vocabulary_set: M5NonVisualSummaryVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5NonVisualSummaryConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5NonVisualSummaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5NonVisualSummaryCatalogPacket {
    /// Builds a non-visual summary catalog packet from seed input.
    pub fn new(input: M5NonVisualSummaryCatalogPacketInput) -> Self {
        Self {
            record_kind: M5_NONVISUAL_SUMMARY_RECORD_KIND.to_owned(),
            schema_version: M5_NONVISUAL_SUMMARY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalog_label: input.catalog_label,
            summaries: input.summaries,
            shared_vocabulary_set: input.shared_vocabulary_set,
            summary_vocabulary_set: input.summary_vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Total number of drill-down routes across every surface summary.
    pub fn drilldown_count(&self) -> usize {
        self.summaries.iter().map(|s| s.drilldowns.len()).sum()
    }

    /// Validates the non-visual summary catalog invariants.
    pub fn validate(&self) -> Vec<M5NonVisualSummaryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_NONVISUAL_SUMMARY_RECORD_KIND {
            violations.push(M5NonVisualSummaryViolation::WrongRecordKind);
        }
        if self.schema_version != M5_NONVISUAL_SUMMARY_SCHEMA_VERSION {
            violations.push(M5NonVisualSummaryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalog_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5NonVisualSummaryViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_summaries(self, &mut violations);
        validate_presentation_state_coverage(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 non-visual summary catalog serializes"),
        ) {
            violations.push(M5NonVisualSummaryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 non-visual summary catalog serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable = self
            .summaries
            .iter()
            .filter(|s| s.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Non-Visual Custom-Surface Summaries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalog_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} stable), {} drill-down routes\n",
            self.summaries.len(),
            stable,
            self.drilldown_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surface summaries\n\n");
        for summary in &self.summaries {
            out.push_str(&format!(
                "- **{}** (`{}`): `{}`, fidelity `{}`, state `{}`\n",
                summary.summary_id,
                summary.surface_kind.as_str(),
                summary.qualification.as_str(),
                summary.non_visual_fidelity.as_str(),
                summary.presentation_state.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", summary.owner_role));
            out.push_str(&format!("  - Object identity: `{}`\n", summary.object_identity_ref));
            out.push_str(&format!(
                "  - Structure (`{}`, `{}`): {}\n",
                summary.structure.structure_message_id,
                summary.structure.role_class.as_str(),
                summary
                    .structure
                    .dimensions
                    .iter()
                    .map(|d| d.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Text alternative: `{}` (provided: {})\n",
                summary.text_alternative.kind.as_str(),
                summary.text_alternative.provided
            ));
            for drilldown in &summary.drilldowns {
                out.push_str(&format!(
                    "  - drill-down `{}` -> {} (`{}`) lands on `{}`\n",
                    drilldown.drilldown_id,
                    drilldown.kind.as_str(),
                    drilldown.route_message_id,
                    drilldown.target_identity_ref
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in non-visual summary export.
#[derive(Debug)]
pub enum M5NonVisualSummaryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5NonVisualSummaryViolation>),
}

impl fmt::Display for M5NonVisualSummaryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 non-visual summary export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 non-visual summary export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5NonVisualSummaryArtifactError {}

/// Validation failures emitted by [`M5NonVisualSummaryCatalogPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5NonVisualSummaryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A governed surface kind has no summary row.
    RequiredSurfaceKindMissing,
    /// Two rows cover the same surface kind.
    DuplicateSurfaceKind,
    /// Two rows share a summary id.
    DuplicateSummaryId,
    /// A summary row is incomplete.
    SummaryIncomplete,
    /// A summary is not bound to an object identity.
    MissingObjectIdentity,
    /// A summary's structure block is incomplete or unquantified.
    StructureIncomplete,
    /// A summary offers fewer than the required object-linked drill-down routes.
    SummaryNotActionable,
    /// Two drill-down routes share an id.
    DuplicateDrillDownId,
    /// A drill-down route is incomplete.
    DrillDownIncomplete,
    /// A drill-down route is not reachable by keyboard alone.
    DrillDownNotKeyboardReachable,
    /// A summary-owned message id is missing the governed prefix.
    MessageIdPrefixMissing,
    /// A surface's text alternative disagrees with whether it materially needs one.
    TextAlternativeInconsistent,
    /// A provisional presentation state is never exercised across the catalog.
    PresentationStateNotExercised,
    /// A summary's non-visual fidelity is not an accessible class.
    SummaryNonVisualFidelityInvalid,
    /// A summary claiming Stable is missing required proof packet refs.
    StableSummaryMissingProof,
    /// A summary has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A summary has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A summary has no reopenable durable fallback surface.
    DurableFallbackMissing,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5NonVisualSummaryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::DuplicateSurfaceKind => "duplicate_surface_kind",
            Self::DuplicateSummaryId => "duplicate_summary_id",
            Self::SummaryIncomplete => "summary_incomplete",
            Self::MissingObjectIdentity => "missing_object_identity",
            Self::StructureIncomplete => "structure_incomplete",
            Self::SummaryNotActionable => "summary_not_actionable",
            Self::DuplicateDrillDownId => "duplicate_drilldown_id",
            Self::DrillDownIncomplete => "drilldown_incomplete",
            Self::DrillDownNotKeyboardReachable => "drilldown_not_keyboard_reachable",
            Self::MessageIdPrefixMissing => "message_id_prefix_missing",
            Self::TextAlternativeInconsistent => "text_alternative_inconsistent",
            Self::PresentationStateNotExercised => "presentation_state_not_exercised",
            Self::SummaryNonVisualFidelityInvalid => "summary_non_visual_fidelity_invalid",
            Self::StableSummaryMissingProof => "stable_summary_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DurableFallbackMissing => "durable_fallback_missing",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable non-visual summary export.
pub fn current_stable_m5_nonvisual_summary_export(
) -> Result<M5NonVisualSummaryCatalogPacket, M5NonVisualSummaryArtifactError> {
    let packet: M5NonVisualSummaryCatalogPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-nonvisual-summary-proof/support_export.json"
    )))
    .map_err(M5NonVisualSummaryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5NonVisualSummaryArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_NONVISUAL_SUMMARY_SCHEMA_REF,
        M5_NONVISUAL_SUMMARY_DOC_REF,
        M5_NONVISUAL_SUMMARY_MATRIX_REF,
        M5_NONVISUAL_SUMMARY_ANNOUNCEMENT_GRAMMAR_REF,
        M5_NONVISUAL_SUMMARY_SURFACE_DESCRIPTOR_REF,
        M5_NONVISUAL_SUMMARY_SCREEN_READER_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5NonVisualSummaryViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    if !packet.shared_vocabulary_set.matches_canonical()
        || !packet.summary_vocabulary_set.matches_canonical()
    {
        violations.push(M5NonVisualSummaryViolation::VocabularySetDrift);
    }
}

fn validate_summaries(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let present: BTreeSet<M5SummarySurfaceKind> =
        packet.summaries.iter().map(|s| s.surface_kind).collect();
    for required in M5SummarySurfaceKind::ALL {
        if !present.contains(&required) {
            violations.push(M5NonVisualSummaryViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let mut seen_summary_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_surface_kinds: BTreeSet<M5SummarySurfaceKind> = BTreeSet::new();
    let mut seen_drilldown_ids: BTreeSet<&str> = BTreeSet::new();
    for summary in &packet.summaries {
        if !seen_summary_ids.insert(summary.summary_id.as_str()) {
            violations.push(M5NonVisualSummaryViolation::DuplicateSummaryId);
        }
        if !seen_surface_kinds.insert(summary.surface_kind) {
            violations.push(M5NonVisualSummaryViolation::DuplicateSurfaceKind);
        }

        if summary.summary_id.trim().is_empty()
            || summary.label.trim().is_empty()
            || summary.owner_role.trim().is_empty()
            || summary.producers.is_empty()
            || summary.source_contract_refs.is_empty()
        {
            violations.push(M5NonVisualSummaryViolation::SummaryIncomplete);
        }

        if summary.object_identity_ref.trim().is_empty() {
            violations.push(M5NonVisualSummaryViolation::MissingObjectIdentity);
        }

        validate_structure(&summary.structure, violations);
        validate_drilldowns(summary, &mut seen_drilldown_ids, violations);
        validate_text_alternative(summary, violations);

        if !is_accessible_fidelity(summary.non_visual_fidelity) {
            violations.push(M5NonVisualSummaryViolation::SummaryNonVisualFidelityInvalid);
        }
        if summary.qualification.is_stable() && summary.required_proof_packet_refs.is_empty() {
            violations.push(M5NonVisualSummaryViolation::StableSummaryMissingProof);
        }
        if summary.downgrade_triggers.is_empty() {
            violations.push(M5NonVisualSummaryViolation::DowngradeTriggersMissing);
        }
        if summary.consumer_surfaces.is_empty() {
            violations.push(M5NonVisualSummaryViolation::ConsumerSurfacesMissing);
        }
        if summary.durable_fallback.surface_ref.trim().is_empty()
            || !summary.durable_fallback.reopenable
        {
            violations.push(M5NonVisualSummaryViolation::DurableFallbackMissing);
        }
    }
}

fn validate_structure(
    structure: &M5SummaryStructure,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    // The structure must be a quantified description, not a vague one-liner: a stable
    // message id plus at least one named, quantified dimension.
    if structure.structure_message_id.trim().is_empty() || structure.dimensions.is_empty() {
        violations.push(M5NonVisualSummaryViolation::StructureIncomplete);
    }
    if !structure
        .structure_message_id
        .starts_with(M5_SUMMARY_MESSAGE_ID_PREFIX)
    {
        violations.push(M5NonVisualSummaryViolation::MessageIdPrefixMissing);
    }
    for dimension in &structure.dimensions {
        if dimension.name.trim().is_empty() || dimension.dimension_message_id.trim().is_empty() {
            violations.push(M5NonVisualSummaryViolation::StructureIncomplete);
        }
        if !dimension
            .dimension_message_id
            .starts_with(M5_SUMMARY_MESSAGE_ID_PREFIX)
        {
            violations.push(M5NonVisualSummaryViolation::MessageIdPrefixMissing);
        }
    }
}

fn validate_drilldowns<'a>(
    summary: &'a M5SurfaceSummary,
    seen_drilldown_ids: &mut BTreeSet<&'a str>,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    // Guardrail: detailed drill-down navigation may not collapse into a vague
    // one-liner, so every surface offers at least the minimum object-linked routes.
    if summary.drilldowns.len() < M5_SUMMARY_MIN_DRILLDOWNS {
        violations.push(M5NonVisualSummaryViolation::SummaryNotActionable);
    }
    for drilldown in &summary.drilldowns {
        if !seen_drilldown_ids.insert(drilldown.drilldown_id.as_str()) {
            violations.push(M5NonVisualSummaryViolation::DuplicateDrillDownId);
        }
        if drilldown.drilldown_id.trim().is_empty()
            || drilldown.label.trim().is_empty()
            || drilldown.route_message_id.trim().is_empty()
            || drilldown.target_identity_ref.trim().is_empty()
        {
            violations.push(M5NonVisualSummaryViolation::DrillDownIncomplete);
        }
        if !drilldown
            .route_message_id
            .starts_with(M5_SUMMARY_MESSAGE_ID_PREFIX)
        {
            violations.push(M5NonVisualSummaryViolation::MessageIdPrefixMissing);
        }
        if !drilldown.keyboard_reachable {
            violations.push(M5NonVisualSummaryViolation::DrillDownNotKeyboardReachable);
        }
    }
}

fn validate_text_alternative(
    summary: &M5SurfaceSummary,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let alt = &summary.text_alternative;
    if summary.surface_kind.requires_text_alternative() {
        // A surface whose visual state materially affects decisions must supply a real
        // text alternative plus an export-safe metadata view.
        let ok = alt.provided
            && alt.kind.is_applicable()
            && !alt.alt_text_message_id.trim().is_empty()
            && !alt.export_metadata_fields.is_empty();
        if !ok {
            violations.push(M5NonVisualSummaryViolation::TextAlternativeInconsistent);
        }
        if alt.provided && !alt.alt_text_message_id.starts_with(M5_SUMMARY_MESSAGE_ID_PREFIX) {
            violations.push(M5NonVisualSummaryViolation::MessageIdPrefixMissing);
        }
    } else {
        // A text-native surface declares no image/chart alternative so the catalog never
        // implies a missing one is a gap.
        let ok = !alt.provided
            && !alt.kind.is_applicable()
            && alt.alt_text_message_id.trim().is_empty()
            && alt.export_metadata_fields.is_empty();
        if !ok {
            violations.push(M5NonVisualSummaryViolation::TextAlternativeInconsistent);
        }
    }
}

/// Confirms every provisional presentation state stays representable in the non-visual
/// catalog, so preview/cached/generated/approximate/sampled/buffered truth is never
/// visible only in the visual chrome.
fn validate_presentation_state_coverage(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let present: BTreeSet<M5SummaryPresentationState> =
        packet.summaries.iter().map(|s| s.presentation_state).collect();
    for state in M5SummaryPresentationState::ALL {
        if state.is_provisional() && !present.contains(&state) {
            violations.push(M5NonVisualSummaryViolation::PresentationStateNotExercised);
            return;
        }
    }
}

/// True when a fidelity class still conveys non-visual truth for a covered summary.
fn is_accessible_fidelity(fidelity: A11yNonVisualFidelity) -> bool {
    matches!(
        fidelity,
        A11yNonVisualFidelity::FullAccessible
            | A11yNonVisualFidelity::DegradedAccessible
            | A11yNonVisualFidelity::SummaryOnly
    )
}

fn validate_conformance_review(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.surfaces_explain_structure_without_vision_or_hover,
        review.drilldowns_remain_actionable_not_vague_one_liners,
        review.summaries_linked_to_same_object_identity_as_visual,
        review.provisional_states_visible_in_non_visual_representation,
        review.visual_artifacts_provide_text_alternative_and_metadata_view,
        review.chart_and_artifact_viewers_no_longer_require_visual_interpretation_alone,
        review.no_pixel_only_or_pointer_only_truth_source,
        review.claimed_surfaces_auto_narrow_when_bridge_or_proof_stale,
        review.downgrade_narrows_instead_of_hides,
    ] {
        if !ok {
            violations.push(M5NonVisualSummaryViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_consumes_summaries,
        projection.terminal_consumes_summaries,
        projection.data_grid_and_tree_consume_summaries,
        projection.observability_logs_traces_charts_consume_summaries,
        projection.review_and_artifact_viewers_consume_summaries,
        projection.docs_help_reuse_summaries,
        projection.support_export_reuses_summaries,
        projection.at_conformance_packets_reuse_summaries,
    ] {
        if !ok {
            violations.push(M5NonVisualSummaryViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5NonVisualSummaryViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5NonVisualSummaryCatalogPacket,
    violations: &mut Vec<M5NonVisualSummaryViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5NonVisualSummaryViolation::ReleasePostureIncomplete);
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
