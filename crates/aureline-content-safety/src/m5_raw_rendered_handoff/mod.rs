//! Raw-versus-rendered representation honesty and handoff preservation across
//! the new M5 review, docs, AI, and structured viewer surfaces.
//!
//! The prior lanes own byte-level suspicious-content findings
//! ([`crate::detector`]) and project one threat-class vocabulary across the new
//! M5 panes ([`crate::m5_suspicious_text_detector_parity`]). This module covers
//! the orthogonal honesty gap those lanes leave open: a viewer that *renders*
//! content — Markdown to HTML, ANSI cell output, an AI summary, a normalized
//! diff, a pretty-printed artifact, a rendered manifest, or a rendered policy —
//! produces a display form that differs *materially* from the canonical raw
//! bytes. Without explicit labels and copy/export semantics, that rendered text
//! or summarized output can masquerade as the raw source during copy, export,
//! or support handoff.
//!
//! This packet binds three things together so no claimed M5 viewer can let
//! rendered output pass for canonical bytes:
//!
//! - **Explicit Raw vs Rendered labels.** Every surface whose render transform
//!   materially diverges from the raw bytes exposes a distinct `Raw` label
//!   (canonical bytes) and a `Rendered` label (the viewer's display form), so a
//!   reader can always tell which representation they are looking at.
//! - **Labeled copy/export, raw stays reachable.** Diverging surfaces offer
//!   `Copy raw`, `Copy rendered`, and an `Export safe` action; the raw copy is
//!   the only one that preserves canonical bytes, and neither rendered copy nor
//!   export ever claims to be byte-identical raw. Bytes are never normalized
//!   away.
//! - **Handoff preserves the warning.** A
//!   [`M5RepresentationHandoffPreservation`] block carries the raw/rendered
//!   labels, the per-surface render transform, and a divergence note across the
//!   support-export, screenshot-caption, and handoff-packet carriers, so a
//!   downstream reader can tell what the original surface warned about. The
//!   carriers carry escaped exemplars only — never raw suspicious bytes.
//!
//! Strong-decision surfaces (install/update review, policy review) render with
//! stricter identity than ordinary browsing panes.
//!
//! The boundary schema is
//! [`schemas/security/m5-raw-rendered-handoff.schema.json`](../../../../schemas/security/m5-raw-rendered-handoff.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_raw_rendered_handoff.md`](../../../../docs/security/m5/m5_raw_rendered_handoff.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{has_suspicious_content, BodyPosture, RepresentationActionId, RepresentationClass};

/// Stable record-kind tag carried by [`M5RawRenderedHandoffPacket`].
pub const M5_RAW_RENDERED_HANDOFF_RECORD_KIND: &str = "m5_raw_rendered_handoff_packet";

/// Stable record-kind tag carried by [`M5RepresentationHandoffPreservation`].
pub const M5_RAW_RENDERED_HANDOFF_PRESERVATION_RECORD_KIND: &str =
    "m5_representation_handoff_preservation";

/// Schema version for the M5 raw-versus-rendered handoff packet.
pub const M5_RAW_RENDERED_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_RAW_RENDERED_HANDOFF_SCHEMA_REF: &str =
    "schemas/security/m5-raw-rendered-handoff.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RAW_RENDERED_HANDOFF_DOC_REF: &str = "docs/security/m5/m5_raw_rendered_handoff.md";

/// Repo-relative path of the checked-in support-export artifact.
pub const M5_RAW_RENDERED_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_raw_rendered_handoff/support_export.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RAW_RENDERED_HANDOFF_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5_raw_rendered_handoff";

/// Stable packet id minted by [`frozen_m5_raw_rendered_handoff_packet`].
pub const M5_RAW_RENDERED_HANDOFF_PACKET_ID: &str = "m5-raw-rendered-handoff:stable:0001";

/// New M5 viewer surface where a render transform can make the rendered form
/// differ materially from the canonical raw bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RawRenderedSurface {
    /// Docs page or in-product browser panel rendered from Markdown/HTML.
    DocsRenderedPanel,
    /// Notebook rich-output cell rendered from raw output bytes.
    NotebookRenderedOutput,
    /// AI summary or finding card derived from longer raw evidence.
    AiSummaryEvidence,
    /// Structured review diff/compare view over normalized source.
    ReviewStructuredDiff,
    /// Generated/structured artifact viewer (pretty-printed JSON, tables).
    StructuredArtifactViewer,
    /// Marketplace install/update review (strong decision).
    MarketplaceInstallReview,
    /// Provider/policy review overlay (strong decision).
    PolicyReviewOverlay,
}

impl M5RawRenderedSurface {
    /// Every surface this lane must cover, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DocsRenderedPanel,
        Self::NotebookRenderedOutput,
        Self::AiSummaryEvidence,
        Self::ReviewStructuredDiff,
        Self::StructuredArtifactViewer,
        Self::MarketplaceInstallReview,
        Self::PolicyReviewOverlay,
    ];

    /// Stable token recorded in packets, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsRenderedPanel => "docs_rendered_panel",
            Self::NotebookRenderedOutput => "notebook_rendered_output",
            Self::AiSummaryEvidence => "ai_summary_evidence",
            Self::ReviewStructuredDiff => "review_structured_diff",
            Self::StructuredArtifactViewer => "structured_artifact_viewer",
            Self::MarketplaceInstallReview => "marketplace_install_review",
            Self::PolicyReviewOverlay => "policy_review_overlay",
        }
    }

    /// Human-readable noun used in copy/export action labels.
    pub const fn label_noun(self) -> &'static str {
        match self {
            Self::DocsRenderedPanel => "page",
            Self::NotebookRenderedOutput => "cell output",
            Self::AiSummaryEvidence => "summary",
            Self::ReviewStructuredDiff => "diff",
            Self::StructuredArtifactViewer => "artifact",
            Self::MarketplaceInstallReview => "listing",
            Self::PolicyReviewOverlay => "policy",
        }
    }

    /// Whether this is a strong-decision surface (install/update review or
    /// policy review) that must render identity more strictly than an ordinary
    /// browsing pane.
    pub const fn is_strong_decision_surface(self) -> bool {
        matches!(
            self,
            Self::MarketplaceInstallReview | Self::PolicyReviewOverlay
        )
    }

    /// Display mode this surface must use.
    pub const fn display_mode(self) -> M5RawRenderedDisplayMode {
        if self.is_strong_decision_surface() {
            M5RawRenderedDisplayMode::StrongDecisionStrictIdentity
        } else {
            M5RawRenderedDisplayMode::OrdinaryBrowsing
        }
    }
}

/// Decision-strictness display mode for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RawRenderedDisplayMode {
    /// Ordinary browsing-pane identity rendering.
    OrdinaryBrowsing,
    /// Stricter owner/origin identity rendering for strong-decision surfaces.
    StrongDecisionStrictIdentity,
}

impl M5RawRenderedDisplayMode {
    /// Stable token recorded in packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryBrowsing => "ordinary_browsing",
            Self::StrongDecisionStrictIdentity => "strong_decision_strict_identity",
        }
    }

    /// Whether this is the stricter strong-decision display mode.
    pub const fn is_strict(self) -> bool {
        matches!(self, Self::StrongDecisionStrictIdentity)
    }
}

/// Render transform a surface applies to go from raw bytes to displayed form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RenderTransform {
    /// Markdown/HTML rendered to a styled, reflowed page.
    MarkdownHtmlRender,
    /// Notebook output bytes rendered as styled rich output.
    NotebookOutputRender,
    /// Longer raw evidence summarized into a short AI summary.
    AiSummarization,
    /// Source normalized (whitespace, EOL) for structured comparison.
    DiffNormalization,
    /// Structured bytes re-laid-out as pretty-printed output.
    StructuredPrettyPrint,
    /// Manifest fields rendered into a listing presentation.
    ManifestRender,
    /// Policy text rendered into a review presentation.
    PolicyRender,
    /// No transform — the rendered form is byte-identical to the raw bytes.
    NoTransform,
}

impl M5RenderTransform {
    /// Stable token recorded in packets, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarkdownHtmlRender => "markdown_html_render",
            Self::NotebookOutputRender => "notebook_output_render",
            Self::AiSummarization => "ai_summarization",
            Self::DiffNormalization => "diff_normalization",
            Self::StructuredPrettyPrint => "structured_pretty_print",
            Self::ManifestRender => "manifest_render",
            Self::PolicyRender => "policy_render",
            Self::NoTransform => "no_transform",
        }
    }

    /// Representation divergence this transform introduces.
    pub const fn divergence(self) -> M5RepresentationDivergence {
        match self {
            Self::MarkdownHtmlRender
            | Self::StructuredPrettyPrint
            | Self::ManifestRender
            | Self::PolicyRender => M5RepresentationDivergence::RenderedReflowsLayout,
            Self::NotebookOutputRender => M5RepresentationDivergence::RenderedAppliesStyling,
            Self::AiSummarization => M5RepresentationDivergence::RenderedSummarizesContent,
            Self::DiffNormalization => M5RepresentationDivergence::RenderedNormalizesForComparison,
            Self::NoTransform => M5RepresentationDivergence::ByteIdentical,
        }
    }
}

/// How a surface's rendered form differs from its canonical raw bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepresentationDivergence {
    /// Layout/order reflowed; glyphs may not match raw source order.
    RenderedReflowsLayout,
    /// Styling reinterprets raw control bytes (e.g. ANSI to rich text).
    RenderedAppliesStyling,
    /// Content summarized; rendered form omits raw bytes.
    RenderedSummarizesContent,
    /// Source normalized for comparison; rendered form drops raw bytes.
    RenderedNormalizesForComparison,
    /// Rendered form is byte-identical to the raw bytes.
    ByteIdentical,
}

impl M5RepresentationDivergence {
    /// Stable token recorded in packets, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderedReflowsLayout => "rendered_reflows_layout",
            Self::RenderedAppliesStyling => "rendered_applies_styling",
            Self::RenderedSummarizesContent => "rendered_summarizes_content",
            Self::RenderedNormalizesForComparison => "rendered_normalizes_for_comparison",
            Self::ByteIdentical => "byte_identical",
        }
    }

    /// Short user-facing note describing the divergence.
    pub const fn note(self) -> &'static str {
        match self {
            Self::RenderedReflowsLayout => {
                "Rendered layout reflows the raw source; displayed order may differ from the bytes."
            }
            Self::RenderedAppliesStyling => {
                "Rendered styling reinterprets raw control bytes into display formatting."
            }
            Self::RenderedSummarizesContent => {
                "Rendered summary omits raw content; it is not the full source."
            }
            Self::RenderedNormalizesForComparison => {
                "Rendered comparison normalizes whitespace and line endings away from the raw bytes."
            }
            Self::ByteIdentical => "Rendered form is byte-identical to the raw bytes.",
        }
    }

    /// Whether the rendered form differs materially from the raw bytes.
    pub const fn is_material(self) -> bool {
        !matches!(self, Self::ByteIdentical)
    }
}

/// One representation label exposed by a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepresentationLabelView {
    /// Representation class token (`raw` or `rendered`).
    pub representation_class: String,
    /// Short user-facing label.
    pub label: String,
    /// Longer description telling the reader what this representation is.
    pub description: String,
    /// Whether this label denotes the canonical raw bytes.
    pub is_canonical_bytes: bool,
}

/// Copy/export action offered by one surface with an explicit representation
/// label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RawRenderedTransferChoice {
    /// Transfer action id drawn from the representation policy vocabulary.
    pub action_id: String,
    /// Representation class drawn from the representation policy vocabulary.
    pub representation_class: String,
    /// Body posture drawn from the representation policy vocabulary.
    pub body_posture: String,
    /// Surface label that names the representation explicitly.
    pub label: String,
    /// True only for the raw-copy action that yields canonical bytes.
    pub preserves_canonical_bytes: bool,
    /// True only when the action's payload is byte-identical raw content.
    ///
    /// Rendered copy and export actions keep this `false` so rendered output
    /// never masquerades as the canonical raw bytes.
    pub implies_byte_identical_raw: bool,
    /// True when the action is an export-safe (sanitized/metadata) path.
    pub export_safe: bool,
}

/// Per-surface projection of the raw-versus-rendered representation honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RawRenderedSurfaceProjection {
    /// Surface this projection describes.
    pub surface: M5RawRenderedSurface,
    /// Stable surface token for compact consumers.
    pub surface_token: String,
    /// Opaque surface subject ref used by reopen and inspection paths.
    pub subject_ref: String,
    /// Display mode this surface must render in.
    pub display_mode: M5RawRenderedDisplayMode,
    /// Render transform applied to go from raw to rendered.
    pub render_transform: M5RenderTransform,
    /// Stable render-transform token.
    pub render_transform_token: String,
    /// Representation divergence introduced by the transform.
    pub divergence: M5RepresentationDivergence,
    /// Stable divergence token.
    pub divergence_token: String,
    /// Whether rendered form differs materially from the raw bytes.
    pub materially_diverges: bool,
    /// Representation labels the surface exposes.
    pub representation_labels: Vec<M5RepresentationLabelView>,
    /// Copy/export actions with explicit representation labels.
    pub copy_export_actions: Vec<M5RawRenderedTransferChoice>,
    /// Whether a raw-copy path that yields canonical bytes stays reachable.
    pub raw_copy_reachable: bool,
    /// Length of the raw bytes the surface presents.
    pub raw_len_bytes: usize,
    /// Length of the rendered form the surface presents.
    pub rendered_len_bytes: usize,
}

impl M5RawRenderedSurfaceProjection {
    /// True when the surface exposes both a canonical-raw label and a rendered
    /// label.
    pub fn exposes_raw_and_rendered_labels(&self) -> bool {
        let has_raw = self.representation_labels.iter().any(|label| {
            label.representation_class == RepresentationClass::Raw.as_str()
                && label.is_canonical_bytes
        });
        let has_rendered = self
            .representation_labels
            .iter()
            .any(|label| label.representation_class == RepresentationClass::Rendered.as_str());
        has_raw && has_rendered
    }

    /// True when the surface offers labeled raw copy, rendered copy, and an
    /// export-safe action.
    pub fn offers_raw_rendered_and_export_actions(&self) -> bool {
        let has_raw = self.copy_export_actions.iter().any(|action| {
            action.action_id == RepresentationActionId::CopyRaw.as_str()
                && action.preserves_canonical_bytes
        });
        let has_rendered = self
            .copy_export_actions
            .iter()
            .any(|action| action.action_id == RepresentationActionId::CopyRendered.as_str());
        let has_export = self
            .copy_export_actions
            .iter()
            .any(|action| action.export_safe);
        has_raw && has_rendered && has_export
    }

    /// True when no rendered or export action claims to be byte-identical raw.
    pub fn rendered_copy_never_implies_raw(&self) -> bool {
        self.copy_export_actions.iter().all(|action| {
            action.representation_class == RepresentationClass::Raw.as_str()
                || !action.implies_byte_identical_raw
        })
    }
}

/// Carrier a representation warning is preserved across at handoff time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffCarrier {
    /// Support/admin export bundle.
    SupportExport,
    /// Screenshot caption attached to a captured image.
    ScreenshotCaption,
    /// Structured handoff packet passed between surfaces.
    HandoffPacket,
}

impl M5HandoffCarrier {
    /// Every handoff carrier, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SupportExport,
        Self::ScreenshotCaption,
        Self::HandoffPacket,
    ];

    /// Stable token recorded in packets, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportExport => "support_export",
            Self::ScreenshotCaption => "screenshot_caption",
            Self::HandoffPacket => "handoff_packet",
        }
    }

    /// Whether this carrier must preserve the canonical-raw label.
    ///
    /// A screenshot can only show the rendered view, so it preserves a rendered
    /// label plus a disclaimer instead of the raw bytes; export and handoff
    /// carriers preserve both labels.
    pub const fn requires_raw_label(self) -> bool {
        matches!(self, Self::SupportExport | Self::HandoffPacket)
    }
}

/// One handoff carrier's preservation of the representation warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HandoffCarrierPreservation {
    /// Carrier this preservation describes.
    pub carrier: M5HandoffCarrier,
    /// Stable carrier token.
    pub carrier_token: String,
    /// How the carrier captured the representation.
    pub captured_representation: String,
    /// Representation-class labels preserved by the carrier.
    pub preserved_labels: Vec<String>,
    /// Render-transform tokens preserved by the carrier.
    pub preserved_render_transforms: Vec<String>,
    /// Whether the carrier preserves a divergence note.
    pub preserves_divergence_note: bool,
    /// Divergence note preserved for the downstream reader.
    pub divergence_note: String,
    /// Whether the carrier explicitly declares that rendered output is not raw.
    pub declares_rendered_not_raw: bool,
    /// Escaped exemplar of a rendered form (never raw suspicious bytes).
    pub escaped_exemplar: String,
    /// Surface tokens covered by this carrier.
    pub surfaces_covered: Vec<String>,
}

/// Handoff preservation block carried by the packet.
///
/// Preserves the raw/rendered labels and render transforms across the
/// support-export, screenshot-caption, and handoff-packet carriers so a
/// downstream reader can tell what the original surface warned about. Carries
/// escaped exemplars only; raw suspicious bytes never cross this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RepresentationHandoffPreservation {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Case id shared with the packet.
    pub case_id: String,
    /// Per-carrier preservation records.
    pub carriers: Vec<M5HandoffCarrierPreservation>,
    /// Whether the block preserves a divergence warning at all.
    pub preserves_divergence_warning: bool,
    /// Redaction class token.
    pub redaction_class_token: String,
}

/// One surface's raw-versus-rendered input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5RawRenderedSurfaceInput<'a> {
    /// Surface this input describes.
    pub surface: M5RawRenderedSurface,
    /// Opaque surface subject ref.
    pub subject_ref: &'a str,
    /// Render transform the surface applies.
    pub render_transform: M5RenderTransform,
    /// Raw source bytes the surface presents.
    pub raw_sample: &'a str,
    /// Rendered form the surface presents.
    pub rendered_sample: &'a str,
}

/// Inputs needed to project the M5 raw-versus-rendered handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5RawRenderedHandoffSeed<'a> {
    /// Stable case id shared by all surface projections.
    pub case_id: &'a str,
    /// Per-surface raw/rendered inputs, one per [`M5RawRenderedSurface::ALL`].
    pub surface_inputs: [M5RawRenderedSurfaceInput<'a>; 7],
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: &'a str,
}

/// Cross-surface raw-versus-rendered handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RawRenderedHandoffPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Number of surfaces whose rendered form materially diverges from raw.
    pub diverging_surface_count: usize,
    /// Distinct divergence tokens across the diverging surfaces.
    pub divergence_kinds: Vec<String>,
    /// Distinct render-transform tokens across surfaces.
    pub render_transforms: Vec<String>,
    /// Whether projection normalized or stripped any source (always false).
    pub normalization_applied: bool,
    /// Surface projections for every M5 surface.
    pub surfaces: Vec<M5RawRenderedSurfaceProjection>,
    /// Handoff preservation block.
    pub handoff_preservation: M5RepresentationHandoffPreservation,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RawRenderedHandoffPacket {
    /// Returns true when every required M5 surface is present exactly once.
    pub fn covers_all_m5_surfaces(&self) -> bool {
        let present: BTreeSet<_> = self
            .surfaces
            .iter()
            .map(|surface| surface.surface)
            .collect();
        M5RawRenderedSurface::ALL
            .iter()
            .all(|surface| present.contains(surface))
            && present.len() == M5RawRenderedSurface::ALL.len()
    }

    /// Returns true when every materially-diverging surface exposes both raw
    /// and rendered labels and labeled raw/rendered/export actions.
    pub fn diverging_surfaces_expose_raw_and_rendered(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            !surface.materially_diverges
                || (surface.exposes_raw_and_rendered_labels()
                    && surface.offers_raw_rendered_and_export_actions())
        })
    }

    /// Returns true when no surface lets rendered copy/export imply raw bytes.
    pub fn rendered_copy_never_implies_raw(&self) -> bool {
        self.surfaces
            .iter()
            .all(M5RawRenderedSurfaceProjection::rendered_copy_never_implies_raw)
    }

    /// Returns true when every strong-decision surface uses strict display.
    pub fn strong_decision_surfaces_use_strict_display(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            !surface.surface.is_strong_decision_surface() || surface.display_mode.is_strict()
        })
    }

    /// Returns true when the handoff block preserves the divergence warning
    /// across every required carrier.
    pub fn handoff_preserves_representation(&self) -> bool {
        let block = &self.handoff_preservation;
        let has_divergence = self.diverging_surface_count > 0;
        if block.preserves_divergence_warning != has_divergence {
            return false;
        }
        let present: BTreeSet<_> = block
            .carriers
            .iter()
            .map(|carrier| carrier.carrier)
            .collect();
        if !M5HandoffCarrier::ALL
            .iter()
            .all(|carrier| present.contains(carrier))
        {
            return false;
        }
        block.carriers.iter().all(|carrier| {
            if !has_divergence {
                return true;
            }
            let preserves_rendered_label = carrier
                .preserved_labels
                .iter()
                .any(|label| label == RepresentationClass::Rendered.as_str());
            let preserves_raw_label = !carrier.carrier.requires_raw_label()
                || carrier
                    .preserved_labels
                    .iter()
                    .any(|label| label == RepresentationClass::Raw.as_str());
            carrier.preserves_divergence_note
                && carrier.declares_rendered_not_raw
                && preserves_rendered_label
                && preserves_raw_label
                && !carrier.surfaces_covered.is_empty()
        })
    }

    /// Validates the raw-versus-rendered handoff invariants.
    pub fn validate(&self) -> Vec<M5RawRenderedHandoffViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RAW_RENDERED_HANDOFF_RECORD_KIND {
            violations.push(M5RawRenderedHandoffViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RAW_RENDERED_HANDOFF_SCHEMA_VERSION {
            violations.push(M5RawRenderedHandoffViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.case_id.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(M5RawRenderedHandoffViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(M5RawRenderedHandoffViolation::MissingSourceContracts);
        }
        if self.normalization_applied {
            violations.push(M5RawRenderedHandoffViolation::NormalizationApplied);
        }
        if !self.covers_all_m5_surfaces() {
            violations.push(M5RawRenderedHandoffViolation::SurfaceMissing);
        }
        for surface in &self.surfaces {
            if surface.materially_diverges && !surface.exposes_raw_and_rendered_labels() {
                violations
                    .push(M5RawRenderedHandoffViolation::DivergingSurfaceMissingRawRenderedLabels);
                break;
            }
        }
        for surface in &self.surfaces {
            if surface.materially_diverges && !surface.offers_raw_rendered_and_export_actions() {
                violations
                    .push(M5RawRenderedHandoffViolation::DivergingSurfaceMissingCopyExportActions);
                break;
            }
        }
        if !self.rendered_copy_never_implies_raw() {
            violations.push(M5RawRenderedHandoffViolation::RenderedCopyImpliesRaw);
        }
        if !self.strong_decision_surfaces_use_strict_display() {
            violations.push(M5RawRenderedHandoffViolation::StrongDecisionDisplayTooWeak);
        }
        if self.diverging_surface_count != self.declared_diverging_count() {
            violations.push(M5RawRenderedHandoffViolation::DivergingCountMismatch);
        }

        validate_handoff_preservation(self, &mut violations);

        violations
    }

    fn declared_diverging_count(&self) -> usize {
        self.surfaces
            .iter()
            .filter(|surface| surface.materially_diverges)
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 raw-rendered handoff packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Raw-versus-Rendered Representation & Handoff\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Case: `{}`\n", self.case_id));
        out.push_str(&format!(
            "- Diverging surfaces: {}\n",
            self.diverging_surface_count
        ));
        out.push_str(&format!(
            "- Divergence kinds: {}\n",
            self.divergence_kinds.join(", ")
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** ({}): transform `{}`, divergence `{}`, raw copy reachable: {}\n",
                surface.surface.as_str(),
                surface.display_mode.as_str(),
                surface.render_transform_token,
                surface.divergence_token,
                surface.raw_copy_reachable
            ));
        }
        out.push_str("\n## Handoff carriers\n\n");
        for carrier in &self.handoff_preservation.carriers {
            out.push_str(&format!(
                "- `{}`: declares rendered != raw: {}, preserves note: {}\n",
                carrier.carrier_token,
                carrier.declares_rendered_not_raw,
                carrier.preserves_divergence_note
            ));
        }
        out
    }
}

/// Validation failures emitted by [`M5RawRenderedHandoffPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RawRenderedHandoffViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Projection normalized or stripped the source bytes.
    NormalizationApplied,
    /// A required M5 surface is missing.
    SurfaceMissing,
    /// A diverging surface does not expose both Raw and Rendered labels.
    DivergingSurfaceMissingRawRenderedLabels,
    /// A diverging surface does not offer labeled raw/rendered/export actions.
    DivergingSurfaceMissingCopyExportActions,
    /// A rendered copy/export action claims to be byte-identical raw.
    RenderedCopyImpliesRaw,
    /// A strong-decision surface does not use strict display mode.
    StrongDecisionDisplayTooWeak,
    /// The declared diverging-surface count does not match the projections.
    DivergingCountMismatch,
    /// The handoff block drops the divergence warning on some carrier.
    HandoffDropsDivergenceWarning,
    /// A required handoff carrier is missing.
    HandoffCarrierMissing,
    /// A handoff carrier leaks raw suspicious bytes in its exemplar.
    HandoffLeaksRawBytes,
}

impl M5RawRenderedHandoffViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NormalizationApplied => "normalization_applied",
            Self::SurfaceMissing => "surface_missing",
            Self::DivergingSurfaceMissingRawRenderedLabels => {
                "diverging_surface_missing_raw_rendered_labels"
            }
            Self::DivergingSurfaceMissingCopyExportActions => {
                "diverging_surface_missing_copy_export_actions"
            }
            Self::RenderedCopyImpliesRaw => "rendered_copy_implies_raw",
            Self::StrongDecisionDisplayTooWeak => "strong_decision_display_too_weak",
            Self::DivergingCountMismatch => "diverging_count_mismatch",
            Self::HandoffDropsDivergenceWarning => "handoff_drops_divergence_warning",
            Self::HandoffCarrierMissing => "handoff_carrier_missing",
            Self::HandoffLeaksRawBytes => "handoff_leaks_raw_bytes",
        }
    }
}

/// Projects the raw-versus-rendered honesty across every new M5 surface.
pub fn project_m5_raw_rendered_handoff(
    seed: &M5RawRenderedHandoffSeed<'_>,
) -> M5RawRenderedHandoffPacket {
    let surfaces: Vec<_> = seed.surface_inputs.iter().map(project_surface).collect();

    let divergence_kinds = distinct_tokens(
        surfaces
            .iter()
            .filter(|surface| surface.materially_diverges)
            .map(|surface| surface.divergence.as_str()),
    );
    let render_transforms = distinct_tokens(
        surfaces
            .iter()
            .map(|surface| surface.render_transform.as_str()),
    );
    let diverging_surface_count = surfaces
        .iter()
        .filter(|surface| surface.materially_diverges)
        .count();

    let handoff_preservation = build_handoff_preservation(seed.case_id, &surfaces);

    M5RawRenderedHandoffPacket {
        record_kind: M5_RAW_RENDERED_HANDOFF_RECORD_KIND.to_owned(),
        schema_version: M5_RAW_RENDERED_HANDOFF_SCHEMA_VERSION,
        packet_id: M5_RAW_RENDERED_HANDOFF_PACKET_ID.to_owned(),
        case_id: seed.case_id.to_owned(),
        diverging_surface_count,
        divergence_kinds,
        render_transforms,
        normalization_applied: false,
        surfaces,
        handoff_preservation,
        source_contract_refs: vec![
            M5_RAW_RENDERED_HANDOFF_SCHEMA_REF.to_owned(),
            M5_RAW_RENDERED_HANDOFF_DOC_REF.to_owned(),
            "schemas/security/trust_class.schema.json".to_owned(),
            "schemas/security/text_representation_policy.schema.json".to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: seed.minted_at.to_owned(),
    }
}

/// Builds the canonical frozen stable M5 raw-versus-rendered handoff packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_RAW_RENDERED_HANDOFF_ARTIFACT_REF`]; the bin emits this packet and a
/// test asserts the checked-in artifact deserializes back to it unchanged.
pub fn frozen_m5_raw_rendered_handoff_packet() -> M5RawRenderedHandoffPacket {
    project_m5_raw_rendered_handoff(&M5RawRenderedHandoffSeed {
        case_id: "case:m5-raw-rendered-handoff:stable",
        surface_inputs: [
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::DocsRenderedPanel,
                subject_ref: "docs:page:guide#install",
                render_transform: M5RenderTransform::MarkdownHtmlRender,
                raw_sample: "# Install\n\nRun `aureline init`.",
                rendered_sample: "Install\nRun aureline init",
            },
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::NotebookRenderedOutput,
                subject_ref: "notebook:cell:demo:out:2",
                render_transform: M5RenderTransform::NotebookOutputRender,
                raw_sample: "\u{1b}[31mError\u{1b}[0m: build failed",
                rendered_sample: "Error: build failed",
            },
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::AiSummaryEvidence,
                subject_ref: "ai:evidence:review:finding:7",
                render_transform: M5RenderTransform::AiSummarization,
                raw_sample: "The handler leaks a file handle on the error path at line 42 and again at line 58, and the retry loop never closes the socket.",
                rendered_sample: "Summary: possible resource leak on error paths.",
            },
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::ReviewStructuredDiff,
                subject_ref: "review:diff:pr:128:file:3",
                render_transform: M5RenderTransform::DiffNormalization,
                raw_sample: "let x =1;\r\n",
                rendered_sample: "let x = 1;",
            },
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::StructuredArtifactViewer,
                subject_ref: "artifact:json:report:9",
                render_transform: M5RenderTransform::StructuredPrettyPrint,
                raw_sample: "{\"k\":1,\"n\":\"a\"}",
                rendered_sample: "{\n  \"k\": 1,\n  \"n\": \"a\"\n}",
            },
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::MarketplaceInstallReview,
                subject_ref: "marketplace:listing:demo@2.0.0",
                render_transform: M5RenderTransform::ManifestRender,
                raw_sample: "name = \"demo\"\nversion = \"2.0.0\"",
                rendered_sample: "demo \u{2014} v2.0.0",
            },
            M5RawRenderedSurfaceInput {
                surface: M5RawRenderedSurface::PolicyReviewOverlay,
                subject_ref: "provider:policy:overlay:org",
                render_transform: M5RenderTransform::PolicyRender,
                raw_sample: "# Data Use\n- Reads repo files",
                rendered_sample: "Data Use\nReads repo files",
            },
        ],
        minted_at: "2026-06-10T00:00:00Z",
    })
}

/// Reads and validates the checked-in stable M5 raw-versus-rendered export.
pub fn current_m5_raw_rendered_handoff_export(
) -> Result<M5RawRenderedHandoffPacket, M5RawRenderedHandoffExportError> {
    let packet: M5RawRenderedHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/m5_raw_rendered_handoff/support_export.json"
    )))
    .map_err(M5RawRenderedHandoffExportError::Parse)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RawRenderedHandoffExportError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in M5 raw-versus-rendered export.
#[derive(Debug)]
pub enum M5RawRenderedHandoffExportError {
    /// Support export failed to parse.
    Parse(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RawRenderedHandoffViolation>),
}

impl std::fmt::Display for M5RawRenderedHandoffExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(
                    formatter,
                    "m5 raw-rendered handoff export parse failed: {error}"
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
                    "m5 raw-rendered handoff export failed validation: {tokens}"
                )
            }
        }
    }
}

impl std::error::Error for M5RawRenderedHandoffExportError {}

fn distinct_tokens<'a, I>(tokens: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a str>,
{
    tokens
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn project_surface(input: &M5RawRenderedSurfaceInput<'_>) -> M5RawRenderedSurfaceProjection {
    let surface = input.surface;
    let divergence = input.render_transform.divergence();
    let materially_diverges = divergence.is_material();

    let representation_labels = if materially_diverges {
        vec![raw_label(), rendered_label()]
    } else {
        vec![byte_identical_label()]
    };

    let copy_export_actions = if materially_diverges {
        vec![
            copy_raw_action(surface),
            copy_rendered_action(surface),
            export_safe_action(surface),
        ]
    } else {
        vec![copy_raw_action(surface)]
    };

    let raw_copy_reachable = copy_export_actions.iter().any(|action| {
        action.action_id == RepresentationActionId::CopyRaw.as_str()
            && action.preserves_canonical_bytes
    });

    M5RawRenderedSurfaceProjection {
        surface,
        surface_token: surface.as_str().to_owned(),
        subject_ref: input.subject_ref.to_owned(),
        display_mode: surface.display_mode(),
        render_transform: input.render_transform,
        render_transform_token: input.render_transform.as_str().to_owned(),
        divergence,
        divergence_token: divergence.as_str().to_owned(),
        materially_diverges,
        representation_labels,
        copy_export_actions,
        raw_copy_reachable,
        raw_len_bytes: input.raw_sample.len(),
        rendered_len_bytes: input.rendered_sample.len(),
    }
}

fn raw_label() -> M5RepresentationLabelView {
    M5RepresentationLabelView {
        representation_class: RepresentationClass::Raw.as_str().to_owned(),
        label: "Raw bytes".to_owned(),
        description: "Exact source bytes as stored; the canonical representation.".to_owned(),
        is_canonical_bytes: true,
    }
}

fn rendered_label() -> M5RepresentationLabelView {
    M5RepresentationLabelView {
        representation_class: RepresentationClass::Rendered.as_str().to_owned(),
        label: "Rendered view".to_owned(),
        description: "Display form produced by the viewer; not the canonical raw bytes.".to_owned(),
        is_canonical_bytes: false,
    }
}

fn byte_identical_label() -> M5RepresentationLabelView {
    M5RepresentationLabelView {
        representation_class: RepresentationClass::Raw.as_str().to_owned(),
        label: "Raw bytes".to_owned(),
        description: "Rendered form is byte-identical to the raw bytes.".to_owned(),
        is_canonical_bytes: true,
    }
}

fn copy_raw_action(surface: M5RawRenderedSurface) -> M5RawRenderedTransferChoice {
    M5RawRenderedTransferChoice {
        action_id: RepresentationActionId::CopyRaw.as_str().to_owned(),
        representation_class: RepresentationClass::Raw.as_str().to_owned(),
        body_posture: BodyPosture::ExactSourceBytes.as_str().to_owned(),
        label: format!("Copy raw {}", surface.label_noun()),
        preserves_canonical_bytes: true,
        implies_byte_identical_raw: true,
        export_safe: false,
    }
}

fn copy_rendered_action(surface: M5RawRenderedSurface) -> M5RawRenderedTransferChoice {
    M5RawRenderedTransferChoice {
        action_id: RepresentationActionId::CopyRendered.as_str().to_owned(),
        representation_class: RepresentationClass::Rendered.as_str().to_owned(),
        body_posture: BodyPosture::RenderedView.as_str().to_owned(),
        label: format!("Copy rendered {}", surface.label_noun()),
        preserves_canonical_bytes: false,
        implies_byte_identical_raw: false,
        export_safe: false,
    }
}

fn export_safe_action(surface: M5RawRenderedSurface) -> M5RawRenderedTransferChoice {
    M5RawRenderedTransferChoice {
        action_id: RepresentationActionId::ExportSanitizedSnapshot
            .as_str()
            .to_owned(),
        representation_class: RepresentationClass::Sanitized.as_str().to_owned(),
        body_posture: BodyPosture::SanitizedStaticSnapshot.as_str().to_owned(),
        label: format!("Export safe {} snapshot", surface.label_noun()),
        preserves_canonical_bytes: false,
        implies_byte_identical_raw: false,
        export_safe: true,
    }
}

fn build_handoff_preservation(
    case_id: &str,
    surfaces: &[M5RawRenderedSurfaceProjection],
) -> M5RepresentationHandoffPreservation {
    let diverging: Vec<&M5RawRenderedSurfaceProjection> = surfaces
        .iter()
        .filter(|surface| surface.materially_diverges)
        .collect();
    let has_divergence = !diverging.is_empty();

    let surfaces_covered: Vec<String> = diverging
        .iter()
        .map(|surface| surface.surface_token.clone())
        .collect();
    let preserved_transforms = distinct_tokens(
        diverging
            .iter()
            .map(|surface| surface.render_transform.as_str()),
    );
    let exemplar = diverging
        .iter()
        .find(|surface| surface.divergence == M5RepresentationDivergence::RenderedSummarizesContent)
        .or_else(|| diverging.first())
        .map(|surface| escape_exemplar(surface.divergence.note()))
        .unwrap_or_default();

    let carriers = M5HandoffCarrier::ALL
        .iter()
        .map(|&carrier| {
            build_carrier(
                carrier,
                has_divergence,
                &surfaces_covered,
                &preserved_transforms,
                &exemplar,
            )
        })
        .collect();

    M5RepresentationHandoffPreservation {
        record_kind: M5_RAW_RENDERED_HANDOFF_PRESERVATION_RECORD_KIND.to_owned(),
        case_id: case_id.to_owned(),
        carriers,
        preserves_divergence_warning: has_divergence,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn build_carrier(
    carrier: M5HandoffCarrier,
    has_divergence: bool,
    surfaces_covered: &[String],
    preserved_transforms: &[String],
    exemplar: &str,
) -> M5HandoffCarrierPreservation {
    if !has_divergence {
        return M5HandoffCarrierPreservation {
            carrier,
            carrier_token: carrier.as_str().to_owned(),
            captured_representation: "byte_identical".to_owned(),
            preserved_labels: vec![RepresentationClass::Raw.as_str().to_owned()],
            preserved_render_transforms: Vec::new(),
            preserves_divergence_note: false,
            divergence_note: "Rendered form byte-identical to raw; no divergence to preserve."
                .to_owned(),
            declares_rendered_not_raw: false,
            escaped_exemplar: String::new(),
            surfaces_covered: Vec::new(),
        };
    }

    let (captured_representation, preserved_labels, divergence_note) = match carrier {
        M5HandoffCarrier::SupportExport => (
            "raw_and_rendered_labeled",
            vec![
                RepresentationClass::Raw.as_str().to_owned(),
                RepresentationClass::Rendered.as_str().to_owned(),
            ],
            "Captured representations are labeled raw versus rendered; rendered text is the viewer's display form and is not the canonical raw bytes.",
        ),
        M5HandoffCarrier::ScreenshotCaption => (
            "rendered_only_with_disclaimer",
            vec![RepresentationClass::Rendered.as_str().to_owned()],
            "Screenshot shows the rendered view only; the canonical raw bytes differ and remain available from the source surface.",
        ),
        M5HandoffCarrier::HandoffPacket => (
            "raw_and_rendered_labeled",
            vec![
                RepresentationClass::Raw.as_str().to_owned(),
                RepresentationClass::Rendered.as_str().to_owned(),
            ],
            "Handoff packet preserves both raw and rendered labels plus the per-surface render transform so a downstream reader can tell rendered output from canonical raw bytes.",
        ),
    };

    M5HandoffCarrierPreservation {
        carrier,
        carrier_token: carrier.as_str().to_owned(),
        captured_representation: captured_representation.to_owned(),
        preserved_labels,
        preserved_render_transforms: preserved_transforms.to_vec(),
        preserves_divergence_note: true,
        divergence_note: divergence_note.to_owned(),
        declares_rendered_not_raw: true,
        escaped_exemplar: exemplar.to_owned(),
        surfaces_covered: surfaces_covered.to_vec(),
    }
}

fn validate_handoff_preservation(
    packet: &M5RawRenderedHandoffPacket,
    violations: &mut Vec<M5RawRenderedHandoffViolation>,
) {
    let block = &packet.handoff_preservation;

    let present: BTreeSet<_> = block
        .carriers
        .iter()
        .map(|carrier| carrier.carrier)
        .collect();
    if !M5HandoffCarrier::ALL
        .iter()
        .all(|carrier| present.contains(carrier))
    {
        violations.push(M5RawRenderedHandoffViolation::HandoffCarrierMissing);
    }

    if !packet.handoff_preserves_representation() {
        violations.push(M5RawRenderedHandoffViolation::HandoffDropsDivergenceWarning);
    }

    if block
        .carriers
        .iter()
        .any(|carrier| has_suspicious_content(&carrier.escaped_exemplar))
    {
        violations.push(M5RawRenderedHandoffViolation::HandoffLeaksRawBytes);
    }
}

fn escape_exemplar(text: &str) -> String {
    text.chars()
        .map(|ch| {
            if ch.is_control() || !ch.is_ascii() {
                format!("\\u{{{:04X}}}", ch as u32)
            } else {
                ch.to_string()
            }
        })
        .collect()
}
